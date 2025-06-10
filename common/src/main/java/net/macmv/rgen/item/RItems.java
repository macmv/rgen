package net.macmv.rgen.item;

import net.macmv.rgen.RGen;
import net.macmv.rgen.block.*;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.item.Item;
import net.minecraft.item.ItemBlock;
import net.minecraft.item.ItemDoor;
import net.minecraft.item.ItemMultiTexture;
import net.minecraft.util.ResourceLocation;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.registries.IForgeRegistry;

import java.util.HashSet;
import java.util.Set;

public class RItems {
  private static final Set<Item> items = new HashSet<>();

  // public static final Item THATCH_ROOF = registerBlockItem(RBlocks.THATCH_ROOF);
  public static final Item DERP_DOG = registerBlockItem(RBlocks.DERP_DOG);
  public static final Item LOG = registerBlockItem(RBlocks.LOG, new ItemMultiTexture(RBlocks.LOG, RBlocks.LOG, ty -> LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Item LOG2 = registerBlockItem(RBlocks.LOG2, new ItemMultiTexture(RBlocks.LOG2, RBlocks.LOG2, it -> LogType.fromMeta(it.getMetadata() + 4).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  // public static final Item MOSSY_STUMP = registerBlockItem(RBlocks.MOSSY_STUMP);
  public static final Item MOSSY_STUMP = registerBlockItem(RBlocks.MOSSY_STUMP, new ItemMultiTexture(RBlocks.MOSSY_STUMP, RBlocks.MOSSY_STUMP, ty -> MossyLogBlock.LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Item LEAVES = registerBlockItem(RBlocks.LEAVES, new ItemMultiTexture(RBlocks.LEAVES, RBlocks.LEAVES, ty -> LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Item LEAVES2 = registerBlockItem(RBlocks.LEAVES2, new ItemMultiTexture(RBlocks.LEAVES2, RBlocks.LEAVES2, it -> LogType.fromMeta(it.getMetadata() + 4).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Item SAPLING = registerBlockItem(RBlocks.SAPLING, new ItemMultiTexture(RBlocks.SAPLING, RBlocks.SAPLING, it -> LogType.fromMeta(it.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Item SAPLING2 = registerBlockItem(RBlocks.SAPLING2, new ItemMultiTexture(RBlocks.SAPLING2, RBlocks.SAPLING2, it -> LogType.fromMeta(it.getMetadata() + 8).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Item RED_POLYPORE = registerBlockItem(RBlocks.RED_POLYPORE);

  public static final Item FLOWER = registerBlockItem(RBlocks.FLOWER, new ItemMultiTexture(RBlocks.FLOWER, RBlocks.FLOWER, it -> FlowerBlock.FlowerType.fromMeta(it.getMetadata()).name).setCreativeTab(RCreativeTabs.DECORATIONS));

  public static final Item PLANKS = registerBlockItem(RBlocks.PLANKS, new ItemMultiTexture(RBlocks.PLANKS, RBlocks.PLANKS, ty -> LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Item LOOSE_ROCK = registerBlockItem(RBlocks.LOOSE_ROCK, new ItemMultiTexture(RBlocks.LOOSE_ROCK, RBlocks.LOOSE_ROCK, ty -> LooseRockBlock.RockSize.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Item PLANT = registerBlockItem(RBlocks.PLANT).setCreativeTab(RCreativeTabs.DECORATIONS);

  public static final Item MOSSY_CARPET = registerBlockItem(RBlocks.MOSSY_CARPET);
  public static final Item MOSSY_BLOCK = registerBlockItem(RBlocks.MOSSY_BLOCK);
  public static final Item MOSSY_COBBLESTONE_RGEN = registerBlockItem(RBlocks.MOSSY_COBBLESTONE_RGEN);
  public static final Item MOSSY_STONE = registerBlockItem(RBlocks.MOSSY_STONE);
  public static final Item BAMBOO = registerBlockItem(RBlocks.BAMBOO);
  public static final Item GLOW_VINE = registerBlockItem(RBlocks.GLOW_VINE);
  public static final Item LAVENDER_PLANT = registerBlockItem(RBlocks.LAVENDER_PLANT, new ItemBlock(RBlocks.LAVENDER_PLANT).setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Item DOUBLE_TALL_LAVENDER_PLANT = registerBlockItem(RBlocks.DOUBLE_TALL_LAVENDER_PLANT, new ItemBlock(RBlocks.DOUBLE_TALL_LAVENDER_PLANT).setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Item BASALT = registerBlockItem(RBlocks.BASALT);
  public static final Item PINK_FLOWERBED = registerBlockItem(RBlocks.PINK_FLOWERBED);
  public static final Item HANGING_VINES = registerBlockItem(RBlocks.HANGING_VINES);


  public static final Item DEAD_DOOR = registerItem("dead_door", new ItemDoor(RBlocks.DEAD_DOOR));
  public static final Item MANGROVE_DOOR = registerItem("mangrove_door", new ItemDoor(RBlocks.MANGROVE_DOOR));
  public static final Item BAMBOO_DOOR = registerItem("bamboo_door", new ItemDoor(RBlocks.BAMBOO_DOOR));
  public static final Item CEDAR_DOOR = registerItem("cedar_door", new ItemDoor(RBlocks.CEDAR_DOOR));
  public static final Item CHERRY_DOOR = registerItem("cherry_door", new ItemDoor(RBlocks.CHERRY_DOOR));
  public static final Item FIR_DOOR = registerItem("fir_door", new ItemDoor(RBlocks.FIR_DOOR));
  public static final Item PALM_DOOR = registerItem("palm_door", new ItemDoor(RBlocks.PALM_DOOR));

  public static final Item DEAD_STAIRS = registerBlockItem(RBlocks.DEAD_STAIRS);
  public static final Item MANGROVE_STAIRS = registerBlockItem(RBlocks.MANGROVE_STAIRS);
  //public static final Item BAMBOO_STAIRS = registerItem("bamboo_stairs", new ItemBlock(RBlocks.BAMBOO_STAIRS));
  public static final Item CEDAR_STAIRS = registerBlockItem(RBlocks.CEDAR_STAIRS);
  public static final Item CHERRY_STAIRS = registerBlockItem(RBlocks.CHERRY_STAIRS);
  public static final Item FIR_STAIRS = registerBlockItem(RBlocks.FIR_STAIRS);
  public static final Item PALM_STAIRS = registerBlockItem(RBlocks.PALM_STAIRS);

  public static final Item DEAD_TRAPDOOR = registerBlockItem(RBlocks.DEAD_TRAPDOOR);
  public static final Item MANGROVE_TRAPDOOR = registerBlockItem(RBlocks.MANGROVE_TRAPDOOR);
  // public static final Item BAMBOO_TRAPDOOR = registerBlockItem(RBlocks.BAMBOO_TRAPDOOR);
  public static final Item CEDAR_TRAPDOOR = registerBlockItem(RBlocks.CEDAR_TRAPDOOR);
  public static final Item CHERRY_TRAPDOOR = registerBlockItem(RBlocks.CHERRY_TRAPDOOR);
  public static final Item FIR_TRAPDOOR = registerBlockItem(RBlocks.FIR_TRAPDOOR);
  public static final Item PALM_TRAPDOOR = registerBlockItem(RBlocks.PALM_TRAPDOOR);


  public static final Item COVERED_OAK_LOG = registerBlockItem(RBlocks.COVERED_OAK_LOG);
  public static final Item COVERED_SPRUCE_LOG = registerBlockItem(RBlocks.COVERED_SPRUCE_LOG);
  public static final Item COVERED_BIRCH_LOG = registerBlockItem(RBlocks.COVERED_BIRCH_LOG);
  public static final Item COVERED_JUNGLE_LOG = registerBlockItem(RBlocks.COVERED_JUNGLE_LOG);


  public static final Item DEBUG_STICK = registerItem("debug_stick", new DebugStickItem()).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
  public static final Item MOSS_COMPASS = registerItem("moss_compass", new MossCompass()).setCreativeTab(RCreativeTabs.DECORATIONS);
  public static final Item GREEN_CACTUS_FRUIT = registerItem("green_cactus_fruit", new GreenCactusFruit()).setCreativeTab(RCreativeTabs.DECORATIONS);
  public static final Item BLUE_CACTUS_FRUIT = registerItem("blue_cactus_fruit", new BlueCactusFruit()).setCreativeTab(RCreativeTabs.DECORATIONS);


  // Other cactus fruits


  private static Item registerBlockItem(Block block) {
    return registerBlockItem(block, new ItemBlock(block));
  }

  private static Item registerBlockItem(Block block, Item item) {
    ResourceLocation path = block.getRegistryName();
    if (path == null) {
      throw new IllegalArgumentException("block must have a registry name");
    }

    return registerItem(path.getResourcePath(), item);
  }

  private static Item registerItem(String name, Item item) {
    item.setRegistryName(new ResourceLocation(RGen.MOD_ID, name));
    item.setUnlocalizedName(name);
    items.add(item);
    return item;
  }

  public static void registerItems(IForgeRegistry<Item> reg) {
    for (Item i : items) {
      reg.register(i);
    }
  }

  public static void registerModels() {
    for (Item i : items) {
      if (i.getRegistryName().toString().equals("rgen:log")) {
        registerModel(i, LogType.FIR.meta, "rgen:fir_log");
        registerModel(i, LogType.PALM.meta, "rgen:palm_log");
        registerModel(i, LogType.SAKURA.meta, "rgen:sakura_log");
        registerModel(i, LogType.CEDAR.meta, "rgen:cedar_log");
      } else if (i.getRegistryName().toString().equals("rgen:log2")) {
        registerModel(i, LogType.MANGROVE.meta - 4, "rgen:mangrove_log");
        registerModel(i, LogType.DEAD.meta - 4, "rgen:dead_log");
      } else if (i.getRegistryName().toString().equals("rgen:leaves")) {
        registerModel(i, LogType.FIR.meta, "rgen:fir_leaves");
        registerModel(i, LogType.PALM.meta, "rgen:palm_leaves");
        registerModel(i, LogType.SAKURA.meta, "rgen:sakura_leaves");
        registerModel(i, LogType.CEDAR.meta, "rgen:cedar_leaves");
      } else if (i.getRegistryName().toString().equals("rgen:leaves2")) {
        registerModel(i, LogType.MANGROVE.meta - 4, "rgen:mangrove_leaves");
      } else if (i.getRegistryName().toString().equals("rgen:sapling")) {
        registerModel(i, LogType.FIR.meta, "rgen:fir_sapling");
        registerModel(i, LogType.PALM.meta, "rgen:palm_sapling");
        registerModel(i, LogType.SAKURA.meta, "rgen:sakura_sapling");
        registerModel(i, LogType.CEDAR.meta, "rgen:cedar_sapling");
        registerModel(i, LogType.MANGROVE.meta, "rgen:mangrove_sapling");
        registerModel(i, LogType.LAVENDER.meta, "rgen:lavender_sapling");
        registerModel(i, LogType.SEASONAL.meta, "rgen:seasonal_sapling");
        registerModel(i, LogType.DEAD.meta, "rgen:dead_sapling");
      } else if (i.getRegistryName().toString().equals("rgen:sapling2")) {
        registerModel(i, LogType.ASPEN.meta - 8, "rgen:aspen_sapling");
      } else if (i.getRegistryName().toString().equals("rgen:planks")) {
        registerModel(i, LogType.FIR.meta, "rgen:fir_planks");
        registerModel(i, LogType.PALM.meta, "rgen:palm_planks");
        registerModel(i, LogType.SAKURA.meta, "rgen:sakura_planks");
        registerModel(i, LogType.CEDAR.meta, "rgen:cedar_planks");
        registerModel(i, LogType.MANGROVE.meta, "rgen:mangrove_planks");
      } else if (i.getRegistryName().toString().equals("rgen:flower")) {
        registerModel(i, FlowerBlock.FlowerType.FORGETMENOT.meta, "rgen:forgetmenot");
      } else if (i.getRegistryName().toString().equals("rgen:mossy_stump")) {
        registerModel(i, 0, "rgen:mossy_oak_stump");
        registerModel(i, 1, "rgen:mossy_birch_stump");
      } else if (i.getRegistryName().toString().equals("rgen:loose_rock")) {
        registerModel(i, 0, "rgen:small_rock");
        registerModel(i, 1, "rgen:medium_rock");
        registerModel(i, 2, "rgen:large_rock");
      } else {
        ModelResourceLocation location = new ModelResourceLocation(i.getRegistryName(), "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 0, location);
      }
    }
  }

  private static void registerModel(Item item, int meta, String name) {
    ModelResourceLocation location = new ModelResourceLocation(name, "inventory");
    ModelLoader.setCustomModelResourceLocation(item, meta, location);
  }
}
