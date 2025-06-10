package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.BlockPlanks;
import net.minecraft.block.BlockSapling;
import net.minecraft.block.material.Material;
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

import java.util.List;
import java.util.Random;

public class JuvenileGreenCactus extends Block {
  protected static final AxisAlignedBB AABB_J_CACTUS = MathUtil.aabb(5, 0, 5, 11, 16, 11);
  // Define the custom stage property
  public static final PropertyInteger STAGE = PropertyInteger.create("stage", 0, 4);

  public JuvenileGreenCactus() {
    super(Material.PLANTS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(STAGE, 0)); // Default to stage 0
    this.setTickRandomly(true); // Enable random ticks
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return AABB_J_CACTUS;
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return AABB_J_CACTUS;
  }

  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 0.5F);
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, STAGE); // Include the custom stage property
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(STAGE, meta);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(STAGE);
  }

  @Override
  public void updateTick(World worldIn, BlockPos pos, IBlockState state, Random rand) {
    super.updateTick(worldIn, pos, state, rand);

    int age = state.getValue(STAGE);

    if (!worldIn.isAreaLoaded(pos, 1)) return; // Check if area is loaded
    if (age < 4) {
      // Grow to the next stage with a random chance
      if (rand.nextInt(5) == 0) { // 20% chance to grow each tick
        worldIn.setBlockState(pos, state.withProperty(STAGE, age + 1), 2);
      }
    } else {
      // Replace this block with an oak sapling when fully grown
      worldIn.setBlockState(pos, Blocks.SAPLING.getDefaultState().withProperty(BlockSapling.TYPE, BlockPlanks.EnumType.OAK), 2);
      Blocks.SAPLING.updateTick(worldIn, pos, Blocks.SAPLING.getDefaultState(), rand); // Make the oak tree grow immediately
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