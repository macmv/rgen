package net.macmv.rgen;

import net.minecraft.block.state.IBlockState;
import net.minecraft.block.BlockLiquid;
import net.minecraft.block.material.Material;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.util.ActionResult;
import net.minecraft.item.ItemBlock;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.RayTraceResult;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class ReedsItem extends ItemBlock {
  public ReedsItem(ReedsBlock block) { super(block); }

  @Override public ActionResult<ItemStack> onItemRightClick(World world, EntityPlayer player, EnumHand hand) {
    ItemStack stack = player.getHeldItem(hand);
    RayTraceResult hit = rayTrace(world, player, true);
    if (hit == null || hit.typeOfHit != RayTraceResult.Type.BLOCK) {
      return new ActionResult<>(EnumActionResult.PASS, stack);
    }

    BlockPos waterPos = hit.getBlockPos();
    BlockPos plantPos = waterPos.up();
    if (!world.isBlockModifiable(player, waterPos)
        || !player.canPlayerEdit(plantPos, hit.sideHit, stack)) {
      return new ActionResult<>(EnumActionResult.FAIL, stack);
    }
    return new ActionResult<>(placeOnWater(world, player, stack, waterPos, hit.sideHit), stack);
  }

  @Override public EnumActionResult onItemUse(EntityPlayer player, World world, BlockPos clickedPos, EnumHand hand,
                                               EnumFacing facing, float hitX, float hitY, float hitZ) {
    return placeOnWater(world, player, player.getHeldItem(hand), clickedPos, facing);
  }

  private EnumActionResult placeOnWater(World world, EntityPlayer player, ItemStack stack, BlockPos waterPos, EnumFacing facing) {
    if (stack.isEmpty()) return EnumActionResult.FAIL;
    IBlockState water = world.getBlockState(waterPos);
    if (water.getMaterial() != Material.WATER) return EnumActionResult.PASS;
    if ((water.getBlock() == Blocks.WATER || water.getBlock() == Blocks.FLOWING_WATER)
        && water.getValue(BlockLiquid.LEVEL) != 0) return EnumActionResult.FAIL;

    BlockPos plantPos = waterPos.up();
    IBlockState plantState = ReedsContent.BLOCK.getDefaultState();
    if (!world.isAirBlock(plantPos) || !player.canPlayerEdit(plantPos, facing, stack)
        || !ReedsContent.BLOCK.canBlockStay(world, plantPos, plantState)) return EnumActionResult.FAIL;

    if (!world.isRemote) {
      world.setBlockState(plantPos, plantState, 3);
      net.minecraft.block.SoundType sound = ReedsContent.BLOCK.getSoundType(plantState, world, plantPos, player);
      world.playSound(player, plantPos, sound.getPlaceSound(), SoundCategory.BLOCKS,
          (sound.getVolume() + 1.0F) / 2.0F, sound.getPitch() * 0.8F);
      if (!player.capabilities.isCreativeMode) stack.shrink(1);
    }
    return EnumActionResult.SUCCESS;
  }
}
