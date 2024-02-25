//! Defines the JNI interface.

use jni::{
  objects::{JCharArray, JClass, JObject, JValue},
  sys::{jint, jlong},
  JNIEnv,
};

use crate::{chunk::Chunk, ctx::Context, pos::ChunkRelPos};

fn lookup_id_opt(env: &mut JNIEnv, block_ids: &JObject, name: &str) -> Option<i32> {
  let jname = env.new_string(name).unwrap();

  // This is effectively `block_ids.get(Blocks.STONE.getDefaultState())`

  let block = env
    .call_static_method(
      "net/minecraft/block/Block",
      "getBlockFromName",
      "(Ljava/lang/String;)Lnet/minecraft/block/Block;",
      &[JValue::Object(&jname.into())],
    )
    .unwrap()
    .l()
    .unwrap();

  if block.is_null() {
    return None;
  }

  let state = env
    .call_method(&block, "getDefaultState", "()Lnet/minecraft/block/state/IBlockState;", &[])
    .unwrap()
    .l()
    .unwrap();

  let id = env
    .call_method(block_ids, "get", "(Ljava/lang/Object;)I", &[JValue::Object(&state)])
    .unwrap()
    .i()
    .unwrap();

  Some(id)
}

fn lookup_id(env: &mut JNIEnv, block_ids: &JObject, name: &str) -> i32 {
  match lookup_id_opt(env, block_ids, name) {
    Some(id) => id,
    None => panic!("block not found: {}", name),
  }
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_init_1generator(
  mut env: JNIEnv,
  _class: JClass,
  block_ids: JObject, // ObjectIntIdentityMap<IBlockState>
  seed: jlong,
) {
  Context::init(|name| lookup_id(&mut env, &block_ids, name), seed);
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_build_1chunk(
  env: JNIEnv,
  _class: JClass,
  data: JCharArray,
  chunk_x: jint,
  chunk_z: jint,
) {
  let len = env.get_array_length(&data).unwrap();
  assert_eq!(len, 65536, "data array must be 65536 elements long");

  let mut chunk = Chunk::new();

  Context::run(|ctx| {
    ctx.generator.generate(chunk_x, chunk_z, &ctx.blocks, &mut chunk);
  });

  env.set_char_array_region(data, 0, chunk.data()).unwrap();
}
