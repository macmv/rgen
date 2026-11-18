package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

/** Small cross-shaped flower whose model follows the direction of its cactus arm. */
public class SmallCactusFlowerBlock extends Block {
  public static final PropertyEnum<ArmFace> FACE = PropertyEnum.create("face", ArmFace.class);

  public SmallCactusFlowerBlock(BlockSettings settings) {
    super(settings.material);
    setSoundType(SoundType.PLANT);
    setLightOpacity(0);
    setDefaultState(blockState.getBaseState().withProperty(FACE, ArmFace.NORTH));
  }

  @Override
  protected BlockStateContainer createBlockState() { return new BlockStateContainer(this, FACE); }

  @Override
  public IBlockState getStateFromMeta(int meta) { return getDefaultState().withProperty(FACE, ArmFace.byMeta(meta)); }

  @Override
  public int getMetaFromState(IBlockState state) { return state.getValue(FACE).meta; }

  @Override
  public IBlockState getStateForPlacement(World world, BlockPos pos, EnumFacing facing, float hitX, float hitY,
                                          float hitZ, int meta, EntityLivingBase placer) {
    IBlockState arm = world.getBlockState(pos.down());
    return arm.getBlock() == RBlocks.CACTUS_ARM
        ? getDefaultState().withProperty(FACE, ArmFace.values()[arm.getValue(CactusArm.FACE).meta])
        : getDefaultState();
  }

  private boolean hasSupport(World world, BlockPos pos, IBlockState state) {
    IBlockState arm = world.getBlockState(pos.down());
    return arm.getBlock() == RBlocks.CACTUS_ARM && arm.getValue(CactusArm.FACE).meta == state.getValue(FACE).meta;
  }

  @Override
  public boolean canPlaceBlockAt(World world, BlockPos pos) {
    return world.getBlockState(pos).getBlock().isReplaceable(world, pos)
        && world.getBlockState(pos.down()).getBlock() == RBlocks.CACTUS_ARM;
  }

  @Override
  public void onBlockAdded(World world, BlockPos pos, IBlockState state) { removeIfUnsupported(world, pos, state); }

  @Override
  public void neighborChanged(IBlockState state, World world, BlockPos pos, Block blockIn, BlockPos fromPos) {
    removeIfUnsupported(world, pos, state);
  }

  private void removeIfUnsupported(World world, BlockPos pos, IBlockState state) {
    if (!world.isRemote && !hasSupport(world, pos, state)) world.destroyBlock(pos, true);
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    // Keep selection on the 6x6 pixel arm tip, one pixel tall, directly above the arm.
    int centerX;
    int centerZ;
    switch (state.getValue(FACE)) {
      case EAST: centerX = 6; centerZ = 8; break;
      case SOUTH: centerX = 8; centerZ = 6; break;
      case WEST: centerX = 10; centerZ = 8; break;
      case NORTH:
      default: centerX = 8; centerZ = 10; break;
    }
    return new AxisAlignedBB((centerX - 3) / 16.0D, 0, (centerZ - 3) / 16.0D,
        (centerX + 3) / 16.0D, 6 / 16.0D, (centerZ + 3) / 16.0D);
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) { return NULL_AABB; }

  @Override
  public boolean isOpaqueCube(IBlockState state) { return false; }

  @Override
  public boolean isFullCube(IBlockState state) { return false; }

  @SideOnly(Side.CLIENT)
  @Override
  public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.CUTOUT; }

  public enum ArmFace implements IStringSerializable {
    NORTH(0), EAST(1), SOUTH(2), WEST(3);
    public final int meta;
    ArmFace(int meta) { this.meta = meta; }
    static ArmFace byMeta(int meta) { return values()[Math.floorMod(meta, values().length)]; }
    @Override public String getName() { return name().toLowerCase(java.util.Locale.ROOT); }
  }
}
