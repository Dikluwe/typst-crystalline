# Independent final5 structural witness review

PASS. This audit reads actual Cargo JSON diagnostics and the frozen copied workspace without running Cargo or mutating productive code/L0.

Provenance: HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`, uncommitted working tree. Structural execution `2026-10-20T03:24:46.926795+00:00` through `2026-10-20T03:24:57.246022+00:00`; review `2026-10-20T03:26:54.468227+00:00`. Full dirty-file stat, exact changed-file list and input hashes are retained in `structural-final5-review.json`.

Receipt SHA-256 `df8b2d3191530f2e3e1d460fd8432ecf69d43d35598fffa40bce3fbecb158e7f`; compiler log SHA-256 `b126c2fff4586bd863283194c71fc3d8e384a838abe7a84ddd23f74a4c37dc13`. All 3555 frozen copied inputs equal current files. All 997 original productive Rust sources equal current files. The copied workspace changes exactly four actual enum declarations, adding one TektV16EvolutionProbe variant each to Content, Value, FigureKind and GridChildOrder; all other productive copied files are byte-identical. Worker hash matches its receipt.

Every selected diagnostic is read from the raw log, with a primary byte range inside the independently found exact match anchor in its owning function. Each deterministic owner path exists and matches its productive @prompt header.

| Productive source | Decision | Primary line | Raw scrutinee |
|---|---|---:|---|
| `01_core/src/compiler/realize/figure_kind.rs` | `fn semantic_children(` / `match content {` | 307 | `content` |
| `01_core/src/compiler/realize/figure_kind.rs` | `fn inspect_value(` / `match value {` | 218 | `value` |
| `01_core/src/compiler/realize/figure_kind.rs` | `fn semantic_children(` / `match descriptor {` | 468 | `descriptor` |
| `01_core/src/compiler/realize/figure_kind.rs` | `fn validate_grid(` / `match descriptor {` | 594 | `descriptor` |
| `01_core/src/compiler/introspect/figure_demands.rs` | `fn visit(` / `match content {` | 52 | `content` |
| `01_core/src/compiler/introspect/figure_counters.rs` | `pub(crate) fn figure_counter_key(` / `match kind {` | 27 | `kind` |
| `01_core/src/entities/elements/figure.rs` | `fn eq(` / `match self {` | 31 | `self` |
| `01_core/src/entities/elements/figure.rs` | `fn hash<` / `match self {` | 115 | `self` |

Raw Cargo exit is101, all 224 errors are E0004 and no other error code appears. The gate is the eight exact witnesses. Historical223 versus current224 is explained by the additional new Ref producer Value target match; final3 and final5 both have224. Error totals themselves are not a completeness gate.

This new review binds the owned counter.final Location guard (source SHA-256 `620d175d9f456c896b11b82c8be138216681087ec39846ea9563102f7d85ee85`) and repaired paged adapter (SHA-256 `1c1f745818f736e02e36d0c12272642f994d0b00baf96887dc711b59bb302e6e`), unlike historical final3. Source/L0 runtime semantics, unresolved parity debts, broader lint inventories and normalized lineage remain separate gates.
