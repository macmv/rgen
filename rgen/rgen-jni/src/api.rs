//! Defines the JNI interface.

use jni::{
  objects::{JCharArray, JClass, JValue},
  sys::{jint, jlong},
  JNIEnv,
};

use crate::{ctx::Context, ChunkContext};
use rgen_base::{Biomes, Blocks, Chunk, ChunkPos};

// TODO: Do we need to worry about obfuscated names anymore?
#[cfg(not(feature = "obf-names"))]
fn lookup_id_opt(env: &mut JNIEnv, name: &str) -> Option<i32> {
  let jname = env.new_string(name).unwrap();

  let block = env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "block_name_to_id",
      "(Ljava/lang/String;)I",
      &[JValue::Object(&jname.into())],
    )
    .unwrap()
    .i()
    .unwrap();

  if block == 0 {
    None
  } else {
    Some(block)
  }
}

#[cfg(feature = "obf-names")]
fn lookup_id_opt(env: &mut JNIEnv, block_ids: &JObject, name: &str) -> Option<i32> {
  let Ok(jname) = env.new_string(name) else { return Some(0) };

  // This is effectively `block_ids.get(Blocks.STONE.getDefaultState())`

  let block = match env.call_static_method(
    "net/minecraft/block/Block",
    "func_149684_b", // getBlockFromName
    "(Ljava/lang/String;)Lnet/minecraft/block/Block;",
    &[JValue::Object(&jname.into())],
  ) {
    Ok(block) => block.l().unwrap(),
    Err(_) => return Some(0),
  };

  if block.is_null() {
    return None;
  }

  let state = match env.call_method(
    &block,
    "func_176223_P", // getDefaultState
    "()Lnet/minecraft/block/state/IBlockState;",
    &[],
  ) {
    Ok(state) => state.l().unwrap(),
    Err(_) => return Some(0),
  };

  match env.call_method(
    block_ids,
    "func_148747_b", // get
    "(Ljava/lang/Object;)I",
    &[JValue::Object(&state)],
  ) {
    Ok(id) => Some(id.i().unwrap()),
    Err(_) => return Some(0),
  }
}

fn lookup_id(env: &mut JNIEnv, name: &str) -> i32 {
  match lookup_id_opt(env, name) {
    Some(id) => id,
    None => panic!("block not found: {}", name),
  }
}

fn lookup_biome_id_opt(env: &mut JNIEnv, name: &str) -> Option<i32> {
  let jname = env.new_string(name).unwrap();

  let biome = env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "biome_name_to_id",
      "(Ljava/lang/String;)I",
      &[JValue::Object(&jname.into())],
    )
    .unwrap()
    .i()
    .unwrap();

  if biome == 0 {
    None
  } else {
    Some(biome)
  }
}

fn lookup_biome_id(env: &mut JNIEnv, name: &str) -> i32 {
  match lookup_biome_id_opt(env, name) {
    Some(id) => id,
    None => panic!("biome not found: {}", name),
  }
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_init_1generator(
  mut env: JNIEnv,
  _class: JClass,
  seed: jlong,
) {
  let blocks = Blocks::init(|name| lookup_id(&mut env, name));
  let biomes = Biomes::init(|name| lookup_biome_id(&mut env, name));
  Context::init(blocks, biomes, seed);
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
    let chunk_ctx =
      ChunkContext { chunk_pos: ChunkPos::new(chunk_x, chunk_z), blocks: &ctx.blocks };

    println!("generating chunk at {:?}", chunk_ctx.chunk_pos);
    ctx.generator.generate(&chunk_ctx, &mut chunk);
  });

  env.set_char_array_region(data, 0, chunk.data()).unwrap();
}
