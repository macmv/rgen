# Project Layout

The RGen project is organized into a few different Rust crates. Each crate serves a specific
purpose, and then can all be found under the `rgen` directory. In no particular order:

## Core Crates:
These are the fundamental crates that define the bulk of the functionality in RGen.

### `rgen-base`

This crate is depended on my almost every other crate. It defines core concepts, like `BlockState`s,
`Biome`s, and block positions with `Pos`.

This code shouldn't really change all that much, because basic concepts like blocks don't have a lot
left to change.

This crate also defines two very useful macros, `block![]` and `biome![]`. These can be used to
create a block state. For example, `block![minecraft:stone[5]]` will create the block state for
andesite (as andesite is `stone` with the data value `5`).

### `rgen-world`

This crate defines the world. The main "world" struct is a `PartialWorld`. This is a partial world
because it may or may not have any number of chunks in the actual world. Chunks that have been
generated a long time ago will get cleaned up, as they are stored in minecraft, and no longer used
by RGen.

### `rgen-placer`

This crate defines placers. Placers are the fundamental building block (no pun intended) of terrain
generation. A placer will place decorations. For example, a placer might place a tree, or some tall
grass, or a small water pool.

### `rgen-biome`

This crate has the real meat and potatoes of terrain generation. It generates a chunk, and defines
biomes for those chunks. The biomes then use placers from `rgen-placer` to build decorations.

### `rgen-jni`

This crate is a thin wrapper over `rgen-biome`, which interfaces directly with java to send chunks
back and forth.

## Utility crates:
These are other miscellaneous utilities we like to keep around.

### `rgen-llama`

This is a structure format. It parses something called a `.ll` (llama) file, and turns that into a
`Structure` which can be placed in the world. For example, village houses are defined in llama
files, as they are large, fixed structures that would be difficult to place with rust code directly.

### `rgen-spline`

Splines are essentially a smoothed line. Given an input number, you can get a smoothed output,
which is transformed in interesting ways. This is used frequently in `rgen-biome` to define height
maps. It lets us build things like spiky mountains, smooth valleys, and sprawling rivers easily.

### `spline-editor`

This is an application that lets you create splines with a user interface.

### `rgen-viewer`

This is an application that shows the world from the top-down view, to get a quick look at where
biomes are placed. It lets you see thousands of blocks at a time, so its good for getting a large
scale view of the world.
