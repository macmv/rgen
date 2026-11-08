package net.macmv.rgen.block;

import net.minecraft.block.state.IBlockState;
import net.minecraft.init.Blocks;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class YellowWildflowerShrub extends JungleFlowerGrassType {
  public YellowWildflowerShrub(BlockSettings settings) {
    super(settings);
  }

  @Override
  public boolean canPlaceBlockAt(World world, BlockPos pos) {
    IBlockState soil = world.getBlockState(pos.down());
    if (soil.getBlock() == Blocks.SAND) {
      return world.getBlockState(pos).getBlock().isReplaceable(world, pos);
    }
    return super.canPlaceBlockAt(world, pos);
  }
}
