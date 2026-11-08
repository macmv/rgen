package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
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

public class DuneGrassBlock extends BlockBush {
  public static final PropertyEnum<Variant> VARIANT = PropertyEnum.create("variant", Variant.class);
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.1D, 0.0D, 0.1D, 0.9D, 0.8D, 0.9D);

  public DuneGrassBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
    setDefaultState(blockState.getBaseState().withProperty(VARIANT, Variant.A));
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT);
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return getDefaultState().withProperty(VARIANT, Variant.byMetadata(meta));
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(VARIANT).metadata;
  }

  @Override
  public int damageDropped(IBlockState state) {
    // All placed variants return the single, variantless dune grass item.
    return 0;
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this, 1, 0));
  }

  @Override
  public boolean canBlockStay(World world, BlockPos pos, IBlockState state) {
    return supportsPlant(world.getBlockState(pos.down()));
  }

  @Override
  public boolean canPlaceBlockAt(World world, BlockPos pos) {
    IBlockState target = world.getBlockState(pos);
    return target.getBlock().isReplaceable(world, pos) && supportsPlant(world.getBlockState(pos.down()));
  }

  private boolean supportsPlant(IBlockState soil) {
    Block block = soil.getBlock();
    return block == Blocks.GRASS || block == Blocks.DIRT || block == Blocks.FARMLAND
        || block == Blocks.SAND || block == Blocks.GRAVEL;
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return BOUNDS.offset(state.getOffset(world, pos));
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return NULL_AABB;
  }

  @Override
  public Block.EnumOffsetType getOffsetType() {
    return Block.EnumOffsetType.XZ;
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  public enum Variant implements IStringSerializable {
    A(0, "a"),
    B(1, "b"),
    C(2, "c");

    private static final Variant[] LOOKUP = values();
    public final int metadata;
    private final String name;

    Variant(int metadata, String name) {
      this.metadata = metadata;
      this.name = name;
    }

    public static Variant byMetadata(int metadata) {
      return LOOKUP[Math.floorMod(metadata, LOOKUP.length)];
    }

    @Override
    public String getName() {
      return name;
    }
  }
}
