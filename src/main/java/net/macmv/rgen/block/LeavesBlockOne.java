package net.macmv.rgen.block;

import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Items;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.stats.StatList;
import net.minecraft.tileentity.TileEntity;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

import javax.annotation.Nullable;

public class LeavesBlockOne extends LeavesBlock {
  public static final PropertyEnum<LogBlockOne.LogType> VARIANT = PropertyEnum.create("variant", LogBlockOne.LogType.class, ty -> ty.meta < 4);

  public LeavesBlockOne() {
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogBlockOne.LogType.FIR).withProperty(CHECK_DECAY, true).withProperty(DECAYABLE, true));
  }

  // TODO: Override
  @Override
  protected int getSaplingDropChance(IBlockState state) {
    return super.getSaplingDropChance(state);
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this, 1, LogBlockOne.LogType.FIR.meta));
    items.add(new ItemStack(this, 1, LogBlockOne.LogType.PALM.meta));
    items.add(new ItemStack(this, 1, LogBlockOne.LogType.SAKURA.meta));
    items.add(new ItemStack(this, 1, LogBlockOne.LogType.CEDAR.meta));
  }

  @Override
  protected ItemStack getSilkTouchDrop(IBlockState state) {
    return new ItemStack(Item.getItemFromBlock(this), 1, state.getValue(VARIANT).meta);
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    LogBlockOne.LogType variant = LogBlockOne.LogType.fromMeta(meta & 3);

    return this.getDefaultState().withProperty(VARIANT, variant).withProperty(DECAYABLE, (meta & 4) == 0).withProperty(CHECK_DECAY, (meta & 8) > 0);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int i = 0;
    i = i | state.getValue(VARIANT).meta;

    if (!state.getValue(DECAYABLE)) {
      i |= 4;
    }

    if (state.getValue(CHECK_DECAY)) {
      i |= 8;
    }

    return i;
  }

  @Override
  public LogBlockOne.LogType getLogType(IBlockState state) {
    return state.getValue(VARIANT);
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, CHECK_DECAY, DECAYABLE);
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta;
  }

  @Override
  public void harvestBlock(World worldIn, EntityPlayer player, BlockPos pos, IBlockState state, @Nullable TileEntity te, ItemStack stack) {
    if (!worldIn.isRemote && stack.getItem() == Items.SHEARS) {
      player.addStat(StatList.getBlockStats(this));
    } else {
      super.harvestBlock(worldIn, player, pos, state, te, stack);
    }
  }

  @Override
  public NonNullList<ItemStack> onSheared(ItemStack item, net.minecraft.world.IBlockAccess world, BlockPos pos, int fortune) {
    return NonNullList.withSize(1, new ItemStack(this, 1, world.getBlockState(pos).getValue(VARIANT).meta));
  }
}
