# Placers

There are three kinds of placers, and each runs at a different point in a chunk's life. Picking the
right one matters, because it decides what the placer can see, what it can change, how fast it is,
and what it is guaranteed to run before or after.

In short:

- **Chunk placers** are for most decorations: ground materials, boulders, water, ores, grass,
  flowers, and so on.
- **Placers** are for things that cross chunk borders and need to see neighboring chunks. That is
  mostly trees.
- **Icing placers** are for things that strictly need to happen after trees, like bamboo.

If you aren't sure, use a chunk placer. They are the fastest, and the easiest to reason about.

## Summary

|                         | Chunk placer              | Placer                       | Icing placer                |
| ----------------------- | ------------------------- | ---------------------------- | --------------------------- |
| Register with           | `g.place_chunk(..)`       | `g.place(..)`                | `g.place_icing(..)`         |
| Trait                   | `ChunkPlacer`             | `Placer`                     | `IcingPlacer`               |
| Lives in                | `chunk_placer`            | `placer`                     | `icing_placer`              |
| Runs                    | During base generation    | During decoration            | When the chunk is final     |
| Threads                 | Parallel                  | One at a time                | One at a time (for now)     |
| Can read                | Its own chunk             | The 3x3 chunks around it     | Its own chunk               |
| Can write               | Its own chunk             | The 3x3 chunks around it     | Its own chunk               |
| Can see trees           | No (none exist yet)       | Only ones that already ran   | Yes, all of them            |
| Can cross chunk borders | Only by replaying (below) | Yes                          | Only by replaying (below)   |
| Typical use             | Most decorations          | Trees                        | Bamboo                      |

## The order things happen in

For a single chunk, in this order:

1. **Base generation.** Terrain is generated, then all the chunk placers run on it. Chunks are
   generated in parallel, and a chunk placer never looks at another chunk.
2. **Decoration.** This runs once the chunk and its neighbors all have base terrain. Every placer
   runs, on a single thread. They can read and write the neighboring chunks.
3. **Icing.** This runs once the chunk, and all 8 of its neighbors, have finished decoration. At
   this point nothing can write to the chunk anymore, so it is final. Then the icing placers run.

Within one pass, placers run in the order they were added to the biome. Across passes, the order is
always chunk placers, then placers, then icing placers.

Chunks do not decorate in a predictable order. This is the important part: a tree from chunk A can
land in chunk B before, or after, B's own placers run, depending on which chunk happened to finish
first. That is why there are three kinds, and why you can't just order placers within decoration to
get the result you want. Only the icing pass is guaranteed to see every tree.

## Chunk placers

A chunk placer decorates a single chunk, during base generation. It is handed the chunk and a
seeded `Rng`, and it can only touch that chunk. It knows where the ground is (the chunk has its
own surfaces), and it knows which biome each column belongs to (use `chunk.is_active(pos)` before
placing anything, so a placer only affects the biome it was added to).

Use a chunk placer for anything that:

- Only needs to look at the ground, and the blocks right around it.
- Doesn't care what trees do, and that trees can build on top of afterwards.

For example: ground patches, material changes, boulders, pools, ores, snow, moss, grass, ferns, and
flowers.

They are the fastest kind. Chunks generate on many threads, and a chunk placer runs there too.

### Features that cross chunk borders

A chunk placer can't see the next chunk, so it can't build something that crosses into it directly.
Instead, each chunk works out what it needs by itself. A boulder centered in the next chunk is
placed by that chunk, but this chunk also works out that the boulder is there, and places the part
that lands in this chunk. Both chunks agree, because everything about the feature comes from the
seed and its position, not from the state of the world.

`chunk_placer/util.rs` has the tools for this:

- `blobs_near` returns every round feature that might touch the chunk, including ones centered in
  the 8 neighbors.
- `for_each_clump_point` does the same for clumps of plants.
- `chunk_rng`, `count`, and `mix` make a seed that depends on the chunk, or a position, without
  depending on what ran earlier.

Never use an `Rng` that depends on what earlier placers did when building something that crosses a
border. The two chunks would disagree, and there would be a seam.

