package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;


public class LavenderPlant extends BlockBush {

  // Sets the deffiniton of the varaiant enum
  public static final PropertyEnum<EnumVariant> VARIANT = PropertyEnum.create("variant", EnumVariant.class);
  protected static final AxisAlignedBB PLANT_AABB = new AxisAlignedBB(0.09999999403953552, 0.0, 0.09999999403953552, 0.8999999761581421, 0.800000011920929, 0.8999999761581421);

  public LavenderPlant() {
    super(Material.PLANTS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, EnumVariant.VARIANT_1)); // Set a default varian
  }

  public boolean isFullCube(IBlockState state) {
    return false;
  }

  @Override
  public Block setLightOpacity(int opacity) {
    return super.setLightOpacity(15);
  }

  // Meta --> State
  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(VARIANT, EnumVariant.byMetadata(meta));
  }
  // State --> Meta
  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(VARIANT).getMetadata();
  }

  @Override
  protected BlockStateContainer createBlockState() {
    // Here we pass the property as varargs (i.e., a list of properties)
    return new BlockStateContainer(this, VARIANT);
  }

  public static enum EnumVariant implements IStringSerializable {
    VARIANT_1(0, "variant0"),
    VARIANT_2(1, "variant1"),
    VARIANT_3(2, "variant2"),
    VARIANT_4(3, "variant3");

    private static final EnumVariant[] META_LOOKUP = new EnumVariant[values().length];
    private final int meta;
    private final String name;

    private EnumVariant(int meta, String name) {
      this.meta = meta;
      this.name = name;
    }

    public int getMetadata() {
      return this.meta;
    }

    public static EnumVariant byMetadata(int meta) {
      return META_LOOKUP[meta % META_LOOKUP.length];
    }

    @Override
    public String getName() {
      return this.name;
    }

    static {
      for (EnumVariant variant : values()) {
        META_LOOKUP[variant.getMetadata()] = variant;
      }
    }
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState p_185496_1_, IBlockAccess p_185496_2_, BlockPos p_185496_3_) {
    return PLANT_AABB;
  }

  @Override
  public Block.EnumOffsetType getOffsetType() {
    return Block.EnumOffsetType.XYZ;
  }

  // Randomizes the variant when placed
  @Override
  public void onBlockPlacedBy(World worldIn, BlockPos pos, IBlockState state, EntityLivingBase placer, ItemStack stack) {
    // Randomize the variant on placement
    int randomVariant = worldIn.rand.nextInt(EnumVariant.values().length);
    worldIn.setBlockState(pos, this.getDefaultState().withProperty(VARIANT, EnumVariant.byMetadata(randomVariant)), 2);
  }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT_MIPPED;
  }
}
