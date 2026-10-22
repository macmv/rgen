package net.macmv.rgen.block;

import net.minecraft.block.BlockFenceGate;
import net.minecraft.block.BlockPlanks;

public class RWoodFenceGate extends BlockFenceGate {
  public RWoodFenceGate(BlockSettings settings, BlockPlanks.EnumType vanillaWood) {
    // #v12-start
    super(vanillaWood);
    // #v12-end
    /* #fabric: super(vanillaWood, settings.vanilla()); */
    this.setHardness(settings.hardness);
    this.setResistance(settings.resistance);
    this.setSoundType(settings.soundType);
    this.setCreativeTab(settings.creativeTab);
  }
}
