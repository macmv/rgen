package net.macmv.rgen.block;

import net.macmv.rgen.RGen;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.block.BlockDoor;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.client.renderer.block.statemap.StateMap;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

import java.util.HashSet;
import java.util.Set;
import java.util.function.Function;

public final class RBlocks {
  private static final Set<Block> blocks = new HashSet<>();

  public static final Block DERP_DOG = register("derp_dog", s -> s.createMat(Block::new));
  public static final Block LOOSE_ROCK = register("loose_rock", LooseRockBlock::new, new BlockSettings().creativeTab(RCreativeTabs.DECORATIONS));
  public static final Block PLANT = register("plant", PlantBlock::new, new BlockSettings().material(Material.GRASS).creativeTab(RCreativeTabs.DECORATIONS));
  // public static final Block MOSSY_STUMP = register("mossy_stump", Block::new, new BlockSettings().material(Material.WOOD));
  public static final Block MOSSY_CARPET = register("mossy_carpet", MossCarpet::new, new BlockSettings().material(Material.CARPET).creativeTab(RCreativeTabs.DECORATIONS));
  public static final Block MOSSY_BLOCK = register("mossy_block", MossBlock::new, new BlockSettings().material(Material.GROUND));
  public static final Block MOSSY_COBBLESTONE_RGEN = register("mossy_cobblestone_rgen", PlantableStone::new, new BlockSettings().material(Material.ROCK).creativeTab(CreativeTabs.DECORATIONS));
  public static final Block MOSSY_STONE = register("mossy_stone", PlantableStone::new, new BlockSettings().material(Material.ROCK).creativeTab(CreativeTabs.DECORATIONS));
  public static final Block POLYPORE = register("polypore", PolyporeBlock::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block RED_POLYPORE = register("red_polypore", PolyporeBlock::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));

  public static final Block LOG = register("log", LogBlockOne::new, new BlockSettings().creativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Block LOG2 = register("log2", LogBlockTwo::new, new BlockSettings().creativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Block PLANKS = register("planks", PlanksBlock::new, new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  // Stairs
  // public static final Block BAMBOO_STAIRS = register("bamboo_stairs", new BambooStairs());
  public static final Block DEAD_STAIRS = register("dead_stairs", s -> new RStairs(s, LogType.DEAD), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_STAIRS = register("mangrove_stairs", s -> new RStairs(s, LogType.MANGROVE), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_STAIRS = register("cedar_stairs", s -> new RStairs(s, LogType.CEDAR), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_STAIRS = register("cherry_stairs", s -> new RStairs(s, LogType.SAKURA), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block FIR_STAIRS = register("fir_stairs", s -> new RStairs(s, LogType.FIR), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block PALM_STAIRS = register("palm_stairs", s -> new RStairs(s, LogType.PALM), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));

  // Doors
  public static final Block DEAD_DOOR = register("dead_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_DOOR = register("mangrove_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block BAMBOO_DOOR = register("bamboo_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_DOOR = register("cedar_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_DOOR = register("cherry_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block FIR_DOOR = register("fir_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block PALM_DOOR = register("palm_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));

  // TrapDoors
  public static final Block DEAD_TRAPDOOR = register("dead_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_TRAPDOOR = register("mangrove_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_TRAPDOOR = register("cedar_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_TRAPDOOR = register("cherry_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block FIR_TRAPDOOR = register("fir_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block PALM_TRAPDOOR = register("palm_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));

  // CoveredLogs
  public static final Block COVERED_OAK_LOG = register("covered_oak_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_SPRUCE_LOG = register("covered_spruce_log", s -> new RCoveredLog(s, false), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_BIRCH_LOG = register("covered_birch_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_JUNGLE_LOG = register("covered_jungle_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));

  public static final Block MOSSY_STUMP = register("mossy_stump", MossyLogBlock::new, new BlockSettings().creativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Block BAMBOO = register("bamboo", Bamboo::new, new BlockSettings().material(Material.PLANTS));

  public static final Block LEAVES = register("leaves", LeavesBlockOne::new);
  public static final Block LEAVES2 = register("leaves2", LeavesBlockTwo::new);
  public static final Block LEAVES3 = register("leaves3", LeavesBlockThree::new);
  public static final Block GLOW_VINE = register("glow_vine", GlowVineBlock::new, new BlockSettings().lightValue(3));

  public static final Block SAPLING = register("sapling", SaplingOne::new);
  public static final Block SAPLING2 = register("sapling2", SaplingTwo::new);

  public static final Block FLOWER = register("flower", FlowerBlock::new, new BlockSettings().creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block CACTUS_ARM = register("cactus_arm", CactusArm::new);
  public static final Block RGEN_CACTUS = register("rgen_cactus", RgenCactus::new, new BlockSettings().hardness(0.4f));
  public static final Block CROSS_CACTUS = register("cross_cactus", CrossCactus::new, new BlockSettings().creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block JUVENILE_GREEN_CACTUS = register("juvenile_green_cactus", JuvenileGreenCactus::new, new BlockSettings().material(Material.PLANTS));
  public static final Block BLUE_CACTUS = register("blue_cactus", BlueCactus::new, new BlockSettings().hardness(0.4f));
  public static final Block CACTUS = register("cactus", Cactus::new, new BlockSettings().hardness(0.4f));
  public static final Block JUVENILE_CACTUS = register("juvenile_cactus", JuvenileCactus::new, new BlockSettings().material(Material.PLANTS).hardness(0.4f));
  public static final Block LAVENDER_PLANT = register("lavender_plant", LavenderPlant::new, new BlockSettings().material(Material.PLANTS));
  public static final Block DOUBLE_TALL_LAVENDER_PLANT = register("double_tall_lavender_plant", DoubleTallLavenderPlant::new);
  public static final Block BASALT = register("basalt", Basalt::new, new BlockSettings().hardness(1.25f).soundType(SoundType.STONE));
  public static final Block PINK_FLOWERBED = register("pink_flowerbed", PinkFlowerbed::new, new BlockSettings().material(Material.PLANTS));
  public static final Block HANGING_VINES = register("hanging_vines", HangingVines::new, new BlockSettings().material(Material.PLANTS));

  private static Block register(String name, Function<BlockSettings, Block> blockSupplier) {
    Block block = blockSupplier.apply(new BlockSettings());
    block.setRegistryName(RGen.MOD_ID, name);
    block.setUnlocalizedName(name);
    blocks.add(block);
    return block;
  }

  private static Block register(String name, Function<BlockSettings, Block> blockSupplier, BlockSettings settings) {
    Block block = blockSupplier.apply(settings);
    block.setRegistryName(RGen.MOD_ID, name);
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
