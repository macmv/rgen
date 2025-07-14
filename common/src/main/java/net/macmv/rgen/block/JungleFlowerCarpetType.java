package net.macmv.rgen.block;

import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.MathHelper;
import net.minecraft.util.math.Vec3d;
import net.minecraft.world.IBlockAccess;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class JungleFlowerCarpetType extends JungleFlower {

    // Shrunk by 1px (1/16 = 0.0625) on each side to allow 1px offset
    private static final AxisAlignedBB SHRUNK_CARPET_AABB = new AxisAlignedBB(
            0.03125D, 0.0D, 0.03125D,
            0.96875D, 0.0625D, 0.96875D
    );

    public JungleFlowerCarpetType(BlockSettings settings) {
        super(settings);
    }

    @Override
    public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
        return SHRUNK_CARPET_AABB;
    }

    // Custom subtle offset — replaces getOffsetType()
    @Override
    @SideOnly(Side.CLIENT)
    public Vec3d getOffset(IBlockState state, IBlockAccess world, BlockPos pos) {
        long seed = MathHelper.getCoordinateRandom(pos.getX(), 0, pos.getZ());
        double x = ((double)((int)(seed >> 16 & 15L)) / 15.0 - 0.5) * 0.1; // ±0.05
        double z = ((double)((int)(seed >> 24 & 15L)) / 15.0 - 0.5) * 0.1;
        return new Vec3d(x, 0.0D, z);
    }
}
