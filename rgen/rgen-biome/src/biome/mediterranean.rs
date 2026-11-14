use rgen_base::{biome, block, BlockFilter};
use rgen_placer::{chunk_placer, icing_placer, placer};

use crate::builder::PlacerStage;
use super::super::BiomeBuilder;

pub fn lebanon_cedar_woodland(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
  add_cedar_woodland_standard(g, 2.0, true, (45, 50, 5), 12.0);
}

pub fn lebanon_cedar_woodland_river(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
  g.set_underwater_block(block![gravel]);
  g.add_underwater_layer(block![gravel], 1, 2);
  add_cedar_woodland_standard(g, 4.0, true, (0, 50, 50), 5.0);
  add_cedar_tall_bushes(g, 4.0);

  for (_name, material, avg_per_chunk) in [
    ("Cedar river ground stone", block![stone], 1.5),
    ("Cedar river ground andesite", block![stone[variant = "andesite"]], 1.5),
    ("Cedar river ground gravel", block![gravel], 2.0),
    ("Cedar river ground cobblestone", block![cobblestone], 1.5),
  ] {
    g.place_chunk(
      chunk_placer::Splotch {
        below_replace: [block![dirt], block![gravel], block![sand]].into(),
        depth: 1..=3,
        replace: [
          block![grass],
          block![dirt],
          block![dirt[variant = "coarse_dirt"]],
          block![dirt[variant = "podzol"]],
        ]
        .into(),
        place: material,
        radius: 1..=2,
        avg_per_chunk,
      },
    );
  }

  // Cedar river mixed stone and gravel patches
  g.place_chunk(
    chunk_placer::MixedSplotch {
      below_replace: [block![dirt], block![gravel], block![sand]].into(),
      depth: 1..=3,
      replace: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
      ]
      .into(),
      place: vec![block![gravel], block![mossy_cobblestone], block![cobblestone], block![stone[variant = "andesite"]]],
      radius: 2..=3,
      avg_per_chunk: 1.5,
    },
  );

  for material in [block![clay], block![stone[variant = "andesite"]], block![stone], block![cobblestone]] {
    g.place(
      "Cedar river bed patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: [block![gravel], block![sand], block![stone]].into(),
        place: material,
        radius: 1..=3,
        attempts: 18,
        avg_per_chunk: 0.65,
      },
    );
  }

  // Cedar river reeds
  g.place_icing(
    icing_placer::Reeds { avg_per_chunk: 5.0, attempts: 64, must_be_one_block_deep: true },
  );
}

pub fn lebanon_cedar_woodland_canyon(g: &mut BiomeBuilder) {
  lebanon_cedar_woodland(g);
}

pub fn lebanon_cedar_woodland_hills(g: &mut BiomeBuilder) {
  lebanon_cedar_woodland(g);
  add_cedar_boulders(g, 4.0);
  g.place_chunk(
    chunk_placer::RockSurface {
      avg_per_chunk: 0.8,
      size: 8,
      mix_in: vec![
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
      ],
    },
  );
}

pub fn lebanon_cedar_woodland_mountains(g: &mut BiomeBuilder) {
  lebanon_cedar_woodland(g);
  g.layers[1].min_depth = 1;
  g.layers[1].max_depth = 2;
  g.place_chunk(
    chunk_placer::RockSurface {
      avg_per_chunk: 0.8,
      size: 12,
      mix_in: vec![
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
      ],
    },
  );
}

