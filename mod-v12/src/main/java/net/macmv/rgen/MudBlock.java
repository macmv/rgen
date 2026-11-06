package net.macmv.rgen;

import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;

/** Minecraft 1.19-style mud, whose collision shape is one pixel shorter than a full block. */
public class MudBlock extends Block {
  private static final AxisAlignedBB SHAPE = new AxisAlignedBB(0, 0, 0, 1, 15.0 / 16.0, 1);

  public MudBlock() {
    super(Material.GROUND);
    setHardness(0.5f);
    setResistance(2.5f);
    setSoundType(SoundType.GROUND);
    setCreativeTab(net.minecraft.creativetab.CreativeTabs.BUILDING_BLOCKS);
    setRegistryName("rgen", "mud");
    setUnlocalizedName("rgen.mud");
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return SHAPE;
  }
}
