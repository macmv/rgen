use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

use super::IdContext;

// TODO:  - add snow topping feature
//        - add fallen logs feature
//        - add branches to dead trees
//        - add bushes feature
//          - add feature to mossy shores
//        - add fir tree feature
//          - add feature to wet rock
//        - add water pools feature
//          - add feature to wet rocks
//        - add palm tree feature
//          - add feature to to palm beach
pub fn snowy_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.gravel.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "sand_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.sand.default_state,
      radius:  2..=4,
    },
  );

  // TODO: add snow ontop of everything
}

pub fn snowy_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.stone.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;
}

pub fn ancient_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.sand.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.gravel.default_state,
      radius:  2..=5,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.cobblestone.default_state,
      radius:  2..=4,
    },
  );
  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.stone.default_state,
      radius:  2..=4,
    },
  );

  gen.place(
    "loose_stone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.stone.default_state,
      attempts:    20,
    },
  );
  gen.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.mossy_cobblestone.default_state,
      attempts:    40,
    },
  );
  gen.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.stone.default_state,
      attempts:    30,
    },
  );

  gen.place(
    "dead_tree",
    PlacerStage::Tree,
    placer::DeadTree { trunk: ctx.blocks.rgen_log2.with_data(12) },
  );
}

pub fn mossy_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.gravel.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.mossy_cobblestone.default_state,
      attempts:    40,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.cobblestone.default_state,
      radius:  2..=4,
    },
  );
  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.mossy_cobblestone.default_state,
      radius:  3..=4,
    },
  );
  gen.place(
    "grass_splatter",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block.into(),
      place:    ctx.blocks.grass.default_state,
      attempts: 40,
    },
  );
  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: gen.top_block.into(),
      log:         ctx.blocks.log.default_state,
      leaves:      ctx.blocks.leaves.default_state,

      radius:        10..=20,
      attempts:      10,
      avg_per_chunk: 1.0,
    },
  );
}

pub fn dry_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.gravel.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "sand_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.sand.default_state,
      radius:  2..=4,
    },
  );
  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.stone.default_state,
      radius:  3..=3,
    },
  );
  gen.place(
    "dead_bushes",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    30,
      place_above: [ctx.blocks.gravel.block, ctx.blocks.sand.block, ctx.blocks.stone.block].into(),
      place:       ctx.blocks.tallgrass.with_data(0),
    },
  )
}

pub fn bare_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.stone.default_state;
}

pub fn wet_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.stone.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "grass_splatter",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block.into(),
      place:    ctx.blocks.grass.default_state,
      attempts: 40,
    },
  );
  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.mossy_cobblestone.default_state,
      radius:  2..=3,
    },
  );
}

pub fn sand_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.top_block = ctx.blocks.sand.default_state;
  gen.sub_layer = ctx.blocks.sand.default_state;

  gen.place(
    "palm_tree",
    PlacerStage::Tree,
    placer::BasicTree {
      avg_in_chunk: 16.0,
      place_above:  gen.top_block.into(),
      trunk:        ctx.blocks.rgen_log.with_data(1),
      leaves:       ctx.blocks.rgen_leaves.with_data(1),
    },
  )
}

pub fn monument_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.top_block = ctx.blocks.sand.default_state;
  gen.sub_layer = ctx.blocks.sand.default_state;

  // places monument
  gen.place(
    "sand_monument",
    PlacerStage::Tree,
    placer::Monument {
      material:       ctx.blocks.sandstone.with_data(0),
      fancy_material: ctx.blocks.sandstone.with_data(1),
      reward:         ctx.blocks.gold_block.default_state,
    },
  )
}

pub fn red_sand_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.top_block = ctx.blocks.sand.with_data(1);
  gen.sub_layer = ctx.blocks.sand.with_data(1);
}

pub fn red_monument_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.top_block = ctx.blocks.sand.with_data(1);
  gen.sub_layer = ctx.blocks.sand.with_data(1);

  // places monument
  gen.place(
    "red_sand_monument",
    PlacerStage::Tree,
    placer::Monument {
      material:       ctx.blocks.red_sandstone.with_data(0),
      fancy_material: ctx.blocks.red_sandstone.with_data(1),
      reward:         ctx.blocks.gold_block.default_state,
    },
  )
}

pub fn palm_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.top_block = ctx.blocks.sand.default_state;
  gen.sub_layer = ctx.blocks.sand.default_state;
}

pub fn chaparral_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: gen.top_block.into(),
      log:         ctx.blocks.log.default_state,
      leaves:      ctx.blocks.leaves.default_state,

      radius:        10..=20,
      attempts:      10,
      avg_per_chunk: 1.0,
    },
  );
}

pub fn jungle_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.jungle;
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block.into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:   4..=10,
      attempts: 20,
    },
  );

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: gen.top_block.into(),
      log:         ctx.blocks.log.with_data(3), //jungle log
      leaves:      ctx.blocks.leaves.with_data(3), //jungle leaves

      radius:        10..=20,
      attempts:      10,
      avg_per_chunk: 4.0,
    },
  );

  gen.place(
    "basic jungle tree",
    PlacerStage::Tree,
    placer::BasicTree {
      avg_in_chunk: 16.0,
      place_above:  gen.top_block.into(),
      trunk:        ctx.blocks.log.with_data(3),
      leaves:       ctx.blocks.leaves.with_data(3),
    },
  )
}