pub fn lebanon_cedar_woodland_beach(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![gravel]);
  g.add_layer(block![gravel], 2, 4);
  for (_name, place, radius, avg_per_chunk) in [
    ("Beach coarse dirt spots", block![dirt[variant = "coarse_dirt"]], 1..=2, 2.0),
    ("Beach grass spots", block![grass], 2..=4, 3.75),
  ] {
    g.place_chunk(
      chunk_placer::Splotch {
        below_replace: [block![dirt], block![gravel], block![sand]].into(),
        depth: if place == block![grass] { 1..=1 } else { 1..=3 },
        replace: block![gravel].into(),
        place,
        radius,
        avg_per_chunk,
      },
    );
  }
  add_cedar_boulders(g, 4.0);

  add_cedar_trees(g, (0, 50, 50), 7.5, true);
  add_cedar_tall_bushes(g, 6.0);
  g.place(
    "Cedar beach shoreline clay",
    PlacerStage::Sand2,
    placer::ShorelineSplotch {
      replace: [block![gravel], block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![clay],
      radius: 2..=3,
      attempts: 40,
      avg_per_chunk: 1.2,
    },
  );
  // Beach moss carpet patches
  g.place_icing(
    icing_placer::Spread {
      replace: [block![grass], block![gravel], block![clay], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![rgen:mossy_carpet],
      radius: 1..=3,
      avg_per_chunk: 1.2,
    },
  );
  // Beach mossy bushes
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![rgen:plant],
      avg_per_chunk: 1.0,
      attempts: 80,
    },
  );
  // Beach ferns
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![tallgrass[type = "fern"]],
      avg_per_chunk: 1.0,
      attempts: 60,
    },
  );
  // Beach tall grass clumps
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: block![grass].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 2..=3,
      attempts: 25,
      avg_per_chunk: 1.2,
    },
  );
  // Rare beach green dried grass
  g.place_icing(
    icing_placer::Scatter {
      place_above: block![grass].into(),
      place: block![rgen:dried_grass[variant = "green_1"]],
      avg_per_chunk: 1.0,
      attempts: 15,
    },
  );
  // Sparse beach salt marsh grass
  g.place_icing(
    icing_placer::Clumps {
      place_above: [block![sand], block![grass]].into(),
      place: block![rgen:salt_marsh_grass],
      radius: 2..=3,
      attempts: 30,
      avg_per_chunk: 0.8,
    },
  );
  // Scattered beach salt marsh grass
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![grass], block![sand]].into(),
      place: block![rgen:salt_marsh_grass],
      avg_per_chunk: 1.0,
      attempts: 80,
    },
  );
  g.place(
    "Beach mossy cobblestone",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: [block![grass], block![sand], block![gravel]].into(),
      place: block![rgen:mossy_cobblestone_rgen],
      avg_per_chunk: 1.5,
      attempts: 40,
    },
  );

  for (material, avg_per_chunk) in [(block![gravel], 1.4), (block![clay], 0.7), (block![stone], 0.7)] {
    g.place(
      "Cedar beach material patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: [block![sand], block![gravel], block![stone]].into(),
        place: material,
        radius: 1..=3,
        attempts: 16,
        avg_per_chunk,
      },
    );
  }
}

