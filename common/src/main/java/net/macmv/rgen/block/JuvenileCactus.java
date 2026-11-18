package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.IGrowable;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import javax.annotation.Nullable;
import java.util.Collections;
import java.util.List;
import java.util.Random;

public class JuvenileCactus extends Block implements IGrowable {
  public static final PropertyEnum<Color> COLOR = PropertyEnum.create("color", Color.class);
  public static final PropertyEnum<Age> AGE = PropertyEnum.create("age", Age.class);

  protected static final AxisAlignedBB AABB_SMALL = MathUtil.aabb(4, 0, 4, 12, 8, 12);
  protected static final AxisAlignedBB AABB_MEDIUM = MathUtil.aabb(1, 0, 1, 15, 16, 15);

  public JuvenileCactus(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
    this.setDefaultState(this.blockState.getBaseState().withProperty(COLOR, Color.GREEN).withProperty(AGE, Age.ZERO));
    this.setTickRandomly(true);
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }

  @Override
  public void updateTick(World world, BlockPos pos, IBlockState state, Random rand) {
    if (!world.isAreaLoaded(pos, 1)) return;
    growOneStage(world, pos, state);
  }

  private void growOneStage(World world, BlockPos pos, IBlockState state) {
    Age age = state.getValue(AGE);
    if (age != Age.THREE) {
      world.setBlockState(pos, state.withProperty(AGE, Age.values()[age.meta + 1]), 2);
    } else {
      // TODO: Grow green cactus into a multi-block cactus instead of a single mature block.
      world.setBlockState(pos, RBlocks.CACTUS.getDefaultState().withProperty(Cactus.COLOR, toMatureColor(state.getValue(COLOR))), 3);
    }
  }

  private Cactus.Color toMatureColor(Color color) {
    switch (color) {
      case BLUE: return Cactus.Color.BLUE;
      case YELLOW: return Cactus.Color.YELLOW;
      case ORANGE: return Cactus.Color.ORANGE;
      case GREEN:
      default: return Cactus.Color.GREEN;
    }
  }

  @Override
  public boolean canGrow(World world, BlockPos pos, IBlockState state, boolean isClient) {
    return true;
  }

  @Override
  public boolean canUseBonemeal(World world, Random rand, BlockPos pos, IBlockState state) {
    return rand.nextFloat() < 0.45F;
  }

  @Override
  public void grow(World world, Random rand, BlockPos pos, IBlockState state) {
    growOneStage(world, pos, state);
  }

  private boolean isValidGround(World worldIn, BlockPos pos) {
    Block blockBelow = worldIn.getBlockState(pos.down()).getBlock();
    return blockBelow == Blocks.SAND || blockBelow == this;
  }

  private void destroyBlockWithDrops(World worldIn, BlockPos pos, IBlockState state) {
    List<ItemStack> drops = getDrops(worldIn, pos, state, 0);
    for (ItemStack drop : drops) {
      spawnAsEntity(worldIn, pos, drop);
    }
    worldIn.setBlockToAir(pos);
    worldIn.playEvent(2001, pos, Block.getStateId(state));
  }

  @Override
  public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
    if (!isValidGround(worldIn, pos)) {
      destroyBlockWithDrops(worldIn, pos, state);
    }
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    return Collections.emptyList();
  }

  @Nullable
  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
    switch (state.getValue(COLOR)) {
      case GREEN: return AABB_SMALL;
      case BLUE: return AABB_SMALL;
      case YELLOW: return AABB_SMALL;
      case ORANGE: return AABB_SMALL;
      default: return AABB_SMALL;
    }
  }


  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(COLOR, Color.fromMeta(meta & 3)).withProperty(AGE, Age.fromMeta((meta >> 2) & 3));
  }

  public int getMetaFromState(IBlockState state) {
    return state.getValue(COLOR).meta | (state.getValue(AGE).meta << 2);
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, COLOR, AGE);
  }

  public enum Color implements IStringSerializable {
    GREEN(0),
    BLUE(1),
    YELLOW(2),
    ORANGE(3);

    public final int meta;

    Color(int meta) {
      this.meta = meta;
    }

    @Override
    public String getName() {
      return name().toLowerCase();
    }

    public static Color fromMeta(int meta) {
      return values()[meta % values().length];
    }
  }

  public static enum Age implements IStringSerializable {
    ZERO(0),
    ONE(1),
    TWO(2),
    THREE(3);

    public final int meta;

    Age(int meta) {
      this.meta = meta;
    }

    @Override
    public String getName() {
      return Integer.toString(meta);
    }

    public static JuvenileCactus.Age fromMeta(int meta) {
      return values()[meta % values().length];
    }
  }
}
