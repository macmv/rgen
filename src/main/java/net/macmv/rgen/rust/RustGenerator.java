package net.macmv.rgen.rust;

import net.minecraft.block.state.IBlockState;
import net.minecraft.util.ObjectIntIdentityMap;
import net.minecraftforge.registries.GameData;

public class RustGenerator {
  private static native void init_generator(ObjectIntIdentityMap<IBlockState> block_ids);
  private static native void build_chunk(char[] data, int x, int z);

  static {
    System.loadLibrary("rgen");
  }

  public static void init() {
    init_generator(GameData.getBlockStateIDMap());
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
}
