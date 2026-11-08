package net.macmv.rgen;

import net.minecraft.block.BlockSlab;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.IProperty;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

import java.util.Random;

public abstract class MudBrickSlab extends BlockSlab {
  public enum Type implements IStringSerializable {
    MUD_BRICKS;
    @Override public String getName() { return "mud_bricks"; }
  }
  public static final IProperty<Type> VARIANT = PropertyEnum.create("variant", Type.class);
  protected MudBrickSlab() {
    super(Material.ROCK);
    setHardness(1.5f);
    setResistance(3.0f);
    setCreativeTab(isDouble() ? null : net.minecraft.creativetab.CreativeTabs.BUILDING_BLOCKS);
    setRegistryName("rgen", isDouble() ? "mud_brick_double_slab" : "mud_brick_slab");
    setUnlocalizedName("rgen.mud_brick_slab");
    IBlockState state = blockState.getBaseState();
    if (!isDouble()) state = state.withProperty(HALF, EnumBlockHalf.BOTTOM);
    setDefaultState(state.withProperty(VARIANT, Type.MUD_BRICKS));
  }

  @Override public abstract boolean isDouble();
  @Override public IProperty<?> getVariantProperty() { return VARIANT; }
  @Override public Comparable<?> getTypeForItem(ItemStack stack) { return Type.MUD_BRICKS; }
  @Override public int damageDropped(IBlockState state) { return 0; }
  @Override public String getUnlocalizedName(int meta) { return "tile.rgen.mud_brick_slab"; }
  @Override public Item getItemDropped(IBlockState state, Random rand, int fortune) { return Item.getItemFromBlock(MudContent.MUD_BRICK_SLAB); }
  @Override public void getSubBlocks(net.minecraft.creativetab.CreativeTabs tab, NonNullList<ItemStack> list) { if (!isDouble()) list.add(new ItemStack(this)); }
  @Override public IBlockState getStateFromMeta(int meta) { return isDouble() ? getDefaultState() : getDefaultState().withProperty(HALF, (meta & 8) == 0 ? EnumBlockHalf.BOTTOM : EnumBlockHalf.TOP); }
  @Override public int getMetaFromState(IBlockState state) { return !isDouble() && state.getValue(HALF) == EnumBlockHalf.TOP ? 8 : 0; }
  @Override protected BlockStateContainer createBlockState() { return isDouble() ? new BlockStateContainer(this, VARIANT) : new BlockStateContainer(this, VARIANT, HALF); }
  @Override public ItemStack getItem(World world, BlockPos pos, IBlockState state) { return new ItemStack(MudContent.MUD_BRICK_SLAB); }
  @Override public MapColor getMapColor(IBlockState state, IBlockAccess world, BlockPos pos) { return MapColor.BROWN; }

  public static final class Half extends MudBrickSlab {
    public Half() {}
    @Override public boolean isDouble() { return false; }
  }

  public static final class Double extends MudBrickSlab {
    public Double() {}
    @Override public boolean isDouble() { return true; }
  }
}
