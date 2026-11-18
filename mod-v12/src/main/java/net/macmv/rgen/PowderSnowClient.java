package net.macmv.rgen;

import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.Gui;
import net.minecraft.client.renderer.GlStateManager;
import net.minecraft.util.ResourceLocation;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.MathHelper;
import net.minecraftforge.client.event.InputUpdateEvent;
import net.minecraftforge.client.event.RenderGameOverlayEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.fml.relauncher.Side;

@Mod.EventBusSubscriber(modid = RGen.MOD_ID, value = Side.CLIENT)
public final class PowderSnowClient {
  // Light blue at approximately 15% opacity.
  private static final int POWDER_TINT = 0x266AAEFF;
  private static final ResourceLocation SNOW_TEXTURE =
      new ResourceLocation("minecraft", "textures/blocks/snow.png");

  private PowderSnowClient() {}
  @SubscribeEvent
  public static void input(InputUpdateEvent event) {
    event.getEntityPlayer().getEntityData().setBoolean("rgenPowderSnowJump", event.getMovementInput().jump);
  }

  @SubscribeEvent
  public static void renderTint(RenderGameOverlayEvent.Pre event) {
    // Draw once, before the HUD, so text and hearts retain their normal colors.
    if (event.getType() != RenderGameOverlayEvent.ElementType.ALL) return;
    Minecraft mc = Minecraft.getMinecraft();
    if (mc.player == null || mc.world == null || !touchingPowderSnow(mc)) return;

    Gui.drawRect(0, 0, event.getResolution().getScaledWidth(),
        event.getResolution().getScaledHeight(), POWDER_TINT);
    GlStateManager.color(1, 1, 1, 1);

    BlockPos eyes = new BlockPos(mc.player.getPositionEyes(event.getPartialTicks()));
    if (mc.world.isBlockLoaded(eyes)
        && mc.world.getBlockState(eyes).getBlock() == PowderSnowContent.BLOCK) {
      renderBuriedOverlay(mc, event.getResolution().getScaledWidth(),
          event.getResolution().getScaledHeight());
    }
  }

  private static void renderBuriedOverlay(Minecraft mc, int width, int height) {
    mc.getTextureManager().bindTexture(SNOW_TEXTURE);
    GlStateManager.disableDepth();
    GlStateManager.depthMask(false);
    GlStateManager.disableBlend();
    // Fully opaque, slightly blue snow hides the outside world while buried.
    GlStateManager.color(0.82F, 0.9F, 1.0F, 1.0F);
    Gui.drawModalRectWithCustomSizedTexture(0, 0, 0, 0, width, height,
        width / 4.0F, height / 4.0F);
    GlStateManager.color(1, 1, 1, 1);
    GlStateManager.disableBlend();
    GlStateManager.depthMask(true);
    GlStateManager.enableDepth();
  }

  private static boolean touchingPowderSnow(Minecraft mc) {
    AxisAlignedBB body = mc.player.getEntityBoundingBox();
    BlockPos.MutableBlockPos pos = new BlockPos.MutableBlockPos();
    for (int x = MathHelper.floor(body.minX); x <= MathHelper.floor(body.maxX); x++) {
      for (int y = MathHelper.floor(body.minY); y <= MathHelper.floor(body.maxY); y++) {
        for (int z = MathHelper.floor(body.minZ); z <= MathHelper.floor(body.maxZ); z++) {
          pos.setPos(x, y, z);
          if (mc.world.isBlockLoaded(pos)
              && mc.world.getBlockState(pos).getBlock() == PowderSnowContent.BLOCK
              // Strict overlap excludes merely standing on the block's top face.
              && body.intersects(new AxisAlignedBB(pos))) return true;
        }
      }
    }
    return false;
  }
}
