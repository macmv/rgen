package net.macmv.rgen.block;

import net.minecraft.block.BlockDoor;
import net.minecraft.block.state.IBlockState;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.math.BlockPos;
import java.util.Random;
import net.minecraft.world.World;
import net.macmv.rgen.item.RItems;

public class RDoor extends BlockDoor {

  public RDoor(BlockSettings settings) {
    super(settings.material);
    this.setHardness(settings.hardness);
    this.setSoundType(settings.soundType);
    this.disableStats();
  }

  @Override
  public Item getItemDropped(IBlockState state, Random random, int fortune) {
    return state.getValue(HALF) == EnumDoorHalf.UPPER ? net.minecraft.init.Items.AIR : getDoorItem();
  }

  @Override
  public ItemStack getItem(World world, BlockPos pos, IBlockState state) {
    Item doorItem = getDoorItem();
    return doorItem == null ? super.getItem(world, pos, state) : new ItemStack(doorItem);
  }

  private Item getDoorItem() {
    String name = this.getRegistryName().getResourcePath();
    switch (name) {
      case "dead_door": return RItems.DEAD_DOOR;
      case "burned_tree_door": return RItems.BURNED_TREE_DOOR;
      case "mangrove_door": return RItems.MANGROVE_DOOR;
      case "bamboo_door": return RItems.BAMBOO_DOOR;
      case "cedar_door": return RItems.CEDAR_DOOR;
      case "cherry_door": return RItems.CHERRY_DOOR;
      case "fir_door": return RItems.FIR_DOOR;
      case "palm_door": return RItems.PALM_DOOR;
      case "lavender_tree_door": return RItems.LAVENDER_TREE_DOOR;
      default: return null;
    }
  }
}
