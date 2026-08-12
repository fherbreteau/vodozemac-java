package io.github.fherbreteau.vodozemac;

public final class Vodozemac {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    private Vodozemac() {

    }

    public static String base64Encode(byte[] src) {
        return nativeBase64Encode(src);
    }

    public static byte[] base64Decode(String src) {
        return nativeBase64Decode(src);
    }

    public static String getVersion() {
        return nativeVersion();
    }

    private static native String nativeBase64Encode(byte[] src);

    private static native byte[] nativeBase64Decode(String src);

    private static native String nativeVersion();
}
