package net.macmv.rgen.block;

import net.macmv.rgen.RGen;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.block.BlockDirt;
import net.minecraft.block.BlockDoor;
import net.minecraft.block.BlockPlanks;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.client.renderer.block.statemap.StateMap;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.function.Function;

public final class RBlocks {
  private static final Set<Block> blocks = new HashSet<>();
  private static final Map<String, Block> aliases = new HashMap<>();

  public static final Block DERP_DOG = register("derp_dog", s -> s.createMat(Block::new));
  public static final Block LOOSE_ROCK = register("loose_rock", LooseRockBlock::new, new BlockSettings().creativeTab(RCreativeTabs.DECORATIONS));
  public static final Block PLANT = register("plant", PlantBlock::new, new BlockSettings().material(Material.GRASS).creativeTab(RCreativeTabs.DECORATIONS));
  // public static final Block MOSSY_STUMP = register("mossy_stump", Block::new, new BlockSettings().material(Material.WOOD));
  // Backported blocks: where vanilla has the block, we use that. Otherwise, we register our own.
  //? if vanilla_moss {
  /*public static final Block MOSSY_CARPET = backport("mossy_carpet", net.minecraft.block.Blocks.MOSS_CARPET);
  public static final Block MOSSY_BLOCK = backport("mossy_block", net.minecraft.block.Blocks.MOSS_BLOCK);*/
  //?} else {
  public static final Block MOSSY_CARPET = register("mossy_carpet", MossCarpet::new, new BlockSettings().material(Material.CARPET).creativeTab(RCreativeTabs.DECORATIONS));
  public static final Block MOSSY_BLOCK = register("mossy_block", MossBlock::new, new BlockSettings().material(Material.GROUND));
  //?}
  public static final Block MOSSY_COBBLESTONE_RGEN = register("mossy_cobblestone_rgen", PlantableStone::new, new BlockSettings().material(Material.ROCK).creativeTab(CreativeTabs.DECORATIONS));
  public static final Block MOSSY_STONE = register("mossy_stone", PlantableStone::new, new BlockSettings().material(Material.ROCK).creativeTab(CreativeTabs.DECORATIONS));
  public static final Block GRAVE_DIRT = register("grave_dirt", GraveDirtBlock::new, new BlockSettings().material(Material.GROUND).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(0.5f).resistance(2.5f).soundType(SoundType.GROUND));
  public static final Block POLYPORE = register("polypore", PolyporeBlock::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block RED_POLYPORE = register("red_polypore", PolyporeBlock::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));

