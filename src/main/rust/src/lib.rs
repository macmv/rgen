use ctx::Blocks;

mod api;
mod chunk;
mod ctx;
mod generator;
mod noise;
mod pos;

pub struct ChunkContext<'a> {
  pub chunk_x: i32,
  pub chunk_z: i32,
  pub blocks:  &'a Blocks,
}
