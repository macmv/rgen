// Small cells resolve the coast; broader fields decide which cells join.
const SPACING: i32 = 256;

fn hash(seed: u64, x: i32, z: i32) -> u64 {
  let mut n = seed ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
    ^ (z as u64).wrapping_mul(0xbf58476d1ce4e5b9);
  n = (n ^ (n >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
  n = (n ^ (n >> 27)).wrapping_mul(0x94d049bb133111eb);
  n ^ (n >> 31)
}

// Smooth seeded values at a chosen scale in blocks.
pub(crate) fn field(seed: u64, x: i32, z: i32, scale: i32) -> f64 {
  let gx = x.div_euclid(scale);
  let gz = z.div_euclid(scale);
  let fade = |v: i32| {
    let t = v.rem_euclid(scale) as f64 / scale as f64;
    t * t * (3.0 - 2.0 * t)
  };
  let row = |dz| {
    let value = |dx| (hash(seed, gx + dx, gz + dz) >> 11) as f64 / (1u64 << 53) as f64;
    let a = value(0);
    a + (value(1) - a) * fade(x)
  };
  let a = row(0);
  a + (row(1) - a) * fade(z)
}

// Cache the nine candidate sites per worker. All columns in a grid square
// share them, even when different chunks generate in a different order.
type Sites = [(i32, i32, bool); 9];
thread_local! {
  static SITES: std::cell::Cell<Option<((u64, i32, i32), Sites)>> = const { std::cell::Cell::new(None) };
}

pub fn land(seed: u64, x: i32, z: i32) -> bool {
  let bend = |salt| ((field(seed ^ salt, x, z, 96) - 0.5) * 72.0) as i32;
  let (x, z) = (x + bend(0xc11a), z + bend(0xb323));
  let island_pos = (x, z);
  let key = (seed, x.div_euclid(SPACING), z.div_euclid(SPACING));
  let x = x.rem_euclid(SPACING);
  let z = z.rem_euclid(SPACING);
  SITES.with(|cache| {
    let sites = match cache.get() {
      Some((old, sites)) if old == key => sites,
      _ => {
        let sites = std::array::from_fn(|i| {
          let dx = i as i32 % 3 - 1;
          let dz = i as i32 / 3 - 1;
          let h = hash(seed, key.1 + dx, key.2 + dz);
          let offset = |bits: u64| SPACING / 4 + (bits % (SPACING / 2) as u64) as i32;
          let sx = dx * SPACING + offset(h);
          let sz = dz * SPACING + offset(h >> 24);
          let wx = key.1 * SPACING + sx;
          let wz = key.2 * SPACING + sz;
          let region = field(seed ^ 0xd391, wx, wz, 2048);
          let local = field(seed ^ 0xa453, wx, wz, 768);
          let land = region * 0.65 + local * 0.35 > 0.43;
          (sx, sz, land)
        });
        cache.set(Some((key, sites)));
        sites
      }
    };
    sites.iter().min_by_key(|&&(sx, sz, _)| (sx-x).pow(2) + (sz-z).pow(2)).unwrap().2
  }) || field(seed ^ 0x1a7d, island_pos.0, island_pos.1, 192) > 0.935
}

// One bounded cache per generator worker. Adjacent grid squares share corners.
thread_local! {
  static RELIEF: std::cell::RefCell<lru::LruCache<(u64, i32, i32), f64>> =
    std::cell::RefCell::new(lru::LruCache::new(std::num::NonZeroUsize::new(1024).unwrap()));
}

pub const ELEVATION_SPACING: i32 = 64;

pub fn mountain_belt(seed: u64, x: i32, z: i32) -> f64 {
  let ridge = 1.0 - (2.0 * field(seed ^ 0x59b1, x, z, 1536) - 1.0).abs();
  let region = field(seed ^ 0x73ac, x, z, 2304);
  let weight = ((ridge * region - 0.30) / 0.42).clamp(0.0, 1.0);
  weight * weight * (3.0 - 2.0 * weight)
}

pub fn bank_noise(seed:u64,x:i32,z:i32) -> f64 {
  field(seed ^ 0x48fa, x, z, 80) - 0.5
}

pub fn rolling_detail(seed: u64, x: i32, z: i32) -> f64 {
  // Skew the detail grids in different directions so shoulders and gullies
  // cross the broad slope instead of following its interpolation axes.
  let shoulder = field(seed ^ 0x915a, x + z.div_euclid(2), z - x.div_euclid(3), 37);
  let fold = field(seed ^ 0xac71, x - z.div_euclid(3), z + x.div_euclid(2), 59);
  let hollow = (1.0 - (2.0 * fold - 1.0).abs()).powi(3);
  12.0 * bank_noise(seed, x, z) + 9.0 * (shoulder - 0.5) - 5.0 * (hollow - 0.35)
}

pub fn relief_strength(terrain: &crate::TerrainNoise, x:i32, z:i32) -> f64 {
  let erosion = terrain.sample_erosion(x as f64,z as f64);
  0.3 + 0.7 * crate::EROSION.sample::<rgen_spline::Cosine>(erosion)
}

pub fn broad_land_height(terrain: &crate::TerrainNoise, seed: u64, x: i32, z: i32) -> f64 {
  const STEP: i32 = ELEVATION_SPACING;
  let gx = x.div_euclid(STEP);
  let gz = z.div_euclid(STEP);
  let tx = x.rem_euclid(STEP) as f64 / STEP as f64;
  let tz = z.rem_euclid(STEP) as f64 / STEP as f64;
  RELIEF.with(|cache| {
    let mut cache = cache.borrow_mut();
    let mut node = |dx, dz| {
      let key = (seed, gx + dx, gz + dz);
      *cache.get_or_insert(key, || {
        let (x, z) = (key.1 * STEP, key.2 * STEP);
        if !land(seed, x, z) { return 64.0; }
        let plains = field(seed ^ 0x2d1a, x, z, 1536);
        // Offset the smaller fields from the planning lattice to break up broad slopes.
        let hills = field(seed ^ 0x821d, x + 71, z - 43, 320) - 0.5;
        let local = field(seed ^ 0xcb47, x - 37, z + 19, 144) - 0.5;
        let belt = mountain_belt(seed,x,z);
        let pv = terrain.sample_peaks_valleys(x as f64,z as f64);
        let shape = crate::PEAKS_VALLEYS.sample::<rgen_spline::Cosine>(pv);
        let ridge = shape.max(0.0)*12.0/256.0 + shape.min(0.0)*4.0/22.0;
        let relief = (30.0 * plains.powi(2) + 92.0 * belt
          + ridge * relief_strength(terrain,x,z) * (1.0-belt)).max(0.0);
        // Ease negative variations near sea level so lowlands retain their slopes.
        let detail = 38.0 * hills + 18.0 * local;
        let lowland_scale = relief / (relief + 28.0);
        64.0 + relief + detail.max(0.0) + detail.min(0.0) * lowland_scale
      })
    };
    let north = node(0, 0) * (1.0 - tx) + node(1, 0) * tx;
    let south = node(0, 1) * (1.0 - tx) + node(1, 1) * tx;
    north * (1.0 - tz) + south * tz
  })
}

// Distance to the opposite shore, sampled on an 8-block lattice. Cache both
// sides so neighboring columns share the coastline search.
thread_local! {
  static SHORE: std::cell::RefCell<lru::LruCache<(u64,i32,i32,bool),f64>> =
    std::cell::RefCell::new(lru::LruCache::new(std::num::NonZeroUsize::new(4096).unwrap()));
}

pub fn coastal_profile(seed:u64,x:i32,z:i32,on_land:bool) -> (f64,f64) {
  let gx=x.div_euclid(8)*8;
  let gz=z.div_euclid(8)*8;
  let distance=SHORE.with(|cache| {
    let mut cache=cache.borrow_mut();
    let mut at=|x,z| *cache.get_or_insert((seed,x,z,on_land),|| {
      let mut best=64*64;
      for dz in -8i32..=8 {
        for dx in -8i32..=8 {
          let d=(dx*dx+dz*dz)*64;
          if d<best && land(seed,x+dx*8,z+dz*8)!=on_land {best=d;}
        }
      }
      ((best as f64).sqrt()-6.0).max(0.0)
    });
    let tx=x.rem_euclid(8) as f64/8.0;
    let tz=z.rem_euclid(8) as f64/8.0;
    (at(gx,gz)*(1.0-tx)+at(gx+8,gz)*tx)*(1.0-tz)
      +(at(gx,gz+8)*(1.0-tx)+at(gx+8,gz+8)*tx)*tz
  });
  let cliff=((field(seed^0x831f,x,z,448)-0.62)/0.20).clamp(0.0,1.0);
  (distance,cliff*cliff*(3.0-2.0*cliff))
}

// Cache exact coastline samples across neighboring beach columns.
thread_local! {
  static BEACH_LAND: std::cell::RefCell<lru::LruCache<(u64,i32,i32), bool>> =
    std::cell::RefCell::new(lru::LruCache::new(std::num::NonZeroUsize::new(4096).unwrap()));
}

pub fn beach_distance(seed: u64, x: i32, z: i32, radius: i32, on_land: bool) -> f64 {
  BEACH_LAND.with(|cache| {
    let mut cache = cache.borrow_mut();
    let mut best = radius * radius;
    for dz in -radius..=radius {
      for dx in -radius..=radius {
        let squared = dx*dx + dz*dz;
        if squared < best && *cache.get_or_insert((seed,x+dx,z+dz), || land(seed,x+dx,z+dz)) != on_land {
          best = squared;
        }
      }
    }
    (best as f64).sqrt()
  })
}
