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
fn field(seed: u64, x: i32, z: i32, scale: i32) -> f64 {
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
  })
}

// One bounded cache per generator worker. Adjacent grid squares share corners.
thread_local! {
  static RELIEF: std::cell::RefCell<lru::LruCache<(u64, i32, i32), f64>> =
    std::cell::RefCell::new(lru::LruCache::new(std::num::NonZeroUsize::new(1024).unwrap()));
}

pub const ELEVATION_SPACING: i32 = 64;

pub fn broad_land_height(seed: u64, x: i32, z: i32) -> f64 {
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
        let belt = field(seed ^ 0x73ac, x, z, 2048);
        let ridge = (1.0 - (2.0 * field(seed ^ 0x59b1, x, z, 1024) - 1.0).abs()).powi(3);
        // Offset the smaller fields from the planning lattice to break up broad slopes.
        let hills = field(seed ^ 0x821d, x + 71, z - 43, 320) - 0.5;
        let local = field(seed ^ 0xcb47, x - 37, z + 19, 144) - 0.5;
        let relief = 30.0 * plains.powi(2) + 110.0 * belt.powi(2) * ridge;
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
