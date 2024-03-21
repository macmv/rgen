# The TODO File



# Block By Block
These are listed bugs or features that needed to be added to each block and or item.


## Bamboo
- Placement | When bamboo is placed by a player, it needs to randomly switch between 4 diffrent positions if it's not being placed above another bamboo
- Falling down | when broken, all the bamboo above needs to break
- Placer | The place program needs to be written for bamboo to have it be between 15-20 blocks tall and have leaves at the top - Assigned to Jaimie
- Sound | Sounds need to be set for the placement, walking, and destruction of bamboo
- Crafting | Bamboo needs to be craftable into things; what things still need to be decided
- Burnable | Bamboo needs to be able to be placed into a furnace to be used as a fuel


## Mossy Stumps
- Dropping | mossy stumps need to drop their parent log and a mossy block
- Crafting | crafted from a log and a mossy block
- Sound | Breaking should sound like a log but walking should sound like grass or custom moss sound


## Mossy Block
- Sound | Walking on it should sound like grass or a custom moss sound


## Mossy Carpet
- Sound | Walking on it should sound like grass or a custom moss sound
- Placement | Can't be placed on self or nonsolid blocks 
- Crafting | Should be craftable from 2 moss blocks to make 6 carpet
- water | When hit with water, it should break similarly to other plant blocks in Minecraft


## Basalt
- Texture | A basalt texture is needed
- Block | a basalt block is needed
- Like Stone | The block needs to feel and act like a stone-like block
- **Variants | Should variants of basalt like in vanilla be added


## Polypores
- Sideways Boundry Boxes | The boundary box of a polypore needs to be changed from a whole block to a side of a block
- **Dropping | Should polypores be dropable, and should it be as simple as breaking with a fist or need a silk touch
- **Placement | If players can place polypores, they need to be placeable by the way their face-like buttons
- water | Like most plants in Minecraft, polypores should break underwater

## Cactus
- 



# Mod systems
For our purpose, a "mod system" refers to a system of blocks, items, and/or mobs that work symbiotically. An example of this is Moss, a block that can be grown to create more of itself and then fused with other blocks to make moss variants of that block. Another example could be a fir tree, which adds fir blocks, which can then be crafted into fir planks, doors, fences, and stairs. 


## Basic tree system example


## Lime System
adds limestone block
limestone is needed to make concrete powder


## Cactus system
changing between cactus types is done by crafting other cactus fruits, which gets you the next cactus in this list and then loops
**Green** is used for green dye and gets you the large variant
**Orange** is used for orange dye and gets you a Small cactus plant
**Pink** is used for pink dye and gets you the Tiny cactus plant
**Aged** can be crafted into **green** dye or be planted for a vanilla cactus 
### Green cactus fruit
- Cactus, as added by Rgen, acts like a tree. They have three life stages the first is the cactus made of the cactus block and has cactus arms 
- When a cactus is broken, the block drops 2-3 **green** cactus fruit, and the arms drop 1-2 **green** cactus fruit
- A **green** cactus fruit that is placed becomes a Juvenile cactus, which, after a set length of time or the use of bone meal, will grow into a cactus tree-style plant
- A **green** cactus fruit that is put into a furnace becomes cactus green dye 
- A **green** cactus fruit that is crafted gets you an **orange** cactus fruit
### Orange Cactus Fruit
- An **orange** cactus, also known as a small_cactus, is a cross build plant that can be found in lush desert biomes


## The Moss System
///NEEDS work





# Placers

## Stumps and logs
stumps and logs are standard of most forests in the real world for our purposes, a stump and log are single logs standing upright (stump) followed by a block of air and then two-five blocks of xz axis logs
||  ===  <- an asci example

### how a log generates
stumps and logs can come in 3 standards 
1) a stump and log - 35%
2) just a log - 30%
3) just a stump - 35%

The **chance of mossy**-ness is chosen by percentage ranging from none-0%, low-25%, mid-45%, high-80%\
When a mossy block is placed, there is a 55% chance of block from the **plant blockset** being placed on top and a 10% chance of _mossy_carpet_

- For a stump, just check blocks -1,0 1,0 0,1 0,-1 to make sure they are not ground blocks
  - each side of the stump has a 10% chance of a _polypore_ being on it
- For a log, first, choose a direction and length, then check all edges for ground and check the start and end for ground (this will allow in rare circumstances for logs to hang between two points of land)
  - the start and end of the logs have a 25% chance of a _polypore_ being on it
  - the sides of each log have a 10% chance of a _polypore_ being on it
- For a log and stump, do both checks, then place the stump and log


## Generated snow layers
Snow generates based on **snow grade** rated 0,1,2,3 or low,mid,high,heavy 
each biome and block will have unique levels of snow-grade
Example:
**Snowy Fir Forest**
- Forest ground: high
    - blocks exposed to sky that are [grass, dirt, fir logs, oak logs, ponzel, gravel] will have a high level of snow on them
- Forest leaf cover: mid
    - Blocks exposed to the sky that are leaves [fir leaves] have a mid-level of snow 




# Other
# Modpack generation problems
A "modpack generation problem" refers to a structure, plant, mob, ore, etc. that is added by another mod to vanilla Minecraft and must then be generated by RGen to make the modpack work. 

## Pams harvest craft
pams harvest craft is one of the largest food mods in minecraft and adds gardens (basicaly breakbale loot chests of ingredients that are based on specific biomes) and fruit trees (oak trees with large oversized fruit hanging from them). The owner of the mod is really defensive of their work.

### The tree problem
Pams harvestcraft trees don't really fit in the RGen style of the world

### The garden problem
We need to figure out how to make sure that gardens both generate and generate in custom biomes

## Ores
A fair few mods add select ores below is a growing list of ores that will need to be considerd.\
[how the ore dictonary works according to forge](https://docs.minecraftforge.net/en/1.12.x/utilities/oredictionary/)\
[list of forge ore tags](https://github.com/Dente222/Minecraft-Forge-Tag-List/blob/master/ores.txt)
- uranium
- copper
- lead
- tin

## Structure generation
A list of exmaple structures that can generate 

## Notes and links
https://github.com/Dente222/Minecraft-Forge-Tag-List
