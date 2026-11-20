// # only v12 #

package net.macmv.rgen.item;

import net.minecraft.block.Block;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemFood;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraftforge.oredict.OreDictionary;

public class ColoredCactusFruit extends ItemFood {
  private final Block juvenileCactus;

  public ColoredCactusFruit(Block juvenileCactus, String dyeColor) {
    super(1, 0.3F, false);
    this.juvenileCactus = juvenileCactus;
    OreDictionary.registerOre("dye" + dyeColor, this);
  }

  @Override
  public EnumActionResult onItemUse(EntityPlayer player, World world, BlockPos pos, EnumHand hand,
                                   EnumFacing facing, float hitX, float hitY, float hitZ) {
    ItemStack stack = player.getHeldItem(hand);
    BlockPos plantPos = pos.offset(facing);
    boolean onSand = world.getBlockState(plantPos.down()).getBlock() == Blocks.SAND;
    boolean replaceable = world.getBlockState(plantPos).getBlock().isReplaceable(world, plantPos);
    if (facing != EnumFacing.UP || !onSand || !replaceable) return EnumActionResult.FAIL;

    if (!world.setBlockState(plantPos, juvenileCactus.getDefaultState())) return EnumActionResult.FAIL;

    if (!player.capabilities.isCreativeMode) stack.shrink(1);
    return EnumActionResult.SUCCESS;
  }
}
