package net.macmv.rgen.world;

import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EnumCreatureType;
import net.minecraft.init.Blocks;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.world.chunk.ChunkPrimer;
import net.minecraft.world.gen.IChunkGenerator;

import javax.annotation.Nullable;
import java.util.Collections;
import java.util.List;

public class RGenChunkGenerator implements IChunkGenerator {
  private final World world;

  public RGenChunkGenerator(World world) {
    this.world = world;
  }

  @Override
  public Chunk generateChunk(int x, int z) {
    ChunkPrimer primer = new ChunkPrimer();

    IBlockState[] blockIds = new IBlockState[]{Blocks.STONE.getDefaultState(), Blocks.GRASS.getDefaultState()};

    for (int i = 0; i < blockIds.length; ++i) {
      IBlockState iblockstate = blockIds[i];

      if (iblockstate != null) {
        for (int j = 0; j < 16; ++j) {
          for (int k = 0; k < 16; ++k) {
            primer.setBlockState(j, i, k, iblockstate);
          }
        }
      }
    }

    Chunk chunk = new Chunk(this.world, primer, x, z);
    Biome[] biomes = this.world.getBiomeProvider().getBiomes(null, x * 16, z * 16, 16, 16);

    byte[] bytes = chunk.getBiomeArray();
    for (int i = 0; i < bytes.length; ++i) {
      bytes[i] = (byte) Biome.getIdForBiome(biomes[i]);
    }

    chunk.generateSkylightMap();
    return chunk;
  }

  @Override
  public void populate(int x, int z) {

  }

  @Override
  public boolean generateStructures(Chunk chunkIn, int x, int z) {
    return false;
  }

  @Override
  public List<Biome.SpawnListEntry> getPossibleCreatures(EnumCreatureType creatureType, BlockPos pos) {
    return Collections.emptyList();
  }

  @Nullable
  @Override
  public BlockPos getNearestStructurePos(World worldIn, String structureName, BlockPos position, boolean findUnexplored) {
    return BlockPos.ORIGIN;
  }

  @Override
  public void recreateStructures(Chunk chunkIn, int x, int z) {

  }

  @Override
  public boolean isInsideStructure(World worldIn, String structureName, BlockPos pos) {
    return false;
  }
}
