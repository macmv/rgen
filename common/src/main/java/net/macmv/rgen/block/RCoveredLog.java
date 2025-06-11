package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockLog;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraftforge.common.EnumPlantType;
import net.minecraftforge.common.IPlantable;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;


public class RCoveredLog extends BlockLog {
  //public static final PropertyEnum<BlockLog.EnumAxis> LOG_AXIS = PropertyEnum.create("axis", BlockLog.EnumAxis.class);

  private final boolean growableSurface;

  public RCoveredLog(boolean growableSurface) {
    super();
    this.growableSurface = growableSurface;
    this.setHardness(3.0F);
    this.setResistance(5.0F);
    this.setSoundType(SoundType.WOOD);
    this.setDefaultState(this.blockState.getBaseState().withProperty(LOG_AXIS, BlockLog.EnumAxis.Y));
  }

  /*
  public BlockRenderLayer getRenderLayer() {
    return BlockRenderLayer.CUTOUT_MIPPED;
  } */

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer()
  {
    return BlockRenderLayer.CUTOUT_MIPPED;
  }

  @Override
  public boolean canSustainPlant(IBlockState state, IBlockAccess world, BlockPos pos, EnumFacing direction, IPlantable plantable) {
    if (!growableSurface) {
      return false;  // Block cannot sustain plants if growableSurface is false
    }
    net.minecraftforge.common.EnumPlantType plantType = plantable.getPlantType(world, pos.offset(direction));
    return plantType == EnumPlantType.Plains;  // Only allow Plains-type plants if growableSurface is true
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    switch (meta & 3) {
      case 1: return this.getDefaultState().withProperty(LOG_AXIS, BlockLog.EnumAxis.X);
      case 2: return this.getDefaultState().withProperty(LOG_AXIS, BlockLog.EnumAxis.Z);
      default: return this.getDefaultState().withProperty(LOG_AXIS, BlockLog.EnumAxis.Y);
    }
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    switch (state.getValue(LOG_AXIS)) {
      case X: return 1;
      case Z: return 2;
      default: return 0;
    }
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, LOG_AXIS);
  }
}
