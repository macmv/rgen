package net.macmv.rgen.item;

import net.macmv.rgen.block.GoldenGrassBlock;
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

public class GoldenGrassItem extends ItemBlock {
  public GoldenGrassItem(Block block) {
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
    BlockPos upperPos = placePos.up();
    boolean tall = world.rand.nextBoolean();
    if (!player.canPlayerEdit(placePos, facing, stack)
        || (tall && !player.canPlayerEdit(upperPos, facing, stack))) {
      return EnumActionResult.FAIL;
    }

    IBlockState atPlace = world.getBlockState(placePos);
    if (!atPlace.getBlock().isReplaceable(world, placePos)) return EnumActionResult.FAIL;

    if (tall) {
      IBlockState atUpper = world.getBlockState(upperPos);
      if (!atUpper.getBlock().isReplaceable(world, upperPos)) return EnumActionResult.FAIL;

      GoldenGrassBlock.Variant variant = GoldenGrassBlock.Variant.byMetadata(3 + world.rand.nextInt(2));
      IBlockState lower = RBlocks.GOLDEN_GRASS.getDefaultState()
          .withProperty(GoldenGrassBlock.VARIANT, variant)
          .withProperty(GoldenGrassBlock.HALF, BlockDoublePlant.EnumBlockHalf.LOWER);
      IBlockState upper = lower.withProperty(GoldenGrassBlock.HALF, BlockDoublePlant.EnumBlockHalf.UPPER);
      if (!((GoldenGrassBlock) RBlocks.GOLDEN_GRASS).canBlockStay(world, placePos, lower)) return EnumActionResult.FAIL;

      if (!world.isRemote) {
        world.setBlockState(placePos, lower, 3);
        world.setBlockState(upperPos, upper, 3);
        playPlaceSound(player, world, placePos);
        if (!player.capabilities.isCreativeMode) stack.shrink(1);
      }
    } else {
      GoldenGrassBlock.Variant variant = GoldenGrassBlock.Variant.byMetadata(world.rand.nextInt(3));
      IBlockState state = RBlocks.GOLDEN_GRASS.getDefaultState()
          .withProperty(GoldenGrassBlock.VARIANT, variant)
          .withProperty(GoldenGrassBlock.HALF, BlockDoublePlant.EnumBlockHalf.LOWER);
      if (!((GoldenGrassBlock) RBlocks.GOLDEN_GRASS).canBlockStay(world, placePos, state)) return EnumActionResult.FAIL;

      if (!world.isRemote) {
        world.setBlockState(placePos, state, 3);
        playPlaceSound(player, world, placePos);
        if (!player.capabilities.isCreativeMode) stack.shrink(1);
      }
    }

    return EnumActionResult.SUCCESS;
  }

  private static void playPlaceSound(EntityPlayer player, World world, BlockPos pos) {
    SoundType sound = RBlocks.GOLDEN_GRASS.getSoundType(RBlocks.GOLDEN_GRASS.getDefaultState(), world, pos, player);
    world.playSound(player, pos, sound.getPlaceSound(), SoundCategory.BLOCKS,
        (sound.getVolume() + 1.0F) / 2.0F, sound.getPitch() * 0.8F);
  }
}
