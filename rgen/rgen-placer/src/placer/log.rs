use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

/// Places a mossy tree stump and a fallen log, optionally with mushrooms.
pub struct LogAndStump {
    pub log:            BlockState,
    pub moss_log:       BlockState,
    pub ground:         BlockState,
    pub plants:         BlockFilter,
    pub avg_per_chunk:  f64,
    pub chance_of_moss: i32,
    pub is_shrooms:     bool,
    pub shroom:         BlockState,
}

impl Default for LogAndStump {
    fn default() -> Self {
        Self {
            log:            block![log[variant = "oak"]],
            moss_log:       block![rgen:mossy_stump[variant = "oak"]],
            ground:         block![grass],
            plants:         block![red_flower].into(),
            avg_per_chunk:  0.5,
            chance_of_moss: 8,
            is_shrooms:     true,
            shroom:         block![rgen:polypore],
        }
    }
}

impl Placer for LogAndStump {
    fn radius(&self) -> u8 { 9 }

    fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

    fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
        // Ensure a 3x3 area is free for the stump
        for dx in -1..=1 {
            for dz in -1..=1 {
                if world.get(pos + Pos::new(dx, 0, dz)) != block![air] {
                    return Err(UndoError);
                }
            }
        }

        // Ensure the stump sits on the correct ground type
        let below = pos + Pos::new(0, -1, 0);
        if world.get(below) != self.ground {
            return Err(UndoError);
        }

        if self.place_stump(world, rng, pos) {
            self.place_log(world, rng, pos);
        }

        Ok(())
    }
}

impl LogAndStump {
    fn place_stump(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> bool {
        // Check for air in a 3x3 area
        for dx in -1..=1 {
            for dz in -1..=1 {
                if world.get(pos + Pos::new(dx, 0, dz)) != block![air] {
                    return false;
                }
            }
        }

        world.set(pos, self.moss_log);

        if self.is_shrooms {
            for dx in -1..=1 {
                for dz in -1..=1 {
                    let offset = Pos::new(dx, 0, dz);
                    let target_pos = pos + offset;

                    if world.get(target_pos) != block![air] {
                        continue;
                    }

                    if rng.range(0..9) < 3 {
                        // Avoid corners and center
                        if (dx == 0 && dz == 0) || dx.abs() == dz.abs() {
                            continue;
                        }

                        let mut data = rng.range(0..3) as u8;

                        // Encode orientation
                        data |= match (dx, dz) {
                            (1, 0) => 0b1000,
                            (-1, 0) => 0b1100,
                            (0, 1) => 0b0100,
                            (0, -1) => 0b0000,
                            _ => continue,
                        };

                        world.set(target_pos, self.shroom.with_data(data));
                    }
                }
            }
        }

        true
    }

    fn place_log(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> bool {
        let mut directions = [
            ((-1, 0), "x"),
            ((1, 0),  "x"),
            ((0, -1), "z"),
            ((0, 1),  "z"),
        ];
        rng.shuffle(&mut directions);

        for &((dx, dz), direction_name) in &directions {
            // You can now use `direction_name` in debug output or for orientation-based logic
            let length = rng.range(4..=6);
            let start = pos + Pos::new(dx * 2, -1, dz * 2);
            let end = pos + Pos::new(dx * length, -1, dz * length);

            // Ensure both ends are not air or water
            let valid_ground = |p: Pos| {
                let block = world.get(p);
                block != block![air] && block != block![water]
            };
            if !valid_ground(start) || !valid_ground(end) {
                continue;
            }

            // Make sure log path is clear
            let mut path_clear = true;
            for i in 1..=length {
                let path_pos = pos + Pos::new(i * dx, 0, i * dz);
                if world.get(path_pos) != block![air] {
                    path_clear = false;
                    break;
                }
            }

            if !path_clear {
                continue;
            }

            // Place the log
            for i in 2..=length {
                let log_pos = pos + Pos::new(i * dx, 0, i * dz);
                let is_mossy = self.chance_of_moss < rng.range(0..=10);
                //let mut base_block = 
                if is_mossy { 
                  let mut base_block = self.moss_log;
                  base_block.set_prop("axis",direction_name);
                  world.set(log_pos, base_block); 
                  println!("dir: {direction_name} on mossy");

                } else { 
                  let mut base_block = self.log ;
                  base_block.set_prop("axis",direction_name); 
                  world.set(log_pos, base_block);
                  println!("dir: {direction_name} on loggy");

                };
            }

            return true;
        }

        false
    }
}
