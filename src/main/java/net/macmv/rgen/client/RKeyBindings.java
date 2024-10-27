package net.macmv.rgen.client;

import net.minecraft.client.settings.KeyBinding;
import net.minecraftforge.fml.client.registry.ClientRegistry;
import org.lwjgl.input.Keyboard;

public class RKeyBindings {
  public static KeyBinding toggleFlySpeed;

  public static void init() {
    // Define the key binding
    toggleFlySpeed = new KeyBinding(
        "key.toggleFlySpeed", // Description (shown in controls menu)
        Keyboard.KEY_V,       // Default key (V)
        "key.categories.movement" // Category
    );
    // Register the key binding
    ClientRegistry.registerKeyBinding(toggleFlySpeed);
  }
}
