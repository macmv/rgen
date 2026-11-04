package net.macmv.rgen.block;

import com.mojang.serialization.MapCodec;
import net.minecraft.block.Block;
import net.minecraft.block.BlockState;
import net.minecraft.block.Blocks;
import net.minecraft.entity.LivingEntity;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.item.HoeItem;
import net.minecraft.item.ItemStack;
import net.minecraft.sound.SoundCategory;
import net.minecraft.sound.SoundEvents;
import net.minecraft.util.Hand;
import net.minecraft.util.ItemActionResult;
import net.minecraft.util.hit.BlockHitResult;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Direction;
import net.minecraft.world.World;

/** The fabric version of the 1.12 `GraveDirtBlock`: coarse dirt that a hoe turns into dirt. */
public class GraveDirtBlock extends Block {
  public GraveDirtBlock(BlockSettings settings) {
    super(settings.vanilla());
  }

  @Override
  protected MapCodec<? extends Block> getCodec() {
    return null;
  }

  @Override
  protected ItemActionResult onUseWithItem(ItemStack stack, BlockState state, World world, BlockPos pos, PlayerEntity player, Hand hand, BlockHitResult hit) {
    if (hit.getSide() == Direction.DOWN || !(stack.getItem() instanceof HoeItem)
        || !world.isAir(pos.up())
        || !player.canModifyAt(world, pos)) {
      return ItemActionResult.PASS_TO_DEFAULT_BLOCK_INTERACTION;
    }

    if (!world.isClient) {
      world.setBlockState(pos, Blocks.DIRT.getDefaultState(), Block.NOTIFY_ALL | Block.REDRAW_ON_MAIN_THREAD);
      stack.damage(1, player, LivingEntity.getSlotForHand(hand));
      world.playSound(null, pos, SoundEvents.ITEM_HOE_TILL, SoundCategory.BLOCKS, 1.0F, 1.0F);
    }
    return ItemActionResult.success(world.isClient);
  }
}