pub fn riparian_woodland(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#779C83";
  g.set_top_block(block![gravel]);
  g.add_layer(block![gravel], 3, 5);
  g.set_underwater_block(block![gravel]);
  g.add_underwater_layer(block![gravel], 2, 4);

  // Scattered patches vary the gravel floor. Grass and coarse dirt support plants.
  for (_name, place, radius, avg_per_chunk) in [
    ("Riparian coarse dirt flecks", block![dirt[variant = "coarse_dirt"]], 1..=2, 10.0),
    ("Riparian grass flecks", block![grass], 1..=3, 5.0),
    ("Riparian clay flecks", block![clay], 1..=3, 5.0),
    ("Riparian andesite flecks", block![stone[variant = "andesite"]], 1..=3, 4.0),
    ("Riparian stone flecks", block![stone], 1..=3, 4.0),
    ("Riparian mossy cobblestone flecks", block![mossy_cobblestone], 1..=3, 3.0),
  ] {
    g.place_chunk(
      chunk_placer::Splotch {
        below_replace: [block![dirt], block![gravel], block![sand]].into(),
        depth: if place == block![grass] { 1..=1 } else { 1..=3 },
        replace: block![gravel].into(),
        place,
        radius,
        avg_per_chunk,
      },
    );
  }

  // Riparian moss carpet
  g.place_icing(
    icing_placer::Spread {
      replace: [block![gravel], block![mossy_cobblestone], block![stone], block![stone[variant = "andesite"]]].into(),
      place: block![rgen:mossy_carpet],
      radius: 2..=4,
      avg_per_chunk: 1.5,
    },
  );
  // Riparian mossy bushes
  g.place_icing(
    icing_placer::Scatter {
      place_above: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![gravel],
        block![clay],
        block![mossy_cobblestone],
      ]
      .into(),
      place: block![rgen:plant],
      avg_per_chunk: 1.5,
      attempts: 80,
    },
  );
  g.place(
    "Riparian boulders",
    PlacerStage::Tree,
    placer::MossBoulder { avg_per_chunk: 1.8, ..placer::MossBoulder::default() },
  );
  let willow_ground: BlockFilter = [
    block![gravel],
    block![grass],
    block![dirt],
    block![dirt[variant = "coarse_dirt"]],
    block![clay],
    block![stone],
    block![stone[variant = "andesite"]],
    block![cobblestone],
    block![mossy_cobblestone],
    block![sand],
  ]
  .into();
  g.place(
    "Riparian willow sites",
    PlacerStage::Tree,
    placer::WillowTree {
      ground: willow_ground.clone(),
      plantable: willow_ground.clone(),
      plants: [
        block![tallgrass],
        block![double_plant],
        block![rgen:plant],
        block![rgen:mossy_carpet],
        block![red_flower],
        block![yellow_flower],
      ]
      .into(),
      // Birch is temporary wood while the willow geometry is being designed.
      log: block![log[variant = "birch"]],
      leaves: block![leaves[variant = "birch", check_decay = false, decayable = false]],
      avg_per_chunk: 6.0,
    },
  );
  let willow_plants: BlockFilter = [
    block![tallgrass], block![double_plant], block![rgen:plant],
    block![rgen:mossy_carpet], block![red_flower], block![yellow_flower],
  ].into();
  g.place(
    "Birch tall bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: willow_ground.clone(),
      plantable: willow_ground.clone(),
      plants: willow_plants.clone(),
      log: block![log[variant = "birch"]],
      leaves: block![leaves[variant = "birch", check_decay = false, decayable = false]],
      allow_water_under_leaves: true,
      avg_per_chunk: 6.0,
    },
  );
  g.place(
    "Aspen tall bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: willow_ground.clone(),
      plantable: willow_ground.clone(),
      plants: willow_plants,
      log: block![log[variant = "birch"]],
      leaves: block![rgen:leaves3[variant = "aspen", check_decay = false, decayable = false]],
      allow_water_under_leaves: true,
      avg_per_chunk: 4.0,
    },
  );
  g.place(
    "Riparian aspen trees",
    PlacerStage::Tree,
    placer::AspenTree {
      place_above: willow_ground.clone(),
      trunk: block![log[variant = "birch"]],
      leaves: block![rgen:leaves3[variant = "aspen", check_decay = false, decayable = false]],
      avg_per_chunk: 1.0,
      ..placer::AspenTree::default()
    },
  );
  g.place(
    "Fallen birch logs",
    PlacerStage::Tree,
    placer::LogAndStump {
      log: block![log[variant = "birch"]],
      moss_log: block![rgen:covered_birch_log[axis = "none"]],
      ground: block![gravel],
      plants: [block![tallgrass], block![double_plant], block![rgen:plant]].into(),
      avg_per_chunk: 0.6,
      chance_of_moss: 7,
      is_shrooms: false,
      shroom: block![rgen:polypore],
    },
  );
  g.place(
    "Rare riparian cobblestone slabs",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: block![gravel].into(),
      place: block![stone_slab[variant = "cobblestone", half = "bottom"]],
      avg_per_chunk: 1.0,
      attempts: 3,
    },
  );
  // Riparian tall grass
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 2..=4,
      attempts: 55,
      avg_per_chunk: 4.0,
    },
  );
  // Riparian fern clumps
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place_short: block![tallgrass[type = "fern"]],
      place_tall_lower: block![double_plant[variant = "double_fern", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 2..=4,
      attempts: 45,
      avg_per_chunk: 3.0,
    },
  );
  // Riparian tall ferns
  g.place_icing(
    icing_placer::DoubleScatter {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place_lower: block![double_plant[variant = "double_fern", half = "lower"]],
      place_upper: block![double_plant[half = "upper"]],
      avg_per_chunk: 1.2,
      attempts: 120,
    },
  );
  // Riparian reeds
  g.place_icing(
    icing_placer::Reeds { avg_per_chunk: 4.0, attempts: 64, must_be_one_block_deep: true },
  );
}

pub fn chaparral_flats(g: &mut BiomeBuilder) {
  build_chaparral_flats(g, None);
}

