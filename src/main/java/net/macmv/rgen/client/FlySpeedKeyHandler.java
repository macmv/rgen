package net.macmv.rgen.client;

import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.fml.common.gameevent.InputEvent;
import net.minecraft.client.Minecraft;

public class FlySpeedKeyHandler {

  @SubscribeEvent
  public void onKeyInput(InputEvent.KeyInputEvent event) {
    if (RKeyBindings.toggleFlySpeed.isPressed()) {
      Minecraft mc = Minecraft.getMinecraft();
      if (mc.player != null && mc.player.capabilities != null) {
        if (mc.player.capabilities.getFlySpeed() >= 1.0f) {
          mc.player.capabilities.setFlySpeed(0.05f);
        } else {
          mc.player.capabilities.setFlySpeed(1.0f);
        }
      }
    }
  }
}
