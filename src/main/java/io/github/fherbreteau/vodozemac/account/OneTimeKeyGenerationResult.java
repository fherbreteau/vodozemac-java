package io.github.fherbreteau.vodozemac.account;

import java.util.List;

public class OneTimeKeyGenerationResult {
    private final List<String> created;
    private final List<String> removed;

    public OneTimeKeyGenerationResult(List<String> created, List<String> removed) {
        this.created = created;
        this.removed = removed;
    }

    public List<String> getCreated() {
        return created;
    }

    public List<String> getRemoved() {
        return removed;
    }
}
