package net.macmv.rgen.block;


import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.BlockFalling;
import net.minecraft.block.state.IBlockState;
import net.minecraft.enchantment.EnchantmentHelper;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.init.Enchantments;
import net.minecraft.item.ItemStack;
import net.minecraft.tileentity.TileEntity;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

import javax.annotation.Nullable;
import java.util.Collections;
import java.util.List;
import java.util.Random;


public class SaltOnSand extends BlockFalling {
    private static final AxisAlignedBB SALT_AABB = new AxisAlignedBB(0.0D, 0.0D, 0.0D, 1.0D, 0.9375D, 1.0D);

    public SaltOnSand(BlockSettings settings) {
        super(settings.material);
        this.setTickRandomly(true);
        this.setHarvestLevel("shovel", 0);

    }
    @Override
    public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
        return SALT_AABB;
    }

    @Override
    public float getAmbientOcclusionLightValue(IBlockState state) {
        return 1.0F;
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
    public boolean isNormalCube(IBlockState state, IBlockAccess world, BlockPos pos) {
        return false;
    }

    @Override
    public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
        if (!worldIn.isRemote) {
            IBlockState above = worldIn.getBlockState(pos.up());

            // If block above is solid, revert to sand
            if (above.isOpaqueCube()) {
                worldIn.setBlockState(pos, Blocks.SAND.getDefaultState(), 2);
            } else {
                // Otherwise behave like sand and check if we should fall
                worldIn.scheduleUpdate(pos, this, this.tickRate(worldIn));
            }
        }
    }

    @Override
    public void harvestBlock(World world, EntityPlayer player, BlockPos pos, IBlockState state,
                             @Nullable TileEntity te, ItemStack tool) {
        if (!world.isRemote) {
            if (tool != null && EnchantmentHelper.getEnchantmentLevel(Enchantments.SILK_TOUCH, tool) > 0) {
                spawnAsEntity(world, pos, new ItemStack(this));
            } else {
                spawnAsEntity(world, pos, new ItemStack(RItems.SALT_DUST));
            }
        }
    }

    @Override
    public boolean canSilkHarvest(World world, BlockPos pos, IBlockState state, EntityPlayer player) {
        return true;
    }







}
