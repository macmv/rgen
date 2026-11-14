use super::{RockSurface, SubsurfaceSplotch};
use crate::{BiomeCachedChunk, ChunkPlacer, Rng, TemporaryBiome};
use rgen_base::{BlockData, BlockId, Chunk, ChunkPos, ChunkRelPos};
use rgen_world::BlockInfoSupplier;

pub(super) fn registry() -> BlockInfoSupplier {
  let mut info = BlockInfoSupplier::default();
  for (i, state) in [
    block![air],
    block![grass],
    block![dirt],
    block![stone],
    block![gravel],
    block![sand],
    block![water],
    block![clay],
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

pub(super) fn terrain(info: &BlockInfoSupplier, height: i32) -> Chunk {
  let mut chunk = Chunk::new();
  for x in 0..16 {
    for z in 0..16 {
      for y in 0..=height {
        chunk.set(
          ChunkRelPos::new(x, y, z),
          info.encode(if y == height {
            block![grass]
          } else if y > height - 8 {
            block![dirt]
          } else {
            block![stone]
          }),
        );
      }
      chunk.add_surface(ChunkRelPos::new(x, height, z));
    }
  }
  chunk
}

#[test]
fn ground_pass_is_repeatable_and_respects_biomes_and_vertical_limits() {
  let info = registry();
  let placers: Vec<Box<dyn ChunkPlacer>> = vec![
    Box::new(RockSurface {
      size:          12,
      avg_per_chunk: 8.0,
      mix_in:        vec![block![stone]],
    }),
    Box::new(SubsurfaceSplotch {
      surface_replace: block![grass].into(),
      below_replace:   block![dirt].into(),
      place:           block![sand],
      below_mix:       vec![block![stone]],
      radius:          2..=4,
      depth:           1..=3,
      avg_per_chunk:   8.0,
      stretch_chance:  100,
    }),
  ];
  for placer in placers {
    for height in [2, 64, 252] {
      let original = terrain(&info, height);
      let mut a = original.clone();
      let mut b = original.clone();
      for chunk in [&mut a, &mut b] {
        placer.place(
          &mut BiomeCachedChunk::new(&info, chunk),
          &mut Rng::new(42),
          ChunkPos::new(-1, 0),
        );
      }
      assert_eq!(a.data(), b.data());
      assert_ne!(a.data(), original.data());
      for x in 0..16 {
        for z in 0..16 {
          for y in height + 1..256 {
            assert_eq!(a.get(ChunkRelPos::new(x, y, z)), rgen_base::StateId::AIR);
          }
          if height == 64 {
            assert_eq!(a.get(ChunkRelPos::new(x, 55, z)), original.get(ChunkRelPos::new(x, 55, z)));
          }
        }
      }
      let mut inactive = original.clone();
      let mut cached = BiomeCachedChunk::new(&info, &mut inactive);
      cached.set_active(TemporaryBiome(1));
      placer.place(&mut cached, &mut Rng::new(42), ChunkPos::new(-1, 0));
      assert_eq!(inactive.data(), original.data());
    }
  }
}
