use rgen_placer::{
  chunk_placer,
  noise::{OpenSimplexNoise, SeededNoise},
  placer,
};

use super::super::{color, IdContext};
use crate::builder::{BiomeBuilder, PlacerStage};

pub fn ice_spikes(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#E3F5FC";
  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new(ctx.blocks));
  gen.place_chunk(chunk_placer::SnowOnStoneSurface::new(ctx.blocks));

  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new(ctx.blocks));
}

pub fn deep_snow_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#E3F5FC";
  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new(ctx.blocks));

  gen.place_chunk(chunk_placer::SnowOnStoneSurface {
    noise:       OpenSimplexNoise::new(0),
    a:           ctx.blocks.snow_layer.default_state,
    add_snow:    2.25,
    min_snow:    0,
    place_above: [ctx.blocks.stone.block].into(),
  });

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
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.stone.default_state,
      attempts:    30,
    },
  );
}

pub fn ice_spike_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#E3F5FC";
  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new(ctx.blocks));
  gen.place_chunk(chunk_placer::SnowOnStoneSurface {
    noise:       OpenSimplexNoise::new(0),
    a:           ctx.blocks.snow_layer.default_state,
    add_snow:    0.75,
    min_snow:    1,
    place_above: [ctx.blocks.stone.block].into(),
  });

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
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.stone.default_state,
      attempts:    30,
    },
  );
}

pub fn glacier(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#82C5E1";
  gen.set_top_block(ctx.blocks.packed_ice.default_state);
  gen.add_layer(ctx.blocks.packed_ice.default_state, 20, 25);

  gen.place_chunk(chunk_placer::Crevasse::new(ctx.blocks));
}

pub fn boulder_field(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#6FAFCE";
  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place_chunk(chunk_placer::SnowOnStoneSurface::new(ctx.blocks));
  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new(ctx.blocks));

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn hard_frozen_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#B2DBEF";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::GRAY));

  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.cobblestone.default_state,
      attempts: 100,
    },
  );
}

pub fn alps(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#4E9BB7";

  gen.set_top_block(ctx.blocks.snow_layer.with_data(7));
  gen.add_layer(ctx.blocks.snow.default_state, 1, 2);
  gen.add_layer(ctx.blocks.stone.default_state, 4, 5);

  gen.place_chunk(chunk_placer::SnowOnSnowSurface::new(ctx.blocks));
  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new(ctx.blocks));
}

pub fn frozen_peak(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#4E9BB7";

  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place_chunk(chunk_placer::SnowOnStoneSurface::new(ctx.blocks));
  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new(ctx.blocks));
}
