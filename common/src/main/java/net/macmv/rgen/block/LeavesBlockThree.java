package net.macmv.rgen.block;

import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Items;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.stats.StatList;
import net.minecraft.tileentity.TileEntity;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.NonNullList;
import net.minecraft.world.IBlockAccess;
import net.minecraft.world.World;

import javax.annotation.Nonnull;
import javax.annotation.Nullable;
import java.util.Collections;
import java.util.List;

public class LeavesBlockThree extends LeavesBlock {
  public static final PropertyEnum<LogType> VARIANT = PropertyEnum.create("variant", LogType.class, type -> type.meta >= LogType.ASPEN.meta && type.meta <= LogType.YELLOW_SEASONAL_TREE.meta);

  public LeavesBlockThree(BlockSettings settings) {
    super(settings);
    setDefaultState(blockState.getBaseState().withProperty(VARIANT, LogType.ASPEN).withProperty(CHECK_DECAY, true).withProperty(DECAYABLE, true));
  }

  @Override
  public LogType getLogType(IBlockState state) {
    return state.getValue(VARIANT);
  }

  @Override
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, CHECK_DECAY, DECAYABLE);
  }

  @Override
  public void getSubBlocks(CreativeTabs tab, NonNullList<ItemStack> items) {
    for (int meta = 0; meta < 4; meta++) {
      items.add(new ItemStack(this, 1, meta));
    }
  }

  @Nonnull
  public List<ItemStack> onSheared(@Nonnull ItemStack item, IBlockAccess world, BlockPos pos, int fortune) {
    return Collections.singletonList(new ItemStack(this, 1, stateVariantIndex(world.getBlockState(pos).getValue(VARIANT))));
  }

  @Override
  public int getMetaFromState(IBlockState state) {
    int meta = stateVariantIndex(state.getValue(VARIANT));
    if (!state.getValue(DECAYABLE)) meta |= 4;
    if (state.getValue(CHECK_DECAY)) meta |= 8;
    return meta;
  }

  @Override
  public int damageDropped(IBlockState state) {
    return stateVariantIndex(state.getValue(VARIANT));
  }

  @Override
  protected ItemStack getSilkTouchDrop(IBlockState state) {
    return new ItemStack(Item.getItemFromBlock(this), 1, stateVariantIndex(state.getValue(VARIANT)));
  }

  @Override
  public void harvestBlock(World world, EntityPlayer player, BlockPos pos, IBlockState state, @Nullable TileEntity te, ItemStack stack) {
    if (!world.isRemote && stack.getItem() == Items.SHEARS) player.addStat(StatList.getBlockStats(this));
    else super.harvestBlock(world, player, pos, state, te, stack);
  }

  @Override
  public IBlockState getStateFromMeta(int meta) {
    LogType type = LogType.fromMeta((meta & 3) + LogType.ASPEN.meta);
    return getDefaultState().withProperty(VARIANT, type).withProperty(DECAYABLE, (meta & 4) == 0).withProperty(CHECK_DECAY, (meta & 8) > 0);
  }

  private static int stateVariantIndex(LogType type) { return type.meta - LogType.ASPEN.meta; }
}
