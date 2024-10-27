package net.macmv.rgen.client;

import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.fml.common.gameevent.InputEvent;
import net.minecraft.client.Minecraft;

public class FlySpeedKeyHandler {

  @SubscribeEvent
  public void onKeyInput(InputEvent.KeyInputEvent event) {
    // Check if the key binding was pressed
    if (RKeyBindings.toggleFlySpeed.isPressed()) {
      Minecraft mc = Minecraft.getMinecraft();
      if (mc.player != null && mc.player.capabilities != null) {
        // Toggle fly speed
        if (mc.player.capabilities.getFlySpeed() >= 1.0f) {
          // Reset to default speed
          mc.player.capabilities.setFlySpeed(0.05f);
        } else {
          // Increase speed
          mc.player.capabilities.setFlySpeed(1.0f);
        }
      }
    }
  }
}
