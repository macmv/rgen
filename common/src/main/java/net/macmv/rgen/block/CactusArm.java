package net.macmv.rgen.block;

import net.macmv.rgen.MathUtil;
import net.macmv.rgen.item.RItems;
import net.minecraft.block.Block;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
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

public class CactusArm extends Block {
  public static final PropertyEnum<CactusArm.Face> FACE = PropertyEnum.create("face", CactusArm.Face.class);
  protected static final AxisAlignedBB AABB_ARM = MathUtil.aabb(4, 1, 4, 12, 15, 12);
  private static final AxisAlignedBB[] ARM_PARTS = {
      new AxisAlignedBB(5 / 16.0D, 0, -1 / 16.0D, 11 / 16.0D, 6 / 16.0D, 13 / 16.0D),
      new AxisAlignedBB(5 / 16.0D, 6 / 16.0D, 7 / 16.0D, 11 / 16.0D, 1, 13 / 16.0D)
  };

  public CactusArm(BlockSettings settings) {
    super(settings.material);
    setLightOpacity(0);
    this.setDefaultState(this.blockState.getBaseState().withProperty(FACE, CactusArm.Face.NORTH));
  }

  @Override
  public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    return AABB_ARM;
  }

  @Override
  public AxisAlignedBB getBoundingBox(IBlockState state, IBlockAccess source, BlockPos pos) {
    AxisAlignedBB bounds = null;
    for (AxisAlignedBB part : ARM_PARTS) {
      // Block selection bounds are block-local; Minecraft offsets them by pos itself.
      AxisAlignedBB rotated = rotate(part, state.getValue(FACE));
      bounds = bounds == null ? rotated : bounds.union(rotated);
    }
    return bounds;
  }

  private static AxisAlignedBB rotate(AxisAlignedBB box, Face face) {
    double minX;
    double maxX;
    double minZ;
    double maxZ;
    switch (face) {
      case EAST:
        // Forge's y=90 blockstate rotation bends the arm toward negative X.
        minX = 1.0D - box.maxZ;
        maxX = 1.0D - box.minZ;
        minZ = box.minX;
        maxZ = box.maxX;
        break;
      case SOUTH:
        minX = 1.0D - box.maxX;
        maxX = 1.0D - box.minX;
        minZ = 1.0D - box.maxZ;
        maxZ = 1.0D - box.minZ;
        break;
      case WEST:
        // Forge's y=270 blockstate rotation bends the arm toward positive X.
        minX = box.minZ;
        maxX = box.maxZ;
        minZ = 1.0D - box.maxX;
        maxZ = 1.0D - box.minX;
        break;
      case NORTH:
      default:
        minX = box.minX;
        maxX = box.maxX;
        minZ = box.minZ;
        maxZ = box.maxZ;
        break;
    }
    return new AxisAlignedBB(minX, box.minY, minZ, maxX, box.maxY, maxZ);
  }

  @Override
  public boolean isOpaqueCube(IBlockState state) {
    return false;
  }

  @Override
  public boolean isFullCube(IBlockState state) {
    return false;
  }

  @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() {
    return BlockRenderLayer.CUTOUT;
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(FACE, CactusArm.Face.fromMeta(meta));
    return state;
  }

  public int getMetaFromState(IBlockState state) {
    int meta = state.getValue(FACE).meta;
    return meta;
  }

  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, FACE);
  }

  public enum Face implements IStringSerializable {
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

  public void onEntityCollidedWithBlock(World worldIn, BlockPos pos, IBlockState state, Entity entityIn) {
    entityIn.attackEntityFrom(DamageSource.CACTUS, 1.0F);
  }

  @Override
  public void neighborChanged(IBlockState state, World worldIn, BlockPos pos, Block blockIn, BlockPos fromPos) {
    removeIfUnsupported(state, worldIn, pos);
  }

  @Override
  public void onBlockAdded(World worldIn, BlockPos pos, IBlockState state) {
    removeIfUnsupported(state, worldIn, pos);
  }

  private void removeIfUnsupported(IBlockState state, World worldIn, BlockPos pos) {
    // Get the current facing direction of the arm
    CactusArm.Face face = state.getValue(FACE);

    // Convert the custom Face enum to the corresponding EnumFacing direction
    EnumFacing facingDirection = getFacingDirection(face);

    // Determine the position of the adjacent block based on the facing direction
    BlockPos adjacentPos = pos.offset(facingDirection);

    // Arms attach only to a green cactus trunk.
    IBlockState adjacentState = worldIn.getBlockState(adjacentPos);
    if (adjacentState.getBlock() != RBlocks.CACTUS && adjacentState.getBlock() != RBlocks.RGEN_CACTUS) {
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
    if (rand.nextFloat() < 0.25F) drops.add(new ItemStack(RItems.GREEN_CACTUS_FRUIT, 1));
    return drops;
  }
}
