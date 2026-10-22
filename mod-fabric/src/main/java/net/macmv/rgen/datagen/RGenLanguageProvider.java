package net.macmv.rgen.datagen;

import net.fabricmc.fabric.api.datagen.v1.FabricDataOutput;
import net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider;
import net.macmv.rgen.RGen;
import net.minecraft.block.Block;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.registry.RegistryWrapper;
import net.minecraft.util.Identifier;

import java.util.Locale;
import java.util.concurrent.CompletableFuture;

// Names everything after its registry name (`log2` is "Log2"). Better names can go in
// `generateTranslations`, after the loops.
public class RGenLanguageProvider extends FabricLanguageProvider {
  public RGenLanguageProvider(FabricDataOutput output, CompletableFuture<RegistryWrapper.WrapperLookup> lookup) {
    super(output, "en_us", lookup);
  }

  @Override
  public void generateTranslations(RegistryWrapper.WrapperLookup lookup, TranslationBuilder builder) {
    for (Block block : Registries.BLOCK) {
      Identifier id = Registries.BLOCK.getId(block);
      if (id.getNamespace().equals(RGen.MOD_ID)) {
        builder.add(block, title(id.getPath()));
      }
    }

    for (Item item : Registries.ITEM) {
      Identifier id = Registries.ITEM.getId(item);
      // Block items are named after their block.
      if (id.getNamespace().equals(RGen.MOD_ID) && !(item instanceof BlockItem)) {
        builder.add(item, title(id.getPath()));
      }
    }
  }

  private static String title(String name) {
    StringBuilder out = new StringBuilder();
    for (String word : name.split("_")) {
      if (!out.isEmpty()) {
        out.append(' ');
      }
      out.append(word.substring(0, 1).toUpperCase(Locale.ROOT)).append(word.substring(1));
    }
    return out.toString();
  }
}
