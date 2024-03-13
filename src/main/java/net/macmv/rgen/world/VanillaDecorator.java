package net.macmv.rgen.world;

import net.minecraft.util.math.ChunkPos;
import net.minecraft.world.World;
import net.minecraft.world.chunk.ChunkPrimer;
import net.minecraft.world.gen.MapGenBase;
import net.minecraft.world.gen.MapGenCaves;
import net.minecraft.world.gen.MapGenRavine;
import net.minecraft.world.gen.structure.*;

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
    caveGenerator = net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(caveGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.CAVE);
    strongholdGenerator = (MapGenStronghold) net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(strongholdGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.STRONGHOLD);
    villageGenerator = (MapGenVillage) net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(villageGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.VILLAGE);
    mineshaftGenerator = (MapGenMineshaft) net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(mineshaftGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.MINESHAFT);
    scatteredFeatureGenerator = (MapGenScatteredFeature) net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(scatteredFeatureGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.SCATTERED_FEATURE);
    ravineGenerator = net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(ravineGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.RAVINE);
    oceanMonumentGenerator = (StructureOceanMonument) net.minecraftforge.event.terraingen.TerrainGen.getModdedMapGen(oceanMonumentGenerator, net.minecraftforge.event.terraingen.InitMapGenEvent.EventType.OCEAN_MONUMENT);
  }

  public void generate(World world, int x, int z, ChunkPrimer chunk) {
    // TODO: Vanilla caves go brrr
    this.caveGenerator.generate(world, x, z, chunk);
    this.ravineGenerator.generate(world, x, z, chunk);

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
  }
}
