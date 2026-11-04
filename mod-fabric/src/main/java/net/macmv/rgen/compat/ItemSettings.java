package net.macmv.rgen.compat;

import net.macmv.rgen.RGen;
import net.minecraft.block.Block;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.util.Identifier;
//? if >=1.21.2 {
/*import net.minecraft.registry.RegistryKey;
import net.minecraft.registry.RegistryKeys;*/
//?}

import java.util.function.Supplier;

/**
 * Item settings. Newer versions need to know the name of an item before it is created, so the
 * settings for an item that isn't a block item come from `create`, after `build` has set its name.
 */
public final class ItemSettings {
  private static String pending = null;

  private ItemSettings() {}

  // Creates the item with `name`. Every item is created like this, or with `forBlock`.
  public static <T extends Item> T build(String name, Supplier<T> constructor) {
    pending = name;
    try {
      return constructor.get();
    } finally {
      pending = null;
    }
  }

  public static Item.Settings create() {
    if (pending == null) {
      throw new IllegalStateException("items must be created with ItemSettings.build");
    }

    Item.Settings settings = new Item.Settings();
    //? if >=1.21.2 {
    /*settings.registryKey(RegistryKey.of(RegistryKeys.ITEM, Identifier.of(RGen.MOD_ID, pending)));*/
    //?}
    return settings;
  }

  // The settings for the item of `block`, which has the same name.
  public static Item.Settings forBlock(Block block) {
    Item.Settings settings = new Item.Settings();
    //? if >=1.21.2 {
    /*settings.registryKey(RegistryKey.of(RegistryKeys.ITEM, Registries.BLOCK.getId(block)));
    settings.useBlockPrefixedTranslationKey();*/
    //?}
    return settings;
  }
}
