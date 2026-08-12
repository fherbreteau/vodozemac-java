package io.github.fherbreteau.vodozemac.olm;

public class SessionKeys {
    private final String sessionId;
    private final String identityKey;
    private final String baseKey;
    private final String oneTimeKey;

    SessionKeys(String sessionId, String identityKey, String baseKey, String oneTimeKey) {
        this.sessionId = sessionId;
        this.identityKey = identityKey;
        this.baseKey = baseKey;
        this.oneTimeKey = oneTimeKey;
    }

    public String sessionId() {
        return sessionId;
    }

    public String identityKey() {
        return identityKey;
    }

    public String baseKey() {
        return baseKey;
    }

    public String oneTimeKey() {
        return oneTimeKey;
    }
}
