/**
 * Defines the Java bindings for the vodozemac cryptographic Rust library.
 *
 * @author François HERBRETEAU
 */
module io.github.fherbreteau.vodozemac {
    requires java.logging;
    exports io.github.fherbreteau.vodozemac;
    exports io.github.fherbreteau.vodozemac.account;
    exports io.github.fherbreteau.vodozemac.backup;
    exports io.github.fherbreteau.vodozemac.ecies;
    exports io.github.fherbreteau.vodozemac.exception;
    exports io.github.fherbreteau.vodozemac.megolm;
    exports io.github.fherbreteau.vodozemac.olm;
    exports io.github.fherbreteau.vodozemac.sas;
    exports io.github.fherbreteau.vodozemac.types;
}