Because a chunk placer runs before any tree exists, it knows nothing about trees. Anything that
needs to avoid, or go under or around, a tree is not a chunk placer. See icing placers below.

## Placers

A placer places a single feature at a position, and can read and write the 3x3 chunks around the
chunk it was run for. It is handed a `PartialWorld`, which acts like a world made of the loaded
chunks around it.

Use a placer for anything that is bigger than a chunk, or that can't be worked out from the seed
alone. That is almost always trees: a tree needs the real ground height, needs to check that
there's room, and builds a large shape that crosses borders.

The things to know about them:

- **`radius()`** is how far, in blocks, the placer might place from its position. It must not be
  more than a chunk (16), because the world only waits for the 8 neighboring chunks before
  decorating.
- **Failing.** A placer can return `Err(UndoError)` to cancel everything it placed. This is how
  trees decide there isn't room.
- **They are single threaded.** Every placer runs one after another, so adding a placer with a high
  `avg_per_chunk` is the quickest way to slow generation down. A placer that could be a chunk
  placer should be.
- **Ordering between chunks is not reliable.** Anything a placer places can overlap with what a
  neighbor's placers place, in either order. If a feature has to avoid trees, it isn't safe here
  (see icing).

The `PlacerStage` argument to `place` is currently unused.

## Icing placers

An icing placer places the last layer of decoration, once a chunk is final. Every tree has been
placed by then, including ones that grew in from a neighbor, and nothing runs after it.

Use an icing placer only for things that strictly need to happen after trees, because they would be
wrong if a tree showed up afterwards.

The motivating example is bamboo. A bamboo stalk is tall. If a tree is placed after it, the leaves
skip the blocks the bamboo is in, and the stalk pokes through the canopy. If bamboo runs in the icing
pass instead, it can see the canopy, and a stalk is cut short to fit underneath it (and skipped if
there isn't room).

An icing placer can only read and write its own chunk. Like a chunk placer, it handles features
that cross borders by replaying them from the seed. Icing placers are a different type from
chunk placers on purpose, so that you can't add one to the wrong pass by accident. The two have the
same signature, but the assumptions are very different.

Things to know:

- **Surfaces are stale.** `chunk.surfaces()` was worked out during base generation, and trees don't
  update it. It tells you where the ground is, not what is on top of it. Check the blocks above
  the ground for anything that is in the way.
- **Don't use icing for things that could be done earlier.** Grass and flowers work fine as chunk
  placers. They don't need to be after trees, and a chunk placer is faster. Use icing only when
  placing it before a tree would be a bug.
- **They are currently run under the global chunk lock.** Keep them cheap.

## Choosing one

Ask these questions, in order:

1. **Does it need to be placed after trees?** Would a tree placed later break it, or does it need
   to look at the canopy? Use an icing placer.
2. **Is it bigger than a chunk, or does it need to read the real world, and not just the seed and
   the ground?** For example, a tree. Use a placer.
3. **Otherwise,** use a chunk placer.

Some examples:

| Feature                         | Kind          | Why                                              |
| ------------------------------- | ------------- | ------------------------------------------------ |
| Ores, caves, moss, snow cover   | Chunk placer  | Only needs its own chunk                         |
| Splotches and patches of block  | Chunk placer  | Trees check what they stand on, so it goes first |
| Boulders, pools, riverside      | Chunk placer  | Same, and they cross borders by replaying        |
| Grass, ferns, flowers           | Chunk placer  | Nothing needs to avoid them                      |
| Oak, birch, evergreen, cactus   | Placer        | Big, needs the world, can fail                   |
| Lava lakes, pillars, monuments  | Placer        | Big, and read the real world                     |
| Bamboo                          | Icing placer  | Must fit under the canopy                        |

## Adding one

- A chunk placer implements `ChunkPlacer`. Put it in `rgen-placer/src/chunk_placer`, and add it to
  a biome with `g.place_chunk(..)`.
- A placer implements `Placer`. Put it in `rgen-placer/src/placer`, and add it to a biome with
  `g.place("name", PlacerStage::Tree, ..)`.
- An icing placer implements `IcingPlacer`. Put it in `rgen-placer/src/icing_placer`, and add it to
  a biome with `g.place_icing(..)`.
