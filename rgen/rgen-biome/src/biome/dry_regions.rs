use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{color, BiomeBuilder, IdContext};

pub fn chaparral_flats(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#B89B6A";
  gen.top_block = ctx.blocks.grass.default_state;
}
pub fn redwood_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.color = "#D1B284";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place("Sequoia", PlacerStage::Tree, placer::Sequoia::new(ctx.blocks));

  gen.place(
    "ponzel",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.grass.default_state,
      place:   ctx.blocks.dirt.with_data(2),

      attempts: 300,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [ctx.blocks.grass.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place("grass", PlacerStage::Tree, placer::GrassClumps::new(ctx.blocks))
}
pub fn open_plain(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#A7895D";
  gen.top_block = ctx.blocks.grass.default_state;
}
pub fn sunflower_plain(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#C49D73";
  gen.top_block = ctx.blocks.grass.default_state;
}
pub fn chaparral_woods(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#AA885C";
  gen.top_block = ctx.blocks.grass.default_state;
}

pub fn dry_grassy_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.color = "#D2B187";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      avg_in_chunk: 4 as f64,
      leaves:       ctx.blocks.leaves.default_state,
      place_above:  ctx.blocks.grass.block.into(),
      trunk:        ctx.blocks.log.default_state,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [ctx.blocks.grass.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place("grass", PlacerStage::Tree, placer::GrassClumps::new(ctx.blocks));
}

pub fn dry_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.color = "#C2926A";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      avg_in_chunk: 8 as f64,
      leaves:       ctx.blocks.leaves.default_state,
      place_above:  ctx.blocks.grass.block.into(),
      trunk:        ctx.blocks.log.default_state,
    },
  );

  gen.place(
    "sparce_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      avg_in_chunk: 0.1,
      place_above:  gen.top_block.into(),
      trunk:        ctx.blocks.log.default_state,
      leaves:       ctx.blocks.leaves.default_state,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    400,
      place_above: [ctx.blocks.grass.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::GrassClumps { attempts: 20, ..placer::GrassClumps::new(ctx.blocks) },
  );
  //loose dry oak tree
  //loose dry oak bush
  //grass
  //tall
}

pub fn wooded_savanna(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.color = "#BA8F63";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "grass",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.grass.default_state,

      attempts: 50,
    },
  );
}
pub fn open_savanna(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.color = "#C2A27E";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "grass",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.grass.default_state,

      attempts: 50,
    },
  );
}

pub fn thorn_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.color = "#B78E67";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "grass",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.grass.default_state,

      attempts: 50,
    },
  );
  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.gravel.default_state,

      attempts: 100,
    },
  );
}
