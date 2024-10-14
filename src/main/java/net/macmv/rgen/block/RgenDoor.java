package net.macmv.rgen.block;

import net.minecraft.block.BlockDoor;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;

// Base class for all custom doors
public abstract class RgenDoor extends BlockDoor {

  public RgenDoor() {
    super(Material.WOOD);
    this.setHardness(3.0F);
    this.setSoundType(SoundType.WOOD);
  }
}

// Specific door classes extending the base class

// Custom properties or methods for Pa
