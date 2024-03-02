cargo build --release -p rgen-jni

INSTANCE=~/.local/share/multimc/instances/$1

rm -r $INSTANCE/.minecraft/saves/1/region

mkdir $INSTANCE/natives
ln -s $PWD/target/release/librgen_jni.so $INSTANCE/natives
