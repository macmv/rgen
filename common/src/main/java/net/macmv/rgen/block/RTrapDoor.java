package net.macmv.rgen.block;

import net.minecraft.block.BlockTrapDoor;

public class RTrapDoor extends BlockTrapDoor {

  public RTrapDoor(BlockSettings settings) {
    // #v12-start
    super(settings.material);
    this.setHardness(settings.hardness);
    this.setSoundType(settings.soundType);
    this.disableStats();
    // #v12-end
    /* #fabric: super(net.minecraft.block.BlockSetType.OAK, settings.vanilla()); */
  }
}
