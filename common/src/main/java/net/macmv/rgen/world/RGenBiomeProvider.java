package net.macmv.rgen.world;

import net.macmv.rgen.rust.RustGenerator;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.biome.BiomeProvider;
import net.minecraft.world.gen.layer.IntCache;

import javax.annotation.Nullable;
import java.util.List;
import java.util.Random;

// TODO: Override all the other functions in this class.
public class RGenBiomeProvider extends BiomeProvider {
  public RGenBiomeProvider(long seed) {
  }
  @Override
  public Biome getBiome(BlockPos pos) {
    return super.getBiome(pos);
  }

  public Biome getBiome(BlockPos pos, Biome defaultBiome) {
    byte id = RustGenerator.biome_id_at(pos.getX(), pos.getZ());
    return Biome.getBiome(id);
  }

  @Nullable
  @Override
  public BlockPos findBiomePosition(int x, int z, int range, List<Biome> search, Random random) {
    long start = System.nanoTime();

    IntCache.resetIntCache();
    int minX = (x - range) / 4;
    int minZ = (z - range) / 4;
    int maxX = (x + range) / 4;
    int maxZ = (z + range) / 4;
    int width = maxX - minX + 1;
    int height = maxZ - minZ + 1;

    byte[] biomes = new byte[width * height];
    RustGenerator.make_biomes_region_4x4(biomes, minX, minZ, width, height);

    BlockPos blockpos = null;
    int foundIndex = 0;

    for (int i = 0; i < width * height; ++i) {
      int blockX = minX + i % width;
      int blockZ = minZ + i / width;
      Biome biome = Biome.getBiome(biomes[i]);

      if (search.contains(biome) && (blockpos == null || random.nextInt(foundIndex + 1) == 0)) {
        blockpos = new BlockPos(blockX, 0, blockZ);
        foundIndex++;
      }
    }

    long end = System.nanoTime();
    System.out.println("findBiomePosition took " + (end - start) / 1_000_000f + "ms");

    return blockpos;
  }

  @Override
  public boolean areBiomesViable(int x, int z, int radius, List<Biome> allowed) {
    IntCache.resetIntCache();
    int minX = (x - radius) / 4;
    int minZ = (z - radius) / 4;
    int maxX = (x + radius) / 4;
    int maxZ = (z + radius) / 4;
    int width = maxX - minX + 1;
    int height = maxZ - minZ + 1;

    byte[] biomes = new byte[width * height];
    RustGenerator.make_biomes_region_4x4(biomes, minX, minZ, width, height);

    for (int i = 0; i < width * height; i++) {
      Biome biome = Biome.getBiome(biomes[i]);

      if (!allowed.contains(biome)) {
        return false;
      }
    }

    return true;
  }
}
