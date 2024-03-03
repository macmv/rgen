package net.macmv.rgen.block;

import net.macmv.rgen.RGen;
import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraftforge.registries.IForgeRegistry;

import java.util.HashSet;
import java.util.Set;

public final class RBlocks {
  private static final Set<Block> blocks = new HashSet<>();

  // FIXME: Need block items.
  // public static final Block THATCH_ROOF = register("thatch_roof", new ThatchRoofBlock(Material.ROCK));
  public static final Block DERP_DOG = register("derp_dog", new Block(Material.CLOTH));
  public static final Block LOOSE_ROCK = register("loose_rock", new LooseRockBlock());

  public static final Block LOG = register("log", new RGenLogBlockOne());

  private static Block register(String name, Block block) {
    Block b = block.setRegistryName(RGen.MODID, name).setUnlocalizedName(name);
    blocks.add(b);
    return b;
  }

  public static void registerBlocks(IForgeRegistry<Block> reg) {
    for (Block b : blocks) {
      reg.register(b);
    }
  }
}
