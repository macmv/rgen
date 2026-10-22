package net.macmv.rgen.datagen;

import net.fabricmc.fabric.api.datagen.v1.DataGeneratorEntrypoint;
import net.fabricmc.fabric.api.datagen.v1.FabricDataGenerator;

// Generates the assets that the 1.12 assets don't have, with `./gradlew :mod-fabric:1.21.1:runDatagen`.
// The output is in `mod-fabric/src/main/generated`, and is checked in.
public class RGenDataGenerator implements DataGeneratorEntrypoint {
  @Override
  public void onInitializeDataGenerator(FabricDataGenerator generator) {
    FabricDataGenerator.Pack pack = generator.createPack();

    pack.addProvider(RGenLanguageProvider::new);
    pack.addProvider(RGenItemModelProvider::new);
  }
}
