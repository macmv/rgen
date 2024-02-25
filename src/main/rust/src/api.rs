//! Defines the JNI interface.

use jni::{
  objects::{JCharArray, JClass},
  sys::jint,
  JNIEnv,
};

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

  let mut buf = vec![0; 65536];

  // x=0,y=5,z=0 = Block ID 3 (dirt)
  buf[5] = 0x30;

  let y = 0;
  for x in 0..16 {
    for z in 0..16 {
      buf[z << 12 | x << 8 | y] = 0x10; // stone
    }
  }

  env.set_char_array_region(data, 0, &buf).unwrap();
}
