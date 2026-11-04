package net.macmv.rgen;

import net.fabricmc.api.ModInitializer;
import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.item.RItems;
import net.minecraft.item.ItemGroup;
import net.minecraft.item.ItemStack;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.text.Text;
import net.minecraft.util.Identifier;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class RGen implements ModInitializer {
  public static final String MOD_ID = "rgen";
  public static final Logger LOG = LoggerFactory.getLogger(MOD_ID);

  @Override
  public void onInitialize() {
    // Blocks go first, because the items refer to them.
    RBlocks.init();
    RItems.init();

    // The 1.12 mod has two creative tabs, but for now everything lives in one.
    Registry.register(
      Registries.ITEM_GROUP,
      Identifier.of(MOD_ID, "rgen"),
      ItemGroup.create(ItemGroup.Row.TOP, 0)
        .displayName(Text.literal("RGen"))
        .icon(() -> new ItemStack(RItems.MOSSY_STUMP))
        .entries((context, entries) -> RItems.all().forEach(entries::add))
        .build()
    );
  }
}
