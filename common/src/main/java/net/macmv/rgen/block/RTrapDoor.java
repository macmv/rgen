package net.macmv.rgen.block;

import net.minecraft.block.BlockTrapDoor;

public class RTrapDoor extends BlockTrapDoor {

  public RTrapDoor(BlockSettings settings) {
    super(settings.material);
    this.setHardness(settings.hardness);
    this.setSoundType(settings.soundType);
    this.disableStats();
  }
}
