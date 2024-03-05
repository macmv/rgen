Building a tree
 - log
 - leaves
 - door
 - trapdoor
 - planks

 - slabs*
 - stairs*
 - fence*
 - gate*

- src/main/resources/assets/rgen/textures/blocks/[]
This is the textures

- src/main/resources/assets/rgen/models/block/[]
This is the model

- src/main/resources/assets/rgen/blockstates/[]
This is the block data and its model mapping 

- src/main/java/net/macmv/rgen/block/RBlocks.java <- edit this to add the block and extend From a block class
Where the block extends from


Editing and creating pngs
Edit and crate .jsons
Edit a .java


# The Moss Update
* = A)texture B)block/model C)blockstates
.# = is a block but may miss features
% = not done
[] = fixes needed 

moss_covered_cobblestone % % % moss covered cobblestone   %
mossy_cobblestone_rgen * * * mossy cobblestone            #
mossy_stone * * * mossy stone                             #
mossy_block * * * moss block                              # done [sound fix, needs item]
mossy_carpet * * * mossy carpet                           # done [sound fix, needs item]
mossy_stump * * * mossy stump                             # needs sides and varients 
                                                            [sound fix, needs item]
mossy_bush * * * mossy bush                               # done [sound fix, needs item]

incubator (entity)
-craffted by stone block sides with a middle stack of water bucket then mossblock then coldron 

- by placing a water bucket in the top block then place the plant to be incubated a duplicate is then generated

- the block being incubated can be: moss, vines, brown mushrooms, red mushrooms

- by placing a water bucket and an inubatable item which over time will get you a duplicate of that block
