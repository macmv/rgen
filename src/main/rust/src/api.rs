//! Defines the JNI interface.

use jni::{
  objects::{JCharArray, JClass},
  sys::jint,
  JNIEnv,
};

use crate::{chunk::Chunk, pos::ChunkRelPos};

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_build_1chunk(
  env: JNIEnv,
  _class: JClass,
  data: JCharArray,
  _x: jint,
  _z: jint,
) {
  let len = env.get_array_length(&data).unwrap();
  assert_eq!(len, 65536, "data array must be 65536 elements long");

  let mut chunk = Chunk::new();

  // dirt
  chunk.set(ChunkRelPos::new(0, 5, 0), 3, 0);

  for x in 0..16 {
    for z in 0..16 {
      // stone
      chunk.set(ChunkRelPos::new(x, 0, z), 1, 0);
    }
  }

  env.set_char_array_region(data, 0, chunk.data()).unwrap();
}
