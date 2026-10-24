# L0 — figure
Hash do Código: 5b57acbd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/figure.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/figure.rs:19`, `01_core/src/compiler/layout/figure.rs:70`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Read `figure.numbering` from the active chain: absence selects `1`, String supplies the pattern, None or another value disables numbering. For an enabled kind, consume the per-kind figure index and obtain the injected introspector number, with the existing index-plus-one fallback. Register the current page for LoF in the enabled-numbering branch, even if no caption is present. Format the number or decimal fallback and use the language owner's kind-specific supplement. Dispatch the body first; if a caption exists, emit a linebreak and dispatch the optional `supplement number: ` prefix plus rich caption. Do not own supplement tables, counter construction or outline fixpoint iteration.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Capture current_location before children. Prepared resolved_kind Figures with present numbered visual caption use typed key and that Location prefix from prepared snapshot, never event index or figure_progress. Missing productive Location/history appends `figure counter snapshot could not be verified` at nearest Source span and returns without fabricated prefix; a present empty state is not missing and can yield an empty formatted number. Use prepared resolved supplement including explicit empty/None. Retain figure_progress/index-plus-one fallback only for unprepared synthetic resolved_kind=None, including historical numbering-enabled/no-caption increments. Func numbering counts/demands; existing missing visual formatting remains explicit debt, no fabricated decimal parity. Rendering still delegates body/caption.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


### Rich prepared caption supplement

The approved resolved supplement is Content. Flattening it with plain_text before emission loses language morphology (for example Strong text), violating ADR-0107. Preserve that Content in the private caption prefix and compose it with a plain numeric/separator Content node; existing synthetic String prefixes become Content::text. The private sole-caller layout_figure prefix changes Option<String> to Option<Content>, with no public gate. Require bold supplement morphology and ordinary number/caption preservation. Function number formatting remains separate debt.


### Figure block paragraph boundaries

Root's frozen target caption control reports pinned oracle `A B Figure 2` versus candidate `AB Figure 2`: consecutive Figure bodies were merged into one paragraph. Figure is a block language object independently of whether it has a caption. Add a rendered text and baseline-order RED control for two prepared captionless Figures followed by ordinary text before changing dispatch. The bounded correction must end the preceding paragraph before each body and finish the Figure's emitted paragraph before following content, preserving numbering/action/snapshot identity and existing body/caption order. No public API, new mode or eval/layout transfer; this is an internal morphology correction at the owning renderer.

Functional RED was observed by the orchestrator in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/integration-tests-6/tests.log`: `figure_blocks_keep_adjacent_bodies_and_following_text_in_separate_paragraphs` emitted ALPHA, BETA and TAIL at the same baseline. Use the existing paragraph owner before the Figure body and after its final emitted body/caption; perform no paragraph mutation before a missing productive snapshot has been rejected.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Route FigureBody, FigureCaption and actually selected FigureSelectedSupplement through exact source-plan edges while retaining body/prefix/caption ordering, snapshot validation, typed numbering and paragraph boundaries. A generated numeric/separator prefix is nonadmitting; the rich selected supplement and caption keep their own source routes inside generated wrappers. Do not infer an edge by comparing Content, execute a dormant supplement or count technical caption wrappers as new language elements. Preserve the legacy synthetic helper entrypoint and all existing fixtures.
