package net.macmv.rgen.item;

import net.minecraft.block.properties.IProperty;
import net.minecraft.block.properties.PropertyBool;
import net.minecraft.block.properties.PropertyInteger;
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
    String red = "§c";
    String blue = "§9";
    String orange = "§6";

    StringBuilder sb = new StringBuilder();

    sb.append(path.getResourceDomain());
    sb.append(":");
    sb.append(cyan);
    sb.append(path.getResourcePath());
    sb.append(white);

    if (!state.getPropertyKeys().isEmpty()) {
      sb.append("[");
      boolean first = true;
      for (IProperty<?> prop : state.getPropertyKeys()) {
        if (!first) {
          sb.append(",");
        }
        first = false;

        sb.append(green);
        sb.append(prop.getName().toLowerCase());

        sb.append(white);
        sb.append("=");

        if (prop instanceof PropertyBool) {
          sb.append(red);
          sb.append(state.getValue(prop));
        } else if (prop instanceof PropertyInteger) {
          sb.append(blue);
          sb.append(state.getValue(prop));
        } else {
          sb.append(orange);
          sb.append(state.getValue(prop).toString().toLowerCase());
        }
        sb.append(white);
      }
      sb.append("]");

      sb.append(" -> ");

      sb.append("[");
      sb.append(green);
      sb.append(metaID);
      sb.append(white);
      sb.append("]");
    }

    Minecraft.getMinecraft().ingameGUI.setOverlayMessage(sb.toString(), false);
  }
}
