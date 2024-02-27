package net.macmv.rgen;

import net.macmv.rgen.world.WorldTypeRGen;
import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.RegistryEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;
import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.registries.IForgeRegistry;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

@Mod(modid = RGen.MODID, version = RGen.VERSION)
public class RGen {
  public static final String MODID = "rgen";
  public static final String VERSION = "1.0";
  public static Logger LOG = LogManager.getLogger(RGen.MODID);

  public static WorldTypeRGen worldType;

  // FIXME: Move to a new RBlocks class.
  // FIXME: Need block items.
  private static final Block THATCH_ROOF = new Block(Material.ROCK).setRegistryName(MODID, "thatch_roof");

  @Mod.EventHandler
  public void preInit(FMLPreInitializationEvent e) {
    MinecraftForge.EVENT_BUS.register(this);

    worldType = new WorldTypeRGen();
  }

  @SubscribeEvent
  public void registerBlocks(RegistryEvent.Register<Block> event) {
    IForgeRegistry<Block> reg = event.getRegistry();

    // FIXME: Move to a more generalized registry.
    String name = THATCH_ROOF.getRegistryName().getResourcePath();
    reg.register(THATCH_ROOF.setUnlocalizedName(name));
  }
}
