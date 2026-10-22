# Advanced Setup
This file is for users who have an understanding about git, forge, and rust

## Build layout

There are two separate gradle builds, because ForgeGradle (1.12.2) doesn't support new versions of
gradle, and we don't want to hold everything else back:

- `mod-v12/` is the 1.12.2 forge mod. Run `./gradlew` from inside that directory. It uses its own
  (old) gradle wrapper, and builds with Java 8 or newer.
- The root build is the fabric mod, in `mod-fabric/`. It needs Java 25 or newer to run gradle.
  [Stonecutter](https://stonecutter.kikugie.dev) builds it for each version listed in
  `settings.gradle` (and `mod-fabric/versions/<version>/gradle.properties`), with
  `./gradlew :mod-fabric:1.21.5:build` for example.

Both builds share the sources in `common/`. For fabric, `v12-to-fabric` translates them first (build
it with `cargo build --release` in `v12-to-fabric/`), and the result is written to
`mod-fabric/src/main/java` (ignored by git). Stonecutter then handles the differences between
minecraft versions, using comments like `//? if vanilla_moss {`. These are just comments to 1.12.

### Backporting blocks

If vanilla added a block that we had to make ourselves (like moss), add a flag for it in
`mod-fabric/build.gradle`, and use that flag to pick between the vanilla block and ours (see
`MossBlock` and `RBlocks.MOSSY_BLOCK`). Rust and the rest of the mod keep using the `rgen:` name,
which `RBlocks.byName` resolves to the vanilla block.

## First time building (1.12.2 only)

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

The resulting jar is in `./build/libs/rgen-1.0.jar`. This is a dev jar: it has no rust code in it, so the RGen terrain option only works if you put the dev libraries on your library path (see "Developing Terrain Generation" below).

To build a jar to ship, add `-Prelease`:
```
./gradlew build -Prelease
```
This runs `cargo build --release -p rgen-jni-impl` and bundles the library into the jar under `natives/<os>-<arch>/`, and the jar loads it directly. It only contains the library for the machine that built it. The `release` GitHub workflow builds every platform and bundles them all into one jar. Release jars can't use `/reload`.

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

Then, you'll need to add the built binaries into your library path (dev jars load `rgen_jni` from there). Both `librgen_jni.so` and `librgen_jni_impl.so` must be in that directory, and hot reloading expects the `release` profile. Add something like this to your JVM options:
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
