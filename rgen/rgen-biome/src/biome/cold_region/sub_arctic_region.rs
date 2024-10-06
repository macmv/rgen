use rgen_placer::{
  chunk_placer,
  noise::{OpenSimplexNoise, SeededNoise},
  placer,
};

use super::super::{color, IdContext};
use crate::builder::{BiomeBuilder, PlacerStage};

// SPRUCE CAT

pub fn spruce_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  ground(ctx, gen);
  gen.set_underwater_block(ctx.blocks.stone.default_state);

  spruce_rainbow_mix(ctx, gen);
  evergreen_grass(ctx, gen);

  gen.place(
    "podzel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.dirt.with_data(2),
      radius:        2..=3,
      avg_per_chunk: 0.6,
    },
  );
}

pub fn spruce_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::BLUE),
    b:       ctx.blocks.concrete.with_data(color::GRAY),
  });
}

pub fn windswept_spruce_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  ground(ctx, gen);
  windswept_evergreen_grass(ctx, gen);

  gen.set_underwater_block(ctx.blocks.stone.default_state);

  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 0.2,
      is_spruce:    true,
      place_above:  [
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.leaves.with_data(1),
      trunk:        ctx.blocks.log.with_data(1),
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
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.leaves.with_data(1),
      trunk:        ctx.blocks.log.with_data(1),
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
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.leaves.with_data(1),
      trunk:        ctx.blocks.log.with_data(1),
      size:         placer::EvergreenSize::Fat,
    },
  );

  gen.place(
    "gravel big",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.gravel.default_state,
      radius:        3..=4,
      avg_per_chunk: 0.1,
    },
  );

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.gravel.default_state,
      radius:        1..=4,
      avg_per_chunk: 0.4,
    },
  );
}

fn spruce_rainbow_mix(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 8.0,
      is_spruce:    true,
      place_above:  [
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.leaves.with_data(1),
      trunk:        ctx.blocks.log.with_data(1),
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
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.leaves.with_data(1),
      trunk:        ctx.blocks.log.with_data(1),
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
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.leaves.with_data(1),
      trunk:        ctx.blocks.log.with_data(1),
      size:         placer::EvergreenSize::Fat,
    },
  );
}

// FIR CAT

pub fn fir_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::MAGENTA),
    b:       ctx.blocks.concrete.with_data(color::BLACK),
  });

  fir_rainbow_mix(ctx, gen);
  // Red
  // Bue
  // Green
  // Rainbow
}

pub fn fir_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::MAGENTA),
    b:       ctx.blocks.concrete.with_data(color::BLACK),
  });
}

pub fn windswept_fir_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::MAGENTA),
    b:       ctx.blocks.concrete.with_data(color::BLACK),
  });
}

fn fir_rainbow_mix(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 8.0,
      is_spruce:    false,
      place_above:  [
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.rgen_leaves.with_data(0),
      trunk:        ctx.blocks.rgen_log.with_data(0),
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
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.rgen_leaves.with_data(0),
      trunk:        ctx.blocks.rgen_log.with_data(0),
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
        ctx.blocks.grass.default_state,
        ctx.blocks.concrete.with_data(color::MAGENTA),
        ctx.blocks.concrete.with_data(color::BLACK),
      ]
      .into(),
      leaves:       ctx.blocks.rgen_leaves.with_data(0),
      trunk:        ctx.blocks.rgen_log.with_data(0),
      size:         placer::EvergreenSize::Fat,
    },
  );
}
// OTHER

pub fn windswept_hill(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  ground(ctx, gen);
  windswept_evergreen_grass(ctx, gen);

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.gravel.default_state,
      radius:        1..=4,
      avg_per_chunk: 0.3,
    },
  );
}

pub fn snowy_peak(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::MAGENTA),
    b:       ctx.blocks.concrete.with_data(color::BLACK),
  });
}

pub fn tiaga_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.taiga;
  gen.color = "#ffffff";

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::MAGENTA),
    b:       ctx.blocks.concrete.with_data(color::BLACK),
  });
}

pub fn mossy_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.gravel.default_state);
  gen.add_layer(ctx.blocks.gravel.default_state, 2, 4);

  gen.place("Mossy Bolders", PlacerStage::Tree, placer::MossBoulder::new(ctx.blocks));

  gen.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.rgen_mossy_cobblestone.default_state,
      attempts:    40,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.cobblestone.default_state,
      radius:        1..=2,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "clay",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.clay.default_state,
      radius:        2..=2,
      avg_per_chunk: 0.3,
    },
  );

  gen.place(
    "mossystone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.rgen_mossy_stone.default_state,
      radius:        1..=4,
      avg_per_chunk: 6.0,
    },
  );
  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.rgen_mossy_cobblestone.default_state,
      radius:        1..=4,
      avg_per_chunk: 1.0,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    130,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    230,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place(
    "mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         ctx.blocks.rgen_mossy_carpet.default_state,
      replace:       [
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.stone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      radius:        2..=4,
      avg_per_chunk: 2.4,
    },
  );
  gen.place(
    "large mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         ctx.blocks.rgen_mossy_carpet.default_state,
      replace:       [
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.stone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
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
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
        ctx.blocks.rgen_mossy_stump.block,
      ]
      .into(),
      place:       ctx.blocks.rgen_plant.default_state,
      attempts:    150,
    },
  );
  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: gen.top_block().into(),
      log:         ctx.blocks.log.default_state,
      leaves:      ctx.blocks.leaves.with_data(4),

      radius:        10..=20,
      avg_per_chunk: 2.3,
    },
  );
}

fn ground(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.add_layer(ctx.blocks.dirt.default_state, 2, 5);
}

fn evergreen_grass(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    3,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    7,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

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
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place_short:      ctx.blocks.tallgrass.with_data(2), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(3), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:        4..=6,
      attempts:      13,
      avg_per_chunk: 3.0,
    },
  );
}

fn windswept_evergreen_grass(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

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
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place_short:      ctx.blocks.tallgrass.with_data(2), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(3), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:        2..=4,
      attempts:      27,
      avg_per_chunk: 3.0,
    },
  );
}
