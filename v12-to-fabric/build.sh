#! /bin/sh

set -e

cd $(dirname "$0")
cargo build --release
cd ..
rm -rf mod-fabric/build/transformed-common
./gradlew build
