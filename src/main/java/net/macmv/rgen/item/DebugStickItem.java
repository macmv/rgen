package net.macmv.rgen.item;

import net.minecraft.block.state.IBlockState;
import net.minecraft.client.Minecraft;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.ActionResult;
import net.minecraft.util.EnumHand;
import net.minecraft.util.ResourceLocation;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class DebugStickItem extends Item {
  @Override
  public boolean canDestroyBlockInCreative(World world, BlockPos pos, ItemStack stack, EntityPlayer player) {
    if (player.getEntityWorld().isRemote) {
      showBlockName(pos, player);
    }

    return false;
  }

  @Override
  public boolean onBlockStartBreak(ItemStack itemstack, BlockPos pos, EntityPlayer player) {
    if (player.getEntityWorld().isRemote) {
      showBlockName(pos, player);
    }

    return true;
  }

  @Override
  public ActionResult<ItemStack> onItemRightClick(World world, EntityPlayer player, EnumHand hand) {
    if (world.isRemote) {
      if (Minecraft.getMinecraft().player.capabilities.getFlySpeed() >= 1.0f) {
        // Default speed
        Minecraft.getMinecraft().player.capabilities.setFlySpeed(0.05f);
      } else {
        // Nyoooom
        Minecraft.getMinecraft().player.capabilities.setFlySpeed(1.0f);
      }
    }

    return super.onItemRightClick(world, player, hand);
  }

  @SideOnly(Side.CLIENT)
  private static void showBlockName(BlockPos pos, EntityPlayer player) {
    IBlockState state = player.getEntityWorld().getBlockState(pos);
    ResourceLocation path = state.getBlock().getRegistryName();
    int metaID = state.getBlock().getMetaFromState(state);

    String cyan = "§b";
    String green = "§a";
    String white = "§f";

    String name = path.getResourceDomain() + ":" + cyan + path.getResourcePath() + white;
    String meta = "[" + green + metaID + white + "]";

    Minecraft.getMinecraft().ingameGUI.setOverlayMessage(name + meta, false);
  }
}
