package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
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

import java.util.List;
import java.util.Random;

public class RgenCactus extends Block {
  protected static final AxisAlignedBB AABB_CACTUS = MathUtil.aabb(1, 0, 1, 15, 16, 15);

  public RgenCactus() {
    super(Material.PLANTS);
    this.setHardness(0.4f);
  }
  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return AABB_CACTUS;
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return AABB_CACTUS;
  }


  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }
  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }

  // Check if the block below is valid (sand or same block type)
  private boolean isValidGround(World worldIn, BlockPos pos) {
    Block blockBelow = worldIn.getBlockState(pos.down()).getBlock();
    return blockBelow == Blocks.SAND || blockBelow == this;
  }

  // Destroy the block and drop fruits
  private void destroyBlockWithDrops(World worldIn, BlockPos pos, IBlockState state) {
    // Get the drops (between 0 and 3 fruits)
    List<ItemStack> drops = getDrops(worldIn, pos, state, 0);
    for (ItemStack drop : drops) {
      spawnAsEntity(worldIn, pos, drop);  // Drop the items in the world
    }
    worldIn.setBlockToAir(pos);  // Destroy the block
    worldIn.playEvent(2001, pos, Block.getStateId(state));  // Show block breaking particles
  }

  @Override
  public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
    // If the ground is not valid, destroy the block
    if (!isValidGround(worldIn, pos)) {
      destroyBlockWithDrops(worldIn, pos, state);
    }
  }

  @Override
  public void onBlockAdded(World worldIn, BlockPos pos, IBlockState state) {
    // When the block is placed, check the block below it
    if (!isValidGround(worldIn, pos)) {
      destroyBlockWithDrops(worldIn, pos, state);
    }
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new java.util.ArrayList<>();
    Random rand = new Random();

    // Random number of drops between 0 and 3
    int dropCount = rand.nextInt(4);  // Will generate a number between 0 and 3
    if (dropCount > 0) {
      drops.add(new ItemStack(RItems.GREEN_CACTUS_FRUIT, dropCount));  // Add the items to the drop list
    }

    return drops;
  }

}
