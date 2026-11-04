// # only v12 #

package net.macmv.rgen.rust;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.Locale;

/**
 * Loads the rust library.
 *
 * Release jars bundle the generator library (`rgen-jni-impl`) for each platform, which is extracted
 * to a temp file and loaded directly.
 *
 * Dev jars bundle nothing. They load the hot-reloading shim (`rgen-jni`) from `java.library.path`,
 * which is what makes `/reload` work.
 */
class NativeLoader {
  private static boolean bundled = false;

  /** Returns true if the bundled (release) library was loaded, which can't be hot reloaded. */
  static boolean isBundled() {
    return bundled;
  }

  static void load() {
    String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
    String arch = System.getProperty("os.arch").toLowerCase(Locale.ROOT);

    String osId = os.contains("win") ? "windows" : os.contains("mac") ? "macos" : "linux";
    String archId = (arch.equals("aarch64") || arch.equals("arm64")) ? "aarch64" : "x86_64";
    String file = osId.equals("windows") ? "rgen_jni_impl.dll"
        : osId.equals("macos") ? "librgen_jni_impl.dylib" : "librgen_jni_impl.so";

    String resource = "/natives/" + osId + "-" + archId + "/" + file;

    try (InputStream in = NativeLoader.class.getResourceAsStream(resource)) {
      if (in == null) {
        // Dev build: nothing is bundled, so use the shim on the library path.
        System.loadLibrary("rgen_jni");
        return;
      }

      // A fresh directory per launch, so two running games never collide.
      File dir = Files.createTempDirectory("rgen-native").toFile();
      File target = new File(dir, file);
      Files.copy(in, target.toPath(), StandardCopyOption.REPLACE_EXISTING);
      target.deleteOnExit();
      dir.deleteOnExit();

      System.load(target.getAbsolutePath());
      bundled = true;
    } catch (IOException e) {
      throw new RuntimeException("Failed to extract the rgen native library", e);
    }
  }
}
