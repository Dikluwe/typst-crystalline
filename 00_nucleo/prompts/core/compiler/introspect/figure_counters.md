# L0 — Figure figure counters
Hash do Código: dc9011e3

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
