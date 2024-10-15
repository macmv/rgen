use rgen_base::{biome, block};
use rgen_placer::{chunk_placer, placer};

use super::super::color;
use crate::builder::{BiomeBuilder, PlacerStage};

// SPRUCE CAT

pub fn spruce_grove(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  ground(gen);
  gen.set_underwater_block(block![stone]);

  spruce_rainbow_mix(gen);
  evergreen_grass(gen);

  gen.place(
    "podzel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![dirt[2]],
      radius:        2..=3,
      avg_per_chunk: 0.6,
    },
  );
}

pub fn spruce_river(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  ground(gen);
  gen.set_underwater_block(block![gravel]);

  river_mass_placer(gen);

  evergreen_grass(gen);
  evergreen_grass(gen);

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 3.0,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.8,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.3,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Fat,
    },
  );
}

pub fn windswept_spruce_grove(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  ground(gen);
  windswept_evergreen_grass(gen);

  gen.set_underwater_block(block![stone]);

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.2,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.4,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.1,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Fat,
    },
  );

  gen.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        3..=4,
      avg_per_chunk: 0.1,
    },
  );

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.4,
    },
  );

  gen.place(
    "fir log",
    PlacerStage::Tree,
    placer::LongLog {
      avg_per_chunk: 2.0,
      ground:        [
        block![dirt],
        block![grass],
        block![gravel],
        block![stone],
        block![cobblestone],
        block![rgen:mossy_cobblestone_rgen],
      ]
      .into(),
      log:           block![log[1]],
    },
  );
}

fn spruce_rainbow_mix(gen: &mut BiomeBuilder) {
  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 8.0,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 2.0,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 1.0,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Fat,
    },
  );
}

// FIR CAT

pub fn fir_grove(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  ground(gen);
  fir_rainbow_mix(gen);
  evergreen_grass(gen);

  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![rgen:mossy_cobblestone_rgen],
      radius:        2..=2,
      avg_per_chunk: 0.7,
    },
  );

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        2..=2,
      avg_per_chunk: 1.2,
    },
  );

  gen.place(
    "cobble_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![cobblestone],
      radius:        1..=2,
      avg_per_chunk: 0.9,
    },
  );
  // Red
  // Bue
  // Green
  // Rainbow
}

pub fn fir_river(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  gen.set_underwater_block(block![stone]);
  ground(gen);

  river_mass_placer(gen);

  evergreen_grass(gen);
  evergreen_grass(gen);

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 3.0,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.8,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.3,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Fat,
    },
  );
}

pub fn windswept_fir_grove(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  ground(gen);
  windswept_evergreen_grass(gen);

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.2,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.2,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.3,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Fat,
    },
  );

  gen.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        3..=4,
      avg_per_chunk: 0.1,
    },
  );

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.4,
    },
  );

  gen.place(
    "fir log",
    PlacerStage::Tree,
    placer::LongLog {
      avg_per_chunk: 2.0,
      ground:        [
        block![dirt],
        block![grass],
        block![gravel],
        block![stone],
        block![cobblestone],
        block![rgen:mossy_cobblestone_rgen],
      ]
      .into(),
      log:           block![rgen:log[0]],
    },
  );
}

fn fir_rainbow_mix(gen: &mut BiomeBuilder) {
  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 8.0,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 2.0,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 1.0,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Fat,
    },
  );
}
// OTHER

pub fn windswept_hill(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  ground(gen);
  windswept_evergreen_grass(gen);

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.9,
    },
  );

  gen.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        3..=4,
      avg_per_chunk: 0.5,
    },
  );
}

#[allow(dead_code)]
pub fn snowy_peak(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  gen.set_top_block(block![stone]);
  gen.set_underwater_block(block![stone]);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: block![stone].into(),
    a:       block![concrete[color::MAGENTA]],
    b:       block![concrete[color::BLACK]],
  });
}

#[allow(dead_code)]
pub fn tiaga_beach(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga];
  gen.color = "#ffffff";

  gen.set_underwater_block(block![dirt]);
  ground(gen);

  gen.place(
    "underwater clay",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_in_chunk:       2.0, //2.4,
      placement:          block![clay],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               3..=5,
      multiplyer:         2,
    },
  );

  gen.place(
    "underwater sand",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_in_chunk:       1.5, //1.0,
      placement:          block![sand],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               2..=4,
      multiplyer:         3,
    },
  );

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.9,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    130,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    430,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[1]],
    },
  );

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 3.0,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.8,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.3,
      is_spruce:    true,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![leaves[1]],
      trunk:        block![log[1]],
      size:         placer::EvergreenSize::Fat,
    },
  );

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 3.0,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Standard,
    },
  );
  gen.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.8,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Tall,
    },
  );
  gen.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.3,
      is_spruce:    false,
      place_above:  [
        block![grass],
        block![concrete[color::MAGENTA]],
        block![concrete[color::BLACK]],
      ]
      .into(),
      leaves:       block![rgen:leaves[0]],
      trunk:        block![rgen:log[0]],
      size:         placer::EvergreenSize::Fat,
    },
  );

  gen.place(
    "fir log",
    PlacerStage::Tree,
    placer::LongLog {
      avg_per_chunk: 1.0,
      ground:        [
        block![dirt],
        block![grass],
        block![gravel],
        block![stone],
        block![cobblestone],
        block![rgen:mossy_cobblestone_rgen],
      ]
      .into(),
      log:           block![rgen:log[0]],
    },
  );

  gen.place(
    "spruce log",
    PlacerStage::Tree,
    placer::LongLog {
      avg_per_chunk: 1.0,
      ground:        [
        block![dirt],
        block![grass],
        block![gravel],
        block![stone],
        block![cobblestone],
        block![rgen:mossy_cobblestone_rgen],
      ]
      .into(),
      log:           block![log[1]],
    },
  );
}

