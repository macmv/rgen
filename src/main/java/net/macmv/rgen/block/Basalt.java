package net.macmv.rgen.block;

import net.minecraft.block.BlockDoublePlant;
import net.minecraft.block.BlockRotatedPillar;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.IBlockState;
import net.minecraft.init.Blocks;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraftforge.common.EnumPlantType;
import net.minecraftforge.common.IPlantable;

public class Basalt extends BlockRotatedPillar {
    public Basalt() {
        super(Material.ROCK);
        this.setHardness(1.25F);
        this.setSoundType(SoundType.STONE);
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