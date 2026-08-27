package io.github.fherbreteau.vodozemac;

import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.nio.file.attribute.FileAttribute;
import java.nio.file.attribute.PosixFilePermission;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.Set;

/**
 * Handles loading of the native vodozemac library for the current platform.
 * <p>
 * The native library is extracted from the classpath (JAR resources) to a
 * temporary directory and loaded via {@link System#load(String)}. Owner-only
 * file permissions are set on the extracted library for security.
 * <p>
 * This class is used internally by the vodozemac bindings and should not be
 * called directly by application code.
 *
 * @author François HERBRETEAU
 */
public final class NativeLibraryLoader {

    private NativeLibraryLoader() {
    }

    private static volatile boolean loaded = false;

    private static final String OS_LINUX = "linux";
    private static final String OS_DARWIN = "darwin";
    private static final String OS_WINDOWS = "windows";

    private static final String ARCH_X86 = "x86_64";
    private static final String ARCH_ARM = "aarch64";

    /**
     * Loads the native vodozemac library for the current platform.
     * <p>
     * This method is idempotent — subsequent calls after the first successful
     * load are no-ops.
     *
     * @throws RuntimeException if the native library cannot be found or loaded
     */
    public static synchronized void loadLibrary() {
        if (loaded) {
            return;
        }

        String osName = System.getProperty("os.name").toLowerCase();
        String osArch = System.getProperty("os.arch").toLowerCase();

        String platform = detectPlatform(osName, osArch);
        String libName = detectLibName(osName);

        // First try to load from root of classpath (where Maven copies it)
        String resourcePath = String.format("/%s", libName);

        try {
            loadFromResources(resourcePath, libName, platform);
            loaded = true;
        } catch (Exception firstException) {
            // If that fails, try the platform-specific directory
            resourcePath = String.format("/native/%s/%s", platform, libName);
            try {
                loadFromResources(resourcePath, libName, platform);
                loaded = true;
            } catch (Exception e2) {
                e2.addSuppressed(firstException);
                throw new RuntimeException("Failed to load native library for " + platform, e2);
            }
        }
    }

    private static String detectPlatform(String osName, String osArch) {
        String os;
        if (osName.contains(OS_LINUX)) {
            os = OS_LINUX;
        } else if (osName.contains("mac") || osName.contains(OS_DARWIN)) {
            os = OS_DARWIN;
        } else if (osName.contains(OS_WINDOWS)) {
            os = OS_WINDOWS;
        } else {
            throw new UnsupportedOperationException("Unsupported OS: " + osName);
        }

        String arch;
        if (osArch.contains(ARCH_ARM) || osArch.contains("arm64")) {
            arch = ARCH_ARM;
        } else if (osArch.contains(ARCH_X86) || osArch.contains("amd64")) {
            arch = ARCH_X86;
        } else {
            throw new UnsupportedOperationException("Unsupported architecture: " + osArch);
        }

        return os + "-" + arch;
    }

    private static String detectLibName(String osName) {
        if (osName.contains(OS_LINUX)) {
            return "libvodozemac_java.so";
        } else if (osName.contains("mac") || osName.contains(OS_DARWIN)) {
            return "libvodozemac_java.dylib";
        } else if (osName.contains(OS_WINDOWS)) {
            return "vodozemac_java.dll";
        }
        throw new UnsupportedOperationException("Unsupported OS: " + osName);
    }

    private static void loadFromResources(String resourcePath, String libName, String platform) throws IOException {
        Path tempDir;
        Path tempLib;
        try (InputStream in = NativeLibraryLoader.class.getResourceAsStream(resourcePath)) {
            if (in == null) {
                throw new FileNotFoundException("Native resource not found: " + resourcePath);
            }

            // Create a temp file to extract the library
            if (!platform.startsWith(OS_WINDOWS)) {
                Set<PosixFilePermission> perms = PosixFilePermissions.fromString("rwx------");
                FileAttribute<Set<PosixFilePermission>> attr = PosixFilePermissions.asFileAttribute(perms);
                tempDir = Files.createTempDirectory("vodozemac-native", attr);
            } else {
                tempDir = Files.createTempDirectory("vodozemac-native");
            }
            tempLib = tempDir.resolve(libName);

            Files.copy(in, tempLib, StandardCopyOption.REPLACE_EXISTING);
            if (!platform.startsWith(OS_WINDOWS)) {
                Set<PosixFilePermission> perms = PosixFilePermissions.fromString("rwx------");
                Files.setPosixFilePermissions(tempLib, perms);
            }
        }

        // Mark for deletion on JVM exit
        tempLib.toFile().deleteOnExit();
        tempDir.toFile().deleteOnExit();

        System.load(tempLib.toAbsolutePath().toString());
    }

    static boolean isLoaded() {
        return loaded;
    }
}
