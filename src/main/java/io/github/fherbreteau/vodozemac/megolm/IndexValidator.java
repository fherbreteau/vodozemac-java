package io.github.fherbreteau.vodozemac.megolm;

final class IndexValidator {

    private IndexValidator() { }

    static void checkIndexRange(long index) {
        long maxValue = Math.powExact(2L, 32);
        if (index >= maxValue) {
            throw new IllegalArgumentException("index must be a valid unsigned 32 bits integer");
        }
        if (index < 0) {
            throw new IllegalArgumentException("index must be positive");
        }
    }
}
