package net.macmv.rgen;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Items;
import net.minecraft.init.SoundEvents;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.ActionResult;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumHand;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.RayTraceResult;
import net.minecraft.util.math.Vec3d;
import net.minecraft.world.World;

public final class PowderSnowBucket extends Item {
  public PowderSnowBucket() {
    setRegistryName("rgen", "powder_snow_bucket");
    setUnlocalizedName("powder_snow_bucket");
    setMaxStackSize(1);
    setContainerItem(Items.BUCKET);
    setCreativeTab(RCreativeTabs.DECORATIONS);
  }

  static boolean place(World world, BlockPos pos) {
    if (pos.getY() < 0 || pos.getY() >= world.getHeight() || !world.isBlockLoaded(pos)) return false;
    if (!world.getBlockState(pos).getBlock().isReplaceable(world, pos)
        || world.getBlockState(pos).getMaterial().isLiquid()) return false;
    if (!world.isRemote) {
      if (!world.setBlockState(pos, PowderSnowContent.BLOCK.getDefaultState(), 3)) return false;
      world.playSound(null, pos, SoundEvents.BLOCK_SNOW_PLACE, SoundCategory.BLOCKS, 1, 1);
    }
    return true;
  }

  @Override public ActionResult<ItemStack> onItemRightClick(World world, EntityPlayer player, EnumHand hand) {
    ItemStack stack = player.getHeldItem(hand);
    RayTraceResult hit = rayTrace(world, player, false);
    // The normal bucket trace skips blocks without collision, including powder
    // snow. Accept a powder-snow hit while preserving normal targeting otherwise.
    Vec3d start = player.getPositionEyes(1.0F);
    double reach = player.getEntityAttribute(EntityPlayer.REACH_DISTANCE).getAttributeValue();
    Vec3d end = start.add(player.getLook(1.0F).scale(reach));
    RayTraceResult powderHit = world.rayTraceBlocks(start, end, false, false, false);
    if (powderHit != null && powderHit.typeOfHit == RayTraceResult.Type.BLOCK
        && world.getBlockState(powderHit.getBlockPos()).getBlock() == PowderSnowContent.BLOCK) {
      hit = powderHit;
    }
    if (hit == null || hit.typeOfHit != RayTraceResult.Type.BLOCK) return new ActionResult<>(EnumActionResult.PASS, stack);
    BlockPos pos = hit.getBlockPos();
    if (world.getBlockState(pos).getBlock() == PowderSnowContent.BLOCK
        || !world.getBlockState(pos).getBlock().isReplaceable(world, pos)) pos = pos.offset(hit.sideHit);
    if (!world.isBlockModifiable(player, pos) || !player.canPlayerEdit(pos, hit.sideHit, stack)
        || !place(world, pos)) return new ActionResult<>(EnumActionResult.FAIL, stack);
    return new ActionResult<>(EnumActionResult.SUCCESS,
        player.capabilities.isCreativeMode ? stack : new ItemStack(Items.BUCKET));
  }
}
