package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.util.ResourceLocation;
import net.minecraft.world.biome.Biome;
import net.minecraftforge.registries.GameData;

public class RustGenerator {
  private static native void init_generator(long seed);
  private static native void build_chunk(char[] data, int x, int z);
  private static native void build_biomes(byte[] data, int x, int z);

  // Helpers for the rust code.

  private static int block_name_to_id(String name) {
    Block block = Block.getBlockFromName(name);
    if (block == null) {
      return 0;
    }

    return GameData.getBlockStateIDMap().get(block.getDefaultState());
  }

  private static int biome_name_to_id(String name) {
    Biome biome = Biome.REGISTRY.getObject(new ResourceLocation(name));
    if (biome == null) {
      return 0;
    }

    return Biome.getIdForBiome(biome);
  }

  private static boolean active = false;

  public static void init(long seed) {
    if (!active) {
      System.loadLibrary("rgen_jni");
    }
    active = true;
    init_generator(seed);
  }

  public static boolean isActive() {
    return active;
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
  public static void make_biomes(byte[] biomes, int x, int z) {
    build_biomes(biomes, x, z);
  }
}
