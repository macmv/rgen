//! Coast-rooted drainage planning, independent of Minecraft chunk generation.
//! Grow upstream through a broad elevation field, then shape terrain around the
//! resulting tree. Planning elevations are virtual: all actual water is Y=63.
//! Catchments are bounded nearest-mouth domains, not an unbounded flow simulation.
use std::{cell::RefCell, num::NonZero, sync::{Arc, Mutex, OnceLock}};
use lru::LruCache;
use crate::terrain::TerrainNoise;

const MOUTH_SPACING: f64 = 768.0;
const BASIN_RADIUS: f64 = 1536.0;
const TILE: f64 = 128.0;
const REACH: f64 = 112.0;
const MAX_NODES: usize = 128;
const SEA: f64 = 63.0;
type Key = (u64, i32, i32);

#[derive(Clone, Copy, Debug)]
pub struct RiverSample {
  pub distance: f64,
  pub half_width: f64,
  /// Physical water height; never the planner's virtual elevation.
  pub water_level: f64,
  /// Monotonically nondecreasing upstream; prevents wet/dry/wet interruptions.
  pub inlandness: f64,
  pub bank_span: f64,
  pub style: f64,
  pub detail: f64,
  pub bank_detail: f64,
  pub mouth_factor: f64,
  pub offshore: bool,
}
impl Default for RiverSample {
  fn default() -> Self {
    Self { distance: f64::INFINITY, half_width: 6.0, water_level: SEA, inlandness: 0.0,
      bank_span:REACH, style:0.5, detail:0.0, bank_detail:0.0, mouth_factor:0.0, offshore:false }
  }
}
impl RiverSample {
  pub fn bank_weight(self) -> f64 {
    let t = ((self.distance - self.half_width) / (self.bank_span.max(self.half_width+8.0) - self.half_width)).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
  }
  pub fn valley_weight(self) -> f64 {
    let t = ((self.inlandness - 0.56) / 0.14).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
  }
  pub fn cliff_strength(self, terrain_height:f64) -> f64 {
    if self.inlandness < 0.56 || !self.distance.is_finite() { return 0.0; }
    let mut center = self;
    center.distance = 0.0;
    let drop = (terrain_height-center.channel_floor(terrain_height)).max(0.0);
    let run = (self.bank_span-self.half_width).max(8.0);
    let t = ((drop/run-1.0)/0.8).clamp(0.0,1.0);
    t*t*(3.0-2.0*t)
  }

  pub fn carving_weight(self, terrain_height:f64) -> f64 {
    let normal = self.bank_weight();
    let cliff = self.cliff_strength(terrain_height);
    if cliff == 0.0 { return normal; }
    let run = (self.bank_span-self.half_width).max(8.0);
    // At full strength the wall rises across three horizontal blocks.
    let t = ((self.distance-self.half_width-run*0.35)/3.0).clamp(0.0,1.0);
    let sheer = 1.0-t*t*(3.0-2.0*t);
    normal*(1.0-cliff)+sheer*cliff
  }

  pub fn channel_floor(self, terrain_height: f64) -> f64 {
    let across = (self.distance / self.half_width).clamp(0.0, 1.0);
    let bank_distance = (self.distance - self.half_width).max(0.0);
    // Long reaches range from narrow/deep to broad/shallow. Fine noise adds
    // pools and shoals; the channel center always remains an open water path.
    // Raise the old bed by five blocks, retaining at least a two-block
    // nominal channel where the old river was already shallow.
    let depth = (3.5 + (1.0-self.style)*4.5 - 5.0).max(2.0);
    let bowl = across.powf(1.1 + self.style*2.4);
    let bed_detail = (self.detail*2.2).min(depth-1.0);
    let mut wet = 64.0 - depth*(1.0-bowl) + bed_detail*(1.0-across);
    // Patchy off-center bars in broad reaches. Fade them out at sea mouths.
    let braid = ((self.style-0.82)/0.18).clamp(0.0,1.0);
    let bar = (1.0-(across-0.55).abs()/0.28).max(0.0);
    wet += braid * bar * (self.bank_detail*2.0+0.5).clamp(0.0,1.0)
      * 3.0 * (1.0-self.mouth_factor);
    wet = wet.min(65.5);
    if bank_distance > 0.0 {
      // Irregular shelves and cuts, with a hard dry-bank guarantee. The noise
      // is world-coordinate based, so neighboring reaches share the same field.
      let ramp = (bank_distance/8.0).min(1.0);
      let slope = 0.12 + (1.0-self.style)*0.38;
      wet = (64.0 + bank_distance*slope + self.bank_detail*11.0*ramp).max(64.0);
    }
    // Broad undulations plus small relief instead of one flat Y=66 shelf.
    // Final terrain caps the existing density field; none of this adds walls.
    let minimum_floor = 66.5 + self.bank_detail + self.detail*0.5;
    let dry = (terrain_height - 18.0 + self.bank_detail*6.0
      + self.detail*2.0 + across*across*3.0).max(minimum_floor);
    let valley = self.valley_weight();
    wet * (1.0 - valley) + dry * valley
  }
}

