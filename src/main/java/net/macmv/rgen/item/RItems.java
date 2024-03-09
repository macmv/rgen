package net.macmv.rgen.item;

import net.macmv.rgen.RGen;
import net.macmv.rgen.block.LooseRockBlock;
import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.block.RGenLogBlockOne;
import net.macmv.rgen.block.RGenMossyLogBlock;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.item.Item;
import net.minecraft.item.ItemBlock;
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
  public static final Item LOG = registerBlockItem(RBlocks.LOG, new ItemMultiTexture(RBlocks.LOG, RBlocks.LOG, ty -> RGenLogBlockOne.LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Item LOG2 = registerBlockItem(RBlocks.LOG2, new ItemMultiTexture(RBlocks.LOG2, RBlocks.LOG2, it -> RGenLogBlockOne.LogType.fromMeta(it.getMetadata() + 4).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  // public static final Item MOSSY_STUMP = registerBlockItem(RBlocks.MOSSY_STUMP);
  public static final Item MOSSY_STUMP = registerBlockItem(RBlocks.MOSSY_STUMP, new ItemMultiTexture(RBlocks.MOSSY_STUMP, RBlocks.MOSSY_STUMP, ty -> RGenMossyLogBlock.LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Item LEAVES = registerBlockItem(RBlocks.LEAVES, new ItemMultiTexture(RBlocks.LEAVES, RBlocks.LEAVES, ty -> RGenLogBlockOne.LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));
  public static final Item LEAVES2 = registerBlockItem(RBlocks.LEAVES2, new ItemMultiTexture(RBlocks.LEAVES2, RBlocks.LEAVES2, it -> RGenLogBlockOne.LogType.fromMeta(it.getMetadata() + 4).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Item LOOSE_ROCK = registerBlockItem(RBlocks.LOOSE_ROCK, new ItemMultiTexture(RBlocks.LOOSE_ROCK, RBlocks.LOOSE_ROCK, ty -> LooseRockBlock.RockSize.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Item PLANT = registerBlockItem(RBlocks.PLANT).setCreativeTab(RCreativeTabs.DECORATIONS);

  public static final Item MOSSY_CARPET = registerBlockItem(RBlocks.MOSSY_CARPET);
  public static final Item MOSSY_BLOCK = registerBlockItem(RBlocks.MOSSY_BLOCK);
  public static final Item MOSSY_COBBLESTONE_RGEN = registerBlockItem(RBlocks.MOSSY_COBBLESTONE_RGEN);
  public static final Item MOSSY_STONE = registerBlockItem(RBlocks.MOSSY_STONE);

  public static final Item DEBUG_STICK = registerItem("debug_stick", new DebugStickItem()).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
  public static final Item MOSS_COMPASS = registerItem("moss_compass", new MossCompass()).setCreativeTab(RCreativeTabs.DECORATIONS);

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
    item.setRegistryName(new ResourceLocation(RGen.MODID, name));
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
        registerModel(i, RGenLogBlockOne.LogType.FIR.meta, "rgen:fir_log");
        registerModel(i, RGenLogBlockOne.LogType.PALM.meta, "rgen:palm_log");
        registerModel(i, RGenLogBlockOne.LogType.SAKURA.meta, "rgen:sakura_log");
        registerModel(i, RGenLogBlockOne.LogType.CEDAR.meta, "rgen:cedar_log");
      } else if (i.getRegistryName().toString().equals("rgen:log2")) {
        registerModel(i, RGenLogBlockOne.LogType.MANGROVE.meta - 4, "rgen:mangrove_log");
        registerModel(i, RGenLogBlockOne.LogType.DEAD.meta - 4, "rgen:dead_log");
      } else if (i.getRegistryName().toString().equals("rgen:leaves")) {
        registerModel(i, RGenLogBlockOne.LogType.FIR.meta, "rgen:fir_leaves");
        registerModel(i, RGenLogBlockOne.LogType.PALM.meta, "rgen:palm_leaves");
        registerModel(i, RGenLogBlockOne.LogType.SAKURA.meta, "rgen:sakura_leaves");
        registerModel(i, RGenLogBlockOne.LogType.CEDAR.meta, "rgen:cedar_leaves");
      } else if (i.getRegistryName().toString().equals("rgen:leaves2")) {
        registerModel(i, RGenLogBlockOne.LogType.MANGROVE.meta - 4, "rgen:mangrove_leaves");
        registerModel(i, RGenLogBlockOne.LogType.DEAD.meta - 4, "rgen:dead_leaves");
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
