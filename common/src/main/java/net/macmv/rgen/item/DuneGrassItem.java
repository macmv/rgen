package net.macmv.rgen.item;

import net.macmv.rgen.block.DuneGrassBlock;
import net.minecraft.block.Block;
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

public class DuneGrassItem extends ItemBlock {
  private final DuneGrassBlock duneGrass;

  public DuneGrassItem(DuneGrassBlock block) {
    super(block);
    this.duneGrass = block;
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
    if (!player.canPlayerEdit(placePos, facing, stack) || !duneGrass.canPlaceBlockAt(world, placePos)) {
      return EnumActionResult.FAIL;
    }

    // Choose the visual form per placement, while keeping the inventory item singular.
    IBlockState state = duneGrass.getDefaultState().withProperty(
        DuneGrassBlock.VARIANT, DuneGrassBlock.Variant.byMetadata(world.rand.nextInt(3)));
    if (!world.isRemote) {
      world.setBlockState(placePos, state, 3);
      Block block = duneGrass;
      SoundType sound = block.getSoundType(state, world, placePos, player);
      world.playSound(player, placePos, sound.getPlaceSound(), SoundCategory.BLOCKS,
          (sound.getVolume() + 1.0F) / 2.0F, sound.getPitch() * 0.8F);
      if (!player.capabilities.isCreativeMode) stack.shrink(1);
    }

    return EnumActionResult.SUCCESS;
  }
}
