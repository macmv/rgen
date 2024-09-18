package net.macmv.rgen;

import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.entity.REntities;

import net.minecraft.block.state.IBlockState;
import net.minecraft.client.Minecraft;
import net.minecraft.client.renderer.color.IBlockColor;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.ColorizerGrass;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.biome.BiomeColorHelper;
import net.minecraft.client.renderer.color.BlockColors;
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
        }


        return 0xFFFFFF; // Default color for non-tinted part
      }
    };

    // Register block color handler for the LavenderPlant block
    blockColors.registerBlockColorHandler(lavenderColor, RBlocks.LAVENDER_PLANT, RBlocks.DOUBLE_TALL_LAVENDER_PLANT, RBlocks.PINK_FLOWERBED);
  }
}
