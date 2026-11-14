use crate::{
  BiomeCachedChunk, IcingPlacer, Random, Rng,
  chunk_placer::util::{chunk_rng, count},
};
use rgen_base::{ChunkPos, ChunkRelPos};

/// Glasswort on finished mud near water inside the current chunk only.
pub struct Glasswort {
  pub avg_per_chunk: f64,
  pub attempts:      u32,
}

impl Default for Glasswort {
  fn default() -> Self { Self { avg_per_chunk: 1.0, attempts: 320 } }
}

impl IcingPlacer for Glasswort {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);
    // Match the other icing scatter placers: old vertical jitter landed on
    // level ground roughly one attempt in nine; now target the surface directly.
    for _ in 0..count(&mut rng, self.avg_per_chunk * f64::from(self.attempts) / 9.0) {
      let column = ChunkRelPos::new(rng.range(0..16) as u8, 0, rng.range(0..16) as u8);
      for i in 0..chunk.chunk.surfaces(column).len() {
        let ground = column.with_y(i32::from(chunk.chunk.surfaces(column)[i]));
        if ground.y() >= 255 {
          continue;
        }
        let plant = ground.with_y(ground.y() + 1);
        // Reject unsuitable ground/occupied spots before searching for water.
        if !chunk.is_active(ground)
          || chunk.get(ground) != block![rgen:mud]
          || chunk.get(plant) != block![air]
        {
          continue;
        }
        if water_within_three_blocks(chunk, plant) {
          chunk.set(plant, block![rgen:glasswort].with_data(rng.range(0..=5) as u8));
        }
      }
    }
  }
}

fn water_within_three_blocks(chunk: &BiomeCachedChunk, plant: ChunkRelPos) -> bool {
  for ring in 1_i32..=3 {
    for dx in -ring..=ring {
      for dz in -ring..=ring {
        if dx.abs().max(dz.abs()) != ring || dx * dx + dz * dz > 9 {
          continue;
        }
        let x = i32::from(plant.x()) + dx;
        let z = i32::from(plant.z()) + dz;
        // No wrapping or neighbor lookup: water outside the chunk does not count.
        if !(0..16).contains(&x) || !(0..16).contains(&z) {
          continue;
        }
        for y in [plant.y(), plant.y() - 1] {
          if (0..256).contains(&y)
            && chunk.get(ChunkRelPos::new(x as u8, y, z as u8)) == block![water]
          {
            return true;
          }
        }
      }
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::{super::tests::registry, *};
  use crate::TemporaryBiome;
  use rgen_base::Chunk;

  #[test]
  fn near_water_stays_inside_chunk_and_preserves_three_block_rule() {
    let info = registry();
    let mut chunk = Chunk::new();
    chunk.set(ChunkRelPos::new(15, 64, 8), info.encode(block![water]));
    let cached = BiomeCachedChunk::new(&info, &mut chunk);
    assert!(!water_within_three_blocks(&cached, ChunkRelPos::new(0, 65, 8)));
    assert!(water_within_three_blocks(&cached, ChunkRelPos::new(12, 65, 8)));
    assert!(!water_within_three_blocks(&cached, ChunkRelPos::new(11, 65, 8)));
    assert!(!water_within_three_blocks(&cached, ChunkRelPos::new(12, 65, 9)));
    assert!(water_within_three_blocks(&cached, ChunkRelPos::new(14, 64, 8)));
    assert!(!water_within_three_blocks(&cached, ChunkRelPos::new(14, 66, 8)));
  }

  #[test]
  fn glasswort_uses_finished_mud_empty_space_and_all_variants() {
    let info = registry();
    let placer = Glasswort { avg_per_chunk: 20.0, attempts: 320 };
    let mut variants = std::collections::HashSet::new();
    for mode in 0..5 {
      let mut chunk = Chunk::new();
      for x in 0..16 {
        for z in 0..16 {
          let ground = ChunkRelPos::new(x, 64, z);
          chunk.add_surface(ground);
          chunk.set(ground, info.encode(if mode == 1 { block![grass] } else { block![rgen:mud] }));
          if mode != 2 && x == 8 {
            chunk.set(ground, info.encode(block![water]));
          }
          if mode == 3 {
            chunk.set(ground.with_y(65), info.encode(block![stone]));
          }
        }
      }
      let original = chunk.clone();
      let mut cached = BiomeCachedChunk::new(&info, &mut chunk);
      if mode == 4 {
        cached.set_active(TemporaryBiome(1));
      }
      placer.place(&mut cached, &mut Rng::new(42), ChunkPos::new(-1, 0));
      if mode != 0 {
        assert_eq!(chunk.data(), original.data());
        continue;
      }
      assert_ne!(chunk.data(), original.data());
      let mut repeated = original.clone();
      placer.place(
        &mut BiomeCachedChunk::new(&info, &mut repeated),
        &mut Rng::new(42),
        ChunkPos::new(-1, 0),
      );
      assert_eq!(chunk.data(), repeated.data());
      for x in 0..16 {
        for z in 0..16 {
          let ground = ChunkRelPos::new(x, 64, z);
          assert_eq!(chunk.get(ground), original.get(ground));
          let state = chunk.get(ground.with_y(65));
          if state != rgen_base::StateId::AIR {
            assert!((5..=11).contains(&x) && x != 8);
            assert_eq!(state.block(), info.encode(block![rgen:glasswort]).block());
            assert!(state.meta() <= 5);
            variants.insert(state.meta());
          }
        }
      }
    }
    assert_eq!(variants.len(), 6);
  }
}
