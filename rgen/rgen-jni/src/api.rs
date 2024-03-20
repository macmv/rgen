//! Defines the JNI interface.

use jni::{
  objects::{JByteArray, JCharArray, JClass, JValue},
  sys::{jbyte, jint, jlong, jobjectArray, jstring},
  JNIEnv,
};

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

    ctx.world.generate(chunk_ctx.chunk_pos, |chunk| {
      env.set_char_array_region(data, 0, chunk.data()).unwrap();
    });
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
    let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);

        biome_out[(z << 4 | x) as usize] = ctx.generator.biomes.choose_biome(pos).id.raw_id() as i8;
      }
    }
  });

  env.set_byte_array_region(biomes, 0, &biome_out).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_build_1biomes_1region(
  env: JNIEnv,
  _class: JClass,
  biomes: JByteArray,
  cell_x: jint,
  cell_z: jint,
  width: jint,
  height: jint,
) {
  let len = env.get_array_length(&biomes).unwrap();
  assert_eq!(len, width * height, "biomes array must be 256 elements long");

  let mut biome_out = vec![0; (width * height) as usize];

  Context::run(|ctx| {
    for x in 0..width {
      for z in 0..height {
        let pos = Pos::new((x + cell_x) * 4, 0, (z + cell_z) * 4);

        biome_out[(z * width + x) as usize] =
          ctx.generator.biomes.choose_biome(pos).id.raw_id() as i8;
      }
    }
  });

  env.set_byte_array_region(biomes, 0, &biome_out).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_debug_1info(
  mut env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_y: jint,
  block_z: jint,
) -> jobjectArray {
  let pos = Pos::new(block_x, block_y, block_z);

  let lines = Context::run(|ctx| {
    let biome = ctx.generator.biomes.choose_biome(pos);
    let continentalness = ctx.generator.biomes.sample_continentalness(pos);
    let erosion = ctx.generator.biomes.sample_erosion(pos);
    let peaks_valleys = ctx.generator.biomes.sample_peaks_valleys(pos);

    [
      format!("biome: {}", biome.name),
      format!("continentalness: {continentalness:.5}"),
      format!("erosion: {erosion:.5}"),
      format!("peaks valleys: {peaks_valleys:.5}"),
    ]
  });

  let mut arr = env
    .new_object_array(lines.len() as i32, "java/lang/String", env.new_string("").unwrap())
    .unwrap();
  for (i, line) in lines.iter().enumerate() {
    env.set_object_array_element(&mut arr, i as i32, env.new_string(line).unwrap()).unwrap();
  }
  arr.as_raw()
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_get_1biome_1at(
  _env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_z: jint,
) -> jbyte {
  let pos = Pos::new(block_x, 0, block_z);

  Context::run(|ctx| {
    let biome = ctx.generator.biomes.choose_biome(pos);
    biome.id.raw_id() as i8
  })
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_get_1biome_1name_1at(
  env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_y: jint,
  block_z: jint,
) -> jstring {
  let pos = Pos::new(block_x, block_y, block_z);

  let biome = Context::run(|ctx| {
    let biome = ctx.generator.biomes.choose_biome(pos);

    biome.name.to_string()
  });

  env.new_string(biome).unwrap().as_raw()
}
