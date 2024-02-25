use jni::{
  objects::{JClass, JString},
  sys::jstring,
  JNIEnv,
};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_hello(
  mut env: JNIEnv,
  _class: JClass,
  input: JString,
) -> jstring {
  let input: String = env.get_string(&input).unwrap().into();

  let output = format!("Hello, {}!", input);

  env.new_string(output).unwrap().into_raw()
}
