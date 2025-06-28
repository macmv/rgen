package net.macmv.rgen.block;

import net.minecraft.block.BlockStairs;

// Base class for all custom stairs
public class RStairs extends BlockStairs {

  public RStairs(BlockSettings settings, LogType type) {
    super(RBlocks.PLANKS.getDefaultState().withProperty(PlanksBlock.VARIANT, type));
  }
}
