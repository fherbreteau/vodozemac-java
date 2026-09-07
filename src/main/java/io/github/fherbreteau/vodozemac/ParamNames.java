package io.github.fherbreteau.vodozemac;

/**
 * Shared parameter-name constants used in {@code Objects.requireNonNull} messages across
 * the bindings. Centralising them avoids duplicating the same string literal in every
 * class that validates the same logical parameter.
 *
 * @author François HERBRETEAU
 */
public final class ParamNames {

    public static final String PICKLE_DATA = "pickleData";
    public static final String PICKLE_KEY = "pickleKey";
    public static final String KEY = "key";
    public static final String MESSAGE = "message";
    public static final String PLAINTEXT = "plaintext";
    public static final String INPUT = "input";
    public static final String INFO = "info";
    public static final String MAC = "mac";

    private ParamNames() {
    }
}
