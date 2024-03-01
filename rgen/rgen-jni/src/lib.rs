use rgen_base::{Blocks, ChunkPos};

mod api;
mod ctx;
mod generator;

pub struct ChunkContext<'a> {
  pub chunk_pos: ChunkPos,
  pub blocks:    &'a Blocks,
}
