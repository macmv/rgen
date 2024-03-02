package net.macmv.rgen.block;

import com.google.common.base.Predicate;
import net.minecraft.block.BlockLog;
import net.minecraft.block.BlockPlanks;
import net.minecraft.block.material.MapColor;
import net.minecraft.block.properties.PropertyEnum;
import net.minecraft.util.IStringSerializable;

import javax.annotation.Nullable;

public class RgenLogOneBlock extends BlockLog {
    public static final PropertyEnum<BlockPlanks.EnumType> VARIANT = PropertyEnum.create("variant", BlockPlanks.EnumType.class, new Predicate<BlockPlanks.EnumType>() {
        public boolean apply(@Nullable BlockPlanks.EnumType p_apply_1_) {
            return p_apply_1_.getMetadata() < 4;
        }
    });
    public RgenLogOneBlock() {
        this.setDefaultState(this.blockState.getBaseState().withProperty(VARIANT, BlockPlanks.EnumType.OAK).withProperty(LOG_AXIS, EnumAxis.Y));
    }
    public static enum EnumType implements IStringSerializable {
        OAK(0, "oak", MapColor.WOOD),
        SPRUCE(1, "spruce", MapColor.OBSIDIAN),
        BIRCH(2, "birch", MapColor.SAND),
        JUNGLE(3, "jungle", MapColor.DIRT),
        ACACIA(4, "acacia", MapColor.ADOBE),
        DARK_OAK(5, "dark_oak", "big_oak", MapColor.BROWN);
    }
}

