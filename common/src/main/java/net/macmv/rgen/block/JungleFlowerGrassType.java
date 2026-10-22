package net.macmv.rgen.block;

import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;

public class JungleFlowerGrassType extends JungleFlower {

    private static final AxisAlignedBB GRASS_AABB = new AxisAlignedBB(
            0.1D, 0.0D, 0.1D,
            0.9D, 0.8D, 0.9D
    );

    public JungleFlowerGrassType(BlockSettings settings) {
        super(settings);
    }

    @Override
    public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
        return GRASS_AABB;
    }

    @Override
    public EnumOffsetType getOffsetType() {
        return EnumOffsetType.XZ;
    }
}
