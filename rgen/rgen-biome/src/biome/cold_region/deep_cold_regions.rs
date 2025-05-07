use rgen_base::{biome, block};
use rgen_placer::{
  chunk_placer,
  noise::{OpenSimplexNoise, SeededNoise},
  placer,
};

use crate::builder::{BiomeBuilder, PlacerStage};

pub fn ice_spikes(g: &mut BiomeBuilder) {
  g.id = biome![ice_flats];
  g.color = "#E3F5FC";
  g.set_top_block(block![stone]);

  g.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new());
  g.place_chunk(chunk_placer::SnowOnStoneSurface::new(g.seed));

  g.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());
}

pub fn deep_snow_beach(g: &mut BiomeBuilder) {
  g.id = biome![ice_flats];
  g.color = "#E3F5FC";
  g.set_top_block(block![stone]);

  g.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new());

  g.place_chunk(chunk_placer::SnowOnStoneSurface {
    noise:       OpenSimplexNoise::new(g.seed),
    a:           block![snow_layer],
    add_snow:    2.25,
    min_snow:    0,
    place_above: block![stone].into(),
  });

  g.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        2..=5,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      avg_per_chunk: 1.0,
      place_above:   block![stone].into(),
      place:         block![stone],
      attempts:      30,
    },
  );
}

pub fn ice_spike_beach(g: &mut BiomeBuilder) {
  g.id = biome![ice_flats];
  g.color = "#E3F5FC";
  g.set_top_block(block![stone]);

  g.place("Ice spikes", PlacerStage::Tree, placer::IceSpikes::new());
  g.place_chunk(chunk_placer::SnowOnStoneSurface {
    noise:       OpenSimplexNoise::new(0),
    a:           block![snow_layer],
    add_snow:    0.75,
    min_snow:    1,
    place_above: [block![stone]].into(),
  });

  g.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![gravel],
      radius:        2..=5,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      avg_per_chunk: 1.0,
      place_above:   block![stone].into(),
      place:         block![stone],
      attempts:      30,
    },
  );
}

pub fn glacier(g: &mut BiomeBuilder) {
  g.id = biome![ice_flats];
  g.color = "#82C5E1";
  g.set_top_block(block![packed_ice]);
  g.add_layer(block![packed_ice], 20, 25);

  g.place_chunk(chunk_placer::Crevasse::new());
}

pub fn boulder_field(g: &mut BiomeBuilder) {
  g.id = biome![ice_flats];
  g.color = "#6FAFCE";
  g.set_top_block(block![stone]);

  g.place_chunk(chunk_placer::SnowOnStoneSurface::new(g.seed));
  g.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());

  g.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter { replace: g.top_block(), place: block![ice], attempts: 100 },
  );
}

#[allow(dead_code)]
pub fn hard_frozen_river(g: &mut BiomeBuilder) {
  g.id = biome![ice_flats];
  g.color = "#B2DBEF";
  g.set_top_block(block![concrete[color = "gray"]]);

  g.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splatter { replace: g.top_block(), place: block![cobblestone], attempts: 100 },
  );
}

pub fn alps(g: &mut BiomeBuilder) {
  g.id = biome![taiga_cold];
  g.color = "#4E9BB7";

  g.set_top_block(block![snow_layer[layers = 8]]);
  g.add_layer(block![snow], 1, 2);
  g.add_layer(block![stone], 4, 5);

  g.place_chunk(chunk_placer::SnowOnSnowSurface::new(g.seed));
  g.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());
}

pub fn frozen_peak(g: &mut BiomeBuilder) {
  g.id = biome![taiga_cold];
  g.color = "#4E9BB7";

  g.set_top_block(block![stone]);

  g.place_chunk(chunk_placer::SnowOnStoneSurface::new(g.seed));
  g.place("Snow", PlacerStage::Tree, placer::BetterTallerSnow::new());
}
