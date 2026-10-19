# L0 — Layout engine coordination
Hash do Código: ea99f568

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/mod.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/mod.rs:240`, `01_core/src/compiler/layout/mod.rs:711`, `01_core/src/compiler/layout/mod.rs:1129`, `01_core/src/compiler/layout/mod.rs:1219`, `01_core/src/compiler/layout/mod.rs:1418`, `01_core/src/compiler/layout/mod.rs:1435`, `01_core/src/compiler/layout/mod.rs:1455`, `01_core/src/compiler/layout/mod.rs:1923`, `01_core/src/compiler/layout/mod.rs:2218`, `01_core/src/compiler/layout/mod.rs:2446`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

Native dispatch and the state/checkpoint contract are specified here.

This is the exclusive owner of `01_core/src/compiler/layout/mod.rs`: Layouter construction and state, checkpoint representation/capture/restore, causal dispatch, public façade entrypoints, attempt coordination, compatibility helpers actually defined here, and terminal document composition. Child owners define their own layout behavior; importing or calling them does not transfer ownership to this prompt.

### State and injected dependencies

Keep the existing Layouter API, generic FontMetrics/ImageSizer injection, tracked introspector and runtime. No filesystem, network, clock, environment access or Func execution is introduced. The active region holds cursors, line start, emitted items, pending line, dimensions and horizontal fractions. PageConfig and region dimensions stay synchronized when a configuration is installed on an empty page.

Preserve the chain/cached TextStyle, locator seed/current Location, document language, page/column and cell contexts, parent bbox, read-only runtime, figure progress, enum/list grouping, smartquote and smallcaps state. Preserve existing deferred buffers for floats, cell tails, footnote bodies, attachments, page/line/equation numbering, alignment paths, repeat and fractional natural widths, decoration collectors, warnings and errors. The hub owns this storage and its lifetime; it does not re-specify child formulas.

Fractional Block allocation remains private optional state: absent allocation is not a zero allocation. Preserve the preflight re-entry guard, terminal fractional-tail flag, one-shot validated-prefix line arm, assumed ascent and inline ascent/descent accumulators. Child policies may set/consume these through the existing descendant interface; no public entity or alternative state machine is created.

### Causal dispatcher and transaction boundary

`layout_content_once` performs the single canonical `is_locatable` gate before its exhaustive Content match. An eligible occurrence advances Locator exactly once, makes current_location available to descendants and records its current physical page and point. Structural recursion and visual children do not allocate extra Locations by heuristic. Measurement does not add emission events.

Native dispatch remains static and exhaustive with thin descendant delegation in the render layer, as required by ADR-0109 form B. Keep the closed Content enum and the current explicit transparent/no-op branches for already-consumed metadata/state/counter updates, structural entry carriers, alignment points and HTML-only elements. Retain the existing Dynamic extension adapter without turning native dispatch into dyn/vtable/string/PropMap dispatch or moving render into entities.

When a stabilized float marker leaves a checkpoint pending, `layout_content` opens the transaction before the next occurrence and routes top-level dispatch to `00_nucleo/prompts/compiler/layout/flow_suffix.md`; nested dispatch remains part of the same atomic unit. The hub provides capture/restore, while `00_nucleo/prompts/compiler/layout/flow_suffix.md` owns acceptance/rejection/replay decisions and `00_nucleo/prompts/compiler/layout/cursor.md` owns regional progress.

### Checkpoint integrity

A FlowSuffixCheckpoint records the causal state and append-only tail boundaries of both active/cell regions, completed pages, collectors, floats, attachments, numbering/alignment/repeat buffers, warnings/errors, columns, and all associated geometry/style/runtime/locator state actually captured by this consumer. RegionTailCheckpoint restores cursor/line-start/dimensions and truncates emitted items, pending line and fraction boundaries together.

Rollback restores the complete speculative suffix, including already-emitted Place items and causal counters/positions, not merely current_line. It reconstructs Locator from the captured seed, restores callback-related numbering state and one-shot arms, truncates buffered candidates/fixups, and clears transaction/replay flags. Committed prefix floats and pre-boundary items remain outside rollback. Missing structural storage where a captured region/collector should still exist is a debug invariant violation, not invented replacement state. Do not copy the checkpoint into children, inspect future Content variants or apply generic translation outside rejected replay.

### Attempt coordination and public entrypoints

Preserve the existing public layout, introspector/metrics, callback-aware, locator-seeded, intrinsic HTML-frame and real-measurement façade paths and signatures. The metrics entrypoint returns SourceResult and diagnoses unresolved math callbacks rather than returning a placeholder document; the legacy FixedMetrics convenience entrypoint retains its current expect behavior.

The callback-aware coordinator installs fresh math and par.line pass state for each candidate Layouter. Each attempt restores the same supplied LocatorSeed and uses the tracked introspector, injected metrics/sizer and current known label/figure/table/page stores. Carry forward extracted page maps until they stabilize within the existing retry limit of five attempts. The source currently runs this convergence comparison regardless of the computed has_outline flag: do not restore an obsolete “no outline means exactly one attempt” promise.

Return Complete only with a stable candidate and successfully consumed math/par.line transcripts; otherwise return Pending requests without exposing the tentative document. A rejected candidate's transcript is discarded, not merged into its successor. Exhausting the retry bound returns the existing empty Pending request sets, not a completed candidate. The mathematical callback protocol remains owned by its math prompt; the par.line protocol belongs to `00_nucleo/prompts/compiler/layout/par_line_callbacks.md`. No eval ↔ layout phase change or layout-time Func execution is introduced.

Discover the first bibliography's supplied style/locale and construct a pure CSL cache only for an explicitly named or already-resolved style. Retain cache-conversion diagnostics in each candidate and the local fallback when no style is requested. `00_nucleo/prompts/compiler/layout/bib_csl.md` owns cache interpretation; `00_nucleo/prompts/compiler/layout/bibliography.md`/`00_nucleo/prompts/compiler/layout/cite.md` own rendering. The current BibEntry compatibility body formatter remains here, preserving conditional field order and organization-as-publisher fallback.

### Terminal composition

`finish` closes the final line through `00_nucleo/prompts/compiler/layout/line_commit.md`, preserving its causal last baseline/bottom. Request finite auto dimensions from `00_nucleo/prompts/compiler/layout/page_geometry.md` before terminal float/footnote emission. Drain pending floats and footnote bodies through Cursor hooks, retaining the bounded extra-region loop for remaining note bodies.

Create the final Page under the existing material/empty-page-run conditions. Apply descendant line-number, equation and horizontal/vertical alignment fixups in their current order, delegate explicit running layers and canvas construction, preserve fill/bleed/supplement, and initialize Page.logical_number to the physical index. Numbering in an automatic margin uses the injected known logical number when available, otherwise the physical index. Function numbering consumes already-realized visible Content; it is never executed here.

Preserve the terminal-only coordination for compound page-number patterns: record the page index, chosen current number and marginal geometry, then format with the final known logical total or physical page total fallback after all pages exist. Intermediate assembly belongs to `00_nucleo/prompts/compiler/layout/page_commit.md`; the hub does not absorb its consumer or duplicate a general page-transition policy.

Transfer pages, document language, attachments, label pages/positions, sealed occurrence positions, figure/table page vectors, warnings and errors into PagedDocument. Do not mutate the injected introspector or fabricate logical overlays belonging to the pipeline.

### Delegation boundaries

Line closure/allocation/physical line numbering: `00_nucleo/prompts/compiler/layout/line_commit.md`. Baseline/wrap/regional transition and marker hooks: `00_nucleo/prompts/compiler/layout/cursor.md`. Sequence preflight/grouping/spacing: `00_nucleo/prompts/compiler/layout/sequence.md`. Isolated sublayout: `00_nucleo/prompts/compiler/layout/sub_frame.md`; constrained/intrinsic measurement: `00_nucleo/prompts/compiler/layout/measurement.md`. Geometry/fixups: `00_nucleo/prompts/compiler/layout/placement.md`, `00_nucleo/prompts/compiler/layout/equation.md`, `00_nucleo/prompts/compiler/layout/page_geometry.md`; page layers: `00_nucleo/prompts/compiler/layout/page_running.md`, `00_nucleo/prompts/compiler/layout/page_canvas.md`. Child element behavior is specified by its own exclusive prompt, including `00_nucleo/prompts/compiler/layout/style_scope.md`, `00_nucleo/prompts/compiler/layout/space.md`, `00_nucleo/prompts/compiler/layout/parbreak.md`, `00_nucleo/prompts/compiler/layout/align.md`, `00_nucleo/prompts/compiler/layout/context_block.md`, `00_nucleo/prompts/compiler/layout/state_display.md`, `00_nucleo/prompts/compiler/layout/counter_display.md`, `00_nucleo/prompts/compiler/layout/counter_display_callback.md`, `00_nucleo/prompts/compiler/layout/outline_entry.md`. Column behavior remains in `00_nucleo/prompts/compiler/columns.md`, which is outside this sanitation scope.

The coordinate and math-callback nuclei pinned above remain the shared invariant authority; child calls are boundary references, not additional consumers or duplicated normative ownership.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/mod.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Manter match exaustivo magro; Space e SpaceFromNewline delegam à mesma unidade Space com origem Markup e Newline respectivamente, sem mudança de fase.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.
