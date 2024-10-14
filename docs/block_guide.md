# Block Guide
Rgen is made of two parts a forge based minecraft mod that adds in new modded content like blocks and mobs, and the rust based rgen world generation program. Blocks as part of the forge mod requires you to create a texture, a model, a java block class, and setup to block inside the mods block list. In this example you will be making an example block called `example`.

## Creating block art and model 
Minecraft blocks have three parts to them: a texture, a model, and a block state.
| Texture                               | Models                                | Blockstates                           |
|---------------------------------------|---------------------------------------|---------------------------------------|
| <div style="text-align: center;"><img src="../art/example_block.png" alt="example block image" width="100"><br> The **Texture** represents the visual appearance of the block. It is the image that gets applied to the surface of the block to give it its look, such as the `dirt.png` being on the dirt block.</div> | <div style="text-align: center;"><img src="../art/example_model.png" alt="Model" width="100"><br> The **Model** defines the 3D shape of the block. It specifies how the texture is wrapped around the block. Think of this as the structure and the texture as the paint.</div> | <div style="text-align: center;"><img src="../art/blockstate.png" alt="Blockstate" width="100"><br> **Blockstates** determine the variations of a block. For example, they manage the orientation, controlling how the block behaves in different states.</div> |



```
src/main/resources/assets/rgen/textures/      # The location of the textures
src/main/resources/assets/rgen/models/        # The location of the models
src/main/resources/assets/rgen/blockstates/   # The location of the blockstates
```

### The texture
Here is our example block texture,s which we are now going to store inside a folder called `derp_dog_folder` so our full path is `src/main/resources/assets/rgen/textures/derp_dog_folder/derp_dog_front.png`

![alt text](../art/example_block.png "example block image")

A texture should be 16x16.

### The model
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

## Creating the block's code

## Setting up the block

## Continuing
[adding an item for a block]()

