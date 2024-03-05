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
  public static final Item LOOSE_ROCK = registerBlockItem(RBlocks.LOOSE_ROCK, new ItemMultiTexture(RBlocks.LOOSE_ROCK, RBlocks.LOOSE_ROCK, ty -> LooseRockBlock.RockSize.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Item PLANT = registerBlockItem(RBlocks.PLANT).setCreativeTab(RCreativeTabs.DECORATIONS);
  // public static final Item MOSSY_STUMP = registerBlockItem(RBlocks.MOSSY_STUMP);
  public static final Item MOSSY_STUMP = registerBlockItem(RBlocks.MOSSY_STUMP, new ItemMultiTexture(RBlocks.MOSSY_STUMP, RBlocks.MOSSY_STUMP, ty -> RGenMossyLogBlock.LogType.fromMeta(ty.getMetadata()).name).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS));

  public static final Item MOSSY_CARPET = registerBlockItem(RBlocks.MOSSY_CARPET);
  public static final Item MOSSY_BLOCK = registerBlockItem(RBlocks.MOSSY_BLOCK);
  public static final Item MOSSY_COBBLESTONE_RGEN = registerBlockItem(RBlocks.MOSSY_COBBLESTONE_RGEN);
  public static final Item MOSSY_STONE = registerBlockItem(RBlocks.MOSSY_STONE);

  public static final Item DEBUG_STICK = registerItem("debug_stick", new DebugStickItem()).setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);

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
        ModelResourceLocation loc0 = new ModelResourceLocation("rgen:cedar_log", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 0, loc0);
        ModelResourceLocation loc1 = new ModelResourceLocation("rgen:fir_log", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 1, loc1);
        ModelResourceLocation loc2 = new ModelResourceLocation("rgen:sakura_log", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 2, loc2);
        ModelResourceLocation loc3 = new ModelResourceLocation("rgen:dead_log", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 3, loc3);
      } else if (i.getRegistryName().toString().equals("rgen:mossy_stump")) {
        ModelResourceLocation loc0 = new ModelResourceLocation("rgen:mossy_oak_stump", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 0, loc0);
        ModelResourceLocation loc1 = new ModelResourceLocation("rgen:mossy_birch_stump", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 1, loc1);
      } else if (i.getRegistryName().toString().equals("rgen:loose_rock")) {
        ModelResourceLocation loc0 = new ModelResourceLocation("rgen:small_rock", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 0, loc0);
        ModelResourceLocation loc1 = new ModelResourceLocation("rgen:medium_rock", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 1, loc1);
        ModelResourceLocation loc2 = new ModelResourceLocation("rgen:large_rock", "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 2, loc2);
      } else {
        ModelResourceLocation location = new ModelResourceLocation(i.getRegistryName(), "inventory");
        ModelLoader.setCustomModelResourceLocation(i, 0, location);
      }
    }
  }
}
