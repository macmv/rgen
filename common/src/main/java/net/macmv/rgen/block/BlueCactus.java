package net.macmv.rgen.block;

import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.IGrowable;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Vec3d;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class BlueCactus extends Block implements IGrowable {
  public static final PropertyInteger AGE = PropertyInteger.create("age", 0, 3);
  public static final PropertyEnum<Location> LOCATION = PropertyEnum.create("location", Location.class);
  private static final double[] WIDTHS = {0.25, 0.375, 0.5, 0.625};
  private static final double[] HEIGHTS = {4.0 / 16.0, 6.0 / 16.0, 6.0 / 16.0, 8.0 / 16.0};

  public BlueCactus(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
    this.setDefaultState(this.blockState.getBaseState().withProperty(AGE, 0).withProperty(LOCATION, Location.CENTER));
    this.setTickRandomly(true);
  }

  @Override
  public void updateTick(World world, BlockPos pos, IBlockState state, Random rand) {
    if (world.isAreaLoaded(pos, 1) && state.getValue(AGE) < 3 && rand.nextInt(5) == 0) {
      world.setBlockState(pos, state.withProperty(AGE, state.getValue(AGE) + 1), 2);
    }
  }

  @Override
  public boolean canGrow(World world, BlockPos pos, IBlockState state, boolean isClient) {
    return state.getValue(AGE) < 3;
  }

  @Override
  public boolean canUseBonemeal(World world, Random rand, BlockPos pos, IBlockState state) {
    return rand.nextFloat() < 0.45F;
  }

  @Override
  public void grow(World world, Random rand, BlockPos pos, IBlockState state) {
    if (state.getValue(AGE) < 3) world.setBlockState(pos, state.withProperty(AGE, state.getValue(AGE) + 1), 2);
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return bounds(state).offset(state.getOffset(world, pos));
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return getBoundingBox(state, world, pos);
  }

  private AxisAlignedBB bounds(IBlockState state) {
    int age = state.getValue(AGE);
    double half = WIDTHS[age] / 2.0;
    return new AxisAlignedBB(0.5 - half, 0, 0.5 - half, 0.5 + half, HEIGHTS[age], 0.5 + half);
  }

  @Override
  public Vec3d getOffset(IBlockState state, IBlockAccess world, BlockPos pos) {
    switch (state.getValue(LOCATION)) {
      case NORTH_WEST: return new Vec3d(-0.1875, 0, -0.1875);
      case NORTH_EAST: return new Vec3d(0.1875, 0, -0.1875);
      case SOUTH_WEST: return new Vec3d(-0.1875, 0, 0.1875);
      case CENTER:
      default: return Vec3d.ZERO;
    }
  }

  @Override
  public void onEntityCollidedWithBlock(World world, BlockPos pos, IBlockState state, Entity entity) {
    AxisAlignedBB bounds = getBoundingBox(state, world, pos).offset(pos);
    AxisAlignedBB damageBounds = new AxisAlignedBB(
        bounds.minX + 1.0 / 16.0, bounds.minY + 1.0 / 16.0, bounds.minZ + 1.0 / 16.0,
        bounds.maxX - 1.0 / 16.0, bounds.maxY - 1.0 / 16.0, bounds.maxZ - 1.0 / 16.0
    );
    if (entity.getEntityBoundingBox().intersects(damageBounds)) {
      entity.attackEntityFrom(DamageSource.CACTUS, 1.0F);
    }
  }

  private boolean hasValidGround(World world, BlockPos pos) {
    return world.getBlockState(pos.down()).getBlock() == Blocks.SAND;
  }

  private void removeIfUnsupported(World world, BlockPos pos, IBlockState state) {
    if (!hasValidGround(world, pos)) {
      for (ItemStack drop : getDrops(world, pos, state, 0)) spawnAsEntity(world, pos, drop);
      world.setBlockToAir(pos);
    }
  }

  @Override
  public void onBlockAdded(World world, BlockPos pos, IBlockState state) {
    removeIfUnsupported(world, pos, state);
  }

  @Override
  public void neighborChanged(IBlockState state, World world, BlockPos pos, Block block, BlockPos fromPos) {
    removeIfUnsupported(world, pos, state);
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new ArrayList<>();
    if (state.getValue(AGE) == 3) drops.add(new ItemStack(RItems.BLUE_CACTUS_FRUIT, 1 + new Random().nextInt(3)));
    return drops;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return getDefaultState().withProperty(AGE, (meta >> 2) & 3).withProperty(LOCATION, Location.fromMeta(meta & 3));
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return (state.getValue(AGE) << 2) | state.getValue(LOCATION).meta;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, AGE, LOCATION);
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) { return false; }

  @Override
  public boolean isFullCube(IBlockState state) { return false; }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }

  public enum Location implements IStringSerializable {
    NORTH_WEST(0), NORTH_EAST(1), SOUTH_WEST(2), CENTER(3);

    public final int meta;
    Location(int meta) { this.meta = meta; }
    @Override public String getName() { return name().toLowerCase(); }
    public static Location fromMeta(int meta) { return values()[meta & 3]; }
  }
}
