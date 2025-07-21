//use core::f64::math::sqrt;
use crate::{Placer, Random, Result, Rng};
use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld,};
//use std::collections::{HashSet, VecDeque}; //rng::Random

// Special traits
pub trait Deepforest {
    fn deepforest() -> Self;
}

pub struct JungleFloorPlace {
  pub place_above:        BlockFilter,
  pub place:              BlockState,
  pub is_large:           bool,
  pub attempts:           u32,
  pub pink_orchid:        BlockState,
  pub passion_flower:     BlockState,
  pub heliconia:          BlockState,
  pub pink_heart:         BlockState,
  pub torch_ginger:       BlockState,
  pub orchidaceae:        BlockState,
  pub ipomoea:            BlockState,
  pub bromeliads:         BlockState,
  pub ficus_elastica:     BlockState,
  pub yellow_jungle_rose: BlockState,
  pub bird_of_paradise:   BlockState,
  pub jungle_bush:        BlockState,
  pub grass:              BlockState,
  pub tall_grass:         BlockState,
  pub is_flower_floor:    bool,
}

impl Default for JungleFloorPlace {
  fn default() -> Self {
    JungleFloorPlace {
      attempts:           2,
      place_above:        [
        block![grass],
        block![dirt],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:covered_jungle_log],
      ]
      .into(),
      is_large:           false,
      place:              block![rgen:lavender_plant],
      pink_orchid:        block![rgen:pink_orchid],
      passion_flower:     block![rgen:passion_flower],
      heliconia:          block![rgen:heliconia],
      pink_heart:         block![rgen:pink_heart],
      torch_ginger:       block![rgen:torch_ginger],
      orchidaceae:        block![rgen:orchidaceae],
      ipomoea:            block![rgen:ipomoea],
      bromeliads:         block![rgen:bromeliads],
      ficus_elastica:     block![rgen:ficus_elastica],
      yellow_jungle_rose: block![rgen:yellow_jungle_rose],
      bird_of_paradise:   block![rgen:bird_of_paradise],
      jungle_bush:        block![rgen:jungle_bush],
      grass:              block![tallgrass],
      tall_grass:         block![double_plant],
      is_flower_floor:    false,
    }
  }
}
impl Deepforest for JungleFloorPlace {
    fn deepforest() -> Self {
        JungleFloorPlace {
            attempts:           4,
            is_large:           true,
            is_flower_floor:    true,
            place:              block![rgen:jungle_bush],
            place_above:        [
                block![grass],
                block![dirt],
                block![rgen:mossy_cobblestone_rgen],
                block![rgen:covered_jungle_log],
            ]
            .into(),
            pink_orchid:        block![rgen:pink_orchid],
            passion_flower:     block![rgen:passion_flower],
            heliconia:          block![rgen:heliconia],
            pink_heart:         block![rgen:pink_heart],
            torch_ginger:       block![rgen:torch_ginger],
            orchidaceae:        block![rgen:orchidaceae],
            ipomoea:            block![rgen:ipomoea],
            bromeliads:         block![rgen:bromeliads],
            ficus_elastica:     block![rgen:ficus_elastica],
            yellow_jungle_rose: block![rgen:yellow_jungle_rose],
            bird_of_paradise:   block![rgen:bird_of_paradise],
            jungle_bush:        block![rgen:jungle_bush],
            grass:              block![tallgrass],
            tall_grass:         block![double_plant],
        }
    }
}


impl Placer for JungleFloorPlace {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    //8  9    10   11
    //0  1    2    3

    for _ in 0..self.attempts {
      let pos = pos + Pos::new(rng.range(-8..=8), 0, rng.range(-8..=8));
      self.circle(world, rng, pos, 8);
    }

    Ok(())
  }
}
impl JungleFloorPlace {
  fn circle(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos, radius: i32) {

    for dx in -radius..=radius {
      for dz in -radius..=radius {
        for dy in -4..4 {
          let set_pos = pos + Pos::new(dx, dy, dz);
          let distannce = ((set_pos.x() - pos.x()) as f64 + (set_pos.y() - pos.y()) as f64).sqrt();
          if !(distannce > radius as f64) {
            if self.place_above.contains(world.get(set_pos))
              && world.get(set_pos + Pos::new(0, 1, 0)) == block!(air)
              && world.get(set_pos + Pos::new(0, 2, 0)) == block!(air)
              && world.get(set_pos + Pos::new(0, 3, 0)) == block!(air)
            {
              let chance = rng.range(0..10);
              // 70% GRASS & FERN MIX
              //     variant: ["sunflower", "syringa", "double_grass", "double_fern",
              // "double_rose", "paeonia"],     type: ["dead_bush",
              // "tall_grass", "fern"]     [00:16:33] [RGen/ERROR] [rgen]:
              // rgen_world::info:99: block minecraft:double_plant does not have a state with
              // the properties     {"facing": Enum("north"), "half":
              // Enum("upper"), "variant": Enum("double_grass")}

              let mut flower_percent = 7;
              // 70% grass
              if !self.is_flower_floor {
                // 90% grass
                flower_percent = 9;
              }
              if chance < flower_percent {
                let grass_chance = rng.range(0..=8);
                if grass_chance < 4 {
                  // Tall Grass
                  world
                    .set(set_pos + Pos::new(0, 1, 0), self.grass.with_prop("type", "tall_grass"));
                } else if grass_chance < 6 {
                  // Double Tall Grass
                  world.set(
                    set_pos + Pos::new(0, 1, 0),
                    self.tall_grass.with_prop("half", "lower").with_prop("variant", "double_grass"),
                  );
                  world.set(
                    set_pos + Pos::new(0, 2, 0),
                    self.tall_grass.with_prop("half", "upper").with_prop("variant", "sunflower"),
                  );
                } else if grass_chance < 7 {
                  // Double Tall Fern
                  world.set(
                    set_pos + Pos::new(0, 1, 0),
                    self.tall_grass.with_prop("half", "lower").with_prop("variant", "double_fern"),
                  );
                  world.set(
                    set_pos + Pos::new(0, 2, 0),
                    self.tall_grass.with_prop("half", "upper").with_prop("variant", "sunflower"),
                  );
                } else if grass_chance < 8 {
                  // Fern
                  world.set(set_pos + Pos::new(0, 1, 0), self.grass.with_prop("type", "fern"));
                }
              } else {
                if world.get(set_pos + Pos::new(0, 1, 0)) == block!(air) {
                  let flower_chance = rng.range(0..5) as usize;
                  let flower_array = [
                    self.pink_orchid,
                    self.passion_flower,
                    self.heliconia,
                    self.pink_heart,
                    self.torch_ginger,
                  ];
                  world.set(set_pos + Pos::new(0, 1, 0), flower_array[flower_chance]);
                }
              }
            }
          }
        }
      }
    }
  }
}
