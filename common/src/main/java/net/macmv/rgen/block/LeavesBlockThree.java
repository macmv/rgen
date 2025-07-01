package net.macmv.rgen.block;

import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Items;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.stats.StatList;
import net.minecraft.tileentity.TileEntity;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

import javax.annotation.Nonnull;
import javax.annotation.Nullable;
import java.util.Collections;
import java.util.List;


public class LeavesBlockThree extends LeavesBlock {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, ty -> ty.meta >= 8 && ty.meta < 9);

  public LeavesBlockThree(BlockSettings settings) {
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.ASPEN));
  }

  @Override
  public LogType getLogType(IBlockState state) {
    return state.getValue(VARIANT);
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, CHECK_DECAY, DECAYABLE);
  }

  @Nonnull
  public List<ItemStack> onSheared(@Nonnull ItemStack item, IBlockAccess world, BlockPos pos, int fortune) {
    return Collections.emptyList();
  }

  public int getMetaFromState(IBlockState state) {
    int i = 0;
    i = i | state.getValue(VARIANT).meta - 8;

    if (!state.getValue(DECAYABLE)) {
      i |= 4;
    }

    if (state.getValue(CHECK_DECAY)) {
      i |= 8;
    }

    return i;
  }


  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta - 8;
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
  protected ItemStack getSilkTouchDrop(IBlockState state) {
    return new ItemStack(Item.getItemFromBlock(this), 1, state.getValue(VARIANT).meta - 8);
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    LogType variant = LogType.fromMeta((meta & 3) + 8);

    return this.getDefaultState().withProperty(VARIANT, variant).withProperty(DECAYABLE, (meta & 4) == 0).withProperty(CHECK_DECAY, (meta & 8) > 0);
  }

}

