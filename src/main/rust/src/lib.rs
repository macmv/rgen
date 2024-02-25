use ctx::Blocks;
use pos::ChunkPos;

mod api;
mod biome;
mod chunk;
mod ctx;
mod generator;
mod noise;
mod pos;

pub struct ChunkContext<'a> {
  pub chunk_pos: ChunkPos,
  pub blocks:    &'a Blocks,
}
