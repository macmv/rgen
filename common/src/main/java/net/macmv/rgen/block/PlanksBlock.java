package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;

public class PlanksBlock extends Block {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, ty -> ty.meta <= LogType.DEAD.meta);

  public PlanksBlock(BlockSettings settings) {
    super(settings.material);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, LogType.FIR));
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta;
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    for (LogType ty : LogType.values()) {
      if (ty.meta <= LogType.DEAD.meta) {
        items.add(new ItemStack(this, 1, ty.meta));
      }
    }
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(VARIANT, LogType.fromMeta(meta));
  }

  @Override
  public MapColor getMapColor(IBlockState state, IBlockAccess world, BlockPos pos) {
    return state.getValue(VARIANT).mapColor;
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(VARIANT).meta;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT);
  }
}
