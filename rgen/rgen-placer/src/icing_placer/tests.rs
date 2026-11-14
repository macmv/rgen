use super::*;
use crate::{BiomeCachedChunk, IcingPlacer, Rng, TemporaryBiome};
use rgen_base::{BlockData, BlockId, Chunk, ChunkPos, ChunkRelPos};
use rgen_world::BlockInfoSupplier;
fn registry() -> BlockInfoSupplier {
  let mut info = BlockInfoSupplier::default();
  for (i, state) in [
    block![air],
    block![grass],
    block![dirt],
    block![stone],
    block![gravel],
    block![sand],
    block![tallgrass],
    block![double_plant],
    block![mossy_cobblestone],
    block![water],
    block![clay],
    block![rgen:mud],
    block![rgen:reeds],
  ]
  .into_iter()
  .enumerate()
  {
    let id = BlockId(i as u16);
    info.lookup.insert(state.block, id);
    info.info.insert(
      id,
      BlockData {
        name:         state.block.name().into(),
        block:        Some(state.block),
        default_meta: 0,
        prop_types:   Default::default(),
        prop_values:  std::array::from_fn(|_| rgen_base::PropMapOwned::empty()),
      },
    );
  }
  info
}

fn fixture(
  info: &BlockInfoSupplier,
  height: i32,
  soil: rgen_base::BlockState,
  ceiling: bool,
) -> Chunk {
  let mut chunk = Chunk::new();
  for x in 0..16 {
    for z in 0..16 {
      let p = ChunkRelPos::new(x, height, z);
      chunk.set(p, info.encode(soil));
      chunk.add_surface(p);
      if ceiling {
        chunk.set(p.with_y(height + 2), info.encode(block![stone]));
      }
    }
  }
  chunk
}

#[test]
fn icing_checks_final_soil_biome_and_space() {
  let info = registry();
  let placers: Vec<Box<dyn IcingPlacer>> = vec![
    Box::new(Scatter {
      place_above:   block![grass].into(),
      place:         block![tallgrass[1]],
      avg_per_chunk: 10.0,
      attempts:      100,
    }),
    Box::new(DoubleScatter {
      place_above:   block![grass].into(),
      place_lower:   block![double_plant[2]],
      place_upper:   block![double_plant[10]],
      avg_per_chunk: 10.0,
      attempts:      100,
    }),
    Box::new(Clumps {
      place_above:   block![grass].into(),
      place:         block![tallgrass[1]],
      radius:        2..=4,
      attempts:      100,
      avg_per_chunk: 10.0,
    }),
    Box::new(GrassClumps {
      place_above:      block![grass].into(),
      place_short:      block![tallgrass[1]],
      place_tall_lower: block![double_plant[2]],
      place_tall_upper: block![double_plant[10]],
      radius:           2..=4,
      attempts:         100,
      avg_per_chunk:    10.0,
    }),
    Box::new(Spread {
      replace:       block![grass].into(),
      place:         block![tallgrass[1]],
      radius:        2..=4,
      avg_per_chunk: 10.0,
    }),
  ];
  for placer in &placers {
    for soil in [block![grass], block![gravel], block![stone]] {
      let original = fixture(&info, 64, soil, false);
      let mut result = original.clone();
      placer.place(
        &mut BiomeCachedChunk::new(&info, &mut result),
        &mut Rng::new(42),
        ChunkPos::new(-1, 0),
      );
      if soil == block![grass] {
        assert_ne!(result.data(), original.data());
      } else {
        assert_eq!(result.data(), original.data());
      }
      let mut repeated = original.clone();
      placer.place(
        &mut BiomeCachedChunk::new(&info, &mut repeated),
        &mut Rng::new(42),
        ChunkPos::new(-1, 0),
      );
      assert_eq!(result.data(), repeated.data());
      let mut inactive = original.clone();
      let mut cached = BiomeCachedChunk::new(&info, &mut inactive);
      cached.set_active(TemporaryBiome(1));
      placer.place(&mut cached, &mut Rng::new(42), ChunkPos::new(-1, 0));
      assert_eq!(inactive.data(), original.data());
    }
    // An overhead block must never be overwritten, even by double plants.
    let mut covered = fixture(&info, 64, block![grass], true);
    placer.place(
      &mut BiomeCachedChunk::new(&info, &mut covered),
      &mut Rng::new(42),
      ChunkPos::new(0, 0),
    );
    for x in 0..16 {
      for z in 0..16 {
        assert_eq!(covered.get(ChunkRelPos::new(x, 66, z)), info.encode(block![stone]));
        assert_ne!(covered.get(ChunkRelPos::new(x, 65, z)), info.encode(block![double_plant[2]]));
      }
    }
  }
}

