use std::sync::{Arc, RwLock};

use rgen_biome::WorldBiomes;
use rgen_world::{BiomeInfoSupplier, BlockInfoSupplier, CachedWorld};

pub struct Context {
  pub generator: Arc<WorldBiomes>,
  pub world:     Arc<CachedWorld>,

  pub context: Arc<rgen_world::Context>,
}

static CONTEXT: RwLock<Option<Context>> = RwLock::new(None);

impl Context {
  pub fn init(blocks: BlockInfoSupplier, biomes: BiomeInfoSupplier, seed: i64) {
    let generator = WorldBiomes::new(&blocks, seed as u64);

    let ctx = Context {
      generator: Arc::new(generator),
      world:     Arc::new(CachedWorld::new()),
      context:   Arc::new(rgen_world::Context { seed: seed as u64, blocks, biomes }),
    };

    ctx.world.spawn_threads(&ctx.context, &ctx.generator);

    CONTEXT.write().unwrap().replace(ctx);
  }

  pub fn run<R>(f: impl FnOnce(&Context) -> R) -> R {
    let ctx = CONTEXT.read().unwrap();
    let ctx = ctx.as_ref().expect("Context not initialized");
    f(ctx)
  }
}
