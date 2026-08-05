package io.github.fherbreteau.vodozemac;

import org.junit.jupiter.api.Test;

import java.io.FileNotFoundException;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

class NativeLibraryLoaderTest {

    private static Object invokeStatic(Method method, Object... args) throws Exception {
        try {
            return method.invoke(null, args);
        } catch (InvocationTargetException e) {
            Throwable cause = e.getCause();
            if (cause instanceof Exception ex) {
                throw ex;
            }
            throw new RuntimeException(cause);
        }
    }

    private static Method detectPlatformMethod() throws Exception {
        Method method = NativeLibraryLoader.class.getDeclaredMethod("detectPlatform", String.class, String.class);
        method.setAccessible(true);
        return method;
    }

    private static Method detectLibNameMethod() throws Exception {
        Method method = NativeLibraryLoader.class.getDeclaredMethod("detectLibName", String.class);
        method.setAccessible(true);
        return method;
    }

    private static Method loadFromResourcesMethod() throws Exception {
        Method method = NativeLibraryLoader.class.getDeclaredMethod("loadFromResources", String.class, String.class);
        method.setAccessible(true);
        return method;
    }

    @Test
    void testDetectPlatformLinux() throws Exception {
        Method method = detectPlatformMethod();
        assertThat(invokeStatic(method, "linux", "x86_64"))
                .as("Linux x86_64 should map to linux-x86_64")
                .isEqualTo("linux-x86_64");
        assertThat(invokeStatic(method, "linux", "amd64"))
                .as("Linux amd64 should map to linux-x86_64")
                .isEqualTo("linux-x86_64");
        assertThat(invokeStatic(method, "linux", "aarch64"))
                .as("Linux aarch64 should map to linux-aarch64")
                .isEqualTo("linux-aarch64");
        assertThat(invokeStatic(method, "linux", "arm64"))
                .as("Linux arm64 should map to linux-aarch64")
                .isEqualTo("linux-aarch64");
    }

    @Test
    void testDetectPlatformDarwin() throws Exception {
        Method method = detectPlatformMethod();
        assertThat(invokeStatic(method, "mac", "x86_64"))
                .as("Mac x86_64 should map to darwin-x86_64")
                .isEqualTo("darwin-x86_64");
        assertThat(invokeStatic(method, "mac", "aarch64"))
                .as("Mac aarch64 should map to darwin-aarch64")
                .isEqualTo("darwin-aarch64");
        assertThat(invokeStatic(method, "darwin", "x86_64"))
                .as("Darwin x86_64 should map to darwin-x86_64")
                .isEqualTo("darwin-x86_64");
        assertThat(invokeStatic(method, "darwin", "amd64"))
                .as("Darwin amd64 should map to darwin-x86_64")
                .isEqualTo("darwin-x86_64");
    }

    @Test
    void testDetectPlatformWindows() throws Exception {
        Method method = detectPlatformMethod();
        assertThat(invokeStatic(method, "windows", "x86_64"))
                .as("Windows x86_64 should map to windows-x86_64")
                .isEqualTo("windows-x86_64");
        assertThat(invokeStatic(method, "windows", "amd64"))
                .as("Windows amd64 should map to windows-x86_64")
                .isEqualTo("windows-x86_64");
        assertThat(invokeStatic(method, "windows", "aarch64"))
                .as("Windows aarch64 should map to windows-aarch64")
                .isEqualTo("windows-aarch64");
    }

    @Test
    void testDetectPlatformUnsupportedOS() throws Exception {
        Method method = detectPlatformMethod();
        assertThatThrownBy(() -> invokeStatic(method, "solaris", "x86_64"))
                .as("Unsupported OS should throw UnsupportedOperationException")
                .isInstanceOf(UnsupportedOperationException.class)
                .hasMessageContaining("OS non supporté: solaris");
    }

    @Test
    void testDetectPlatformUnsupportedArch() throws Exception {
        Method method = detectPlatformMethod();
        assertThatThrownBy(() -> invokeStatic(method, "linux", "powerpc"))
                .as("Unsupported architecture should throw UnsupportedOperationException")
                .isInstanceOf(UnsupportedOperationException.class)
                .hasMessageContaining("Architecture non supportée: powerpc");
    }

    @Test
    void testDetectLibNameAllSupported() throws Exception {
        Method method = detectLibNameMethod();
        assertThat(invokeStatic(method, "linux"))
                .as("Linux should use .so library")
                .isEqualTo("libvodozemac_java.so");
        assertThat(invokeStatic(method, "mac"))
                .as("Mac should use .dylib library")
                .isEqualTo("libvodozemac_java.dylib");
        assertThat(invokeStatic(method, "darwin"))
                .as("Darwin should use .dylib library")
                .isEqualTo("libvodozemac_java.dylib");
        assertThat(invokeStatic(method, "windows"))
                .as("Windows should use .dll library")
                .isEqualTo("vodozemac_java.dll");
    }

    @Test
    void testDetectLibNameUnsupported() throws Exception {
        Method method = detectLibNameMethod();
        assertThatThrownBy(() -> invokeStatic(method, "solaris"))
                .as("Unsupported OS should throw UnsupportedOperationException")
                .isInstanceOf(UnsupportedOperationException.class)
                .hasMessageContaining("OS non supporté: solaris");
    }

    @Test
    void testLoadFromResourcesNonExistentResource() throws Exception {
        Method method = loadFromResourcesMethod();
        assertThatThrownBy(() -> invokeStatic(method, "/nonexistent/native/lib.so", "lib.so"))
                .as("Non-existent resource should throw FileNotFoundException")
                .isInstanceOf(FileNotFoundException.class)
                .hasMessageContaining("/nonexistent/native/lib.so");
    }

    @Test
    void testLoadLibraryIsIdempotent() {
        NativeLibraryLoader.loadLibrary();
        assertThat(NativeLibraryLoader.isLoaded()).isTrue();
        NativeLibraryLoader.loadLibrary();
        assertThat(NativeLibraryLoader.isLoaded()).isTrue();
    }

    @Test
    void testLoadLibraryFallbackFailure() throws Exception {
        Field loadedField = NativeLibraryLoader.class.getDeclaredField("loaded");
        loadedField.setAccessible(true);

        String originalOsName = System.getProperty("os.name");
        String fakeOsName = originalOsName.toLowerCase().contains("mac")
                ? "linux"
                : "mac";

        loadedField.setBoolean(null, false);
        try {
            System.setProperty("os.name", fakeOsName);
            assertThatThrownBy(NativeLibraryLoader::loadLibrary)
                    .as("Should throw RuntimeException when fallback also fails")
                    .isInstanceOf(RuntimeException.class)
                    .hasMessageContaining("Impossible de charger la librairie native");
        } finally {
            System.setProperty("os.name", originalOsName);
            loadedField.setBoolean(null, false);
            NativeLibraryLoader.loadLibrary();
        }
    }
}
