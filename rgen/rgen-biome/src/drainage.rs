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
  runoff: u64,
}

thread_local! {
  static REACHES: RefCell<LruCache<(u64,i32,i32),Vec<Reach>>> =
    RefCell::new(LruCache::new(NonZeroUsize::new(256).unwrap()));
}

impl WorldBiomes {
  /// Y of the top solid block. Water fills through Y=63 in the chunk pass.
  pub fn carved_ground(&self, pos: Pos) -> f64 {
    let ground = self.broad_elevation(pos);
    if ground < 64.0 { return ground; }
    let gx = pos.x.div_euclid(STEP);
    let gz = pos.z.div_euclid(STEP);
    REACHES.with(|cache| {
      let mut cache = cache.borrow_mut();
      let reaches = cache.get_or_insert((self.seed,gx,gz), || {
        let mut reaches = Vec::new();
        // All carving tapers to zero within one grid step of its segment.
        for z in gz-1..=gz+2 {
          for x in gx-1..=gx+2 {
            let node = Pos::new(x*STEP,0,z*STEP);
            let (dx,dz) = self.drainage(node).direction;
            if (dx,dz)==(0,0) {continue;}
            let runoff = self.runoff(node);
            if runoff < 16 {continue;}
            let dx = f64::from(dx)*f64::from(STEP);
            let dz = f64::from(dz)*f64::from(STEP);
            reaches.push(Reach { x:node.x,z:node.z,dx,dz,runoff,
              inverse_length_squared:1.0/(dx*dx+dz*dz) });
          }
        }
        reaches
      });
      let mut surface = ground;
      for r in reaches.iter() {
        let x = f64::from(pos.x-r.x);
        let z = f64::from(pos.z-r.z);
        let t = ((x*r.dx+z*r.dz)*r.inverse_length_squared).clamp(0.0,1.0);
        let distance = (x-t*r.dx).hypot(z-t*r.dz);
        let magnitude = (r.runoff as f64 / 16.0).log2().min(12.0);
        let valley_radius = 24.0 + magnitude*2.5;
        if distance >= valley_radius {continue;}
        let smooth = |x:f64| x*x*(3.0-2.0*x);
        let valley = (ground-(4.0+magnitude)*smooth(1.0-distance/valley_radius)).max(64.0);
        surface = surface.min(valley);
        if r.runoff < 64 {continue;}
        let width = 3.0 + magnitude;
        let canyon = smooth(((ground-83.0)/10.0).clamp(0.0,1.0));
        let wall_run = (valley_radius-width)*(1.0-0.72*canyon);
        let bank = smooth(((distance-width)/wall_run).clamp(0.0,1.0));
        let bed = 63.0-(2.0+0.3*magnitude)*(1.0-(distance/width).min(1.0).powi(2));
        surface = surface.min(bed+(valley-bed)*bank);
      }
      surface
    })
  }
}
