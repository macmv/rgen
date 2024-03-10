// FIXME: This really shouldn't live here.

use rgen_base::{Blocks, Chunk, ChunkPos, Pos};
use rgen_world::{Context, Generator, PartialWorld};

pub struct TerrainGenerator {
  pub seed: u64,

  pub biomes: rgen_biome::WorldBiomes,
}

impl TerrainGenerator {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes, seed: u64) -> TerrainGenerator {
    TerrainGenerator { seed, biomes: rgen_biome::WorldBiomes::new(blocks, biome_ids) }
  }
}

impl Generator for TerrainGenerator {
  fn height_at(&self, pos: Pos) -> f64 { self.biomes.height_at(pos) }

  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.biomes.generate_base(self.seed, ctx, chunk, chunk_pos);
  }

  fn decorate(&self, ctx: &Context, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    self.biomes.decorate(&ctx.blocks, self.seed, world, chunk_pos);

    world.set(chunk_pos.min_block_pos() + Pos::new(0, 6, 0), ctx.blocks.dirt.block);
  }
}
