# Migration from jOlm

Coming from the archived [jOlm](https://github.com/brevilo/jolm) (libolm JNA
bindings)? The repository carries a complete, class-by-class migration guide:

**[MIGRATION-GUIDE.md](https://github.com/fherbreteau/vodozemac-java/blob/main/MIGRATION-GUIDE.md)**

It covers:

- the class and API mapping (`Account`, `Session`, group sessions, SAS,
  PK signing → `Ed25519KeyPair` for cross-signing keys)
- the **pickle migration path** through the `unpickleLegacy` entry points
  (jOlm pickles are libolm pickles) and re-pickling with 32-byte keys
- error-handling and message-shape differences
- behavioral notes (one-time key cap, thread safety, Java 25 requirement)
- a step-by-step migration checklist
