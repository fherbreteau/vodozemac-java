# Resource management

Every native-handle class implements `AutoCloseable` and should be used in a
try-with-resources block so the native memory is released deterministically:

```java
// Recommended: use try-with-resources
try (Account account = new Account()) {
    Curve25519PublicKey key = account.curve25519Key();
    // Use the account...
} // Automatically freed when block exits

// Manual management also supported
Account account = new Account();
try {
    Ed25519PublicKey key = account.ed25519Key();
} finally {
    account.close(); // Explicit cleanup
}
```

## Guarantees

- `close()` is **idempotent** — calling it more than once has no effect
- Using a handle after `close()` throws an `IllegalStateException`
- Methods that consume a handle and transfer its native ownership elsewhere
  (e.g. `Sas.diffieHellman(...)`) invalidate the original object as if it had
  been closed

## Cleaner safety net

As a best-effort safety net, a handle that becomes unreachable without being
closed is cleaned up by a `java.lang.Cleaner`: its native memory is released
and a warning is logged. The safety net is non-deterministic — always close
handles explicitly.
