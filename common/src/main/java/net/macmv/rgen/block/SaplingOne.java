package net.macmv.rgen.block;

import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;

public class SaplingOne extends Sapling {
  private static final int OFFSET = 0;
  public static final PropertyEnum<LogType> TYPE = PropertyEnum.create("type", LogType.class, ty -> ty.meta >= OFFSET && ty.meta < OFFSET + 8);

  public SaplingOne(BlockSettings settings) {
    this.setDefaultState(this.blockState.getBaseState().withProperty(TYPE, LogType.FIR).withProperty(STAGE, 0));
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(TYPE).meta - OFFSET;
  }

  @Override
  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    for (LogType ty : LogType.values()) {
      if (ty.meta >= OFFSET && ty.meta < OFFSET + 8) {
        items.add(new ItemStack(this, 1, ty.meta));
      }
    }
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(TYPE, LogType.fromMeta(meta & 7 + OFFSET)).withProperty(STAGE, (meta & 8) >> 3);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return ((state.getValue(TYPE)).meta - OFFSET) | (state.getValue(STAGE) << 3);
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, TYPE, STAGE);
  }
}
