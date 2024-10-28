package net.macmv.rgen.block;

import net.minecraft.block.BlockBush;
import net.minecraft.block.IGrowable;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.IBlockState;
import net.minecraft.init.Blocks;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraft.world.gen.feature.WorldGenBirchTree;

import java.util.Random;

public abstract class Sapling extends BlockBush implements IGrowable {
  public static final PropertyInteger STAGE = PropertyInteger.create("stage", 0, 1);
  public static final AxisAlignedBB SAPLING_AABB = new AxisAlignedBB(0.09999999403953552D, 0.0D, 0.09999999403953552D, 0.8999999761581421D, 0.800000011920929D, 0.8999999761581421D);

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return SAPLING_AABB;
  }

  @Override
  public void updateTick(World worldIn, BlockPos pos, IBlockState state, Random rand) {
    if (!worldIn.isRemote) {
      super.updateTick(worldIn, pos, state, rand);

      if (!worldIn.isAreaLoaded(pos, 1)) {
        return; // Forge: prevent loading unloaded chunks when checking neighbor's light
      }
      if (worldIn.getLightFromNeighbors(pos.up()) >= 9 && rand.nextInt(7) == 0) {
        this.grow(worldIn, pos, state, rand);
      }
    }
  }

  public void grow(World worldIn, BlockPos pos, IBlockState state, Random rand) {
    if (state.getValue(STAGE) == 0) {
      worldIn.setBlockState(pos, state.cycleProperty(STAGE), 4);
    } else {
      this.generateTree(worldIn, pos, state, rand);
    }
  }

  public void generateTree(World worldIn, BlockPos pos, IBlockState state, Random rand) {
    // Set to air, generate, then if it failed, put the sapling back.
    worldIn.setBlockState(pos, Blocks.AIR.getDefaultState(), 4);
    if (!new WorldGenBirchTree(true, false).generate(worldIn, rand, pos)) {
      worldIn.setBlockState(pos, state, 4);
    }
  }

  @Override
  public boolean canGrow(World worldIn, BlockPos pos, IBlockState state, boolean isClient) {
    return true;
  }

  @Override
  public boolean canUseBonemeal(World worldIn, Random rand, BlockPos pos, IBlockState state) {
    return (double) worldIn.rand.nextFloat() < 0.45D;
  }

  @Override
  public void grow(World worldIn, Random rand, BlockPos pos, IBlockState state) {
    this.grow(worldIn, pos, state, rand);
  }
}
