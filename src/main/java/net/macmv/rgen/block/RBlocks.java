package net.macmv.rgen.block;

import net.macmv.rgen.RGen;
import net.macmv.rgen.tab.RCreativeTabs;
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
  public static final Block LOOSE_ROCK = register("loose_rock", new LooseRockBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Block PLANT = register("plant", new PlantBlock().setCreativeTab(RCreativeTabs.DECORATIONS));
  // public static final Block MOSSY_STUMP = register("mossy_stump", new Block(Material.WOOD));
  public static final Block MOSSY_CARPET = register("mossy_carpet", new MossCarpet().setCreativeTab(RCreativeTabs.DECORATIONS));
  public static final Block MOSSY_BLOCK = register("mossy_block", new MossBlock());
  public static final Block MOSSY_COBBLESTONE_RGEN = register("mossy_cobblestone_rgen", new PlantableStone());
  public static final Block MOSSY_STONE = register("mossy_stone", new PlantableStone());
  public static final Block POLYPORE = register("polypore", new PolyporeBlock());

  public static final Block LOG = register("log", new LogBlockOne());
  public static final Block LOG2 = register("log2", new LogBlockTwo());
  public static final Block MOSSY_STUMP = register("mossy_stump", new MossyLogBlock());
  public static final Block BAMBOO = register("bamboo", new Bamboo());

  public static final Block LEAVES = register("leaves", new LeavesBlockOne());
  public static final Block LEAVES2 = register("leaves2", new LeavesBlockTwo());
  public static final Block GLOW_VINE = register("glow_vine", new GlowVineBlock());

  public static final Block PLANKS = register("planks", new PlanksBlock());

  public static final Block FLOWER = register("flower", new FlowerBlock());
  public static final Block CACTUS_ARM = register("cactus_arm", new CactusArm());
  public static final Block RGEN_CACTUS = register("rgen_cactus", new RgenCactus());
  public static final Block CROSS_CACTUS = register("cross_cactus", new CrossCactus());

  private static Block register(String name, Block block) {
    if (block.getCreativeTabToDisplayOn() == null) {
      block.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    }

    block.setRegistryName(RGen.MODID, name);
    block.setUnlocalizedName(name);
    blocks.add(block);
    return block;
  }

  public static void registerBlocks(IForgeRegistry<Block> reg) {
    for (Block b : blocks) {
      reg.register(b);
    }
  }
}
