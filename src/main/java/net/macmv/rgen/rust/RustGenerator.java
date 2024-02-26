package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraftforge.registries.GameData;

public class RustGenerator {
  private static native void init_generator(long seed);
  private static native void build_chunk(char[] data, int x, int z);

  // Helpers for the rust code.

  private static int block_name_to_id(String name) {
    Block block = Block.getBlockFromName(name);
    if (block == null) {
      return 0;
    }

    return GameData.getBlockStateIDMap().get(block.getDefaultState());
  }

  static {
    System.loadLibrary("rgen_jni");
  }

  public static void init(long seed) {
    init_generator(seed);
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
}
