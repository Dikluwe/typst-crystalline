# L0 — Figure figure counters
Hash do Código: 7c3adec1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/figure_counters.rs`.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Own pub(crate) figure_counter_key(kind: &FigureKind) -> CounterKey using the registered native Figure constructor Selector::Element filtered by kind Value::Func or Value::Str; Name(table) differs from Element(table) and user counter("figure:table"). Own private prepare_figure_counters(content, intr, engine, observer) -> SourceResult<()> and a private complete-body variant if necessary. Collect actual visual demands, deduplicate by counter_keys_eq in first-demand order, call unchanged resolve_filtered_counter for complete histories in this transaction, and install snapshots atomically after every demand validates. Validate target exists, exact typed selector membership, numbered canonical automatic action and present staged prefix at that causal Location. Present empty Vec is valid; missing invariant errors `figure counter snapshot could not be verified` at demand span. Skip implicit Html, preserve Paged target/features. No partial snapshot, read-time callback, explicit CounterFold observation, shared result cache or result reuse between validation and retained transactions.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


### Existing scalar label projection

After all histories/demands validate successfully, refresh existing figure_number_for_label projections for prepared Figure labels only from matching demanded typed histories at the label's actual first Location. This adds no demand, callback, fold, action or string shadow state; empty state yields absent scalar, never zero. Remove an earlier candidate's prepared scalar when its current history is not demanded. Preserve explicit synthetic legacy label data. Numeric productive references still use typed key plus Location; this projection retains the existing public observer where a valid materialized history exists.


## Approved physical-page source construction

### Evidence before decision

Clean baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`; exact source/owner bytes, hashes, UTC and working-tree state are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/before-l0.json.gz`. Current-pin bilateral source/query evidence and independent source-driver anchors are in `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json` (SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`) and `lab/results/set-page-header-2026-10-21/agents/parity/index-driver.md`. Repeated physical raw marginals contribute distinct located semantic occurrences; counter/query order is background/header/body/footer/foreground. Dormant overwritten templates remain inert. These are language semantics/refusals, not frame scanning, Rust equality or execution-count parity. The human ratified the exact additive API/phase package in the frozen `lab/results/set-page-header-2026-10-21/agents/core/proposal.md` (SHA-256 `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2`).

### Exclusive responsibility

Finalize the existing demanded typed histories only after all body/running source stores and actions have complete admitted causal order. Reuse unchanged selector equality, strict demand validation and transaction retention; source and running demands share one candidate. Never repair a missing running action with synthetic ordinal captions, cached prefix or a separate counter fold. Preserve HTML implicit-rendering debt and actual per-body targets.

### Acceptance and preserved boundaries

Require owning L0 before productive code and actual functional RED→GREEN, complete source-packet/parent/authority proofs, fresh raw versus Located-alias controls, retained retries and no phantom dormant/measurement effects. Reuse the shared arena and existing strict read/Sink transactions. No additional public surface, default, field, phase or compatibility change beyond the frozen approved package; no global Frame introspection, copied histories, invented Location, I/O or observer weakening. Preserve old tests and immutable F/G receipts; maintain exhaustive closed decisions and validate full suites/build/strict lint/Clippy/Rustdoc/fmt/paths/forward and reverse lineage/Nucleus DAG. Unrelated bounded debts remain explicit.
