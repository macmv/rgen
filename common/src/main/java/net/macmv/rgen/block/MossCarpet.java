package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

public class MossCarpet extends Block {
  //protected static final AxisAlignedBB PLANT_AABB = new AxisAlignedBB(0.09999999403953552, 0.0, 0.09999999403953552, 0.8999999761581421, 0.800000011920929, 0.8999999761581421);
  protected static final AxisAlignedBB CARPET_AABB = new AxisAlignedBB(0.0D, 0.0D, 0.0D, 1.0D, 0.0625D, 1.0D);

  public MossCarpet(BlockSettings settings) {
    super(settings.material);
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return CARPET_AABB;
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
  public boolean canPlaceBlockAt(World worldIn, BlockPos pos) {
    return super.canPlaceBlockAt(worldIn, pos) && this.canBlockStay(worldIn, pos);
  }

  @Override
  public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
    this.checkForDrop(worldIn, pos, state);
  }

  private boolean checkForDrop(World worldIn, BlockPos pos, IBlockState state) {
    if (!this.canBlockStay(worldIn, pos)) {
      this.dropBlockAsItem(worldIn, pos, state, 0);
      worldIn.setBlockToAir(pos);
      return false;
    } else {
      return true;
    }
  }

  private boolean canBlockStay(World worldIn, BlockPos pos) {
    return !worldIn.isAirBlock(pos.down());
  }

  //@Override
  //public AxisAlignedBB getBoundingBox(IBlockState p_185496_1_, IBlockAccess p_185496_2_, BlockPos p_185496_3_) {
  //    return PLANT_AABB;
  //}
}
