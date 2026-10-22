package net.macmv.rgen.item;

import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.compat.ItemSettings;
import net.minecraft.block.Blocks;
import net.minecraft.component.type.FoodComponent;
import net.minecraft.item.Item;
import net.minecraft.item.ItemUsageContext;
import net.minecraft.util.ActionResult;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Direction;
import net.minecraft.world.World;

// The fabric version of the green cactus fruit. The 1.12 one is a `ItemFood` in `common/`.
public class GreenCactusFruit extends Item {
  public GreenCactusFruit() {
    super(ItemSettings.create().food(new FoodComponent.Builder().nutrition(1).saturationModifier(0.3F).build()));
  }

  @Override
  public ActionResult useOnBlock(ItemUsageContext context) {
    World world = context.getWorld();
    BlockPos pos = context.getBlockPos().offset(context.getSide());

    if (world.getBlockState(pos.down()).isOf(Blocks.SAND) && context.getSide() == Direction.UP) {
      world.setBlockState(pos, RBlocks.JUVENILE_GREEN_CACTUS.getDefaultState());
      context.getStack().decrement(1);
      return ActionResult.SUCCESS;
    }

    return ActionResult.FAIL;
  }
}
