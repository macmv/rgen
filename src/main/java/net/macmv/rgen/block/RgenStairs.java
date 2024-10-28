package net.macmv.rgen.block;

import net.minecraft.block.BlockStairs;
import net.minecraft.block.SoundType;

// Base class for all custom stairs
public class RgenStairs extends BlockStairs {

  public RgenStairs(LogType type) {
    super(RBlocks.PLANKS.getDefaultState().withProperty(PlanksBlock.VARIANT, type));
    this.setHardness(2.0F);
    this.setSoundType(SoundType.WOOD);
  }
}