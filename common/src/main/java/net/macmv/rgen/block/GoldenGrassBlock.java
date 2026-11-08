package net.macmv.rgen.block;

import net.macmv.rgen.item.RItems;
import net.minecraft.block.BlockBush;
import net.minecraft.block.BlockDoublePlant;
import net.minecraft.block.SoundType;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.ArrayList;
import java.util.List;

public class GoldenGrassBlock extends BlockBush {
  public static final PropertyEnum<Variant> VARIANT = PropertyEnum.create("variant", Variant.class);
  public static final PropertyEnum<BlockDoublePlant.EnumBlockHalf> HALF = PropertyEnum.create("half", BlockDoublePlant.EnumBlockHalf.class);
  private static final AxisAlignedBB SHORT_BOUNDS = new AxisAlignedBB(0.1D, 0.0D, 0.1D, 0.9D, 0.8D, 0.9D);
  private static final AxisAlignedBB TALL_BOUNDS = new AxisAlignedBB(0.0D, 0.0D, 0.0D, 1.0D, 1.0D, 1.0D);

  public GoldenGrassBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
    setDefaultState(blockState.getBaseState()
        .withProperty(VARIANT, Variant.SHORT_1)
        .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.LOWER));
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    Variant variant = Variant.byMetadata(meta & 7);
    BlockDoublePlant.EnumBlockHalf half = (meta & 8) == 0
        ? BlockDoublePlant.EnumBlockHalf.LOWER
        : BlockDoublePlant.EnumBlockHalf.UPPER;
    return getDefaultState().withProperty(VARIANT, variant).withProperty(HALF, half);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(VARIANT).metadata;
    if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) meta |= 8;
    return meta;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, HALF, VARIANT);
  }

  @Override
  public boolean canBlockStay(World world, BlockPos pos, IBlockState state) {
    Variant variant = state.getValue(VARIANT);
    if (variant.isTall() && state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
      IBlockState below = world.getBlockState(pos.down());
      return below.getBlock() == this
          && below.getValue(VARIANT) == variant
          && below.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER;
    }

    IBlockState soil = world.getBlockState(pos.down());
    return soil.getBlock() == Blocks.GRASS || soil.getBlock() == Blocks.DIRT
        || soil.getBlock() == Blocks.FARMLAND || soil.getBlock() == Blocks.SAND;
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new ArrayList<>();
    if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER) {
      drops.add(new ItemStack(RItems.GOLDEN_GRASS));
    }
    return drops;
  }

  @Override
  public ItemStack getItem(World world, BlockPos pos, IBlockState state) {
    return new ItemStack(RItems.GOLDEN_GRASS);
  }

  @Override
  public void onBlockHarvested(World world, BlockPos pos, IBlockState state, EntityPlayer player) {
    Variant variant = state.getValue(VARIANT);
    if (variant.isTall()) {
      BlockPos otherHalf = state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER ? pos.up() : pos.down();
      IBlockState otherState = world.getBlockState(otherHalf);
      if (otherState.getBlock() == this && otherState.getValue(VARIANT) == variant) {
        // Removing the upper half removes the lower half without dropping an item.
        world.setBlockToAir(otherHalf);
      }
    }
    super.onBlockHarvested(world, pos, state, player);
  }

  @Override
  public int damageDropped(IBlockState state) {
    return 0;
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this));
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return state.getValue(VARIANT).isTall() ? TALL_BOUNDS : SHORT_BOUNDS;
  }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  public enum Variant implements IStringSerializable {
    SHORT_1(0, "short_1", false),
    SHORT_2(1, "short_2", false),
    SHORT_3(2, "short_3", false),
    TALL_1(3, "tall_1", true),
    TALL_2(4, "tall_2", true);

    private static final Variant[] LOOKUP = values();
    public final int metadata;
    private final String name;
    private final boolean tall;

    Variant(int metadata, String name, boolean tall) {
      this.metadata = metadata;
      this.name = name;
      this.tall = tall;
    }

    public static Variant byMetadata(int metadata) {
      return LOOKUP[Math.floorMod(metadata, LOOKUP.length)];
    }

    public boolean isTall() {
      return tall;
    }

    @Override
    public String getName() {
      return name;
    }
  }
}
