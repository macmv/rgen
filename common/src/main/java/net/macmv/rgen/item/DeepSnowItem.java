package net.macmv.rgen.item;

import net.macmv.rgen.block.DeepSnowBlock;
import net.minecraft.advancements.CriteriaTriggers;
import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.entity.player.EntityPlayerMP;
import net.minecraft.item.ItemBlock;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class DeepSnowItem extends ItemBlock {
  public DeepSnowItem(Block block) { super(block); }

  @Override public EnumActionResult onItemUse(EntityPlayer player, World world, BlockPos pos,
      EnumHand hand, EnumFacing face, float hitX, float hitY, float hitZ) {
    IBlockState state = world.getBlockState(pos);
    BlockPos target = pos;
    if (state.getBlock() != block || face != EnumFacing.UP) {
      target = pos.offset(face);
      state = world.getBlockState(target);
    }
    if (state.getBlock() == block && state.getValue(DeepSnowBlock.LAYERS) < 4) {
      ItemStack stack = player.getHeldItem(hand);
      if (stack.isEmpty() || !player.canPlayerEdit(target, face, stack)) return EnumActionResult.FAIL;
      IBlockState next = state.withProperty(DeepSnowBlock.LAYERS, state.getValue(DeepSnowBlock.LAYERS) + 1);
      AxisAlignedBB box = next.getCollisionBoundingBox(world, target);
      if (!world.checkNoEntityCollision(box.offset(target))) return EnumActionResult.FAIL;
      if (!world.setBlockState(target, next, 3)) return EnumActionResult.FAIL;
      SoundType sound = block.getSoundType(next, world, target, player);
      world.playSound(player, target, sound.getPlaceSound(), SoundCategory.BLOCKS,
          (sound.getVolume() + 1) / 2, sound.getPitch() * 0.8F);
      if (player instanceof EntityPlayerMP) CriteriaTriggers.PLACED_BLOCK.trigger((EntityPlayerMP) player, target, stack);
      if (!player.capabilities.isCreativeMode) stack.shrink(1);
      return EnumActionResult.SUCCESS;
    }
    return super.onItemUse(player, world, pos, hand, face, hitX, hitY, hitZ);
  }

  @Override public boolean canPlaceBlockOnSide(World world, BlockPos pos, EnumFacing side,
      EntityPlayer player, ItemStack stack) {
    IBlockState state = world.getBlockState(pos);
    if (side == EnumFacing.UP && state.getBlock() == block && state.getValue(DeepSnowBlock.LAYERS) < 4) return true;
    state = world.getBlockState(pos.offset(side));
    return state.getBlock() == block && state.getValue(DeepSnowBlock.LAYERS) < 4
        || super.canPlaceBlockOnSide(world, pos, side, player, stack);
  }
}
