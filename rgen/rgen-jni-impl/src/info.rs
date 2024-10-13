use jni::{objects::JValue, JNIEnv};
use rgen_base::{BlockData, BlockId, BlockKind};
use rgen_world::BlockInfoSupplier;

pub fn lookup_block_info(env: &mut JNIEnv) -> BlockInfoSupplier {
  let mut supplier = BlockInfoSupplier::default();
  read(&mut supplier, env);
  supplier
}

fn read(info: &mut BlockInfoSupplier, env: &mut JNIEnv) {
  for kind in BlockKind::ALL {
    let id = call_block_name_to_id(env, kind.name());

    if id != 0 {
      info.lookup.insert(*kind, BlockId(id as u16));
    }
  }

  let max_id = call_max_block_id(env);

  info.info.insert(
    BlockId::AIR,
    BlockData {
      name:         "air".to_string(),
      block:        Some(BlockKind::Air),
      default_meta: 0,
    },
  );

  // Lookup all the block infos, and skip air.
  for id in 1..=max_id {
    let name = call_block_id_to_name(env, id);
    let block = BlockKind::by_name(&name);

    info.info.insert(
      BlockId(id as u16),
      BlockData { name, block, default_meta: call_lookup_default_meta(env, id) as u8 },
    );
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
