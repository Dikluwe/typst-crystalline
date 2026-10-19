# L0 — compiler/introspect/from_tags/state_updates
Hash do Código: ec977787

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/from_tags/state_updates.rs`. The original logic was in `01_core/src/compiler/introspect/from_tags.rs:108-155` at HEAD `4e2a321ce`, clean tree, 2026-09-30 23:49:20 UTC. This is internal extraction; inspection is neither new parity evidence nor a performance measurement.

## Contract

Own post-walk StateUpdate::Func replay. Preserve location order, closure capture, contextual evaluation, uninitialized-value behavior and callback diagnostics. Preserve public contracts, pipeline phase, defaults, compatibility, static dispatch and L1 purity.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.


## Ratified State replay boundary

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
before.json.gz and final-freeze.json.gz under
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
preserve UTC/status/diffstat/sources. state-counterproof.json.gz records bilateral
RED: readers initialized to 1/10 with the same key require 2/11 after +1; an
unread callback never executes; reused updates execute per occurrence. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`; upstream
introspection/state.rs:226–245,419–455,477–520. The reader's init starts the
sequence; document order, not the Location integer, orders it. The measured
method treats Func as Set and eager postprocessing loses that semantics. These
are language values/errors, not Rust equality or a mandated algorithm.
The earlier State proposal was withdrawn; the owner ratified the object-reader
runtime gate in that session. Review:
`lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`,
SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`.

### Exclusive responsibility and partial supersession

Public apply_state_funcs preserves its signature and legacy explicit application
for Rust callers and existing fixtures. Automatic runtime/fixpoint no longer
calls it. Do not broaden it or use it to initialize a modern reader; demand-driven
functional reading belongs to the State resolver. This supersedes only the
historical automatic State phase, preserving explicit compatibility behavior.

### Acceptance and boundaries

The object-reader gate is ratified for its five signatures and read phase.
L0 first, functional RED before code; GREEN against the stated pin for language
and independent controls for Tekt extensions. Do not broaden other public
contracts. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt,
V1/V5/V15/V26 and reverse lineage; no increase in V16. Limits:
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; heavy tasks sequentially. Measure consumption
without inferring global benefit from representation alone.
