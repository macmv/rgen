package net.macmv.rgen.tab;

import net.macmv.rgen.item.RItems;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class RCreativeTabs {
  public static final CreativeTabs BUILDING_BLOCKS = new CreativeTabs("rgen_building_blocks") {
    @SideOnly(Side.CLIENT)
    @Override
    public ItemStack getTabIconItem() {
      return new ItemStack(RItems.MOSSY_STUMP);
    }
  };

  public static final CreativeTabs DECORATIONS = new CreativeTabs("rgen_decorations") {
    @SideOnly(Side.CLIENT)
    @Override
    public ItemStack getTabIconItem() {
      return new ItemStack(RItems.PLANT);
    }

    @Override
    public void displayAllRelevantItems(NonNullList<ItemStack> p_78018_1_) {
      System.out.println("displaying decoration items");
      for (Item item : Item.REGISTRY) {
        if (item == RItems.LOOSE_ROCK) {
          System.out.println("foo: " + item.getCreativeTab());
          System.out.println("foo: " + p_78018_1_.size());
        }
        item.getSubItems(this, p_78018_1_);
      }
      // super.displayAllRelevantItems(p_78018_1_);
    }
  };
}
