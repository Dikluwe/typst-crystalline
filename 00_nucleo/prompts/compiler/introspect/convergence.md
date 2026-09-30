# L0 — introspect/convergence
Hash do Código: 49ad3608

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/convergence.rs`.
The body of `compute_tags_hash` at `:29` hashes the complete tag slice with
Rust DefaultHasher. `01_core/src/compiler/introspect/fixpoint.rs:129` compares
successive hashes. This is an internal structural change detector, not an
implementation of the production pipeline's semantic convergence policy.

## Contract

Keep `pub fn compute_tags_hash(tags: &[Tag]) -> u64` pure and stateless.
Hash the ordered slice through its Hash implementation, including payloads,
Locations and End hashes. Equal input in the same implementation yields the
same digest, including empty input. Order is significant.

No I/O, memoization, global cache or new dependency is required.
Do not promise a collision-free comparison, a stable persisted digest across
Rust versions, or a fixed named hashing algorithm behind DefaultHasher.
A changed payload, Location, End hash or ordering is a relevant change input;
representative differing inputs must be tested, not elevated to a mathematical
guarantee that all unequal inputs have unequal hashes.

## Acceptance

Cover repeatability, empty input, equivalent slices and representative changes
in payload, Location, End hash and order. The bounded loop and its errors belong
only to `00_nucleo/prompts/compiler/introspect/fixpoint.md`.
Language convergence and upstream parity need their own observable evidence.
