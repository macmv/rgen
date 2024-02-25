//! Defines the JNI interface.

use jni::{
  objects::JClass,
  sys::{jint, jstring},
  JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_build_1chunk(
  env: JNIEnv,
  _class: JClass,
  x: jint,
  z: jint,
) -> jstring {
  let output = format!("building chunk for {x}, {z}");

  env.new_string(output).unwrap().into_raw()
}