pub fn chaparral_flats_river(g: &mut BiomeBuilder) {
  build_chaparral_flats(g, Some(5.0));
  g.set_underwater_block(block![gravel]);
  g.add_underwater_layer(block![sand], 1, 2);
  g.add_underwater_layer(block![stone], 1, 2);

  g.place(
    "Chaparral river gravel bank flecks",
    PlacerStage::Sand2,
    placer::ShorelineSplotch {
      replace: [block![grass], block![dirt]].into(),
      place: block![gravel],
      radius: 2..=4,
      attempts: 24,
      avg_per_chunk: 0.7,
    },
  );
  g.place(
    "Chaparral river stone bank flecks",
    PlacerStage::Sand2,
    placer::ShorelineSplotch {
      replace: [block![grass], block![dirt]].into(),
      place: block![stone],
      radius: 1..=3,
      attempts: 20,
      avg_per_chunk: 0.4,
    },
  );
  // Chaparral river reeds
  g.place_icing(
    icing_placer::Reeds { avg_per_chunk: 9.0, attempts: 64, must_be_one_block_deep: true },
  );
  g.place(
    "Chaparral river basic oak trees",
    PlacerStage::Tree,
    placer::BasicTree {
      place_above: block![grass].into(),
      trunk: block![log[variant = "oak"]],
      leaves: block![leaves[variant = "oak"]],
      avg_per_chunk: 1.3,
    },
  );
}

pub fn chaparral_flats_canyon(g: &mut BiomeBuilder) {
  build_chaparral_flats(g, None);
}

pub fn chaparral_flats_hills(g: &mut BiomeBuilder) {
  build_chaparral_flats(g, None);
  add_chaparral_rock_surface(g, 8);
}

pub fn chaparral_flats_mountains(g: &mut BiomeBuilder) {
  build_chaparral_flats(g, Some(0.4));
  // The mountain variant exposes its stone surface after a thin soil layer.
  g.add_layer(block![dirt], 1, 2);
  add_chaparral_rock_surface(g, 12);
}

fn build_chaparral_flats(g: &mut BiomeBuilder, tall_bush_rate: Option<f64>) {
  g.id = biome![savanna];
  g.color = "#A9A06B";
  g.set_top_block(block![grass]);

  let below_mix = vec![
    block![stone], block![stone], block![stone], block![stone],
    block![stone], block![stone], block![stone], block![stone],
    block![stone[variant = "andesite"]], block![gravel],
  ];
  let below_replace: BlockFilter = [
    block![dirt],
    block![dirt[variant = "coarse_dirt"]],
    block![dirt[variant = "podzol"]],
  ]
  .into();
  g.place_chunk(
    chunk_placer::SubsurfaceSplotch {
      surface_replace: block![grass].into(),
      below_replace: below_replace.clone(),
      place: block![gravel],
      below_mix: below_mix.clone(),
      radius: 1..=3,
      depth: 1..=3,
      avg_per_chunk: 1.0,
      stretch_chance: 25,
    },
  );
  g.place_chunk(
    chunk_placer::SubsurfaceSplotch {
      surface_replace: block![grass].into(),
      below_replace,
      place: block![sand[variant = "red_sand"]],
      below_mix: vec![block![red_sandstone], block![sand[variant = "red_sand"]]],
      radius: 1..=3,
      depth: 1..=3,
      avg_per_chunk: 0.6,
      stretch_chance: 25,
    },
  );
  // Chaparral dead bushes on red sand
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![sand[variant = "red_sand"]]].into(),
      place: block![tallgrass[type = "dead_bush"]],
      avg_per_chunk: 1.0,
      attempts: 80,
    },
  );
  // Chaparral dried grass patches
  g.place_icing(
    icing_placer::DriedGrassClumps {
      place_above: [block![grass]].into(),
      radius: 2..=4,
      attempts: 40,
      avg_per_chunk: 2.0,
    },
  );
  // Chaparral tall grass patches
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 2..=4,
      attempts: 40,
      avg_per_chunk: 1.5,
    },
  );
  // Chaparral yellow wildflower shrubs
  g.place_icing(
    icing_placer::Scatter {
      place_above: block![grass].into(),
      place: block![rgen:yellow_wildflower_shrub],
      avg_per_chunk: 0.25,
      attempts: 80,
    },
  );
  g.place(
    "Chaparral leaf clumps",
    PlacerStage::Tree,
    placer::LeafClumps {
      place_above: block![grass].into(),
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "Chaparral fallen oak logs and stumps",
    PlacerStage::Tree,
    placer::LogAndStump {
      log: block![log[variant = "oak"]],
      moss_log: block![log[axis = "y", variant = "oak"]],
      ground: block![grass],
      plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
      avg_per_chunk: 1.2,
      chance_of_moss: 10,
      is_shrooms: false,
      shroom: block![rgen:polypore],
    },
  );

  if let Some(avg_per_chunk) = tall_bush_rate {
    add_chaparral_tall_bushes(g, avg_per_chunk);
  }

}

