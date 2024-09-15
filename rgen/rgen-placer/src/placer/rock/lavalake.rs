use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};
const GRID_SIZE: usize = 40;
const BLOB_SIZE: usize = 20;

pub struct LavaLake {
  pub place_above:  BlockFilter,
  pub material:     BlockState,
  pub avg_in_chunk: f64,
  pub fluid:        BlockState,
}

impl LavaLake {
  pub fn new(blocks: &Blocks) -> Self {
    LavaLake {
      place_above:  [blocks.stone.block, blocks.dirt.block].into(),
      material:     blocks.gold_block.default_state.into(),
      avg_in_chunk: 50.0,
      fluid:        blocks.lava.default_state.into(),
    }
  }
}

impl Placer for LavaLake {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }
    let grid = self.build_base(rng);
    for (row_index, row) in grid.iter().enumerate() {
      for (col_index, &cell) in row.iter().enumerate() {
        world.set(
          pos + Pos::new(5, 0, 5) + Pos::new(row_index as i32, 0, col_index as i32),
          self.material,
        )
      }
    }
    world.set(pos, self.fluid);
    world.set(pos + Pos::new(5, 0, 5), self.fluid)
  }
}

impl LavaLake {
  fn build_bolder(
    &self,
    world: &mut PartialWorld,
    pos: Pos,
    cell: bool,
    x: i32,
    height: i32,
    z: i32,
  ) {
    let rel_pos = pos + Pos::new(x as i32, height, z as i32);
    if cell {
      world.set(rel_pos, self.material);
    }
  }
  fn build_base(&self, rng: &mut Rng) -> [[i32; 20]; 20] {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
    let mut blob_cells = vec![(GRID_SIZE / 2, GRID_SIZE / 2)];
    grid[GRID_SIZE / 2][GRID_SIZE / 2] = 1;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Grow the blob until it reaches the desired size
    let mut i = 0;
    while blob_cells.len() < BLOB_SIZE || i > 100 {
      i += 1;
      // Randomly pick an active cell from the blob
      let (current_row, current_col) =
        blob_cells[rng.rand_inclusive(0, blob_cells.len() as i32) as usize];

      // Randomly select a direction to grow
      let (row_offset, col_offset) = directions[rng.rand_inclusive(0, 4) as usize];
      let new_row = current_row as isize + row_offset;
      let new_col = current_col as isize + col_offset;

      // Check if the new cell is within bounds and not already part of the blob
      if new_row >= 0
        && new_row < GRID_SIZE as isize
        && new_col >= 0
        && new_col < GRID_SIZE as isize
      {
        let new_row = new_row as usize;
        let new_col = new_col as usize;

        if grid[new_row][new_col] == 0 {
          // Mark the new cell as part of the blob
          grid[new_row][new_col] = 1;
          blob_cells.push((new_row, new_col));
        }
      }
    }
    return grid;
  }
}
