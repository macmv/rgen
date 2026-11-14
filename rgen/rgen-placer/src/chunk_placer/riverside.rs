use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::{chunk_rng, count};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Lines the edge of rivers with gravel and cobblestone.
///
/// This never crosses a chunk border.
pub struct RiverSide {
  pub ground:        BlockFilter,
  pub material:      Vec<BlockState>,
  pub avg_per_chunk: f64,
  pub fluid:         BlockState,
}

impl Default for RiverSide {
  fn default() -> Self {
    RiverSide {
      ground:        [block![dirt], block![grass]].into(),
      material:      vec![
        block![gravel],
        block![gravel],
        block![rgen:mossy_cobblestone_rgen],
        block![cobblestone],
      ],
      avg_per_chunk: 3.0,
      fluid:         block![lava],
    }
  }
}

impl ChunkPlacer for RiverSide {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);

    for _ in 0..count(&mut rng, self.avg_per_chunk) {
      // The siding touches blocks up to 2 away from the center.
      let column = ChunkRelPos::new(rng.range(2..=13) as u8, 0, rng.range(2..=13) as u8);

      for i in 0..chunk.chunk.surfaces(column).len() {
        let pos = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
        if !chunk.is_active(pos) || pos.y() + 21 >= 255 || pos.y() < 1 {
          continue;
        }

        for rel_x in -1..=1_i32 {
          for rel_z in -1..=1_i32 {
            self.build_siding(chunk, &mut rng, offset(pos, rel_x, rel_z));
          }
        }
      }
    }
  }
}

fn offset(pos: ChunkRelPos, x: i32, z: i32) -> ChunkRelPos {
  ChunkRelPos::new((pos.x() as i32 + x) as u8, pos.y(), (pos.z() as i32 + z) as u8)
}

impl RiverSide {
  fn build_siding(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, pos: ChunkRelPos) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if !(rel_x == 0 && rel_z == 0)
          && chunk.get(offset(pos, rel_x, rel_z)) == block![water]
          && self.ground.contains(chunk.get(pos))
        {
          chunk.set(pos, *rng.choose(&self.material));
        }
      }
    }
  }
}
