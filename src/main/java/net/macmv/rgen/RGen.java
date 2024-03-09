package net.macmv.rgen;

import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.entity.REntities;
import net.macmv.rgen.item.RItems;
import net.macmv.rgen.rust.RustGenerator;
import net.macmv.rgen.world.WorldTypeRGen;
import net.minecraft.block.Block;
import net.minecraft.client.Minecraft;
import net.minecraft.item.Item;
import net.minecraftforge.client.event.ModelRegistryEvent;
import net.minecraftforge.client.event.RenderGameOverlayEvent;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.RegistryEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;
import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.fml.common.registry.EntityEntry;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

@Mod(modid = RGen.MODID, version = RGen.VERSION)
@Mod.EventBusSubscriber(value = Side.CLIENT, modid = RGen.MODID)
public class RGen {
  public static final String MODID = "rgen";
  public static final String VERSION = "1.0";
  public static Logger LOG = LogManager.getLogger(RGen.MODID);

  public static WorldTypeRGen worldType;


  @Mod.EventHandler
  public void preInit(FMLPreInitializationEvent e) {
    MinecraftForge.EVENT_BUS.register(this);

    worldType = new WorldTypeRGen();

    REntities.registerModels();
  }

  @SubscribeEvent
  public void registerBlocks(RegistryEvent.Register<Block> event) {
    IForgeRegistry<Block> reg = event.getRegistry();

    RBlocks.registerBlocks(reg);
  }

  @SubscribeEvent
  public void registerItems(RegistryEvent.Register<Item> event) {
    IForgeRegistry<Item> reg = event.getRegistry();

    RItems.registerItems(reg);
  }

  @SubscribeEvent
  public static void registerModels(ModelRegistryEvent event) {
    RItems.registerModels();
  }

  @SubscribeEvent
  public static void registerEntities(RegistryEvent.Register<EntityEntry> event) {
    IForgeRegistry<EntityEntry> reg = event.getRegistry();

    REntities.registerEntities(reg);
  }

  @SideOnly(Side.CLIENT)
  @SubscribeEvent
  public static void renderDebugText(RenderGameOverlayEvent.Text event) {
    if (Minecraft.getMinecraft().gameSettings.showDebugInfo && RustGenerator.isActive()) {
      event.getLeft().add("");
      event.getLeft().add("RGen");

      int x = Minecraft.getMinecraft().player.getPosition().getX();
      int y = Minecraft.getMinecraft().player.getPosition().getY();
      int z = Minecraft.getMinecraft().player.getPosition().getZ();

      String[] lines = RustGenerator.getDebugInfo(x, y, z);
      for (String line : lines) {
        event.getLeft().add(line);
      }
    }
  }
}
