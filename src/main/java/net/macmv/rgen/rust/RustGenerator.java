package net.macmv.rgen.rust;

public class RustGenerator {
  private static native String build_chunk(int x, int z);

  static {
    System.loadLibrary("rgen");
  }

  public static void make_chunk(int x, int z) {
    String output = build_chunk(x, z);
    System.out.println("rust said: " + output);
  }
}
