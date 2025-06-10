package net.macmv.rgen.rust;

import org.apache.logging.log4j.Level;

public class OwnedLog {
  public byte level;
  public String message;

  public Level getLevel() {
    switch (level) {
      case 1: return Level.ERROR;
      case 2: return Level.WARN;
      case 3: return Level.INFO;
      case 4: return Level.DEBUG;
      case 5: return Level.TRACE;
      default: return Level.OFF;
    }
  }
}
