package net.macmv.rgen.block;


import net.minecraft.block.Block;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class DeadlyHangingVines extends Block {
  public static final PropertyEnum<Type> TYPE = PropertyEnum.create("type", Type.class);

  public DeadlyHangingVines(BlockSettings settings) {
    super(settings.material);
    this.setDefaultState(this.blockState.getBaseState().withProperty(TYPE, Type.BOTTOM));
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
  @Override
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT_MIPPED;
  }

  @Override
  public boolean isPassable(IBlockAccess worldIn, BlockPos pos) {
    return true; // Makes the block passable
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
    return NULL_AABB; // No collision box
  }


  @Override
  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
    if (entityIn instanceof EntityLivingBase) {
      entityIn.motionY = 0.2; // Controls climb speed
      entityIn.fallDistance = 0.0F; // Prevents fall damage
    }
  }


  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, TYPE);
  }

  @Override
  public IBlockState getActualState(IBlockState state, IBlockAccess world, BlockPos pos) {
    IBlockState belowState = world.getBlockState(pos.down());
    if (belowState.getBlock() == this) {
      return state.withProperty(TYPE, Type.STANDARD);
    } else {
      return state.withProperty(TYPE, Type.BOTTOM);
    }
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(TYPE) == Type.STANDARD ? 1 : 0;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(TYPE, meta == 1 ? Type.STANDARD : Type.BOTTOM);
  }

  public enum Type implements IStringSerializable {
    BOTTOM("bottom"),
    STANDARD("standard");

    private final String name;

    Type(String name) {
      this.name = name;
    }

    @Override
    public String getName() {
      return this.name;
    }
  }
}
