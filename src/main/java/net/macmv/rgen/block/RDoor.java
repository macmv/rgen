package net.macmv.rgen.block;

import net.minecraft.block.BlockDoor;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;

public class RDoor extends BlockDoor {

  public RDoor() {
    super(Material.WOOD);
    this.setHardness(3.0F);
    this.setSoundType(SoundType.WOOD);
  }
}
