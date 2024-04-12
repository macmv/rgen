use rgen_base::{Blocks, ChunkPos};

mod api;
mod ctx;

pub struct ChunkContext<'a> {
  pub chunk_pos: ChunkPos,
  pub blocks:    &'a Blocks,
}
