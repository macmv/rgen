package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.block.properties.IProperty;
import net.minecraft.block.properties.PropertyBool;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;

public class PropMap {
  public PropValue[] values = new PropValue[0];

  public static PropMap[] lookup(Block block) {
    BlockStateContainer container = block.getBlockState();
    int prop_count = container.getProperties().size();

    PropMap[] res = new PropMap[16];
    for (int i = 0; i < 16; i++) {
      IBlockState state;
      try {
        state = block.getStateFromMeta(i);
      } catch (Exception e) {
        // Minecraft sometimes blows up when trying to get a state from a meta that doesn't exist.
        // So just store an empty prop map.
        res[i] = new PropMap();
        continue;
      }

      PropMap map = new PropMap();
      map.values = new PropValue[prop_count];

      int j = 0;
      for (IProperty<?> prop : container.getProperties()) {
        PropValue value = new PropValue();
        value.name = prop.getName();

        if (prop instanceof PropertyBool) {
          value.kind = 0;
          value.bool = (Boolean) state.getValue(prop);
        } else if (prop instanceof PropertyInteger) {
          value.kind = 1;
          value.integer = (Integer) state.getValue(prop);
        } else {
          value.kind = 2;
          value.str = state.getValue(prop).toString();
        }

        map.values[j++] = value;
      }

      res[i] = map;
    }

    return res;
  }
}
