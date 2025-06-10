package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.block.properties.IProperty;
import net.minecraft.block.properties.PropertyBool;
import net.minecraft.block.properties.PropertyInteger;

import java.util.Collection;

// A java representation of `rgen_base::PropType`.
class PropType {
  public String name;
  public int kind; // 0 -> bool, 1 -> int, 2 -> enum.

  // For ints.
  public int min;
  public int max;

  // For enums.
  public String[] variants;

  public static PropType[] lookup(Block block) {
    Collection<IProperty<?>> props = block.getBlockState().getProperties();
    PropType[] res = new PropType[props.size()];
    int i = 0;
    for (IProperty<?> prop : props) {
      PropType p = new PropType();
      p.name = prop.getName();
      Class<?> type = prop.getValueClass();

      if (prop instanceof PropertyBool) {
        p.kind = 0;
      } else if (prop instanceof PropertyInteger) {
        p.kind = 1;
        // Integer props are always created with a contiguous range, but they store all those values
        // in a set. So just recover the min/max here.
        p.min = ((PropertyInteger) prop).getAllowedValues().stream().min(Integer::compare).orElse(0);
        p.max = ((PropertyInteger) prop).getAllowedValues().stream().max(Integer::compare).orElse(0);
      } else {
        p.kind = 2;
        p.variants = new String[prop.getAllowedValues().size()];
        int j = 0;
        for (Object o : prop.getAllowedValues()) {
          p.variants[j++] = o.toString();
        }
      }

      res[i++] = p;
    }

    return res;
  }
}
