package net.macmv.rgen.rust;

public class RustGenerator {
  private static native void build_chunk(char[] data, int x, int z);

  static {
    System.loadLibrary("rgen");
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
}
