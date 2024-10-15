package net.macmv.rgen.block;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.BlockLog;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;

public class LogBlockTwo extends BlockLog {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, (type) -> type == LogType.MANGROVE || type == LogType.DEAD);

  public LogBlockTwo() {
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.MANGROVE).withProperty(LOG_AXIS, EnumAxis.Y));
    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, LOG_AXIS);
  }

  @Override
  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    // TODO: Add these back
    items.add(new ItemStack(this, 1, LogType.MANGROVE.meta - 4));
    items.add(new ItemStack(this, 1, LogType.DEAD.meta - 4));
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta - 4;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(VARIANT, LogType.fromMeta((meta & 3) + 4));

    switch (meta & 12) {
      case 0: return state.withProperty(LOG_AXIS, EnumAxis.Y);
      case 4: return state.withProperty(LOG_AXIS, EnumAxis.X);
      case 8: return state.withProperty(LOG_AXIS, EnumAxis.Z);
      default: return state.withProperty(LOG_AXIS, EnumAxis.NONE);
    }
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int i = state.getValue(VARIANT).meta - 4;

    switch (state.getValue(LOG_AXIS)) {
      case X: return i | 4;
      case Z: return i | 8;
      case NONE: return i | 12;
      default: return i;
    }
  }
}

