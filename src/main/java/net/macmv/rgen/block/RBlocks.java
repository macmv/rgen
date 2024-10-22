package net.macmv.rgen.block;

import net.macmv.rgen.RGen;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.block.BlockDoor;
import net.minecraft.block.material.Material;
import net.minecraft.client.renderer.block.statemap.StateMap;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;


import java.util.HashSet;
import java.util.Set;

public final class RBlocks {
  private static final Set<Block> blocks = new HashSet<>();

  // FIXME: Need block items.
  public static final Block THATCH_ROOF = register("thatch_roof", new ThatchRoofBlock(Material.ROCK));
  public static final Block DERP_DOG = register("derp_dog", new Block(Material.CLOTH));
  public static final Block LOOSE_ROCK = register("loose_rock", new LooseRockBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Block PLANT = register("plant", new PlantBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
  // public static final Block MOSSY_STUMP = register("mossy_stump", new Block(Material.WOOD));
  public static final Block MOSSY_CARPET = register("mossy_carpet", new MossCarpet().setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Block MOSSY_BLOCK = register("mossy_block", new MossBlock());
  public static final Block MOSSY_COBBLESTONE_RGEN = register("mossy_cobblestone_rgen", new PlantableStone());
  public static final Block MOSSY_STONE = register("mossy_stone", new PlantableStone());
  public static final Block POLYPORE = register("polypore", new PolyporeBlock());

  public static final Block LOG = register("log", new LogBlockOne());
  public static final Block LOG2 = register("log2", new LogBlockTwo());

  public static final Block PLANKS = register("planks", new PlanksBlock());
  //Stairs
  //public static final Block BAMBOO_STAIRS = register("bamboo_stairs", new BambooStairs());
  public static final Block DEAD_STAIRS = register("dead_stairs", new RgenStairs(LogType.DEAD));
  public static final Block MANGROVE_STAIRS = register("mangrove_stairs", new RgenStairs(LogType.MANGROVE));
  public static final Block CEDAR_STAIRS = register("cedar_stairs", new RgenStairs(LogType.CEDAR));
  public static final Block CHERRY_STAIRS = register("cherry_stairs", new RgenStairs(LogType.SAKURA));
  public static final Block FIR_STAIRS = register("fir_stairs", new RgenStairs(LogType.FIR));
  public static final Block PALM_STAIRS = register("palm_stairs", new RgenStairs(LogType.PALM));

  //Doors
  public static final Block DEAD_DOOR = register("dead_door", new DeadDoor());
  public static final Block MANGROVE_DOOR = register("mangrove_door", new MangroveDoor());
  public static final Block BAMBOO_DOOR = register("bamboo_door", new BambooDoor());
  public static final Block CEDAR_DOOR = register("cedar_door", new CedarDoor());
  public static final Block CHERRY_DOOR = register("cherry_door", new CherryDoor());
  public static final Block FIR_DOOR = register("fir_door", new FirDoor());
  public static final Block PALM_DOOR = register("palm_door", new PalmDoor());

  //TrapDoors
  public static final Block PALM_TRAPDOOR = register("palm_trapdoor", new RgenTrapDoors());

  public static final Block MOSSY_STUMP = register("mossy_stump", new MossyLogBlock());
  public static final Block BAMBOO = register("bamboo", new Bamboo());

  public static final Block LEAVES = register("leaves", new LeavesBlockOne());
  public static final Block LEAVES2 = register("leaves2", new LeavesBlockTwo());
  public static final Block LEAVES3 = register("leaves3", new LeavesBlockThree());
  public static final Block GLOW_VINE = register("glow_vine", new GlowVineBlock());

  public static final Block SAPLING = register("sapling", new SaplingOne());
  public static final Block SAPLING2 = register("sapling2", new SaplingTwo());

  public static final Block FLOWER = register("flower", new FlowerBlock());
  public static final Block CACTUS_ARM = register("cactus_arm", new CactusArm());
  public static final Block RGEN_CACTUS = register("rgen_cactus", new RgenCactus());
  public static final Block CROSS_CACTUS = register("cross_cactus", new CrossCactus());
  public static final Block JUVENILE_GREEN_CACTUS = register("juvenile_green_cactus", new JuvenileGreenCactus());
  public static final Block BLUE_CACTUS = register("blue_cactus", new BlueCactus());
  public static final Block CACTUS = register("cactus", new Cactus());
  public static final Block JUVENILE_CACTUS = register("juvenile_cactus", new JuvenileCactus());
  public static final Block LAVENDER_PLANT = register("lavender_plant", new LavenderPlant());
  public static final Block DOUBLE_TALL_LAVENDER_PLANT = register("double_tall_lavender_plant", new DoubleTallLavenderPlant());
  public static final Block BASALT = register("basalt", new Basalt());
  public static final Block PINK_FLOWERBED = register("pink_flowerbed", new PinkFlowerbed());


  private static Block register(String name, Block block) {
    if (block.getCreativeTabToDisplayOn() == null) {
      block.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    }

    block.setRegistryName(RGen.MODID, name);
    block.setUnlocalizedName(name);
    blocks.add(block);
    return block;
  }

  public static void registerBlocks(IForgeRegistry<Block> reg) {
    for (Block b : blocks) {
      reg.register(b);
    }
  }

  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    // Ignores powered metadata alongside the rest of the metadata on the client side.
    ModelLoader.setCustomStateMapper(DEAD_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(MANGROVE_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(BAMBOO_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(CEDAR_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(CHERRY_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(FIR_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(PALM_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
  }
}
