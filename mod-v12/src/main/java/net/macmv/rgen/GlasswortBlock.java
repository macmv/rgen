package net.macmv.rgen;

import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class GlasswortBlock extends BlockBush {
  public static final PropertyInteger VARIANT = PropertyInteger.create("variant", 0, 5);
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.05D, 0.0D, 0.05D, 0.95D, 0.5D, 0.95D);

  public GlasswortBlock() {
    super(Material.PLANTS);
    setRegistryName("rgen", "glasswort");
    setUnlocalizedName("rgen.glasswort");
    setSoundType(SoundType.PLANT);
    setDefaultState(blockState.getBaseState().withProperty(VARIANT, 0));
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT);
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return getDefaultState().withProperty(VARIANT, Math.floorMod(meta, 6));
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(VARIANT);
  }

  @Override
  public int damageDropped(IBlockState state) {
    return 0;
  }

  @Override
  public IBlockState getStateForPlacement(World world, BlockPos pos, EnumFacing facing,
      float hitX, float hitY, float hitZ, int meta, EntityLivingBase placer, EnumHand hand) {
    return getDefaultState().withProperty(VARIANT, world.rand.nextInt(6));
  }

  @Override
  public boolean canBlockStay(World world, BlockPos pos, IBlockState state) {
    if (world.getBlockState(pos.down()).getBlock() == MudContent.MUD) return true;
    return super.canBlockStay(world, pos, state);
  }

  @Override
  public boolean canPlaceBlockAt(World world, BlockPos pos) {
    if (world.isAirBlock(pos) && world.getBlockState(pos.down()).getBlock() == MudContent.MUD) return true;
    return super.canPlaceBlockAt(world, pos);
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
}
