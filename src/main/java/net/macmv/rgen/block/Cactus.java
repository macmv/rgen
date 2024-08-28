package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.init.Blocks;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.*;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import javax.annotation.Nullable;
import java.util.List;
import java.util.Random;

public class Cactus extends Block {
    public static final PropertyEnum<Cactus.Color> COLOR = PropertyEnum.create("color", Cactus.Color.class);

    protected static final AxisAlignedBB AABB_BLUE = MathUtil.aabb(4, 0, 4, 12, 8, 12);
    protected static final AxisAlignedBB AABB_GREEN = MathUtil.aabb(1, 0, 1, 15, 16, 15);
    protected static final AxisAlignedBB AABB_ORANGE_SELECT = MathUtil.aabb(4, 0, 4, 12, 14, 12);
    protected static final AxisAlignedBB AABB_ORANGE_COLIDE = NULL_AABB;



    public Cactus() {
        super(Material.PLANTS);
        this.setHardness(0.4f);
        this.setDefaultState(this.blockState.getBaseState().withProperty(COLOR, Color.GREEN));

    }

    @Override
    public boolean isOpaqueCube(IBlockState state) {
        return false;
    }

    @SideOnly(Side.CLIENT)
    public BlockRenderLayer getBlockLayer()
    {
        return BlockRenderLayer.CUTOUT;
    }

    @Override
    public boolean isFullCube(IBlockState state) {
        return false;
    }

    public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn)
    {entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);}

    //pos is the position of the cactus block not the ground
    private boolean isValidGround(World worldIn, BlockPos pos) {
        Block blockBelow = worldIn.getBlockState(pos.down()).getBlock();
        IBlockState blockCurrent = worldIn.getBlockState(pos);
        if (blockCurrent.getValue(COLOR) ==  Color.GREEN) {
            return blockBelow == Blocks.SAND || blockBelow == this;
        } else {
            return blockBelow == Blocks.SAND;
        }
    }

    // Destroy the block and drop fruits
    private void destroyBlockWithDrops(World worldIn, BlockPos pos, IBlockState state) {
        // Get the drops (between 0 and 3 fruits)
        List<ItemStack> drops = getDrops(worldIn, pos, state, 0);
        for (ItemStack drop : drops) {
            spawnAsEntity(worldIn, pos, drop);  // Drop the items in the world
        }
        worldIn.setBlockToAir(pos);  // Destroy the block
        worldIn.playEvent(2001, pos, Block.getStateId(state));  // Show block breaking particles
    }

    @Override
    public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
        // If the ground is not valid, destroy the block
        if (!isValidGround(worldIn, pos)) {
            destroyBlockWithDrops(worldIn, pos, state);
        }
    }

    @Override
    public void onBlockAdded(World worldIn, BlockPos pos, IBlockState state) {
        // When the block is placed, check the block below it
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
            case GREEN: fruit = RItems.GREEN_CACTUS_FRUIT;
            case BLUE: fruit = RItems.BLUE_CACTUS_FRUIT;
            case YELLOW: fruit = RItems.GREEN_CACTUS_FRUIT;
            case ORANGE: fruit = RItems.GREEN_CACTUS_FRUIT;
            default: fruit = RItems.GREEN_CACTUS_FRUIT;
        }
        // Random number of drops between 0 and 3
        int dropCount = rand.nextInt(3)+1;  // Will generate a number between 0 and 3
        if (dropCount > 0) {
            drops.add(new ItemStack(fruit, dropCount));  // Add the items to the drop list
        }
        return drops;
    }

    @Nullable
    @Override
    public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
        switch (state.getValue(COLOR)) {
            case GREEN: return AABB_GREEN;
            case BLUE: return AABB_BLUE;
            case YELLOW: return AABB_GREEN;
            case ORANGE: return AABB_ORANGE_COLIDE;
            default: return AABB_GREEN;
        }
    }

    @Override
    public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
        switch (state.getValue(COLOR)) {
            case GREEN: return AABB_GREEN;
            case BLUE: return AABB_BLUE;
            case YELLOW: return AABB_GREEN;
            case ORANGE: return AABB_ORANGE_SELECT;
            default: return AABB_GREEN;
        }
    }



    @Override
    public IBlockState getStateFromMeta(int meta) {
        IBlockState state = this.getDefaultState().withProperty(COLOR, Cactus.Color.fromMeta(meta));
        return state;
    }

    public int getMetaFromState(IBlockState state) {
        return state.getValue(COLOR).meta;
    }

    protected BlockStateContainer createBlockState() {
        return new BlockStateContainer(this, COLOR);
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
            switch (this) {
                case GREEN: return "green";
                case BLUE: return "blue";
                case YELLOW: return "yellow";
                case ORANGE: return "orange";
                default: return "green";
            }
        }
        public static Cactus.Color fromMeta(int meta) {
            switch (meta) {
                case 0: return GREEN;
                case 1: return BLUE;
                case 2: return YELLOW;
                case 3: return ORANGE;
                default: return GREEN;
            }
        }
    }

}
