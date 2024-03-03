use rgen_base::BlockSet;
use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

use super::IdContext;

// TODO:  - add snow topping feature
//        - add fallen logs feature
//        - add branches to dead trees
//        - add bushes feature
//          - add feature to mossy shores
//          - add feature to chapparel beach
//          - add feature to jungle beach
//        - add fir tree feature
//          - add feature to wet rock
//        - add water pools feature
//          - add feature to wet rocks
//        - add palm tree feature
//          - add feature to to palm beach
//        - add basic tree (jungle style) to jungle beach
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
    "dead tree",
    PlacerStage::Tree,
    placer::DeadTree { trunk: ctx.blocks.rgen_log.with_data(12) },
  )
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
  )
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
}

pub fn jungle_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.jungle;
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;
}
