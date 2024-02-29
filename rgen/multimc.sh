cargo build --release

INSTANCE=~/.local/share/multimc/instances/1.12.2-mod-testing

rm -r $INSTANCE/.minecraft/saves/1/region

mkdir $INSTANCE/natives
ln -s $PWD/target/release/librgen_jni.so $INSTANCE/natives
