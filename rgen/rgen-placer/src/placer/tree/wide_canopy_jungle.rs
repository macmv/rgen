//use core::random;
use std::hint;

use rgen_base::{BlockState, BlockFilter, Pos};
use rgen_world::{PartialWorld, UndoError};
use crate::{rng, Placer, Random, Result, Rng};

pub struct WideCanopyJungle {
    pub place_above: BlockFilter,
    pub trunk: BlockState,
    pub branch: BlockState,
    pub top:   BlockState,
    pub roots:   BlockState,
    pub moss_roots:   BlockState,
    pub leaves: BlockState,
    pub vine:         BlockState


}

impl Default for WideCanopyJungle {
    fn default() -> Self {
        Self {
            place_above: block![grass].into(),
            //leaves[variant = "birch"]
            trunk: block!(log[variant="jungle"]), //block![concrete[color="red"]],
            branch: block!(log[variant="jungle",axis="none"]),
            top: block![concrete[color="yellow"]],
            roots: block!(log[variant="jungle",axis="none"]), //block!(log[variant="jungle"]),//block![concrete[color="green"]],
            moss_roots: block![rgen:covered_jungle_log],//block![concrete[color="lime"]],
            leaves: block![leaves[variant = "jungle", check_decay = false, decayable = true]],
            vine: block![vine],
        }
    }
}

impl Placer for WideCanopyJungle {
    fn radius(&self) -> u8 { 10 }

    fn avg_per_chunk(&self) -> f64 { 4.0 }

    fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
        if pos.y + 24 >= 255 || pos.y <= 1 {
            return Err(UndoError);
        }

        let below = pos + Pos::new(0, -1, 0);
        if !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
            return Err(UndoError);
        }

        //self.place_blob_tower(world, rng, pos);
        let (mid,top) = self.place_pole(world, rng, pos);
        self.place_rotating_walls(world, rng, pos);
        let mut all_tips = self.make_mid_branches(world, rng, mid);
        all_tips.extend(self.make_top_branches(world, rng, top));
        all_tips.extend([top]);
        self.leaf_tops(world, rng, &all_tips);
        self.place_root_lines(world, rng, pos);
        self.add_vines_around_leaves(world, rng, &all_tips);
        
        Ok(())
    }
}

impl WideCanopyJungle {

    // POLE
    fn place_pole(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) -> (Pos, Pos) {
        let height = rng.range(10..=21); //15 old12
        let mut branch_mid_point = base;
        let mut top_pos = base;


        let lean_enabled = true;

        let mut lean_x = 0;
        let mut lean_z = 0;
        if lean_enabled {
            let total_lean = rng.range(1..=4);
            let direction = match rng.range(0..4) {
                0 => (1, 0),
                1 => (-1, 0),
                2 => (0, 1),
                _ => (1, 1),
            };
            lean_x = direction.0 * total_lean;
            lean_z = direction.1 * total_lean;
        }

        for y in 0..height {
            let shift_x = (lean_x * y) / height;
            let shift_z = (lean_z * y) / height;
            for dx in -1..=1 {
                for dz in -1..=1 {
                    let pos = base + Pos::new(dx + shift_x, y, dz + shift_z);
                    if dx == 0 && dz == 0{
                        if height-rng.range(3..4) == y{ //-3
                        branch_mid_point = pos;
                        }
                        //self.make_mid_branches(world,rng, pos);}
                        if y == height - 1 {
                        top_pos = pos; }
                    
                    }
                    if world.get(pos) == block![air] || world.get(pos) == block![water] || world.get(pos) == self.leaves{
                        world.set(pos, self.trunk);
                    }
                }
            }
        }

        // Add branches 5 blocks down from the top


        // Hollow corners
        for dx in [-1, 1] {
            for dz in [-1, 1] {
                if rng.range(0..=8) != 0 {
                    for y in 5 + rng.range(0..=2)..height {
                        let shift_x = (lean_x * y) / height;
                        let shift_z = (lean_z * y) / height;
                        let pos = base + Pos::new(dx + shift_x, y, dz + shift_z);
                        if world.get(pos) == self.trunk {
                            world.set(pos, block![air]);
                        }
                    }
                }
            }
        }
        (branch_mid_point, top_pos)
    }
    
