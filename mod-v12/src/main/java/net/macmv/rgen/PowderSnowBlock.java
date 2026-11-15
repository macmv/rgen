package net.macmv.rgen;

import java.util.List;
import java.util.Random;
import net.minecraft.block.Block;
import net.minecraft.block.SoundType;
import net.minecraft.block.material.Material;
import net.minecraft.block.state.BlockFaceShape;
import net.minecraft.block.state.IBlockState;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityLivingBase;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Items;
import net.minecraft.inventory.EntityEquipmentSlot;
import net.minecraft.item.ItemArmor;
import net.minecraft.item.ItemStack;
import net.minecraft.util.DamageSource;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.math.AxisAlignedBB;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

public final class PowderSnowBlock extends Block {
  private static final DamageSource COLD = new DamageSource("rgen_cold").setDamageBypassesArmor();
  private static final String LAST_TICK = "rgenPowderSnowTick";
  private static final String EXPOSURE = "rgenPowderSnowExposure";

  public PowderSnowBlock() {
    super(Material.CRAFTED_SNOW);
    setRegistryName("rgen", "powder_snow");
    setUnlocalizedName("powder_snow");
    setHardness(0.25F);
    setSoundType(SoundType.SNOW);
    setLightOpacity(0);
  }

  static boolean boots(Entity entity) {
    return entity instanceof EntityLivingBase && ((EntityLivingBase) entity)
        .getItemStackFromSlot(EntityEquipmentSlot.FEET).getItem() == Items.LEATHER_BOOTS;
  }
  private static boolean protectedFromCold(EntityLivingBase entity) {
    for (ItemStack stack : entity.getArmorInventoryList()) {
      if (stack.getItem() instanceof ItemArmor
          && ((ItemArmor) stack.getItem()).getArmorMaterial() == ItemArmor.ArmorMaterial.LEATHER) return true;
    }
    return false;
  }
  @Override public boolean isOpaqueCube(IBlockState state) { return false; }
  @Override public boolean isFullCube(IBlockState state) { return false; }
  @Override public BlockFaceShape getBlockFaceShape(IBlockAccess world, IBlockState state, BlockPos pos, EnumFacing face) {
    return BlockFaceShape.UNDEFINED;
  }
  @Override public AxisAlignedBB getCollisionBoundingBox(IBlockState state, IBlockAccess world, BlockPos pos) {
    return NULL_AABB;
  }
  @Override public void addCollisionBoxToList(IBlockState state, World world, BlockPos pos,
      AxisAlignedBB box, List<AxisAlignedBB> boxes, Entity entity, boolean actual) {
    if (boots(entity) && !entity.isSneaking() && entity.getEntityBoundingBox().minY >= pos.getY() + 0.99) {
      addCollisionBoxToList(pos, box, boxes, FULL_BLOCK_AABB);
    }
  }
  @Override public boolean isLadder(IBlockState state, IBlockAccess world, BlockPos pos, EntityLivingBase entity) {
    return boots(entity) && !entity.isSneaking();
  }
  @Override public int quantityDropped(Random random) { return 0; }
  @Override protected boolean canSilkHarvest() { return false; }
  @Override public ItemStack getItem(World world, BlockPos pos, IBlockState state) {
    return new ItemStack(PowderSnowContent.BUCKET);
  }
  @Override public void onEntityCollidedWithBlock(World world, BlockPos pos, IBlockState state, Entity entity) {
    if (entity instanceof EntityPlayer && (((EntityPlayer) entity).isSpectator()
        || ((EntityPlayer) entity).capabilities.isFlying)) return;
    if (entity.isBurning()) {
      if (!world.isRemote) {
        entity.extinguish();
        world.setBlockToAir(pos);
      }
      return;
    }
    // A wide entity can touch several powder blocks in one tick: apply effects once.
    int tick = entity.ticksExisted;
    net.minecraft.nbt.NBTTagCompound data = entity.getEntityData();
    if (data.hasKey(LAST_TICK) && data.getInteger(LAST_TICK) == tick) return;
    int previous = data.hasKey(LAST_TICK) ? data.getInteger(LAST_TICK) : tick - 2;
    data.setInteger(LAST_TICK, tick);
    entity.motionX *= 0.7;
    entity.motionZ *= 0.7;
    entity.fallDistance = 0;
    if (boots(entity)) {
      // Players climb using their normal jump input; vanilla movement packets
      // already carry the resulting position. No core patches or input packets.
      boolean climb = entity instanceof EntityPlayer
          ? data.getBoolean("rgenPowderSnowJump") : entity.collidedHorizontally;
      if (entity.isSneaking()) entity.motionY = -0.08;
      else if (climb) entity.motionY = 0.2;
      else entity.motionY = Math.max(entity.motionY, -0.08);
    } else {
      // Without leather boots, stop upward motion inside the snow while
      // retaining the slow sinking behavior. Boot-assisted climbing is above.
      entity.motionY = Math.min(entity.motionY, 0.0) * 0.5;
    }
    if (!world.isRemote && entity instanceof EntityLivingBase) {
      EntityLivingBase living = (EntityLivingBase) entity;
      int exposure = previous == tick - 1 ? data.getInteger(EXPOSURE) : 0;
      if (protectedFromCold(living)) exposure = 0;
      else if (++exposure >= 40) {
        living.attackEntityFrom(COLD, 1.0F);
        exposure = 0;
      }
      data.setInteger(EXPOSURE, exposure);
    }
  }
}