fn add_chaparral_tall_bushes(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  let ground: BlockFilter = [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into();
  g.place(
    "Chaparral tall bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: ground.clone(),
      plantable: [block![grass], block![dirt]].into(),
      plants: [
        block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower],
        block![rgen:yellow_wildflower_shrub], block![rgen:dried_grass], block![rgen:double_tall_dried_grass],
      ]
      .into(),
      log: block![log[axis = "y", variant = "oak"]],
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = true]],
      allow_water_under_leaves: false,
      avg_per_chunk,
    },
  );
}

fn add_chaparral_rock_surface(g: &mut BiomeBuilder, size: u8) {
  g.place_chunk(
    chunk_placer::RockSurface {
      avg_per_chunk: 0.8,
      size,
      mix_in: vec![block![gravel], block![stone], block![cobblestone], block![stone[variant = "andesite"]]],
    },
  );
}

pub fn coastal_dunes(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D8C783";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 3, 3);
  g.set_underwater_block(block![sand]);

  // Rare coastal dune grass patches
  g.place_chunk(
    chunk_placer::Splotch {
      below_replace: [block![dirt], block![gravel], block![sand]].into(),
      depth: 1..=1,
      replace: block![sand].into(),
      place: block![grass],
      radius: 1..=2,
      avg_per_chunk: 0.6,
    },
  );
  g.place(
    "Common coastal oak stumps and logs",
    PlacerStage::Tree,
    placer::LogAndStump {
      log: block![log[variant = "oak"]],
      moss_log: block![log[axis = "y", variant = "oak"]],
      ground: block![sand],
      plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
      avg_per_chunk: 1.0,
      chance_of_moss: 10,
      is_shrooms: false,
      shroom: block![rgen:polypore],
    },
  );
  for variant in ["a", "b", "c"] {
    // Common coastal dune grass
  g.place_icing(
    icing_placer::Scatter {
        place_above: [block![sand], block![grass]].into(),
        place: block![rgen:dune_grass[variant = variant]],
        avg_per_chunk: 1.5,
        attempts: 60,
      },
    );
  }
  // Rare coastal salt marsh grass
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![sand], block![grass]].into(),
      place: block![rgen:salt_marsh_grass],
      avg_per_chunk: 0.05,
      attempts: 60,
    },
  );
  // Rare coastal yellow wildflower shrubs
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![sand], block![grass]].into(),
      place: block![rgen:yellow_wildflower_shrub],
      avg_per_chunk: 0.05,
      attempts: 60,
    },
  );
  // Rare coastal tall grass
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: block![grass].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 1..=2,
      attempts: 15,
      avg_per_chunk: 0.5,
    },
  );
}

/// Blocks considered Salt Marsh plants, shared by vegetation placers.
pub fn salt_marsh_plants() -> BlockFilter {
  [
    block![tallgrass],
    block![double_plant],
    block![red_flower],
    block![yellow_flower],
    block![rgen:glasswort],
    block![rgen:salt_marsh_grass],
    block![rgen:dried_grass],
    block![rgen:double_tall_dried_grass],
  ]
  .into()
}

