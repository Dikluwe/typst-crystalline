# L0 — pagebreak
Hash do Código: ad7a7863

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/footnote-entry-settings.toml sha256:69e358bd5dd06f821f96981bef9ab5c6b6d35b3bc592d2ee41d9f7ecf767698a
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/pagebreak.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/pagebreak.rs:16`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The composed-position candidate failed the existing explicit-footer page-reference test (historical probe P1159) (workspace.log archived in page-boundary-candidate.json.gz).
The CLI emits R8 instead of R7, while the heading itself remains on physical
page 1. pagebreak.rs:31-34 only flushes horizontal progress; a label captured
after a closed block has no horizontal advance and its private carrier stays
in current_line across the page transition. The page-reference regression
probe preserves both binaries and source. This is a capture-boundary defect,
not a reason to change logical numbering or the test expectation.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Consume an already-open empty page-run boundary without an additional ordinary break. Otherwise close any horizontally pending line, or a line containing a private paged position carrier, before requesting the existing regional transition. A carrier-only flush must remain neutral for line advance and page dimensions, but must associate those captured positions with the page being closed. Preserve ordinary overflow handling: do not flush pending content at every regional transition. For `to`, request the additional transition only when the next physical page index does not match parity. The weak-flag limitation is superseded by the correction below.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Weak pagebreak parity correction — 2026-10-23

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/baseline.json` binds the uncommitted source/L0/config state to HEAD, dirty stat and UTC; `before/receipt.json` binds both verified executables, inputs, commands and observations. `pagebreak.rs:21-58` ignores weak and always requests transitions; `finish.rs:103-106` requires the newly opened page unless it is a PageRun boundary. `page_run.rs:126-131` unconditionally closes its body. The pinned source `lab/typst-original/crates/typst-library/src/layout/page.rs:554-557` explicitly defines skipping weak breaks on empty pages; its tests `tests/suite/layout/pagebreak.typ:15-24,61-76` document weak/material distinctions. Initial, repeated, terminal and strong/weak controls plus parity and explicit PageRun probes distinguish language intent from mechanics. Page membership, content and geometry are observables; the private successor flag is a chosen mechanism. The inference that an optional successor suffices is refuted by lost notes/placed content, changed strong/parity empties, or leaked rollback state.

### Decision and exclusive responsibility

Skip a weak ordinary transition on an empty page, while still honoring `to` against the current physical page. A weak transition after material closes that page, but leaves its successor optional until material arrives. Repeated weak breaks do not force that successor. A strong break after an empty weak successor makes that successor required without producing another empty page. Parity adjustment commits intermediate pages deliberately and leaves only the final weak successor optional. Preserve carrier flushes and PageRun boundary consumption. Tests cover initial/terminal/repeated weak breaks, parity, strong/weak combinations, PageRun, notes, placed content and rollback.

This is an ADR-0127 continuous-flow language-parity correction: L0 first, reseal, RED→GREEN and complete revalidation. No public contract, default mode, pipeline phase or compatibility change is authorized. Existing nucleus obligations and unrelated limits remain binding.

### Material after a PageRun boundary

`lab/results/weak-pagebreak-2026-10-23/before-boundaries/receipt.json`, case `run-material-break`, measures `#page[One]Two #pagebreak(weak:true)Three` on the same verified baseline: Tekt puts Two and Three on one page whereas the pinned oracle separates them. `pagebreak.rs:21` consumes the PageRun flag without checking material already admitted since that boundary. Consume such a boundary only while the current page is still empty. This correction is owned here; no global content-dispatch reset is required.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Include admitted unpainted material in page emptiness. When a weak ordinary break is skipped on an empty required successor, its parity adjustment preserves that required-successor status; only an actual weak transition after material or a consumed weak PageRun boundary opens an optional successor. Preserve required initial and strong successor pages through parity adjustments.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.


## R12 — shared-flow footnote entry settings

Measurement before decision: `lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/{baseline.json,received-source.json.gz,before/receipt.json,before-expanded/receipt.json,type-probes.json}` binds the received R11 dirty tree at HEAD212aed044, fixed oracle bd5ebec285505889e8ff1d4f68804ab147a699f3/tree cac836c07e4dec2a89219370428292393a3216b5/SHA9b237315..., sources, binaries, UTC and exact changed-file list. Original indent/gap are ignored; explicit separator:none retains a rule. The short clearance control alone is not an admission witness. Expanded54 sources include actual three-font settings, affine units, aliases, text/string/line separators, shared versus local and late scopes, and literal continuation. Clearance30pt at80pt page height yields three vanilla pages (carrier, empty region, complete entry); received yields one. Local block overrides and late settings do not change the shared entry configuration; a global2em indent also applies to an entry called under a local0pt override. Pinned rules.rs:400–410 resolves indent inside the85% entry; flow/mod.rs:275–282 resolves separator/clearance/gap from shared outer styles, and flow/compose.rs:645–677,794–833,977–994 uses actual separator height and gap for admission/placement. Integer lengths including0 are rejected, string separator accepted, positional and unknown settings rejected. These observed geometry/syntax/diagnostics are language; storage and dispatch mechanics need not mirror upstream.

Classification: internal parity correction under ADR-0127 continuous flow. Keep all public entity fields, traits, signatures, modes and pipeline phases unchanged. New state is private layout state only, not a new language setting. Preserve the exact received R11 source and historical evidence outside this declared scope.

Exclusive owning obligation: At an explicit source flow boundary, after preserving the current pagebreak intent/flush semantics, invalidate only the private shared footnote-entry scope so the following source flow establishes its own prefix. Do not reset this snapshot on automatic pagination or nested ordinary subframes. Existing weak/strong/required successors, page-running plans and pending note ownership remain unchanged.

Require source-bound assertional module RED→GREEN before runtime changes; actual PDF complete per-page words/page sizes and SVG glyph/rule geometry at unchanged0.001pt in debug/release; preserve original R8–R11 corpora and all A–H/R1–R7 language/render controls, full workspace/tests/Clippy/fmt/rustdoc, full lint and both lineage gates, resource/stress gates and received evidence. Configuration warnings and old unrelated limits remain explicit.
