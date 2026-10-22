package net.macmv.rgen.block;

import net.macmv.rgen.compat.CreativeTabs;
import net.macmv.rgen.compat.Material;
import net.macmv.rgen.compat.SoundType;
import net.minecraft.block.AbstractBlock;
import net.minecraft.block.Block;
import net.minecraft.sound.BlockSoundGroup;

import java.util.function.Function;

/**
 * The fabric version of the 1.12 block settings polyfill. This is what the common sources build,
 * and `vanilla()` turns it into the real thing.
 */
public class BlockSettings {
  public float hardness = 0.0f;
  public float resistance = 0.0f;
  public BlockSoundGroup soundType = SoundType.STONE;
  public Material material = Material.ROCK;
  public CreativeTabs creativeTab = CreativeTabs.BUILDING_BLOCKS;
  public int lightValue = 0;

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

  public BlockSettings soundType(BlockSoundGroup soundType) {
    this.soundType = soundType;
    return this;
  }

  public BlockSettings lightValue(int lightValue) {
    this.lightValue = lightValue;
    return this;
  }

  // Builds a block from vanilla settings, like `Block::new`.
  public <T extends Block> T createMat(Function<AbstractBlock.Settings, T> constructor) {
    return constructor.apply(this.vanilla());
  }

  public <T extends Block> T create(Function<BlockSettings, T> constructor) {
    return constructor.apply(this);
  }

  public AbstractBlock.Settings vanilla() {
    AbstractBlock.Settings settings = AbstractBlock.Settings.create();
    this.material.apply(settings);
    settings.strength(this.hardness, this.resistance);
    settings.sounds(this.soundType);
    settings.luminance(state -> this.lightValue);
    return settings;
  }
}
