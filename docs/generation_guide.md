# Terrain Generation Guide

This assumes you have the terrain generator setup. See `generation_setup.md` for details.

## Adding a Biome

This example will cover adding a desert, with cactuses.

This will cover adding a new biome. Biomes are declared in the `biome` package of the `rgen-biome`
crate. Each file in here is loosely categorized based on the temperature of the biomes. So, lets
add a new biome into the `hot_regions.rs`, as that's where a desert generally fits.

I'm going to start by copying an existing biome, the `flat_desert`:
```rs
pub fn flat_desert(gen: &mut BiomeBuilder) {
  gen.id = biome![desert];
  gen.color = "#E0705F";
  gen.set_top_block(block![sand]);
  gen.add_layer(block![sandstone], 5, 8);

  gen.place("Large Cactus", PlacerStage::Tree, placer::Cactus::new());
}
```

Firstly, this declares a new function, `flat_desert`. All biomes are defined as a single function,
that takes a `&mut BiomeBuilder`, and modifies the biome builder to build that specific biome.

After the function definition, there are a few notable things. In order:

### `id`

The `id` is set to `biome![desert]`. This is the vanilla biome ID, which controls grass color, what
mobs spawn, if there is rainfall, and a few more properties.

### `color`

This color is for debugging purposes, and can be any color you'd like. If you don't want to bother
picking a color, set it to all black (#000000), so that its clear that a color hasn't been picked
for this biome.

### `set_top_block`

Then, `set_top_block()` is called. This is the surface block for the biome. Most biomes use grass,
but in this case we're building a desert, so we want sand on the surface.

This is also the first use of the `block![]` macro. Macros are called with a word, then a `!` token.
In this case, the macro takes a single argument, which is a block name. The block name is any block
we've hooked up to RGen.

Block names in the `block![]` macro can be namespaced, so `block![sand]` is the same thing as
`block![minecraft:sand]`. Additionally, modded blocks can be referred to, for example by writing
`block![rgen:foobar]`, you'd be picking the `foobar` block from the RGen mod.

### `add_layer`

Layers are a key concept in biomes. By default, biomes are all solid stone. To put dirt and grass
on the top of the biome, layers can be added. Layers are defined top to bottom, so the first layer
is the surface block, the next layer is just beneath it, etc.

In this case, we want to place a few layers of sandstone below the top layer of sand. So, we call
`add_layer(block![sandstone], 5, 8)`, which places sandstone that is 5 to 8 blocks deep below the
top layer of sand.

Also, if you're curious, take a look at how `set_top_block` is implemented. It's really just the
same as any other layer, it's just one block deep. `set_top_block` is just more convenient than
calling `add_layer(block![sand], 1, 1)`, and it expresses intent more clearly.

### `place`

Finally, we call `gen.place` to place some cactuses. Placers are the final, key concept in biome
definitions. A placer, well, places things. For example, this placer places a cactus. A placer
might place a bush, or a tree, or a clump of grass. These placers can then be added into biome
definitions using the `place` function. Any number of placers can be added to a biome definition,
and they will all place in the order added.

The name of the placer ("Large Cactus" in this case), and the stage (PlacerStage::Tree) are both
unused. The name should be somewhat descriptive, but we have no naming convention right now (if you
take a look through the code, you'll see we switch between capitalized names and lowercase names
all over the place). The placer stage was intended to fix some issues with trees overlapping, but
it turned out to be difficult to implement, so it doesn't do anything.

## Placers

Placers are the building blocks of biomes. Each placer gets its own file (usually), and they can be
found in the `rgen-placer` crate, under the `placer` package.

Placers are meant to be repeatable. For example, we have a placer that places boulders on the
ground. This is handy in a few biomes, but it also can be configured between biomes. Some biomes
might want mossy boulders, whereas others might want plain stone boulders. This is what placers are
designed for: small, repeatable structures that can be used in a variety of biomes.

Now, there are of course exceptions to this rule. Namely, each different type of tree requires its
own very specific placer, and those generally aren't very customizable. Even birch and oak trees,
which are very similar, use their own placers to get the most fine tuned control.

Finally, for simple placers, we have a running joke to name them all starting with 's'. Right now,
we've got splatter, scatter, splotch, and spread. I could not tell you the difference between them.
I do know that each one is very specifically different from the others, but I could not tell you
how. Feel free to add more to this list of you'd like.
