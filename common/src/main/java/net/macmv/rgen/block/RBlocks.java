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
import java.util.function.Supplier;

public final class RBlocks {
  private static final Set<Block> blocks = new HashSet<>();

  public static final Block DERP_DOG = register("derp_dog", () -> new Block(Material.CLOTH));
  public static final Block LOOSE_ROCK = register("loose_rock", () -> new LooseRockBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Block PLANT = register("plant", () -> new PlantBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
  // public static final Block MOSSY_STUMP = register("mossy_stump", new Block(Material.WOOD));
  public static final Block MOSSY_CARPET = register("mossy_carpet", () -> new MossCarpet().setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Block MOSSY_BLOCK = register("mossy_block", MossBlock::new);
  public static final Block MOSSY_COBBLESTONE_RGEN = register("mossy_cobblestone_rgen", PlantableStone::new);
  public static final Block MOSSY_STONE = register("mossy_stone", PlantableStone::new);
  public static final Block POLYPORE = register("polypore", PolyporeBlock::new);
  public static final Block RED_POLYPORE = register("red_polypore", PolyporeBlock::new);

  public static final Block LOG = register("log", LogBlockOne::new);
  public static final Block LOG2 = register("log2", LogBlockTwo::new);

  public static final Block PLANKS = register("planks", PlanksBlock::new);
  // Stairs
  // public static final Block BAMBOO_STAIRS = register("bamboo_stairs", new BambooStairs());
  public static final Block DEAD_STAIRS = register("dead_stairs", () -> new RStairs(LogType.DEAD));
  public static final Block MANGROVE_STAIRS = register("mangrove_stairs", () -> new RStairs(LogType.MANGROVE));
  public static final Block CEDAR_STAIRS = register("cedar_stairs", () -> new RStairs(LogType.CEDAR));
  public static final Block CHERRY_STAIRS = register("cherry_stairs", () -> new RStairs(LogType.SAKURA));
  public static final Block FIR_STAIRS = register("fir_stairs", () -> new RStairs(LogType.FIR));
  public static final Block PALM_STAIRS = register("palm_stairs", () -> new RStairs(LogType.PALM));

  // Doors
  public static final Block DEAD_DOOR = register("dead_door", RDoor::new);
  public static final Block MANGROVE_DOOR = register("mangrove_door", RDoor::new);
  public static final Block BAMBOO_DOOR = register("bamboo_door", RDoor::new);
  public static final Block CEDAR_DOOR = register("cedar_door", RDoor::new);
  public static final Block CHERRY_DOOR = register("cherry_door", RDoor::new);
  public static final Block FIR_DOOR = register("fir_door", RDoor::new);
  public static final Block PALM_DOOR = register("palm_door", RDoor::new);

  // TrapDoors
  public static final Block DEAD_TRAPDOOR = register("dead_trapdoor", RTrapDoor::new);
  public static final Block MANGROVE_TRAPDOOR = register("mangrove_trapdoor", RTrapDoor::new);
  public static final Block CEDAR_TRAPDOOR = register("cedar_trapdoor", RTrapDoor::new);
  public static final Block CHERRY_TRAPDOOR = register("cherry_trapdoor", RTrapDoor::new);
  public static final Block FIR_TRAPDOOR = register("fir_trapdoor", RTrapDoor::new);
  public static final Block PALM_TRAPDOOR = register("palm_trapdoor", RTrapDoor::new);

  // CoveredLogs
  public static final Block COVERED_OAK_LOG = register("covered_oak_log", () -> new RCoveredLog(true));
  public static final Block COVERED_SPRUCE_LOG = register("covered_spruce_log", () -> new RCoveredLog(false));
  public static final Block COVERED_BIRCH_LOG = register("covered_birch_log", () -> new RCoveredLog(true));
  public static final Block COVERED_JUNGLE_LOG = register("covered_jungle_log", () -> new RCoveredLog(true));

  public static final Block MOSSY_STUMP = register("mossy_stump", MossyLogBlock::new);
  public static final Block BAMBOO = register("bamboo", Bamboo::new);

  public static final Block LEAVES = register("leaves", LeavesBlockOne::new);
  public static final Block LEAVES2 = register("leaves2", LeavesBlockTwo::new);
  public static final Block LEAVES3 = register("leaves3", LeavesBlockThree::new);
  public static final Block GLOW_VINE = register("glow_vine", GlowVineBlock::new);

  public static final Block SAPLING = register("sapling", SaplingOne::new);
  public static final Block SAPLING2 = register("sapling2", SaplingTwo::new);

  public static final Block FLOWER = register("flower", FlowerBlock::new);
  public static final Block CACTUS_ARM = register("cactus_arm", CactusArm::new);
  public static final Block RGEN_CACTUS = register("rgen_cactus", RgenCactus::new);
  public static final Block CROSS_CACTUS = register("cross_cactus", CrossCactus::new);
  public static final Block JUVENILE_GREEN_CACTUS = register("juvenile_green_cactus", JuvenileGreenCactus::new);
  public static final Block BLUE_CACTUS = register("blue_cactus", BlueCactus::new);
  public static final Block CACTUS = register("cactus", Cactus::new);
  public static final Block JUVENILE_CACTUS = register("juvenile_cactus", JuvenileCactus::new);
  public static final Block LAVENDER_PLANT = register("lavender_plant", LavenderPlant::new);
  public static final Block DOUBLE_TALL_LAVENDER_PLANT = register("double_tall_lavender_plant", DoubleTallLavenderPlant::new);
  public static final Block BASALT = register("basalt", Basalt::new);
  public static final Block PINK_FLOWERBED = register("pink_flowerbed", PinkFlowerbed::new);
  public static final Block HANGING_VINES = register("hanging_vines", HangingVines::new);

  private static Block register(String name, Supplier<Block> blockSupplier) {
    Block block = blockSupplier.get();
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
