package net.macmv.rgen.block;

import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;

public class JungleFlowerFlowerType extends JungleFlower {

    private static final AxisAlignedBB ALLIUM_AABB = new AxisAlignedBB(
            0.125D, 0.0D, 0.125D,
            0.875D, 0.8125D, 0.875D
    );

    public JungleFlowerFlowerType(BlockSettings settings) {
        super(settings);
    }

    @Override
    public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
        return ALLIUM_AABB;
    }

    @Override
    public EnumOffsetType getOffsetType() {
        return EnumOffsetType.XZ;
    }
}
