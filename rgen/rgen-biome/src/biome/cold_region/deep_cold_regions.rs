use rgen_base::{biome, block};
use rgen_placer::{
  chunk_placer,
  noise::{OpenSimplexNoise, SeededNoise},
  placer,
};

use super::super::color;
use crate::builder::{BiomeBuilder, PlacerStage};

pub fn ice_spikes(gen: &mut BiomeBuilder) {
  gen.id = biome![ice_flats];
  gen.color = "#E3F5FC";
  gen.set_top_block(block![stone]);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new());
  gen.place_chunk(chunk_placer::SnowOnStoneSurface::new());

  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());
}

pub fn deep_snow_beach(gen: &mut BiomeBuilder) {
  gen.id = biome![ice_flats];
  gen.color = "#E3F5FC";
  gen.set_top_block(block![stone]);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new());

  gen.place_chunk(chunk_placer::SnowOnStoneSurface {
    noise:       OpenSimplexNoise::new(0),
    a:           block![snow_layer],
    add_snow:    2.25,
    min_snow:    0,
    place_above: block![stone].into(),
  });

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        2..=5,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![stone],
      attempts:    30,
    },
  );
}

pub fn ice_spike_beach(gen: &mut BiomeBuilder) {
  gen.id = biome![ice_flats];
  gen.color = "#E3F5FC";
  gen.set_top_block(block![stone]);

  gen.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new());
  gen.place_chunk(chunk_placer::SnowOnStoneSurface {
    noise:       OpenSimplexNoise::new(0),
    a:           block![snow_layer],
    add_snow:    0.75,
    min_snow:    1,
    place_above: [block![stone]].into(),
  });

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![gravel],
      radius:        2..=5,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![stone],
      attempts:    30,
    },
  );
}

pub fn glacier(gen: &mut BiomeBuilder) {
  gen.id = biome![ice_flats];
  gen.color = "#82C5E1";
  gen.set_top_block(block![packed_ice]);
  gen.add_layer(block![packed_ice], 20, 25);

  gen.place_chunk(chunk_placer::Crevasse::new());
}

pub fn boulder_field(gen: &mut BiomeBuilder) {
  gen.id = biome![ice_flats];
  gen.color = "#6FAFCE";
  gen.set_top_block(block![stone]);

  gen.place_chunk(chunk_placer::SnowOnStoneSurface::new());
  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter { replace: gen.top_block(), place: block![ice], attempts: 100 },
  );
}

#[allow(dead_code)]
pub fn hard_frozen_river(gen: &mut BiomeBuilder) {
  gen.id = biome![ice_flats];
  gen.color = "#B2DBEF";
  gen.set_top_block(block![concrete[color::GRAY]]);

  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splatter { replace: gen.top_block(), place: block![cobblestone], attempts: 100 },
  );
}

pub fn alps(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga_cold];
  gen.color = "#4E9BB7";

  gen.set_top_block(block![snow_layer[7]]);
  gen.add_layer(block![snow], 1, 2);
  gen.add_layer(block![stone], 4, 5);

  gen.place_chunk(chunk_placer::SnowOnSnowSurface::new());
  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());
}

pub fn frozen_peak(gen: &mut BiomeBuilder) {
  gen.id = biome![taiga_cold];
  gen.color = "#4E9BB7";

  gen.set_top_block(block![stone]);

  gen.place_chunk(chunk_placer::SnowOnStoneSurface::new());
  gen.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());
}
