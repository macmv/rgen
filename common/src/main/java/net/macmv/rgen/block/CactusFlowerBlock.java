package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

/** Large cross-shaped flower supported by the top of a green cactus trunk. */
public class CactusFlowerBlock extends Block {
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.28D, 0.0D, 0.28D, 0.72D, 0.5D, 0.72D);

  public CactusFlowerBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
    setLightOpacity(0);
  }

  private boolean hasSupport(World world, BlockPos pos) {
    Block trunk = world.getBlockState(pos.down()).getBlock();
    // The mature green trunk currently exists under both registry names while older
    // generated worlds may still contain the generic RGen cactus block.
    return trunk == RBlocks.CACTUS || trunk == RBlocks.RGEN_CACTUS;
  }

  @Override
  public boolean canPlaceBlockAt(World world, BlockPos pos) {
    return world.getBlockState(pos).getBlock().isReplaceable(world, pos) && hasSupport(world, pos);
  }

  @Override
  public void onBlockAdded(World world, BlockPos pos, IBlockState state) { removeIfUnsupported(world, pos); }

  @Override
  public void neighborChanged(IBlockState state, World world, BlockPos pos, Block blockIn, BlockPos fromPos) {
    removeIfUnsupported(world, pos);
  }

  private void removeIfUnsupported(World world, BlockPos pos) {
    if (!world.isRemote && !hasSupport(world, pos)) world.destroyBlock(pos, true);
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return BOUNDS; }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return NULL_AABB; }

  @Override
  public boolean isOpaqueCube(IBlockState state) { return false; }

  @Override
  public boolean isFullCube(IBlockState state) { return false; }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }
}
