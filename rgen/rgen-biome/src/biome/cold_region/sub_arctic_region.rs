use rgen_base::{biome, block};
use rgen_placer::{chunk_placer, placer};

use crate::builder::{BiomeBuilder, PlacerStage};

// SPRUCE CAT

pub fn spruce_grove(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  ground(g);
  g.set_underwater_block(block![stone]);

  spruce_rainbow_mix(g);
  evergreen_grass(g);

  g.place(
    "podzel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![dirt[variant = "podzol"]],
      radius:        2..=3,
      avg_per_chunk: 0.6,
    },
  );
}

pub fn spruce_river(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  ground(g);
  g.set_underwater_block(block![gravel]);

  river_mass_placer(g);

  evergreen_grass(g);
  evergreen_grass(g);

  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 3.0,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.8,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.3,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Fat,
    },
  );
}

pub fn windswept_spruce_grove(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  ground(g);
  windswept_evergreen_grass(g);

  g.set_underwater_block(block![stone]);

  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.2,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.4,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.1,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Fat,
    },
  );

  g.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        3..=4,
      avg_per_chunk: 0.1,
    },
  );

  g.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.4,
    },
  );

  g.place(
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
      log:           block![log[variant = "spruce"]],
    },
  );
}

fn spruce_rainbow_mix(g: &mut BiomeBuilder) {
  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 8.0,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 2.0,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 1.0,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Fat,
    },
  );
}

// FIR CAT

pub fn fir_grove(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  ground(g);
  fir_rainbow_mix(g);
  evergreen_grass(g);

  g.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![rgen:mossy_cobblestone_rgen],
      radius:        2..=2,
      avg_per_chunk: 0.7,
    },
  );

  g.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        2..=2,
      avg_per_chunk: 1.2,
    },
  );

  g.place(
    "cobble_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
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

pub fn fir_river(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  g.set_underwater_block(block![stone]);
  ground(g);

  river_mass_placer(g);

  evergreen_grass(g);
  evergreen_grass(g);

  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 3.0,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.8,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.3,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Fat,
    },
  );
}

pub fn windswept_fir_grove(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  ground(g);
  windswept_evergreen_grass(g);

  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.2,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.2,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.3,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Fat,
    },
  );

  g.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        3..=4,
      avg_per_chunk: 0.1,
    },
  );

  g.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.4,
    },
  );

  g.place(
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
      log:           block![rgen:log[variant = "fir"]],
    },
  );
}

fn fir_rainbow_mix(g: &mut BiomeBuilder) {
  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 8.0,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 2.0,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 1.0,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Fat,
    },
  );
}
// OTHER

pub fn windswept_hill(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  ground(g);
  windswept_evergreen_grass(g);

  g.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.9,
    },
  );

  g.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        3..=4,
      avg_per_chunk: 0.5,
    },
  );
}

#[allow(dead_code)]
pub fn snowy_peak(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  g.set_top_block(block![stone]);
  g.set_underwater_block(block![stone]);
  g.place_chunk(chunk_placer::CheckerboardSurface {
    replace: block![stone].into(),
    a:       block![concrete[color = "magenta"]],
    b:       block![concrete[color = "black"]],
  });
}

#[allow(dead_code)]
pub fn tiaga_beach(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#ffffff";

  g.set_underwater_block(block![dirt]);
  ground(g);

  g.place(
    "underwater clay",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_per_chunk:      2.0, //2.4,
      placement:          block![clay],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               3..=5,
      multiplyer:         2,
    },
  );

  g.place(
    "underwater sand",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_per_chunk:      1.5, //1.0,
      placement:          block![sand],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               2..=4,
      multiplyer:         3,
    },
  );

  g.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        1..=4,
      avg_per_chunk: 0.9,
    },
  );

  g.place(
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
      place:       block![tallgrass[type = "fern"]],
    },
  );

  g.place(
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
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 3.0,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.8,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.3,
      is_spruce:     true,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![leaves[variant = "spruce"]],
      trunk:         block![log[variant = "spruce"]],
      size:          placer::EvergreenSize::Fat,
    },
  );

  g.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 3.0,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Standard,
    },
  );
  g.place(
    "tall",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.8,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Tall,
    },
  );
  g.place(
    "fat",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_per_chunk: 0.3,
      is_spruce:     false,
      place_above:   [
        block![grass],
        block![concrete[color = "magenta"]],
        block![concrete[color = "black"]],
      ]
      .into(),
      leaves:        block![rgen:leaves[variant = "fir"]],
      trunk:         block![rgen:log[variant = "fir"]],
      size:          placer::EvergreenSize::Fat,
    },
  );

  g.place(
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
      log:           block![rgen:log[variant = "fir"]],
    },
  );

  g.place(
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
      log:           block![log[variant = "spruce"]],
    },
  );
}

