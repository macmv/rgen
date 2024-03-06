//! Defines the JNI interface.

use jni::{
  objects::{JByteArray, JCharArray, JClass, JValue},
  sys::{jint, jlong, jobjectArray},
  JNIEnv,
};
use rgen_world::Generator;

use crate::{ctx::Context, ChunkContext};
use rgen_base::{Biome, Biomes, BlockInfo, Blocks, ChunkPos, Pos};

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

fn lookup_block(env: &mut JNIEnv, name: &str) -> BlockInfo {
  match lookup_id_opt(env, name) {
    Some(id) => BlockInfo::temp_new(name, id),
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

fn lookup_biome(env: &mut JNIEnv, name: &str) -> Biome {
  match lookup_biome_id_opt(env, name) {
    Some(id) => Biome::from_raw_id(id),
    None => panic!("biome not found: {}", name),
  }
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_init_1generator(
  mut env: JNIEnv,
  _class: JClass,
  seed: jlong,
) {
  let blocks = Blocks::init(|name| lookup_block(&mut env, name));
  let biomes = Biomes::init(|name| lookup_biome(&mut env, name));
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

  Context::run(|ctx| {
    let chunk_ctx =
      ChunkContext { chunk_pos: ChunkPos::new(chunk_x, chunk_z), blocks: &ctx.context.blocks };

    // FIXME: This really shouldn't grab a lock on the whole world. Not sure how to
    // fix though.
    println!("generating chunk at {:?}", chunk_ctx.chunk_pos);

    let mut world = ctx.world.lock().unwrap();
    let chunk = world.generate(&ctx.context, &ctx.generator, chunk_ctx.chunk_pos);
    env.set_char_array_region(data, 0, chunk.data()).unwrap();
  });
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_build_1biomes(
  env: JNIEnv,
  _class: JClass,
  biomes: JByteArray,
  chunk_x: jint,
  chunk_z: jint,
) {
  let len = env.get_array_length(&biomes).unwrap();
  assert_eq!(len, 256, "biomes array must be 256 elements long");

  let mut biome_out = [0; 256];

  Context::run(|ctx| {
    let chunk_ctx =
      ChunkContext { chunk_pos: ChunkPos::new(chunk_x, chunk_z), blocks: &ctx.context.blocks };

    println!("generating chunk at {:?}", chunk_ctx.chunk_pos);
    ctx.generator.generate_biomes(chunk_ctx.chunk_pos, &mut biome_out);
  });

  let biome_i8s = unsafe { &*(&biome_out as *const [u8] as *const [i8]) };

  env.set_byte_array_region(biomes, 0, &biome_i8s).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_debug_1info(
  mut env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_y: jint,
  block_z: jint,
) -> jobjectArray {
  let pos = Pos::new(block_x, block_y as u8, block_z);

  let lines = Context::run(|ctx| {
    let continentalness = ctx.generator.biomes.sample_continentalness(ctx.generator.seed, pos);

    [format!("continentalness: {continentalness:.5}")]
  });

  let mut arr = env
    .new_object_array(lines.len() as i32, "java/lang/String", env.new_string("").unwrap())
    .unwrap();
  for (i, line) in lines.iter().enumerate() {
    env.set_object_array_element(&mut arr, i as i32, env.new_string(line).unwrap()).unwrap();
  }
  arr.as_raw()
}
