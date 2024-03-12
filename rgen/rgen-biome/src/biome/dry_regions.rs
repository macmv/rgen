use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

const SILVER: u8 = 8;

pub fn dry_grassy_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.top_block = ctx.blocks.grass.default_state;
}

pub fn dry_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
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
  )
  //loose dry oak tree
  //loose dry oak bush
  //grass
  //tall
}

pub fn wooded_savanna(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.top_block = ctx.blocks.wool.with_data(SILVER);

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

pub fn thorn_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.savanna;
  gen.top_block = ctx.blocks.wool.with_data(SILVER);

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
