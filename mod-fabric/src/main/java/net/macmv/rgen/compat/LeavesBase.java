package net.macmv.rgen.compat;

import net.minecraft.block.AbstractBlock;

/** Leaves that are built from settings only. Newer versions also want a particle chance. */
public abstract class LeavesBase extends
  //? if >=1.21.5 {
  /*net.minecraft.block.TintedParticleLeavesBlock*/
  //?} else {
  net.minecraft.block.LeavesBlock
  //?}
{
  public LeavesBase(AbstractBlock.Settings settings) {
    //? if >=1.21.5 {
    /*super(0.01F, settings);*/
    //?} else {
    super(settings);
    //?}
  }
}
