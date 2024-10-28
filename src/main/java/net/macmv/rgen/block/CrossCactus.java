package net.macmv.rgen.block;
// This plant is for all cross plants that grown on sand

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;

public class CrossCactus extends BlockBush {
  protected CrossCactus() {
    this.setCreativeTab(RCreativeTabs.DECORATIONS);
    this.setSoundType(SoundType.PLANT);
  }
}
