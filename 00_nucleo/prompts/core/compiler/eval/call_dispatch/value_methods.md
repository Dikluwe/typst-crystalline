# L0 — compiler/eval/call_dispatch/value_methods
Hash do Código: c1ef8f77

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/call_dispatch/value_methods.rs`. At HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` with a clean tree, `01_core/src/compiler/eval/call_dispatch.rs:722-1214` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contract

Dispatch static value methods and validate receivers, arguments and special forms. Preserve public signatures, diagnostics, interception priorities, traces and argument semantics from `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Extraction remains in L1 with static dispatch, without I/O or mutable global state.

## Acceptance

Preserve the L1 suite, call_dispatch tests and `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Ratified object-reader State dispatch

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

Migrate only State dispatch: update uses the internal Func/Set adapter;
get/final/at forward the already available Engine. The captured-value route keeps
direct-call semantics without name recognition. Preserve every other receiver,
validation and diagnostic. Only the historical State phase is superseded.

### Acceptance and boundaries

The object-reader gate is ratified for its five signatures and read phase.
L0 first, functional RED before code; GREEN against the stated pin for language
and independent controls for Tekt extensions. Do not broaden other public
contracts. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt,
V1/V5/V15/V26 and reverse lineage; no increase in V16. Limits:
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; heavy tasks sequentially. Measure consumption
without inferring global benefit from representation alone.
