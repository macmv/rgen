use rgen_placer::placer;

use super::super::{color, IdContext};
use crate::builder::{BiomeBuilder, PlacerStage};

pub fn ice_spikes(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#E3F5FC";
  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new(ctx.blocks));
}

pub fn glacier(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#82C5E1";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::LIGHT_BLUE));

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

pub fn boulder_field(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#6FAFCE";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::LIGHT_BLUE));

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
  gen.set_top_block(ctx.blocks.concrete.with_data(color::GRAY));
}

pub fn frozen_peak(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#4E9BB7";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::GRAY));
}