#[derive(Clone, Copy, Debug)]
struct Point { x: f64, z: f64 }
impl Point {
  fn distance(self, b: Self) -> f64 { (self.x - b.x).hypot(self.z - b.z) }
  fn toward(self, b: Self, t: f64) -> Self {
    Self { x: self.x + (b.x - self.x) * t, z: self.z + (b.z - self.z) * t }
  }
}
#[derive(Clone, Copy)]
struct Mouth { id: (i32, i32), sea: Point, land: Point }
struct Node {
  point: Point,
  parent: usize,
  level: f64,
  inlandness: f64,
  direction: f64,
  depth: usize,
  area: usize,
}
#[derive(Clone, Copy)]
struct Segment {
  a: Point, b: Point,
  inland_a: f64, inland_b: f64,
  width_a: f64, width_b: f64,
  mouth_a: f64, mouth_b: f64,
  offshore: bool,
}
struct Plan { segments: Vec<Segment> }
thread_local! {
  static MOUTHS: RefCell<LruCache<Key, Option<Mouth>>> =
    RefCell::new(LruCache::new(NonZero::new(2048).unwrap()));
  static TILES: RefCell<LruCache<Key, Arc<Plan>>> =
    RefCell::new(LruCache::new(NonZero::new(512).unwrap()));
}
// Share in-flight construction as well as finished plans. Only requests for
// the same basin wait; unrelated basins still build concurrently.
type BasinCell = Arc<OnceLock<Arc<Plan>>>;
static BASINS: OnceLock<Mutex<LruCache<Key, BasinCell>>> = OnceLock::new();
fn hash(mut n: u64) -> u64 {
  n = (n ^ (n >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
  n = (n ^ (n >> 27)).wrapping_mul(0x94d049bb133111eb);
  n ^ (n >> 31)
}
fn random(seed: u64, n: u64) -> f64 { (hash(seed ^ hash(n)) >> 11) as f64 / ((1u64 << 53) as f64) }
fn projection(p: Point, a: Point, b: Point) -> (f64, f64) {
  let dx = b.x - a.x;
  let dz = b.z - a.z;
  let t = (((p.x - a.x) * dx + (p.z - a.z) * dz) / (dx * dx + dz * dz)).clamp(0.0, 1.0);
  (p.distance(a.toward(b, t)), t)
}
fn orientation(a: Point, b: Point, c: Point) -> f64 {
  (b.x - a.x) * (c.z - a.z) - (b.z - a.z) * (c.x - a.x)
}
fn intersects(a: Point, b: Point, c: Point, d: Point) -> bool {
  orientation(a,b,c) * orientation(a,b,d) < 0.0 && orientation(c,d,a) * orientation(c,d,b) < 0.0
}
impl TerrainNoise {
  /// The shoreline includes submerged beaches, not just Ocean geography.
  /// Uses uncarved terrain so the network never depends on its own carving.
  fn drainage_land(&self, p: Point) -> bool {
    let c = self.sample_continentalness(p.x,p.z);
    if c < 0.32 { return false; }
    // With the watershed's nonnegative relief spline, every c>=0.37 is
    // already dry for ALL erosion/relief values. Only the coast needs a
    // full height evaluation; this preserves the one-block shoreline checks.
    if crate::feature::WATERSHEDS && c >= 0.37 { return true; }
    let raw = self.sample_unrouted_with_continentalness(p.x,p.z,c);
    crate::terrain::unrouted_height(
      &crate::CONTINENTALNESS, &crate::EROSION,
      &crate::PEAKS_VALLEYS, &crate::HEIGHT_IMPACT, raw).height >= 64.0
  }

  fn land_route(&self, a:Point, b:Point) -> bool {
    let steps = a.distance(b).ceil().max(1.0) as usize;
    (0..=steps).all(|i| self.drainage_land(a.toward(b,i as f64/steps as f64)))
  }

  fn inland_route(&self, a:Point, b:Point, mouth:Point) -> bool {
    let steps = (a.distance(b)/16.0).ceil().max(1.0) as usize;
    (0..=steps).all(|i| {
      let p = a.toward(b,i as f64/steps as f64);
      // First 128 blocks are the mouth approach; beyond it, keep a 64-block
      // coastal buffer instead of laying a channel alongside the water's edge.
      if p.distance(mouth) <= 128.0 { return true; }
      (0..8).all(|k| {
        let angle = k as f64*std::f64::consts::TAU/8.0;
        self.drainage_land(Point { x:p.x+64.0*angle.cos(),z:p.z+64.0*angle.sin() })
      })
    })
  }

  fn offshore_height(&self,p:Point) -> f64 {
    crate::terrain::unrouted_height(&crate::CONTINENTALNESS,&crate::EROSION,
      &crate::PEAKS_VALLEYS,&crate::HEIGHT_IMPACT,self.sample_unrouted(p.x,p.z)).height
  }

  fn outlet(&self,mouth:Mouth,width:f64) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut p = mouth.sea;
    let mut heading = (mouth.sea.z-mouth.land.z).atan2(mouth.sea.x-mouth.land.x);
    // Bridge the bisected shoreline before following the submerged shelf.
    segments.push(Segment { a:p,b:mouth.land,inland_a:0.32,inland_b:0.32,
      width_a:width,width_b:width,mouth_a:1.0,mouth_b:1.0,offshore:true });
    for step in 0..16 {
      let h = self.offshore_height(p);
      if step >= 2 && h <= 52.0 { break; }
      let score_at = |q:Point| self.offshore_height(q)+self.sample_continentalness(q.x,q.z)*40.0;
      let previous_score = score_at(p);
      let mut best: Option<(f64,Point,f64)> = None;
      for k in 0..13 {
        let angle = heading + (k as f64-6.0)*0.22;
        let q = Point { x:p.x+24.0*angle.cos(),z:p.z+24.0*angle.sin() };
        if (1..=24).any(|i| self.drainage_land(p.toward(q,i as f64/24.0))) { continue; }
        let score = score_at(q);
        if score >= previous_score { continue; }
        let cost = score+(angle-heading).abs()*0.2;
        if best.is_none_or(|v| cost < v.0) { best = Some((cost,q,angle)); }
      }
      let Some((_,q,angle)) = best else { break; };
      // Downstream -> upstream ordering, also used by the viewer arrows.
      segments.push(Segment { a:q,b:p,inland_a:0.32,inland_b:0.32,
        width_a:width,width_b:width,mouth_a:1.0,mouth_b:1.0,offshore:true });
      p = q;
      heading = angle;
    }
    segments
  }

  fn mouth(&self, x: i32, z: i32) -> Option<Mouth> {
    let key = (self.seed, x, z);
    if let Some(value) = MOUTHS.with(|c| c.borrow_mut().get(&key).copied()) { return value; }
    // Search a stratified coarse coastline, then bisect to the exact mouth.
    // These grid points choose outlets only; no river follows their edges.
    let salt = hash(self.seed ^ hash(x as u64) ^ hash(z as u64 ^ 0xD317));
    let mut best: Option<(f64, Point, Point)> = None;
    for iz in 0..4 {
      for ix in 0..4 {
        let p = Point {
          x: (x as f64 + (ix as f64 + 0.5) / 4.0) * MOUTH_SPACING,
          z: (z as f64 + (iz as f64 + 0.5) / 4.0) * MOUTH_SPACING,
        };
        let land_p = self.drainage_land(p);
        for (dx, dz) in [(192.0,0.0),(0.0,192.0)] {
          let q = Point { x: p.x + dx, z: p.z + dz };
          let land_q = self.drainage_land(q);
          if land_p == land_q { continue; }
          let score = random(salt, (iz * 8 + ix * 2 + (dz > 0.0) as usize) as u64);
          if best.as_ref().is_none_or(|v| score < v.0) {
            let (sea, land) = if land_p { (q,p) } else { (p,q) };
            best = Some((score,sea,land));
          }
        }
      }
    }
    let value = best.map(|(_, mut sea, mut land)| {
      for _ in 0..10 {
        let mid = sea.toward(land,0.5);
        if self.drainage_land(mid) { land = mid; } else { sea = mid; }
      }
      Mouth { id: (x,z), sea, land }
    });
    MOUTHS.with(|c| c.borrow_mut().put(key,value));
    value
  }

  fn basin(&self, mouth: Mouth) -> Arc<Plan> {
    let key = (self.seed,mouth.id.0,mouth.id.1);
    let cache = BASINS.get_or_init(|| Mutex::new(LruCache::new(NonZero::new(256).unwrap())));
    let cell = {
      let mut cache = cache.lock().unwrap();
      if let Some(cell) = cache.get(&key) { Arc::clone(cell) }
      else {
        let cell = Arc::new(OnceLock::new());
        cache.put(key,Arc::clone(&cell));
        cell
      }
    };
    Arc::clone(cell.get_or_init(|| self.build_basin(mouth)))
  }

  fn build_basin(&self, mouth:Mouth) -> Arc<Plan> {
    profile_scope!("coast rooted basin plan");
    let salt = hash(self.seed ^ hash(mouth.id.0 as u64) ^ hash(mouth.id.1 as u64 ^ 0x812));
    let mut neighbors = Vec::new();
    // Any mouth capable of winning ownership inside this basin lies within
    // 2*radius. Include the mouth-search overshoot when bounding its cell.
    for z in mouth.id.1-5..=mouth.id.1+5 {
      for x in mouth.id.0-5..=mouth.id.0+5 {
        if let Some(m) = self.mouth(x,z) { neighbors.push(m); }
      }
    }
    let direction = (mouth.land.z-mouth.sea.z).atan2(mouth.land.x-mouth.sea.x);
    let mut nodes = vec![Node { point: mouth.land, parent: 0, level: 0.0,
      inlandness: 0.32, direction, depth: 0, area: 1 }];
    let mut frontier = vec![0];
    while !frontier.is_empty() && nodes.len() < MAX_NODES {
      // Lowest virtual elevation first: tributaries develop across the basin
      // rather than one greedy branch consuming the entire node allowance.
      let slot = (0..frontier.len()).min_by(|&a,&b|
        nodes[frontier[a]].level.total_cmp(&nodes[frontier[b]].level)).unwrap();
      let parent = frontier.swap_remove(slot);
      if nodes[parent].depth >= 16 { continue; }
      let p = nodes[parent].point;
      let fan = if parent == 0 { 1 } else if random(salt,parent as u64 * 31) < 0.42 { 2 } else { 1 };
      for arm in 0..fan {
        let n = parent as u64 * 103 + arm as u64 * 19;
        let length = 85.0 + random(salt,n+2) * 65.0;
        let turn = if fan == 2 { if arm == 0 { -0.65 } else { 0.65 } } else { 0.0 };
        let mut choice: Option<(f64, Point, f64, f64, f64)> = None;
        for attempt in 0..9 {
          let angle = nodes[parent].direction + turn + (random(salt,n+300+attempt) - 0.5) * 1.8;
          let q = Point { x:p.x + angle.cos()*length, z:p.z + angle.sin()*length };
          let dist = q.distance(mouth.sea);
          if dist > BASIN_RADIUS || neighbors.iter().any(|m|
            m.id != mouth.id && (q.distance(m.sea),m.id) < (dist,mouth.id)) { continue; }
          let raw = self.sample_unrouted(q.x,q.z);
          if raw.continentalness < 0.325 { continue; }
          if nodes.iter().enumerate().any(|(i,v)| {
            if i == 0 || i == parent { return false; }
            let a = nodes[v.parent].point;
            intersects(p,q,a,v.point) || projection(q,a,v.point).0 < 64.0
          }) { continue; }
          // The broad field guides the river's virtual uphill development.
          // A modest virtual lift can cross a shallow local depression; large
          // downhill jumps are rejected rather than made into random lakes.
          let broad = (raw.continentalness-0.32)*150.0 + raw.peaks_valleys*28.0*(1.0-raw.erosion);
          if parent != 0 && broad + 12.0 < nodes[parent].level { continue; }
          let level = broad.max(nodes[parent].level + 1.5);
          let score = (level-nodes[parent].level-5.0).abs()
            + (angle-nodes[parent].direction-turn).abs()*3.0;
          if choice.as_ref().is_none_or(|v| score < v.0) {
            // Check the actual shoreline at <=1-block intervals. Sparse checks
            // can jump across an inlet and incorrectly resume a river inland.
            // Validate exactly the pieces that the straight fallback emits, so
            // subdivision cannot shift coastal-buffer probes past a missed inlet.
            if !(0..8).all(|step| {
              let a = p.toward(q,step as f64/8.0);
              let b = p.toward(q,(step+1) as f64/8.0);
              self.land_route(a,b) && self.inland_route(a,b,mouth.land)
            }) { continue; }
            choice = Some((score,q,angle,level,raw.continentalness));
          }
        }
        if let Some((_,q,angle,level,c)) = choice {
          let inlandness = nodes[parent].inlandness.max(c);
          let depth = nodes[parent].depth+1;
          nodes.push(Node { point:q, parent, level, inlandness, direction:angle, depth, area:1 });
          frontier.push(nodes.len()-1);
        }
        if nodes.len() >= MAX_NODES { break; }
      }
    }
    for i in (1..nodes.len()).rev() { let p = nodes[i].parent; nodes[p].area += nodes[i].area; }
    let widths: Vec<f64> = nodes.iter().enumerate().map(|(i,n)| {
      let normal = 3.0 + (n.area as f64).sqrt().min(9.0);
      // Deliberately chosen inline lowland lakes, never isolated sink pools.
      if n.depth > 2 && n.area >= 5 && n.inlandness < 0.53
        && random(salt,i as u64+90000) < 0.06 {
        24.0 + random(salt,i as u64+91000)*12.0
      } else { normal }
    }).collect();
    let mut segments = Vec::new();
    for i in 1..nodes.len() {
      let parent = nodes[i].parent;
      let a = &nodes[parent];
      let b = &nodes[i];
      assert!(b.level > a.level);
      assert!(b.inlandness >= a.inlandness);
      let length = a.point.distance(b.point);
      let mut control = Point { x:a.point.x+a.direction.cos()*length*0.3,
        z:a.point.z+a.direction.sin()*length*0.3 };
      // Keep the entire Bezier inside its convex nearest-mouth domain.
      // Its other control lies on the original segment.
      let distance = control.distance(mouth.sea);
      if distance > BASIN_RADIUS || !self.drainage_land(control)
        || neighbors.iter().any(|m| m.id != mouth.id
          && (control.distance(m.sea),m.id) < (distance,mouth.id)) {
        control = a.point.toward(b.point,0.3);
      }
      let end_control = b.point.toward(a.point,0.3);
      let point = |t:f64| {
        let u = 1.0-t;
        let bend = (random(salt,i as u64+45000)-0.5)*length*0.40
          * (std::f64::consts::PI*t).sin().powi(2)
          * (std::f64::consts::TAU*t+random(salt,i as u64+46000)*6.0).sin();
        Point { x:u*u*u*a.point.x+3.0*u*u*t*control.x+3.0*u*t*t*end_control.x+t*t*t*b.point.x
            - (b.point.z-a.point.z)/length*bend,
          z:u*u*u*a.point.z+3.0*u*u*t*control.z+3.0*u*t*t*end_control.z+t*t*t*b.point.z
            + (b.point.x-a.point.x)/length*bend }
      };
      // Reject a coastal shortcut caused by smoothing over a small bay.
      let smooth = (0..8).all(|step| {
        let a = point(step as f64/8.0);
        let b = point((step+1) as f64/8.0);
        self.land_route(a,b) && self.inland_route(a,b,mouth.land)
      });
      for step in 0..8 {
        let t0 = step as f64/8.0;
        let t1 = (step+1) as f64/8.0;
        let at = |t| if smooth { point(t) } else { a.point.toward(b.point,t) };
        segments.push(Segment { a:at(t0), b:at(t1),
          inland_a:a.inlandness+(b.inlandness-a.inlandness)*t0,
          inland_b:a.inlandness+(b.inlandness-a.inlandness)*t1,
          width_a:widths[parent]+(widths[i]-widths[parent])*t0,
          width_b:widths[parent]+(widths[i]-widths[parent])*t1,
          mouth_a:(1.0-at(t0).distance(mouth.land)/192.0).clamp(0.0,1.0),
          mouth_b:(1.0-at(t1).distance(mouth.land)/192.0).clamp(0.0,1.0),
          offshore:false });
      }
    }
    if nodes.len() > 1 { segments.extend(self.outlet(mouth,widths[0])); }
    let plan = Arc::new(Plan { segments });
    plan
  }

  fn drainage_tile(&self, x:i32,z:i32) -> Arc<Plan> {
    let key = (self.seed,x,z);
    if let Some(p) = TILES.with(|c| c.borrow_mut().get(&key).cloned()) { return p; }
    let lo = Point { x:x as f64*TILE,z:z as f64*TILE };
    let hi = Point { x:lo.x+TILE,z:lo.z+TILE };
    let radius = BASIN_RADIUS+REACH;
    let mut segments = Vec::new();
    // The grid is just a spatial index. Both sides of a tile boundary retrieve
    // the same canonical basin edges, with a full carving-support halo.
    for mz in ((lo.z-radius)/MOUTH_SPACING).floor() as i32-1..=((hi.z+radius)/MOUTH_SPACING).floor() as i32 {
      for mx in ((lo.x-radius)/MOUTH_SPACING).floor() as i32-1..=((hi.x+radius)/MOUTH_SPACING).floor() as i32 {
        let Some(m) = self.mouth(mx,mz) else { continue; };
        let nearest = Point { x:m.sea.x.clamp(lo.x,hi.x),z:m.sea.z.clamp(lo.z,hi.z) };
        if nearest.distance(m.sea) > radius { continue; }
        for &s in &self.basin(m).segments {
          if s.a.x.min(s.b.x)-REACH <= hi.x && s.a.x.max(s.b.x)+REACH >= lo.x
            && s.a.z.min(s.b.z)-REACH <= hi.z && s.a.z.max(s.b.z)+REACH >= lo.z {
            segments.push(s);
          }
        }
      }
    }
    let p = Arc::new(Plan { segments });
    TILES.with(|c| c.borrow_mut().put(key,Arc::clone(&p)));
    p
  }

  pub fn watershed_sample(&self,x:f64,z:f64) -> RiverSample {
    let plan = self.drainage_tile((x/TILE).floor() as i32,(z/TILE).floor() as i32);
    let mut best = RiverSample::default();
    if plan.segments.is_empty() { return best; }
    let (style_noise, width_noise, detail, bank_detail) = self.river_texture(x,z);
    let style = (0.5+style_noise*1.5).clamp(0.0,1.0);
    let width_scale = (0.85+style*0.55+width_noise*1.1+bank_detail*0.45).clamp(0.5,2.2);
    let bank_span = (78.0+width_noise*55.0+bank_detail*24.0).clamp(40.0,REACH);
    for s in &plan.segments {
      let (distance,t) = projection(Point { x,z },s.a,s.b);
      if distance >= REACH { continue; }
      let width = (s.width_a+(s.width_b-s.width_a)*t)*width_scale;
      let inlandness = s.inland_a+(s.inland_b-s.inland_a)*t;
      let steep = ((inlandness-0.50)/0.20).clamp(0.0,1.0);
      let steep = steep*steep*(3.0-2.0*steep);
      // Keep the lowland banks broad. Inland, reduce their horizontal run
      // to a quarter, without lowering the river or dry-valley floor.
      let bank_run = (bank_span-width).max(16.0)*(1.0-steep*0.75);
      let candidate = RiverSample { distance,half_width:width,water_level:SEA,
        inlandness,
        bank_span:(width+bank_run.max(8.0)).min(REACH),style,detail,bank_detail,
        mouth_factor:s.mouth_a+(s.mouth_b-s.mouth_a)*t, offshore:s.offshore };
      // Compare the actual support weight, not distance minus width: a wide
      // lake edge at the halo limit must not hide a closer tributary's bank.
      if candidate.bank_weight() > best.bank_weight()
        || (candidate.bank_weight() == best.bank_weight()
          && distance/width < best.distance/best.half_width) {
        best = candidate;
      }
    }
    best
  }
}

impl TerrainNoise {
  /// Diagnostic output from the actual cached planner, not a second model.
  pub fn watershed_preview_paths(&self, x:f64,z:f64,size:f64,scale:f64) -> (String,usize) {
    use std::fmt::Write;
    let mut out = String::new();
    let mut basins = 0;
    for mz in ((z-BASIN_RADIUS)/MOUTH_SPACING).floor() as i32-1..=((z+size+BASIN_RADIUS)/MOUTH_SPACING).floor() as i32 {
      for mx in ((x-BASIN_RADIUS)/MOUTH_SPACING).floor() as i32-1..=((x+size+BASIN_RADIUS)/MOUTH_SPACING).floor() as i32 {
        let Some(m) = self.mouth(mx,mz) else { continue; };
        let plan = self.basin(m);
        if plan.segments.is_empty() { continue; }
        let mut visible = false;
        for s in &plan.segments {
          if s.a.x < x || s.a.x > x+size || s.a.z < z || s.a.z > z+size { continue; }
          visible = true;
          let color = if s.inland_b > 0.56 { "#9b6849" } else { "#175f91" };
          write!(out,"<path d='M{:.2} {:.2}L{:.2} {:.2}' stroke='{color}' stroke-width='{:.2}' stroke-linecap='round' fill='none'/>",
            (s.a.x-x)*scale,(s.a.z-z)*scale,(s.b.x-x)*scale,(s.b.z-z)*scale,
            (s.width_a+s.width_b)*scale).unwrap();
        }
        if visible { basins += 1; }
      }
    }
    (out,basins)
  }

  /// Small executable smoke audit: shared tile borders and physical sea level.
  /// This deliberately doesn't assert appearance or Minecraft fluid behavior.
  pub fn audit_watershed_preview(&self) -> String {
    let mut checks = 0;
    for i in -8..=8 {
      for j in -8..=8 {
        let x = i as f64*TILE;
        let z = j as f64*TILE+17.3;
        for (ax,az,bx,bz) in [(x-1e-5,z,x+1e-5,z),(z,x-1e-5,z,x+1e-5)] {
          let a = self.watershed_sample(ax,az);
          let b = self.watershed_sample(bx,bz);
          assert_eq!(a.water_level,SEA);
          assert_eq!(b.water_level,SEA);
          assert!((a.bank_weight()-b.bank_weight()).abs()<1e-4,"tile seam at {x},{z}");
          if a.bank_weight()>0.0 && b.bank_weight()>0.0 {
            assert!((a.valley_weight()-b.valley_weight()).abs()<1e-4,"valley seam at {x},{z}");
          }
          checks += 1;
        }
      }
    }
    let mut coast_checks = 0;
    for z in -3..=3 {
      for x in -3..=3 {
        let Some(m) = self.mouth(x,z) else { continue; };
        for segment in &self.basin(m).segments {
          if !segment.offshore {
            assert!(self.land_route(segment.a,segment.b),
              "river crosses natural water near {},{}",segment.a.x,segment.a.z);
            assert!(self.inland_route(segment.a,segment.b,m.land),"coastal-parallel route");
          } else if segment.a.distance(segment.b) >= 1.0 {
            let mid = segment.a.toward(segment.b,0.5);
            assert!(!self.drainage_land(mid),"offshore outlet returned to land");
          }
          coast_checks += 1;
        }
      }
    }
    let mut bank_checks = 0;
    // Regression: wide banks must never flood previously dry land. Exercise
    // the actual terrain composition with deltas off (they intentionally flood).
    for c in [0.37, 0.40, 0.45, 0.50, 0.55, 0.65, 0.75] {
      for p in [0.25, 0.45, 0.5, 0.6, 0.75] {
        let mut raw = self.sample_unrouted(0.0, 0.0);
        raw.continentalness = c;
        raw.peaks_valleys = p;
        raw.erosion = 0.65;
        let splines = crate::Splines::game();
        // Default empty network bypasses the legacy noise-river path.
        raw.watershed = Some(RiverSample::default());
        let base = splines.height_parts(raw).height;
        if base < 64.0 { continue; }
        for width in [4.0, 12.0, 36.0] {
          for inlandness in [0.4, 0.6, 0.7] {
            for step in 0..=32 {
              let distance = width + (REACH-width)*step as f64/32.0;
              raw.watershed = Some(RiverSample { distance, half_width:width,
                water_level:SEA, inlandness, ..RiverSample::default() });
              for (style,detail,bank_detail,mouth_factor) in [
                (0.0,-1.0,-1.0,0.0),(0.0,1.0,1.0,1.0),
                (0.5,-1.0,1.0,0.0),(0.5,1.0,-1.0,1.0),
                (1.0,-1.0,-1.0,0.0),(1.0,1.0,1.0,1.0),
              ] {
                let r = raw.watershed.as_mut().unwrap();
                r.style = style;
                r.detail = detail;
                r.bank_detail = bank_detail;
                r.mouth_factor = mouth_factor;
                assert!(splines.height_parts(raw).height >= 64.0,
                  "bank flooded: base={base} distance={distance} width={width}");
                bank_checks += 1;
              }
            }
          }
        }
      }
    }
    let mut land_checks = 0;
    let splines = crate::Splines::game();
    // No river, lake or delta: every inland continentalness must stay dry,
    // including the old peaks/valleys trough at 0.48..0.52.
    for ci in 37..=100 {
      for pi in 0..=100 {
        for ei in [0, 25, 50, 75, 100] {
          let mut raw = self.sample_unrouted(0.0,0.0);
          raw.continentalness = ci as f64/100.0;
          raw.peaks_valleys = pi as f64/100.0;
          raw.erosion = ei as f64/100.0;
          raw.watershed = Some(RiverSample::default());
          assert!(splines.height_parts(raw).height >= 64.0,
            "unplanned inland water: continentalness={} peaks={} erosion={}",
            raw.continentalness,raw.peaks_valleys,raw.erosion);
          land_checks += 1;
        }
      }
    }
    let mut geography_checks = 0;
    let mut delta_checks = 0;
    for ci in 20..=50 {
      let mut raw = self.sample_unrouted(0.0,0.0);
      raw.continentalness = ci as f64/100.0;
      raw.peaks_valleys = 0.5;
      raw.erosion = 0.65;
      raw.watershed = Some(RiverSample::default());
      let base = splines.height_parts(raw).height;
      if ci < 37 {
        let coast = crate::terrain::watershed_coast(raw).unwrap();
        assert_eq!(coast, if base < 62.0 { crate::GeographicType::Ocean }
          else { crate::GeographicType::Beach });
        geography_checks += 1;
      }
      raw.delta_rarity = 1.0;
      raw.watershed = Some(RiverSample { distance:0.0, mouth_factor:1.0,
        ..RiverSample::default() });
      for island in [0.0,0.5,0.7,1.0] {
        raw.delta_islands = island;
        if crate::terrain::is_delta(raw) {
          assert!(splines.height_parts(raw).height <= 64.6+1e-8,
            "delta platform raised above island target");
          delta_checks += 1;
        }
      }
      raw.watershed.as_mut().unwrap().mouth_factor = 0.0;
      assert_eq!(crate::terrain::delta_strength(raw),0.0,"delta away from mouth");
    }
    assert!(delta_checks > 0,"no delta cases covered");
    let mut raw = self.sample_unrouted(0.0,0.0);
    raw.continentalness = 0.5;
    raw.watershed = Some(RiverSample { distance:12.0, half_width:6.0,
      ..RiverSample::default() });
    assert!(crate::terrain::is_river(raw),"dry bank missing from river geography");
    raw.watershed.as_mut().unwrap().distance = 20.0;
    assert!(!crate::terrain::is_river(raw),"river geography too broad");
    let mut profile = RiverSample { distance:0.0,..RiverSample::default() };
    let inland_bed = profile.channel_floor(80.0);
    profile.mouth_factor = 1.0;
    assert_eq!(inland_bed,profile.channel_floor(80.0),"extra mouth depth");
    profile.inlandness = 0.7;
    profile.bank_span = profile.half_width+8.0;
    assert_eq!(profile.cliff_strength(100.0),1.0,"steep valley did not become cliff");
    profile.distance = profile.half_width+2.7;
    let bottom = profile.carving_weight(100.0);
    profile.distance += 3.2;
    assert!(bottom > 0.99 && profile.carving_weight(100.0) < 0.01,"cliff wall too broad");
    format!("{checks} tile-border/sea-level checks, {bank_checks} dry-bank checks, and {land_checks} inland-height checks, and {coast_checks} coastline segment checks, {geography_checks} shore checks and {delta_checks} delta checks passed")
  }
}

/// A piece of the actual drainage plan, in world X/Z coordinates. The direction
/// is downstream -> upstream; water/valley flow points from `to` toward `from`.
#[derive(Clone, Copy, Debug)]
pub struct DrainageVector {
  pub from: (f64, f64),
  pub to: (f64, f64),
  pub valley: bool,
}

impl TerrainNoise {
  /// Read-only vector query for tools. Does not generate Minecraft chunks.
  pub fn drainage_vectors(&self, x:f64, z:f64, width:f64, height:f64) -> Vec<DrainageVector> {
    let mut result = Vec::new();
    if !crate::feature::WATERSHEDS { return result; }
    for mz in ((z-BASIN_RADIUS)/MOUTH_SPACING).floor() as i32-1..=((z+height+BASIN_RADIUS)/MOUTH_SPACING).floor() as i32 {
      for mx in ((x-BASIN_RADIUS)/MOUTH_SPACING).floor() as i32-1..=((x+width+BASIN_RADIUS)/MOUTH_SPACING).floor() as i32 {
        let Some(mouth) = self.mouth(mx,mz) else { continue; };
        for s in &self.basin(mouth).segments {
          if s.a.x.min(s.b.x) > x+width || s.a.x.max(s.b.x) < x
            || s.a.z.min(s.b.z) > z+height || s.a.z.max(s.b.z) < z { continue; }
          result.push(DrainageVector { from:(s.a.x,s.a.z),to:(s.b.x,s.b.z),
            valley:(s.inland_a+s.inland_b)*0.5 > 0.56 });
        }
      }
    }
    result
  }
}
