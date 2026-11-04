package net.macmv.rgen.compat;

import net.minecraft.block.AbstractBlock;

/**
 * The 1.12 block material, which is gone in modern versions. The common sources still pick a
 * material for each block, so this carries the parts of it that matter as block settings.
 */
public final class Material {
  public static final Material ROCK = new Material(false);
  public static final Material WOOD = new Material(false);
  public static final Material GRASS = new Material(false);
  public static final Material GROUND = new Material(false);
  public static final Material SAND = new Material(false);
  public static final Material CARPET = new Material(false);
  public static final Material LEAVES = new Material(false);
  public static final Material PLANTS = new Material(true);

  private final boolean noCollision;

  private Material(boolean noCollision) {
    this.noCollision = noCollision;
  }

  public AbstractBlock.Settings apply(AbstractBlock.Settings settings) {
    if (this.noCollision) {
      settings.noCollision();
    }
    return settings;
  }
}
