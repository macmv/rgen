package net.macmv.rgen.compat;

import net.minecraft.block.AbstractBlock;
import net.minecraft.block.BlockState;
import net.minecraft.block.FallingBlock;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.BlockView;

/** A falling block without a color. Newer versions require every falling block to have one. */
public abstract class FallingBase extends FallingBlock {
  public FallingBase(AbstractBlock.Settings settings) {
    super(settings);
  }

  // Only abstract on newer versions.
  public int getColor(BlockState state, BlockView world, BlockPos pos) {
    return 0xFFFFFFFF;
  }
}
