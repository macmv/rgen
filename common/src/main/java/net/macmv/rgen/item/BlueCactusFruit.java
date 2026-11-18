// # only v12 #

package net.macmv.rgen.item;

import net.macmv.rgen.block.RBlocks;
import net.macmv.rgen.block.BlueCactus;
import net.macmv.rgen.block.Cactus;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.init.Blocks;
import net.minecraft.item.ItemFood;
import net.minecraft.item.ItemStack;
import net.minecraft.util.EnumActionResult;
import net.minecraft.util.EnumFacing;
import net.minecraft.util.EnumHand;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraftforge.oredict.OreDictionary;

public class BlueCactusFruit extends ItemFood {

    public BlueCactusFruit() {
        // 2 half-haunches = 1 full haunch of hunger restored
        super(1, 0.3F, false);
        //this.setUnlocalizedName("green_cactus_fruit");
        //this.setRegistryName("green_cactus_fruit");

        // Register the item as a green dye in the Ore Dictionary
        OreDictionary.registerOre("dyeBlue", this);
    }

    @Override
    public EnumActionResult onItemUse(EntityPlayer player, World worldIn, BlockPos pos, EnumHand hand, EnumFacing facing, float hitX, float hitY, float hitZ) {
        ItemStack itemstack = player.getHeldItem(hand);

        // Check if the block being clicked is dirt or grass
        BlockPos blockpos = pos.offset(facing);
        if (facing == EnumFacing.UP && worldIn.getBlockState(blockpos.down()).getBlock() == Blocks.SAND
                && worldIn.getBlockState(blockpos).getBlock().isReplaceable(worldIn, blockpos)) {
            BlueCactus.Location location = BlueCactus.Location.values()[worldIn.rand.nextInt(BlueCactus.Location.values().length)];
            if (worldIn.setBlockState(blockpos, RBlocks.BLUE_CACTUS.getDefaultState()
                    .withProperty(BlueCactus.LOCATION, location))) {
                if (!player.capabilities.isCreativeMode) itemstack.shrink(1);
                return EnumActionResult.SUCCESS;
            }
        }

        return EnumActionResult.FAIL;
    }

}
