package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class JungleFlower extends Block {
    public static final PropertyInteger VARIANT = PropertyInteger.create("variant", 1, 12);
    protected static final AxisAlignedBB FLOWER_AABB = new AxisAlignedBB(0, 0, 0, 1, 0.75, 1);

    public JungleFlower(BlockSettings settings) {
        super(settings.material);
        this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, 1));
        this.setTickRandomly(true);
        this.setSoundType(SoundType.PLANT);
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
        return new BlockStateContainer(this, VARIANT);
    }

    @Override
    public IBlockState getStateFromMeta(int meta) {
        return this.getDefaultState().withProperty(VARIANT, meta + 1);
    }

    @Override
    public int getMetaFromState(IBlockState state) {
        return state.getValue(VARIANT) - 1;
    }

    @Override
    public int damageDropped(IBlockState state) {
        return getMetaFromState(state);
    }

    @Override
    public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
        return FLOWER_AABB;
    }

    @SideOnly(Side.CLIENT)
    @Override
    public BlockRenderLayer getBlockLayer() {
        return BlockRenderLayer.CUTOUT;
    }
}
