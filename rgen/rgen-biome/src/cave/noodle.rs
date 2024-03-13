use rgen_base::{Block, Chunk, ChunkPos, Pos};
use rgen_placer::{
  grid::PointGrid,
  noise::{NoiseGenerator3D, OctavedNoise, PerlinNoise},
};

use crate::biome::IdContext;

/// Noodle caves are the long thin tunnels, the "normal" caves.
pub struct NoodleCarver {
  grid: PointGrid,

  cave_map:    OctavedNoise<PerlinNoise>,
  density_map: OctavedNoise<PerlinNoise>,
}

struct NoodleCave<'a> {
  carver:    &'a NoodleCarver,
  pos:       (f64, f64, f64),
  cave_seed: u64,
}

impl NoodleCarver {
  pub fn new(_ctx: &IdContext) -> Self {
    NoodleCarver {
      grid:        PointGrid::new(),
      cave_map:    OctavedNoise { octaves: 2, freq: 1.0 / 64.0, ..Default::default() },
      density_map: OctavedNoise { octaves: 2, freq: 1.0 / 16.0, ..Default::default() },
    }
  }

  pub fn carve(&self, seed: u64, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    let radius = 100;
    let scale = 48.0;

    let min_pos = chunk_pos.min_block_pos();
    let cave_min_x = ((min_pos.x - radius) as f64) / scale;
    let cave_min_z = ((min_pos.z - radius) as f64) / scale;
    let cave_max_x = ((min_pos.x + 16 + radius) as f64) / scale;
    let cave_max_z = ((min_pos.z + 16 + radius) as f64) / scale;

    let points = self.grid.points_in_area(seed, cave_min_x, cave_min_z, cave_max_x, cave_max_z);
    for point in points {
      let pos = ((point.0 * scale), 64.0, (point.1 * scale));

      // A seed unique to this cave.
      let cave_seed =
        seed ^ (((pos.0 * 2048.0).round() as u64) << 8) ^ (((pos.2 * 2048.0).round() as u64) << 16);

      let mut cave = NoodleCave { carver: self, pos, cave_seed };

      for _ in 0..100 {
        cave.dig(chunk, chunk_pos);
      }
    }
  }
}

impl NoodleCave<'_> {
  fn radius(&self) -> f64 {
    (self.carver.cave_map.generate_3d(
      self.pos.0,
      self.pos.1,
      self.pos.2,
      self.cave_seed.wrapping_add(4),
    ) * 0.5
      + 0.5)
      * 4.0
      + 1.0
  }

  fn dig(&mut self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    let dx = self.carver.cave_map.generate_3d(
      self.pos.0,
      self.pos.1,
      self.pos.2,
      self.cave_seed.wrapping_add(1),
    );
    let dy = self.carver.cave_map.generate_3d(
      self.pos.0,
      self.pos.1,
      self.pos.2,
      self.cave_seed.wrapping_add(2),
    );
    let dz = self.carver.cave_map.generate_3d(
      self.pos.0,
      self.pos.1,
      self.pos.2,
      self.cave_seed.wrapping_add(3),
    );

    let dy = dy / 2.0;

    let radius = self.radius();

    self.dig_delta(chunk_pos, chunk, radius, dx, dy, dz);
  }

  fn dig_delta(
    &mut self,
    chunk_pos: ChunkPos,
    chunk: &mut Chunk,
    radius: f64,
    dx: f64,
    dy: f64,
    dz: f64,
  ) {
    let radius_squared = (radius * radius).round() as i32;
    let max_radius = radius.ceil() as i32;

    for _ in 0..5 {
      self.pos.0 += dx;
      self.pos.1 += dy;
      self.pos.2 += dz;

      let pos = self.block_pos();
      for y in -max_radius..=max_radius {
        for z in -max_radius..=max_radius {
          for x in -max_radius..=max_radius {
            let r = x * x + y * y + z * z;
            if r > radius_squared {
              continue;
            }
            let dist_to_center = r as f64 / radius_squared as f64;

            let pos = Pos::new(pos.x + x, pos.y + y, pos.z + z);
            if pos.in_chunk(chunk_pos) {
              let density = self.carver.density_map.generate_3d(
                pos.x as f64,
                pos.y as f64,
                pos.z as f64,
                self.cave_seed,
              ) * 0.4
                + 0.6;

              if density > dist_to_center {
                chunk.set(pos.chunk_rel(), Block::AIR);
              }
            }
          }
        }
      }
    }
  }

  fn block_pos(&self) -> Pos { Pos::new(self.pos.0 as i32, self.pos.1 as i32, self.pos.2 as i32) }
}
