cargo build --release -p rgen-jni

INSTANCE=$1
# Jaimie's path: ~/Minecraft/multimcTar/MultiMC/instances/rgenTest

rm -r $INSTANCE/.minecraft/saves/1/region

mkdir $INSTANCE/natives
ln -s $PWD/target/release/librgen_jni.so $INSTANCE/natives
