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
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraft.block.BlockDoublePlant;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.properties.PropertyEnum;

public class DoubleTallLavenderPlant extends BlockBush {

    // Define the properties: your custom variant and half
    public static final PropertyEnum<EnumVariant> VARIANT = PropertyEnum.create("variant", EnumVariant.class);
    public static final PropertyEnum<BlockDoublePlant.EnumBlockHalf> HALF = PropertyEnum.create("half", BlockDoublePlant.EnumBlockHalf.class);

    public DoubleTallLavenderPlant() {
        super(Material.VINE);
        this.setDefaultState(this.blockState.getBaseState()
                .withProperty(VARIANT, EnumVariant.VARIANT_1)
                .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.LOWER));
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
        int randomVariant = worldIn.rand.nextInt(4); // Randomly choose between 4 variants

        // Set bottom part
        worldIn.setBlockState(pos, this.getDefaultState()
                .withProperty(VARIANT, EnumVariant.byMetadata(randomVariant))
                .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.LOWER), 2);

        // Set top part
        worldIn.setBlockState(pos.up(), this.getDefaultState()
                .withProperty(VARIANT, EnumVariant.byMetadata(randomVariant))
                .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.UPPER), 2);
    }

    /*
    @SideOnly(Side.CLIENT)
    @Override
    public BlockRenderLayer getBlockLayer() {
        return BlockRenderLayer.CUTOUT_MIPPED;
    }

     */

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

    // Handle block harvesting to break both top and bottom parts
    @Override
    public void onBlockHarvested(World worldIn, BlockPos pos, IBlockState state, EntityPlayer player) {
        BlockPos otherHalfPos = state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER ? pos.up() : pos.down();
        IBlockState otherHalfState = worldIn.getBlockState(otherHalfPos);

        if (otherHalfState.getBlock() == this) {
            worldIn.setBlockToAir(otherHalfPos);
        }
        super.onBlockHarvested(worldIn, pos, state, player);
    }

    // Handle block updates when neighboring blocks change
    @Override
    public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
        if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
            if (worldIn.getBlockState(pos.down()).getBlock() != this) {
                worldIn.setBlockToAir(pos);
            }
        } else {
            IBlockState aboveState = worldIn.getBlockState(pos.up());
            if (aboveState.getBlock() != this || aboveState.getValue(HALF) != BlockDoublePlant.EnumBlockHalf.UPPER) {
                worldIn.setBlockToAir(pos);
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
