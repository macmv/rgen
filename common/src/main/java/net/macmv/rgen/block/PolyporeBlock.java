package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.BlockHorizontal;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

public class PolyporeBlock extends Block {
  public static final PropertyEnum<PolyporeType> TYPE = PropertyEnum.create("type", PolyporeType.class);

  protected static final AxisAlignedBB NORTH_1_AABB = MathUtil.aabb(5, 8, 12, 14, 9, 16);
  protected static final AxisAlignedBB SOUTH_1_AABB = MathUtil.aabb(2, 8, 0, 11, 9, 4);
  protected static final AxisAlignedBB EAST_1_AABB = MathUtil.aabb(0, 8, 5, 4, 9, 14);
  protected static final AxisAlignedBB WEST_1_AABB = MathUtil.aabb(12, 8, 2, 16, 9, 11);

  protected static final AxisAlignedBB NORTH_2_AABB = MathUtil.aabb(2, 6, 12, 14, 10, 16);
  protected static final AxisAlignedBB SOUTH_2_AABB = MathUtil.aabb(2, 6, 0, 14, 10, 4);
  protected static final AxisAlignedBB EAST_2_AABB = MathUtil.aabb(0, 6, 2, 4, 10, 14);
  protected static final AxisAlignedBB WEST_2_AABB = MathUtil.aabb(12, 6, 2, 16, 10, 14);

  protected static final AxisAlignedBB NORTH_3_AABB = MathUtil.aabb(2, 7, 12, 14, 11, 16);
  protected static final AxisAlignedBB SOUTH_3_AABB = MathUtil.aabb(2, 7, 0, 14, 11, 4);
  protected static final AxisAlignedBB EAST_3_AABB = MathUtil.aabb(0, 7, 2, 4, 11, 14);
  protected static final AxisAlignedBB WEST_3_AABB = MathUtil.aabb(12, 7, 2, 16, 11, 14);

  public PolyporeBlock(BlockSettings settings) {
    super(settings.material);
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return NULL_AABB;
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    switch (state.getValue(BlockHorizontal.FACING)) {
      case NORTH: switch (state.getValue(TYPE)) {
        case ONE: return NORTH_1_AABB;
        case TWO: return NORTH_2_AABB;
        default: return NORTH_3_AABB;
      }
      case SOUTH: switch (state.getValue(TYPE)) {
        case ONE: return SOUTH_1_AABB;
        case TWO: return SOUTH_2_AABB;
        default: return SOUTH_3_AABB;
      }
      case EAST: switch (state.getValue(TYPE)) {
        case ONE: return EAST_1_AABB;
        case TWO: return EAST_2_AABB;
        default: return EAST_3_AABB;
      }
      default: switch (state.getValue(TYPE)) {
        case ONE: return WEST_1_AABB;
        case TWO: return WEST_2_AABB;
        default: return WEST_3_AABB;
      }
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

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, TYPE, BlockHorizontal.FACING);
  }

  public IBlockState getStateForPlacement(World worldIn, BlockPos pos, EnumFacing facing, float hitX, float hitY, float hitZ, int meta, EntityLivingBase placer) {
    return this.getStateFromMeta(meta).withProperty(BlockHorizontal.FACING, placer.getHorizontalFacing());
  }

  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(TYPE, PolyporeType.fromMeta(meta & 3));

    switch (meta & 12) {
      case 0: return state.withProperty(BlockHorizontal.FACING, EnumFacing.NORTH);
      case 4: return state.withProperty(BlockHorizontal.FACING, EnumFacing.SOUTH);
      case 8: return state.withProperty(BlockHorizontal.FACING, EnumFacing.EAST);
      default: return state.withProperty(BlockHorizontal.FACING, EnumFacing.WEST);
    }
  }

  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(TYPE).meta;

    switch (state.getValue(BlockHorizontal.FACING)) {
      case SOUTH: return meta | 4;
      case EAST: return meta | 8;
      case WEST: return meta | 12;
      default: return meta;
    }
  }

  public enum PolyporeType implements IStringSerializable {
    ONE(0),
    TWO(1),
    THREE(2);

    public final int meta;

    PolyporeType(int meta) {
      this.meta = meta;
    }

    @Override
    public String getName() {
      return Integer.toString(meta + 1);
    }

    public static PolyporeType fromMeta(int meta) {
      switch (meta) {
        case 0: return ONE;
        case 1: return TWO;
        default: return THREE;
      }
    }
  }
}
