package net.macmv.rgen.item;

import net.macmv.rgen.block.DoubleTallDriedGrassBlock;
import net.macmv.rgen.block.DriedGrassBlock;
import net.macmv.rgen.block.RBlocks;
import net.minecraft.block.Block;
import net.minecraft.block.BlockDoublePlant;
import net.minecraft.block.SoundType;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.item.ItemBlock;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class DriedGrassItem extends ItemBlock {
  public DriedGrassItem(Block block) {
    super(block);
  }

  @Override
  public EnumActionResult onItemUse(EntityPlayer player, World world, BlockPos clickedPos, EnumHand hand,
                                    EnumFacing facing, float hitX, float hitY, float hitZ) {
    ItemStack stack = player.getHeldItem(hand);
    if (stack.isEmpty()) return EnumActionResult.FAIL;

    BlockPos placePos = clickedPos;
    if (!world.getBlockState(placePos).getBlock().isReplaceable(world, placePos)) {
      placePos = placePos.offset(facing);
    }

    boolean doubleTall = world.rand.nextBoolean();
    BlockPos upperPos = placePos.up();
    if (!player.canPlayerEdit(placePos, facing, stack)
        || (doubleTall && !player.canPlayerEdit(upperPos, facing, stack))) {
      return EnumActionResult.FAIL;
    }

    IBlockState targetState = world.getBlockState(placePos);
    if (!targetState.getBlock().isReplaceable(world, placePos)) return EnumActionResult.FAIL;

    IBlockState placedState;
    if (doubleTall) {
      IBlockState upperState = world.getBlockState(upperPos);
      if (!upperState.getBlock().isReplaceable(world, upperPos)
          || !RBlocks.DOUBLE_TALL_DRIED_GRASS.canPlaceBlockAt(world, placePos)) {
        return EnumActionResult.FAIL;
      }

      DoubleTallDriedGrassBlock.Variant variant = DoubleTallDriedGrassBlock.Variant.byMetadata(world.rand.nextInt(6));
      placedState = RBlocks.DOUBLE_TALL_DRIED_GRASS.getDefaultState()
          .withProperty(DoubleTallDriedGrassBlock.VARIANT, variant)
          .withProperty(DoubleTallDriedGrassBlock.HALF, BlockDoublePlant.EnumBlockHalf.LOWER);
      IBlockState topState = placedState.withProperty(DoubleTallDriedGrassBlock.HALF, BlockDoublePlant.EnumBlockHalf.UPPER);

      if (!world.isRemote) {
        world.setBlockState(placePos, placedState, 3);
        world.setBlockState(upperPos, topState, 3);
        playPlaceSound(player, world, placePos, RBlocks.DOUBLE_TALL_DRIED_GRASS);
        if (!player.capabilities.isCreativeMode) stack.shrink(1);
      }
    } else {
      DriedGrassBlock.Variant variant = DriedGrassBlock.Variant.byMetadata(world.rand.nextInt(9));
      placedState = RBlocks.DRIED_GRASS.getDefaultState().withProperty(DriedGrassBlock.VARIANT, variant);
      if (!RBlocks.DRIED_GRASS.canPlaceBlockAt(world, placePos)) return EnumActionResult.FAIL;

      if (!world.isRemote) {
        world.setBlockState(placePos, placedState, 3);
        playPlaceSound(player, world, placePos, RBlocks.DRIED_GRASS);
        if (!player.capabilities.isCreativeMode) stack.shrink(1);
      }
    }

    return EnumActionResult.SUCCESS;
  }

  private static void playPlaceSound(EntityPlayer player, World world, BlockPos pos, Block block) {
    SoundType sound = block.getSoundType(block.getDefaultState(), world, pos, player);
    world.playSound(player, pos, sound.getPlaceSound(), SoundCategory.BLOCKS,
        (sound.getVolume() + 1.0F) / 2.0F, sound.getPitch() * 0.8F);
  }
}
