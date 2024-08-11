//! Defines the JNI interface.

#![allow(non_snake_case)]

use std::{ffi::CStr, process::Command};

use jni::{
  objects::{JByteArray, JCharArray, JClass, JValue},
  sys::{jbyte, jint, jlong, jobjectArray, jstring},
  JNIEnv,
};
use libc::{c_void, dlclose, dlerror, dlmopen, dlsym, LM_ID_NEWLM, RTLD_LOCAL, RTLD_NOW};
use parking_lot::RwLock;

// FIXME: Specify this path in `build.gradle`, somehow.
// const PWD: &str = env!("PWD");
const PWD: &str = "/home/macmv/Desktop/programming/minecraft/mods/rgen-1.12/rgen";

macro_rules! functions {
  (
    $(
      fn $name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty;
    )*
  ) => {
    struct Symbols {
      handle: *mut c_void,

      rgen_get_seed: fn() -> u64,

      $(
        $name: fn($($arg_ty),*) -> $ret,
      )*
    }

    $(
      #[no_mangle]
      pub extern "system" fn $name($($arg: $arg_ty),*) -> $ret {
        symbols(|s| {
          (s.$name)($($arg),*)
        })
      }
    )*

    impl Symbols {
      unsafe fn load(handle: *mut c_void) -> Self {
        Self {
          handle,

          rgen_get_seed: std::mem::transmute(sym(handle, CStr::from_bytes_with_nul_unchecked(b"rgen_get_seed\0"))),

          $(
            $name: std::mem::transmute(sym(handle, CStr::from_bytes_with_nul_unchecked(concat!(stringify!($name), "\0").as_bytes()))),
          )*
        }
      }
    }
  };
}

unsafe impl Send for Symbols {}
unsafe impl Sync for Symbols {}

functions! {
  fn Java_net_macmv_rgen_rust_RustGenerator_init_1generator(
    env: JNIEnv,
    class: JClass,
    seed: jlong,
  ) -> ();

  fn Java_net_macmv_rgen_rust_RustGenerator_build_1chunk(
    env: JNIEnv,
    class: JClass,
    data: JCharArray,
    chunk_x: jint,
    chunk_z: jint,
  ) -> ();

  fn Java_net_macmv_rgen_rust_RustGenerator_build_1biomes(
    env: JNIEnv,
    class: JClass,
    biomes: JByteArray,
    chunk_x: jint,
    chunk_z: jint,
  ) -> ();

  fn Java_net_macmv_rgen_rust_RustGenerator_build_1biomes_1region(
    env: JNIEnv,
    class: JClass,
    biomes: JByteArray,
    cell_x: jint,
    cell_z: jint,
    width: jint,
    height: jint,
  ) -> ();

  fn Java_net_macmv_rgen_rust_RustGenerator_debug_1info(
    env: JNIEnv,
    class: JClass,
    block_x: jint,
    block_y: jint,
    block_z: jint,
  ) -> jobjectArray;

  fn Java_net_macmv_rgen_rust_RustGenerator_get_1biome_1at(
    env: JNIEnv,
    class: JClass,
    block_x: jint,
    block_z: jint,
  ) -> jbyte;

  fn Java_net_macmv_rgen_rust_RustGenerator_get_1biome_1name_1at(
    env: JNIEnv,
    class: JClass,
    block_x: jint,
    block_y: jint,
    block_z: jint,
  ) -> jstring;
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_init(_env: JNIEnv, _class: JClass) {
  let mut s = SYMBOLS.write();
  if s.is_some() {
    panic!("Library already initialized");
  }
  *s = Some(load());
}

#[no_mangle]
pub extern "system" fn Java_net_macmv_rgen_rust_RustGenerator_reload_1generator(
  mut env: JNIEnv,
  class: JClass,
) -> jint {
  match check() {
    Ok(m) => print_warnings(&mut env, &m),
    Err(m) => {
      print_errors(&mut env, &m);
      return 1;
    }
  };

  let mut s = SYMBOLS.write();
  if let Some(s) = s.as_mut() {
    let seed = (s.rgen_get_seed)();

    // We're holding onto the symbols lock, so nothing can access those symbols
    // while we're messing with the file. Still, best practice is to unload them
    // before messing with the file.
    unload(s);
    recompile();
    *s = load();

    // And re-initialize the new generator.
    (s.Java_net_macmv_rgen_rust_RustGenerator_init_1generator)(env, class, seed as i64);
  } else {
    panic!("Library not initialized");
  }

  0
}

static SYMBOLS: RwLock<Option<Symbols>> = RwLock::new(None);

fn symbols<R>(f: impl FnOnce(&Symbols) -> R) -> R {
  let s = SYMBOLS.read();
  f(&s.as_ref().expect("Library not initialized"))
}

fn unload(s: &Symbols) {
  unsafe {
    let res = dlclose(s.handle);

    if res < 0 {
      let err = dlerror();
      let err = CStr::from_ptr(err).to_str().unwrap();

      panic!("Failed to load library: {err}");
    }
  }
}

fn load() -> Symbols {
  unsafe {
    let ptr = dlmopen(
      LM_ID_NEWLM, // make sure to give it a new namespace
      CStr::from_bytes_with_nul_unchecked(
        concat!(env!("PWD"), "/target/release/librgen_jni_impl.so\0").as_bytes(),
      )
      .as_ptr(),
      RTLD_NOW | RTLD_LOCAL,
    );

    if ptr.is_null() {
      let err = dlerror();
      let err = CStr::from_ptr(err).to_str().unwrap();

      panic!("Failed to load library: {err}");
    }

    Symbols::load(ptr)
  }
}

unsafe fn sym(ptr: *mut c_void, name: &CStr) -> *mut c_void {
  unsafe {
    let sym = dlsym(ptr, name.as_ptr());

    if sym.is_null() {
      let err = dlerror();
      let err = CStr::from_ptr(err).to_str().unwrap();

      panic!("Failed to load symbol: {err}");
    }

    sym
  }
}

fn check() -> Result<String, String> {
  let res = Command::new("cargo")
    .arg("check")
    .arg("-p")
    .arg("rgen-jni-impl")
    .current_dir(env!("PWD"))
    .output()
    .unwrap();

  if res.status.success() {
    let stderr = String::from_utf8_lossy(&res.stderr);
    Ok(stderr.to_string())
  } else {
    let stderr = String::from_utf8_lossy(&res.stderr);
    Err(stderr.to_string())
  }
}

fn recompile() {
  Command::new("cargo")
    .arg("build")
    .arg("--release")
    .arg("-p")
    .arg("rgen-jni-impl")
    .current_dir(env!("PWD"))
    .status()
    .unwrap();
}

fn print_warnings(env: &mut JNIEnv, m: &str) {
  let message = env.new_string(m).unwrap();

  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "print_warnings",
      "(Ljava/lang/String;)V",
      &[JValue::Object(&message.into())],
    )
    .unwrap();
}

fn print_errors(env: &mut JNIEnv, m: &str) {
  let message = env.new_string(m).unwrap();

  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "print_errors",
      "(Ljava/lang/String;)V",
      &[JValue::Object(&message.into())],
    )
    .unwrap();
}
