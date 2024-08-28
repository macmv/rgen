package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.BlockStairs;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

import static net.macmv.rgen.block.Cactus.COLOR;

public class CactusArm extends Block {

  public static final PropertyEnum<CactusArm.Face> FACE = PropertyEnum.create("face", CactusArm.Face.class);


  protected static final AxisAlignedBB AABB_ARM = MathUtil.aabb(4, 1, 4, 12, 15, 12);
  public CactusArm() {
    super(Material.PLANTS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(FACE, CactusArm.Face.NORTH));
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {return AABB_ARM;}

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {return AABB_ARM;}

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) { return false; }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(FACE, CactusArm.Face.fromMeta(meta));
    return state;
  }

  public int getMetaFromState(IBlockState state) { int meta = state.getValue(FACE).meta; return meta; }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, FACE);
  }

  public static enum Face implements IStringSerializable {
    NORTH(0),
    EAST(1),
    SOUTH(2),
    WEST(3);


    public final int meta;

    Face(int meta) {
      this.meta = meta;
    }

    @Override
    public String getName() {
      switch (this) {
        case NORTH: return "north";
        case EAST: return "east";
        case SOUTH: return "south";
        case WEST: return "west";
        default: return "north";
      }
    }

    public static CactusArm.Face fromMeta(int meta) {
      switch (meta) {
        case 0: return NORTH;
        case 1: return EAST;
        case 2: return SOUTH;
        case 3: return WEST;
        default: return NORTH;
      }
    }
  }

  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn)
  {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }

  @Override
  public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
    // Get the current facing direction of the arm
    CactusArm.Face face = state.getValue(FACE);

    // Convert the custom Face enum to the corresponding EnumFacing direction
    EnumFacing facingDirection = getFacingDirection(face);

    // Determine the position of the adjacent block based on the facing direction
    BlockPos adjacentPos = pos.offset(facingDirection);

    // Check if the adjacent block is a cactus (or your custom RgenCactus block)
    IBlockState adjacentState = worldIn.getBlockState(adjacentPos);
    if (!((adjacentState.getBlock() instanceof Cactus) && adjacentState.getValue(COLOR) == Cactus.Color.GREEN)) {
      List<ItemStack> drops = getDrops(worldIn, pos, state, 0);
      for (ItemStack drop : drops) {
        spawnAsEntity(worldIn, pos, drop);  // Spawn the dropped items in the world
      }

      worldIn.setBlockToAir(pos);  // Set the block to air
      worldIn.playEvent(2001, pos, Block.getStateId(state));  // Generate breaking particles
    }
  }

  private EnumFacing getFacingDirection(CactusArm.Face face) {
    switch (face) {
      case NORTH: return EnumFacing.NORTH;
      case EAST: return EnumFacing.EAST;
      case SOUTH: return EnumFacing.SOUTH;
      case WEST: return EnumFacing.WEST;
      default: return EnumFacing.NORTH;
    }
  }

  @Override
  public List<ItemStack> getDrops(IBlockAccess world, BlockPos pos, IBlockState state, int fortune) {
    List<ItemStack> drops = new ArrayList<>();
    Random rand = new Random();
    // Random number of drops between 0 and 3
    int dropCount = rand.nextInt(3);  // Will generate a number between 0 and 2
    if (dropCount > 0) {
      drops.add(new ItemStack(RItems.GREEN_CACTUS_FRUIT, dropCount));  // Add the items to the drop list
    }
    return drops;
  }



}

