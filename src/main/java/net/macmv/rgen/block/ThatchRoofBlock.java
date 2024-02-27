package net.macmv.rgen.block;

import net.minecraft.block.BlockStairs;
import net.minecraft.block.material.Material;
import net.minecraft.init.Blocks;

public class ThatchRoofBlock extends BlockStairs {
  public ThatchRoofBlock(Material material) {
    super(Blocks.PLANKS.getDefaultState());
  }
}
