// Solve a padded grid from the sea inward. Artificial edge outlets provide a
// lower bound: expand the window if one of them could win inside the tile.
use std::{cell::RefCell, cmp::Reverse, collections::BinaryHeap, num::NonZeroUsize};
use lru::LruCache;
use rgen_base::Pos;
use crate::{ELEVATION_SPACING as STEP, WorldBiomes};

const TILE: i32 = 16;
#[derive(Clone, Copy, Default)]
pub struct Drainage {
  pub direction: (i8, i8),
  pub outlet: (i32, i32),
  pub spill_y: f64,
}

thread_local! {
  static TILES: RefCell<LruCache<(u64,i32,i32), Vec<Drainage>>> =
    RefCell::new(LruCache::new(NonZeroUsize::new(64).unwrap()));
}

impl WorldBiomes {
  pub fn drainage(&self, pos: Pos) -> Drainage {
    let x = (pos.x + STEP/2).div_euclid(STEP);
    let z = (pos.z + STEP/2).div_euclid(STEP);
    let tx = x.div_euclid(TILE);
    let tz = z.div_euclid(TILE);
    TILES.with(|cache| {
      let mut cache = cache.borrow_mut();
      let tile = cache.get_or_insert((self.seed,tx,tz), || {
        let mut pad = 16;
        loop {
          if let Some(tile) = self.solve_drainage(tx*TILE,tz*TILE,pad) { break tile; }
          pad *= 2;
        }
      });
      tile[(z.rem_euclid(TILE)*TILE+x.rem_euclid(TILE)) as usize]
    })
  }

  fn solve_drainage(&self, x0:i32,z0:i32,pad:i32) -> Option<Vec<Drainage>> {
    let n = TILE + 2*pad;
    let size = (n*n) as usize;
    let mut heights = Vec::with_capacity(size);
    let mut costs = vec![(u32::MAX,u32::MAX);size];
    let mut result = vec![Drainage::default();size];
    let mut edge = vec![false;size];
    let mut queue = BinaryHeap::new();
    for z in 0..n {
      for x in 0..n {
        let i = (z*n+x) as usize;
        let wx = (x0+x-pad)*STEP;
        let wz = (z0+z-pad)*STEP;
        let h = (self.broad_elevation(Pos::new(wx,0,wz))*256.0).round() as u32;
        heights.push(h);
        let ocean = h < 64*256;
        if ocean || x==0 || z==0 || x==n-1 || z==n-1 {
          costs[i] = (h,0);
          edge[i] = !ocean;
          result[i].outlet = (wx,wz);
          result[i].spill_y = h as f64/256.0;
          queue.push(Reverse((costs[i],!edge[i],z,x)));
        }
      }
    }
    while let Some(Reverse((cost,real,z,x))) = queue.pop() {
      let i = (z*n+x) as usize;
      if costs[i]!=cost || real==edge[i] { continue; }
      for dz in -1..=1 {
        for dx in -1..=1 {
          let (xx,zz)=(x+dx,z+dz);
          if xx<0 || zz<0 || xx>=n || zz>=n { continue; }
          let j=(zz*n+xx) as usize;
          let level=cost.0.max(heights[j]);
          let next=(level, if level>cost.0 {1} else {cost.1+1});
          if (next,real) >= (costs[j],!edge[j]) {continue;}
          costs[j]=next;
          edge[j]=!real;
          result[j]=Drainage {direction:(-dx as i8,-dz as i8),outlet:result[i].outlet,spill_y:level as f64/256.0};
          queue.push(Reverse((next,real,zz,xx)));
        }
      }
    }
    let mut tile=Vec::with_capacity((TILE*TILE) as usize);
    for z in pad..pad+TILE {
      for x in pad..pad+TILE {
        let i=(z*n+x) as usize;
        if edge[i] {return None;}
        tile.push(result[i]);
      }
    }
    Some(tile)
  }
}

thread_local! {
  static RUNOFF: RefCell<LruCache<(u64,i32,i32),u64>> =
    RefCell::new(LruCache::new(NonZeroUsize::new(32768).unwrap()));
}

