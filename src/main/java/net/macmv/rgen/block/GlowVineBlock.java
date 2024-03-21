package net.macmv.rgen.block;

import net.minecraft.block.BlockVine;
import net.minecraft.block.state.IBlockState;

public class GlowVineBlock extends BlockVine {

  @Override
  public int getLightValue(IBlockState state) {
    return 3;
  }
}
