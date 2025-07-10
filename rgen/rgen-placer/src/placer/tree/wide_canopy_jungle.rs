use rgen_base::{BlockState, BlockFilter, Pos};
use rgen_world::{PartialWorld, UndoError};
use crate::{Placer, Random, Result, Rng};

pub struct WideCanopyJungle {
    pub place_above: BlockFilter,
    pub trunk: BlockState,
    pub top:   BlockState,
    pub roots:   BlockState,
    pub moss_roots:   BlockState,

}

impl Default for WideCanopyJungle {
    fn default() -> Self {
        Self {
            place_above: block![grass].into(),
            //leaves[variant = "birch"]
            trunk: block![concrete[color="red"]],
            top: block![concrete[color="yellow"]],
            roots: block![concrete[color="green"]],
            moss_roots: block![concrete[color="lime"]],
        }
    }
}

impl Placer for WideCanopyJungle {
    fn radius(&self) -> u8 { 10 }

    fn avg_per_chunk(&self) -> f64 { 1.5 }

    fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
        if pos.y + 24 >= 255 || pos.y <= 1 {
            return Err(UndoError);
        }

        let below = pos + Pos::new(0, -1, 0);
        if !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
            return Err(UndoError);
        }

        
        self.place_root_lines(world, rng, pos);
        self.place_blob_tower(world, rng, pos);
        
        Ok(())
    }
}

impl WideCanopyJungle {

    fn place_blob_tower(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) {
        let height = rng.range(7..=8);
        let mut current_layer: std::collections::HashSet<(i32, i32)> = {
            let mut set = std::collections::HashSet::new();
            set.insert((0, 0)); // start with center block
            if rng.range(0..2) == 0 {
                set.insert((1, 0));
            }
            if rng.range(0..2) == 0 {
                set.insert((0, 1));
            }
            if rng.range(0..2) == 0 {
                set.insert((-1, 0));
            }
            if rng.range(0..2) == 0 {
                set.insert((0, -1));
            }
            set
        };

        let mut layers: Vec<std::collections::HashSet<(i32, i32)>> = Vec::new();

        for _ in 0..height {
            layers.push(current_layer.clone());

            let mut next_layer = std::collections::HashSet::new();

            for &(x, z) in &current_layer {
                for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    if rng.range(0..4) == 0 {
                        next_layer.insert((x + dx, z + dz));
                    }
                }

                next_layer.insert((x, z));
            }

            next_layer = next_layer
                .into_iter()
                .filter(|(x, z)| x.abs().max(z.abs()) <= 3)
                .collect();

            current_layer = next_layer;
        }

        // Place from bottom to top
        for (y, layer) in layers.iter().rev().enumerate() {
            for &(x, z) in layer {
                let pos = base + Pos::new(x, y as i32, z);
                if world.get(pos) == block![air] {
                    world.set(pos, self.trunk);
                }
            }
        }

        // Add yellow marker block at top center
        let top_y = layers.len() as i32;
        let top_pos = base + Pos::new(0, top_y, 0);
        if world.get(top_pos) == block![air] {
            world.set(top_pos, self.top);
        }
    }

    
    fn place_root_lines(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) {
    let directions = [
        (1, 0),   // east
        (1, 1),   // southeast
        (0, 1),   // south
        (-1, 1),  // southwest
        (-1, 0),  // west
        (-1, -1), // northwest
        (0, -1),  // north
        (1, -1),  // northeast
    ];

    for &(dx, dz) in directions.iter() {
        let mut pos = base;

        // Step 1 and 2 in the main direction
        for _ in 0..3 {
            pos = pos + Pos::new(dx, 0, dz);
            if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                world.set(pos, self.roots);
            }
        }

        // Calculate opposite direction to exclude it
        let opposite = (-dx, -dz);

        // Build list of allowed tail directions (exclude opposite)
        let tail_options: Vec<(i32, i32)> = directions
            .iter()
            .cloned()
            .filter(|&(x, z)| (x, z) != opposite)
            .collect();

        let &(tdx, tdz) = rng.choose(&tail_options);
        
        for _ in 0..rng.range(2..3) {
            pos = pos + Pos::new(tdx, 0, tdz);
            if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                world.set(pos, self.moss_roots);
            }
        }
    }
}





   }