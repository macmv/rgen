package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.Collections;
import java.util.List;

/** The single-state green cactus trunk block. */
public class Cactus extends Block {
  private static final AxisAlignedBB BOUNDS = MathUtil.aabb(1, 0, 1, 15, 16, 15);

  public Cactus(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
  }

  @Override public boolean isOpaqueCube(IBlockState state) { return false; }
  @Override public boolean isFullCube(IBlockState state) { return false; }

  @SideOnly(Side.CLIENT)
  @Override public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }

  @Override
  public void onEntityCollidedWithBlock(World world, BlockPos pos, IBlockState state, Entity entity) {
    entity.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }

  private boolean hasValidGround(World world, BlockPos pos) {
    Block below = world.getBlockState(pos.down()).getBlock();
    return below == Blocks.SAND || below == this;
  }

  private void removeIfUnsupported(World world, BlockPos pos, IBlockState state) {
    if (!hasValidGround(world, pos)) {
      for (ItemStack drop : getDrops(world, pos, state, 0)) spawnAsEntity(world, pos, drop);
      world.setBlockToAir(pos);
      world.playEvent(2001, pos, Block.getStateId(state));
    }
  }

  @Override
  public void neighborChanged(IBlockState state, World world, BlockPos pos, Block block, BlockPos fromPos) {
    removeIfUnsupported(world, pos, state);
  }

  @Override
  public void onBlockAdded(World world, BlockPos pos, IBlockState state) {
    removeIfUnsupported(world, pos, state);
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    return Collections.singletonList(new ItemStack(Item.getItemFromBlock(Blocks.CACTUS), 1));
  }

  @Override public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return BOUNDS; }
  @Override public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return BOUNDS; }
}
