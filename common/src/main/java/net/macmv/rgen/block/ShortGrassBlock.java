package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;
import net.minecraft.block.state.IBlockState;
import net.minecraft.init.Blocks;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class ShortGrassBlock extends BlockBush {
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.1D, 0.0D, 0.1D, 0.9D, 0.8D, 0.9D);

  public ShortGrassBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
  }

  @Override
  public boolean canBlockStay(World world, BlockPos pos, IBlockState state) {
    Block soil = world.getBlockState(pos.down()).getBlock();
    return soil == Blocks.GRASS || soil == Blocks.DIRT || soil == Blocks.FARMLAND
        || soil == Blocks.SAND || soil == Blocks.GRAVEL;
  }

  @Override
  public boolean canPlaceBlockAt(World world, BlockPos pos) {
    return world.getBlockState(pos).getBlock().isReplaceable(world, pos)
        && canBlockStay(world, pos, getDefaultState());
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
  public boolean isOpaqueCube(IBlockState state) { return false; }

  @Override
  public boolean isFullCube(IBlockState state) { return false; }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }
}
