package net.macmv.rgen.compat;

import net.minecraft.item.Item;

import java.util.ArrayList;
import java.util.List;

/**
 * A stand in for the 1.12 creative tabs. Items add themselves to a tab when they are registered,
 * and `RGen` turns these into real item groups.
 */
public final class CreativeTabs {
  public static final CreativeTabs BUILDING_BLOCKS = new CreativeTabs("building_blocks");
  public static final CreativeTabs DECORATIONS = new CreativeTabs("decorations");

  public final String name;
  public final List<Item> items = new ArrayList<>();

  public CreativeTabs(String name) {
    this.name = name;
  }
}
