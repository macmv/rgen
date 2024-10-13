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

#[allow(dead_code)]
pub fn ancient_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.sand.default_state);
  gen.add_layer(ctx.blocks.stone.default_state, 1, 3);

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.gravel.default_state,
      radius:        2..=5,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.cobblestone.default_state,
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.stone.default_state,
      radius:        2..=4,
      avg_per_chunk: 1.0,
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

  gen.place("dead_tree", PlacerStage::Tree, placer::DeadTree::new(ctx.blocks));
}

#[allow(dead_code)]
pub fn dry_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.gravel.default_state);
  gen.add_layer(ctx.blocks.stone.default_state, 1, 3);

  gen.place(
    "sand_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.sand.default_state,
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.stone.default_state,
      radius:        3..=3,
      avg_per_chunk: 1.0,
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
#[allow(dead_code)]
pub fn bare_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.stone.default_state);
}
#[allow(dead_code)]
pub fn wet_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.add_layer(ctx.blocks.stone.default_state, 1, 3);

  gen.place(
    "grass_splatter",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.grass.default_state,
      attempts: 40,
    },
  );
  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.mossy_cobblestone.default_state,
      radius:        2..=3,
      avg_per_chunk: 1.0,
    },
  );
}
#[allow(dead_code)]
pub fn sand_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.sand.default_state);
  gen.add_layer(ctx.blocks.sand.default_state, 1, 3);

  gen.set_underwater_block(ctx.blocks.sand.default_state);

  gen.place("palm_tree", PlacerStage::Tree, placer::PalmTree::new(ctx.blocks));
}
#[allow(dead_code)]
pub fn monument_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.sand.default_state);
  gen.add_layer(ctx.blocks.sand.default_state, 1, 3);

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

#[allow(dead_code)]
pub fn palm_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.color = "#ffffff";
  gen.set_top_block(ctx.blocks.sand.default_state);
  gen.add_layer(ctx.blocks.sand.default_state, 1, 3);
}
