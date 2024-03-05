package net.macmv.rgen.block;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.BlockLog;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.block.state.BlockStateContainer;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.IStringSerializable;
import net.minecraft.util.NonNullList;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.IBlockAccess;
import net.minecraftforge.common.EnumPlantType;
import net.minecraftforge.common.IPlantable;

public class RGenMossyLogBlock extends BlockLog {
  public static final PropertyEnum<RGenMossyLogBlock.LogType> VARIANT = PropertyEnum.create("variant", RGenMossyLogBlock.LogType.class);

  public RGenMossyLogBlock() {
    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
    this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, RGenMossyLogBlock.LogType.OAK).withProperty(LOG_AXIS, BlockLog.EnumAxis.Y));
  }

  public boolean canSustainPlant(IBlockState state, IBlockAccess world, BlockPos pos, EnumFacing direction, IPlantable plantable)
  {
    IBlockState plant = plantable.getPlant(world, pos.offset(direction));
    net.minecraftforge.common.EnumPlantType plantType = plantable.getPlantType(world, pos.offset(direction));

    if(plantType == EnumPlantType.Plains)
      return true;

    return false;
  }
  protected BlockStateContainer createBlockState() {
    return new BlockStateContainer(this, VARIANT, LOG_AXIS);
  }

  public void getSubBlocks(CreativeTabs itemIn, NonNullList<ItemStack> items) {
    // TODO: Add these back
    // items.add(new ItemStack(this, 1, LogType.CEDAR.meta));
    // items.add(new ItemStack(this, 1, LogType.FIR.meta));
    items.add(new ItemStack(this, 1, LogType.OAK.meta));
    items.add(new ItemStack(this, 1, LogType.BIRCH.meta));

  }

  @Override
  public int damageDropped(IBlockState state) {
    return state.getValue(VARIANT).meta;
  }

  public IBlockState getStateFromMeta(int meta) {
    IBlockState state = this.getDefaultState().withProperty(VARIANT, LogType.fromMeta(meta & 3));

    switch (meta & 12) {
      case 0:
        state = state.withProperty(LOG_AXIS, BlockLog.EnumAxis.Y);
        break;
      case 4:
        state = state.withProperty(LOG_AXIS, BlockLog.EnumAxis.X);
        break;
      case 8:
        state = state.withProperty(LOG_AXIS, BlockLog.EnumAxis.Z);
        break;
      default:
        state = state.withProperty(LOG_AXIS, BlockLog.EnumAxis.NONE);
    }

    return state;
  }

  public int getMetaFromState(IBlockState state) {
    int i = 0;
    i = i | state.getValue(VARIANT).meta;

    switch (state.getValue(LOG_AXIS)) {
      case X:
        i |= 4;
        break;
      case Z:
        i |= 8;
        break;
      case NONE:
        i |= 12;
    }

    return i;
  }

  public static enum LogType implements IStringSerializable {
    // TODO: Add models for these guys
    OAK(0, "oak", MapColor.WOOD),
    BIRCH(1, "birch", MapColor.OBSIDIAN);
    // SAKURA(2, "sakura", MapColor.SAND),
    // DEAD(3, "dead", MapColor.SAND);

    public final int meta;
    public final String name;
    public final MapColor mapColor;

    LogType(int meta, String name, MapColor mapColor) {
      this.meta = meta;
      this.name = name;
      this.mapColor = mapColor;
    }

    @Override
    public String getName() {
      return name;
    }

    public static RGenMossyLogBlock.LogType fromMeta(int meta) {
      switch (meta) {
        case 0:
          return OAK;
        // case 1:
        //   return FIR;
        // case 2:
        //   return SAKURA;
        default:
          return BIRCH;
      }
    }
  }
}