    // BRANCHES
    fn make_mid_branches( &self, world: &mut PartialWorld, rng: &mut Rng, center: Pos) -> Vec<Pos> {
        let directions = [
            (1, 1),   // SE
            (1, -1),  // NE
            (-1, 1),  // SW
            (-1, -1), // NW
        ];

        let mut third_branch_points = Vec::new();

        for &(dx, dz) in &directions {
            let mut pos = center;

            // Step 1 & 2: diagonal blocks
            for _ in 0..rng.range(2..=3) {
                pos = pos + Pos::new(dx, 0, dz);
                let block = world.get(pos);
                if block == block![air] || block == block![grass] {
                    world.set(pos, self.branch);
                }
            }

            // Step 3: continue or veer
            let third_pos = if rng.range(0..3) == 0 {
                // Veer left or right
                let (off_dx, off_dz) = if rng.range(0..2) == 0 {
                    (dx, 0)
                } else {
                    (0, dz)
                };
                pos + Pos::new(off_dx, 1, off_dz)
            } else {
                pos + Pos::new(dx, 1, dz)
            };

            if world.get(third_pos) == block![air] || world.get(third_pos) == block![grass] {
                world.set(third_pos, self.branch);
            }

            third_branch_points.push(third_pos);
        }

        third_branch_points
    }
    fn make_top_branches(&self,world: &mut PartialWorld,rng: &mut Rng,center: Pos,) -> Vec<Pos> {
        
        let directions = [
            (1, 0),   // +x
            (-1, 0),  // -x
            (0, 1),   // +z
            (0, -1),  // -z
        ];

        let mut branch_tips = Vec::new();

        for &(dx, dz) in &directions {
            let mut pos = center;

            // Step 1: straight horizontal
            pos = pos + Pos::new(dx, 0, dz);
            if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                world.set(pos, self.branch);
            }

            // Step 2: up one
            pos = pos + Pos::new(dx, 1, dz);
            if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                world.set(pos, self.branch);
            }
            
