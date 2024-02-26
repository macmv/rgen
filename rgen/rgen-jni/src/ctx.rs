use std::sync::RwLock;

use rgen_base::Blocks;

use crate::generator::Generator;

pub struct Context {
  pub generator: Generator,
  pub blocks:    Blocks,
}

static CONTEXT: RwLock<Option<Context>> = RwLock::new(None);

impl Context {
  pub fn init(lookup_id: impl FnMut(&str) -> i32, seed: i64) {
    let ctx =
      Context { generator: Generator::new(seed as u64), blocks: Blocks::init(lookup_id) };

    CONTEXT.write().unwrap().replace(ctx);
  }

  pub fn run<R>(f: impl FnOnce(&Context) -> R) -> R {
    let ctx = CONTEXT.read().unwrap();
    let ctx = ctx.as_ref().expect("Context not initialized");
    f(ctx)
  }
}
