package net.macmv.rgen.item;

import net.macmv.rgen.block.RBlocks;
import net.minecraft.block.Block;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.item.Item;
import net.minecraft.item.ItemBlock;
import net.minecraft.util.ResourceLocation;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.registries.IForgeRegistry;

import java.util.HashSet;
import java.util.Set;

public class RItems {
  private static final Set<Item> items = new HashSet<>();

  public static final Item THATCH_ROOF = registerBlockItem(RBlocks.THATCH_ROOF);
  public static final Item DERP_DOG = registerBlockItem(RBlocks.DERP_DOG);
  public static final Item DEAD_LOG = registerBlockItem(RBlocks.DEAD_LOG);

  private static Item registerBlockItem(Block block) {
    ResourceLocation path = block.getRegistryName();
    if (path == null) {
      throw new IllegalArgumentException("block must have a registry name");
    }

    Item item = new ItemBlock(block);
    item.setRegistryName(path);
    item.setUnlocalizedName(path.getResourcePath());
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
      ModelResourceLocation location = new ModelResourceLocation(i.getRegistryName(), "inventory");
      ModelLoader.setCustomModelResourceLocation(i, 0, location);
    }
  }
}
