package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.BlockHorizontal;
import net.minecraft.block.BlockLog;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyBool;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.client.renderer.EnumFaceDirection;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.common.EnumPlantType;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class Bamboo extends Block {


  public static final PropertyEnum<Bamboo.Placement> PLACEMENT = PropertyEnum.create("placement", Bamboo.Placement.class);
  public static final PropertyBool HAS_LEAVES = PropertyBool.create("has_leaves");

  protected static final AxisAlignedBB COLLIDE_BAMBOO_STANDARD = MathUtil.aabb(3, 0, 3, 5, 16, 5);
  protected static final AxisAlignedBB COLLIDE_BAMBOO_X = MathUtil.aabb(11, 0, 3, 13, 16, 5);
  protected static final AxisAlignedBB COLLIDE_BAMBOO_Z = MathUtil.aabb(3, 0, 11, 5, 16, 13);
  protected static final AxisAlignedBB COLLIDE_BAMBOO_XZ = MathUtil.aabb(11, 0, 11, 13, 16, 13);

  protected static final AxisAlignedBB BOUND_BAMBOO_STANDARD = MathUtil.aabb(2, 0, 2, 6, 16, 6);
  protected static final AxisAlignedBB BOUND_BAMBOO_X = MathUtil.aabb(10, 0, 2, 14, 16, 6);
  protected static final AxisAlignedBB BOUND_BAMBOO_Z = MathUtil.aabb(2, 0, 10, 6, 16, 14);
  protected static final AxisAlignedBB BOUND_BAMBOO_XZ = MathUtil.aabb(10, 0, 10, 14, 16, 14);

  public Bamboo() {
    super(Material.PLANTS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(PLACEMENT, Placement.STANDARD).withProperty(HAS_LEAVES, false));
  }

  @Override
  public IBlockState getStateForPlacement(World world, BlockPos pos, EnumFacing facing, float x, float y, float z, int meta, EntityLivingBase placer) {
    //return super.getStateForPlacement(world, pos, facing, x, y, z, meta, placer);
    System.out.println(pos.toString());
    new BlockPos(0,-1,0);
    IBlockState belowBlock = world.getBlockState(pos.down());
    if (belowBlock.getBlock() instanceof Bamboo) {
      return this.getDefaultState().withProperty(PLACEMENT, belowBlock.getValue(PLACEMENT));
    }else{
      return this.getDefaultState();
    }

  }
  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    switch (state.getValue(PLACEMENT)) {
      case X:
        return COLLIDE_BAMBOO_X;
      case Z:
        return COLLIDE_BAMBOO_Z;
      case XZ:
        return COLLIDE_BAMBOO_XZ;
      default:
        return COLLIDE_BAMBOO_STANDARD;
    }
  }


  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    switch (state.getValue(PLACEMENT)) {
      case X:
        return BOUND_BAMBOO_X;
      case Z:
        return BOUND_BAMBOO_Z;
      case XZ:
        return BOUND_BAMBOO_XZ;
      default:
        return BOUND_BAMBOO_STANDARD;
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

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer()
  {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(PLACEMENT, Bamboo.Placement.fromMeta(meta & 3));
    state = state.withProperty(HAS_LEAVES, (meta &4) != 0);
    return state;
  }

  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(PLACEMENT).meta;
    if (state.getValue(HAS_LEAVES)) {
      return meta | 4;
    } else {
      return meta;
    }
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, PLACEMENT, HAS_LEAVES);
  }

  public static enum Placement implements IStringSerializable {
    STANDARD(0),
    X(1),
    Z(2),
    XZ(3);


    public final int meta;

    Placement(int meta) {
      this.meta = meta;
    }

    @Override
    public String getName() {
      switch (this) {

        case X:
          return "x";
        case Z:
          return "z";
        case XZ:
          return "xz";
        default:
          return "standard";
      }
    }

    public static Bamboo.Placement fromMeta(int meta) {
      switch (meta) {

        case 1:
           return X;
        case 2:
           return Z;
        case 3:
          return XZ;
        default:
          return STANDARD;
      }
    }
  }

}