pub fn salt_marsh(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#A99A67";
  g.set_top_block(block![grass]);
  g.set_underwater_block(block![sand]);

  g.place(
    "Tall marsh bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [block![grass], block![dirt], block![gravel], block![rgen:mud]].into(),
      plantable: [block![grass], block![dirt], block![gravel]].into(),
      plants: salt_marsh_plants(),
      log: block![log[axis = "y", variant = "oak"]],
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = true]],
      allow_water_under_leaves: false,
      avg_per_chunk: 4.25,
    },
  );

  g.place(
    "Fallen oak logs",
    PlacerStage::Tree,
    placer::LongLog {
      avg_per_chunk: 0.6,
      ground: block![grass].into(),
      log: block![log[variant = "oak"]],
    },
  );
  // Mud patches
  g.place_chunk(
    chunk_placer::Splotch {
      below_replace: [block![dirt], block![gravel], block![sand]].into(),
      depth: 1..=1,
      replace: block![grass].into(),
      place: block![rgen:mud],
      radius: 2..=5,
      avg_per_chunk: 1.2,
    },
  );
  // Gravel patches
  g.place_chunk(
    chunk_placer::Splotch {
      below_replace: [block![dirt], block![gravel], block![sand]].into(),
      depth: 1..=1,
      replace: block![grass].into(),
      place: block![gravel],
      radius: 2..=5,
      avg_per_chunk: 1.5,
    },
  );
  // Rare sand patches
  g.place_chunk(
    chunk_placer::Splotch {
      below_replace: [block![dirt], block![gravel], block![sand]].into(),
      depth: 1..=1,
      replace: block![grass].into(),
      place: block![sand],
      radius: 1..=3,
      avg_per_chunk: 0.5,
    },
  );

  g.place(
    "Glasswort",
    PlacerStage::Tree,
    placer::Glasswort {
      avg_per_chunk: 1.0,
      attempts: 320,
    },
  );

  g.place(
    "Gravel patches under marsh channels",
    PlacerStage::Sand,
    placer::WaterbedPatches {
      replace: [block![sand], block![rgen:mud], block![clay]].into(),
      place: block![gravel],
      radius: 2..=4,
      attempts: 36,
      avg_per_chunk: 1.0,
    },
  );

  // Reeds
  g.place_icing(
    icing_placer::Reeds {
      avg_per_chunk: 6.0,
      attempts: 64,
      must_be_one_block_deep: true,
    },
  );

  // Salt marsh dried grass clumps
  g.place_icing(
    icing_placer::DriedGrassClumps {
      place_above: [block![sand], block![grass]].into(),
      radius: 3..=5,
      attempts: 45,
      avg_per_chunk: 4.6,
    },
  );

  // Salt marsh grass clumps
  g.place_icing(
    icing_placer::Clumps {
      place_above: [block![sand], block![grass]].into(),
      place: block![rgen:salt_marsh_grass],
      radius: 2..=5,
      attempts: 45,
      avg_per_chunk: 2.0,
    },
  );

  // Salt marsh grasses
  g.place_icing(
    icing_placer::Scatter {
      avg_per_chunk: 1.0,
      attempts: 320,
      place_above: [block![grass]].into(),
      place: block![tallgrass[type = "tall_grass"]],
    },
  );
  // Double tall grass
  g.place_icing(
    icing_placer::DoubleScatter {
      avg_per_chunk: 0.2,
      attempts: 320,
      place_above: [block![grass]].into(),
      // Vanilla metadata 2 is lower double-grass; 10 is upper.
      place_lower: block![double_plant[2]],
      place_upper: block![double_plant[half = "upper"]],
    },
  );
}

#[derive(Clone, Copy)]
struct PlacerProfile {
  enabled: bool,
  avg_per_chunk: f64,
}

#[derive(Clone, Copy)]
struct OpenPlainsProfile {
  dry_grass_avg_per_chunk: f64,
  tall_grass_avg_per_chunk: f64,
  coarse_dirt: PlacerProfile,
  gravel_patches: PlacerProfile,
  dry_bush: PlacerProfile,
  fallen_logs: PlacerProfile,
  river_features: bool,
}

const OPEN_PLAINS_BASE: OpenPlainsProfile = OpenPlainsProfile {
  dry_grass_avg_per_chunk: 3.0,
  tall_grass_avg_per_chunk: 1.2,
  coarse_dirt: PlacerProfile { enabled: true, avg_per_chunk: 0.2 },
  gravel_patches: PlacerProfile { enabled: true, avg_per_chunk: 0.12 },
  dry_bush: PlacerProfile { enabled: true, avg_per_chunk: 0.75 },
  fallen_logs: PlacerProfile { enabled: true, avg_per_chunk: 0.15 },
  river_features: false,
};

const OPEN_PLAINS_STANDARD: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_RIVER: OpenPlainsProfile = OpenPlainsProfile {
  dry_bush: PlacerProfile { avg_per_chunk: 3.75, ..OPEN_PLAINS_BASE.dry_bush },
  river_features: true,
  ..OPEN_PLAINS_BASE
};
const OPEN_PLAINS_CANYON: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_HILLS: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_MOUNTAINS: OpenPlainsProfile = OpenPlainsProfile {
  dry_bush: PlacerProfile { enabled: false, ..OPEN_PLAINS_BASE.dry_bush },
  ..OPEN_PLAINS_BASE
};

pub fn open_plains_standard(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_STANDARD);
}

pub fn open_plains_river(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_RIVER);
}

pub fn open_plains_canyon(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_CANYON);
}

pub fn open_plains_hills(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_HILLS);
}

pub fn open_plains_mountains(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_MOUNTAINS);
  g.layers[1].min_depth = 1;
  g.layers[1].max_depth = 2;
}

