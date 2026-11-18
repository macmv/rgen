package net.macmv.rgen.block;

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

import java.util.Collections;
import java.util.List;
import java.util.Random;

public class JuvenileGreenCactus extends Block implements IGrowable {
  private static final AxisAlignedBB[] STAGE_BOUNDS = {
      new AxisAlignedBB(5.0 / 16.0, 0, 5.0 / 16.0, 11.0 / 16.0, 5.0 / 16.0, 11.0 / 16.0),
      new AxisAlignedBB(4.0 / 16.0, 0, 4.0 / 16.0, 12.0 / 16.0, 6.0 / 16.0, 12.0 / 16.0),
      new AxisAlignedBB(3.0 / 16.0, 0, 3.0 / 16.0, 13.0 / 16.0, 10.0 / 16.0, 13.0 / 16.0),
      new AxisAlignedBB(4.0 / 16.0, 0, 4.0 / 16.0, 12.0 / 16.0, 14.0 / 16.0, 12.0 / 16.0)
  };
  public static final PropertyInteger STAGE = PropertyInteger.create("stage", 0, 3);

  public JuvenileGreenCactus(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
    this.setDefaultState(this.blockState.getBaseState().withProperty(STAGE, 0)); // Default to stage 0
    this.setTickRandomly(true); // Enable random ticks
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return STAGE_BOUNDS[state.getValue(STAGE)];
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return STAGE_BOUNDS[state.getValue(STAGE)];
  }

  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
    AxisAlignedBB bounds = getBoundingBox(state, worldIn, pos).offset(pos);
    AxisAlignedBB damageBounds = new AxisAlignedBB(
        bounds.minX + 1.0 / 16.0, bounds.minY + 1.0 / 16.0, bounds.minZ + 1.0 / 16.0,
        bounds.maxX - 1.0 / 16.0, bounds.maxY - 1.0 / 16.0, bounds.maxZ - 1.0 / 16.0
    );
    if (entityIn.getEntityBoundingBox().intersects(damageBounds)) {
      entityIn.attackEntityFrom(DamageSource.CACTUS, 0.5F);
    }
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, STAGE); // Include the custom stage property
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(STAGE, meta & 3);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(STAGE);
  }

  @Override
  public void updateTick(World worldIn, BlockPos pos, IBlockState state, Random rand) {
    super.updateTick(worldIn, pos, state, rand);

    // TODO: Replace stage 3 with the multi-block green cactus once trunk growth is implemented.
    int age = state.getValue(STAGE);

    if (!worldIn.isAreaLoaded(pos, 1)) return; // Check if area is loaded
    if (age < 3) {
      // Grow to the next stage with a random chance
      if (rand.nextInt(5) == 0) { // 20% chance to grow each tick
        worldIn.setBlockState(pos, state.withProperty(STAGE, age + 1), 2);
      }
    }
  }

  @Override
  public boolean canGrow(World world, BlockPos pos, IBlockState state, boolean isClient) {
    return state.getValue(STAGE) < 3;
  }

  @Override
  public boolean canUseBonemeal(World world, Random rand, BlockPos pos, IBlockState state) {
    return rand.nextFloat() < 0.45F;
  }

  @Override
  public void grow(World world, Random rand, BlockPos pos, IBlockState state) {
    int stage = state.getValue(STAGE);
    if (stage < 3) {
      world.setBlockState(pos, state.withProperty(STAGE, stage + 1), 2);
    }
  }

  @Override
  public boolean canPlaceBlockAt(World worldIn, BlockPos pos) {
    Block blockBelow = worldIn.getBlockState(pos.down()).getBlock();
    return blockBelow == Blocks.SAND; // Can only be placed on dirt or grass
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  private boolean isValidGround(World worldIn, BlockPos pos) {
    Block blockBelow = worldIn.getBlockState(pos.down()).getBlock();
    return blockBelow == Blocks.SAND;
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
    return Collections.emptyList();
  }
}
