package net.macmv.rgen.world;

import net.macmv.rgen.rust.RustGenerator;
import net.minecraft.block.BlockFalling;
import net.minecraft.entity.EnumCreatureType;
import net.minecraft.init.Biomes;
import net.minecraft.init.Blocks;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.ChunkPos;
import net.minecraft.world.World;
import net.minecraft.world.WorldEntitySpawner;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.world.chunk.ChunkPrimer;
import net.minecraft.world.gen.ChunkGeneratorOverworld;
import net.minecraft.world.gen.IChunkGenerator;
import net.minecraft.world.gen.feature.WorldGenDungeons;
import net.minecraft.world.gen.feature.WorldGenLakes;
import net.minecraftforge.event.ForgeEventFactory;
import net.minecraftforge.event.terraingen.PopulateChunkEvent;
import net.minecraftforge.event.terraingen.TerrainGen;

import javax.annotation.Nullable;
import java.lang.reflect.Field;
import java.util.Collections;
import java.util.List;
import java.util.Random;

public class RGenChunkGenerator extends ChunkGeneratorOverworld {
  private final World world;
  private final Random rand;
  private final VanillaDecorator vanillaDecorator = new VanillaDecorator();

  public RGenChunkGenerator(World world) {
    super(world, world.getSeed(), false, "");
    this.world = world;
    this.rand = new Random(world.getSeed());

    RustGenerator.init(world.getSeed());
  }

  @Override
  public Chunk generateChunk(int x, int z) {
    ChunkPrimer primer = new ChunkPrimer();

    build_rust_chunk(primer, x, z);

    Chunk chunk = new Chunk(this.world, primer, x, z);

    RustGenerator.make_biomes(chunk.getBiomeArray(), x, z);

    chunk.generateSkylightMap();
    return chunk;
  }

  private void build_rust_chunk(ChunkPrimer primer, int x, int z) {
    try {
      // FIXME: Use an access transformer instead.
      Field dataField;
      try {
        dataField = ChunkPrimer.class.getDeclaredField("field_177860_a");
      } catch (NoSuchFieldException e) {
        dataField = ChunkPrimer.class.getDeclaredField("data");
      }
      dataField.setAccessible(true);

      char[] data = (char[]) dataField.get(primer);
      RustGenerator.make_chunk(data, x, z);
    } catch (NoSuchFieldException | IllegalAccessException e) {
      throw new RuntimeException(e);
    }
  }

  @Override
  public List<Biome.SpawnListEntry> getPossibleCreatures(EnumCreatureType creatureType, BlockPos pos) {
    return Collections.emptyList();
  }

  @Override
  public void populate(int x, int z) {
    long startTime = System.nanoTime();

    BlockFalling.fallInstantly = true;
    int i = x * 16;
    int j = z * 16;
    BlockPos blockpos = new BlockPos(i, 0, j);
    Biome biome = this.world.getBiome(blockpos.add(16, 0, 16));
    this.rand.setSeed(this.world.getSeed());
    long k = this.rand.nextLong() / 2L * 2L + 1L;
    long l = this.rand.nextLong() / 2L * 2L + 1L;
    this.rand.setSeed((long) x * k + (long) z * l ^ this.world.getSeed());
    boolean flag = false;
    ChunkPos chunkpos = new ChunkPos(x, z);

    ForgeEventFactory.onChunkPopulate(true, this, this.world, this.rand, x, z, flag);

    this.vanillaDecorator.decorate(this.world, this.rand, chunkpos);

    if (TerrainGen.populate(this, this.world, this.rand, x, z, flag, PopulateChunkEvent.Populate.EventType.DUNGEON)) {
      // dungeonChance is 8
      for (int j2 = 0; j2 < 8; ++j2) {
        int i3 = this.rand.nextInt(16) + 8;
        int l3 = this.rand.nextInt(256);
        int l1 = this.rand.nextInt(16) + 8;
        (new WorldGenDungeons()).generate(this.world, this.rand, blockpos.add(i3, l3, l1));
      }
    }

    // biome.decorate(this.world, this.rand, new BlockPos(i, 0, j));

    if (TerrainGen.populate(this, this.world, this.rand, x, z, flag, PopulateChunkEvent.Populate.EventType.ANIMALS)) {
      WorldEntitySpawner.performWorldGenSpawning(this.world, biome, i + 8, j + 8, 16, 16, this.rand);
    }

    // TODO: Move this over to rust.
    /*
    blockpos = blockpos.add(8, 0, 8);
    if (TerrainGen.populate(this, this.world, this.rand, x, z, flag, PopulateChunkEvent.Populate.EventType.ICE)) {
      for (int k2 = 0; k2 < 16; ++k2) {
        for (int j3 = 0; j3 < 16; ++j3) {
          BlockPos blockpos1 = this.world.getPrecipitationHeight(blockpos.add(k2, 0, j3));
          BlockPos blockpos2 = blockpos1.down();

          if (this.world.canBlockFreezeWater(blockpos2)) {
            this.world.setBlockState(blockpos2, Blocks.ICE.getDefaultState(), 2);
          }

          if (this.world.canSnowAt(blockpos1, true)) {
            this.world.setBlockState(blockpos1, Blocks.SNOW_LAYER.getDefaultState(), 2);
          }
        }
      }
    }
     */

    ForgeEventFactory.onChunkPopulate(false, this, this.world, this.rand, x, z, flag);

    BlockFalling.fallInstantly = false;

    long endTime = System.nanoTime();
    long duration = (endTime - startTime);
    System.out.println("chunk took " + (float) duration / 1_000_000f + " millis");
  }
}
