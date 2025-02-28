//! Defines the JNI interface.

use std::cell::RefCell;

use jni::{
  JNIEnv,
  objects::{JByteArray, JCharArray, JClass, JValue},
  sys::{jbyte, jint, jlong, jobject, jobjectArray, jstring},
};
use rgen_world::PartialWorldStorage;

use crate::{ctx::Context, lookup_biome_info, lookup_block_info};
use rgen_base::{BiomeId, ChunkPos, Pos, StateId};
use rgen_spline::Cosine;

#[allow(dead_code)]
struct JniWorldStorage<'a, 'b: 'a> {
  env: RefCell<&'a mut JNIEnv<'b>>,
}

impl PartialWorldStorage for JniWorldStorage<'_, '_> {
  fn get(&self, pos: Pos) -> StateId {
    let raw_id = self
      .env
      .borrow_mut()
      .call_static_method(
        "net/macmv/rgen/rust/RustGenerator",
        "get_block",
        "(IIII)S",
        &[JValue::Int(0), JValue::Int(pos.x), JValue::Int(pos.y), JValue::Int(pos.z)],
      )
      .unwrap();

    StateId(raw_id.s().unwrap() as u16)
  }

  fn set(&mut self, pos: Pos, block: StateId) {
    self
      .env
      .borrow_mut()
      .call_static_method(
        "net/macmv/rgen/rust/RustGenerator",
        "set_block",
        "(IIIIS)V",
        &[
          JValue::Int(0),
          JValue::Int(pos.x),
          JValue::Int(pos.y),
          JValue::Int(pos.z),
          JValue::Short(block.0 as i16),
        ],
      )
      .unwrap();
  }

  fn surfaces(&self, _: Pos) -> &[u8] { &[] }
}

/// Initializes the terrain generator for a specific seed. Call this function on
/// each world load.
#[unsafe(no_mangle)]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_init(_env: JNIEnv, _class: JClass) {
  crate::logger::init();

  let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
  let puffin_server = puffin_http::Server::new(&server_addr).unwrap();
  unsafe {
    PUFFIN_SERVER = Some(puffin_server);
  }
  puffin::set_scopes_on(true);
}

static mut PUFFIN_SERVER: Option<puffin_http::Server> = None;

#[unsafe(no_mangle)]
pub extern "system" fn rgen_deinit() {
  // Drop the server, to unbind from the port.
  unsafe {
    PUFFIN_SERVER = None;
  }
}

/// Initializes the terrain generator for a specific seed. Call this function on
/// each world load.
#[unsafe(no_mangle)]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_init_1world(
  mut env: JNIEnv,
  _class: JClass,
  seed: jlong,
) {
  let blocks = lookup_block_info(&mut env);
  let biomes = lookup_biome_info(&mut env);
  Context::init(blocks, biomes, seed);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_wait_1for_1log(
  mut env: JNIEnv,
  _class: JClass,
) -> jobject {
  match crate::logger::poll() {
    Some(log) => {
      let jmessage = env.new_string(log.message).unwrap();
      let obj = env.new_object("net/macmv/rgen/rust/OwnedLog", "()V", &[]).unwrap();

      env.set_field(&obj, "level", "B", (log.level as i8).into()).unwrap();
      env.set_field(&obj, "message", "Ljava/lang/String;", (&jmessage).into()).unwrap();

      obj.into_raw()
    }
    None => std::ptr::null_mut(),
  }
}

#[unsafe(no_mangle)]
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
    puffin::GlobalProfiler::lock().new_frame();

    ctx.world.generate(ChunkPos::new(chunk_x, chunk_z), |chunk| {
      env.set_char_array_region(data, 0, chunk.data()).unwrap();
    });
  });
}

#[unsafe(no_mangle)]
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
        // Neil's stupid ass caves are causing problems again the fix of 255 makes the
        // vanilla biome set to the surface rgen biome rather than the underground
        // bullshit.
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 255, z);

        // FIXME: Translate biome ids!
        let biome = ctx.generator.choose_biome(pos).id;
        biome_out[(z << 4 | x) as usize] =
          ctx.context.biomes.lookup(biome).unwrap_or(BiomeId::VOID).0 as i8;
      }
    }
  });

  env.set_byte_array_region(biomes, 0, &biome_out).unwrap();
}

#[unsafe(no_mangle)]
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
        // Neil's stupid ass caves are causing problems again the fix of 255 makes the
        // vanilla biome set to the surface rgen biome rather than the underground
        // bullshit.
        let pos = Pos::new((x + cell_x) * 4, 255, (z + cell_z) * 4);

        let biome = ctx.generator.choose_biome(pos).id;
        biome_out[(z * width + x) as usize] =
          ctx.context.biomes.lookup(biome).unwrap_or(BiomeId::VOID).0 as i8;
      }
    }
  });

  env.set_byte_array_region(biomes, 0, &biome_out).unwrap();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_debug_1info(
  mut env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_y: jint,
  block_z: jint,
) -> jobjectArray {
  let pos = Pos::new(block_x, block_y, block_z);

  let lines = Context::run(|ctx| {
    let biome = ctx.generator.choose_biome(pos);

    let continentalness = ctx.generator.sample_continentalness(pos);
    let peaks_valleys = ctx.generator.sample_peaks_valleys(pos);
    let erosion = ctx.generator.sample_erosion(pos);

    let continentalness_cat = ctx.generator.continentalness_category(pos);
    let peaks_valleys_cat = ctx.generator.peaks_valleys_category(pos);
    let erosion_cat = ctx.generator.erosion_category(pos);

    let geographic_type = ctx.generator.geographic_type(pos);
    let climate_type = ctx.generator.climate_type(pos);

    let c = rgen_biome::CONTINENTALNESS.sample::<Cosine>(continentalness);
    let i = rgen_biome::HEIGHT_IMPACT.sample::<Cosine>(c / 128.0);
    let p = rgen_biome::PEAKS_VALLEYS.sample::<Cosine>(peaks_valleys);
    let e = rgen_biome::EROSION.sample::<Cosine>(erosion);

    [
      format!("biome: {}", biome.name),
      format!("continentalness: {continentalness_cat:?} ({continentalness:.3})"),
      format!("peaks valleys: {peaks_valleys_cat:?} ({peaks_valleys:.3})"),
      format!("erosion: {erosion_cat} ({erosion:.3})"),
      format!("geo: {geographic_type:?}, clim: {climate_type:?}"),
      format!("c: {c:.3} p: {p:.3} e: {e:.3} i: {i:.3}"),
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

#[unsafe(no_mangle)]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_get_1biome_1at(
  _env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_z: jint,
) -> jbyte {
  let pos = Pos::new(block_x, 0, block_z);

  Context::run(|ctx| {
    let biome = ctx.generator.choose_biome(pos);
    ctx.context.biomes.lookup(biome.id).unwrap_or(BiomeId::VOID).0 as i8
  })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_get_1biome_1name_1at(
  env: JNIEnv,
  _class: JClass,
  block_x: jint,
  block_y: jint,
  block_z: jint,
) -> jstring {
  let pos = Pos::new(block_x, block_y, block_z);

  let biome = Context::run(|ctx| {
    let biome = ctx.generator.choose_biome(pos);

    biome.name.to_string()
  });

  env.new_string(biome).unwrap().as_raw()
}

// This is for re-loading the generator.
#[unsafe(no_mangle)]
pub extern "system" fn rgen_get_seed() -> u64 { Context::run(|ctx| ctx.context.seed) }
