cargo build

INSTANCE=~/.local/share/multimc/instances/1.12.2-mod-testing

rm -r $INSTANCE/.minecraft/saves/1/region

mkdir $INSTANCE/natives
ln -s $PWD/target/debug/librgen_jni.so $INSTANCE/natives
