use std::collections::HashMap;

use jni::{
  objects::{JObject, JObjectArray, JValue},
  JNIEnv,
};
use rgen_base::{
  Biome, BiomeId, BlockData, BlockId, BlockKind, PropMapOwned, PropType, PropValueOwned,
};
use rgen_world::{BiomeInfoSupplier, BlockInfoSupplier};

pub fn lookup_block_info(env: &mut JNIEnv) -> BlockInfoSupplier {
  let mut supplier = BlockInfoSupplier::default();
  read_blocks(&mut supplier, env);
  supplier
}

pub fn lookup_biome_info(env: &mut JNIEnv) -> BiomeInfoSupplier {
  let mut supplier = BiomeInfoSupplier::default();
  read_biomes(&mut supplier, env);
  supplier
}

fn read_blocks(info: &mut BlockInfoSupplier, env: &mut JNIEnv) {
  for kind in BlockKind::ALL {
    let id = call_block_name_to_id(env, kind.name());

    if id != 0 {
      info.lookup.insert(*kind, BlockId(id as u16));
    }
  }

  let max_id = call_max_block_id(env);

  info.info.insert(
    BlockId::AIR,
    BlockData {
      name:         "air".to_string(),
      block:        Some(BlockKind::Air),
      default_meta: 0,
      prop_types:   HashMap::new(),
      prop_values:  [const { PropMapOwned::empty() }; 16],
    },
  );

  // Lookup all the block infos, and skip air.
  for id in 1..=max_id {
    let name = call_block_id_to_name(env, id);
    let block = BlockKind::by_name(&name);

    info.info.insert(
      BlockId(id as u16),
      BlockData {
        name,
        block,
        default_meta: call_lookup_default_meta(env, id) as u8,
        prop_types: call_lookup_prop_types(env, id),
        prop_values: call_lookup_prop_values(env, id),
      },
    );
  }
}

fn read_biomes(info: &mut BiomeInfoSupplier, env: &mut JNIEnv) {
  for kind in Biome::ALL {
    let id = call_biome_name_to_id(env, kind.name());

    if id != 0 {
      info.lookup.insert(*kind, BiomeId(id as u8));
    }
  }
}

fn call_block_name_to_id(env: &mut JNIEnv, name: &str) -> i32 {
  let jname = env.new_string(name).unwrap();

  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "block_name_to_id",
      "(Ljava/lang/String;)I",
      &[JValue::Object(&jname.into())],
    )
    .unwrap()
    .i()
    .unwrap()
}

fn call_biome_name_to_id(env: &mut JNIEnv, name: &str) -> i32 {
  let jname = env.new_string(name).unwrap();

  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "biome_name_to_id",
      "(Ljava/lang/String;)I",
      &[JValue::Object(&jname.into())],
    )
    .unwrap()
    .i()
    .unwrap()
}

fn call_block_id_to_name(env: &mut JNIEnv, id: i32) -> String {
  let jname = env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "block_id_to_name",
      "(I)Ljava/lang/String;",
      &[JValue::Int(id)],
    )
    .unwrap()
    .l()
    .unwrap();

  env.get_string(&jname.into()).unwrap().into()
}

fn call_max_block_id(env: &mut JNIEnv) -> i32 {
  env
    .call_static_method("net/macmv/rgen/rust/RustGenerator", "max_block_id", "()I", &[])
    .unwrap()
    .i()
    .unwrap()
}

fn call_lookup_default_meta(env: &mut JNIEnv, id: i32) -> i32 {
  env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "lookup_default_meta",
      "(I)I",
      &[JValue::Int(id)],
    )
    .unwrap()
    .i()
    .unwrap()
}

fn call_lookup_prop_types(env: &mut JNIEnv, id: i32) -> HashMap<String, PropType> {
  let types: JObjectArray = env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "lookup_block_prop_types",
      "(I)[Lnet/macmv/rgen/rust/PropType;",
      &[JValue::Int(id)],
    )
    .unwrap()
    .l()
    .unwrap()
    .into();

  let mut out = HashMap::new();
  for i in 0..env.get_array_length(&types).unwrap() {
    let ty: JObject = env.get_object_array_element(&types, i).unwrap();

    let jname = env.get_field(&ty, "name", "Ljava/lang/String;").unwrap().l().unwrap().into();
    let name = env.get_string(&jname).unwrap().into();

    match env.get_field(&ty, "kind", "I").unwrap().i().unwrap() {
      0 => {
        out.insert(name, PropType::Bool);
      }
      1 => {
        let min = env.get_field(&ty, "min", "I").unwrap().i().unwrap();
        let max = env.get_field(&ty, "max", "I").unwrap().i().unwrap();
        out.insert(name, PropType::Int(min, max));
      }
      2 => {
        let array: JObjectArray =
          env.get_field(&ty, "variants", "[Ljava/lang/String;").unwrap().l().unwrap().into();
        let len = env.get_array_length(&array).unwrap();

        let mut variants = vec![String::new(); len as usize];

        for i in 0..len {
          let jname = env.get_object_array_element(&array, i).unwrap().into();
          variants[i as usize] = env.get_string(&jname).unwrap().into();
        }

        out.insert(name, PropType::Enum(variants));
      }
      v => {
        panic!("unknown prop kind {v} for prop named {name}");
      }
    }
  }

  out
}

fn call_lookup_prop_values(env: &mut JNIEnv, id: i32) -> [PropMapOwned; 16] {
  let types: JObjectArray = env
    .call_static_method(
      "net/macmv/rgen/rust/RustGenerator",
      "lookup_block_prop_values",
      "(I)[Lnet/macmv/rgen/rust/PropMap;",
      &[JValue::Int(id)],
    )
    .unwrap_or_else(|_| {
      env.exception_describe().unwrap();
      panic!();
    })
    .l()
    .unwrap()
    .into();

  let mut out = [const { PropMapOwned::empty() }; 16];
  for i in 0..16 {
    let map: JObject = env.get_object_array_element(&types, i).unwrap();

    let values: JObjectArray =
      env.get_field(map, "values", "[Lnet/macmv/rgen/rust/PropValue;").unwrap().l().unwrap().into();

    for j in 0..env.get_array_length(&values).unwrap() {
      let value: JObject = env.get_object_array_element(&values, j).unwrap();

      let jname = env.get_field(&value, "name", "Ljava/lang/String;").unwrap().l().unwrap().into();
      let name = env.get_string(&jname).unwrap().into();

      let value = match env.get_field(&value, "kind", "I").unwrap().i().unwrap() {
        0 => PropValueOwned::Bool(env.get_field(&value, "bool", "Z").unwrap().z().unwrap()),
        1 => PropValueOwned::Int(env.get_field(&value, "integer", "I").unwrap().i().unwrap()),
        2 => {
          let jstr =
            env.get_field(&value, "str", "Ljava/lang/String;").unwrap().l().unwrap().into();
          let str = env.get_string(&jstr).unwrap().into();

          PropValueOwned::Enum(str)
        }
        v => {
          panic!("unknown prop kind {v} for prop named {name}");
        }
      };

      out[i as usize].entries[j as usize] = (name, value);
    }

    // Keep the entries sorted.
    out[i as usize].entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
  }

  out
}
