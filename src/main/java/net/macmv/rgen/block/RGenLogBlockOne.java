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
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, (type) -> type.meta <= LogType.CEDAR.meta);

  public RGenLogBlockOne() {
    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.FIR).withProperty(LOG_AXIS, EnumAxis.Y));
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, LOG_AXIS);
  }

  @Override
  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    // TODO: Add these back
    items.add(new ItemStack(this, 1, LogType.FIR.meta));
    items.add(new ItemStack(this, 1, LogType.PALM.meta));
    items.add(new ItemStack(this, 1, LogType.SAKURA.meta));
    items.add(new ItemStack(this, 1, LogType.CEDAR.meta));
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta;
  }

  @Override
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

  @Override
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
    FIR(0, "fir", MapColor.OBSIDIAN),
    PALM(1, "palm", MapColor.SAND),
    SAKURA(2, "sakura", MapColor.PINK),
    CEDAR(3, "cedar", MapColor.SAND),
    MANGROVE(4, "mangrove", MapColor.RED),
    LAVENDER(5, "lavender", MapColor.PURPLE),
    SEASONAL(6, "seasonal", MapColor.ORANGE_STAINED_HARDENED_CLAY),
    DEAD(7, "dead", MapColor.BROWN);

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
          return FIR;
        case 1:
          return PALM;
        case 2:
          return SAKURA;
        case 3:
          return CEDAR;
        case 4:
          return MANGROVE;
        case 5:
          return LAVENDER;
        case 6:
          return SEASONAL;
        default:
          return DEAD;
      }
    }
  }
}

