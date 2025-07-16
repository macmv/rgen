use std::collections::HashMap;
use rgen_base::{BlockState, BlockFilter, Pos};
use rgen_world::{PartialWorld, UndoError};
use crate::{rng, Placer, Random, Result, Rng};

pub struct BetterBush {
    pub place_above: BlockFilter,
    pub trunk: BlockState,
    pub leaves: BlockState,
    pub avg_per_chunk: f64,
}

impl Default for BetterBush {
    fn default() -> Self {
        Self {
            place_above: block![grass].into(),
            trunk: block!(log[variant="jungle"]),
            leaves: block![leaves[variant = "jungle", check_decay = false, decayable = true]],
            avg_per_chunk: 30.0, //12
        }
    }
}

impl Placer for BetterBush {
    fn radius(&self) -> u8 { 10 }

    fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

    fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
        if pos.y + 24 >= 255 || pos.y <= 1 {
            return Err(UndoError);
        }

        let below = pos + Pos::new(0, -1, 0);
        if !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
            return Err(UndoError);
        }
        if self.can_place_bush(world, pos){
            self.bush_placer(world, rng, pos);
        }
        Ok(())
    }
}

impl BetterBush {
    fn can_place_bush(&self, world: &PartialWorld, base: Pos) -> bool {
        for dy in 0..2 {
            for dz in -1..=1 {
                for dx in -1..=1 {
                    let pos = base + Pos::new(dx, dy, dz);
                    let block = world.get(pos);
                    if block != block![air] && block != self.leaves {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn bush_placer(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) {
        let bottoms = vec![
            vec!["-ll--", "-lll-", "-lcll", "-lll-", "-----"], // uses top 1
            vec!["--ll-", "-llll", "-lcll", "-lll-", "-----"], // uses top 2
            vec!["--l--", "-llll", "-lcll", "-lll-", "-----"], // uses top 1
            vec!["-----", "-lll-", "-lcl-", "-lll-", "-----"], // uses top 3
            vec!["-ll--", "-ll--", "-lcl-", "--ll-", "-----"], // uses top 1
            vec!["--l--", "-lll-", "-lcl-", "-ll--", "-----"], // uses top 4
        ];

        let tops = vec![
            vec!["-l-", "lcl", "-l-"],  // top 1
            vec!["-ll", "lcl", "-l-"],  // top 2
            vec!["-l-", "lcl", "ll-"],  // top 3
            vec!["-l-", "lc-", "-l-"],  // top 4
        ];

        // mapping bottom index to top index
        let top_map = [0, 1, 0, 2, 0, 3];

        let index = rng.range(0..bottoms.len() as i32) as usize;
        let rotation = rng.range(0..=4) as usize;
        //let bottom_idx = rng.range(0..bottoms.len() as i32) as usize;
        //let rotation = rng.range(0..4) as usize;

        let bottom_layout = rotate_layout(&bottoms[index], rotation);
        let top_layout = rotate_layout(&tops[top_map[index]], rotation);

        let base_y = base.y;

        self.place_layer(world, &bottom_layout, base.x, base_y, base.z, true,rng);
        self.place_layer(world, &top_layout, base.x, base_y + 1, base.z, false,rng);
    }

    fn place_layer(&self, world: &mut PartialWorld, layout: &[String], cx: i32, y: i32, cz: i32, is_bottom: bool,rng: &mut Rng) {
        let size = layout.len() as i32;
        let offset = size / 2;
        for (dz, row) in layout.iter().enumerate() {
            for (dx, ch) in row.chars().enumerate() {
        //for (dz, row) in layout.iter().enumerate() {
        //    for (dx, ch) in row.chars().enumerate() {
                let x = cx + dx as i32 - offset;
                let z = cz + dz as i32 - offset;
                let pos = Pos::new(x, y, z);

                match ch {
                    'l' => {
                        if world.get(pos)==block!(air){
                            world.set(pos, self.leaves);
                        }
                        
                    }
                    'c' => {
                        world.set(pos, if is_bottom { self.trunk } else { self.leaves });
                        if !is_bottom && rng.range(0..=3)==0{
                            world.set(pos+Pos::new(0, 1, 0), self.leaves)
                        }
                    }
                    _ => {} // skip '-'
                }
            }
        }
    }
}

/// Rotates a 2D layout 90° clockwise n times (n ∈ 0..=3)
fn rotate_layout(layout: &[&str], turns: usize) -> Vec<String> {
    let mut matrix: Vec<Vec<char>> = layout.iter().map(|row| row.chars().collect()).collect();
    for _ in 0..turns {
        matrix = rotate_90(&matrix);
    }
    matrix.into_iter().map(|row| row.into_iter().collect()).collect()
}


/// Helper: rotate a 2D char matrix 90° clockwise
fn rotate_90(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut new_matrix = vec![vec!['-'; n]; m];

    for i in 0..n {
        for j in 0..m {
            new_matrix[j][n - i - 1] = matrix[i][j];
        }
    }

    new_matrix
}
