package net.macmv.rgen.block;

import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.BlockBush;
import net.minecraft.block.BlockDoublePlant;
import net.minecraft.block.SoundType;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.ArrayList;
import java.util.List;

public class DoubleTallDriedGrassBlock extends BlockBush {
  public static final PropertyEnum<Variant> VARIANT = PropertyEnum.create("variant", Variant.class);
  public static final PropertyEnum<BlockDoublePlant.EnumBlockHalf> HALF = PropertyEnum.create("half", BlockDoublePlant.EnumBlockHalf.class);
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.0D, 0.0D, 0.0D, 1.0D, 1.0D, 1.0D);

  public DoubleTallDriedGrassBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
    setDefaultState(blockState.getBaseState()
        .withProperty(VARIANT, Variant.BROWN_1)
        .withProperty(HALF, BlockDoublePlant.EnumBlockHalf.LOWER));
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    Variant variant = Variant.byMetadata(meta & 7);
    BlockDoublePlant.EnumBlockHalf half = (meta & 8) == 0
        ? BlockDoublePlant.EnumBlockHalf.LOWER
        : BlockDoublePlant.EnumBlockHalf.UPPER;
    return getDefaultState().withProperty(VARIANT, variant).withProperty(HALF, half);
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(VARIANT).metadata;
    if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) meta |= 8;
    return meta;
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, HALF, VARIANT);
  }

  @Override
  public boolean canBlockStay(World world, BlockPos pos, IBlockState state) {
    if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.UPPER) {
      IBlockState below = world.getBlockState(pos.down());
      return below.getBlock() == this && below.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER;
    }

    IBlockState soil = world.getBlockState(pos.down());
    return soil.getBlock() == Blocks.GRASS || soil.getBlock() == Blocks.DIRT || soil.getBlock() == Blocks.FARMLAND;
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new ArrayList<>();
    if (state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER) {
      drops.add(new ItemStack(RItems.DRIED_GRASS));
    }
    return drops;
  }

  @Override
  public ItemStack getItem(World world, BlockPos pos, IBlockState state) {
    return new ItemStack(RItems.DRIED_GRASS);
  }

  @Override
  public void onBlockHarvested(World world, BlockPos pos, IBlockState state, EntityPlayer player) {
    BlockPos otherHalf = state.getValue(HALF) == BlockDoublePlant.EnumBlockHalf.LOWER ? pos.up() : pos.down();
    IBlockState otherState = world.getBlockState(otherHalf);
    if (otherState.getBlock() == this) {
      // Breaking the upper half removes the plant without dropping the item.
      world.setBlockToAir(otherHalf);
    }
    super.onBlockHarvested(world, pos, state, player);
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
    BROWN_1(0, "brown_1", DriedGrassBlock.Variant.BROWN_1),
    BROWN_2(1, "brown_2", DriedGrassBlock.Variant.BROWN_2),
    YELLOW_1(2, "yellow_1", DriedGrassBlock.Variant.YELLOW_1),
    YELLOW_2(3, "yellow_2", DriedGrassBlock.Variant.YELLOW_2),
    GREEN_1(4, "green_1", DriedGrassBlock.Variant.GREEN_1),
    GREEN_2(5, "green_2", DriedGrassBlock.Variant.GREEN_2);

    private static final Variant[] LOOKUP = values();
    public final int metadata;
    public final DriedGrassBlock.Variant singleVariant;
    private final String name;

    Variant(int metadata, String name, DriedGrassBlock.Variant singleVariant) {
      this.metadata = metadata;
      this.name = name;
      this.singleVariant = singleVariant;
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
