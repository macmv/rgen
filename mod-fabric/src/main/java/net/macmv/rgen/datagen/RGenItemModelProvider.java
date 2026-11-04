package net.macmv.rgen.datagen;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import net.fabricmc.fabric.api.datagen.v1.FabricDataOutput;
import net.macmv.rgen.RGen;
import net.minecraft.data.DataOutput;
import net.minecraft.data.DataProvider;
import net.minecraft.data.DataWriter;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.util.Identifier;

import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

// The 1.12 assets name item models after the variant (`fir_log`), but items are named after the
// block now. This makes the item model of each block item that doesn't have one use the first
// model of its blockstate.
public class RGenItemModelProvider implements DataProvider {
  private final DataOutput.PathResolver resolver;

  public RGenItemModelProvider(FabricDataOutput output) {
    this.resolver = output.getResolver(DataOutput.OutputType.RESOURCE_PACK, "models/item");
  }

  @Override
  public CompletableFuture<?> run(DataWriter writer) {
    List<CompletableFuture<?>> futures = new ArrayList<>();

    for (Item item : Registries.ITEM) {
      Identifier id = Registries.ITEM.getId(item);
      if (!id.getNamespace().equals(RGen.MOD_ID) || !(item instanceof BlockItem)) {
        continue;
      }

      // Hand written models win.
      if (resource("models/item/" + id.getPath() + ".json") != null) {
        continue;
      }

      String model = firstModel(id.getPath());
      if (model == null) {
        RGen.LOG.warn("no blockstate to make an item model from for {}", id);
        continue;
      }

      JsonObject json = new JsonObject();
      json.addProperty("parent", model);
      futures.add(DataProvider.writeToPath(writer, json, this.resolver.resolveJson(id)));
    }

    return CompletableFuture.allOf(futures.toArray(CompletableFuture[]::new));
  }

  private static String firstModel(String name) {
    JsonElement state = resource("blockstates/" + name + ".json");
    if (state == null) {
      return null;
    }

    JsonObject root = state.getAsJsonObject();
    JsonElement entry;
    if (root.has("variants")) {
      entry = root.getAsJsonObject("variants").entrySet().stream().findFirst().map(Map.Entry::getValue).orElse(null);
    } else if (root.has("multipart")) {
      entry = root.getAsJsonArray("multipart").get(0).getAsJsonObject().get("apply");
    } else {
      return null;
    }

    if (entry != null && entry.isJsonArray()) {
      entry = entry.getAsJsonArray().get(0);
    }
    return entry == null ? null : entry.getAsJsonObject().get("model").getAsString();
  }

  private static JsonElement resource(String path) {
    try (InputStream in = RGenItemModelProvider.class.getResourceAsStream("/assets/" + RGen.MOD_ID + "/" + path)) {
      if (in == null) {
        return null;
      }
      return JsonParser.parseReader(new InputStreamReader(in, StandardCharsets.UTF_8));
    } catch (IOException e) {
      throw new RuntimeException(e);
    }
  }

  @Override
  public String getName() {
    return "RGen item models";
  }
}
