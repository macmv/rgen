package net.macmv.rgen.client;

import net.minecraft.client.settings.KeyBinding;
import net.minecraftforge.fml.client.registry.ClientRegistry;
import org.lwjgl.input.Keyboard;

public class RKeyBindings {
  public static KeyBinding toggleFlySpeed;

  private static final String CATEGORY = "Rgen";


  public static void init() {

    toggleFlySpeed = new KeyBinding(
        "Toggle Fast Flight", // sets the type name
        Keyboard.KEY_V,
        CATEGORY
    );
    // Register the key binding
    ClientRegistry.registerKeyBinding(toggleFlySpeed);
  }
}
