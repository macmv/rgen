package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraft.block.properties.PropertyEnum;

public class DriedGrassBlock extends BlockBush {
  public static final PropertyEnum<Variant> VARIANT = PropertyEnum.create("variant", Variant.class);
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.1D, 0.0D, 0.1D, 0.9D, 0.8D, 0.9D);

  public DriedGrassBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
    setDefaultState(blockState.getBaseState().withProperty(VARIANT, Variant.BROWN_1));
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    return getDefaultState().withProperty(VARIANT, Variant.byMetadata(meta));
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(VARIANT).metadata;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT);
  }

  @Override
  public int damageDropped(IBlockState state) {
    return 0;
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    items.add(new ItemStack(this));
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
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return BOUNDS;
  }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  public enum Variant implements IStringSerializable {
    BROWN_1(0, "brown_1"),
    BROWN_2(1, "brown_2"),
    BROWN_3(2, "brown_3"),
    GREEN_1(3, "green_1"),
    GREEN_2(4, "green_2"),
    GREEN_3(5, "green_3"),
    YELLOW_1(6, "yellow_1"),
    YELLOW_2(7, "yellow_2"),
    YELLOW_3(8, "yellow_3");

    private static final Variant[] LOOKUP = values();
    public final int metadata;
    private final String name;

    Variant(int metadata, String name) {
      this.metadata = metadata;
      this.name = name;
    }

    public static Variant byMetadata(int metadata) {
      return LOOKUP[Math.floorMod(metadata, LOOKUP.length)];
    }

    @Override
    public String getName() {
      return name;
    }
  }
}
