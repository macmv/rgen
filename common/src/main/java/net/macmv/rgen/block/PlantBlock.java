package net.macmv.rgen.block;

import net.minecraft.block.BlockBush;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;

public class PlantBlock extends BlockBush {
  protected static final AxisAlignedBB PLANT_AABB = new AxisAlignedBB(0.09999999403953552, 0.0, 0.09999999403953552, 0.8999999761581421, 0.800000011920929, 0.8999999761581421);

  public PlantBlock(BlockSettings settings) {
    super(settings.material);
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState p_185496_1_, IBlockAccess p_185496_2_, BlockPos p_185496_3_) {
    return PLANT_AABB;
  }
}
