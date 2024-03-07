package net.macmv.rgen.item;

import net.macmv.rgen.rust.RustGenerator;
import net.minecraft.client.Minecraft;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.item.Item;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class MossCompass extends Item {
  @Override
  public EnumActionResult onItemUse(EntityPlayer player, World worldIn, BlockPos pos, EnumHand hand, EnumFacing facing, float hitX, float hitY, float hitZ) {
    if (RustGenerator.isActive()) {
      String biome = RustGenerator.getBiomeAt(pos.getX(), pos.getY(), pos.getZ());

      Minecraft.getMinecraft().ingameGUI.setOverlayMessage(biome, false);
    }

    return super.onItemUse(player, worldIn, pos, hand, facing, hitX, hitY, hitZ);
  }
}
