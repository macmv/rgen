//! Grass and dirt placeholders for the Warm Temperate biome mix.

use rgen_base::{biome, block};

use crate::builder::{BiomeBuilder, PlacerStage};
use rgen_placer::{chunk_placer, icing_placer, placer};

fn grass_and_dirt(g: &mut BiomeBuilder) {
  g.id = biome![plains];
  g.color = "#78A65F";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
}

pub fn woodland(g: &mut BiomeBuilder) {
  grass_and_dirt(g);
  g.id = biome![forest];
  let log = block![log[variant = "oak", axis = "y"]];
  let leaves = block![leaves[variant = "oak", check_decay = false, decayable = false]];
  g.place("Woodland oak trees", PlacerStage::Tree, placer::BasicTree {
    trunk: log, leaves, avg_per_chunk: 12.0, trunk_height: 6..=9,
    ..placer::BasicTree::default()
  });
  woodland_bushes(g, 0.1, 0.05);
  g.place("Woodland boulders", PlacerStage::Tree, placer::MossBoulder {
    avg_per_chunk: 0.3, ..placer::MossBoulder::default()
  });
  g.place("Woodland fallen oak logs", PlacerStage::Tree, placer::LogAndStump {
    log, moss_log: block![rgen:covered_oak_log[axis = "none"]],
    plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
    avg_per_chunk: 0.2, shroom: block![rgen:polypore],
    ..placer::LogAndStump::default()
  });
  g.place_chunk(chunk_placer::Splotch {
    replace: block![grass].into(), below_replace: block![dirt].into(),
    place: block![dirt[variant = "podzol"]], radius: 1..=2,
    depth: 1..=1, avg_per_chunk: 0.05,
  });
  woodland_grass(g);
  g.place_icing(icing_placer::DoubleClumps {
    place_above: block![grass].into(), lower: block![double_plant[4]],
    upper: block![double_plant[12]], radius: 2..=4, attempts: 12, avg_per_chunk: 0.07,
  });
  for (plant, rate) in [
    (block![tallgrass[type = "fern"]], 1.0),
    (block![yellow_flower], 0.5),
    (block![red_flower], 0.5),
  ] {
    g.place_icing(icing_placer::Scatter { place: plant, avg_per_chunk: rate, ..Default::default() });
  }
  g.place_icing(icing_placer::DoubleScatter {
    place_above: block![grass].into(), place_lower: block![double_plant[3]],
    place_upper: block![double_plant[11]], attempts: 12, avg_per_chunk: 0.4,
  });
}

fn woodland_grass(g: &mut BiomeBuilder) {
  g.place_icing(icing_placer::Clumps {
    place_above: block![grass].into(), place: block![tallgrass[type = "tall_grass"]],
    radius: 4..=10, attempts: 60, avg_per_chunk: 3.0,
  });
  g.place_icing(icing_placer::DoubleClumps {
    place_above: block![grass].into(), lower: block![double_plant[2]],
    upper: block![double_plant[10]], radius: 2..=5, attempts: 18, avg_per_chunk: 0.2,
  });
  g.place_icing(icing_placer::Scatter {
    place: block![tallgrass[type = "tall_grass"]], avg_per_chunk: 2.0,
    ..Default::default()
  });
  g.place_icing(icing_placer::DoubleScatter {
    place_above: block![grass].into(), place_lower: block![double_plant[2]],
    place_upper: block![double_plant[10]], attempts: 12, avg_per_chunk: 0.1,
  });
}

fn woodland_banks(g: &mut BiomeBuilder) {
  for (place, rate) in [
    (block![stone], 1.0), (block![stone[variant = "andesite"]], 1.0),
    (block![mossy_cobblestone], 0.8), (block![gravel], 1.0),
    (block![sand], 0.6), (block![grass], 0.8),
  ] {
    g.place_chunk(chunk_placer::ShorelineSplotch {
      replace: [block![grass], block![dirt], block![gravel], block![sand], block![stone], block![stone[variant = "andesite"]], block![mossy_cobblestone]].into(),
      place: vec![place], radius: 1..=3, attempts: 1, avg_per_chunk: rate,
    });
  }
}

