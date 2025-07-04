use rgen_base::{BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct BasicJungle {
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
  pub is_cocoa:    bool,
  pub shroom:        BlockState,
  pub ground:        BlockState,
  pub vine:         BlockState
}

impl Default for BasicJungle {
  fn default() -> Self {
    BasicJungle {
      trunk:         block![log[variant = "birch"]],
      leaves:        block![leaves[variant = "birch"]],
      avg_per_chunk: 5.0,
      is_cocoa:    true,
      shroom:        block![cocoa[age = 2]],
      ground:        block![grass],
      vine:          block![vine],
    }
  }
}

impl Placer for BasicJungle {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(8..=11);

    // Checks if outside world boundry
    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 0, rel_z)) != block![air] {
          return Err(UndoError);
        }
      }
    }

    // Checks if on ground
    if world.get(pos + Pos::new(0, -1, 0)) != self.ground {
      return Err(UndoError);
    }

    // Builds the bottom of the canopy
    for y in height - 3..=height - 2_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if rng.range(0..=4) == 1 && x.abs() == 2 && z.abs() == 2 {
            continue;
          }
          //sets the leaves
          let leafloc = pos + Pos::new(x, y, z);
          if world.get(leafloc) == block![air] || world.get(leafloc) == block![cocoa]{
            world.set(leafloc, self.leaves);
          }
        }
      }
    }

    // Builds the top of the canopy
    for y in height..=height + 1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners of the top to make the plus shape
          if y == height + 1 && x.abs() == 1 && z.abs() == 1 {
            continue;
          }
          // Sometimes removes the lower level of the leaves on the corner
          if rng.range(0..=4) == 1 && x.abs() == 1 && z.abs() == 1 {
            continue;
          }
          
          let leafloc = pos + Pos::new(x, y - 1, z);
          if world.get(leafloc) == block![air] || world.get(leafloc) == block![cocoa]{
            world.set(leafloc, self.leaves);
          }
        }
      }
    }

    // Builds trunk
    for y in 0..height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    // Builds polypores
    if self.is_cocoa && 3 == *rng.choose(&[0,1,2,3]){
        for rel_x in -1..=1_i32 {
          for rel_z in -1..=1_i32 {
            if rng.range(0..16) < 3 {
              let mut state = self.shroom.with_prop("facing", *rng.choose(&["north","east","south","west"])).with_prop("age", 2);

              // This removes the coners and the center
              if (rel_x == 0 && rel_z == 0) || (rel_x.abs() == rel_z.abs()) {
                continue;
              }

              if rel_x == 1 {
                state.set_prop("facing", "west");
              } else if rel_x == -1 {
                state.set_prop("facing", "east");
              } else if rel_z == 1 {
                state.set_prop("facing", "north");
              } else if rel_z == -1 {
                state.set_prop("facing", "south");
              }
              let cocoaLoc = pos + Pos::new(rel_x, height-5, rel_z);
              if world.get(cocoaLoc) == block![air] || world.get(cocoaLoc) == block![cocoa]{
                world.set(cocoaLoc, state);
              }

            }
          }
        }
      
    }

    //Build vines
    let y = height - 3_i32; 
    for x in -3..=3_i32 {
      for z in -3..=3_i32 {
        // sets locaiton of vines
        let vineloc = pos + Pos::new(x, y, z);
        // Checks if space to grow vine
        if world.get(vineloc) == block![air] && rng.range(0..=1)==0{
          let mut is_space_to_place = false;
          let mut first_avilable_face = (false,"angle");
          let mut aVine = self.vine;
          for side in [(1,0,"east"),(0,1,"south"),(-1,0,"west"),(0,-1,"north")]{
            if world.get(vineloc+Pos::new(side.0,0,side.1)) == self.leaves{
              //a working vine face was found
              is_space_to_place = true;
              // set what the hanging vines should look like
              if !first_avilable_face.0{
                first_avilable_face.0 = true;
                first_avilable_face.1 = side.2;
              }
              // update vine with new prop
              aVine.set_prop(side.2, true);
            }
          // VINE can be placed
          if is_space_to_place{
            world.set(vineloc, aVine);
            let mut hangingVine = (self.vine);
            hangingVine.set_prop(first_avilable_face.1, true);
            // set the above vine
            if (world.get(vineloc+Pos::new(0,1,0))== block![air])&& rng.range(0..=3)==0{
              world.set(vineloc+Pos::new(0,1,0), hangingVine);

            }
            // add the long hanging vines
            for y in 1 .. rng.range(3..=6){
              if (world.get(vineloc+Pos::new(0,y*-1,0))== block![air]){
                world.set(vineloc+Pos::new(0,y*-1,0), hangingVine);
              }else{
                break;
              }
            }
            }
          }
        }
      }
    }
    Ok(())}}

    
  

