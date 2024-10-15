package net.macmv.rgen.rust;

public class PropValue {
  public String name;
  public int kind; // 0 -> bool, 1 -> int, 2 -> enum.

  // For bools.
  public boolean bool;

  // For ints.
  public int integer;

  // For enums.
  public String str;
}
