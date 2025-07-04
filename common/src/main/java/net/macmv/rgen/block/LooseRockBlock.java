package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.BlockHorizontal;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

public class LooseRockBlock extends Block {
  public static final PropertyEnum<RockSize> SIZE = PropertyEnum.create("size", RockSize.class);

  protected static final AxisAlignedBB SMALL_COLLISION_AABB = MathUtil.aabb(6, 0, 6, 10, 1, 10);
  protected static final AxisAlignedBB MEDIUM_COLLISION_AABB = MathUtil.aabb(5, 0, 5, 11, 1, 11);
  protected static final AxisAlignedBB LARGE_COLLISION_AABB = MathUtil.aabb(4, 0, 4, 12, 1, 12);
  protected static final AxisAlignedBB SMALL_SELECTED_AABB = MathUtil.aabb(6, 0, 6, 10, 2, 10);
  protected static final AxisAlignedBB MEDIUM_SELECTED_NORTH_AABB = MathUtil.aabb(5, 0, 5, 11, 2, 12);
  protected static final AxisAlignedBB MEDIUM_SELECTED_SOUTH_AABB = MathUtil.aabb(5, 0, 4, 11, 2, 11);
  protected static final AxisAlignedBB MEDIUM_SELECTED_EAST_AABB = MathUtil.aabb(4, 0, 5, 11, 2, 11);
  protected static final AxisAlignedBB MEDIUM_SELECTED_WEST_AABB = MathUtil.aabb(5, 0, 5, 12, 2, 11);
  protected static final AxisAlignedBB LARGE_SELECTED_NORTH_AABB = MathUtil.aabb(4, 0, 3, 12, 2, 13);
  protected static final AxisAlignedBB LARGE_SELECTED_SOUTH_AABB = MathUtil.aabb(4, 0, 3, 12, 2, 13);
  protected static final AxisAlignedBB LARGE_SELECTED_EAST_AABB = MathUtil.aabb(3, 0, 4, 13, 2, 12);
  protected static final AxisAlignedBB LARGE_SELECTED_WEST_AABB = MathUtil.aabb(3, 0, 4, 13, 2, 12);

  public LooseRockBlock(BlockSettings settings) {
    super(settings.material);
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    switch (state.getValue(SIZE)) {
      case SMALL: return SMALL_COLLISION_AABB;
      case MEDIUM: return MEDIUM_COLLISION_AABB;
      case LARGE: return LARGE_COLLISION_AABB;
      default: return NULL_AABB;
    }
  }


  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    switch (state.getValue(SIZE)) {
      case SMALL: return SMALL_SELECTED_AABB;
      case MEDIUM: switch (state.getValue(BlockHorizontal.FACING)) {
        case NORTH: return MEDIUM_SELECTED_NORTH_AABB;
        case SOUTH: return MEDIUM_SELECTED_SOUTH_AABB;
        case EAST: return MEDIUM_SELECTED_EAST_AABB;
        case WEST: return MEDIUM_SELECTED_WEST_AABB;
      }
      case LARGE: switch (state.getValue(BlockHorizontal.FACING)) {
        case NORTH: return LARGE_SELECTED_NORTH_AABB;
        case SOUTH: return LARGE_SELECTED_SOUTH_AABB;
        case EAST: return LARGE_SELECTED_EAST_AABB;
        case WEST: return LARGE_SELECTED_WEST_AABB;
      }
      default: return NULL_AABB;
    }
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  @Override
  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this, 1, RockSize.SMALL.meta));
    items.add(new ItemStack(this, 1, RockSize.MEDIUM.meta));
    items.add(new ItemStack(this, 1, RockSize.LARGE.meta));
  }

  // Returns the item metadata when dropping the item for this block, and when middle clicking.
  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(SIZE).meta;
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, SIZE, BlockHorizontal.FACING);
  }

  public IBlockState getStateForPlacement(World worldIn, BlockPos pos, EnumFacing facing, float hitX, float hitY, float hitZ, int meta, EntityLivingBase placer) {
    return this.getStateFromMeta(meta).withProperty(BlockHorizontal.FACING, placer.getHorizontalFacing());
  }

  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(SIZE, RockSize.fromMeta(meta & 3));

    switch (meta & 12) {
      case 0: return state.withProperty(BlockHorizontal.FACING, EnumFacing.NORTH);
      case 4: return state.withProperty(BlockHorizontal.FACING, EnumFacing.SOUTH);
      case 8: return state.withProperty(BlockHorizontal.FACING, EnumFacing.EAST);
      default: return state.withProperty(BlockHorizontal.FACING, EnumFacing.WEST);
    }
  }

  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(SIZE).meta;

    switch (state.getValue(BlockHorizontal.FACING)) {
      case SOUTH: return meta | 4;
      case EAST: return meta | 8;
      case WEST: return meta | 12;
      default: return meta;
    }
  }

  public enum RockSize implements IStringSerializable {
    SMALL(0, "small"),
    MEDIUM(1, "medium"),
    LARGE(2, "large");

    public final int meta;
    public final String name;

    RockSize(int meta, String name) {
      this.meta = meta;
      this.name = name;
    }

    @Override
    public String getName() {
      return name;
    }

    public static RockSize fromMeta(int meta) {
      switch (meta) {
        case 0: return SMALL;
        case 1: return MEDIUM;
        default: return LARGE;
      }
    }
  }
}
