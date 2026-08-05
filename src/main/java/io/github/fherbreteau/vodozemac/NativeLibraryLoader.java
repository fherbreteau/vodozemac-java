package io.github.fherbreteau.vodozemac;

import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public final class NativeLibraryLoader {

    private NativeLibraryLoader() {
    }

    private static boolean loaded = false;

    private static final String OS_LINUX = "linux";
    private static final String OS_DARWIN = "darwin";
    private static final String OS_WINDOWS = "windows";

    private static final String ARCH_X86 = "x86_64";
    private static final String ARCH_ARM = "aarch64";

    public static synchronized void loadLibrary() {
        if (loaded) {
            return;
        }

        String osName = System.getProperty("os.name").toLowerCase();
        String osArch = System.getProperty("os.arch").toLowerCase();

        String platform = detectPlatform(osName, osArch);
        String libName = detectLibName(osName);

        // First try to load from root of classpath (where Maven copies it)
        String resourcePath = "/" + libName;

        try {
            loadFromResources(resourcePath, libName);
            loaded = true;
        } catch (Exception e1) {
            // If that fails, try the platform-specific directory
            resourcePath = String.format("/native/%s/%s", platform, libName);
            try {
                loadFromResources(resourcePath, libName);
                loaded = true;
            } catch (Exception e2) {
                throw new RuntimeException(
                        "Impossible de charger la librairie native pour " + platform + ". Tried: " + resourcePath, e2);
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
            throw new UnsupportedOperationException("OS non supporté: " + osName);
        }

        String arch;
        if (osArch.contains(ARCH_ARM) || osArch.contains("arm64")) {
            arch = ARCH_ARM;
        } else if (osArch.contains(ARCH_X86) || osArch.contains("amd64")) {
            arch = ARCH_X86;
        } else {
            throw new UnsupportedOperationException("Architecture non supportée: " + osArch);
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
        throw new UnsupportedOperationException("OS non supporté: " + osName);
    }

    private static void loadFromResources(String resourcePath, String libName) throws IOException {
        InputStream in = NativeLibraryLoader.class.getResourceAsStream(resourcePath);
        if (in == null) {
            throw new FileNotFoundException("Ressource native introuvable: " + resourcePath);
        }

        // Créer un fichier temporaire pour extraire la lib
        Path tempDir = Files.createTempDirectory("vodozemac-native");
        Path tempLib = tempDir.resolve(libName);

        Files.copy(in, tempLib, StandardCopyOption.REPLACE_EXISTING);
        in.close();

        // Marquer pour suppression à la fin du programme
        tempLib.toFile().deleteOnExit();
        tempDir.toFile().deleteOnExit();

        System.load(tempLib.toAbsolutePath().toString());
    }
}
