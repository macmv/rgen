use rgen_base::{Biome, BlockState, ChunkPos, Pos, block};
use rgen_placer::{BiomeCachedChunk, ChunkPlacer, Placer, Random, Rng, grid::PointGrid};
use rgen_world::PartialWorld;
use smallvec::{SmallVec, smallvec};

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

pub struct BiomeBuilder {
  pub seed:   u64,
  pub name:   &'static str,
  pub rarity: u32,
  pub id:     rgen_base::Biome,
  pub color:  &'static str,

  pub layers:            SmallVec<[Layer; 2]>,
  pub underwater_layers: SmallVec<[Layer; 2]>,

  pub min_height: u32,
  pub max_height: u32,

  // First pass placers. These run on multiple threads, and can only access a single chunk.
  chunk_placers: Vec<Box<dyn ChunkPlacer>>,

  // Second pass placers. These all run on one thread, but can access the 8 surrounding chunks.
  placers: Vec<PlacerBuilder>,
}

pub struct Layer {
  pub state:     BlockState,
  pub min_depth: u32,
  pub max_depth: u32,
}

struct PlacerBuilder {
  placer: Box<dyn Placer>,
  name:   &'static str,
  grid:   PointGrid,
}

impl PlacerBuilder {
  fn new(placer: Box<dyn Placer>, name: &'static str) -> Self {
    Self { placer, name, grid: PointGrid::new() }
  }
}

impl BiomeBuilder {
  pub fn new(seed: u64, name: &'static str, rarity: u32) -> Self {
    Self {
      seed,
      name,
      rarity,
      id: Biome::Void,
      color: "",
      layers: smallvec![Layer { state: block![grass], min_depth: 1, max_depth: 1 }],
      underwater_layers: smallvec![Layer { state: block![gravel], min_depth: 1, max_depth: 1 }],
      min_height: 64,
      max_height: 128,
      placers: vec![],
      chunk_placers: vec![],
    }
  }

  pub fn finish(&mut self) {
    if self.layers.len() == 1 && self.top_block().block == block![grass] {
      self.add_layer(block![dirt], 3, 5);
    }

    // Default underwater layers to being a bit thicker.
    if self.underwater_layers.len() == 1 {
      self.add_underwater_layer(self.underwater_layers[0].state, 1, 3);
    }
  }

  pub fn set_top_block(&mut self, state: BlockState) { self.layers[0].state = state; }
  pub fn add_layer(&mut self, state: BlockState, min_depth: u32, max_depth: u32) {
    self.layers.push(Layer { state, min_depth, max_depth });
  }

  pub fn set_underwater_block(&mut self, state: BlockState) {
    self.underwater_layers[0].state = state;
  }
  pub fn add_underwater_layer(&mut self, state: BlockState, min_depth: u32, max_depth: u32) {
    self.underwater_layers.push(Layer { state, min_depth, max_depth });
  }

  pub fn top_block(&self) -> BlockState { self.layers[0].state }

  pub fn place(&mut self, name: &'static str, stage: PlacerStage, placer: impl Placer + 'static) {
    self.place0(stage, name, Box::new(placer));
  }

  // Don't monomorphise this.
  fn place0(&mut self, _stage: PlacerStage, name: &'static str, placer: Box<dyn Placer>) {
    // TODO: Using the stage, insert this at the right spot.
    self.placers.push(PlacerBuilder::new(placer, name));
  }

  pub fn place_chunk(&mut self, placer: impl ChunkPlacer + 'static) {
    self.chunk_placers.push(Box::new(placer));
  }

  pub fn generate(&self, rng: &mut Rng, chunk: &mut BiomeCachedChunk, chunk_pos: ChunkPos) {
    profile_scope!("generate biome", self.name);

    for placer in self.chunk_placers.iter() {
      placer.place(chunk, rng, chunk_pos);
    }
  }

  /// Decorates the given chunk. The `rng` passed in should only be seeded with
  /// the world seed.
  pub fn decorate(
    &self,
    rng: &mut Rng,
    chunk_pos: ChunkPos,
    world: &mut PartialWorld,
    is_in_chunk: impl Fn(Pos) -> bool,
  ) {
    profile_scope!("decorate biome", self.name);

    for placer in self.placers.iter() {
      profile_scope!("placer", placer.name);

      let seed = rng.next();

      const SCALE: f64 = 1.0 / 16.0;
      let scale = SCALE * placer.placer.avg_per_chunk().powf(0.5);

      let min_x = chunk_pos.min_block_pos().x as f64 * scale;
      let min_y = chunk_pos.min_block_pos().z as f64 * scale;
      let max_x = (chunk_pos.min_block_pos().x + 15) as f64 * scale;
      let max_y = (chunk_pos.min_block_pos().z + 15) as f64 * scale;

      for point in placer.grid.points_in_area(seed, min_x, min_y, max_x, max_y) {
        let pos = Pos::new((point.0 / scale) as i32, 0, (point.1 / scale) as i32);
        // NB: Assume the surfaces won't change in `world.attempt`, and that re-fetching
        // the surfaces is effectively free (which it should be).
        let mut i = 0;
        while let Some(surface) = world.surfaces(pos).get(i) {
          let pos = pos.with_y(*surface as i32 + 1);

          if is_in_chunk(pos) {
            // This builds a unique seed for each placer. This gives the placer the same
            // seed if it crosses chunk boundaries.
            let seed = rng.next() ^ (pos.x as u64) << 32 ^ pos.z as u64;
            world.attempt(|world| placer.placer.place(world, &mut Rng::new(seed), pos));
          }

          i += 1;
        }
      }
    }
  }
}

impl Layer {
  pub fn sample_depth(&self, t: f64) -> u32 {
    if self.min_depth == self.max_depth {
      self.min_depth
    } else {
      ((self.max_depth - self.min_depth) as f64 * t) as u32 + self.min_depth
    }
  }
}
