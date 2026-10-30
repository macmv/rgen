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
  `./gradlew :mod-fabric:1.21.1:build` for example. There is only 1.21.1 for now.

Both builds share the sources in `common/`. For fabric, `v12-to-fabric` translates them first (build
it with `cargo build --release` in `v12-to-fabric/`), and the result is written to
`mod-fabric/src/main/java` (ignored by git). Stonecutter then handles the differences between
minecraft versions, using comments like `//? if vanilla_moss {`. These are just comments to 1.12.

### Porting the common sources to fabric

Gradle needs Java 25 or newer for the fabric build (loom requires it), even though the mod itself
compiles to Java 21.

The transformer handles the mechanical parts: renaming classes and imports, and adding the
`ShapeContext` argument to shape methods. It also removes the pieces that only exist in 1.12 (block
metadata, `getSubBlocks`, etc). Each removed method leaves a `// v12-to-fabric: dropped` comment in
the generated source, so find the things that still need porting with:

```
grep -r "v12-to-fabric: dropped" mod-fabric/src/main/java
```

To port one, take its name out of `drop_methods` in `v12-to-fabric/src/main.rs` (or scope it to a
class, like `Sapling::grow`), and write the fabric version next to the 1.12 one with the markers
below. Files that are entirely 1.12 start with `// # only v12 #`, and are skipped.

- `/* #fabric: <code> */` is replaced with `<code>` on fabric.
- `// #v12-start` and `// #v12-end` wrap lines that are commented out on fabric.
- `//? if >=1.21.2 {` picks between minecraft versions (see below). Code inside `/* */` here has to
  be written for fabric, as the transformer doesn't touch comments.

Anything that is too different goes in a handwritten fabric file, which needs to be allowed in
`mod-fabric/src/main/java/.gitignore`. `compat/` has polyfills for the 1.12 names that the common
sources still use (`Material`, `SoundType`, `CreativeTabs`), and base classes that hide differences
between minecraft versions (`LeavesBase`, `FallingBase`).

The assets in `common/src/main/resources` use the modern layout (`textures/block`, `textures/item`,
and blockstates that name models with their full path, like `rgen:block/wood/fir/fir_log`). Fabric
uses them as they are, and `mod-v12` converts them to the 1.12 layout when it builds (see
`mod-v12/assets.gradle`). Write new assets in the modern layout.

The fabric item models and language file are made by the data generator, not by hand. After adding
a block or item, run `./gradlew :mod-fabric:1.21.1:runDatagen` and commit the result in
`mod-fabric/src/main/generated`. Names there are just the registry names in title case.

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
