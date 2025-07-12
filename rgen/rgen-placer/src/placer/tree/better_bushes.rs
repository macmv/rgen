use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};
use crate::{Placer, Random, Result, Rng};

pub struct BetterBush {
    pub place_above: BlockFilter,
    pub trunk: BlockState,
    pub leaves: BlockState,
}

impl Default for BetterBush {
    fn default() -> Self {
        Self {
            place_above: block![grass].into(),
            trunk: block!(log[variant = "jungle"]),
            leaves: block![leaves[variant = "jungle", check_decay = false, decayable = true]],
        }
    }
}

impl Placer for BetterBush {
    fn radius(&self) -> u8 {
        10
    }

    fn avg_per_chunk(&self) -> f64 {
        2.0
    }

    fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
        if pos.y + 24 >= 255 || pos.y <= 1 {
            return Err(UndoError);
        }

        let below = pos + Pos::new(0, -1, 0);
        if !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
            return Err(UndoError);
        }

        self.bush_placer(world, rng, pos);
        Ok(())
    }
}

impl BetterBush {
    fn bush_placer(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) {
        let bottoms = vec![
            vec!["ll--", "lll-", "lcll", "lll-"],
            vec!["-ll-", "llll", "lcll", "lll-"],
            vec!["-l--", "llll", "lcll", "lll-"],
            vec!["----", "lll-", "lcl-", "lll-"],
            vec!["ll--", "ll--", "lcl-", "-ll-"],
            vec!["-l--", "lll-", "lcl-", "ll--"],
        ];

        let tops = vec![
            vec!["-l-", "lcl", "-l-"],
            vec!["-ll", "lcl", "-l-"],
            vec!["-l-", "lcl", "ll-"],
            vec!["-l-", "lc-", "-l-"],
        ];

        let bottom_idx = rng.range(0..bottoms.len() as i32) as usize;
        let rotation = rng.range(0..4) as usize;

        let (bottom_layout, bottom_center) = Self::rotate_layout_around_center(&bottoms[bottom_idx], rotation);
        let (top_layout, _top_center) = Self::rotate_layout_around_center(&tops[Self::map_bottom_to_top(bottom_idx)], rotation);

        let true_base = base - Pos::new(bottom_center.0, 0, bottom_center.1);

        self.place_layout(world, &bottom_layout, true_base, 0, false);
        self.place_layout(world, &top_layout, true_base, 1, true);

    }

    fn place_layout(&self, world: &mut PartialWorld, layout: &[Vec<char>], base: Pos, dy: i32, is_top: bool) {
        for (z, row) in layout.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                let pos = base + Pos::new(x as i32, dy, z as i32);

                match ch {
                    'l' => {
                        if world.get(pos) == block![air] {
                            world.set(pos, self.leaves);
                        }
                    }
                    'c' => {
                        if world.get(pos) == block![air] {
                            let block_to_place = if is_top { self.leaves } else { self.trunk };
                            world.set(pos, block_to_place);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn rotate_layout_around_center(layout: &[&str], times: usize) -> (Vec<Vec<char>>, (i32, i32)) {
        let mut grid: Vec<Vec<char>> = layout.iter().map(|&row| row.chars().collect()).collect();
        let mut height = grid.len();
        let mut width = grid[0].len();

        let mut cx = 0;
        let mut cz = 0;
        for (z, row) in grid.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch == 'c' {
                    cx = x as i32;
                    cz = z as i32;
                    break;
                }
            }
        }

        for _ in 0..times {
            grid = (0..width)
                .map(|x| grid.iter().rev().map(|row| row[x]).collect())
                .collect();
            let temp = cx;
            cx = (height as i32 - 1) - cz;
            cz = temp;
            std::mem::swap(&mut width, &mut height);
        }

        (grid, (cx, cz))
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
}
