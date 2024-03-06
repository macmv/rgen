package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockStone;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.init.Blocks;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraftforge.common.EnumPlantType;
import net.minecraftforge.common.IPlantable;

public class RGenPlantableStone extends Block {

  public RGenPlantableStone() {
    super(Material.ROCK);
    this.setCreativeTab(CreativeTabs.DECORATIONS);
  }

  @Override
  public boolean canSustainPlant(IBlockState state, IBlockAccess world, BlockPos pos, EnumFacing direction, IPlantable plantable) {
    IBlockState plant = plantable.getPlant(world, pos.offset(direction));
    net.minecraftforge.common.EnumPlantType plantType = plantable.getPlantType(world, pos.offset(direction));

    if (plant.getBlock() == Blocks.SAPLING) {
      return true;
    }

    return plantType == EnumPlantType.Plains;
  }
}
