package net.macmv.rgen.block;

import net.macmv.rgen.tab.RCreativeTabs;
import net.macmv.rgen.block.RGenLogBlockOne.LogType;
import net.minecraft.block.BlockLog;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;

public class RGenLogBlockTwo extends BlockLog {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, (type) -> type == LogType.MANGROVE || type == LogType.DEAD);

  public RGenLogBlockTwo() {
    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.MANGROVE).withProperty(LOG_AXIS, EnumAxis.Y));
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
      case 0:
        state = state.withProperty(LOG_AXIS, EnumAxis.Y);
        break;
      case 4:
        state = state.withProperty(LOG_AXIS, EnumAxis.X);
        break;
      case 8:
        state = state.withProperty(LOG_AXIS, EnumAxis.Z);
        break;
      default:
        state = state.withProperty(LOG_AXIS, EnumAxis.NONE);
    }

    return state;
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int i = 0;
    i = i | state.getValue(VARIANT).meta - 4;

    switch (state.getValue(LOG_AXIS)) {
      case X:
        i |= 4;
        break;
      case Z:
        i |= 8;
        break;
      case NONE:
        i |= 12;
    }

    return i;
  }
}

