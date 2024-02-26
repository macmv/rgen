use rgen_base::{Blocks, ChunkPos};

mod api;
mod biome;
mod ctx;
mod generator;
mod noise;

pub struct ChunkContext<'a> {
  pub chunk_pos: ChunkPos,
  pub blocks:    &'a Blocks,
}
