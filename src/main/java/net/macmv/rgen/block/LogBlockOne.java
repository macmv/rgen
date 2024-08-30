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

public class LogBlockOne extends BlockLog {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, (type) -> type.meta <= LogType.CEDAR.meta);

  public LogBlockOne() {
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.FIR).withProperty(LOG_AXIS, EnumAxis.Y));
    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
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
      case 0: return state.withProperty(LOG_AXIS, EnumAxis.Y);
      case 4: return state.withProperty(LOG_AXIS, EnumAxis.X);
      case 8: return state.withProperty(LOG_AXIS, EnumAxis.Z);
      default: return state.withProperty(LOG_AXIS, EnumAxis.NONE);
    }
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int i = state.getValue(VARIANT).meta;

    switch (state.getValue(LOG_AXIS)) {
      case X: return i | 4;
      case Z: return i | 8;
      case NONE: return i | 12;
      default: return i;
    }
  }

  public static enum LogType implements IStringSerializable {
    FIR(0, "fir", MapColor.OBSIDIAN),
    PALM(1, "palm", MapColor.SAND),
    SAKURA(2, "sakura", MapColor.PINK),
    CEDAR(3, "cedar", MapColor.SAND),
    MANGROVE(4, "mangrove", MapColor.RED),
    LAVENDER(5, "lavender", MapColor.PURPLE),
    SEASONAL(6, "seasonal", MapColor.ORANGE_STAINED_HARDENED_CLAY),
    DEAD(7, "dead", MapColor.BROWN),
    ASPEN(8, "aspen", MapColor.SAND);

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
        case 0: return FIR;
        case 1: return PALM;
        case 2: return SAKURA;
        case 3: return CEDAR;
        case 4: return MANGROVE;
        case 5: return LAVENDER;
        case 6: return SEASONAL;
        case 7: return DEAD;
        default: return ASPEN;
      }
    }
  }
}

