package net.macmv.rgen.block;

import net.minecraft.block.material.MapColor;
import net.minecraft.util.IStringSerializable;

public enum LogType implements IStringSerializable {
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
