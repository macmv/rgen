package net.macmv.rgen;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.item.Item;
import net.minecraft.item.ItemBlock;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

public final class BrittleIceContent {
  public static final BrittleIceBlock BLOCK = new BrittleIceBlock();
  public static final Item ITEM = new ItemBlock(BLOCK).setRegistryName("rgen", "brittle_ice")
      .setUnlocalizedName("brittle_ice").setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
  private BrittleIceContent() {}
  public static void registerBlocks(IForgeRegistry<Block> registry) { registry.register(BLOCK); }
  public static void registerItems(IForgeRegistry<Item> registry) { registry.register(ITEM); }
  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    ModelLoader.setCustomModelResourceLocation(ITEM, 0,
        new ModelResourceLocation("rgen:brittle_ice", "inventory"));
  }
}
