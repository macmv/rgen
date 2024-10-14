# Block Guide
Rgen is made of two parts a forge based minecraft mod that adds in new modded content like blocks and mobs, and the rust based rgen world generation program. Blocks as part of the forge mod requires you to create a texture, a model, a java block class, and setup to block inside the mods block list. The example block here will be `derp_dog`.

## Creating block art and model 
Minecraft blocks have three parts to them: a texture, a model, and a block state.
| Texture                               | Models                                | Blockstates                           |
|---------------------------------------|---------------------------------------|---------------------------------------|
| <div style="text-align: center;"><img src="../art/example_block.png" alt="example block image" width="100"><br> The **Texture** represents the visual appearance of the block. It is the image that gets applied to the surface of the block to give it its look, such as the `dirt.png` being on the dirt block.</div> | <div style="text-align: center;"><img src="../art/example_model.png" alt="Model" width="100"><br> The **Model** defines the 3D shape of the block. It specifies how the texture is wrapped around the block. Think of this as the structure and the texture as the paint.</div> | <div style="text-align: center;"><img src="../art/example_blockstate.png" alt="Blockstate" width="100"><br> **Blockstates** determine the variations of a block. For example, they manage the orientation, controlling how the block behaves in different states.</div> |



```
src/main/resources/assets/rgen/textures/      # The location of the textures
src/main/resources/assets/rgen/models/        # The location of the models
src/main/resources/assets/rgen/blockstates/   # The location of the blockstates
```

### The texture
Here is our example block texture, which we are now going to store inside a folder called `derp_dog_folder`. Here are all the textures being used on Derp dog;
```
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_bottom.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_front.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_side.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_tail.png
src/main/resources/assets/rgen/textures/blocks/derp_dog_folder/derp_dog_top.png
```

![alt text](../art/example_block.png "example block image")

### The model
The JSON code below is how a model appears in JSON; however, we recommend you use the [blockbench tool](###blockbench), as editing a model file in a text editor is a sign of madness. 
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
**Note:** This section goes into hyper detail on models feel free to move onto Blockstates

The parent refers to the Minecraft model from which this block is inheriting. As a basic block, Derp dog simply inherits from `block/block`; however, if you're making a flower like a tulip, you would inherit from `block/cross` to get the cross shape of Minecraft plants.

The textures being mapped onto the block are then listed next alphanumerically. All textures start with `rgen:blocks/` and then the path. **They do not include `.png`** 
The particles are what appears when you walk on the block and are made of the colors from the image in the texture.

Next are the elements, each block shape. You can see the textures being wrapped onto the faces of each element. In this example, only one element is listed.

### The Blockstate
Block states are used to set different states on a block. If the block has a different shape or texture, a different model will be needed. If it's being rotated, a command is entered to rotate it.
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

## Creating the block's code
WIP

## Setting up the block
WIP

## Continuing
[adding an item for a block]()
more &
more...
### Blockbench
[Download Blockbench](https://www.blockbench.net/downloads) 

How to use Blockbench *I could find no guide that i found worthwhile to recomend**
