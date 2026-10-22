package net.macmv.rgen.block;

import net.minecraft.block.BlockFenceGate;
import net.minecraft.block.BlockPlanks;

public class RWoodFenceGate extends BlockFenceGate {
  public RWoodFenceGate(BlockSettings settings, BlockPlanks.EnumType vanillaWood) {
    super(vanillaWood);
    this.setHardness(settings.hardness);
    this.setResistance(settings.resistance);
    this.setSoundType(settings.soundType);
    this.setCreativeTab(settings.creativeTab);
  }
}
