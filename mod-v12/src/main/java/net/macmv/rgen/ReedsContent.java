package net.macmv.rgen;

import net.minecraft.block.Block;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.item.Item;
import net.minecraft.util.ResourceLocation;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

public final class ReedsContent {
  public static final ReedsBlock BLOCK = new ReedsBlock();
  public static final Item ITEM = (new ReedsItem(BLOCK)).setRegistryName("rgen", "reeds")
      .setCreativeTab(RCreativeTabs.DECORATIONS);

  private ReedsContent() {}

  public static void registerBlocks(IForgeRegistry<Block> registry) { registry.register(BLOCK); }
  public static void registerItems(IForgeRegistry<Item> registry) { registry.register(ITEM); }

  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    ModelLoader.setCustomModelResourceLocation(ITEM, 0,
        new ModelResourceLocation(new ResourceLocation("rgen", "reeds"), "inventory"));
  }
}
