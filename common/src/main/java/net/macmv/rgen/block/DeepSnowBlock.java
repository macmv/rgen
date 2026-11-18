package net.macmv.rgen.block;

import java.util.Random;
import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockFaceShape;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.init.Blocks;
import net.minecraft.init.Items;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

/** Four quarter-block layers, including support on ice. */
public class DeepSnowBlock extends Block {
  public static final PropertyInteger LAYERS = PropertyInteger.create("layers", 1, 4);

  public DeepSnowBlock(BlockSettings settings) {
    super(Material.CRAFTED_SNOW);
    setDefaultState(blockState.getBaseState().withProperty(LAYERS, 1));
    setHardness(0.2F);
    setSoundType(SoundType.SNOW);
    setCreativeTab(settings.creativeTab);
    setHarvestLevel("shovel", 0);
    setLightOpacity(0);
  }

  @Override protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, LAYERS);
  }
  @Override public IBlockState getStateFromMeta(int meta) {
    return getDefaultState().withProperty(LAYERS, (meta & 3) + 1);
  }
  @Override public int getMetaFromState(IBlockState state) { return state.getValue(LAYERS) - 1; }
  @Override public int damageDropped(IBlockState state) { return 0; }
  @Override public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return new AxisAlignedBB(0, 0, 0, 1, state.getValue(LAYERS) / 4.0, 1);
  }
  @Override public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return getBoundingBox(state, world, pos);
  }
  @Override public boolean isFullCube(IBlockState state) { return state.getValue(LAYERS) == 4; }
  @Override public boolean isOpaqueCube(IBlockState state) { return isFullCube(state); }
  @Override public boolean isTopSolid(IBlockState state) { return isFullCube(state); }
  @Override public BlockFaceShape getBlockFaceShape(IBlockAccess world, IBlockState state, BlockPos pos, EnumFacing face) {
    return face == EnumFacing.DOWN || isFullCube(state) ? BlockFaceShape.SOLID : BlockFaceShape.UNDEFINED;
  }
  private boolean supported(World world, BlockPos pos) {
    IBlockState below = world.getBlockState(pos.down());
    return below.getBlock() == Blocks.ICE || below.getBlock() == Blocks.PACKED_ICE
        || below.getBlockFaceShape(world, pos.down(), EnumFacing.UP) == BlockFaceShape.SOLID;
  }
  @Override public boolean canPlaceBlockAt(World world, BlockPos pos) {
    return super.canPlaceBlockAt(world, pos) && supported(world, pos);
  }
  @Override public void neighborChanged(IBlockState state, World world, BlockPos pos, Block block, BlockPos from) {
    if (!world.isRemote && !supported(world, pos)) {
      dropBlockAsItem(world, pos, state, 0);
      world.setBlockToAir(pos);
    }
  }
  @Override public Item getItemDropped(IBlockState state, Random random, int fortune) { return Items.SNOWBALL; }
  @Override public int quantityDropped(IBlockState state, int fortune, Random random) {
    return state.getValue(LAYERS) * 2;
  }
  @Override protected boolean canSilkHarvest() { return true; }
  @Override protected ItemStack getSilkTouchDrop(IBlockState state) {
    return new ItemStack(this, state.getValue(LAYERS), 0);
  }
}