            if rng.range(1..=3) == 1 {
                pos = pos + Pos::new(dx, 0, dz);
                if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                    world.set(pos, self.branch);
                }
            }

            // Step 3: optional 25% chance for one more block up and out
            if rng.range(0..4) == 0 {
                pos = pos + Pos::new(dx, 1, dz);
            }else{
                pos = pos + Pos::new(dx, 0, dz);
            }

            // Place tip block
            if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                world.set(pos, self.branch);
            }

            if rng.range(0..=6) == 0 {
                pos = pos + Pos::new(0, 1, 0);
                if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                    world.set(pos, self.branch);
                }
            }
            branch_tips.push(pos);
        }

        branch_tips
    }

    // LEAF TOPS
    fn leaf_tops(&self, world: &mut PartialWorld, rng: &mut Rng, points: &[Pos]) {
        for &center in points {
            // --- Bottom Layer ---
            for dx in -3i32..=3 {
                for dz in -3i32..=3 {
                    // Skip corners
                    if dx.abs() == 3 && dz.abs() == 3 {
                        continue;
                    }

                    let pos = center + Pos::new(dx, 0, dz);
                    if world.get(pos) == block![air]  || world.get(pos) == self.leaves{
                        world.set(pos, self.leaves);
                    }
                }
            }

            // --- Top Layer ---
            for dx in -2i32..=2 {
                for dz in -2i32..=2 {
                    // 30% chance to skip corners
                    if dx.abs() == 2 && dz.abs() == 2{
                        continue;
                    }

                    let pos = center + Pos::new(dx, 1, dz);
                    if world.get(pos) == block![air]  || world.get(pos) == self.leaves{
                        world.set(pos, self.leaves);
                    }
                }
            }

            // --- Top Layer ---
            for dx in -1i32..=1 {
                for dz in -1i32..=1 {
                    // 30% chance to skip corners
                    if dx.abs() == 1 && dz.abs() == 1 && rng.range(0..10) < 3 {
                        continue;
                    }

                    let pos = center + Pos::new(dx, 1, dz);
                    if world.get(pos) == block![air] || world.get(pos) == self.leaves{
                        //world.set(pos, self.leaves);
                    }
                }
            }
        }
    }
    fn add_vines_around_leaves(&self, world: &mut PartialWorld, rng: &mut Rng, points: &[Pos]) {
        for &center in points {
            let y = center.y;//- 3;

            for dx in -4..=4 {
                for dz in -4..=4 {
                    let vine_pos = center + Pos::new(dx, 0, dz);
                    let pos = Pos::new(vine_pos.x, y, vine_pos.z);

                    if world.get(pos) == block![air] && rng.range(0..=1) == 0 {
                        let mut is_space_to_place = false;
                        let mut first_face = (false, "angle", (0, 0));
                        let mut vine = self.vine;

                        for side in [(1, 0, "east"), (0, 1, "south"), (-1, 0, "west"), (0, -1, "north")] {
                            let adj = pos + Pos::new(side.0, 0, side.1);
                            if world.get(adj) == self.leaves {
                                is_space_to_place = true;

                                if !first_face.0 {
                                    first_face = (true, side.2, (side.0, side.1));
                                }

                                vine.set_prop(side.2, true);
                            }
                        }

                        if is_space_to_place {
                            world.set(pos, vine);

                            let mut hanging = self.vine;
                            hanging.set_prop(first_face.1, true);

                            // Vine above
                            let above = pos + Pos::new(0, 1, 0);
                            let support = pos + Pos::new(first_face.2.0, 1, first_face.2.1);
                            if world.get(above) == block![air]
                                && world.get(support) != block![air]
                                && rng.range(0..=3) == 0
                            {
                                world.set(above, hanging);
                            }

                            // Hanging down
                            for dy in 1..rng.range(3..=6) {
                                let below = pos + Pos::new(0, -dy, 0);
                                if world.get(below) == block![air] {
                                    world.set(below, hanging);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    // WALLS
    fn place_rotating_walls(&self, world: &mut PartialWorld, rng: &mut Rng, base: Pos) {
        let mut pos = base;

        for rotate in 0..4 {
            let mut pos = Pos::new(2, 0, 0); // Local direction (east)
            let maxtower = rng.range(2..=4);
            for z in -2i32..=2{
                for x in 0..=1{
                    let spread = z.abs(); // spread is 0 to 2
                    let mut height_ajust = 0;
                    for j in 0..spread {
                        //println!("z: {}, j: {}", z, j);
                        height_ajust = height_ajust + rng.range(1..2)
                    }
                    let height = maxtower-height_ajust-(x*rng.range(1..=2));
                    if height>0{
                        if !(height == 1 && 
                        (world.get(Self::rotate_face(pos+ Pos::new(0, -1, 0), base, rotate)) == block![air] 
                        || world.get(Self::rotate_face(pos+ Pos::new(0, -1, 0), base, rotate)) == block![water])){
                            for y in 0..height{
                                self.special_trunk_place(world,pos + Pos::new(x, y, z),base,rotate);
                            }
                            self.special_check_ground(world,pos + Pos::new(x, 0, z),base,rotate,rng);
                        }
                    }  
                }
            }
        }
    }
    fn special_trunk_place(&self, world: &mut PartialWorld, pos: Pos, base: Pos, rotate: u8){
        if world.get(Self::rotate_face(pos, base, rotate)) == block![air] || 
            world.get(Self::rotate_face(pos, base, rotate)) == self.leaves || 
            world.get(Self::rotate_face(pos, base, rotate)) == block![water]{
                world.set(Self::rotate_face(pos, base, rotate), self.trunk);
            }
    }
    fn special_check_ground(&self, world: &mut PartialWorld, pos: Pos, base: Pos, rotate: u8, rng: &mut Rng) {
        let mut touched_ground = false;

        for y in (-4..= -1).rev() {
            if world.get(Self::rotate_face(pos + Pos::new(0, 1 * y, 0), base, rotate)) != block![air] &&
            world.get(Self::rotate_face(pos + Pos::new(0, 1 * y, 0), base, rotate)) != self.leaves &&
            world.get(Self::rotate_face(pos + Pos::new(0, 1 * y, 0), base, rotate)) != block![water] {
                touched_ground = true;
                break;
            }
        }

        if !touched_ground {
            for y in -1..rng.range(-5..=-3) {
                world.set(Self::rotate_face(pos + Pos::new(0, 1 * y, 0), base, rotate), self.trunk);
            }
        } else {
            let mut loop_y = 0;
            loop { 
                loop_y += 1;
                if world.get(Self::rotate_face(pos + Pos::new(0, -1 * loop_y, 0), base, rotate)) == block![air] ||
                world.get(Self::rotate_face(pos + Pos::new(0, -1 * loop_y, 0), base, rotate)) == self.leaves ||
                world.get(Self::rotate_face(pos + Pos::new(0, -1 * loop_y, 0), base, rotate)) == block![water] {
                    world.set(Self::rotate_face(pos + Pos::new(0, -1 * loop_y, 0), base, rotate), self.trunk);
                } else {
                    return;
                }
            }
        }
    }
    fn rotate_face(local: Pos, base: Pos, rotation: u8) -> Pos {
        let Pos { x, y, z } = local;

        let rotated = match rotation % 4 {
            0 => Pos::new(x, y, z),         // no rotation
            1 => Pos::new(-z, y, x),        // 90° clockwise
            2 => Pos::new(-x, y, -z),       // 180°
            3 => Pos::new(z, y, -x),        // 270°
            _ => local,
        };

        return (base + rotated)
    }

    // ROOTS
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

        'outer: for &(dx, dz) in directions.iter() {
            let mut elevation_score = 0;
            let mut pos = base;

            // Step 1 and 2 in the main direction
            for _ in 0..5 {
                pos = pos + Pos::new(dx, 0, dz);
                if world.get(pos+ Pos::new(0, -1, 0)) == block![air] || world.get(pos+ Pos::new(0, -1, 0)) == block![water] {
                    elevation_score -= 1;
                    pos = pos+ Pos::new(0, -1, 0);
                }

                // Check if hanging for a while if so make dangle and end
                if elevation_score < -1 && world.get(pos + Pos::new(0, -1, 0)) == block![air] {
                    self.ground_seeker(world, pos, rng);
                    continue 'outer;
                }
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
                // Check if hanging
                if world.get(pos+ Pos::new(0, -1, 0)) == block![air] || world.get(pos+ Pos::new(0, -1, 0)) == block![water]{
                    elevation_score -= 1;
                    pos = pos+ Pos::new(0, -1, 0);
                }
                // Check if hanging for a while if so make dangle and end
                // Ground seaker code:
                if elevation_score < -1 && (world.get(pos + Pos::new(0, -1, 0)) == block![air] || world.get(pos+ Pos::new(0, -1, 0)) == block![water]){
                    self.ground_seeker(world, pos, rng);
                    continue 'outer;
                }
                // Place next portion
                if world.get(pos) == block![air] || world.get(pos) == block![grass] {
                    if world.get(pos+ Pos::new(0, 1, 0)) == block![air]{
                        world.set(pos, self.moss_roots);
                    }else{
                        world.set(pos, self.roots);
                    }
                }else{
                    break;
                }   
            }
            if world.get(Pos::new(0, -1, 0)) == block![air] || world.get(pos+ Pos::new(0, -1, 0)) == block![water] {
                    self.ground_seeker(world, pos, rng);
                    continue 'outer;
                }
            
        }
        

    }
    fn ground_seeker(&self, world: &mut PartialWorld, mut pos: Pos,rng: &mut Rng) {
        if world.get(pos) == block![air] || world.get(pos) == block![grass] {
            if world.get(pos+ Pos::new(0, 1, 0)) == block![air]{
                world.set(pos, self.moss_roots);
            }else{
                world.set(pos, self.roots);
            }
            
        }

        pos = pos + Pos::new(0, -1, 0);
        let mut depth = 0;
        for i in 5 ..=rng.range(5..35) {
            // check if its part of big set of roots
            depth += 1;
            if depth > rng.range(3..5){
                for dx in -1..=1 {
                    for dz in -1..=1 {
                        if dx == 0 && dz == 0 {
                            continue; // skip center
                        }
                        let check_pos = pos + Pos::new(dx, 0, dz);
                        if world.get(check_pos) == self.trunk {
                            return;
                        }
                    }
                }
            }

            if world.get(pos) == block![air] || world.get(pos) == block![water] || world.get(pos) == self.leaves{
                world.set(pos, self.roots);
            } else {
                break;
            }
            pos = pos + Pos::new(0, -1, 0);
        }
    }



   }