fn build_open_plains(g: &mut BiomeBuilder, profile: OpenPlainsProfile) {
  g.id = biome![savanna];
  g.color = "#A9B477";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);

  if profile.river_features {
    g.set_underwater_block(block![sand]);
    g.add_underwater_layer(block![sand], 1, 2);
  }

  if profile.coarse_dirt.enabled {
    // Open plains coarse dirt patches
    g.place_chunk(
      chunk_placer::Splotch {
        below_replace: [block![dirt], block![gravel], block![sand]].into(),
        depth: 1..=3,
        replace: [block![grass], block![dirt]].into(),
        place: block![dirt[variant = "coarse_dirt"]],
        radius: 1..=3,
        avg_per_chunk: profile.coarse_dirt.avg_per_chunk,
      },
    );
  }

  if profile.gravel_patches.enabled {
    // Open plains gravel patches
    g.place_chunk(
      chunk_placer::Splotch {
        below_replace: [block![dirt], block![gravel], block![sand]].into(),
        depth: 1..=3,
        replace: [block![grass], block![dirt]].into(),
        place: block![gravel],
        radius: 1..=3,
        avg_per_chunk: profile.gravel_patches.avg_per_chunk,
      },
    );
  }

  add_open_plains_dry_grass(g, profile.dry_grass_avg_per_chunk);

  // Open plains tall grass clumps
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[2]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 4..=8,
      attempts: 110,
      avg_per_chunk: profile.tall_grass_avg_per_chunk,
    },
  );

  if profile.dry_bush.enabled {
    g.place(
      "Open plains dry bushes",
      PlacerStage::Tree,
      placer::BasicDryBush {
        avg_per_chunk: profile.dry_bush.avg_per_chunk,
        leaves: block![leaves[variant = "oak", check_decay = false, decayable = true]],
        place_above: block![grass].into(),
        ground: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into(),
        plants: [
          block![tallgrass],
          block![double_plant],
          block![yellow_flower],
          block![red_flower],
          block![rgen:dried_grass],
          block![rgen:double_tall_dried_grass],
        ]
        .into(),
        trunk: block![log[variant = "oak"]],
      },
    );
  }

  if profile.fallen_logs.enabled {
    g.place(
      "Open plains fallen logs",
      PlacerStage::Tree,
      placer::LongLog {
        avg_per_chunk: profile.fallen_logs.avg_per_chunk,
        ground: [block![grass], block![dirt]].into(),
        log: block![log[variant = "oak"]],
      },
    );
  }

  if profile.river_features {
    // Open plains yellow wildflower shrubs
  g.place_icing(
    icing_placer::Scatter {
        avg_per_chunk: 0.9,
        attempts: 120,
        place_above: [block![grass]].into(),
        place: block![rgen:yellow_wildflower_shrub],
      },
    );

    // Open plains dandelions
  g.place_icing(
    icing_placer::Scatter {
        avg_per_chunk: 0.8,
        attempts: 120,
        place_above: [block![grass]].into(),
        place: block![yellow_flower[type = "dandelion"]],
      },
    );

    // Rare open plains oxeye daisies
  g.place_icing(
    icing_placer::Scatter {
        avg_per_chunk: 0.15,
        attempts: 30,
        place_above: [block![grass]].into(),
        place: block![red_flower[type = "oxeye_daisy"]],
      },
    );

    g.place(
      "Open plains river gravel patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: block![sand].into(),
        place: block![gravel],
        radius: 1..=3,
        attempts: 12,
        avg_per_chunk: 0.6,
      },
    );

    g.place(
      "Open plains river clay patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: block![sand].into(),
        place: block![clay],
        radius: 1..=3,
        attempts: 10,
        avg_per_chunk: 0.4,
      },
    );
  }
}

fn add_cedar_boulders(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  g.place(
    "Cedar woodland boulders",
    PlacerStage::Tree,
    placer::MossBoulder { avg_per_chunk, ..placer::MossBoulder::default() },
  );
}

