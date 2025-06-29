set -e

rm -rf server
mkdir -p server
cd server

mkdir mods
ln -s ../../build/libs/rgen-1.0.jar mods

mkdir natives
ln -s ../../rgen/target/release/librgen_jni.so natives

FORGE_VERSION=1.12.2-14.23.5.2860

curl -o forge-installer.jar https://maven.minecraftforge.net/net/minecraftforge/forge/$FORGE_VERSION/forge-$FORGE_VERSION-installer.jar
java -jar forge-installer.jar --installServer .
echo "eula=true" > eula.txt

cat > server.properties <<EOF
allow-flight=true
level-seed=1
level-type=rgen
EOF
