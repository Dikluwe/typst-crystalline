# L0 — entities/content/construction
Hash do Código: 17826cc5

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/construction.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own public `Content` constructors other than sequence normalization. They must preserve every signature, default, omission
state, and payload. Construction remains pure L1 and uses only closed native
variants plus the existing explicit `Dynamic` extension boundary. No registry,
property map, I/O, or rendering is allowed.

## Acceptance

All existing constructor, representation, morphology, and parity tests pass;
callers compile unchanged; the module includes local tests.
