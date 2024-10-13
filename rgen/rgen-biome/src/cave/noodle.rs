use rgen_base::{block, Block, Chunk, ChunkPos, Pos, StateId};
use rgen_placer::{
  grid::PointGrid,
  noise::{NoiseGenerator3D, OctavedNoise, PerlinNoise},
};
use rgen_world::BlockInfoSupplier;

/// Noodle caves are the long thin tunnels, the "normal" caves.
pub struct NoodleCarver {
  seed: u64,
  grid: PointGrid,

  density_map: OctavedNoise<PerlinNoise, 2>,

  water: StateId,
}

#[derive(Clone)]
struct NoodleCave<'a> {
  carver: &'a NoodleCarver,
  pos:    (f64, f64, f64),
  origin: (f64, f64, f64),

  // Note that this is re-created for every chunk that this cave could appear in, so it must be
  // fast to create. This is why it still uses perlin noise, as the creation time for open simplex
  // noise is too slow.
  radius_map:  OctavedNoise<PerlinNoise, 2>,
  delta_x_map: OctavedNoise<PerlinNoise, 2>,
  delta_y_map: OctavedNoise<PerlinNoise, 2>,
  delta_z_map: OctavedNoise<PerlinNoise, 2>,

  // -1.0 or 1.0
  direction: f64,
}

const CAVE_RADIUS: i32 = 96;
// FIXME: This needs to be as much as 30 blocks smaller than the radius to avoid
// all the chunk border artifacts. Need to figure out whats going on with this.
const MAX_CAVE_AREA: f64 = CAVE_RADIUS as f64 - 4.0;

impl NoodleCarver {
  pub fn new(info: &impl BlockInfoSupplier, seed: u64) -> Self {
    NoodleCarver {
      seed,

      grid: PointGrid::new(),
      density_map: OctavedNoise::new(seed, 1.0 / 16.0),

      water: info.encode(block![water]),
    }
  }

  pub fn carve(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    let scale = 48.0;

    let min_pos = chunk_pos.min_block_pos();
    let cave_min_x = ((min_pos.x - CAVE_RADIUS) as f64) / scale;
    let cave_min_z = ((min_pos.z - CAVE_RADIUS) as f64) / scale;
    let cave_max_x = ((min_pos.x + 16 + CAVE_RADIUS) as f64) / scale;
    let cave_max_z = ((min_pos.z + 16 + CAVE_RADIUS) as f64) / scale;

    let points =
      self.grid.points_in_area(self.seed, cave_min_x, cave_min_z, cave_max_x, cave_max_z);
    for point in points {
      let pos = ((point.0 * scale), 32.0, (point.1 * scale));

      // A seed unique to this cave.
      let cave_seed = self.seed
        ^ (((pos.0 * 2048.0).round() as u64) << 8)
        ^ (((pos.2 * 2048.0).round() as u64) << 16);

      let mut cave = NoodleCave {
        carver: self,
        pos,
        origin: pos,
        radius_map: OctavedNoise::new(cave_seed, 1.0 / 64.0),
        delta_x_map: OctavedNoise::new(cave_seed.wrapping_add(1), 1.0 / 64.0),
        delta_y_map: OctavedNoise::new(cave_seed.wrapping_add(2), 1.0 / 64.0),
        delta_z_map: OctavedNoise::new(cave_seed.wrapping_add(3), 1.0 / 64.0),
        direction: 1.0,
      };

      let mut cave2 = cave.clone();
      cave2.direction = -1.0;

      for _ in 0..100 {
        if cave.dig(chunk, chunk_pos) {
          break;
        }
      }
      for _ in 0..100 {
        if cave2.dig(chunk, chunk_pos) {
          break;
        }
      }
    }
  }
}

impl NoodleCave<'_> {
  fn radius(&self) -> f64 {
    (self.radius_map.generate_3d(self.pos.0, self.pos.1, self.pos.2) * 0.5 + 0.5) * 4.0 + 1.0
  }

  fn dig(&mut self, chunk: &mut Chunk, chunk_pos: ChunkPos) -> bool {
    let dx = self.delta_x_map.generate_3d(self.pos.0, self.pos.1, self.pos.2) * self.direction;
    let dy = self.delta_y_map.generate_3d(self.pos.0, self.pos.1, self.pos.2) * self.direction;
    let dz = self.delta_z_map.generate_3d(self.pos.0, self.pos.1, self.pos.2) * self.direction;

    let dy = dy / 2.0;

    let radius = self.radius();

    self.dig_delta(chunk_pos, chunk, radius, dx, dy, dz)
  }

  fn dig_delta(
    &mut self,
    chunk_pos: ChunkPos,
    chunk: &mut Chunk,
    radius: f64,
    dx: f64,
    dy: f64,
    dz: f64,
  ) -> bool {
    let radius_squared = (radius * radius).round() as i32;
    let max_radius = radius.ceil() as i32;

    for _ in 0..5 {
      self.pos.0 += dx;
      self.pos.1 += dy;
      self.pos.2 += dz;

      if self.pos.1 < 0.0 || self.pos.1 > 256.0 {
        return true;
      }
      if (self.pos.0 - self.origin.0).abs() > MAX_CAVE_AREA
        || (self.pos.2 - self.origin.2).abs() > MAX_CAVE_AREA
      {
        return true;
      }

      let pos = self.block_pos();

      let mut hit_water = false;
      for y in -max_radius..=max_radius {
        for z in -max_radius..=max_radius {
          for x in -max_radius..=max_radius {
            // Squish the cave a bit, so its wider than it is tall.
            let x0 = x * 4 / 5;
            let z0 = z * 4 / 5;
            let r = x0 * x0 + y * y + z0 * z0;
            if r > radius_squared {
              continue;
            }

            let dist_to_center = r as f64 / radius_squared as f64;

            let pos = Pos::new(pos.x + x, pos.y + y, pos.z + z);
            if pos.in_chunk(chunk_pos) {
              let density =
                self.carver.density_map.generate_3d(pos.x as f64, pos.y as f64, pos.z as f64) * 0.4
                  + 0.6;

              if density > dist_to_center {
                // TODO: This is pretty dumb. Maybe add a concept of "near water" so we can skip
                // this sometimes?
                let mut near_water = false;
                for offset in [
                  Pos::new(-1, 0, 0),
                  Pos::new(1, 0, 0),
                  Pos::new(0, 0, -1),
                  Pos::new(0, 0, 1),
                  Pos::new(0, 1, 0),
                  Pos::new(0, -1, 0),
                ] {
                  let pos = pos + offset;
                  // Chunk borders: we don't care! We can let a bit of floating water exist.
                  if !pos.in_chunk(chunk_pos) {
                    continue;
                  }

                  let block = chunk.get(pos.chunk_rel());

                  if block == self.carver.water {
                    near_water = true;
                    break;
                  }
                }

                if near_water {
                  hit_water = true;
                } else {
                  chunk.set(pos.chunk_rel(), Block::AIR);
                }
              }
            }
          }
        }
      }
      if hit_water {
        return true;
      }
    }
    false
  }

  fn block_pos(&self) -> Pos { Pos::new(self.pos.0 as i32, self.pos.1 as i32, self.pos.2 as i32) }
}
