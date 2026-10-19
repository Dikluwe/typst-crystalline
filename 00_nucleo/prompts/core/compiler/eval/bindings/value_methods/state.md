# L0 — compiler/eval/bindings/value_methods/state
Hash do Código: 64d3a70c

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/bindings/value_methods/state.rs`. Internal extraction measured at HEAD `ce27af795` with a clean tree at 2026-09-30 23:12:44 UTC. The source is this unit's parent consumer; extraction is neither a bilateral parity measurement nor performance evidence.

The extracted logic was in `01_core/src/compiler/eval/bindings/value_methods.rs:42-176` and `:655-682` at the indicated HEAD. This is an internal behavior extraction, not a new parity decision.

## Contract

Own State method evaluation, at-location/label resolution and context observations. Preserve validation order, arguments, spans and contextual behavior. Preserve the public contract, pipeline phase, default behavior and compatibility.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains from structural separation.


## Ratified object-reader State runtime

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

This supersedes only the historical State phase. Project functional update
through state_method_update without executing it. Get/at/final pass Engine and
mutable context to the approved helpers. Preserve validation before context,
label resolution/observation, spans and capture. Reject excess/named arguments
where exact arity is required; AST calls must not accept what captured methods
reject.

### Acceptance and boundaries

The object-reader gate is ratified for its five signatures and read phase.
L0 first, functional RED before code; GREEN against the stated pin for language
and independent controls for Tekt extensions. Do not broaden other public
contracts. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt,
V1/V5/V15/V26 and reverse lineage; no increase in V16. Limits:
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; heavy tasks sequentially. Measure consumption
without inferring global benefit from representation alone.