pub fn mossy_shores(gen: &mut BiomeBuilder) {
  gen.id = biome![stone_beach];
  gen.color = "#ffffff";
  gen.set_top_block(block![gravel]);
  gen.add_layer(block![gravel], 2, 4);

  gen.place("Mossy Bolders", PlacerStage::Tree, placer::MossBoulder::new());

  gen.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![rgen:mossy_cobblestone_rgen],
      attempts:    40,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![cobblestone],
      radius:        1..=2,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "clay",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![clay],
      radius:        2..=2,
      avg_per_chunk: 0.3,
    },
  );

  gen.place("underwater clay", PlacerStage::Sand, placer::WaterResources::new());

  gen.place(
    "mossystone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![rgen:mossy_stone],
      radius:        1..=2,
      avg_per_chunk: 4.3,
    },
  );

  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![stone],
      radius:        1..=3,
      avg_per_chunk: 5.4,
    },
  );
  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![rgen:mossy_cobblestone_rgen],
      radius:        1..=4,
      avg_per_chunk: 1.0,
    },
  );

  gen.place(
    "mossycobblestone_smaller_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![rgen:mossy_cobblestone_rgen],
      radius:        1..=2,
      avg_per_chunk: 4.0,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    130,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    430,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[1]],
    },
  );

  gen.place(
    "mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         block![rgen:mossy_carpet],
      replace:       [block![rgen:mossy_cobblestone_rgen], block![stone], block![rgen:mossy_stone]]
        .into(),
      radius:        2..=4,
      avg_per_chunk: 2.4,
    },
  );
  gen.place(
    "large mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         block![rgen:mossy_carpet],
      replace:       [block![rgen:mossy_cobblestone_rgen], block![stone], block![rgen:mossy_stone]]
        .into(),
      radius:        4..=5,
      avg_per_chunk: 0.4,
    },
  );

  gen.place(
    "mossy_bush",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: [
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
        block![rgen:mossy_stump],
      ]
      .into(),
      place:       block![rgen:plant],
      attempts:    150,
    },
  );
  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: gen.top_block().into(),
      log:         block![log],
      leaves:      block![leaves[4]],

      radius:        10..=20,
      avg_per_chunk: 5.3,
    },
  );
}

// EFFECTS
fn ground(gen: &mut BiomeBuilder) {
  gen.set_top_block(block![grass]);
  gen.add_layer(block![dirt], 2, 5);
  gen.set_underwater_block(block![gravel]);
}

fn evergreen_grass(gen: &mut BiomeBuilder) {
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    3,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    7,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[1]],
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=6,
      attempts:      10,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "tall ferns",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place_short:      block![tallgrass[2]],     // Grass
      place_tall_lower: block![double_plant[3]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=6,
      attempts:      13,
      avg_per_chunk: 3.0,
    },
  );
  gen.place(
    "little brown mushrooms",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    200,
      place_above: [block![dirt[2]]].into(),
      place:       block![brown_mushroom],
    },
  );
}

fn windswept_evergreen_grass(gen: &mut BiomeBuilder) {
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place:       block![tallgrass[1]],
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        2..=4,
      attempts:      18,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "tall ferns",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      [
        block![grass],
        block![rgen:mossy_stump],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:mossy_stone],
      ]
      .into(),
      place_short:      block![tallgrass[2]],     // Grass
      place_tall_lower: block![double_plant[3]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        2..=4,
      attempts:      27,
      avg_per_chunk: 3.0,
    },
  );
}

fn river_mass_placer(gen: &mut BiomeBuilder) {
  gen.place(
    "underwater clay",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_in_chunk:       8.0, //2.4,
      placement:          block![clay],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               1..=2,
      multiplyer:         3,
    },
  );

  gen.place(
    "underwater sand",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_in_chunk:       5.0, //1.0,
      placement:          block![sand],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               1..=2,
      multiplyer:         1,
    },
  );
}
