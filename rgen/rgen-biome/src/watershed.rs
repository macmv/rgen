//! Bounded, lazy drainage graph. Sites are shared across planning-cell borders;
//! a plan contains nearby edges, never Minecraft chunks or recursive upstream
//! exploration. Inland local minima are explicit sink lakes in this first pass.
use std::{cell::RefCell, num::NonZero, sync::Arc};
use lru::LruCache;
use crate::terrain::TerrainNoise;

const SPACING: f64 = 512.0;
const WIDTH: f64 = 8.0;
const REACH: f64 = 96.0;
const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Copy, Debug)]
pub struct RiverSample {
  pub distance: f64,
  pub half_width: f64,
  /// Highest water block, interpolated along the downstream profile.
  pub water_level: f64,
}
impl Default for RiverSample {
  fn default() -> Self { Self { distance: f64::INFINITY, half_width: WIDTH, water_level: 63.0 } }
}
impl RiverSample {
  pub fn bank_weight(self) -> f64 {
    let t = ((self.distance - self.half_width - 8.0) / 48.0).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
  }
  pub fn wet(self) -> bool { self.distance <= self.half_width + 8.0 }
}

#[derive(Clone, Copy, Debug)]
struct Site {
  cell: (i32, i32),
  x: f64,
  z: f64,
  height: f64,
  continentalness: f64,
}
impl Site {
  fn level(self) -> f64 { 63.0 + (self.height - 64.0).max(0.0) * 0.35 }
  // Strict order means routing cannot cycle, including on flat terrain.
  fn lower_than(self, other: Self) -> bool {
    self.height.total_cmp(&other.height).then(self.cell.cmp(&other.cell)).is_lt()
  }
}

#[derive(Clone, Copy)]
struct Segment { from: Site, to: Site }
struct Plan { segments: Vec<Segment>, lakes: Vec<Site> }
thread_local! {
  static SITES: RefCell<LruCache<(u64, i32, i32), Site>> =
    RefCell::new(LruCache::new(NonZero::new(4096).unwrap()));
  static PLANS: RefCell<LruCache<(u64, i32, i32), Arc<Plan>>> =
    RefCell::new(LruCache::new(NonZero::new(128).unwrap()));
}

