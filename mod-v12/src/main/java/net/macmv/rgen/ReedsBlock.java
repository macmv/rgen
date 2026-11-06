package net.macmv.rgen;

import net.minecraft.block.BlockBush;
import net.minecraft.block.BlockDirt;
import net.minecraft.block.IGrowable;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.List;
import java.util.Random;

public class ReedsBlock extends BlockBush implements IGrowable {
  public enum Half implements IStringSerializable {
    LOWER, UPPER;
    @Override public String getName() { return name().toLowerCase(); }
  }

  public static final PropertyInteger AGE = PropertyInteger.create("age", 0, 3);
  public static final PropertyEnum<Half> HALF = PropertyEnum.create("half", Half.class);
  private static final AxisAlignedBB BOUNDS = new AxisAlignedBB(0.1D, 0.0D, 0.1D, 0.9D, 1.0D, 0.9D);

  public ReedsBlock() {
    super(Material.PLANTS);
    setRegistryName("rgen", "reeds");
    setUnlocalizedName("rgen.reeds");
    setSoundType(SoundType.PLANT);
    setTickRandomly(true);
    setDefaultState(blockState.getBaseState().withProperty(AGE, 0).withProperty(HALF, Half.LOWER));
  }

  @Override protected BlockStateContainer createBlockState() { return new BlockStateContainer(this, AGE, HALF); }
  @Override public int getMetaFromState(IBlockState state) { return state.getValue(AGE) | (state.getValue(HALF) == Half.UPPER ? 4 : 0); }
  @Override public IBlockState getStateFromMeta(int meta) { return getDefaultState().withProperty(AGE, meta & 3).withProperty(HALF, (meta & 4) == 0 ? Half.LOWER : Half.UPPER); }
  @Override public boolean isOpaqueCube(IBlockState state) { return false; }
  @Override public boolean isFullCube(IBlockState state) { return false; }
  @Override public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return BOUNDS; }

  @SideOnly(Side.CLIENT)
  @Override public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }

  @Override public boolean canPlaceBlockAt(World world, BlockPos pos) {
    return world.isAirBlock(pos) && canBlockStay(world, pos, getDefaultState());
  }

  @Override public boolean canBlockStay(World world, BlockPos pos, IBlockState state) {
    if (state.getValue(HALF) == Half.UPPER) {
      IBlockState lower = world.getBlockState(pos.down());
      return lower.getBlock() == this
          && lower.getValue(HALF) == Half.LOWER
          && lower.getValue(AGE).equals(state.getValue(AGE))
          && canBlockStay(world, pos.down(), lower);
    }

    if (world.getBlockState(pos).getBlock() == this
        && world.getBlockState(pos).getValue(AGE).equals(state.getValue(AGE))
        && state.getValue(AGE) >= 2
        && state.getValue(HALF) == Half.LOWER) {
      IBlockState above = world.getBlockState(pos.up());
      if (above.getBlock() != this || above.getValue(HALF) != Half.UPPER
          || !above.getValue(AGE).equals(state.getValue(AGE))) return false;
    }

    IBlockState water = world.getBlockState(pos.down());
    IBlockState soil = world.getBlockState(pos.down(2));
    return water.getMaterial() == Material.WATER && isValidSoil(soil);
  }

  private boolean isValidSoil(IBlockState state) {
    net.minecraft.block.Block block = state.getBlock();
    return block instanceof BlockDirt || block == net.minecraft.init.Blocks.GRASS
        || block == net.minecraft.init.Blocks.CLAY || block == net.minecraft.init.Blocks.GRAVEL
        || block == net.minecraft.init.Blocks.SAND || block == MudContent.MUD;
  }

  @Override public void randomTick(World world, BlockPos pos, IBlockState state, Random random) {
    if (world.isRemote || state.getValue(HALF) != Half.LOWER || state.getValue(AGE) == 3
        || random.nextInt(5) != 0 || !canBlockStay(world, pos, state)) return;

    growOneStage(world, pos, state);
  }

  private void growOneStage(World world, BlockPos pos, IBlockState state) {
    int age = state.getValue(AGE);
    if (age < 1) {
      world.setBlockState(pos, state.withProperty(AGE, 1), 2);
    } else if (age == 1) {
      BlockPos topPos = pos.up();
      if (!world.isAirBlock(topPos)) return;
      IBlockState tallLower = state.withProperty(AGE, 2);
      if (!canBlockStay(world, pos, tallLower)) return;
      world.setBlockState(pos, tallLower, 2);
      world.setBlockState(topPos, getDefaultState().withProperty(AGE, 2).withProperty(HALF, Half.UPPER), 2);
    } else {
      world.setBlockState(pos, state.withProperty(AGE, 3), 2);
      world.setBlockState(pos.up(), getDefaultState().withProperty(AGE, 3).withProperty(HALF, Half.UPPER), 2);
    }
  }

  @Override public boolean canGrow(World world, BlockPos pos, IBlockState state, boolean isClient) {
    BlockPos lowerPos = state.getValue(HALF) == Half.UPPER ? pos.down() : pos;
    IBlockState lower = world.getBlockState(lowerPos);
    if (lower.getBlock() != this || lower.getValue(HALF) != Half.LOWER
        || lower.getValue(AGE) >= 3 || !canBlockStay(world, lowerPos, lower)) return false;
    return lower.getValue(AGE) != 1 || world.isAirBlock(lowerPos.up());
  }

  @Override public boolean canUseBonemeal(World world, Random random, BlockPos pos, IBlockState state) { return true; }

  @Override public void grow(World world, Random random, BlockPos pos, IBlockState state) {
    BlockPos lowerPos = state.getValue(HALF) == Half.UPPER ? pos.down() : pos;
    IBlockState lower = world.getBlockState(lowerPos);
    if (canGrow(world, pos, state, world.isRemote)) growOneStage(world, lowerPos, lower);
  }

  @Override public void neighborChanged(IBlockState state, World world, BlockPos pos, net.minecraft.block.Block neighbor, BlockPos neighborPos) {
    if (!world.isRemote && !canBlockStay(world, pos, state)) {
      if (state.getValue(HALF) == Half.LOWER) {
        IBlockState above = world.getBlockState(pos.up());
        if (above.getBlock() == this) world.setBlockState(pos.up(), net.minecraft.init.Blocks.AIR.getDefaultState(), 2);
      }
      world.setBlockState(pos, net.minecraft.init.Blocks.AIR.getDefaultState(), 2);
    }
  }

  @Override public void onBlockHarvested(World world, BlockPos pos, IBlockState state, EntityPlayer player) {
    BlockPos otherPos = state.getValue(HALF) == Half.LOWER ? pos.up() : pos.down();
    IBlockState other = world.getBlockState(otherPos);
    if (other.getBlock() == this && other.getValue(HALF) != state.getValue(HALF)) {
      if (state.getValue(HALF) == Half.UPPER) {
        world.setBlockState(otherPos, other.withProperty(AGE, 0), 2);
      } else {
        world.setBlockState(otherPos, net.minecraft.init.Blocks.AIR.getDefaultState(), 2);
      }
    }
  }

  @Override public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    NonNullList<ItemStack> drops = NonNullList.create();
    int age = state.getValue(AGE);
    if (state.getValue(HALF) == Half.UPPER && age < 3) return drops;
    int count = age == 3 ? 1 + RANDOM.nextInt(2) : 1;
    drops.add(new ItemStack(ReedsContent.ITEM, count));
    return drops;
  }

  @Override public ItemStack getItem(World world, BlockPos pos, IBlockState state) { return new ItemStack(ReedsContent.ITEM); }
  @Override public int damageDropped(IBlockState state) { return 0; }
}
