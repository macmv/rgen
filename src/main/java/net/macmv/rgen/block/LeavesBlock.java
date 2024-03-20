package net.macmv.rgen.block;

import net.macmv.rgen.tab.RCreativeTabs;
import net.minecraft.block.Block;
import net.minecraft.block.BlockLeaves;
import net.minecraft.block.BlockPlanks;
import net.minecraft.block.state.IBlockState;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;
import net.minecraftforge.common.util.Constants;

import java.util.Random;

public abstract class LeavesBlock extends BlockLeaves {

  public LeavesBlock() {
    // always have fancy leaves.
    this.leavesFancy = true;

    this.setCreativeTab(RCreativeTabs.BUILDING_BLOCKS);
  }

  private int[] surroundings;
  private final int[][] offsets = new int[][]{
    // vanilla offsets
    new int[]{-1, 0, 0},
    new int[]{1, 0, 0},
    new int[]{0, -1, 0},
    new int[]{0, 1, 0},
    new int[]{0, 0, -1},
    new int[]{0, 0, 1},
    // new offsets just for palm leaves
    new int[]{-1, 0, -1},
    new int[]{1, 0, -1},
    new int[]{-1, 0, 1},
    new int[]{1, 0, 1},
    new int[]{-1, -1, 0},
    new int[]{1, -1, 0},
    new int[]{-1, 1, 0},
    new int[]{1, 1, 0},
    new int[]{0, -1, -1},
    new int[]{0, 1, -1},
    new int[]{0, -1, 1},
    new int[]{0, 1, 1},
  };

  @Override
  public void updateTick(World world, BlockPos pos, IBlockState state, Random rand) {
    if (!world.isRemote) {
      if (state.getValue(CHECK_DECAY) && state.getValue(DECAYABLE)) {
        int originX = pos.getX();
        int originY = pos.getY();
        int originZ = pos.getZ();

        // Only use the extra offsets for palm leaves
        int maxOffset = 6;
        if (state.getBlock() instanceof LeavesBlock) {
          if (((LeavesBlock) state.getBlock()).getLogType(state) == LogBlockOne.LogType.PALM) {
            maxOffset = offsets.length;
          }
        }

        if (surroundings == null) {
          surroundings = new int[32768];
        }

        // Forge: prevent decaying leaves from updating neighbors and loading unloaded chunks
        if (!world.isAreaLoaded(pos, 1)) {
          return;
        }

        // Forge: extend range from 5 to 6 to account for neighbor checks in world.markAndNotifyBlock -> world.updateObservingBlocksAt
        if (world.isAreaLoaded(pos, 6)) {
          BlockPos.MutableBlockPos mutPos = new BlockPos.MutableBlockPos();

          for (int offsetX = -4; offsetX <= 4; ++offsetX) {
            for (int offsetY = -4; offsetY <= 4; ++offsetY) {
              for (int offsetZ = -4; offsetZ <= 4; ++offsetZ) {
                mutPos.setPos(originX + offsetX, originY + offsetY, originZ + offsetZ);
                IBlockState iblockstate = world.getBlockState(mutPos);
                Block block = iblockstate.getBlock();

                int i3 = (offsetX + 16) * 1024 + (offsetY + 16) * 32 + offsetZ + 16;
                if (!block.canSustainLeaves(iblockstate, world, mutPos)) {
                  if (block.isLeaves(iblockstate, world, mutPos)) {
                    surroundings[i3] = -2;
                  } else {
                    surroundings[i3] = -1;
                  }
                } else {
                  surroundings[i3] = 0;
                }
              }
            }
          }

          for (int distance = 1; distance <= 4; ++distance) {
            for (int offsetX = -4; offsetX <= 4; ++offsetX) {
              for (int offsetY = -4; offsetY <= 4; ++offsetY) {
                for (int offsetZ = -4; offsetZ <= 4; ++offsetZ) {

                  int i1 = (offsetX + 16) * 1024 + (offsetY + 16) * 32 + offsetZ + 16;
                  // if (offsetX,offsetY,offsetZ) is 1 below distance ???
                  if (surroundings[i1] == distance - 1) {
                    for (int offsetIndex = 0; offsetIndex < maxOffset; offsetIndex++) {
                      int[] delta = offsets[offsetIndex];
                      int offsetX2 = offsetX + delta[0];
                      int offsetY2 = offsetY + delta[1];
                      int offsetZ2 = offsetZ + delta[2];
                      int index = (offsetX2 + 16) * 1024 + (offsetY2 + 16) * 32 + offsetZ2 + 16;

                      if (surroundings[index] == -2) {
                        surroundings[index] = distance;
                      }
                    }
                  }
                }
              }
            }
          }
        }

        // check distance of (0,0,0).
        int l2 = surroundings[(0 + 16) * 1024 + (0 + 16) * 32 + 0 + 16];

        if (l2 >= 0) {
          world.setBlockState(pos, state.withProperty(CHECK_DECAY, false), Constants.BlockFlags.NO_RERENDER);
        } else {
          this.destroy(world, pos);
        }
      }
    }
  }

  private void destroy(World worldIn, BlockPos pos) {
    this.dropBlockAsItem(worldIn, pos, worldIn.getBlockState(pos), 0);
    worldIn.setBlockToAir(pos);
  }

  // FIXME: Maybe don't use BlockLeaves? Seems to work fine though
  @Override
  public BlockPlanks.EnumType getWoodType(int meta) {
    return BlockPlanks.EnumType.OAK;
  }

  public abstract LogBlockOne.LogType getLogType(IBlockState state);
}
