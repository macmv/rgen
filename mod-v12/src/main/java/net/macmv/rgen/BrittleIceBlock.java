package net.macmv.rgen;

import java.util.Random;
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.BlockBreakable;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.properties.PropertyInteger;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.SoundEvents;
import net.minecraft.item.ItemStack;
import net.minecraft.util.BlockRenderLayer;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;

/** Step-sensitive ice; all behavior is confined to this RGen block. */
public final class BrittleIceBlock extends BlockBreakable {
  public static final PropertyInteger CRACKS = PropertyInteger.create("cracks", 0, 3);
  private static final int CRACK_INTERVAL = 24;

  public BrittleIceBlock() {
    // BlockBreakable hides internal faces only against the same block, even
    // when its crack state differs. Vanilla ice rendering is not modified.
    super(Material.ICE, false);
    setRegistryName("rgen", "brittle_ice");
    setUnlocalizedName("brittle_ice");
    setDefaultState(blockState.getBaseState().withProperty(CRACKS, 0));
    setHardness(0.5F);
    setResistance(0.5F);
    setSoundType(SoundType.GLASS);
    setHarvestLevel("pickaxe", 0);
    setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    setLightOpacity(3);
    slipperiness = 0.98F;
    // No random melting or per-block tile entity.
  }
  @Override protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, CRACKS);
  }
  @Override public IBlockState getStateFromMeta(int meta) {
    return getDefaultState().withProperty(CRACKS, meta & 3);
  }
  @Override public int getMetaFromState(IBlockState state) { return state.getValue(CRACKS); }
  @Override public int damageDropped(IBlockState state) { return 0; }
  @Override public int quantityDropped(Random random) { return 0; }
  @Override protected boolean canSilkHarvest() { return true; }
  @Override protected ItemStack getSilkTouchDrop(IBlockState state) {
    return new ItemStack(this, 1, 0);
  }
  @Override @SideOnly(Side.CLIENT)
  public BlockRenderLayer getBlockLayer() { return BlockRenderLayer.TRANSLUCENT; }

  private static boolean canCrack(Entity entity) {
    return entity instanceof EntityLivingBase && entity.isEntityAlive() && !entity.isRiding()
        && !(entity instanceof EntityPlayer && ((EntityPlayer) entity).isSpectator());
  }
  private void schedule(World world, BlockPos pos) {
    if (!world.isRemote) world.scheduleUpdate(pos, this, CRACK_INTERVAL);
  }
  @Override public void onEntityWalk(World world, BlockPos pos, Entity entity) {
    if (canCrack(entity)) schedule(world, pos);
    super.onEntityWalk(world, pos, entity);
  }
  @Override public void onFallenUpon(World world, BlockPos pos, Entity entity, float distance) {
    if (canCrack(entity)) schedule(world, pos);
    super.onFallenUpon(world, pos, entity, distance);
  }
  @Override public void onBlockAdded(World world, BlockPos pos, IBlockState state) {
    // Also handle a block placed directly under a stationary entity.
    schedule(world, pos);
  }
  @Override public void updateTick(World world, BlockPos pos, IBlockState state, Random random) {
    if (world.isRemote || world.getBlockState(pos).getBlock() != this) return;
    double top = pos.getY() + 1.0;
    AxisAlignedBB standingArea = new AxisAlignedBB(pos.getX(), top, pos.getZ(),
        pos.getX() + 1.0, top + 0.1, pos.getZ() + 1.0);
    boolean occupied = false;
    for (EntityLivingBase entity : world.getEntitiesWithinAABB(EntityLivingBase.class, standingArea)) {
      double feet = entity.getEntityBoundingBox().minY;
      if (canCrack(entity) && entity.onGround && feet >= top - 0.01 && feet <= top + 0.1) {
        occupied = true;
        break;
      }
    }
    // Leaving stops further cracking, but existing cracks remain.
    if (!occupied) return;
    int cracks = state.getValue(CRACKS);
    if (cracks == 3) {
      // Existing glass/ice break sound and particles; no drops or new water.
      world.destroyBlock(pos, false);
    } else {
      world.setBlockState(pos, state.withProperty(CRACKS, cracks + 1), 3);
      world.playSound(null, pos, SoundEvents.BLOCK_GLASS_BREAK, SoundCategory.BLOCKS, 0.35F, 0.8F);
      schedule(world, pos);
    }
  }
}
