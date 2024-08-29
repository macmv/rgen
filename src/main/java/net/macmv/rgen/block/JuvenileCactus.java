package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import javax.annotation.Nullable;
import java.util.List;
import java.util.Random;

public class JuvenileCactus extends Block {
    public static final PropertyEnum<Color> COLOR = PropertyEnum.create("color", Color.class);
    public static final PropertyEnum<Age> AGE = PropertyEnum.create("age", Age.class);

    protected static final AxisAlignedBB AABB_SMALL = MathUtil.aabb(4, 0, 4, 12, 8, 12);
    protected static final AxisAlignedBB AABB_MEDIUM = MathUtil.aabb(1, 0, 1, 15, 16, 15);

    public JuvenileCactus() {
        super(Material.PLANTS);
        this.setHardness(0.4f);
        this.setDefaultState(this.blockState.getBaseState().withProperty(COLOR, Color.GREEN).withProperty(AGE, Age.ZERO));

    }

    @Override
    public boolean isOpaqueCube(IBlockState state) {
        return false;
    }

    @SideOnly(Side.CLIENT)
    public BlockRenderLayer getBlockLayer() {
        return BlockRenderLayer.CUTOUT;
    }

    @Override
    public boolean isFullCube(IBlockState state) {
        return false;
    }

    public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
        entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
    }

    private boolean isValidGround(World worldIn, BlockPos pos) {
        Block blockBelow = worldIn.getBlockState(pos.down()).getBlock();
        return blockBelow == Blocks.SAND || blockBelow == this;
    }

    private void destroyBlockWithDrops(World worldIn, BlockPos pos, IBlockState state) {
        List<ItemStack> drops = getDrops(worldIn, pos, state, 0);
        for (ItemStack drop : drops) {
            spawnAsEntity(worldIn, pos, drop);
        }
        worldIn.setBlockToAir(pos);
        worldIn.playEvent(2001, pos, Block.getStateId(state));
    }

    @Override
    public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
        if (!isValidGround(worldIn, pos)) {
            destroyBlockWithDrops(worldIn, pos, state);
        }
    }

    @Override
    public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
        List<ItemStack> drops = new java.util.ArrayList<>();
        Random rand = new Random();
        Item fruit = RItems.GREEN_CACTUS_FRUIT;
        switch (state.getValue(COLOR)) {
            case GREEN: fruit = RItems.GREEN_CACTUS_FRUIT; break;
            case BLUE: fruit = RItems.BLUE_CACTUS_FRUIT; break;
            case YELLOW: fruit = RItems.GREEN_CACTUS_FRUIT; break;
            case ORANGE: fruit = RItems.GREEN_CACTUS_FRUIT; break;
        }
        int dropCount = rand.nextInt(3) + 1;
        if (dropCount > 0) {
            drops.add(new ItemStack(fruit, dropCount));
        }
        return drops;
    }

    @Nullable
    @Override
    public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
        switch (state.getValue(COLOR)) {
            case GREEN: return AABB_SMALL;
            case BLUE: return AABB_SMALL;
            case YELLOW: return AABB_SMALL;
            case ORANGE: return AABB_SMALL;
            default: return AABB_SMALL;
        }
    }


    @Override
    public IBlockState getStateFromMeta(int meta) {
        return this.getDefaultState()
                .withProperty(COLOR, Color.fromMeta(meta & 3))
                .withProperty(AGE, Age.fromMeta((meta >> 2) & 3));
    }

    public int getMetaFromState(IBlockState state) {
        return state.getValue(COLOR).meta | (state.getValue(AGE).meta << 2);
    }

    protected BlockStateContainer createBlockState() {
        return new BlockStateContainer(this, COLOR, AGE);
    }

    public static enum Color implements IStringSerializable {
        GREEN(0),
        BLUE(1),
        YELLOW(2),
        ORANGE(3);

        public final int meta;

        Color(int meta) {
            this.meta = meta;
        }

        @Override
        public String getName() {
            return name().toLowerCase();
        }

        public static Color fromMeta(int meta) {
            return values()[meta % values().length];
        }
    }

    public static enum Age implements IStringSerializable {
        ZERO(0),
        ONE(1),
        TWO(2),
        THREE(3);

        public final int meta;

        Age(int meta) {
            this.meta = meta;

        }



        @Override
        public String getName() {
            return Integer.toString(meta);
        }

        public static JuvenileCactus.Age fromMeta(int meta) {
            return values()[meta % values().length];
        }
    }
}
