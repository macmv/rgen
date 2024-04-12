package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.client.Minecraft;
import net.minecraft.util.ResourceLocation;
import net.minecraft.util.text.TextComponentString;
import net.minecraft.util.text.TextFormatting;
import net.minecraft.world.biome.Biome;
import net.minecraftforge.registries.GameData;

public class RustGenerator {
  private static native void init_generator(long seed);
  private static native void init();
  private static native int reload_generator(long seed);
  private static native void build_chunk(char[] data, int x, int z);
  private static native void build_biomes(byte[] data, int x, int z);
  private static native void build_biomes_region(byte[] data, int cellX, int cellZ, int width, int height);
  private static native String[] debug_info(int x, int y, int z);
  private static native String get_biome_name_at(int x, int y, int z);
  private static native byte get_biome_at(int x, int z);

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

  private static void print_warnings(String name) {
    Minecraft.getMinecraft().player.sendMessage(new TextComponentString(name + "\n\n" + TextFormatting.YELLOW + "Reload successful."));
  }

  private static void print_errors(String name) {
    Minecraft.getMinecraft().player.sendMessage(new TextComponentString(name + "\n\n" + TextFormatting.RED + "Failed to reload."));
  }

  private static boolean active = false;

  public static void init(long seed) {
    if (!active) {
      System.loadLibrary("rgen_jni");
      init();
    }
    active = true;
    init_generator(seed);
  }

  public static void reload(long seed) {
    System.out.println(seed);
    int res = reload_generator(seed);
    if (res == 0) {
      // TODO: Wipe out the world.
    }
  }

  public static boolean isActive() {
    return active;
  }

  public static String[] getDebugInfo(int x, int y, int z) {
    return debug_info(x, y, z);
  }
  public static String getBiomeAt(int x, int y, int z) {
    return get_biome_name_at(x, y, z);
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
  public static void make_biomes(byte[] biomes, int x, int z) {
    build_biomes(biomes, x, z);
  }
  public static void make_biomes_region_4x4(byte[] biomes, int cellX, int cellZ, int width, int height) {
    build_biomes_region(biomes, cellX, cellZ, width, height);
  }
  public static byte biome_id_at(int x, int z) {
    return get_biome_at(x, z);
  }
}
