//use core::f64::math::sqrt;
use crate::{Placer, Random, Result, Rng};
use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld,};
//use std::collections::{HashSet, VecDeque}; //rng::Random

// Special traits
pub trait Style {
    fn style(style: FloorStyle) -> Self;
}

pub enum FloorStyle {
    Normal,
    Flower,
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
  pub floor_style:        FloorStyle,
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
      floor_style:        FloorStyle::Normal,
    }
  }
}

impl Style for JungleFloorPlace {
    fn style(style: FloorStyle) -> Self {
        JungleFloorPlace {
            attempts:           4,
            is_large:           true,
            floor_style:        style,
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

#[derive(Debug, Clone, Copy)]
pub enum PlantCategory {
    None,
    Grass,
    DoubleTall,
    MixA,
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
              && world.get(set_pos + Pos::new(0, 3, 0)) == block!(air){

              }
            
          }
        }
      }
    }
  }
}
