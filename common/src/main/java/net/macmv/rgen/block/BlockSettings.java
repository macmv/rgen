package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.creativetab.CreativeTabs;

import java.util.function.Function;

// This is effectively a polyfill for modern block settings.
public class BlockSettings {
  public float hardness = 0.0f;
  public float resistance = 0.0f;
  public SoundType soundType = SoundType.STONE;
  public Material material = Material.ROCK;
  public CreativeTabs creativeTab = CreativeTabs.BUILDING_BLOCKS;
  public int lightValue = 0;

  public <T extends Block> T createMat(Function<Material, T> constructor) {
    T block = constructor.apply(this.material);
    this.apply(block);
    return block;
  }

  public <T extends Block> T create(Function<BlockSettings, T> constructor) {
    T block = constructor.apply(this);
    this.apply(block);
    return block;
  }

  public void apply(Block block) {
    block.setHardness(hardness);
    block.setResistance(hardness);
    block.setCreativeTab(creativeTab);
    block.setLightLevel(((float) lightValue) / 15f);
  }

  public BlockSettings material(Material material) {
    this.material = material;
    return this;
  }

  public BlockSettings creativeTab(CreativeTabs creativeTab) {
    this.creativeTab = creativeTab;
    return this;
  }

  public BlockSettings hardness(float hardness) {
    this.hardness = hardness;
    return this;
  }

  public BlockSettings resistance(float resistance) {
    this.resistance = resistance;
    return this;
  }

  public BlockSettings soundType(SoundType soundType) {
    this.soundType = soundType;
    return this;
  }

  public BlockSettings lightValue(int lightValue) {
    this.lightValue = lightValue;
    return null;
  }
}