fn woodland_bushes(g: &mut BiomeBuilder, basic_rate: f64, tall_rate: f64) {
  let log = block![log[variant = "oak", axis = "y"]];
  let leaves = block![leaves[variant = "oak", check_decay = false, decayable = false]];
  g.place("Woodland oak bushes", PlacerStage::Tree, placer::BasicDryBush {
    trunk: log, leaves, avg_per_chunk: basic_rate, ..placer::BasicDryBush::default()
  });
  g.place("Woodland tall oak bushes", PlacerStage::Tree, placer::TallBush {
    ground: [block![grass], block![dirt]].into(), plantable: block![grass].into(),
    plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
    log, leaves, allow_water_under_leaves: false, avg_per_chunk: tall_rate,
  });
}

pub fn woodland_river(g: &mut BiomeBuilder) {
  woodland(g);
  woodland_banks(g);
  g.place_icing(icing_placer::Clumps {
    place_above: block![grass].into(), place: block![tallgrass[type = "fern"]],
    radius: 2..=5, attempts: 20, avg_per_chunk: 4.0,
  });
  g.place_icing(icing_placer::DoubleClumps {
    place_above: block![grass].into(), lower: block![double_plant[3]],
    upper: block![double_plant[11]], radius: 2..=5, attempts: 18, avg_per_chunk: 3.0,
  });
  g.place_icing(icing_placer::Scatter {
    place: block![tallgrass[type = "fern"]], avg_per_chunk: 3.0,
    ..Default::default()
  });
  g.place_icing(icing_placer::DoubleScatter {
    place_above: block![grass].into(), place_lower: block![double_plant[3]],
    place_upper: block![double_plant[11]], attempts: 12, avg_per_chunk: 2.0,
  });
  for material in [block![cobblestone], block![gravel], block![stone], block![stone[variant = "andesite"]]] {
    g.place_chunk(chunk_placer::WaterbedPatches {
      replace: [block![gravel], block![sand], block![dirt], block![stone]].into(),
      place: material, radius: 1..=3, depth: 1..=1, attempts: 12, avg_per_chunk: 0.65,
    });
  }
  woodland_bushes(g, 0.1, 0.1);
}

pub fn woodland_beach(g: &mut BiomeBuilder) {
  woodland(g);
  woodland_banks(g);
  woodland_bushes(g, 0.5, 0.3);
  g.place_chunk(chunk_placer::Splotch {
    replace: block![grass].into(), below_replace: block![dirt].into(),
    place: block![sand], radius: 5..=9, depth: 1..=1, avg_per_chunk: 1.0,
  });
  for plant in [block![rgen:dune_grass[variant = "a"]], block![rgen:short_grass]] {
    g.place_icing(icing_placer::Scatter {
      place_above: block![sand].into(), place: plant,
      avg_per_chunk: 1.0, ..Default::default()
    });
  }
}
pub fn birch_woodland(g: &mut BiomeBuilder) { grass_and_dirt(g); }
pub fn cherry_blossom_grove(g: &mut BiomeBuilder) {
  grass_and_dirt(g);
  g.id = biome![forest];
  g.place_chunk(icing_placer::BambooClump {
    bamboo: icing_placer::Bamboo { shorten_by: 5, ..Default::default() },
    avg_per_chunk: 0.5,
    ..Default::default()
  });
  g.place("Cherry Tree V2", PlacerStage::Tree, placer::CherryV2::default());
  g.place_icing(icing_placer::GrassClumps { avg_per_chunk: 3.0, ..icing_placer::GrassClumps::new() });
  g.place_icing(icing_placer::DoubleScatter {
    place_above: block![grass].into(), avg_per_chunk: 1.5, attempts: 12,
    place_lower: block![double_plant[2]], place_upper: block![double_plant[10]],
  });
  add_lavender(g, 2.0);
  for variant in ["variant_1", "variant_2", "variant_3", "variant_4"] {
    let lavender = match variant {
      "variant_1" => block![rgen:lavender_plant[variant = "variant_1"]],
      "variant_2" => block![rgen:lavender_plant[variant = "variant_2"]],
      "variant_3" => block![rgen:lavender_plant[variant = "variant_3"]],
      _ => block![rgen:lavender_plant[variant = "variant_4"]],
    };
    g.place_icing(icing_placer::Scatter {
      place_above: [block![grass], block![dirt]].into(),
      place: lavender, avg_per_chunk: 0.5, ..Default::default()
    });
  }
  g.place_icing(icing_placer::Clumps {
    place_above: block![grass].into(), place: block![rgen:pink_flowerbed],
    radius: 2..=4, attempts: 12, avg_per_chunk: 2.0,
  }.with_random_metadata());
  for metadata in 0..16 {
    let flower = block![rgen:pink_flowerbed].with_data(metadata);
    g.place_icing(icing_placer::Scatter {
      place: flower, avg_per_chunk: 0.05, ..Default::default()
    });
  }
}
pub fn cherry_blossom_grove_river(g: &mut BiomeBuilder) {
  cherry_blossom_grove(g);
  g.place("Cherry river tall stumps", PlacerStage::Tree, placer::TallStump {
    log: block![rgen:log[variant = "cherry", axis = "y"]],
    covered_log: block![rgen:covered_cherry_log[axis = "y"]],
    avg_per_chunk: 0.01,
    ..placer::TallStump::default()
  });
  super::mediterranean::add_cedar_boulders(g, 0.2);
  let andesite = block![stone[variant = "andesite"]];
  let stone = block![stone];
  let mossy = block![mossy_cobblestone];
  let gravel = block![gravel];
  for (place, avg_per_chunk) in [
    (vec![andesite, block![cobblestone], stone, mossy], 4.0),
    (vec![gravel, andesite, stone], 2.0),
    (vec![andesite, stone, mossy], 4.0),
  ] {
    g.place_chunk(chunk_placer::ShorelineSplotch {
      replace: [block![grass], block![dirt], block![sand]].into(),
      place,
      radius: 1..=3,
      attempts: 1,
      avg_per_chunk,
    });
  }
  g.place_chunk(chunk_placer::Splotch {
    replace: [block![grass], block![dirt], gravel].into(),
    below_replace: [block![dirt], gravel].into(),
    place: block![sand],
    radius: 2..=4,
    depth: 1..=1,
    avg_per_chunk: 0.3,
  }.in_groups(2..=4));
  let bed_patch = |place| chunk_placer::WaterbedPatches {
    replace: [gravel, block![sand], block![dirt]].into(),
    place,
    radius: 1..=3,
    depth: 1..=1,
    attempts: 18,
    avg_per_chunk: 0.65,
  };
  g.place_chunk(bed_patch(stone).with_mix(vec![stone, andesite]));
  g.place_chunk(bed_patch(block![cobblestone]));
  g.place_chunk(bed_patch(stone));
}

