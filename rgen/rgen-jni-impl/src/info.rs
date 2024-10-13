use std::collections::HashMap;

use jni::{objects::JValue, JNIEnv};
use rgen_base::{block_kind, Block, BlockId, BlockInfo};
use rgen_world::BlockInfoSupplier;

pub struct JniBlockInfoSupplier {
  lookup: HashMap<Block, BlockId>,
  info:   HashMap<BlockId, BlockInfo>,
}

impl BlockInfoSupplier for JniBlockInfoSupplier {
  fn lookup(&self, kind: Block) -> Option<BlockId> {
    // Air is constant, so we don't cache it.
    if kind == block_kind![air] {
      return Some(BlockId::AIR);
    }

    self.lookup.get(&kind).copied()
  }

  fn get(&self, id: BlockId) -> BlockInfo {
    if id == BlockId::AIR {
      return BlockInfo {
        name:         "air".to_string(),
        block:        Some(block_kind![air]),
        default_meta: 0,
      };
    }

    self
      .info
      .get(&id)
      .unwrap_or_else(|| {
        panic!("no such block with id {id:?}");
      })
      .clone()
  }
}

impl JniBlockInfoSupplier {
  pub fn new(env: &mut JNIEnv) -> Self {
    let mut supplier = JniBlockInfoSupplier { lookup: HashMap::new(), info: HashMap::new() };

    supplier.read(env);
    supplier
  }

  fn read(&mut self, env: &mut JNIEnv) {
    for kind in Block::ALL {
      let id = call_block_name_to_id(env, kind.name());

      if id != 0 {
        self.lookup.insert(*kind, BlockId(id as u16));
      }
    }

    let max_id = call_max_block_id(env);

    // Lookup all the block infos, and skip air.
    for id in 1..=max_id {
      let name = call_block_id_to_name(env, id);
      let block = Block::by_name(&name);

      self.info.insert(
        BlockId(id as u16),
        BlockInfo { name, block, default_meta: call_lookup_default_meta(env, id) as u8 },
      );
    }
  }
}

fn call_block_name_to_id(env: &mut JNIEnv, name: &str) -> i32 {
  let jname = env.new_string(name).unwrap();

  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "block_name_to_id",
      "(Ljava/lang/String;)I",
      &[JValue::Object(&jname.into())],
    )
    .unwrap()
    .i()
    .unwrap()
}

fn call_block_id_to_name(env: &mut JNIEnv, id: i32) -> String {
  let jname = env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "block_id_to_name",
      "(I)Ljava/lang/String;",
      &[JValue::Int(id)],
    )
    .unwrap()
    .l()
    .unwrap();

  env.get_string(&jname.into()).unwrap().into()
}

fn call_max_block_id(env: &mut JNIEnv) -> i32 {
  env
    .call_static_method("net/macmv/rgen/rust/RustGenerator", "max_block_id", "()I", &[])
    .unwrap()
    .i()
    .unwrap()
}

fn call_lookup_default_meta(env: &mut JNIEnv, id: i32) -> i32 {
  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "lookup_default_meta",
      "(I)I",
      &[JValue::Int(id)],
    )
    .unwrap()
    .i()
    .unwrap()
}
