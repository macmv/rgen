# Block Guide
Rgen is made of two parts a forge based minecraft mod that adds in new modded content like blocks and mobs, and the rust based rgen world generation program. Blocks as part of the forge mod requires you to create a texture, a model, a java block class, and setup to block inside the mods block list. In this example you will be making an example block called `example`.

## Creating block art and model 
Minecraft blocks have three parts to them a texture, a model, and a blockstate.
```
src/main/resources/assets/rgen/textures/      # The location of the textures
src/main/resources/assets/rgen/models/        # The location of the models
src/main/resources/assets/rgen/blockstates/   # The location of the blockstates
```

Here is our example block texture which we are now going to store in texture file inside a folder called `example_folder` so our full path is `src/main/resources/assets/rgen/textures/example_folder/example_block.png`
![alt text](art/example_block.png "example block image")

## Creating the block's code

## Setting up the block

## Continuing
[adding an item for a block]()

