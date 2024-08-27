package net.macmv.rgen.block;
// This plant is for all cross plants that grown on sand
import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.BlockBush;
import net.minecraft.block.SoundType;
import net.minecraft.block.state.IBlockState;
import net.minecraft.init.Blocks;

public class CrossCactus extends BlockBush {
    protected CrossCactus() {
        this.setCreativeTab(RCreativeTabs.DECORATIONS);
        this.setSoundType(SoundType.PLANT);}
    protected boolean canSustainBush(IBlockState state) {return state.getBlock() == Blocks.SAND || state.getBlock() == Blocks.SOUL_SAND;}

}