  public static final Block LOG = register("log", LogBlockOne::new, new BlockSettings().creativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Block LOG2 = register("log2", LogBlockTwo::new, new BlockSettings().creativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Block PLANKS = register("planks", PlanksBlock::new, new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block PLANK_SLAB = register("plank_slab", RWoodSlab.Half::new, new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block PLANK_DOUBLE_SLAB = register("plank_double_slab", RWoodSlab.Double::new, new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block FIR_FENCE = register("fir_fence", s -> new RWoodFence(s, LogType.FIR), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block PALM_FENCE = register("palm_fence", s -> new RWoodFence(s, LogType.PALM), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_FENCE = register("cherry_fence", s -> new RWoodFence(s, LogType.CHERRY), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_FENCE = register("cedar_fence", s -> new RWoodFence(s, LogType.CEDAR), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_FENCE = register("mangrove_fence", s -> new RWoodFence(s, LogType.MANGROVE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block LAVENDER_TREE_FENCE = register("lavender_tree_fence", s -> new RWoodFence(s, LogType.LAVENDER_TREE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block DEAD_FENCE = register("dead_fence", s -> new RWoodFence(s, LogType.DEAD), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block BURNED_TREE_FENCE = register("burned_tree_fence", s -> new RWoodFence(s, LogType.BURNED_TREE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));

  public static final Block FIR_FENCE_GATE = register("fir_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.SPRUCE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block PALM_FENCE_GATE = register("palm_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.JUNGLE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_FENCE_GATE = register("cherry_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.BIRCH), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_FENCE_GATE = register("cedar_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.SPRUCE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_FENCE_GATE = register("mangrove_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.JUNGLE), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block LAVENDER_TREE_FENCE_GATE = register("lavender_tree_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.ACACIA), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block DEAD_FENCE_GATE = register("dead_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.DARK_OAK), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block BURNED_TREE_FENCE_GATE = register("burned_tree_fence_gate", s -> new RWoodFenceGate(s, BlockPlanks.EnumType.DARK_OAK), new BlockSettings().material(Material.WOOD).creativeTab(RCreativeTabs.BUILDING_BLOCKS).hardness(2.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block LAVENDER_TREE_STAIRS = register("lavender_tree_stairs", s -> new RStairs(s, LogType.LAVENDER_TREE), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  // Stairs
  // public static final Block BAMBOO_STAIRS = register("bamboo_stairs", new BambooStairs());
  public static final Block DEAD_STAIRS = register("dead_stairs", s -> new RStairs(s, LogType.DEAD), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block BURNED_TREE_STAIRS = register("burned_tree_stairs", s -> new RStairs(s, LogType.BURNED_TREE), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_STAIRS = register("mangrove_stairs", s -> new RStairs(s, LogType.MANGROVE), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_STAIRS = register("cedar_stairs", s -> new RStairs(s, LogType.CEDAR), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_STAIRS = register("cherry_stairs", s -> new RStairs(s, LogType.CHERRY), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block FIR_STAIRS = register("fir_stairs", s -> new RStairs(s, LogType.FIR), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));
  public static final Block PALM_STAIRS = register("palm_stairs", s -> new RStairs(s, LogType.PALM), new BlockSettings().material(Material.WOOD).hardness(2.0f).soundType(SoundType.WOOD));

  // Doors
  public static final Block DEAD_DOOR = register("dead_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block BURNED_TREE_DOOR = register("burned_tree_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_DOOR = register("mangrove_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block BAMBOO_DOOR = register("bamboo_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_DOOR = register("cedar_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_DOOR = register("cherry_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block FIR_DOOR = register("fir_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block PALM_DOOR = register("palm_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));
  public static final Block LAVENDER_TREE_DOOR = register("lavender_tree_door", RDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).soundType(SoundType.WOOD));

  // TrapDoors
  public static final Block DEAD_TRAPDOOR = register("dead_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block BURNED_TREE_TRAPDOOR = register("burned_tree_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block MANGROVE_TRAPDOOR = register("mangrove_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CEDAR_TRAPDOOR = register("cedar_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block CHERRY_TRAPDOOR = register("cherry_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block FIR_TRAPDOOR = register("fir_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block PALM_TRAPDOOR = register("palm_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block LAVENDER_TREE_TRAPDOOR = register("lavender_tree_trapdoor", RTrapDoor::new, new BlockSettings().material(Material.WOOD).hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));

  // CoveredLogs
  public static final Block COVERED_OAK_LOG = register("covered_oak_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_SPRUCE_LOG = register("covered_spruce_log", s -> new RCoveredLog(s, false), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_BIRCH_LOG = register("covered_birch_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_JUNGLE_LOG = register("covered_jungle_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_ACACIA_LOG = register("covered_acacia_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_DARK_OAK_LOG = register("covered_dark_oak_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_FIR_LOG = register("covered_fir_log", s -> new RCoveredLog(s, false), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_PALM_LOG = register("covered_palm_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_CHERRY_LOG = register("covered_cherry_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_CEDAR_LOG = register("covered_cedar_log", s -> new RCoveredLog(s, true), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));
  public static final Block COVERED_DEAD_LOG = register("covered_dead_log", s -> new RCoveredLog(s, false), new BlockSettings().hardness(3.0f).resistance(5.0f).soundType(SoundType.WOOD));

  // This block is to be deprecated do not USE
  public static final Block MOSSY_STUMP = register("mossy_stump", MossyLogBlock::new, new BlockSettings().creativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Block BAMBOO = register("bamboo", Bamboo::new, new BlockSettings().material(Material.PLANTS));

  public static final Block LEAVES = register("leaves", LeavesBlockOne::new);
  public static final Block LEAVES2 = register("leaves2", LeavesBlockTwo::new);
  public static final Block LEAVES3 = register("leaves3", LeavesBlockThree::new);
  public static final Block GLOW_VINE = register("glow_vine", GlowVineBlock::new, new BlockSettings().lightValue(3));

  public static final Block SAPLING = register("sapling", SaplingOne::new);
  public static final Block BURNED_TREE_SAPLING = register("burned_tree_sapling", BurnedTreeSapling::new);
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

  public static final Block SALT = register("salt",SaltBlock::new, new BlockSettings().material(Material.SAND).hardness(1.0f).resistance(2.5f).soundType(SoundType.SAND));
  public static final Block SALT_ON_SAND = register("salt_on_sand",SaltOnSand::new, new BlockSettings().material(Material.SAND).hardness(1.0f).resistance(2.5f).soundType(SoundType.SAND));

  public static final Block PINK_ORCHID = register("pink_orchid", JungleFlowerFlowerType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block PASSION_FLOWER = register("passion_flower", JungleFlowerFlowerType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block HELICONIA = register("heliconia", JungleFlowerFlowerType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block PINK_HEART = register("pink_heart", JungleFlowerFlowerType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block TORCH_GINGER = register("torch_ginger", JungleFlowerFlowerType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block ORCHIDACEAE = register("orchidaceae", JungleFlowerFlowerType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block IPOMOEA = register("ipomoea", JungleFlowerCarpetType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block BROMELIADS = register("bromeliads", JungleFlowerGrassType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block FICUS_ELASTICA = register("ficus_elastica", JungleFlowerGrassType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block YELLOW_JUNGLE_ROSE = register("yellow_jungle_rose", JungleFlowerGrassType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block BIRD_OF_PARADISE = register("bird_of_paradise", JungleFlowerGrassType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));
  public static final Block JUNGLE_BUSH = register("jungle_bush", JungleFlowerGrassType::new, new BlockSettings().material(Material.PLANTS).creativeTab(RCreativeTabs.DECORATIONS).soundType(SoundType.PLANT));



  private static Block register(String name, Function<BlockSettings, Block> blockSupplier) {
    Block block = blockSupplier.apply(new BlockSettings() /* #fabric: .name(name) */);
    // #v12-start
    block.setRegistryName(RGen.MOD_ID, name);
    block.setUnlocalizedName(name);
    // #v12-end
    /* #fabric: net.minecraft.registry.Registry.register(net.minecraft.registry.Registries.BLOCK, net.minecraft.util.Identifier.of(RGen.MOD_ID, name), block); */
    blocks.add(block);
    return block;
  }

  private static Block register(String name, Function<BlockSettings, Block> blockSupplier, BlockSettings settings) {
    Block block = blockSupplier.apply(settings /* #fabric: .name(name) */);
    // #v12-start
    block.setRegistryName(RGen.MOD_ID, name);
    block.setUnlocalizedName(name);
    // #v12-end
    /* #fabric: net.minecraft.registry.Registry.register(net.minecraft.registry.Registries.BLOCK, net.minecraft.util.Identifier.of(RGen.MOD_ID, name), block); */
    blocks.add(block);
    return block;
  }

  // Registers `block` (a vanilla block) as the stand in for our block named `name`.
  //
  // Nothing gets registered for it, but it can still be found with `byName("rgen:<name>")`, so the
  // terrain generator can keep placing `rgen:<name>` on every version.
  private static Block backport(String name, Block block) {
    aliases.put(RGen.MOD_ID + ":" + name, block);
    return block;
  }

  // Finds a block by its registry name, following the backports above. Returns null if there is no
  // such block.
  public static Block byName(String name) {
    Block alias = aliases.get(name);
    if (alias != null) {
      return alias;
    }

    // #v12-start
    return Block.getBlockFromName(name);
    // #v12-end
    /* #fabric: return net.minecraft.registry.Registries.BLOCK.get(net.minecraft.util.Identifier.of(name)); */
  }

  // Loads this class, which registers the blocks on fabric.
  public static void init() {}

  // #v12-start
  public static void registerBlocks(IForgeRegistry<Block> reg) {
    for (Block b : blocks) {
      reg.register(b);
    }
  }

  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    ModelLoader.setCustomStateMapper(GRAVE_DIRT, (new StateMap.Builder()).ignore(BlockDirt.SNOWY).build());
    // Ignores powered metadata alongside the rest of the metadata on the client side.
    ModelLoader.setCustomStateMapper(DEAD_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(BURNED_TREE_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(MANGROVE_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(BAMBOO_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(CEDAR_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(CHERRY_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(FIR_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(PALM_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
    ModelLoader.setCustomStateMapper(LAVENDER_TREE_DOOR, (new StateMap.Builder()).ignore(BlockDoor.POWERED).build());
  }
  // #v12-end
}