pub fn mossy_shores(g: &mut BiomeBuilder) {
  g.id = biome![stone_beach];
  g.color = "#ffffff";
  g.set_top_block(block![gravel]);
  g.add_layer(block![gravel], 2, 4);

  g.place("Mossy Bolders", PlacerStage::Tree, placer::MossBoulder::new());

  g.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![rgen:mossy_cobblestone_rgen],
      attempts:    40,
    },
  );
  g.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![cobblestone],
      radius:        1..=2,
      avg_per_chunk: 3.0,
    },
  );

  g.place(
    "clay",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![clay],
      radius:        2..=2,
      avg_per_chunk: 0.3,
    },
  );

  g.place("underwater clay", PlacerStage::Sand, placer::WaterResources::new());

  g.place(
    "mossystone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![rgen:mossy_stone],
      radius:        1..=2,
      avg_per_chunk: 4.3,
    },
  );

  g.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![stone],
      radius:        1..=3,
      avg_per_chunk: 5.4,
    },
  );
  g.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![rgen:mossy_cobblestone_rgen],
      radius:        1..=4,
      avg_per_chunk: 1.0,
    },
  );

  g.place(
    "mossycobblestone_smaller_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![rgen:mossy_cobblestone_rgen],
      radius:        1..=2,
      avg_per_chunk: 4.0,
    },
  );

  g.place(
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
      place:       block![tallgrass[type = "fern"]],
    },
  );

  g.place(
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
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
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
  g.place(
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

  g.place(
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
  g.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: g.top_block().into(),
      log:         block![log],
      leaves:      block![leaves[variant = "oak", decayable = false]],

      radius:        10..=20,
      avg_per_chunk: 5.3,
    },
  );
}

// EFFECTS
fn ground(g: &mut BiomeBuilder) {
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 2, 5);
  g.set_underwater_block(block![gravel]);
}

fn evergreen_grass(g: &mut BiomeBuilder) {
  g.place(
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
      place:       block![tallgrass[type = "fern"]],
    },
  );

  g.place(
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
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
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
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=6,
      attempts:      10,
      avg_per_chunk: 3.0,
    },
  );

  g.place(
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
      place_short:      block![tallgrass[type = "fern"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_fern"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=6,
      attempts:      13,
      avg_per_chunk: 3.0,
    },
  );
  g.place(
    "little brown mushrooms",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    200,
      place_above: [block![dirt[variant = "podzol"]]].into(),
      place:       block![brown_mushroom],
    },
  );
}

fn windswept_evergreen_grass(g: &mut BiomeBuilder) {
  g.place(
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
      place:       block![tallgrass[type = "fern"]],
    },
  );

  g.place(
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
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
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
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        2..=4,
      attempts:      18,
      avg_per_chunk: 3.0,
    },
  );

  g.place(
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
      place_short:      block![tallgrass[type = "fern"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_fern"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        2..=4,
      attempts:      27,
      avg_per_chunk: 3.0,
    },
  );
}

fn river_mass_placer(g: &mut BiomeBuilder) {
  g.place(
    "underwater clay",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_per_chunk:      8.0, //2.4,
      placement:          block![clay],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               1..=2,
      multiplyer:         3,
    },
  );

  g.place(
    "underwater sand",
    PlacerStage::Sand,
    placer::WaterResources {
      avg_per_chunk:      5.0, //1.0,
      placement:          block![sand],
      tool_placement:     block![gold_block],
      tool_placement_two: block![iron_ore],
      size:               1..=2,
      multiplyer:         1,
    },
  );
}
