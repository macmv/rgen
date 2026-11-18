package net.macmv.rgen;

import net.minecraft.block.Block;
import net.minecraft.block.BlockDispenser;
import net.minecraft.client.renderer.block.model.ModelResourceLocation;
import net.minecraft.dispenser.BehaviorDefaultDispenseItem;
import net.minecraft.dispenser.IBlockSource;
import net.minecraft.init.Items;
import net.minecraft.init.SoundEvents;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.SoundCategory;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.RayTraceResult;
import net.minecraftforge.client.model.ModelLoader;
import net.minecraftforge.event.entity.player.FillBucketEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.common.eventhandler.Event;
import net.minecraftforge.fml.common.eventhandler.EventPriority;
import net.minecraftforge.fml.common.eventhandler.SubscribeEvent;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

@Mod.EventBusSubscriber(modid = RGen.MOD_ID)
public final class PowderSnowContent {
  public static final PowderSnowBlock BLOCK = new PowderSnowBlock();
  public static final PowderSnowBucket BUCKET = new PowderSnowBucket();
  private PowderSnowContent() {}

  public static void registerBlocks(IForgeRegistry<Block> registry) { registry.register(BLOCK); }
  public static void registerItems(IForgeRegistry<Item> registry) { registry.register(BUCKET); }
  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    ModelLoader.setCustomModelResourceLocation(BUCKET, 0,
        new ModelResourceLocation("rgen:powder_snow_bucket", "inventory"));
  }
  public static void init() {
    // Only our filled bucket: never replace vanilla empty-bucket dispenser behavior.
    BlockDispenser.DISPENSE_BEHAVIOR_REGISTRY.putObject(BUCKET, new BehaviorDefaultDispenseItem() {
      @Override protected ItemStack dispenseStack(IBlockSource source, ItemStack stack) {
        BlockPos target = source.getBlockPos().offset(source.getBlockState().getValue(BlockDispenser.FACING));
        if (PowderSnowBucket.place(source.getWorld(), target)) return new ItemStack(Items.BUCKET);
        return super.dispenseStack(source, stack);
      }
    });
  }
  @SubscribeEvent(priority = EventPriority.LOWEST)
  public static void fillBucket(FillBucketEvent event) {
    if (event.getResult() != Event.Result.DEFAULT || event.getEmptyBucket().getItem() != Items.BUCKET) return;
    RayTraceResult hit = event.getTarget();
    if (hit == null || hit.typeOfHit != RayTraceResult.Type.BLOCK) return;
    BlockPos pos = hit.getBlockPos();
    if (event.getWorld().getBlockState(pos).getBlock() != BLOCK) return;
    if (!event.getWorld().isBlockModifiable(event.getEntityPlayer(), pos)
        || !event.getEntityPlayer().canPlayerEdit(pos, hit.sideHit, event.getEmptyBucket())) return;
    if (!event.getWorld().isRemote) {
      if (!event.getWorld().setBlockToAir(pos)) return;
      event.getWorld().playSound(null, pos, SoundEvents.ITEM_BUCKET_FILL,
          SoundCategory.BLOCKS, 1, 1);
    }
    event.setFilledBucket(new ItemStack(BUCKET));
    event.setResult(Event.Result.ALLOW);
  }
}
