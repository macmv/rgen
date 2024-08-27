package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.minecraft.block.Block;
import net.minecraft.block.BlockStairs;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.init.Blocks;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.DamageSource;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

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
        case 1: return NORTH;
        case 2: return EAST;
        case 3: return SOUTH;
        case 4: return WEST;
        default: return NORTH;
      }
    }
  }








  /*public BlockRenderLayer getBlockLayer()
  {
    return BlockRenderLayer.CUTOUT;
  }*/



  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn)
  {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }
}

