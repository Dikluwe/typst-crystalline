# L0 — format options
Hash do Código: 0acf14c9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/format_options.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/format_options.rs:215`, `01_core/src/compiler/layout/format_options.rs:249`, `01_core/src/compiler/layout/format_options.rs:19`, `01_core/src/compiler/layout/format_options.rs:20`, `01_core/src/compiler/layout/format_options.rs:21`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The source-origin baseline at `lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv and binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. This unit has the corresponding transparent operation in existing ShowRecipe/Located/Realized branches; Source must not leak as an observable element. Human approval was `confirmo`, 2026-10-07, with the contract preserved at `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. `workspace-seventh/bundle` refutes treating Source as a format container: root document-option collection traverses only body without increasing depth/container, preserving style/field validation and actual rejection of nested options.

Value-projection evidence: `01_core/src/compiler/layout/format_options.rs:34` (typst_type_name, emitted-v16). `01_core/src/entities/value.rs:59–195` has 39 materialized variants; the catalogue and payload forms are in the contemporary inventory rather than a historical future-variant table. `lab/results/construction/diagnostics/tekt-v16-value-projections-2026-10-16/before.json.gz` preserves HEAD, uncommitted tree, UTC, status/diff HEAD --stat, hashes and original owner/consumer text. `contexts.json` comes from the current AST checked against pinned SARIF; `classification.json` distinguishes nine emitted V16 and eight latent projections. Rust form/exhaustiveness is mechanics; language names, conversions, borrows and observables retain the existing contract. This is no new parity or performance measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own pure traversal collecting effective document.format.* options. Keep the canonical keys and defaults for SVG pretty, PNG ppi 144, HTML pretty, PDF pretty/tagged/pages/standards. Preserve independent option folding and lexical precedence: later effective values win; an empty set delta is neutral rather than resetting unrelated fields.

Traverse only the current transparent root Sequence/Styled paths; a document-format set nested in an unsupported container remains diagnosed through the existing error path, not silently applied. Preserve validated typed projections including tagged Auto/bool, positive PDF page selectors/ranges/arrays with empty selection distinct from absence, and accepted standard names. Reconstruct standards through the existing pure validated types and fail closed for impossible input forms.

The hub only reexports this helper. Target-specific export, feature flags, CLI options, PNG raster resizing and format I/O belong elsewhere; this collector does not change layout phase or page geometry.

Source-origin handling retains exclusive ownership and static dispatch, without I/O or a new signature/trait/flag or eval↔layout phase change.

`typst_type_name` retains diagnostic aliases Bool→boolean, Int→integer and Str→string. The other 36 Value variants form a nominal group delegating to `value.type_name()`, without changing the body. Do not unify this policy with the document constructor, which does not rename Str. Do not change collection, keys, order/precedence, validation, render, options, targets, default or container traversal. Names are observable error text, not a new claim of upstream intent.

Variants sharing a result retain a common body. The nominal list requires a decision if Value grows; do not duplicate bodies to remove V19. V19 notes remain informative: they record written alternatives, not executed tests, reachable combinations or an N× deficit. Choose positive/negative controls and limits by behavior; executing one arm does not prove every alternative. No allow/expect or severity/exception relaxation may hide V16.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Source-origin tests and C2/C/macro collections, builds and lineage are required before certification. Value control C6 covers aliases and delegation for none/auto/float/array/content/length and Value::Type.

Continuous flow under ADR-0127 permits no public signature/trait/field, productive new variant, product default, compatibility break or phase change. L0 precedes tests/implementation; restrict bilateral resealing and require valid V1/V5/V15/V26. Write module controls and confirm behavior before/after. Structural RED is absence of E0004 at all 17 selected points in a core copy with a sentinel variant added only to copied Value. GREEN requires E0004 at every point without stubs and exact removal of the nine selected V16 while preserving all other diagnostics. Require build, workspace suite, strict Clippy, formatting, Rustdoc and lineage under 12 GiB, zero swap and one Cargo job. The global gate remains open while external V16 exist. The parser false positive on explicit None is a separate investigation and must not be hidden in this consumer.
