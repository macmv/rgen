package net.macmv.rgen.block;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.BlockLog;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;

public class RGenLogBlockOne extends BlockLog {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class);

  public RGenLogBlockOne() {
    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.CEDAR).withProperty(LOG_AXIS, EnumAxis.Y));
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, LOG_AXIS);
  }

  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this, 1, LogType.CEDAR.meta));
    items.add(new ItemStack(this, 1, LogType.FIR.meta));
    items.add(new ItemStack(this, 1, LogType.SAKURA.meta));
    items.add(new ItemStack(this, 1, LogType.DEAD.meta));
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta;
  }

  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(VARIANT, LogType.fromMeta(meta & 3));

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

  public int getMetaFromState(IBlockState state) {
    int i = 0;
    i = i | state.getValue(VARIANT).meta;

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

  public static enum LogType implements IStringSerializable {
    CEDAR(0, "cedar", MapColor.WOOD),
    FIR(1, "fir", MapColor.OBSIDIAN),
    SAKURA(2, "sakura", MapColor.SAND),
    DEAD(3, "dead", MapColor.SAND);

    public final int meta;
    public final String name;
    public final MapColor mapColor;

    LogType(int meta, String name, MapColor mapColor) {
      this.meta = meta;
      this.name = name;
      this.mapColor = mapColor;
    }

    @Override
    public String getName() {
      return name;
    }

    public static LogType fromMeta(int meta) {
      switch (meta) {
        case 0:
          return CEDAR;
        case 1:
          return FIR;
        case 2:
          return SAKURA;
        default:
          return DEAD;
      }
    }
  }
}

