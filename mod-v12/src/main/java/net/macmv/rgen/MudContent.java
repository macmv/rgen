package net.macmv.rgen;

import net.minecraft.block.Block;
import net.minecraft.block.BlockStairs;
import net.minecraft.block.BlockWall;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.IBlockState;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.item.Item;
import net.minecraft.item.ItemBlock;
import net.minecraft.item.ItemSlab;
import net.minecraft.util.ResourceLocation;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.event.RegistryEvent;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

/** Minecraft 1.12-only backport of the vanilla mud building set. */
public final class MudContent {
  public static final MudBlock MUD = new MudBlock();
  public static final Block PACKED_MUD = new MudSolidBlock("packed_mud", Material.GROUND, SoundType.GROUND, 1.0f, 3.0f);
  public static final Block MUD_BRICKS = new MudSolidBlock("mud_bricks", Material.ROCK, SoundType.STONE, 1.5f, 3.0f);
  public static final BlockStairs MUD_BRICK_STAIRS = new MudStairs(MUD_BRICKS.getDefaultState());
  public static final MudBrickSlab MUD_BRICK_SLAB = new MudBrickSlab.Half();
  public static final MudBrickSlab MUD_BRICK_DOUBLE_SLAB = new MudBrickSlab.Double();
  public static final BlockWall MUD_BRICK_WALL = new MudWall(MUD_BRICKS);

  private MudContent() {}

  static {
    MUD_BRICK_STAIRS.setRegistryName("rgen", "mud_brick_stairs").setUnlocalizedName("rgen.mud_brick_stairs");
    MUD_BRICK_WALL.setRegistryName("rgen", "mud_brick_wall").setUnlocalizedName("rgen.mud_brick_wall");
    MUD_BRICK_WALL.setCreativeTab(net.minecraft.creativetab.CreativeTabs.BUILDING_BLOCKS);
  }

  public static void registerBlocks(IForgeRegistry<Block> registry) {
    registry.registerAll(MUD, PACKED_MUD, MUD_BRICKS, MUD_BRICK_STAIRS, MUD_BRICK_SLAB, MUD_BRICK_DOUBLE_SLAB, MUD_BRICK_WALL);
  }

  public static void registerItems(IForgeRegistry<Item> registry) {
    registry.register(new ItemBlock(MUD).setRegistryName(MUD.getRegistryName()));
    registry.register(new ItemBlock(PACKED_MUD).setRegistryName(PACKED_MUD.getRegistryName()));
    registry.register(new ItemBlock(MUD_BRICKS).setRegistryName(MUD_BRICKS.getRegistryName()));
    registry.register(new ItemBlock(MUD_BRICK_STAIRS).setRegistryName(MUD_BRICK_STAIRS.getRegistryName()));
    registry.register(new ItemSlab(MUD_BRICK_SLAB, MUD_BRICK_SLAB, MUD_BRICK_DOUBLE_SLAB).setRegistryName(MUD_BRICK_SLAB.getRegistryName()));
    registry.register(new ItemBlock(MUD_BRICK_WALL).setRegistryName(MUD_BRICK_WALL.getRegistryName()));
  }

  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    model(MUD, "mud"); model(PACKED_MUD, "packed_mud"); model(MUD_BRICKS, "mud_bricks");
    model(MUD_BRICK_STAIRS, "mud_brick_stairs"); model(MUD_BRICK_SLAB, "mud_brick_slab"); model(MUD_BRICK_WALL, "mud_brick_wall");
  }

  @SideOnly(Side.CLIENT)
  private static void model(Block block, String name) {
    ModelLoader.setCustomModelResourceLocation(Item.getItemFromBlock(block), 0,
        new ModelResourceLocation(new ResourceLocation("rgen", name), "inventory"));
  }

  private static final class MudSolidBlock extends Block {
    MudSolidBlock(String name, Material material, SoundType sound, float hardness, float resistance) {
      super(material); setHardness(hardness); setResistance(resistance); setSoundType(sound);
      setCreativeTab(net.minecraft.creativetab.CreativeTabs.BUILDING_BLOCKS);
      setRegistryName("rgen", name); setUnlocalizedName("rgen." + name);
    }
  }

  private static final class MudStairs extends BlockStairs {
    MudStairs(IBlockState state) { super(state); setCreativeTab(net.minecraft.creativetab.CreativeTabs.BUILDING_BLOCKS); }
  }

  private static final class MudWall extends BlockWall {
    MudWall(Block block) { super(block); setCreativeTab(net.minecraft.creativetab.CreativeTabs.BUILDING_BLOCKS); }
  }
}
