# L0 — compiler/stdlib/state/resolve
Hash do Código: 9cfe05a8

Layer: L1. Exclusive consumer: `01_core/src/compiler/stdlib/state/resolve.rs`.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:58a1e343cd346a842c0e0a2760c8b7438e89054b228d00a1b4972019320996e5


## Evidence before specification

L1 exclusive consumer `01_core/src/compiler/stdlib/state/resolve.rs`. `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/` before.json.gz/final-freeze.json.gz preserve HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, dirty tree/UTC/status/stat/sources. state-counterproof.json.gz bilateral RED against current `bd5ebec285505889e8ff1d4f68804ab147a699f3` proves reader init 1/10 same key after+1 yields 2/11, unread callback never executes, reused update executes per occurrence. Pinned state.rs:226-245,419-455,477-520 uses reader init and document order, not integer Location. Existing Func-as-Set/eager postpass loses semantics. Values/errors are language, storage/algorithm mechanics.

Earlier A1-S1 is withdrawn. Human A1-S2 gate confirms object reads; `lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`, SHA `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`. target-controls.json.gz state-future-error then refutes abbreviated prefix-only error scope: earlier get still errors FUTURE when later callback on demanded key panics. Pinned StateAtIntrospection state.rs:419-437 computes whole sequence before selecting offset; this does not imply executing other keys. pre-repair-validation.json.gz/complete-architecture.log identifies V2 missing local plan tests despite functional caller tests.

## Exclusive ratified contract

Privately resolve registered sequence initialized by reader. Apply Set by value, Func with one current-value argument/lexical capture/local Scopes; do not inherit reader in_context/current_location and restore both even on error. No Func execution during construction/walk/layout. Return SourceResult<Value>, modern route observes full result, no Solid/None/Empty fallback concealing update error.

Resolve entire demanded key's sequence, propagating any later callback error while retaining only selected prefix/final value and necessary accumulator. No read means no callback. This refines the historical “until limit” wording only for errors; approved signatures/phase stay. Do not retain vector of all intermediate Values unnecessarily, copy init/Set payload deeply per snapshot, promise global call count, create display-body/callback/new reads through replay/instrumentation, use global cache/I/O/dynamic dispatch/new public API or change field ownership. Cow<Value> may borrow init/Set and move/clone only for Func argument/selected result as needed.

Prepare legacy private read plan containing actions/shared init/causal limit before mutably borrowing EvalContext; do not clone whole StateRegistry for borrow workaround. No registered actions preserves materialized legacy prefix/final lookup for Rust fixtures/API.

## Acceptance and limits

R1–R12/order/prefix/error controls must refute before acceptance. Local plan tests cover key capture/nonmonotonic IDs/retained future actions for error/shared Set without deep clone; no-actions fallback prefix/final/missing key. These test plan invariants, not resolver mock/new contract. V2 is structural RED; local test must produce V2zero while retaining functional RED→GREEN evidence. Preserve language results/errors rather than bytes/algorithm, validate under 12GiB/High8GiB/swap0/jobs1.
