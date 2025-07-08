# Advanced Setup
This file is for users who have an understanding about git, forge, and rust

## First time building

This mod compiles to both 1.12.2 and the latest version of fabric. This requires an extra step to build it:

```
git checkout setup-v12-cache
./gradlew build
git checkout main
./gradlew build
```

Forge gradle requires a plugin to download the vanilla jars and deobfuscate them. However, this gradle plugin is fundamentally incompatible with fabric. So, I've opted to remove the forge gradle plugin, which works fine for recompiling the mod, it just doesn't work the first time the mod is built.

## Building

Building is a bit of a pain. If all you want is the client jar, just set your java to java 8, and then build:
```
./gradlew build
```

The resulting jar is in `./build/libs/rgen-1.0.jar`. This can be loaded into a Minecraft instance, and the mod will work as normal, but the RGen terrain option will not work.

### Developing Assets

If you want to develop textures and/or items, you can run a vanilla client with the `runClient` command:
```
./gradlew runClient
```

### Developing Terrain Generation

This requires building the rust library. This is a bit of a manual process at the moment. Go ahead and build the library:
```
cd rgen
cargo build
```

Then, you'll need to add the built binary into your library path. Add something like this to your JVM options:
```
-Djava.library.path=/path/to/repo/rgen/target/debug
```

Once you've added that, run the client with
```
./gradlew runClient
```

Note that this will not work with MultiMC, because MultiMC overwrites the `java.library.path`. I don't know how to get it to work with MultiMC.

### Fixing the null pointer thing

Delete `~/.gradle` lmao.
