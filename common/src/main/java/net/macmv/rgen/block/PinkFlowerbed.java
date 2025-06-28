package net.macmv.rgen.block;


import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.properties.PropertyDirection;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class PinkFlowerbed extends Block {
  public static final PropertyDirection ROTATION = PropertyDirection.create("rotation", EnumFacing.Plane.HORIZONTAL);
  public static final PropertyInteger COUNT = PropertyInteger.create("count", 1, 4);
  protected static final AxisAlignedBB PLANT_AABB = MathUtil.aabb(0, 0, 0, 16, 3, 16);

  public PinkFlowerbed(BlockSettings settings) {
    super(settings.material);
    this.setDefaultState(this.blockState.getBaseState().withProperty(ROTATION, EnumFacing.NORTH).withProperty(COUNT, 1));
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
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, ROTATION, COUNT);
  }


  public IBlockState onBlockPlacedBy(World worldIn, BlockPos pos, EnumFacing facing, float hitX, float hitY, float hitZ, int meta, EntityLivingBase placer) {
    // Randomly select a count between 1 and 4
    int randomCount = 1 + RANDOM.nextInt(4);

    // Randomly select a rotation direction
    EnumFacing randomRotation = EnumFacing.HORIZONTALS[RANDOM.nextInt(EnumFacing.HORIZONTALS.length)];

    return this.getDefaultState().withProperty(ROTATION, randomRotation).withProperty(COUNT, randomCount);
  }

  public AxisAlignedBB getBoundingBox(IBlockState p_185496_1_, IBlockAccess p_185496_2_, BlockPos p_185496_3_) {
    return PLANT_AABB;
  }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    EnumFacing facing = EnumFacing.getHorizontal(meta & 3);
    int count = (meta >> 2) + 1;
    return this.getDefaultState().withProperty(ROTATION, facing).withProperty(COUNT, count);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int facing = state.getValue(ROTATION).getHorizontalIndex();
    int count = state.getValue(COUNT) - 1;
    return facing | (count << 2);
  }

  @Override
  public int damageDropped(IBlockState state) {
    return getMetaFromState(state);
  }
}

