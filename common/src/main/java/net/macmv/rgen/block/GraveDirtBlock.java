package net.macmv.rgen.block;

import net.minecraft.block.BlockDirt;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.init.SoundEvents;
import net.minecraft.item.ItemHoe;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.NonNullList;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class GraveDirtBlock extends BlockDirt {
  public GraveDirtBlock(BlockSettings settings) {
    super();
    this.setHardness(settings.hardness);
    this.setResistance(settings.resistance);
    this.setSoundType(settings.soundType);
    this.setCreativeTab(settings.creativeTab);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, DirtType.COARSE_DIRT));
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState();
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return DirtType.COARSE_DIRT.getMetadata();
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this, 1, DirtType.COARSE_DIRT.getMetadata()));
  }

  @Override
  public boolean onBlockActivated(World world, BlockPos pos, IBlockState state, EntityPlayer player,
                                   EnumHand hand, EnumFacing side, float hitX, float hitY, float hitZ) {
    ItemStack held = player.getHeldItem(hand);
    if (side == EnumFacing.DOWN || held.isEmpty() || !(held.getItem() instanceof ItemHoe)
        || !world.isAirBlock(pos.up())
        || !player.canPlayerEdit(pos, side, held)) {
      return false;
    }

    if (!world.isRemote) {
      world.setBlockState(pos, Blocks.DIRT.getDefaultState().withProperty(VARIANT, DirtType.DIRT), 11);
      held.damageItem(1, player);
      world.playSound(null, pos, SoundEvents.ITEM_HOE_TILL, SoundCategory.BLOCKS, 1.0F, 1.0F);
    }
    return true;
  }
}
