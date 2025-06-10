package net.macmv.rgen;

import net.fabricmc.api.ModInitializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class RGen implements ModInitializer {
  public static final String MOD_ID = "rgen";
  public static final Logger LOG = LoggerFactory.getLogger(MOD_ID);

  @Override
  public void onInitialize() {
    LOG.info("Hello Fabric world!");
  }
}
