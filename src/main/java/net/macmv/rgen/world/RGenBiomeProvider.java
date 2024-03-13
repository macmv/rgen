package net.macmv.rgen.world;

import net.macmv.rgen.rust.RustGenerator;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.biome.BiomeProvider;
import net.minecraft.world.gen.layer.IntCache;

import java.util.List;

public class RGenBiomeProvider extends BiomeProvider {
  public RGenBiomeProvider(long seed) {
  }

  @Override
  public boolean areBiomesViable(int x, int z, int radius, List<Biome> allowed) {
    System.out.println("checking if the biomes are viable at " + x + ", " + z + " with radius " + radius);

    IntCache.resetIntCache();
    int minX = x - radius;
    int minZ = z - radius;
    int maxX = x + radius;
    int maxZ = z + radius;
    int width = maxX - minX + 1;
    int height = maxZ - minZ + 1;

    byte[] biomes = new byte[width * height];
    RustGenerator.make_biomes_region(biomes, minX, minZ, width, height);

    for (int i = 0; i < width * height; i++) {
      Biome biome = Biome.getBiome(biomes[i]);

      if (!allowed.contains(biome)) {
        return false;
      }
    }

    return true;
  }

}
