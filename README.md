# RGen

A terrain generation mod for Minecraft 1.12.2, which generates terrain in Rust.

# Building

Building is a bit of a pain. If all you want is the client jar, just set your java to java 8, and then build:
```
./gradlew build
```

The resulting jar is in `./build/libs/rgen-1.0.jar`. This can be loaded into a Minecraft instance, and the mod will work as normal, but the RGen terrain option will not work.

## Developing Assets

If you want to develop textures and/or items, you can run a vanilla client with the `runClient` command:
```
./gradlew runClient
```

## Developing Terrain Generation

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
