use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos};

use crate::{BiomeCachedChunk, IcingPlacer, Random, Rng, chunk_placer::util::for_each_clump_point};

/// Clusters of a single plant.
pub struct Clumps {
  pub place_above: BlockFilter,
  pub place:       BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

/// Clusters of short and tall grass.
pub struct GrassClumps {
  pub place_above:      BlockFilter,
  pub place_short:      BlockState,
  pub place_tall_lower: BlockState,
  pub place_tall_upper: BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl GrassClumps {
  pub fn new() -> Self {
    GrassClumps {
      place_above:      block![grass].into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=10,
      attempts:      60,
      avg_per_chunk: 3.0,
    }
  }
}

impl IcingPlacer for Clumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    for_each_clump_point(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      &self.radius,
      self.attempts,
      |column, _| {
        for i in 0..chunk.chunk.surfaces(column).len() {
          let below = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
          let pos = below.with_y(below.y() + 1);

          if pos.y() <= 255
            && chunk.is_active(below)
            && self.place_above.contains(chunk.get(below))
            && chunk.get(pos) == block![air]
          {
            chunk.set(pos, self.place);
          }
        }
      },
    );
  }
}

impl IcingPlacer for GrassClumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    for_each_clump_point(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      &self.radius,
      self.attempts,
      |column, seed| {
        let mut rng = Rng::new(seed);

        for i in 0..chunk.chunk.surfaces(column).len() {
          let below = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
          let pos = below.with_y(below.y() + 1);
          let above = below.with_y(below.y() + 2);

          if pos.y() <= 255
            && chunk.is_active(below)
            && self.place_above.contains(chunk.get(below))
            && chunk.get(pos) == block![air]
          {
            let height = *rng.choose(&[1, 1, 1, 1, 1, 1, 2]);

            if height == 1 {
              chunk.set(pos, self.place_short);
            } else if above.y() <= 255 && chunk.get(above) == block![air] {
              chunk.set(pos, self.place_tall_lower);
              chunk.set(above, self.place_tall_upper);
            }
          }
        }
      },
    );
  }
}

pub struct DriedGrassClumps {
  pub place_above:   BlockFilter,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl IcingPlacer for DriedGrassClumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    self.place_colors(chunk, rng, chunk_pos, &[0, 1, 2], true);
  }
}

struct BrownYellowGrassClumps(DriedGrassClumps);

impl IcingPlacer for BrownYellowGrassClumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    self.0.place_colors(chunk, rng, chunk_pos, &[0, 2], true);
  }
}

struct BrownYellowShortGrassClumps(DriedGrassClumps);

impl IcingPlacer for BrownYellowShortGrassClumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    self.0.place_colors(chunk, rng, chunk_pos, &[0, 2], false);
  }
}

impl DriedGrassClumps {
  /// Keep height and shape variation, but use only brown and yellow plants.
  pub fn without_green(self) -> impl IcingPlacer { BrownYellowGrassClumps(self) }

  /// Use only short brown and yellow plants.
  pub fn brown_yellow_short_only(self) -> impl IcingPlacer { BrownYellowShortGrassClumps(self) }

  fn place_colors(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos, colors: &[usize], allow_tall: bool) {
    let short = [
      [
        block![rgen:dried_grass[variant = "brown_1"]],
        block![rgen:dried_grass[variant = "brown_2"]],
        block![rgen:dried_grass[variant = "brown_3"]],
      ],
      [
        block![rgen:dried_grass[variant = "green_1"]],
        block![rgen:dried_grass[variant = "green_2"]],
        block![rgen:dried_grass[variant = "green_3"]],
      ],
      [
        block![rgen:dried_grass[variant = "yellow_1"]],
        block![rgen:dried_grass[variant = "yellow_2"]],
        block![rgen:dried_grass[variant = "yellow_3"]],
      ],
    ];
    let tall_lower = [
      [
        block![rgen:double_tall_dried_grass[half = "lower", variant = "brown_1"]],
        block![rgen:double_tall_dried_grass[half = "lower", variant = "brown_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "lower", variant = "green_1"]],
        block![rgen:double_tall_dried_grass[half = "lower", variant = "green_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "lower", variant = "yellow_1"]],
        block![rgen:double_tall_dried_grass[half = "lower", variant = "yellow_2"]],
      ],
    ];
    let tall_upper = [
      [
        block![rgen:double_tall_dried_grass[half = "upper", variant = "brown_1"]],
        block![rgen:double_tall_dried_grass[half = "upper", variant = "brown_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "upper", variant = "green_1"]],
        block![rgen:double_tall_dried_grass[half = "upper", variant = "green_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "upper", variant = "yellow_1"]],
        block![rgen:double_tall_dried_grass[half = "upper", variant = "yellow_2"]],
      ],
    ];

    for_each_clump_point(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      &self.radius,
      self.attempts,
      |column, seed| {
        let mut rng = Rng::new(seed);
        for i in 0..chunk.chunk.surfaces(column).len() {
          let below = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
          let pos = below.with_y(below.y() + 1);
          if pos.y() > 255
            || !chunk.is_active(below)
            || !self.place_above.contains(chunk.get(below))
            || chunk.get(pos) != block![air]
          {
            continue;
          }
          let color = *rng.choose(colors);
          if !allow_tall || rng.range(0..2) == 0 {
            let variant = rng.range(0..3) as usize;
            chunk.set(pos, short[color][variant]);
          } else {
            let above = pos.with_y(pos.y() + 1);
            if above.y() > 255 || chunk.get(above) != block![air] {
              continue;
            }
            let variant = rng.range(0..2) as usize;
            chunk.set(pos, tall_lower[color][variant]);
            chunk.set(above, tall_upper[color][variant]);
          }
        }
      },
    );
  }
}
