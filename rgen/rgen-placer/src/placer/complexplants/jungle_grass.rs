//use core::f64::math::sqrt;
use crate::{Placer, Random, Result, Rng};
use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld,};
use std::collections::HashMap;
//use std::collections::HashMap;
//use std::collections::{HashSet, VecDeque}; //rng::Random

// Special traits
pub trait Style {
    fn style(style: FloorStyle) -> Self;
}

pub enum FloorStyle {
    Normal,
    CanopiedJungle,
    FlowerCanopiedJungle,
    LightJungle,
    TerracedJungle

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
            attempts:           2,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantCategory {
    None,
    Grass,
    DoubleTallGrass,
    DoubleTallFern,
    Fern,
    BirdOfParadise,
    MixA,
    MixB,
    MixC
}



impl JungleFloorPlace {
  fn circle(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos, radius: i32) {
    let weights = self.floor_style.category_weights();
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
              let category = select_category(rng, &weights);
              match category {
                PlantCategory::None => {},
                PlantCategory::Grass => {
                  world.set(set_pos + Pos::new(0, 1, 0), self.grass.with_prop("type", "tall_grass"));
                },
                PlantCategory::DoubleTallGrass => {
                  world.set(
                    set_pos + Pos::new(0, 1, 0),
                    self.tall_grass.with_prop("half", "lower").with_prop("variant", "double_grass"),
                  );
                  world.set(
                    set_pos + Pos::new(0, 2, 0),
                    self.tall_grass.with_prop("half", "upper").with_prop("variant", "sunflower"),
                  );
                },
                PlantCategory::DoubleTallFern =>{
                  // Double Tall Fern
                  world.set(
                    set_pos + Pos::new(0, 1, 0),
                    self.tall_grass.with_prop("half", "lower").with_prop("variant", "double_fern"),
                  );
                  world.set(
                    set_pos + Pos::new(0, 2, 0),
                    self.tall_grass.with_prop("half", "upper").with_prop("variant", "sunflower"),
                  );
                },
                PlantCategory::Fern =>{
                  world.set(set_pos + Pos::new(0, 1, 0), self.grass.with_prop("type", "fern"));
                }
                PlantCategory::BirdOfParadise =>{
                  world.set(set_pos + Pos::new(0, 1, 0), self.bird_of_paradise);
                }
                PlantCategory::MixA => {
                  let flower_chance = rng.range(0..5) as usize;
                  let flower_array = [
                    self.pink_orchid,
                    self.passion_flower,
                    self.heliconia,
                    self.pink_heart,
                    self.torch_ginger,
                  ];
                  world.set(set_pos + Pos::new(0, 1, 0), flower_array[flower_chance]);
                },
                PlantCategory::MixB => {
                  //Deep Green Mix
                  let flower_chance = rng.range(0..5) as usize;
                  let flower_array = [
                    self.yellow_jungle_rose,
                    self.jungle_bush,
                    self.ficus_elastica,
                    self.bromeliads,
                    self.bird_of_paradise
                  ];
                  world.set(set_pos + Pos::new(0, 1, 0), flower_array[flower_chance]);
                },
                PlantCategory::MixC => {
                  //Deep Green Mix
                  let flower_chance = rng.range(0..12) as usize;
                  let flower_array = [
                    self.yellow_jungle_rose,
                    self.jungle_bush,
                    self.ficus_elastica,
                    self.bromeliads,
                    self.bird_of_paradise,
                    self.heliconia,
                    self.torch_ginger,
                    self.pink_heart,
                    self.pink_orchid,
                    self.ipomoea,
                    self.orchidaceae,
                    self.passion_flower,
                  ];
                  world.set(set_pos + Pos::new(0, 1, 0), flower_array[flower_chance]);
                },
              }
            }
          }
      }
  }
}
  }
}




pub fn select_category(
    rng: &mut Rng,
    weights: &HashMap<PlantCategory, u8>
) -> PlantCategory {
    let total: u8 = weights.values().sum();
    let mut roll = rng.range(0..total as i32) as u8;

    for (category, &weight) in weights {
        if roll < weight {
            return *category;
        }
        roll -= weight;
    }

    PlantCategory::None // Fallback
}


impl FloorStyle {
    pub fn category_weights(&self) -> HashMap<PlantCategory, u8> {
        use PlantCategory::*;
        match self {
            FloorStyle::Normal => {
                let mut map = HashMap::new();
                map.insert(Grass, 100);
                map
            }
            FloorStyle::CanopiedJungle => {
              let mut map: HashMap<PlantCategory, u8> = HashMap::new();
              map.insert(MixA, 10); // 10% flowers
              map.insert(Grass, 35);
              map.insert(DoubleTallGrass, 25);
              map.insert(DoubleTallFern, 15);
              map.insert(Fern, 15);
              map
            }
            FloorStyle::FlowerCanopiedJungle => {
              let mut map: HashMap<PlantCategory, u8> = HashMap::new();
              map.insert(MixA, 70); // 70% flowers
              map.insert(Grass, 10);
              map.insert(DoubleTallGrass, 8);
              map.insert(DoubleTallFern, 6);
              map.insert(Fern, 6);
              map
            }
            FloorStyle::LightJungle => {
              let mut map: HashMap<PlantCategory, u8> = HashMap::new();
              map.insert(None, 83); 
              map.insert(Grass, 7);
              map.insert(Fern, 7);
              map.insert(DoubleTallGrass, 2);
              map.insert(MixC, 1);
              map
            }
            FloorStyle::TerracedJungle => {
              let mut map: HashMap<PlantCategory, u8> = HashMap::new();
              map.insert(None, 60);
              map.insert(Grass, 15);
              map.insert(Fern, 15);
              map.insert(MixB, 5);
              map.insert(DoubleTallGrass, 5); 
              map
            }

        }
    }
}

