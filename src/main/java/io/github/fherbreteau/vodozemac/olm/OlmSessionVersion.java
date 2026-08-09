package io.github.fherbreteau.vodozemac.olm;

public enum OlmSessionVersion {
    V1(1),
    V2(2);

    private final int value;

    OlmSessionVersion(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public static OlmSessionVersion defaultVersion() {
        return V1;
    }
}
