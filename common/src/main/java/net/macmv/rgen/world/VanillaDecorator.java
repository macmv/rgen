package net.macmv.rgen.world;

import net.minecraft.util.math.ChunkPos;
import net.minecraft.world.World;
import net.minecraft.world.chunk.ChunkPrimer;
import net.minecraft.world.gen.MapGenBase;
import net.minecraft.world.gen.MapGenCaves;
import net.minecraft.world.gen.MapGenRavine;
import net.minecraft.world.gen.structure.*;
import net.minecraftforge.event.terraingen.InitMapGenEvent;
import net.minecraftforge.event.terraingen.TerrainGen;

import java.util.Random;

// This is basically all the private fields from the overworld generator, like mineshafts and
// villages.
public class VanillaDecorator {
  private MapGenBase caveGenerator = new MapGenCaves();
  private MapGenStronghold strongholdGenerator = new MapGenStronghold();
  private MapGenVillage villageGenerator = new MapGenVillage();
  private MapGenMineshaft mineshaftGenerator = new MapGenMineshaft();
  private MapGenScatteredFeature scatteredFeatureGenerator = new MapGenScatteredFeature();
  private MapGenBase ravineGenerator = new MapGenRavine();
  private StructureOceanMonument oceanMonumentGenerator = new StructureOceanMonument();

  public VanillaDecorator() {
    caveGenerator = TerrainGen.getModdedMapGen(caveGenerator, InitMapGenEvent.EventType.CAVE);
    strongholdGenerator = (MapGenStronghold) TerrainGen.getModdedMapGen(strongholdGenerator, InitMapGenEvent.EventType.STRONGHOLD);
    villageGenerator = (MapGenVillage) TerrainGen.getModdedMapGen(villageGenerator, InitMapGenEvent.EventType.VILLAGE);
    mineshaftGenerator = (MapGenMineshaft) TerrainGen.getModdedMapGen(mineshaftGenerator, InitMapGenEvent.EventType.MINESHAFT);
    scatteredFeatureGenerator = (MapGenScatteredFeature) TerrainGen.getModdedMapGen(scatteredFeatureGenerator, InitMapGenEvent.EventType.SCATTERED_FEATURE);
    ravineGenerator = TerrainGen.getModdedMapGen(ravineGenerator, InitMapGenEvent.EventType.RAVINE);
    oceanMonumentGenerator = (StructureOceanMonument) TerrainGen.getModdedMapGen(oceanMonumentGenerator, InitMapGenEvent.EventType.OCEAN_MONUMENT);
  }

  public void generate(World world, int x, int z, ChunkPrimer chunk) {
    // NOTE: If we ever want to add vanilla caves/ravines, we can do so with this.
    // this.caveGenerator.generate(world, x, z, chunk);
    // this.ravineGenerator.generate(world, x, z, chunk);

    this.mineshaftGenerator.generate(world, x, z, chunk);
    this.villageGenerator.generate(world, x, z, chunk);
    this.strongholdGenerator.generate(world, x, z, chunk);
    this.scatteredFeatureGenerator.generate(world, x, z, chunk);
    this.oceanMonumentGenerator.generate(world, x, z, chunk);
  }

  public void decorate(World world, Random rand, ChunkPos chunk_pos) {
    this.mineshaftGenerator.generateStructure(world, rand, chunk_pos);
    this.villageGenerator.generateStructure(world, rand, chunk_pos);
    this.strongholdGenerator.generateStructure(world, rand, chunk_pos);
    this.scatteredFeatureGenerator.generateStructure(world, rand, chunk_pos);
    this.oceanMonumentGenerator.generateStructure(world, rand, chunk_pos);

    // Generate decorations for the void biome, which ends up just being ores and some stone patches.
    //decorator.decorate(world, rand, Biomes.VOID, new BlockPos(chunk_pos.x * 16, 0, chunk_pos.z * 16));
  }
}
