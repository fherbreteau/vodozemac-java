# Thread safety

All handle classes (`Account`, `OlmSession`, `OutboundGroupSession`,
`InboundGroupSession`, `Sas`, `EstablishedSas`, `Ecies`, `EstablishedEcies`,
`Ed25519KeyPair`, `PkEncryption`, `PkDecryption`) are safe to use from
multiple threads. Methods that access the native state are synchronized on
the instance, and `close()` can never race with an ongoing native call.

- Sharing a single handle across threads **serializes** the native calls on
  that object.
- Distinct handles can be used concurrently without any locking.
- `InboundGroupSession.connected/compare/merge` lock both operands in
  deterministic order to prevent deadlocks.

Value classes (`MegolmMessage`, `OlmMessage`, `IdentityKeys`, result types,
key types) are immutable and can always be shared freely.