pub fn cherry_blossom_grove_mountains(g: &mut BiomeBuilder) {
  cherry_blossom_grove(g);
  g.place("Mountain grove creeping oak leaves", PlacerStage::Tree, placer::CreepingLeaves {
    arm_length: 2,
    place_above: block![grass].into(),
    leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
    avg_per_chunk: 2.0,
  });
}

pub fn fern_gully(g: &mut BiomeBuilder) {
  grass_and_dirt(g);
  g.id = biome![forest];
  g.place_icing(icing_placer::Clumps {
    place_above: block![grass].into(), place: block![tallgrass[type = "fern"]],
    radius: 2..=5, attempts: 25, avg_per_chunk: 2.0,
  });
  g.place_icing(icing_placer::DoubleClumps {
    place_above: block![grass].into(), avg_per_chunk: 3.0,
    radius: 2..=5, attempts: 18,
    lower: block![double_plant[3]], upper: block![double_plant[11]],
  });
  for material in [block![gravel], block![stone]] {
    g.place_chunk(chunk_placer::Splotch {
      replace: [block![grass], block![dirt]].into(),
      below_replace: [block![grass], block![dirt]].into(),
      place: material, avg_per_chunk: 1.2, radius: 1..=3, depth: 1..=1,
    });
  }
  g.place_icing(icing_placer::Scatter {
    place: block![yellow_flower], avg_per_chunk: 0.08, ..Default::default()
  });
}
pub fn mountain_oak(g: &mut BiomeBuilder) {
  grass_and_dirt(g);
  woodland_grass(g);
  g.id = biome![forest];
  let leaves = block![leaves[variant = "oak", check_decay = false, decayable = false]];
  g.place("Mountain basic oaks", PlacerStage::Tree, placer::BasicTree {
    avg_per_chunk: 0.3, trunk_height: 6..=9, leaves,
    ..placer::BasicTree::default()
  });
  g.place_chunk(chunk_placer::RockSurface {
    avg_per_chunk: 0.8, size: 12,
    mix_in: vec![block![stone], block![stone[variant = "andesite"]], block![cobblestone]],
  });
  for (shape, name) in [
    (placer::MountainOakShape::Stressed, "Stressed oak"),
    (placer::MountainOakShape::Starved, "Starved oak"),
  ] {
    g.place(name, PlacerStage::Tree, placer::MountainOak {
      shape, place_above: block![grass].into(), avg_per_chunk: 2.0,
    });
  }
  g.place("Mountain creeping oak leaves", PlacerStage::Tree, placer::CreepingLeaves {
    arm_length: 3, place_above: block![grass].into(), leaves, avg_per_chunk: 1.0,
  });
}
pub fn birch_aspen(g: &mut BiomeBuilder) { grass_and_dirt(g); }
pub fn lavender_beach(g: &mut BiomeBuilder) {
  grass_and_dirt(g);
  g.id = biome![forest];
  let log = block![rgen:log2[variant = "lavender_tree", axis = "y"]];
  let leaves = block![rgen:leaves2[variant = "lavender_tree", check_decay = false, decayable = true]];
  g.place_chunk(chunk_placer::Splotch {
    replace: block![grass].into(), below_replace: [block![dirt], block![sand]].into(),
    place: block![sand], avg_per_chunk: 1.0, radius: 5..=11, depth: 1..=2,
  });
  g.place("Lavender tall bushes", PlacerStage::Tree, placer::TallBush {
    ground: [block![grass], block![sand]].into(),
    plantable: [block![grass], block![sand]].into(),
    plants: [block![tallgrass], block![rgen:lavender_plant]].into(),
    log, leaves, allow_water_under_leaves: false, avg_per_chunk: 2.0,
  });
  g.place("Lavender trees", PlacerStage::Tree, placer::BasicTree {
    place_above: block![grass].into(), trunk: log, leaves,
    avg_per_chunk: 0.05, ..Default::default()
  });
  g.place("Lavender stumps", PlacerStage::Tree, placer::LavenderStump {
    place_above: block![grass].into(), log, avg_per_chunk: 0.02,
  });
  g.place("Lavender fallen logs", PlacerStage::Tree, placer::LongLog {
    log, ground: block![grass].into(), avg_per_chunk: 0.02,
  });
  g.place_icing(icing_placer::Scatter {
    place_above: [block![grass], block![dirt]].into(),
    place: block![rgen:short_grass], avg_per_chunk: 1.0, ..Default::default()
  });
  g.place_icing(icing_placer::Scatter {
    place_above: block![sand].into(),
    place: block![rgen:dune_grass[variant = "a"]], avg_per_chunk: 1.0,
    ..Default::default()
  });
  g.place_chunk(chunk_placer::Splotch {
    replace: [block![grass], block![sand]].into(),
    below_replace: [block![dirt], block![sand]].into(),
    place: block![gravel], avg_per_chunk: 0.04, radius: 1..=3, depth: 1..=1,
  });
  add_lavender(g, 1.4);
}
pub fn cypress_swamp(g: &mut BiomeBuilder) { grass_and_dirt(g); }

