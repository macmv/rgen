package net.macmv.rgen.item;

import net.macmv.rgen.compat.ItemSettings;
import net.minecraft.item.Item;

// TODO: Port the debug stick, the 1.12 one cycles block metadata.
public class DebugStickItem extends Item {
  public DebugStickItem() {
    super(ItemSettings.create());
  }
}
