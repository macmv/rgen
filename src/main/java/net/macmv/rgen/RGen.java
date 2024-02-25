package net.macmv.rgen;

import net.macmv.rgen.world.WorldTypeRGen;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

@Mod(modid = RGen.MODID, version = RGen.VERSION)
public class RGen {
  public static final String MODID = "rgen";
  public static final String VERSION = "1.0";
  public static Logger LOG = LogManager.getLogger(RGen.MODID);

  public static WorldTypeRGen worldType;

  @Mod.EventHandler
  public void preInit(FMLPreInitializationEvent e) {
    worldType = new WorldTypeRGen();
  }
}
