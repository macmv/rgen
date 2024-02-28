package net.macmv.rgen;

import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.item.RItems;
import net.macmv.rgen.world.WorldTypeRGen;
import net.minecraft.block.Block;
import net.minecraft.item.Item;
import net.minecraftforge.client.event.ModelRegistryEvent;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.RegistryEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;
import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.fml.relauncher.Side;
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
}
