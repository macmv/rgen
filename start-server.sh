set -e

if [ ! -d server ]; then
  echo "Server directory not found, set it up with setup-server.sh"
  exit 1
fi

cd server
rm -rf world

readlink mods/rgen-1.0.jar > /dev/null || ret=$?
if [[ $ret -eq 1 ]]; then
  echo "Mod not found, compile it first"
  exit 1
fi
readlink natives/librgen_jni.so > /dev/null || ret=$?
if [[ $ret -eq 1 ]]; then
  echo "Native library not found, compile it first"
  exit 1
fi

FORGE_VERSION=1.12.2-14.23.5.2860

java \
  -Dfml.queryResult=confirm \
  -Djava.library.path=natives \
  -jar forge-$FORGE_VERSION.jar \
  nogui
