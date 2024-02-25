package net.macmv.rgen.rust;

public class RustGenerator {
  private static native String hello(String input);

  static {
    System.loadLibrary("rgen");
  }

  public static void foo() {
    String output = hello("foooo");
    System.out.println("rust said: " + output);
  }
}
