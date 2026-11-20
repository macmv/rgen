package net.macmv.rgen.block;

import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
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

public class RedCactus extends BlockBush implements IGrowable {
  public static final PropertyInteger AGE = PropertyInteger.create("age", 0, 3);
  private static final AxisAlignedBB[] STAGE_BOUNDS = {
      new AxisAlignedBB(0.2, 0, 0.2, 0.8, 0.5, 0.8),
      new AxisAlignedBB(0.2, 0, 0.2, 0.8, 0.5, 0.8),
      new AxisAlignedBB(0.2, 0, 0.2, 0.8, 1.0, 0.8),
      new AxisAlignedBB(0.2, 0, 0.2, 0.8, 1.0, 0.8)
  };

  public RedCactus(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
    setDefaultState(blockState.getBaseState().withProperty(AGE, 0));
    setTickRandomly(true);
  }

  @Override public boolean canBlockStay(World world, BlockPos pos, IBlockState state) { return world.getBlockState(pos.down()).getBlock() == Blocks.SAND; }

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

  @Override public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return STAGE_BOUNDS[state.getValue(AGE)].offset(state.getOffset(world, pos)); }
  @Override public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return NULL_AABB; }
  @Override public Block.EnumOffsetType getOffsetType() { return Block.EnumOffsetType.XZ; }
  @Override public void onEntityCollidedWithBlock(World world, BlockPos pos, IBlockState state, Entity entity) {
    AxisAlignedBB damageBounds = getBoundingBox(state, world, pos).offset(pos);
    if (entity.getEntityBoundingBox().intersects(damageBounds)) {
      entity.attackEntityFrom(DamageSource.CACTUS, 1.0F);
    }
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new ArrayList<>();
    if (state.getValue(AGE) == 3) drops.add(new ItemStack(RItems.RED_CACTUS_FRUIT, 1 + new Random().nextInt(3)));
    return drops;
  }

  @Override public IBlockState getStateFromMeta(int meta) { return getDefaultState().withProperty(AGE, meta & 3); }
  @Override public int getMetaFromState(IBlockState state) { return state.getValue(AGE); }
  @Override protected BlockStateContainer createBlockState() { return new BlockStateContainer(this, AGE); }
  @Override public boolean isOpaqueCube(IBlockState state) { return false; }
  @Override public boolean isFullCube(IBlockState state) { return false; }
  @SideOnly(Side.CLIENT) public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }
}
