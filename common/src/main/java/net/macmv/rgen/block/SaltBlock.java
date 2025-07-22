package net.macmv.rgen.block;

import net.macmv.rgen.item.RItems;
import net.minecraft.block.BlockFalling;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.IBlockState;
import net.minecraft.enchantment.EnchantmentHelper;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Enchantments;
import net.minecraft.item.ItemStack;
import net.minecraft.tileentity.TileEntity;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

import javax.annotation.Nullable;
import java.util.Collections;
import java.util.List;
import java.util.Random;

public class SaltBlock extends BlockFalling {
    public SaltBlock(BlockSettings settings) {
        super(settings.material);
        this.setHarvestLevel("shovel", 0);
    }

    @Override
    public void harvestBlock(World world, EntityPlayer player, BlockPos pos, IBlockState state,
                             @Nullable TileEntity te, ItemStack tool) {
        if (!world.isRemote) {
            if (tool != null && EnchantmentHelper.getEnchantmentLevel(Enchantments.SILK_TOUCH, tool) > 0) {
                spawnAsEntity(world, pos, new ItemStack(this));
            } else {
                Random rand = world.rand;
                int amount = 1 + rand.nextInt(4); // 1–4 dust
                int fortune = EnchantmentHelper.getEnchantmentLevel(Enchantments.FORTUNE, tool);
                if (fortune > 0) {
                    amount += rand.nextInt(fortune + 1); // Fortune bonus
                }
                spawnAsEntity(world, pos, new ItemStack(RItems.SALT_DUST, amount));
            }
        }
    }

    @Override
    public boolean canSilkHarvest(World world, BlockPos pos, IBlockState state, EntityPlayer player) {
        return true;
    }



}