#[test]
fn double_scatter_never_places_a_half_at_world_ceiling() {
  let info = registry();
  let placer = DoubleScatter {
    place_above:   block![grass].into(),
    place_lower:   block![double_plant[2]],
    place_upper:   block![double_plant[10]],
    avg_per_chunk: 10.0,
    attempts:      100,
  };
  let mut chunk = fixture(&info, 254, block![grass], false);
  let original = chunk.clone();
  placer.place(
    &mut BiomeCachedChunk::new(&info, &mut chunk),
    &mut Rng::new(42),
    ChunkPos::new(0, 0),
  );
  assert_eq!(chunk.data(), original.data());
}

#[test]
fn dried_grass_rejects_finished_rock_and_gravel() {
  let info = registry();
  let placer = DriedGrassClumps {
    place_above:   [block![grass], block![sand]].into(),
    radius:        2..=4,
    attempts:      100,
    avg_per_chunk: 10.0,
  };
  for soil in [block![stone], block![gravel]] {
    let mut chunk = fixture(&info, 64, soil, false);
    let original = chunk.clone();
    placer.place(
      &mut BiomeCachedChunk::new(&info, &mut chunk),
      &mut Rng::new(42),
      ChunkPos::new(0, 0),
    );
    assert_eq!(chunk.data(), original.data());
  }
}

#[test]
fn reeds_respect_finished_water_depth_soil_and_clearance() {
  let info = registry();
  for soil in [
    block![grass],
    block![dirt],
    block![sand],
    block![gravel],
    block![clay],
    block![rgen:mud],
    block![stone],
  ] {
    for depth in [1, 2, 5] {
      for shallow_only in [true, false] {
        for blocked in [0, 1, 2] {
          let mut chunk = fixture(&info, 60, soil, false);
          for x in 0..16 {
            for z in 0..16 {
              for y in 61..=60 + depth {
                chunk.set(ChunkRelPos::new(x, y, z), info.encode(block![water]));
              }
              if blocked > 0 {
                chunk.set(ChunkRelPos::new(x, 60 + depth + blocked, z), info.encode(block![stone]));
              }
            }
          }
          let original = chunk.clone();
          let reeds = Reeds {
            avg_per_chunk:          10.0,
            attempts:               256,
            must_be_one_block_deep: shallow_only,
          };
          reeds.place(
            &mut BiomeCachedChunk::new(&info, &mut chunk),
            &mut Rng::new(42),
            ChunkPos::new(-1, 0),
          );
          let allowed = soil != block![stone] && (!shallow_only || depth == 1) && blocked == 0;
          if !allowed {
            assert_eq!(chunk.data(), original.data());
            continue;
          }
          assert_ne!(chunk.data(), original.data());
          for x in 0..16 {
            for z in 0..16 {
              let lower = ChunkRelPos::new(x, 61 + depth, z);
              if chunk.get(lower) != rgen_base::StateId::AIR {
                assert_eq!(chunk.get(lower), info.encode(block![rgen:reeds].with_data(3)));
                assert_eq!(
                  chunk.get(lower.with_y(lower.y() + 1)),
                  info.encode(block![rgen:reeds].with_data(7))
                );
              }
              for y in 61..=60 + depth {
                assert_eq!(chunk.get(ChunkRelPos::new(x, y, z)), info.encode(block![water]));
              }
            }
          }
        }
      }
    }
  }
}

#[test]
fn reeds_skip_inactive_biomes_and_world_ceiling() {
  let info = registry();
  let reeds = Reeds { avg_per_chunk: 10.0, attempts: 256, ..Reeds::default() };
  assert!(reeds.must_be_one_block_deep);
  for (floor_y, inactive) in [(60, true), (252, false), (254, false)] {
    let mut chunk = fixture(&info, floor_y, block![gravel], false);
    for x in 0..16 {
      for z in 0..16 {
        chunk.set(ChunkRelPos::new(x, floor_y + 1, z), info.encode(block![water]));
      }
    }
    let original = chunk.clone();
    let mut cached = BiomeCachedChunk::new(&info, &mut chunk);
    if inactive {
      cached.set_active(TemporaryBiome(1));
    }
    reeds.place(&mut cached, &mut Rng::new(42), ChunkPos::new(0, 0));
    assert_eq!(chunk.data(), original.data());
  }
}