fn add_cedar_woodland_standard(g: &mut BiomeBuilder, boulder_rate: f64, trees: bool, tree_mix: (u8, u8, u8), tree_rate: f64) {
  // Common coarse dirt and gravel, with less frequent podzol pockets.
  for (_name, place, radius, avg_per_chunk) in [
    ("Frequent coarse dirt patches", block![dirt[variant = "coarse_dirt"]], 2..=3, 8.0),
    ("Gravel patches", block![gravel], 1..=2, 5.0),
    ("Podzol patches", block![dirt[variant = "podzol"]], 2..=5, 0.8),
  ] {
    g.place_chunk(
      chunk_placer::Splotch {
        below_replace: [block![dirt], block![gravel], block![sand]].into(),
        depth: if place == block![grass] { 1..=1 } else { 1..=3 },
        replace: [block![grass], block![dirt]].into(),
        place,
        radius,
        avg_per_chunk,
      },
    );
  }

  add_cedar_boulders(g, boulder_rate);

  // Dried grass clumps
  g.place_icing(
    icing_placer::DriedGrassClumps {
      place_above: [block![grass]].into(),
      radius: 2..=4,
      attempts: 36,
      avg_per_chunk: 1.5,
    },
  );
  // Grass clumps
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 3..=6,
      attempts: 50,
      avg_per_chunk: 3.0,
    },
  );
  // Fern clumps
  g.place_icing(
    icing_placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "fern"]],
      place_tall_lower: block![double_plant[variant = "double_fern", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 3..=6,
      attempts: 40,
      avg_per_chunk: 2.0,
    },
  );
  // Brown mushrooms
  g.place_icing(
    icing_placer::Scatter {
      place_above: [block![grass], block![dirt], block![dirt[variant = "podzol"]]].into(),
      place: block![brown_mushroom],
      avg_per_chunk: 0.8,
      attempts: 40,
    },
  );

  g.place(
    "Fallen cedar logs",
    PlacerStage::Tree,
    placer::LogAndStump {
      log: block![rgen:log[variant = "cedar"]],
      moss_log: block![rgen:covered_cedar_log[axis = "none"]],
      ground: block![grass],
      plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
      avg_per_chunk: 0.45,
      chance_of_moss: 5,
      is_shrooms: true,
      shroom: block![rgen:polypore[type = "one", facing = "north"]],
    },
  );

  if trees {
    add_cedar_trees(g, tree_mix, tree_rate, false);
  }
}

fn add_cedar_trees(g: &mut BiomeBuilder, tree_mix: (u8, u8, u8), avg_per_chunk: f64, grow_on_gravel: bool) {
  let mut growable_mix: BlockFilter = [
    block![grass],
    block![dirt],
    block![dirt[variant = "coarse_dirt"]],
    block![dirt[variant = "podzol"]],
  ]
  .into();
  if grow_on_gravel {
    growable_mix = [
      block![grass],
      block![dirt],
      block![dirt[variant = "coarse_dirt"]],
      block![dirt[variant = "podzol"]],
      block![gravel],
    ]
    .into();
  }
  g.place(
    "Cedar trees",
    PlacerStage::Tree,
    placer::CedarTree {
      ground_mix: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
        block![gravel],
      ]
      .into(),
      growable_mix,
      plant_mix: [
        block![tallgrass],
        block![double_plant],
        block![red_flower],
        block![yellow_flower],
        block![brown_mushroom],
        block![rgen:dried_grass],
        block![rgen:double_tall_dried_grass],
      ]
      .into(),
      trunk: block![rgen:log[variant = "cedar"]],
      leaves: block![rgen:leaves[variant = "cedar", check_decay = false, decayable = true]],
      large_chance: tree_mix.0,
      medium_chance: tree_mix.1,
      small_chance: tree_mix.2,
      avg_per_chunk,
    },
  );
}

fn add_cedar_tall_bushes(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  g.place(
    "Cedar tall bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
        block![gravel],
        block![clay],
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
      ]
      .into(),
      plantable: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
        block![gravel],
      ]
      .into(),
      plants: [
        block![tallgrass],
        block![double_plant],
        block![red_flower],
        block![yellow_flower],
        block![brown_mushroom],
        block![rgen:dried_grass],
        block![rgen:double_tall_dried_grass],
      ]
      .into(),
      log: block![rgen:log[axis = "y", variant = "cedar"]],
      leaves: block![rgen:leaves[variant = "cedar", check_decay = false, decayable = true]],
      allow_water_under_leaves: false,
      avg_per_chunk,
    },
  );
}

fn add_open_plains_dry_grass(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  // Open plains dried grass clumps
  g.place_icing(
    icing_placer::DriedGrassClumps {
      place_above: [block![grass]].into(),
      radius: 1..=2,
      attempts: 25,
      avg_per_chunk,
    },
  );
}
