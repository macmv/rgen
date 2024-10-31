package net.macmv.rgen.block;

import net.minecraft.block.BlockTrapDoor;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;

public class RTrapDoor extends BlockTrapDoor {

  public RTrapDoor() {
    super(Material.WOOD);
    this.setHardness(3.0F);
    this.setResistance(5.0F);
    this.setSoundType(SoundType.WOOD);
  }
}