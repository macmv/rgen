use std::hint;

use rgen_base::{BlockState, BlockFilter, Pos};
use rgen_world::{PartialWorld, UndoError};
use crate::{rng, Placer, Random, Result, Rng};

pub struct BetterBush {
    pub place_above: BlockFilter,
    pub trunk: BlockState,
    pub leaves: BlockState,



}

impl Default for BetterBush {
    fn default() -> Self {
        Self {
            place_above: block![grass].into(),
            trunk: block!(log[variant="jungle"]), //block![concrete[color="red"]],
            leaves: block![leaves[variant = "jungle", check_decay = false, decayable = true]],
        }
    }
}

impl Placer for BetterBush {
    fn radius(&self) -> u8 { 10 }

    fn avg_per_chunk(&self) -> f64 { 2.0 }

    fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
        if pos.y + 24 >= 255 || pos.y <= 1 {
            return Err(UndoError);
        }

        let below = pos + Pos::new(0, -1, 0);
        if !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
            return Err(UndoError);
        }

        //self.place_blob_tower(world, rng, pos);
        self.bush_placer(world, rng, pos);

        
        Ok(())
    }
}

impl BetterBush {

    fn bush_placer(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) {
    // Define bush bottom layouts
    let bottoms = vec![
        vec!["ll--", "lll-", "lcll", "lll-"], // Top 1
        vec!["-ll-", "llll", "lcll", "lll-"], // Top 2
        vec!["-l--", "llll", "lcll", "lll-"], // Top 1
        vec!["----", "lll-", "lcl-", "lll-"], // Top 3
        vec!["ll--", "ll--", "lcl-", "-ll-"], // Top 1
        vec!["-l--", "lll-", "lcl-", "ll--"], // Top 4
    ];

    let tops = vec![
        vec!["-l-", "lcl", "-l-"],  // For bottoms 1, 3, 5
        vec!["-ll", "lcl", "-l-"],  // For bottom 2
        vec!["-l-", "lcl", "ll-"],  // For bottom 4
        vec!["-l-", "lc-", "-l-"],  // For bottom 6
    ];

    // Random bottom index
    let bottom_idx = rng.range(0..bottoms.len() as i32) as usize;
    let rotation = rng.range(0..4) as usize;
    let bottom_layout = Self::rotate_layout(&bottoms[bottom_idx], rotation);
    let top_layout = Self::rotate_layout(&tops[Self::map_bottom_to_top(bottom_idx)], rotation);

    // Place bottom
    Self::place_layout(world, &bottom_layout, base, 0, self.leaves);

    // Place top
    Self::place_layout(world, &top_layout, base, 1, self.leaves);
    }

    fn rotate_layout(layout: &[&str], times: usize) -> Vec<Vec<char>> {
        let mut grid: Vec<Vec<char>> = layout.iter().map(|&row| row.chars().collect()).collect();
        for _ in 0..times {
            grid = (0..grid[0].len())
                .map(|i| grid.iter().rev().map(|row| row[i]).collect())
                .collect();
        }
        grid
    }

    fn map_bottom_to_top(idx: usize) -> usize {
        match idx {
            0 | 2 | 4 => 0,
            1 => 1,
            3 => 2,
            5 => 3,
            _ => 0,
        }
    }

    fn place_layout(world: &mut PartialWorld, layout: &[Vec<char>], base: Pos, dy: i32, leaf_block: BlockState) {
        let offset_y = dy;
        let offset_x = -(layout[0].len() as i32 / 2);
        let offset_z = -(layout.len() as i32 / 2);

        for (z, row) in layout.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let pos = base + Pos::new(offset_x + x as i32, offset_y, offset_z + z as i32);
                if c == 'l' || c == 'c' {
                    if world.get(pos) == block![air] {
                        world.set(pos, leaf_block);
                    }
                }
            }
        }
    }


}