impl WorldBiomes {
  /// Count contributing land samples by following incoming Phase 3 links.
  pub fn runoff(&self, pos: Pos) -> u64 {
    let center = |v:i32| (v+STEP/2).div_euclid(STEP)*STEP;
    let root = (self.seed,center(pos.x),center(pos.z));
    RUNOFF.with(|cell| {
      let mut cache = cell.borrow_mut();
      if let Some(&total) = cache.get(&root) {return total;}
      // Postorder traversal. Partial sums on the stack survive cache eviction.
      let mut stack = vec![(root,0,0u64)];
      loop {
        let &mut ((_,x,z),ref mut next,ref mut total) = stack.last_mut().unwrap();
        if *next == 9 {
          let (key,_,sum) = stack.pop().unwrap();
          cache.put(key,sum);
          if let Some(parent) = stack.last_mut() {parent.2 += sum;} else {return sum;}
          continue;
        }
        let dx = *next % 3 - 1;
        let dz = *next / 3 - 1;
        *next += 1;
        if dx == 0 && dz == 0 {
          *total += self.is_continent_land(Pos::new(x,0,z)) as u64;
          continue;
        }
        let p = Pos::new(x+dx*STEP,0,z+dz*STEP);
        if self.drainage(p).direction != (-dx as i8,-dz as i8) {continue;}
        let key = (self.seed,p.x,p.z);
        if let Some(&sum) = cache.get(&key) {*total += sum;} else {stack.push((key,0,0));}
      }
    })
  }
}

struct Reach {
  x: i32,
  z: i32,
  dx: f64,
  dz: f64,
  inverse_length_squared: f64,
  headwater_step: Option<usize>,
  runoff: u64,
}

thread_local! {
  static REACHES: RefCell<LruCache<(u64,i32,i32),Vec<Reach>>> =
    RefCell::new(LruCache::new(NonZeroUsize::new(256).unwrap()));
}

impl WorldBiomes {
  /// Warp the whole network in world coordinates so tributaries share junctions.
  pub fn channel_path(&self, node: Pos, direction: (i8,i8)) -> [(i32,i32); 9] {
    let point = |i: usize, node: Pos, direction: (i8,i8)| {
      let along = i as i32 * STEP / 8;
      let x = node.x + i32::from(direction.0)*along;
      let z = node.z + i32::from(direction.1)*along;
      let offset = |salt| {
        let noise = |scale, tag| crate::continents::field(self.seed ^ salt ^ tag,x,z,scale)-0.5;
        (64.0*noise(192,1) + 38.0*noise(80,2) + 14.0*noise(32,3)).round() as i32
      };
      let target = (x+offset(0x7731u64), z+offset(0xdb19u64));
      // Keep shoreline crossings anchored where the drainage plan meets water.
      if self.is_continent_land(Pos::new(x,0,z)) {
        let (distance,_) = crate::continents::coastal_profile(self.seed,x,z,true);
        let t = (distance/52.0).min(1.0);
        (x+((target.0-x) as f64*t).round() as i32,z+((target.1-z) as f64*t).round() as i32)
      } else { (x,z) }
    };
    let mut path = std::array::from_fn(|i| point(i, node, direction));
    let next = Pos::new(node.x + i32::from(direction.0)*STEP, 0,
                        node.z + i32::from(direction.1)*STEP);
    let outgoing = self.drainage(next).direction;
    if outgoing != (0,0) && outgoing != direction {
      // Ease into the outgoing tangent before the junction. All tributaries
      // still arrive at exactly the same warped endpoint.
      let before = path[1];
      let start = path[2];
      let end = path[8];
      let ahead = point(1, next, outgoing);
      for i in 1..6 {
        let t = i as f64 / 6.0;
        let u = 1.0-t;
        let curve = |prev:i32, a:i32, b:i32, next:i32| {
          let entry = f64::from(a) + 2.0*f64::from(a-prev);
          let exit = f64::from(b) - 2.0*f64::from(next-b);
          (u*u*u*f64::from(a) + 3.0*u*u*t*entry + 3.0*u*t*t*exit + t*t*t*f64::from(b)).round() as i32
        };
        path[2+i] = (curve(before.0,start.0,end.0,ahead.0), curve(before.1,start.1,end.1,ahead.1));
      }
    }
    path
  }

