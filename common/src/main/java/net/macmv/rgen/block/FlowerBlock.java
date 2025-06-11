package net.macmv.rgen.block;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;

public class FlowerBlock extends BlockBush {
  public static PropertyEnum<FlowerType> TYPE = PropertyEnum.create("type", FlowerType.class);

  protected FlowerBlock() {
    this.setDefaultState(this.blockState.getBaseState().withProperty(TYPE, FlowerType.FORGETMENOT));

    this.setCreativeTab(RCreativeTabs.DECORATIONS);
    this.setSoundType(SoundType.PLANT);
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState blockState, IBlockAccess worldIn, BlockPos pos) {
    return NULL_AABB;
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }


  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return super.getBoundingBox(state, source, pos).offset(state.getOffset(source, pos));
  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(TYPE).meta;
  }

  @Override
  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    for (FlowerType flower : FlowerType.values()) {
      items.add(new ItemStack(this, 1, flower.meta));
    }
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(TYPE, FlowerType.fromMeta(meta));
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(TYPE).meta;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, TYPE);
  }

  @Override
  public Block.EnumOffsetType getOffsetType() {
    return Block.EnumOffsetType.XZ;
  }

  public enum FlowerType implements IStringSerializable {
    FORGETMENOT(0, "forgetmenot");

    public final int meta;
    public final String name;

    FlowerType(int meta, String name) {
      this.meta = meta;
      this.name = name;
    }

    @Override
    public String getName() {
      return name;
    }

    public static FlowerType fromMeta(int meta) {
      switch (meta) {
        default:
          return FORGETMENOT;
      }
    }
  }
}
