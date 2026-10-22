package net.macmv.rgen.block;

import net.minecraft.block.BlockFence;
import net.minecraft.block.material.Material;

public class RWoodFence extends BlockFence {
  public RWoodFence(BlockSettings settings, LogType type) {
    // #v12-start
    super(Material.WOOD, type.mapColor);
    // #v12-end
    /* #fabric: super(settings.vanilla()); */
    this.setHardness(settings.hardness);
    this.setResistance(settings.resistance);
    this.setSoundType(settings.soundType);
    this.setCreativeTab(settings.creativeTab);
  }
}
