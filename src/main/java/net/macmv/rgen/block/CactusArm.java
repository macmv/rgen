package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.BlockStairs;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class CactusArm extends BlockStairs {

  protected static final AxisAlignedBB AABB_ARM = MathUtil.aabb(5, 1, 5, 11, 15, 11);
  public CactusArm(Material material) {
    super(Blocks.CACTUS.getDefaultState());
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {return AABB_ARM;}

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {return AABB_ARM;}

  /*public BlockRenderLayer getBlockLayer()
  {
    return BlockRenderLayer.CUTOUT;
  }*/
  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer()
  {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }
  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn)
  {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }
}
