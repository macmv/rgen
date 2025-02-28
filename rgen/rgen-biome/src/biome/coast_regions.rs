use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

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
pub fn ancient_shores(g: &mut BiomeBuilder) {
  g.id = biome![stone_beach];
  g.color = "#ffffff";
  g.set_top_block(block![sand]);
  g.add_layer(block![stone], 1, 3);

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
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![cobblestone],
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![stone],
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );

  g.place(
    "loose_stone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![stone],
      attempts:    20,
    },
  );
  g.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![mossy_cobblestone],
      attempts:    40,
    },
  );
  g.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: block![stone].into(),
      place:       block![stone],
      attempts:    30,
    },
  );

  g.place("dead_tree", PlacerStage::Tree, placer::DeadTree::new());
}

#[allow(dead_code)]
pub fn dry_shores(g: &mut BiomeBuilder) {
  g.id = biome![stone_beach];
  g.color = "#ffffff";
  g.set_top_block(block![gravel]);
  g.add_layer(block![stone], 1, 3);

  g.place(
    "sand_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![sand],
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![stone],
      radius:        3..=3,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "dead_bushes",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    30,
      place_above: [block![gravel], block![sand], block![stone]].into(),
      place:       block![tallgrass[type = "dead_bush"]],
    },
  )
}
#[allow(dead_code)]
pub fn bare_rock(g: &mut BiomeBuilder) {
  g.id = biome![stone_beach];
  g.color = "#ffffff";
  g.set_top_block(block![stone]);
}
#[allow(dead_code)]
pub fn wet_rock(g: &mut BiomeBuilder) {
  g.id = biome![plains];
  g.color = "#ffffff";
  g.set_top_block(block![stone]);
  g.add_layer(block![stone], 1, 3);

  g.place(
    "grass_splatter",
    PlacerStage::Sand,
    placer::Splatter { replace: g.top_block(), place: block![grass], attempts: 40 },
  );
  g.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![mossy_cobblestone],
      radius:        2..=3,
      avg_per_chunk: 1.0,
    },
  );
}
#[allow(dead_code)]
pub fn sand_beach(g: &mut BiomeBuilder) {
  g.id = biome![beaches];
  g.color = "#ffffff";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 1, 3);

  g.set_underwater_block(block![sand]);

  g.place("palm_tree", PlacerStage::Tree, placer::PalmTree::new());
}
#[allow(dead_code)]
pub fn monument_beach(g: &mut BiomeBuilder) {
  g.id = biome![beaches];
  g.color = "#ffffff";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 1, 3);

  // places monument
  g.place(
    "sand_monument",
    PlacerStage::Tree,
    placer::Monument {
      material:       block![sandstone],
      fancy_material: block![sandstone[type = "chiseled_sandstone"]],
      reward:         block![gold_block],
    },
  )
}

#[allow(dead_code)]
pub fn palm_beach(g: &mut BiomeBuilder) {
  g.id = biome![beaches];
  g.color = "#ffffff";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 1, 3);
}
