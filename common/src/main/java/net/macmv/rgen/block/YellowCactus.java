package net.macmv.rgen.block;

import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.IGrowable;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class YellowCactus extends Block implements IGrowable {
  public static final PropertyInteger AGE = PropertyInteger.create("age", 0, 3);
  private static final double[] WIDTHS = {0.25, 0.375, 0.5, 0.375};
  private static final double[] HEIGHTS = {5.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 7.0 / 16.0};

  public YellowCactus(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
    setDefaultState(blockState.getBaseState().withProperty(AGE, 0));
    setTickRandomly(true);
  }

  @Override
  public void updateTick(World world, BlockPos pos, IBlockState state, Random rand) {
    if (world.isAreaLoaded(pos, 1) && state.getValue(AGE) < 3 && rand.nextInt(5) == 0) {
      world.setBlockState(pos, state.withProperty(AGE, state.getValue(AGE) + 1), 2);
    }
  }

  @Override public boolean canGrow(World world, BlockPos pos, IBlockState state, boolean isClient) { return state.getValue(AGE) < 3; }
  @Override public boolean canUseBonemeal(World world, Random rand, BlockPos pos, IBlockState state) { return rand.nextFloat() < 0.45F; }
  @Override public void grow(World world, Random rand, BlockPos pos, IBlockState state) {
    if (state.getValue(AGE) < 3) world.setBlockState(pos, state.withProperty(AGE, state.getValue(AGE) + 1), 2);
  }

  private AxisAlignedBB bounds(IBlockState state) {
    int age = state.getValue(AGE);
    double half = WIDTHS[age] / 2.0;
    return new AxisAlignedBB(0.5 - half, 0, 0.5 - half, 0.5 + half, HEIGHTS[age], 0.5 + half);
  }

  @Override public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return bounds(state); }
  @Override public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return state.getValue(AGE) == 0 ? NULL_AABB : bounds(state);
  }
  @Override public void onEntityCollidedWithBlock(World world, BlockPos pos, IBlockState state, Entity entity) { entity.attackEntityFrom(DamageSource.CACTUS, 1.0F); }

  private boolean hasValidGround(World world, BlockPos pos) { return world.getBlockState(pos.down()).getBlock() == Blocks.SAND; }
  private void removeIfUnsupported(World world, BlockPos pos, IBlockState state) {
    if (!hasValidGround(world, pos)) {
      for (ItemStack drop : getDrops(world, pos, state, 0)) spawnAsEntity(world, pos, drop);
      world.setBlockToAir(pos);
    }
  }
  @Override public void onBlockAdded(World world, BlockPos pos, IBlockState state) { removeIfUnsupported(world, pos, state); }
  @Override public void neighborChanged(IBlockState state, World world, BlockPos pos, Block block, BlockPos fromPos) { removeIfUnsupported(world, pos, state); }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new ArrayList<>();
    if (state.getValue(AGE) == 3) drops.add(new ItemStack(RItems.YELLOW_CACTUS_FRUIT, 1 + new Random().nextInt(3)));
    return drops;
  }

  @Override public IBlockState getStateFromMeta(int meta) { return getDefaultState().withProperty(AGE, meta & 3); }
  @Override public int getMetaFromState(IBlockState state) { return state.getValue(AGE); }
  @Override protected BlockStateContainer createBlockState() { return new BlockStateContainer(this, AGE); }
  @Override public boolean isOpaqueCube(IBlockState state) { return false; }
  @Override public boolean isFullCube(IBlockState state) { return false; }
  @SideOnly(Side.CLIENT) public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }
}
