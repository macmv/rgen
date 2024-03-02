package net.macmv.rgen.block;

import net.minecraft.block.BlockLog;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.IStringSerializable;

public class RGenLogBlockOne extends BlockLog {
  public static final PropertyEnum<EnumType> VARIANT = PropertyEnum.create("variant", EnumType.class);

  public RGenLogBlockOne() {
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, EnumType.CEDAR).withProperty(LOG_AXIS, EnumAxis.Y));
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, LOG_AXIS);
  }

  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(VARIANT, EnumType.fromMeta(meta & 3));

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

  public static enum EnumType implements IStringSerializable {
    CEDAR(0, "cedar", MapColor.WOOD),
    FIR(1, "fir", MapColor.OBSIDIAN),
    SAKURA(2, "sakura", MapColor.SAND),
    DEAD(3, "dead", MapColor.SAND);

    public final int meta;
    public final String name;
    public final MapColor mapColor;

    EnumType(int meta, String name, MapColor mapColor) {
      this.meta = meta;
      this.name = name;
      this.mapColor = mapColor;
    }

    @Override
    public String getName() {
      return name;
    }

    public static EnumType fromMeta(int meta) {
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

