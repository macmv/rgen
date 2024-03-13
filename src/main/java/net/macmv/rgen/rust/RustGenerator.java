package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.util.ResourceLocation;
import net.minecraft.world.biome.Biome;
import net.minecraftforge.registries.GameData;

public class RustGenerator {
  private static native void init_generator(long seed);
  private static native void build_chunk(char[] data, int x, int z);
  private static native void build_biomes(byte[] data, int x, int z);
  private static native void build_biomes_region(byte[] data, int blockX, int blockZ, int width, int height);
  private static native String[] debug_info(int x, int y, int z);
  private static native String get_biome_at(int x, int y, int z);

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

  public static String[] getDebugInfo(int x, int y, int z) {
    return debug_info(x, y, z);
  }
  public static String getBiomeAt(int x, int y, int z) {
    return get_biome_at(x, y, z);
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
  public static void make_biomes(byte[] biomes, int x, int z) {
    build_biomes(biomes, x, z);
  }
  public static void make_biomes_region(byte[] biomes, int blockX, int blockZ, int width, int height) {
    build_biomes_region(biomes, blockX, blockZ, width, height);
  }
}
