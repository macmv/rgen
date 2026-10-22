package net.macmv.rgen.block;

import net.minecraft.block.material.MapColor;
import net.minecraft.util.IStringSerializable;

public enum LogType implements IStringSerializable {
  FIR(0, "fir", MapColor.OBSIDIAN),
  PALM(1, "palm", MapColor.SAND),
  CHERRY(2, "cherry", MapColor.PINK),
  CEDAR(3, "cedar", MapColor.SAND),
  MANGROVE(4, "mangrove", MapColor.RED),
  LAVENDER_TREE(5, "lavender_tree", MapColor.PURPLE),
  SEASONAL(6, "seasonal", MapColor.ORANGE_STAINED_HARDENED_CLAY),
  DEAD(7, "dead", MapColor.BROWN),
  BURNED_TREE(12, "burned_tree", MapColor.BLACK),
  ASPEN(8, "aspen", MapColor.SAND),
  RED_SEASONAL_TREE(9, "red_seasonal_tree", MapColor.RED),
  ORANGE_SEASONAL_TREE(10, "orange_seasonal_tree", MapColor.ORANGE_STAINED_HARDENED_CLAY),
  YELLOW_SEASONAL_TREE(11, "yellow_seasonal_tree", MapColor.SAND);


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

  public int groupTwoMeta() {
    switch (this) {
      case MANGROVE: return 0;
      case LAVENDER_TREE: return 1;
      case BURNED_TREE: return 2;
      case DEAD: return 3;
      default: throw new IllegalArgumentException("Not a group 2 tree: " + this);
    }
  }

  public static LogType groupTwoFromMeta(int meta) {
    switch (meta & 3) {
      case 1: return LAVENDER_TREE;
      case 2: return BURNED_TREE;
      case 3: return DEAD;
      default: return MANGROVE;
    }
  }

  public static LogType fromMeta(int meta) {
    switch (meta) {
      case 0: return FIR;
      case 1: return PALM;
      case 2: return CHERRY;
      case 3: return CEDAR;
      case 4: return MANGROVE;
      case 5: return LAVENDER_TREE;
      case 6: return SEASONAL;
      case 7: return DEAD;
      case 8: return ASPEN;
      case 9: return RED_SEASONAL_TREE;
      case 10: return ORANGE_SEASONAL_TREE;
      case 11: return YELLOW_SEASONAL_TREE;
      case 12: return BURNED_TREE;
      default: return ASPEN;
    }
  }
}
