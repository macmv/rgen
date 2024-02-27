use std::sync::{Mutex, RwLock};

use rgen_base::{Biomes, Blocks};
use rgen_world::PartialWorld;

use crate::generator::TerrainGenerator;

pub struct Context {
  pub generator: TerrainGenerator,
  pub world:     Mutex<PartialWorld>,

  pub context: rgen_world::Context,
}

static CONTEXT: RwLock<Option<Context>> = RwLock::new(None);

impl Context {
  pub fn init(blocks: Blocks, biomes: Biomes, seed: i64) {
    let generator = TerrainGenerator::new(&blocks, &biomes, seed as u64);

    let ctx = Context {
      generator,
      world: Mutex::new(PartialWorld::new()),
      context: rgen_world::Context { seed: seed as u64, blocks, biomes },
    };

    CONTEXT.write().unwrap().replace(ctx);
  }

  pub fn run<R>(f: impl FnOnce(&Context) -> R) -> R {
    let ctx = CONTEXT.read().unwrap();
    let ctx = ctx.as_ref().expect("Context not initialized");
    f(ctx)
  }
}
