package net.macmv.rgen.block;


import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateBase;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class HangingVines extends Block {
  public static final PropertyEnum<Type> TYPE = PropertyEnum.create("type", Type.class);

  public HangingVines() {
    super(Material.PLANTS); // Change material if needed
    this.setDefaultState(this.blockState.getBaseState().withProperty(TYPE, Type.BOTTOM));
  }


  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, TYPE);
  }

  @Override
  public IBlockState getActualState(IBlockState state, IBlockAccess world, BlockPos pos) {
    IBlockState belowState = world.getBlockState(pos.down());
    if (belowState.getBlock() == this) {
      return state.withProperty(TYPE, Type.STANDARD);
    } else {
      return state.withProperty(TYPE, Type.BOTTOM);
    }
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(TYPE) == Type.STANDARD ? 1 : 0;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(TYPE, meta == 1 ? Type.STANDARD : Type.BOTTOM);
  }

  public enum Type implements IStringSerializable {
    BOTTOM("bottom"),
    STANDARD("standard");

    private final String name;

    Type(String name) {
      this.name = name;
    }

    @Override
    public String getName() {
      return this.name;
    }
  }
}
