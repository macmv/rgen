package net.macmv.rgen;

import net.macmv.rgen.block.LeavesBlockTwo;
import net.macmv.rgen.block.LogType;
import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.entity.REntities;
import net.macmv.rgen.item.RItems;

import net.minecraft.block.state.IBlockState;
import net.minecraft.client.Minecraft;
import net.minecraft.client.renderer.color.IBlockColor;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.ColorizerFoliage;
import net.minecraft.world.ColorizerGrass;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.biome.BiomeColorHelper;
import net.minecraft.client.renderer.color.BlockColors;
import net.minecraft.client.renderer.color.IItemColor;
import net.minecraft.client.renderer.color.ItemColors;
import net.minecraft.item.ItemStack;
import net.minecraftforge.fml.common.event.FMLInitializationEvent;
import net.minecraftforge.fml.common.event.FMLPreInitializationEvent;


public class RClientProxy extends RCommonProxy {

  @Override
  public void preInit() {
    REntities.registerModels();
  }

  @Override
  public void init() {
    BlockColors blockColors = Minecraft.getMinecraft().getBlockColors();

    // Create an IBlockColor instance for biome-based coloring
    IBlockColor lavenderColor = new IBlockColor() {
      @Override
      public int colorMultiplier(IBlockState state, IBlockAccess world, BlockPos pos, int tintIndex) {

        if (tintIndex == 0) {
          // Use vanilla grass biome color
          return world != null && pos != null ? BiomeColorHelper.getGrassColorAtPos(world, pos) : ColorizerGrass.getGrassColor(0.5D, 1.0D);
        } else if (tintIndex == 1){
          return world != null && pos != null ? BiomeColorHelper.getFoliageColorAtPos(world, pos) : ColorizerFoliage.getFoliageColor(0.5D, 1.0D);
        }


        return 0xFFFFFF; // Default color for non-tinted part
      }
    };

    IBlockColor mangroveLeavesColor = new IBlockColor() {
      @Override
      public int colorMultiplier(IBlockState state, IBlockAccess world, BlockPos pos, int tintIndex) {
        if (tintIndex != 0 || state.getValue(LeavesBlockTwo.VARIANT) != LogType.MANGROVE) {
          return 0xFFFFFF;
        }

        return world != null && pos != null
            ? BiomeColorHelper.getFoliageColorAtPos(world, pos)
            : ColorizerFoliage.getFoliageColor(0.5D, 1.0D);
      }
    };

    blockColors.registerBlockColorHandler(mangroveLeavesColor, RBlocks.LEAVES2);

    IItemColor mangroveLeavesItemColor = new IItemColor() {
      @Override
      public int colorMultiplier(ItemStack stack, int tintIndex) {
        if (tintIndex != 0 || stack.getMetadata() != LogType.MANGROVE.meta - 4) {
          return 0xFFFFFF;
        }

        return ColorizerFoliage.getFoliageColor(0.5D, 1.0D);
      }
    };
    ItemColors itemColors = Minecraft.getMinecraft().getItemColors();
    itemColors.registerItemColorHandler(mangroveLeavesItemColor, RItems.LEAVES2);

    IItemColor coveredJungleLogItemColor = new IItemColor() {
      @Override
      public int colorMultiplier(ItemStack stack, int tintIndex) {
        return tintIndex == 0 ? ColorizerGrass.getGrassColor(0.5D, 1.0D) : 0xFFFFFF;
      }
    };
    itemColors.registerItemColorHandler(coveredJungleLogItemColor, RItems.COVERED_JUNGLE_LOG);

    // Register block color handler for the LavenderPlant block
    blockColors.registerBlockColorHandler(lavenderColor, RBlocks.LAVENDER_PLANT, RBlocks.DOUBLE_TALL_LAVENDER_PLANT, RBlocks.PINK_FLOWERBED, RBlocks.FLOWER, RBlocks.HANGING_VINES, RBlocks.COVERED_JUNGLE_LOG);
  }
}
