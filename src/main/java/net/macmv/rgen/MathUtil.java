package net.macmv.rgen;

import net.minecraft.util.math.AxisAlignedBB;

public class MathUtil {
  public static AxisAlignedBB aabb(int minX, int minY, int minZ, int maxX, int maxY, int maxZ) {
    if (minX < 0 || minX >= 16 || minY < 0 || minY >= 16 || minZ < 0 || minZ >= 16 || maxX < 0 || maxX >= 16 || maxY < 0 || maxY >= 16 || maxZ < 0 || maxZ >= 16) {
      throw new IllegalArgumentException("all arguments must be within 0..16");
    }

    return new AxisAlignedBB(minX / 16.0, minY / 16.0, minZ / 16.0, maxX / 16.0, maxY / 16.0, maxZ / 16.0);
  }
}
