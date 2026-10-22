#!/usr/bin/env bash
set -euo pipefail

RGEN_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
INSTANCE="${1:?Usage: multimc.sh /path/to/MultiMC/instance}"

cargo build --release -p rgen-jni -p rgen-jni-impl --manifest-path "$RGEN_ROOT/Cargo.toml"

# Reset generated chunks in the test save so they are regenerated with this build.
rm -rf "$INSTANCE/.minecraft/saves/1/region"

mkdir -p "$INSTANCE/natives"
ln -sfn "$RGEN_ROOT/target/release/librgen_jni.so" "$INSTANCE/natives/librgen_jni.so"
ln -sfn "$RGEN_ROOT/target/release/librgen_jni_impl.so" "$INSTANCE/natives/librgen_jni_impl.so"
