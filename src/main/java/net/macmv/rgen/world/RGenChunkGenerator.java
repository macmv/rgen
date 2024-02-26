package net.macmv.rgen.world;

import net.macmv.rgen.rust.RustGenerator;
import net.minecraft.entity.EnumCreatureType;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.world.chunk.ChunkPrimer;
import net.minecraft.world.gen.IChunkGenerator;

import javax.annotation.Nullable;
import java.lang.reflect.Field;
import java.util.Collections;
import java.util.List;

public class RGenChunkGenerator implements IChunkGenerator {
  private final World world;

  public RGenChunkGenerator(World world) {
    this.world = world;

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