fn hash(mut n: u64) -> u64 {
  n = (n ^ (n >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
  n = (n ^ (n >> 27)).wrapping_mul(0x94d049bb133111eb);
  n ^ (n >> 31)
}

impl TerrainNoise {
  fn drainage_site(&self, x: i32, z: i32) -> Site {
    SITES.with(|cache| {
      let mut cache = cache.borrow_mut();
      let key = (self.seed, x, z);
      if let Some(site) = cache.get(&key) { return *site; }
      let h = hash(self.seed ^ hash(x as u64) ^ hash(z as u64 ^ 0xD317_A006));
      let jitter = |bits: u64| (bits as u16 as f64 / 65535.0 - 0.5) * 0.3;
      let px = (x as f64 + 0.5 + jitter(h)) * SPACING;
      let pz = (z as f64 + 0.5 + jitter(h >> 16)) * SPACING;
      // Uncarved spline height only: never calls sample(), which queries this graph.
      // Game splines keep plans independent of inspector edits and cache order.
      let raw = self.sample_unrouted(px, pz);
      let height = crate::terrain::unrouted_height(
        &crate::CONTINENTALNESS, &crate::EROSION,
        &crate::PEAKS_VALLEYS, &crate::HEIGHT_IMPACT, raw,
      ).height;
      let site = Site { cell: (x, z), x: px, z: pz, height, continentalness: raw.continentalness };
      cache.put(key, site);
      site
    })
  }

  fn downstream(&self, site: Site) -> Option<Site> {
    if site.continentalness < 0.30 && site.height < 63.0 { return None; }
    NEIGHBORS.iter().map(|&(dx, dz)| self.drainage_site(site.cell.0 + dx, site.cell.1 + dz))
      .filter(|n| n.lower_than(site))
      .min_by(|a, b| a.height.total_cmp(&b.height).then(a.cell.cmp(&b.cell)))
  }

  fn drainage_plan(&self, x: i32, z: i32) -> Arc<Plan> {
    PLANS.with(|cache| {
      let mut cache = cache.borrow_mut();
      let key = (self.seed, x, z);
      if let Some(plan) = cache.get(&key) { return Arc::clone(plan); }
      profile_scope!("watershed plan cache miss");
      let mut plan = Plan { segments: Vec::new(), lakes: Vec::new() };
      // Every edge touching this cell has its upstream site in this 3x3 halo.
      // Portal agreement comes from shared site coordinates and routing rules,
      // not from consulting previously generated neighboring regions.
      for dz in -1..=1 {
        for dx in -1..=1 {
          let site = self.drainage_site(x + dx, z + dz);
          if let Some(to) = self.downstream(site) {
            let vx = to.x - site.x;
            let vz = to.z - site.z;
            let length = vx.hypot(vz);
            let bend = (hash(self.seed ^ hash(site.cell.0 as u64)
              ^ hash(site.cell.1 as u64 ^ 0x713)) as u16 as f64 / 65535.0 - 0.5) * 64.0;
            let point = |t: f64| {
              let offset = (t * std::f64::consts::PI).sin().powi(2) * bend;
              Site { x: site.x + vx * t - vz / length * offset,
                z: site.z + vz * t + vx / length * offset,
                height: site.height * (1.0 - t) + to.height * t, ..site }
            };
            for step in 0..8 {
              let from = point(step as f64 / 8.0);
              let end = point((step + 1) as f64 / 8.0);
              if from.x.min(end.x) - REACH <= (x + 1) as f64 * SPACING
                && from.x.max(end.x) + REACH >= x as f64 * SPACING
                && from.z.min(end.z) - REACH <= (z + 1) as f64 * SPACING
                && from.z.max(end.z) + REACH >= z as f64 * SPACING
              { plan.segments.push(Segment { from, to: end }); }
            }
          } else if site.continentalness >= 0.30 && site.height >= 63.0 {
            plan.lakes.push(site);
          }
        }
      }
      // Stable tie ordering on both sides of a planning boundary.
      plan.segments.sort_by_key(|s| s.from.cell);
      plan.lakes.sort_by_key(|s| s.cell);
      let plan = Arc::new(plan);
      cache.put(key, Arc::clone(&plan));
      plan
    })
  }

  pub fn watershed_sample(&self, x: f64, z: f64) -> RiverSample {
    let plan = self.drainage_plan((x / SPACING).floor() as i32, (z / SPACING).floor() as i32);
    let mut best = RiverSample::default();
    let mut level_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut include = |sample: RiverSample| {
      // Compact support is zero at the halo edge, so neighboring plans agree.
      // Blend elevations where reaches meet instead of creating a hard ridge
      // at the nearest-segment boundary. Drainage edges themselves stay downhill.
      let weight = (1.0 - sample.distance / REACH).max(0.0).powi(4)
        / (1.0 + sample.distance * sample.distance);
      level_sum += sample.water_level * weight;
      weight_sum += weight;
      if sample.distance - sample.half_width < best.distance - best.half_width { best = sample; }
    };
    for edge in &plan.segments {
      let dx = edge.to.x - edge.from.x;
      let dz = edge.to.z - edge.from.z;
      let t = (((x - edge.from.x) * dx + (z - edge.from.z) * dz) / (dx * dx + dz * dz)).clamp(0.0, 1.0);
      let distance = (x - edge.from.x - t * dx).hypot(z - edge.from.z - t * dz);
      if distance > REACH { continue; }
      // Projection onto the locally indexed, gently curved drainage route.
      let sample = RiverSample { distance, half_width: WIDTH,
        water_level: edge.from.level() * (1.0 - t) + edge.to.level() * t };
      include(sample);
    }
    for lake in &plan.lakes {
      let distance = (x - lake.x).hypot(z - lake.z);
      if distance > REACH { continue; }
      let sample = RiverSample { distance, half_width: 24.0, water_level: lake.level() };
      include(sample);
    }
    if weight_sum > 0.0 { best.water_level = level_sum / weight_sum; }
    best
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn edges_descend_and_have_matching_endpoint_levels() {
    let terrain = TerrainNoise::new(1);
    for x in -8..=8 {
      for z in -8..=8 {
        let site = terrain.drainage_site(x, z);
        if let Some(next) = terrain.downstream(site) {
          assert!(next.lower_than(site));
          assert!(next.level() <= site.level());
          assert_eq!(next.level(), terrain.drainage_site(next.cell.0, next.cell.1).level());
        }
      }
    }
  }

  #[test]
  fn neighboring_plans_agree_at_borders() {
    let terrain = TerrainNoise::new(7);
    for border in -2..=2 {
      for offset in -64..64 {
        let x = border as f64 * SPACING;
        let z = offset as f64 * 16.0 + 0.125;
        for (ax, az, bx, bz) in [(x - 1e-6, z, x + 1e-6, z), (z, x - 1e-6, z, x + 1e-6)] {
          let a = terrain.watershed_sample(ax, az);
          let b = terrain.watershed_sample(bx, bz);
          assert!((a.bank_weight() - b.bank_weight()).abs() < 1e-5);
          if a.bank_weight() > 0.0 && b.bank_weight() > 0.0 {
            assert!((a.water_level - b.water_level).abs() < 1e-4);
          }
        }
      }
    }
  }

  #[test]
  fn cache_eviction_and_query_order_do_not_change_results() {
    let terrain = TerrainNoise::new(11);
    let before = terrain.watershed_sample(-17.0, 512.0);
    PLANS.with(|c| c.borrow_mut().clear());
    SITES.with(|c| c.borrow_mut().clear());
    terrain.watershed_sample(12000.0, -18000.0);
    let after = terrain.watershed_sample(-17.0, 512.0);
    assert_eq!(before.distance, after.distance);
    assert_eq!(before.water_level, after.water_level);
  }
}
