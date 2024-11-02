package net.macmv.rgen.block;


import net.minecraft.block.Block;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

public class HangingVines extends Block {
  public static final PropertyEnum<Type> TYPE = PropertyEnum.create("type", Type.class);

  public HangingVines() {
    super(Material.PLANTS); // Change material if needed
    this.setDefaultState(this.blockState.getBaseState().withProperty(TYPE, Type.BOTTOM));
  }


  // LOOK
  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }
  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }
  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT_MIPPED;
  }



  // CAN WALK THROUGH
  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess worldIn, BlockPos pos) {
    return NULL_AABB; // No collision box
  }

  @Override
  public boolean isPassable(IBlockAccess worldIn, BlockPos pos) {
    return true; // Makes the block passable
  }

  // BREAKS WHEN IT'S MISSING THE STUFF ABOVE
  @Override
  public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
    super.neighborChanged(state, worldIn, pos, blockIn, fromPos);

    // Check if the block above is a log, leaf, or another HangingVines
    BlockPos abovePos = pos.up();
    IBlockState aboveState = worldIn.getBlockState(abovePos);
    Block aboveBlock = aboveState.getBlock();

    if (!(aboveBlock == this ||
        aboveBlock.isLeaves(aboveState, worldIn, abovePos) ||
        aboveBlock.isWood(worldIn, abovePos))) {
      // Drop the block as an item and set the position to air if not supported
      worldIn.destroyBlock(pos, true);
    }
  }

  // SETS THE STATE WORK
  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, TYPE);
  }
  @Override
  public IBlockState getActualState(IBlockState state, IBlockAccess world, BlockPos pos) {
    IBlockState belowState = world.getBlockState(pos.down());
    if (belowState.getBlock() == this) {
      return state.withProperty(TYPE, Type.STANDARD);
    } else {
      return state.withProperty(TYPE, Type.BOTTOM);
    }
  }
  @Override
  public int getMetaFromState(IBlockState state) {
    return state.getValue(TYPE) == Type.STANDARD ? 1 : 0;
  }
  @Override
  public IBlockState getStateFromMeta(int meta) {
    return this.getDefaultState().withProperty(TYPE, meta == 1 ? Type.STANDARD : Type.BOTTOM);
  }
  public enum Type implements IStringSerializable {
    BOTTOM("bottom"),
    STANDARD("standard");

    private final String name;

    Type(String name) {
      this.name = name;
    }

    @Override
    public String getName() {
      return this.name;
    }
  }

}
