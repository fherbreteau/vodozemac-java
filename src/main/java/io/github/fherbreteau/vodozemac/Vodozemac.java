package io.github.fherbreteau.vodozemac;

/**
 * Entry point and utility class for the vodozemac library.
 * <p>
 * This class provides:
 * <ul>
 *   <li>Base64 encoding / decoding utilities, using the same unpadded base64
 *       variant as the underlying vodozemac Rust crate.</li>
 *   <li>Access to the version string of the bundled vodozemac crate.</li>
 * </ul>
 * Loading the native library is handled automatically when this class is
 * first accessed.
 *
 * @author François HERBRETEAU
 */
public final class Vodozemac {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    private Vodozemac() {

    }

    /**
     * Encodes the given bytes to a base64 string without padding.
     *
     * @param src the raw bytes to encode
     * @return the base64-encoded representation, without padding
     */
    public static String base64Encode(byte[] src) {
        return nativeBase64Encode(src);
    }

    /**
     * Decodes a base64 string to raw bytes.
     * <p>
     * Both padded and unpadded base64 input are accepted.
     *
     * @param src the base64 string to decode
     * @return the decoded raw bytes
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if the input is not valid base64
     */
    public static byte[] base64Decode(String src) {
        return nativeBase64Decode(src);
    }

    /**
     * Returns the version of the underlying vodozemac Rust crate.
     *
     * @return the vodozemac crate version (e.g. {@code "0.10.0"})
     */
    public static String getVersion() {
        return nativeVersion();
    }

    private static native String nativeBase64Encode(byte[] src);

    private static native byte[] nativeBase64Decode(String src);

    private static native String nativeVersion();
}
