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

  gen.set_top_block(ctx.blocks.stone.default_state);
  gen.set_underwater_block(ctx.blocks.stone.default_state);
  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: ctx.blocks.stone.block.into(),
    a:       ctx.blocks.concrete.with_data(color::MAGENTA),
    b:       ctx.blocks.concrete.with_data(color::BLACK),
  });

  spruce_rainbow_mix(ctx, gen);
}

pub fn spruce_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
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

pub fn windswept_spruce_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
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

fn spruce_rainbow_mix(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.place(
    "standard",
    PlacerStage::Tree,
    placer::EverGreen {
      avg_in_chunk: 9.0 + 3.0 + 4.0,
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
  )
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

// OTHER

pub fn windswept_hill(ctx: &IdContext, gen: &mut BiomeBuilder) {
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
