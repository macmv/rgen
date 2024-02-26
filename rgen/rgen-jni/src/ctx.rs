use std::sync::RwLock;

use rgen_base::{Biomes, Blocks};

use crate::generator::Generator;

pub struct Context {
  pub generator: Generator,
  pub blocks:    Blocks,
  pub biomes:    Biomes,
}

static CONTEXT: RwLock<Option<Context>> = RwLock::new(None);

impl Context {
  pub fn init(blocks: Blocks, biomes: Biomes, seed: i64) {
    let generator = Generator::new(&blocks, seed as u64);

    let ctx = Context { generator, blocks, biomes };

    CONTEXT.write().unwrap().replace(ctx);
  }

  pub fn run<R>(f: impl FnOnce(&Context) -> R) -> R {
    let ctx = CONTEXT.read().unwrap();
    let ctx = ctx.as_ref().expect("Context not initialized");
    f(ctx)
  }
}
