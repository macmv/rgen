# Block Guide
Rgen is made of two parts: a minecraft mod written with forge, and a terrain generator written in
rust. The forge mod creates all the new blocks, defines how they behave, and what they look like in
game. This guide covers setting up a new block in the forge mod.

For this example, we're going to create a block called `derp_dog`. This block is a cube, with no
custom behaviors attached (for example, it doesn't get powered by redstone).

## Creating block art and model 
Minecraft blocks have three parts to them: a texture, a model, and a block state.
| Texture                               | Models                                | Blockstates                           |
|---------------------------------------|---------------------------------------|---------------------------------------|
| <div style="text-align: center;"><img src="../art/example_block.png" alt="example block image" width="100"><br> The **Texture** represents the visual appearance of the block. It is the image that gets applied to the surface of the block to give it its look, such as the `dirt.png` being on the dirt block.</div> | <div style="text-align: center;"><img src="../art/example_model.png" alt="Model" width="100"><br> The **Model** defines the 3D shape of the block. It specifies how the texture is wrapped around the block. Think of this as the structure and the texture as the paint.</div> | <div style="text-align: center;"><img src="../art/example_blockstate.png" alt="Blockstate" width="100"><br> **Blockstates** determine the variations of a block. For example, they manage the orientation, controlling how the block behaves in different states.</div> |



```sh
src/main/resources/assets/rgen/textures/blocks/ # The location of the block textures (note the 's' at the end)
src/main/resources/assets/rgen/models/block/    # The location of the block models
src/main/resources/assets/rgen/blockstates/     # The location of the blockstates
```

### The texture
Here is our example block texture, which we are now going to store inside a folder called
`derp_dog_folder`. Here are all the textures being used on Derp dog:
```sh
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_bottom.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_front.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_side.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_tail.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_top.png
```

![alt text](../art/example_block.png "example block image")

### The model
The JSON code below is how a model appears in JSON. However, we recommend you use the
[blockbench tool](###blockbench), as editing a model file in a text editor is a sign of madness. 
```json
{
  "parent": "block/block",
  "textures": {
    "0": "rgen:blocks/derp_dog_folder/derp_dog_bottom",
    "1": "rgen:blocks/derp_dog_folder/derp_dog_front",
    "2": "rgen:blocks/derp_dog_folder/derp_dog_side",
    "3": "rgen:blocks/derp_dog_folder/derp_dog_tail",
    "4": "rgen:blocks/derp_dog_folder/derp_dog_top",
    "particle": "rgen:blocks/derp_dog_folder/derp_dog_bottom"
  },
  "elements": [
    {
      "from": [0, 0, 0],
      "to": [16, 16, 16],
      "faces": {
        "north": {"uv": [0, 0, 16, 16], "texture": "#3"},
        "east": {"uv": [0, 0, 16, 16], "texture": "#2"},
        "south": {"uv": [0, 0, 16, 16], "texture": "#1"},
        "west": {"uv": [0, 0, 16, 16], "texture": "#2"},
        "up": {"uv": [0, 0, 16, 16], "texture": "#4"},
        "down": {"uv": [0, 0, 16, 16], "texture": "#0"}
      }
    }
  ]
}
```

**Note:** This section goes into hyper detail on models, feel free to move onto Blockstates.

Models define the following:
- A list of elements. Each element is a cuboid, that can be rotated in increments of 22.5 degrees.
- Each element contains a list of faces. Each face contains a texture.
- A list of named textures to attach to faces.
- Transforms for how the model looks in the hand.

Models may inherit the above properties from the `parent` key. In our example, the `derp_dog` model
inherits from `block/block`. This a traditional minecraft block (like stone or dirt).

Another common parent is `block/cross`, which is used for tall grass and flowers.

Because this model has a specific front, side, and back texture, we define a single element, which
ranges from 0,0,0 to 16,16,16. This element is a cube, which covers the entire area of the block.

Finally, we attach our textures to this element in the `faces` section (the `#2` refers to the
texture `2`, for example).

Also note that the texture paths do not include `.png` at the end. `png` textures are the only
supported texture in minecraft.

### The Blockstate
Block states are used to set different states on a block. If the block has a different shape or
texture, a different model will be needed. If it's being rotated, a command is entered to rotate it.

_This example shows the polypore blockstate_
```json
{
  "variants": {
    "facing=north,type=1": { "model": "rgen:polypore_type_one" },
    "facing=south,type=1": { "model": "rgen:polypore_type_one", "y": 180 },
    "facing=east,type=1": { "model": "rgen:polypore_type_one", "y": 90 },
    "facing=west,type=1": { "model": "rgen:polypore_type_one", "y": 270 },
    "facing=north,type=2": { "model": "rgen:polypore_type_two" },
    "facing=south,type=2": { "model": "rgen:polypore_type_two", "y": 180 },
    "facing=east,type=2": { "model": "rgen:polypore_type_two", "y": 90 },
    "facing=west,type=2": { "model": "rgen:polypore_type_two", "y": 270 },
    "facing=north,type=3": { "model": "rgen:polypore_type_three" },
    "facing=south,type=3": { "model": "rgen:polypore_type_three", "y": 180 },
    "facing=east,type=3": { "model": "rgen:polypore_type_three", "y": 90 },
    "facing=west,type=3": { "model": "rgen:polypore_type_three", "y": 270 }
  }
}

```
Polypore has two different variations:
- `facing`, which rotates the block on its y-axis `"y": 90` is polypore rotated by 90 degrees.
- `type`, which has a different model for if the polypore is made of 1, 2, or 3 polypores. 

The variations must be in alphabetical order ie. `apple=true,banana=false`

## Creating the block's code (creating a Block Class)

To create a custom block in Minecraft, you need to define the block class in Java. This section
walks through the basic steps for creating a block in Minecraft using the Forge API. In this
example, we’ll create a custom block called `DerpDog`.
### 1. Create a java class
in the path `src/main/java/net/macmv/rgen/block` and create a class file. 
In JetBrains this can be done by right-clicking on the block folder and selecting `new > Java Class`
The naming convention is capitals at the beginning and after spaces. (Don't include the spaces) `DerpDog`

### 2. Setup the class
Start by creating a new class for your block. To set the right package, include
`net.macmv.rgen.block`. This basic structure for a custom block extends Minecraft's `Block` class
and sets its material type.

Here’s an example of a simple block class:

```java
package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.material.Material;

public class DerpDog extends Block {

    // Constructor to set the block's material type
    public DerpDog() {
        super(Material.GROUND);  // Defines the material of the block as 'GROUND'
    }
}
```

### 3. Setup the block registration
The final step is registering the block with `RBlocks`. The path is
`src/main/java/net/macmv/rgen/block/RBlocks.java` in the `RBlocks` class add the following:
```java
public static final Block EXAMPLE_BLOCK = register("example_block", new ExampleBlock());
```
`public`: This makes the variable `EXAMPLE_BLOCK` accessible from anywhere in the code.

`static`: This means the block belongs to the class, not an instance of the class. So, you can
reference it directly without needing to create an instance of the class.

`final`: This makes the block a constant, meaning once it’s assigned a value, it cannot be changed.

`Block`: This is the type of object being created—specifically, a Minecraft block.

`EXAMPLE_BLOCK`: The variable name for the block, which is used when referencing it in code.

`register(string, Block)`: takes the name of the block in the format of `example_block` followed by
a new block class of the block you're adding.

#### Adding to a creative tab
The block can also be added to one of the Rgen creative tabs like this:
```java
public static final Block EXAMPLE_BLOCK = register("example_block", new ExampleBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
```

## Continuing
[adding an item for a block]()
more &
more...
### Blockbench
[Download Blockbench](https://www.blockbench.net/downloads) 

How to use Blockbench *I could find no guide that i found worthwhile to recomend**
