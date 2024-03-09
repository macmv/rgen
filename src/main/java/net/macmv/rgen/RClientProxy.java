package net.macmv.rgen;

import net.macmv.rgen.entity.REntities;

public class RClientProxy extends RCommonProxy {

  @Override
  public void preInit() {
    REntities.registerModels();
  }
}
