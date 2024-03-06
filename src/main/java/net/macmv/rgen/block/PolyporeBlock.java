package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.BlockHorizontal;
import net.minecraft.block.material.Material;
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
  protected static final AxisAlignedBB POLYPORE_AABB = MathUtil.aabb(0, 0, 0, 15, 15, 15);

  public PolyporeBlock() {
    super(Material.PLANTS);
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return POLYPORE_AABB;
  }


  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return POLYPORE_AABB;
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
      case 0:
        return state.withProperty(BlockHorizontal.FACING, EnumFacing.NORTH);
      case 4:
        return state.withProperty(BlockHorizontal.FACING, EnumFacing.SOUTH);
      case 8:
        return state.withProperty(BlockHorizontal.FACING, EnumFacing.EAST);
      default:
        return state.withProperty(BlockHorizontal.FACING, EnumFacing.WEST);
    }
  }

  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(TYPE).meta;

    switch (state.getValue(BlockHorizontal.FACING)) {
      case SOUTH:
        return meta | 4;
      case EAST:
        return meta | 8;
      case WEST:
        return meta | 12;
      default:
        return meta;
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
      return Integer.toString(meta);
    }

    public static PolyporeType fromMeta(int meta) {
      switch (meta) {
        case 0:
          return ONE;
        case 1:
          return TWO;
        default:
          return THREE;
      }
    }
  }
}
