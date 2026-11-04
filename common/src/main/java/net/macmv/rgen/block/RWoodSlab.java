// # only v12 #

package net.macmv.rgen.block;

import net.minecraft.block.BlockSlab;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.IProperty;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.item.ItemStack;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.macmv.rgen.item.RItems;

import java.util.Random;

/** Shared metadata slab for the group 1 and group 2 plank variants. */
public abstract class RWoodSlab extends BlockSlab {
  public static final IProperty<LogType> VARIANT = PlanksBlock.VARIANT;
  private static final LogType[] SLAB_TYPES = { LogType.FIR, LogType.PALM, LogType.CHERRY, LogType.CEDAR, LogType.MANGROVE, LogType.LAVENDER_TREE, LogType.DEAD, LogType.BURNED_TREE };

  public static int slabMeta(LogType type) {
    for (int i = 0; i < SLAB_TYPES.length; i++) if (SLAB_TYPES[i] == type) return i;
    return 0;
  }

  private static LogType slabType(int meta) { return SLAB_TYPES[meta & 7]; }

  protected RWoodSlab(BlockSettings settings) {
    super(Material.WOOD);
    this.setHardness(settings.hardness);
    this.setResistance(settings.resistance);
    this.setSoundType(settings.soundType);
    this.setCreativeTab(settings.creativeTab);
    IBlockState state = this.blockState.getBaseState();
    if (!this.isDouble()) {
      state = state.withProperty(HALF, EnumBlockHalf.BOTTOM);
    }
    this.setDefaultState(state.withProperty(VARIANT, LogType.FIR));
  }

  @Override
  public String getUnlocalizedName(int meta) {
    return "tile.plank_slab." + slabType(meta).name;
  }

  @Override
  public IProperty<?> getVariantProperty() {
    return VARIANT;
  }

  @Override
  public Comparable<?> getTypeForItem(ItemStack stack) {
    return slabType(stack.getMetadata());
  }

  @Override
  public int damageDropped(IBlockState state) {
    return slabMeta(state.getValue(VARIANT));
  }

  @Override
  public net.minecraft.item.Item getItemDropped(IBlockState state, Random random, int fortune) {
    return RItems.PLANK_SLAB;
  }

  @Override
  public ItemStack getItem(World world, BlockPos pos, IBlockState state) {
    return new ItemStack(RItems.PLANK_SLAB, 1, slabMeta(state.getValue(VARIANT)));
  }

  @Override
  public MapColor getMapColor(IBlockState state, IBlockAccess world, BlockPos pos) {
    return state.getValue(VARIANT).mapColor;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(VARIANT, slabType(meta & 7));
    if (!this.isDouble()) {
      state = state.withProperty(HALF, (meta & 8) == 0 ? EnumBlockHalf.BOTTOM : EnumBlockHalf.TOP);
    }
    return state;
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int meta = slabMeta(state.getValue(VARIANT));
    if (!this.isDouble() && state.getValue(HALF) == EnumBlockHalf.TOP) {
      meta |= 8;
    }
    return meta;
  }

  @Override
  public void getSubBlocks(net.minecraft.creativetab.CreativeTabs tab, NonNullList<ItemStack> items) {
    if (!this.isDouble()) {
      for (LogType type : LogType.values()) {
        if (type != LogType.SEASONAL && type.meta <= LogType.DEAD.meta) {
          items.add(new ItemStack(this, 1, slabMeta(type)));
        }
      }
      items.add(new ItemStack(this, 1, slabMeta(LogType.BURNED_TREE)));
    }
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return this.isDouble()
        ? new BlockStateContainer(this, VARIANT)
        : new BlockStateContainer(this, VARIANT, HALF);
  }

  public static final class Half extends RWoodSlab {
    public Half(BlockSettings settings) {
      super(settings);
    }

    @Override
    public boolean isDouble() {
      return false;
    }
  }

  public static final class Double extends RWoodSlab {
    public Double(BlockSettings settings) {
      super(settings);
    }

    @Override
    public boolean isDouble() {
      return true;
    }
  }
}
