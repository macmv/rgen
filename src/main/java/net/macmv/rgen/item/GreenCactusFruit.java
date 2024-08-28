package net.macmv.rgen.item;

import net.macmv.rgen.block.RBlocks;
import net.minecraft.block.state.IBlockState;
import net.minecraft.creativetab.CreativeTabs;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemFood;
import net.minecraft.item.ItemStack;
import net.minecraft.util.ActionResult;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraftforge.oredict.OreDictionary;

public class GreenCactusFruit extends ItemFood {

    public GreenCactusFruit() {
        // 2 half-haunches = 1 full haunch of hunger restored
        super(1, 0.3F, false);
        //this.setUnlocalizedName("green_cactus_fruit");
        //this.setRegistryName("green_cactus_fruit");

        // Register the item as a green dye in the Ore Dictionary
        OreDictionary.registerOre("dyeGreen", this);
    }

    @Override
    public EnumActionResult onItemUse(EntityPlayer player, World worldIn, BlockPos pos, EnumHand hand, EnumFacing facing, float hitX, float hitY, float hitZ) {
        ItemStack itemstack = player.getHeldItem(hand);

        // Check if the block being clicked is dirt or grass
        BlockPos blockpos = pos.offset(facing);
        IBlockState state = worldIn.getBlockState(blockpos.down());
        if ((state.getBlock() == Blocks.SAND) && facing == EnumFacing.UP) {
            worldIn.setBlockState(blockpos, RBlocks.JUVENILE_GREEN_CACTUS.getDefaultState()); // Place the juvenile cactus
            itemstack.shrink(1); // Use up one fruit
            return EnumActionResult.SUCCESS;
        }

        return EnumActionResult.FAIL;
    }

}
