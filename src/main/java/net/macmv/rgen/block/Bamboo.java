package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyBool;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class Bamboo extends Block {
  public static final PropertyEnum<Bamboo.Placement> PLACEMENT = PropertyEnum.create("placement", Bamboo.Placement.class);
  public static final PropertyBool HAS_LEAVES = PropertyBool.create("has_leaves");

  protected static final AxisAlignedBB BAMBOO_STANDARD = MathUtil.aabb(2.5, 0, 2.5, 5.5, 16, 5.5);
  protected static final AxisAlignedBB BAMBOO_X = MathUtil.aabb(10.5, 0, 2.5, 13.5, 16, 5.5);
  protected static final AxisAlignedBB BAMBOO_Z = MathUtil.aabb(2.5, 0, 10.5, 5.5, 16, 13.5);
  protected static final AxisAlignedBB BAMBOO_XZ = MathUtil.aabb(10.5, 0, 10.5, 13.5, 16, 13.5);

  public Bamboo() {
    super(Material.PLANTS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(PLACEMENT, Placement.STANDARD).withProperty(HAS_LEAVES, false));
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    switch (state.getValue(PLACEMENT)) {
      case X: return BAMBOO_X;
      case Z: return BAMBOO_Z;
      case XZ: return BAMBOO_XZ;
      default: return BAMBOO_STANDARD;
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
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(PLACEMENT, Bamboo.Placement.fromMeta(meta & 3));
    state = state.withProperty(HAS_LEAVES, (meta & 4) != 0);
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

  @Override
  public IBlockState getStateForPlacement(World world, BlockPos pos, EnumFacing facing, float hitX, float hitY, float hitZ, int meta, EntityLivingBase placer, EnumHand hand) {
    IBlockState belowBlock = world.getBlockState(pos.down());
    if (belowBlock.getBlock() instanceof Bamboo) {
      return this.getDefaultState().withProperty(PLACEMENT, belowBlock.getValue(PLACEMENT));
    } else {
      Placement placement = (hitX > 0.5 && hitZ > 0.5) ? Placement.XZ : (hitX > 0.5) ? Placement.X : (hitZ > 0.5) ? Placement.Z : Placement.STANDARD;
      return this.getDefaultState().withProperty(PLACEMENT, placement);
    }
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
        case X: return "x";
        case Z: return "z";
        case XZ: return "xz";
        default: return "standard";
      }
    }

    public static Bamboo.Placement fromMeta(int meta) {
      switch (meta) {
        case 1: return X;
        case 2: return Z;
        case 3: return XZ;
        default: return STANDARD;
      }
    }
  }
}
