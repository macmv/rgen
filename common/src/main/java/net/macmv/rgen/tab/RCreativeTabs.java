// # only v12 #

package net.macmv.rgen.tab;

import net.macmv.rgen.item.RItems;
import net.macmv.rgen.block.LogType;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.HashSet;
import java.util.Set;

public class RCreativeTabs {
  public static final CreativeTabs BUILDING_BLOCKS = new CreativeTabs("rgen_building_blocks") {
    @SideOnly(Side.CLIENT)
    @Override
    public ItemStack getTabIconItem() {
      return new ItemStack(RItems.MOSSY_STUMP);
    }

    @Override
    public void displayAllRelevantItems(NonNullList<ItemStack> items) {
      NonNullList<ItemStack> defaultItems = NonNullList.create();
      super.displayAllRelevantItems(defaultItems);

      Set<Item> organizedWoodItems = new HashSet<>();
      addTree(items, organizedWoodItems, LogType.FIR);
      addTree(items, organizedWoodItems, LogType.PALM);
      addTree(items, organizedWoodItems, LogType.CHERRY);
      addTree(items, organizedWoodItems, LogType.CEDAR);

      addTree(items, organizedWoodItems, LogType.MANGROVE);
      addTree(items, organizedWoodItems, LogType.DEAD);
      addTree(items, organizedWoodItems, LogType.LAVENDER_TREE);

      for (LogType type : new LogType[] {
          LogType.ASPEN,
          LogType.RED_SEASONAL_TREE,
          LogType.ORANGE_SEASONAL_TREE,
          LogType.YELLOW_SEASONAL_TREE
      }) {
        int variant = type.meta - LogType.ASPEN.meta;
        addItem(items, organizedWoodItems, RItems.LEAVES3, variant);
        addItem(items, organizedWoodItems, RItems.SAPLING2, variant);
      }

      // Other wood-related blocks that do not belong to one of the three tree groups.
      addItem(items, organizedWoodItems, RItems.MOSSY_STUMP, 0);
      addItem(items, organizedWoodItems, RItems.MOSSY_STUMP, 1);
      addItem(items, organizedWoodItems, RItems.COVERED_OAK_LOG, 0);
      addItem(items, organizedWoodItems, RItems.COVERED_SPRUCE_LOG, 0);
      addItem(items, organizedWoodItems, RItems.COVERED_BIRCH_LOG, 0);
      addItem(items, organizedWoodItems, RItems.COVERED_JUNGLE_LOG, 0);
      addItem(items, organizedWoodItems, RItems.BAMBOO, 0);
      addItem(items, organizedWoodItems, RItems.BAMBOO_DOOR, 0);

      for (ItemStack stack : defaultItems) {
        if (!organizedWoodItems.contains(stack.getItem())) {
          items.add(stack);
        }
      }
    }

    private void addTree(NonNullList<ItemStack> items, Set<Item> organizedWoodItems, LogType type) {
      if (type.meta < 4) {
        addItem(items, organizedWoodItems, RItems.LOG, type.meta);
        addItem(items, organizedWoodItems, RItems.LEAVES, type.meta);
      } else {
        addItem(items, organizedWoodItems, RItems.LOG2, type.meta - 4);
        addItem(items, organizedWoodItems, RItems.LEAVES2, type.meta - 4);
      }
      addItem(items, organizedWoodItems, RItems.SAPLING, type.meta);
      addItem(items, organizedWoodItems, RItems.PLANKS, type.meta);
      if (type.meta <= LogType.DEAD.meta && type != LogType.SEASONAL) {
        addItem(items, organizedWoodItems, RItems.PLANK_SLAB, type.meta);
      }

      switch (type) {
        case FIR:
          addItem(items, organizedWoodItems, RItems.FIR_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.FIR_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.FIR_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.FIR_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.FIR_FENCE_GATE, 0);
          break;
        case PALM:
          addItem(items, organizedWoodItems, RItems.PALM_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.PALM_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.PALM_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.PALM_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.PALM_FENCE_GATE, 0);
          break;
        case CHERRY:
          addItem(items, organizedWoodItems, RItems.CHERRY_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.CHERRY_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.CHERRY_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.CHERRY_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.CHERRY_FENCE_GATE, 0);
          break;
        case CEDAR:
          addItem(items, organizedWoodItems, RItems.CEDAR_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.CEDAR_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.CEDAR_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.CEDAR_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.CEDAR_FENCE_GATE, 0);
          break;
        case MANGROVE:
          addItem(items, organizedWoodItems, RItems.MANGROVE_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.MANGROVE_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.MANGROVE_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.MANGROVE_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.MANGROVE_FENCE_GATE, 0);
          break;
        case DEAD:
          addItem(items, organizedWoodItems, RItems.DEAD_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.DEAD_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.DEAD_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.DEAD_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.DEAD_FENCE_GATE, 0);
          break;
        case LAVENDER_TREE:
          addItem(items, organizedWoodItems, RItems.LAVENDER_TREE_STAIRS, 0);
          addItem(items, organizedWoodItems, RItems.LAVENDER_TREE_DOOR, 0);
          addItem(items, organizedWoodItems, RItems.LAVENDER_TREE_TRAPDOOR, 0);
          addItem(items, organizedWoodItems, RItems.LAVENDER_TREE_FENCE, 0);
          addItem(items, organizedWoodItems, RItems.LAVENDER_TREE_FENCE_GATE, 0);
          break;
        default:
          break;
      }
    }

    private void addItem(NonNullList<ItemStack> items, Set<Item> organizedWoodItems, Item item, int metadata) {
      items.add(new ItemStack(item, 1, metadata));
      organizedWoodItems.add(item);
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