  /// Y of the top solid block. Water fills through Y=63 in the chunk pass.
  pub(crate) fn carve_ground(&self, pos: Pos, ground: f64) -> (f64, bool) {
    if ground < 64.0 { return (ground, false); }
    let gx = pos.x.div_euclid(STEP);
    let gz = pos.z.div_euclid(STEP);
    REACHES.with(|cache| {
      let mut cache = cache.borrow_mut();
      let reaches = cache.get_or_insert((self.seed,gx,gz), || {
        let mut reaches = Vec::new();
        // All carving tapers to zero within one grid step of its segment.
        for z in gz-2..=gz+3 {
          for x in gx-2..=gx+3 {
            let node = Pos::new(x*STEP,0,z*STEP);
            let (dx,dz) = self.drainage(node).direction;
            if (dx,dz)==(0,0) {continue;}
            let runoff = self.runoff(node);
            if runoff < 16 {continue;}
            // Only taper the first wet reach. A confluence with an existing
            // river must carry that channel through at full depth.
            let headwater = runoff >= 64 && !(-1..=1).any(|nz:i32| (-1..=1).any(|nx:i32| {
              if nx == 0 && nz == 0 { return false; }
              let upstream = Pos::new(node.x + nx*STEP, 0, node.z + nz*STEP);
              self.drainage(upstream).direction == (-nx as i8, -nz as i8)
                && self.runoff(upstream) >= 64
            }));
            for (index,pair) in self.channel_path(node,(dx,dz)).windows(2).enumerate() {
              let [(ax,az),(bx,bz)] = *pair else { unreachable!() };
              let radius = ((24.0 + (runoff as f64/16.0).log2().min(12.0)*2.5)*1.25).ceil() as i32;
              if ax.max(bx)+radius < gx*STEP || ax.min(bx)-radius >= (gx+1)*STEP
                || az.max(bz)+radius < gz*STEP || az.min(bz)-radius >= (gz+1)*STEP {continue;}
              let dx = f64::from(bx-ax);
              let dz = f64::from(bz-az);
              if dx==0.0 && dz==0.0 {continue;}
              reaches.push(Reach {x:ax,z:az,dx,dz,runoff,headwater_step:headwater.then_some(index),inverse_length_squared:1.0/(dx*dx+dz*dz)});
            }
          }
        }
        reaches
      });
      if reaches.is_empty() { return (ground, false); }
      let noise = |salt, scale| crate::continents::field(self.seed ^ salt, pos.x, pos.z, scale);
      let width_factor = 0.65 + 0.80 * noise(0x71ad, 91);
      let depth_factor = 0.70 + 0.90 * noise(0x8a34, 53);
      let slope_factor = noise(0xc172, 47);
      let bank_variation = crate::continents::bank_noise(self.seed,pos.x,pos.z)*0.5;
      let mut surface = ground;
      let mut river_bank = false;
      for r in reaches.iter() {
        let x = f64::from(pos.x-r.x);
        let z = f64::from(pos.z-r.z);
        let t = ((x*r.dx+z*r.dz)*r.inverse_length_squared).clamp(0.0,1.0);
        let distance = (x-t*r.dx).hypot(z-t*r.dz);
        let magnitude = (r.runoff as f64 / 16.0).log2().min(12.0);
        let base_radius = 24.0 + magnitude*2.5;
        let margin = ((distance-(3.0+magnitude))/(base_radius-3.0-magnitude)).clamp(0.0,1.0);
        let valley_radius = base_radius*(1.0+bank_variation*margin*margin*(3.0-2.0*margin));
        if distance >= valley_radius {continue;}
        let smooth = |x:f64| x*x*(3.0-2.0*x);
        let valley = (ground-(4.0+magnitude)*depth_factor*smooth(1.0-distance/valley_radius)).max(64.0);
        surface = surface.min(valley);
        if r.runoff < 64 {continue;}
        let wet = r.headwater_step.map_or(1.0, |index| smooth((index as f64 + t) / 8.0));
        // Segment distance already rounds the end cap. Give the source a
        // small radius and a shallow bed so that cap remains visible.
        let width = (3.0 + magnitude)*width_factor*(0.55 + 0.45*wet);
        let canyon = smooth(((ground-83.0)/10.0).clamp(0.0,1.0));
        let wall_run = (valley_radius-width)*(1.0-0.72*canyon);
        let rise = ((distance-width)/wall_run).clamp(0.0,1.0);
        // Shift the steepest part up or down the bank while keeping both ends smooth.
        let bank = smooth(rise + (2.0*slope_factor-1.0)*rise*(1.0-rise));
        let bed = 63.0-(2.0+0.3*magnitude)*depth_factor*(0.45+0.55*wet)*(1.0-(distance/width).min(1.0).powi(2));
        let bank_ground = valley + (bed-valley)*(1.0-bank);
        surface = surface.min(bank_ground);
        // Extend ordinary rivers across the low floodplain. Keep the existing
        // canyon corridor and stop the extra reach at the first higher terraces.
        let low_bank = ground < 93.0 && bank_ground < 66.0
          && distance <= width + 8.0 + 4.0*slope_factor;
        river_bank |= distance <= width + 3.0 || low_bank;
      }
      (surface, river_bank)
    })
  }
}
