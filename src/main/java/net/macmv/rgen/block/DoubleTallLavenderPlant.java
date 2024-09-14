package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraft.block.BlockDoublePlant;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.properties.PropertyEnum;

import java.util.ArrayList;
import java.util.List;

public class DoubleTallLavenderPlant extends BlockBush {
    //this is the placement of the large plants
    //8  9    10   11
    //0  1    2    3

    // Define the properties: your custom variant and half
    public static final PropertyEnum<EnumVariant> VARIANT = PropertyEnum.create("variant", EnumVariant.class);
    public static final PropertyEnum<BlockDoublePlant.EnumBlockHalf> HALF = PropertyEnum.create("half", BlockDoublePlant.EnumBlockHalf.class);
    //protected static final AxisAlignedBB PLANT_AABB = new AxisAlignedBB(0.09999999403953552, 0.0, 0.09999999403953552, 0.8999999761581421, 1, 0.8999999761581421);
    protected static final AxisAlignedBB PLANT_AABB = new AxisAlignedBB(0, 0.0, 0, 1, 1, 1);


    public DoubleTallLavenderPlant() {
        super(Material.VINE);
        this.setDefaultState(this.blockState.getBaseState()
                .withProperty(VARIANT, EnumVariant.VARIANT_1)
                .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.LOWER));
    }

    public AxisAlignedBB getBoundingBox(IBlockState p_185496_1_, IBlockAccess p_185496_2_, BlockPos p_185496_3_) {
        return PLANT_AABB;
    }

    @Override
    public Block.EnumOffsetType getOffsetType() {
        return Block.EnumOffsetType.XYZ;
    }

    // Meta --> State
    @Override
    public IBlockState getStateFromMeta(int meta) {
        int variantMeta = meta & 3; // Meta for variants (0-3)
        BlockDoublePlant.EnumBlockHalf half = (meta & 8) == 0 ? BlockDoublePlant.EnumBlockHalf.LOWER : BlockDoublePlant.EnumBlockHalf.UPPER;
        return this.getDefaultState().withProperty(VARIANT, EnumVariant.byMetadata(variantMeta)).withProperty(HALF, half);
    }

    // State --> Meta
    @Override
    public int getMetaFromState(IBlockState state) {
        int meta = state.getValue(VARIANT).getMetadata();
        if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
            meta |= 8;
        }
        return meta;
    }

    @Override
    protected BlockStateContainer createBlockState() {
        // Create a custom BlockStateContainer with the custom properties
        return new BlockStateContainer(this, HALF, VARIANT);
    }

    // Randomize the variant when placed

    // Place the plant with both top and bottom parts

    @Override
    public void onBlockPlacedBy(World worldIn, BlockPos pos, IBlockState state, EntityLivingBase placer, ItemStack stack) {
        int randomVariant = worldIn.rand.nextInt(4);

        IBlockState lowerState = this.getDefaultState()
                .withProperty(VARIANT, EnumVariant.byMetadata(randomVariant))
                .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.LOWER);

        IBlockState upperState = this.getDefaultState()
                .withProperty(VARIANT, EnumVariant.byMetadata(randomVariant))
                .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.UPPER);

        // Set both parts with the same offset
        worldIn.setBlockState(pos, lowerState, 2);
        worldIn.setBlockState(pos.up(), upperState, 2);
    }




    // Custom EnumVariant for different lavender variants
    public static enum EnumVariant implements IStringSerializable {
        VARIANT_1(0, "variant1"),
        VARIANT_2(1, "variant2"),
        VARIANT_3(2, "variant3"),
        VARIANT_4(3, "variant4");

        private static final EnumVariant[] META_LOOKUP = new EnumVariant[values().length];
        private final int meta;
        private final String name;

        private EnumVariant(int meta, String name) {
            this.meta = meta;
            this.name = name;
        }

        public int getMetadata() {
            return this.meta;
        }

        public static EnumVariant byMetadata(int meta) {
            return META_LOOKUP[meta % META_LOOKUP.length];
        }

        @Override
        public String getName() {
            return this.name;
        }

        static {
            for (EnumVariant variant : values()) {
                META_LOOKUP[variant.getMetadata()] = variant;
            }
        }
    }


    // Ensure the block can stay in its current position
    @Override
    public boolean canBlockStay(World worldIn, BlockPos pos, IBlockState state) {
        if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
            // Check that the block below is this block and the bottom half
            IBlockState belowState = worldIn.getBlockState(pos.down());
            return belowState.getBlock() == this && belowState.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER;
        } else {
            // Check that the block below is a valid ground block (grass, dirt, etc.)
            IBlockState soil = worldIn.getBlockState(pos.down());
            return soil.getBlock() == Blocks.GRASS || soil.getBlock() == Blocks.DIRT || soil.getBlock() == Blocks.FARMLAND;
        }
    }

    @Override
    public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
        List<ItemStack> drops = new ArrayList<>();

        // Only drop the item if the block is the lower half
        if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER) {
            EnumVariant variant = state.getValue(VARIANT);

            // Add the appropriate ItemStack for the plant variant
            drops.add(new ItemStack(this, 1, variant.getMetadata()));
        }

        return drops;
    }

    @Override
    public void onBlockHarvested(World worldIn, BlockPos pos, IBlockState state, EntityPlayer player) {
        BlockPos otherHalfPos;

        if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER) {
            // This is the lower half, so remove the top half
            otherHalfPos = pos.up();
            IBlockState otherHalfState = worldIn.getBlockState(otherHalfPos);
            if (otherHalfState.getBlock() == this && otherHalfState.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
                worldIn.setBlockToAir(otherHalfPos);
                spawnAsEntity(worldIn, pos, new ItemStack(this, 1, state.getValue(VARIANT).getMetadata()));
            }
        } else {
            // This is the upper half, remove the lower half without dropping anything
            otherHalfPos = pos.down();
            IBlockState otherHalfState = worldIn.getBlockState(otherHalfPos);
            if (otherHalfState.getBlock() == this && otherHalfState.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER) {
                worldIn.setBlockToAir(otherHalfPos);
            }
        }

        // Remove this block
        worldIn.setBlockToAir(pos);
        super.onBlockHarvested(worldIn, pos, state, player);
    }

    @Override
    public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
        if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
            // Check the block below to make sure it's the correct bottom half of this plant
            if (worldIn.getBlockState(pos.down()).getBlock() != this) {
                worldIn.setBlockToAir(pos);

            }

        } else if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER) {
            IBlockState soil = worldIn.getBlockState(pos.down());
            if (soil.getBlock() != Blocks.GRASS && soil.getBlock() != Blocks.DIRT && soil.getBlock() != Blocks.FARMLAND) {
                // THE GROUND IS LOST
                BlockPos topPos = pos.up();
                IBlockState topState = worldIn.getBlockState(topPos);
                //if (topState.getBlock() == this && topState.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
                //    worldIn.setBlockToAir(topPos);
                //}
                worldIn.setBlockToAir(pos);
                spawnAsEntity(worldIn, pos, new ItemStack(this, 1, state.getValue(VARIANT).getMetadata()));
            } else {
                // Check the block above to ensure it's the upper half of this plant
                IBlockState aboveState = worldIn.getBlockState(pos.up());
                if (aboveState.getBlock() != this || aboveState.getValue(HALF) != BlockDoublePlant.EnumBlockHalf.UPPER) {
                    worldIn.setBlockToAir(pos);
                    //spawnAsEntity(worldIn, pos, new ItemStack(this, 1, state.getValue(VARIANT).getMetadata()));
                }
            }
        }
    }







    @Override
    public int getLightOpacity(IBlockState state, IBlockAccess world, BlockPos pos) {
        return 0; // Makes the block fully transparent to light
    }

    @Override
    public boolean doesSideBlockRendering(IBlockState state, IBlockAccess world, BlockPos pos, EnumFacing face) {
        return false;
    }


}
