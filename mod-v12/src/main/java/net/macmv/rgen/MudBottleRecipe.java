package net.macmv.rgen;

import net.minecraft.init.Items;
import net.minecraft.init.Blocks;
import net.minecraft.init.PotionTypes;
import net.minecraft.inventory.InventoryCrafting;
import net.minecraft.item.ItemStack;
import net.minecraft.item.Item;
import net.minecraft.item.crafting.IRecipe;
import net.minecraft.item.crafting.Ingredient;
import net.minecraft.potion.PotionUtils;
import net.minecraft.util.NonNullList;
import net.minecraft.util.ResourceLocation;
import net.minecraft.world.World;
import net.minecraftforge.registries.IForgeRegistryEntry;

public class MudBottleRecipe extends IForgeRegistryEntry.Impl<IRecipe> implements IRecipe {
  public MudBottleRecipe() { setRegistryName(new ResourceLocation("rgen", "mud_from_water_bottle")); }

  private boolean isWaterBottle(ItemStack stack) {
    return stack.getItem() == Items.POTIONITEM && PotionUtils.getPotionFromItem(stack) == PotionTypes.WATER;
  }

  @Override public boolean matches(InventoryCrafting inv, World world) {
    int dirt = 0, water = 0;
    for (int i = 0; i < inv.getSizeInventory(); i++) {
      ItemStack stack = inv.getStackInSlot(i);
      if (stack.isEmpty()) continue;
      if (stack.getItem() == Item.getItemFromBlock(Blocks.DIRT) && stack.getMetadata() == 0) dirt++;
      else if (isWaterBottle(stack)) water++;
      else return false;
    }
    return dirt == 1 && water == 1;
  }

  @Override public ItemStack getCraftingResult(InventoryCrafting inv) { return new ItemStack(MudContent.MUD); }
  @Override public boolean canFit(int width, int height) { return width * height >= 2; }
  @Override public ItemStack getRecipeOutput() { return new ItemStack(MudContent.MUD); }
  @Override public NonNullList<Ingredient> getIngredients() {
    NonNullList<Ingredient> ingredients = NonNullList.create();
    ingredients.add(Ingredient.fromStacks(new ItemStack(Item.getItemFromBlock(Blocks.DIRT))));
    ingredients.add(Ingredient.fromStacks(PotionUtils.addPotionToItemStack(new ItemStack(Items.POTIONITEM), PotionTypes.WATER)));
    return ingredients;
  }
  @Override public NonNullList<ItemStack> getRemainingItems(InventoryCrafting inv) {
    NonNullList<ItemStack> result = NonNullList.withSize(inv.getSizeInventory(), ItemStack.EMPTY);
    for (int i = 0; i < inv.getSizeInventory(); i++) if (isWaterBottle(inv.getStackInSlot(i))) result.set(i, new ItemStack(Items.GLASS_BOTTLE));
    return result;
  }
}
