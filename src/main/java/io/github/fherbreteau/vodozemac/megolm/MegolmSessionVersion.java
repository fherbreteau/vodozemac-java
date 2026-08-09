package io.github.fherbreteau.vodozemac.megolm;

public enum MegolmSessionVersion {
    V1(1),
    V2(2);

    private final int value;

    MegolmSessionVersion(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public static MegolmSessionVersion defaultVersion() {
        return V1;
    }
}
