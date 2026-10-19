# L0 — compiler/eval/call_dispatch/value_methods
Hash do Código: 0ab3643f

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

## Closed selector decisions and measured admission

### Evidence before decision

`lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/oracle-probes.json`
records the verified oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree, binary SHA-256, recipe receipt, UTC, HEAD and exact working-tree diffstat.
`source-evidence.json` beside it freezes the consulted source. Upstream
`lab/typst-original/crates/typst-library/src/foundations/selector.rs:108-124`
defines literal/regex validation, :178-194 accepts Selector for logical operands,
:204-241 and :285-297 use LocatableSelector for contextual boundaries, :354-364
casts element functions by identity and :421-450 separately validates locatability.
These are language admission/equality/selection obligations, not required storage
or function-pointer mechanics. Successful eval of a show expression records
construction only, not realization; actual selection requires a compile probe.

The local adapter at `01_core/src/compiler/eval/bindings/value_methods/selector.rs:20-40`
only admits heading/figure; the constructor at
`01_core/src/compiler/stdlib/foundations/selector.rs:34-91` admits other element
identities and preserves a separate Tekt String-to-Kind extension. Current-pin
probes refute treating the adapter's refusal as intentional language behavior.
Native page equality is true against an alias and false against page.where().
The catalogue admits engine-native page/op; the old equality address accessor
excludes this ABI. The native matcher exits on engine ABI before causal descent.
Inference: existing representations suffice for these corrections; a case
requiring new public identity or phase would refute that bounded inference.

### Owned contextual operand validation

Use the existing adapter's expanded operand admission without reevaluating the
receiver or arguments. Logical and/or admit all valid selector operands and keep
order. Within and before/after cast their boundary, then call the existing typed
locatability validator recursively before constructing the selector. A local
contextual boundary validator also rejects Regex leaves, including nested ones,
as text is not locatable; do not change the older validator's separate legacy
leaf contract. Expose this internal helper through the call_dispatch facade
so the AST adapter uses the same policy. A cell/page
boundary must fail with its locatability diagnostic at the supplied call span;
a nonlocatable base may still construct before/after/within and is rejected only
when a querying consumer requires its locatability. Keep named-argument, arity,
inclusive type and missing-argument precedence; do not execute element callbacks.

Tests first cover direct versus already-converted cell/page rejection, valid
locatable aliases, nested invalid boundaries, once-only argument effects and
prior argument-error precedence. This is measured parity inside existing internal
signatures, ADR-0127 continuous flow; no new public API/default/phase/compatibility.

### Structural and integration gates

L0 first and restricted reseal precede Rust changes. Preservation controls pass
before/after; semantic corrections require real failing tests before code. A real
copied-workspace enum extension must produce attributable E0004 at every selected
closed match after repair, including both pair axes; use no stubs. Keep V19 groups
visible and review their semantic domains. No suppression, scan/severity/exclusion
change or equivalent open fallback hidden in another syntax. Validate full tests,
paths, direct/reverse lineage, V15/V26, build, all-target Clippy, fmt, Rustdoc and
exact SARIF delta. Preserve all earlier contracts and separately reported debts.
