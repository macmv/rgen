use rgen_base::{Chunk, ChunkPos, Pos, StateId, block};
use rgen_placer::noise::{NoiseGenerator3D, OctavedNoise, PerlinNoise};
use rgen_world::BlockInfoSupplier;

use crate::WorldBiomes;

/// Cheese caves are the big caverns.
pub struct CheeseCarver {
  cave_map: OctavedNoise<PerlinNoise, 3>,

  water: StateId,
}

impl CheeseCarver {
  pub fn new(info: &BlockInfoSupplier, seed: u64) -> Self {
    CheeseCarver {
      cave_map: OctavedNoise::new(seed, 1.0 / 64.0),

      water: info.encode(block![water]),
    }
  }

  pub fn carve(&self, world: &WorldBiomes, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    profile_function!();

    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        let info = world.height_info(pos);
        let height = info.min_height();

        // The closer to the river we are, the higher this number is.
        let river_closeness = 1.0 - world.sample_river_distance(pos);

        for y in 0..=height as i32 {
          let pos = pos.with_y(y);
          let noise =
            self.cave_map.generate_3d(pos.x as f64, pos.y as f64 * 4.0, pos.z as f64) * 0.5 + 0.5;

          // Scale down caves towards the surface, to make narrow entraces that widen into
          // larger caves.
          let surface_modifier = match y as f64 - height {
            -8.0.. => 0.0,
            v @ -16.0..=-8.0 => 1.0 - (v + 16.0) / 8.0,
            _ => 1.0,
          };

          // Scale down caves towards bedrock, because bedrock is ugly, and we'd like to
          // hide it under normal stone.
          let bedrock_modifier = if y < 10 { y as f64 / 10.0 } else { 1.0 };

          // Rivers have more impact the higher the cave is.
          let river_modifier = if y < 40 {
            0.0
          } else {
            let height_modifier = (y as f64 - 40.0) / (height - 40.0);
            river_closeness * height_modifier
          };
          let river_modifier = 1.0 - river_modifier;

          if noise < 0.3 * surface_modifier * bedrock_modifier * river_modifier {
            let mut near_water = false;
            for offset in [
              Pos::new(0, 0, 0),
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

              if block == self.water {
                near_water = true;
                break;
              }
            }

            if !near_water {
              chunk.set(pos.chunk_rel(), StateId::AIR);
            }
          }
        }
      }
    }
  }
}
