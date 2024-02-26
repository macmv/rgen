use std::collections::HashMap;

use climate::Climate;
use rgen_base::{Blocks, Chunk, ChunkPos, Pos};
use rgen_placer::{Placer, Random, Rng, World};

mod biome;
mod climate;

pub struct BiomeBuilder {
  placers: Vec<Box<dyn Placer>>,
}

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

impl BiomeBuilder {
  pub fn new() -> Self { Self { placers: vec![] } }

  pub fn place(&mut self, name: &str, stage: PlacerStage, placer: impl Placer + 'static) {
    // TODO: Do we even need name? Its a pain to add them later, so I'm keeping them
    // for now.
    let _ = name;

    self.place0(stage, Box::new(placer));
  }

  // Don't monomorphise this.
  fn place0(&mut self, _stage: PlacerStage, placer: Box<dyn Placer>) {
    // TODO: Using the stage, insert this at the right spot.
    self.placers.push(placer);
  }

  pub fn generate(&self, _blocks: &Blocks, chunk_pos: ChunkPos, chunk: &mut rgen_base::Chunk) {
    let mut world = World::new(chunk_pos, chunk);
    let mut rng = Rng::new(1234);

    for placer in &self.placers {
      for _ in 0..placer.amount_per_chunk() {
        let mut pos = chunk_pos.min_block_pos()
          + Pos::new(rng.rand_exclusive(0, 16), 255, rng.rand_exclusive(0, 16));
        while pos.y > 0 && world.get(pos) == rgen_base::Block::AIR {
          pos.y -= 1;
        }
        println!("placing at {:?}", pos);

        placer.place(&mut world, &mut rng, pos);
      }
    }
  }
}

pub struct Biomes {
  climates: HashMap<Climate, Vec<BiomeBuilder>>,
}

impl Biomes {
  pub fn new(blocks: &Blocks) -> Self {
    let mut climates = HashMap::new();

    climates.insert(Climate::Tundra, vec![]);

    Biomes { climates }
  }

  pub fn generate(&self, blocks: &Blocks, pos: ChunkPos, chunk: &mut Chunk) {
    let mut biome = BiomeBuilder::new();
    biome::lush_swamp(blocks, &mut biome);
    biome.generate(blocks, pos, chunk);
  }
}