fn add_lavender(g: &mut BiomeBuilder, rate: f64) {
  for (lower, upper, short) in [
    (block![rgen:double_tall_lavender_plant[half = "lower", variant = "variant_1"]], block![rgen:double_tall_lavender_plant[half = "upper", variant = "variant_1"]], block![rgen:lavender_plant[variant = "variant_1"]]),
    (block![rgen:double_tall_lavender_plant[half = "lower", variant = "variant_2"]], block![rgen:double_tall_lavender_plant[half = "upper", variant = "variant_2"]], block![rgen:lavender_plant[variant = "variant_2"]]),
    (block![rgen:double_tall_lavender_plant[half = "lower", variant = "variant_3"]], block![rgen:double_tall_lavender_plant[half = "upper", variant = "variant_3"]], block![rgen:lavender_plant[variant = "variant_3"]]),
    (block![rgen:double_tall_lavender_plant[half = "lower", variant = "variant_4"]], block![rgen:double_tall_lavender_plant[half = "upper", variant = "variant_4"]], block![rgen:lavender_plant[variant = "variant_4"]]),
  ] {
    g.place_icing(icing_placer::Clumps {
      place_above: [block![grass], block![dirt]].into(), place: short,
      radius: 2..=5, attempts: 12, avg_per_chunk: rate / 4.0,
    });
    g.place_icing(icing_placer::DoubleScatter {
      place_above: [block![grass], block![dirt]].into(), avg_per_chunk: rate / 4.0,
      attempts: 10, place_lower: lower, place_upper: upper,
    });
  }
}
