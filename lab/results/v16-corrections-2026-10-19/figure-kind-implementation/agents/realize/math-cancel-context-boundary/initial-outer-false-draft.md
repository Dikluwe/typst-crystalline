# L0 — infra/pipeline/paged
Hash do Código: 63261524

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:389926b89934d625412cbbb22bad324ea6579533d79aa6b2dcf92fa5877095e4
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf




**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/paged.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/pipeline/paged.rs:188` — `compile_to_paged_document_full_error_and_features_with_introspector`.
- `03_infra/src/pipeline/paged.rs:686` — `bind_logical_page_numbers`.
- `03_infra/src/pipeline/paged.rs:712` — `realize_page_numberings`.
- `03_infra/src/pipeline/paged.rs:318`, `:390`, `:521` — the ordinary page-numbering cycle performs two grid layouts with a fresh `FallbackFontMetrics` each time. The local GDB probe in `lab/results/construction/diagnostics/grid-cache-real-2026-10-03.md` measured 962 shaping-cache misses for distinct text versus 5 for repeated text; `C000` misses used the same size/font key in separate cache instances. This is evidence of repeated internal work, not a new vanilla language-parity requirement. The proposed cache lifetime is refuted if a fresh cache is necessary for changed World or if text/geometry/diagnostics change.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/paged.rs`.

## Owned contract

- Own paged compilation, runtime introspection, outline realization, callback coordination, PageStore construction and final logical-number binding. Only complete validated candidates may reach shaping and export.
- Forward explicit features unchanged to evaluation; compatibility wrappers use the empty set. Features, target, PDF tagging and stream modes remain independent axes.
- Delegate selective contextual preparation to its child. A selected result already supplies final content, introspector, document and warnings: do not run a second contextual/page-numbering loop. Without selection, preserve the ordinary path and its errors.
- Realize outlines through L1 using each occurrence's lexical snapshot. After realization, discover newly produced context blocks with causal Locations and expand before layout. Remove only markers of already executed causal producers from the discovery view, not sibling results; do not replay old producers or identify them solely by numeric id.
- Coordinate independent math and `par.line` stores outside L1 layout. Pending requests are realized with their captured style/span and Engine, then replayed under the existing finite eight-attempt budget. Math angle requests receive the default Angle and must return Angle; line numbering receives exactly one integer and uses canonical `value_to_content`.
- New transcripts or partially consumed stores invalidate the candidate. Errors, invalid casts or exhausted callback budgets prevent export. Rebuild stores for every external relayout; never cache them across compilations or merge their budget with introspection fixpoints.
- Within one ordinary paged compilation and the same World, reuse the `FallbackFontMetrics` cache allocation across page-numbering relayout candidates by cloning its cache-sharing handle. Keep each candidate, callback transcript/store, and introspector fresh as before. Do not retain metric caches across compilations; the selected-path extensions are specified below. Cache reuse must preserve language-visible text, geometry and diagnostics; it is an internal optimization, not a semantic equality rule for `Content`.
- The final line-position reconciliation of that ordinary compilation may borrow the same metrics handle after the numbering loop. The GDB cache probe in `lab/results/construction/diagnostics/grid-cache-real-2026-10-03.md` observed a separate third cache instance for the same `C000` width key in reconciliation (`03_infra/src/shaper/reconciliation.rs:234`). This predicts redundant internal shaping; the hypothesis is refuted by changed output or no decrease in misses/time. The selected-path extension is specified below; other callers retain their existing metrics behavior. No cache survives the compilation.
- The pre-shaping bidi pass currently receives fresh metrics at `03_infra/src/pipeline/paged.rs:477`, while `03_infra/src/layout_bidi.rs:128-173` asks `FontMetrics::text_width` on RTL words. In the same ordinary compilation and World, it may borrow the layout metrics handle as reconciliation does, avoiding widths already measured in layout. The selected contextual result may borrow its transported handle under the extension below. This is an internal cache-lifetime experiment: preserve bidi order, text, geometry and diagnostics, and retain the change only if a real RTL A/B shows a repeatable benefit.
- Realize visible page numbering with `[current,total]`; realize the unary reference view `[current]` only for actual resolved page-reference targets. Preserve raw numbering objects separately from realized content and propagate callback diagnostics.
- The page-numbering cycle is bounded to five passes and checks page counts, logical snapshots, raw objects and realized content, not page count alone. Reconstruct from original input; never accumulate markers, callbacks or warnings.
- Preserve zero-size causal context markers for layout position sealing. Convert layout-warning strings to detached `SourceDiagnostic::warning` in L3 and retain them through errors.
- Associate L1-collected format options with each complete candidate. Return its realized PageStore, not a pre-layout clone; inject final positions after shaping. Bind every page's logical number from the aligned final store, failing on mismatch.
- Keep semantic wrappers and all page layers intact. Resource collection and shaping must include background, items and foreground. HTML does not enter this global paged path.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Realization index and unchanged outline reuse

`03_infra/src/pipeline/paged.rs:242` rebuilds an index already produced by realization. Consult Module::realization_introspector, governed by `00_nucleo/prompts/core/entities/module.md`, and clone its snapshot when available; otherwise introspect intr_content as before. Preserve visual bibliography conversion and initial bibliography_styles injection. Later contextual/runtime/position/page indices remain this consumer's responsibility; never replace them with the initial snapshot. Verify counters, labels, original/generated content, columns, context and geometry.

`paged.rs:90–109` introspects prefixes even when realize_outlines preserves Content; `00_nucleo/prompts/core/compiler/eval/rules/outline.md` requires this carrier preservation when no rule applies. The coordinator may reuse the received introspector and finish the same phase after proving immutable-root continuity with Arc::ptr_eq for compatible Sequence/Styled/Located/Realized roots. Without proof retain the former path. Content Hash/PartialEq is not proof or language identity by address; do not duplicate outline rules in L3. Actual prefix/error/callback/context production keeps its existing flow. The helper may be pub(super) for pipeline descendants' tests, never public L3 API. This is internal continuous flow.

## Reusing a page-independent complete candidate

`paged.rs:364–453` constructs PageStore then repeats runtime/expansion/realization/layout without page dependencies. `lab/results/construction/diagnostics/grid-pagination-cost-2026-10-05.md` measured two calls and about15ms CPU for the second320-cell call in its historical snapshot; this alone proves no causal gain. `lab/results/construction/diagnostics/layout-inicial-2026-10-05/before-state.json.gz` identifies the baseline source/binary.

Reuse the first complete candidate only with a positive structural proof over both original contextual and final realized content, and absent numbering on every final page. Visual equality, a single page or source-text inspection is insufficient. Preserve PageStore, warnings and page/position injection before ending the cycle, final binding, format options, bidi, shaping and reconciliation. Do not affect prepared.settled or bundles.

The borrowed, nontransforming proof accepts only Empty/Text/Space/SpaceFromNewline/Parbreak/Raw/HSpace/VSpace/Pagebreak/Colbreak/Linebreak; Sequence/Par/Strong/Emph; Styled with only text.font/text.size/par.linebreaks, page.width/page.height/page.margin-left/page.margin-right/page.margin-top/page.margin-bottom, eval.flow.scope; geometry scalars, flow Bool and font Str or Array of Str; Located and Realized covering original and visual. Table/Grid covers every cell body, caption and header/footer; other fields are already materialized data. SetPage accepts physical geometry/styles but rejects present numbering, explicit supplement, content marginals and visual layers. Explicit none/auto fields without a body introduce no dependency. Every unlisted variant fails closed, including ContextBlock/markers, Ref, Outline/Entry, Heading, math/callbacks, counter/state, bibliography, Dynamic, PageRun and residual ShowRecipe. Do not expand by inertia. Source is transparent only to its body under the ratified source-origin contract; unknown body still fails closed.

The multipage sentinel exposed page geometry/flow customs in eval/rules/set/page.rs and eval/markup_stream.rs:218; eval/rules/set/text.rs:269 normalizes text.font to Array. This evidence precedes admitting those exact keys and literal Str-family arrays. Dictionary/regex fonts and other keys keep fallback.

A measured local-cell refinement in `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/layout-call-counts.json.gz` records release SHA `4d63c08cb3a01aed112ac4612a6e6413894284ff579626c4c277b818e1ba18be`, HEAD/uncommitted status/diffstat/UTC/sources/commands/GDB script. At 1600 cells local-set resolves columns/rows twice with3200 subframes versus no-fill once/1600; the same relation holds at100. grid-invocation-traces-deep.json.gz locates both local resolutions in the ordinary coordinator. paged.rs:30–44 rejects all Cell custom keys and eval.table.local.recipe; :434–468 reruns after PageStore. content.rs:202–211 captures recipe body/rules/styles/guards. The rejection's causal cost is an inference, refuted without fewer layouts or paired time benefit; visible color alone proves no independence.

Permit additionally only physical scalar cell.fill (Color or None), eval.table.local Bool(true), and eval.table.local.recipe holding a ShowRecipe whose body passes this proof, with no local rules or guards and captured collapsed style delta containing only certified scalar properties and physical cell.fill. The wrapper body must independently pass. A recipe remains lexical data, never directly layoutable. Other cell.* properties, recipes, rules, guards, functions, contexts, markers, counters, numbering or unknown keys keep the full cycle. No visual/Rust equality proof. Preserve PageStore/warnings/positions/binding and both input proofs; this only reuses an already complete candidate in the same phase/API/default.

Acceptance: RED→GREEN one wrapper call for simple/local multipage tables without numbering; two for numbering controls, local show/context recipes and all unproved inputs. Keep existing outline/context controls. A cfg(test) call counter is permitted, no production probe. Local GDB must confirm3200→1600 subframes; compare language/text/pages/boxes/raster bilaterally across23 fixtures and paired vanilla benchmarks. Reject changed geometry/errors/positions/numbering, no demonstrated benefit or unreduced local excess. Preserve exclusive ownership/static dispatch, no I/O/new signature/trait/flag/eval↔layout phase. C2/C/macro collections, suites/build/lint and restricted lineage validation remain required.

## Page snapshot evidence and ratified boundary

`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/measurement.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, the uncommitted tree and exact binaries. Historical vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`. `single` prints B1/0 F0 versus B1/1 F1; two pages total/final 0 versus2; initial update7 leaves total7 versus8; an intermediate update changes the earlier page (7/7 and8/7 versus1/7 and7/7); final step prints final1 versus2. Bundle prints1/7 in the first PDF versus1/1, and final7 in the second versus8. Mixed HTML returns1 versus7. `paged.rs:360–365,840–913` and `context_stabilization.rs:969–1010` use the last update as every page/body start; `compiler/stdlib/counter.rs:490–513` and `counter/display.rs:96–110` read the causal global final. Historical upstream `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963` combines document scope, updates and physical advances. This is language parity, without requiring upstream memoization mechanics.

Apply the pinned page-counter-snapshot nucleus. The human ratified `lab/results/construction/diagnostics/contexto-page-total-2026-10-05/l0-manifest.json`, SHA-256 `1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`: its ADR-0127 gate is satisfied, superseding the earlier pending gate and historical final scope exclusion only for this package. Preserve signatures, cycles and budget; do not guess page/total before the snapshot is known. Subsequent internal clarifications use continuous flow, L0 first and RED→GREEN.

Supply the pure fold with occurrences per body and the candidate contribution's pages/positions. Prepare Page snapshots after physical positions, then reconstruct and validate under the same budget. Never use the last update as every page's start, reevaluate the entire source, borrow another document's caches or move I/O into L1. Numbering/PageStore and marginals consume their relevant realized numbering; an export-local projection must not contaminate the global snapshot. Preserve producer error/warning retention. Other owners require their own current L0; this owner does not authorize counter handlers or render logic.

In the legacy paged path, realize Page snapshots after candidate positions; iterate context until both views and relevant positions stabilize within five attempts. Reintrospection must not erase snapshots before a new generation. Numbering receives its document's realized logical sequence/final, never last-update start+index. With contextual blocks both Page views participate in convergence; without blocks retain the former page-count criterion, with no extra pass solely to publish an unconsumed view.

## Metrics lifetime: evidence, scope and acceptance

`lab/results/construction/diagnostics/contexto-cache-fontes-2026-10-05/before-state.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted state and baseline binary `5bc45f4ec0972c6c46450c0cab3479e2ae2ef41c286c424201c1b7fc07823eb1`. GDB `count-baseline.json.gz` measured selected_distinct2243 queries/965 shaping computations after misses/two wrapper entries, selected_repeated8 misses, ordinary_control321 misses. `context_stabilization.rs:548` calls the wrapper without metrics; `paged.rs:656–661` creates fresh metrics when absent; `font_metrics.rs:71–96` creates Arc caches per instance and Clone shares them. `grid-cache-share-ab-2026-10-03.md:17–23` explicitly restricted the earlier scope to the ordinary path, with fresh contextual/bundle wrappers. The measured extension is not a correction of an assumed accidental restriction; the older fresh wrapper's origin remains unaudited.

The selected paged session may pass its existing metrics handle to layout. Share only text/size/style/font metrics for the same World and compilation. Every attempt keeps its own content, introspector, regions, callbacks, transcripts, sinks and budget. No cache of Content, frames or contextual execution, no retention across compilations/edits, and no source-key/font algorithm copied into pipeline. HTML/bundle and other callers retain their resources. This internal ADR-0127 continuous-flow change preserves public contracts, target, mode and phase; retain it only with demonstrated work reduction, A/B measurements and unchanged language/geometry/diagnostics.

`lab/results/construction/diagnostics/contexto-cache-reconciliacao-2026-10-05/before-state.json.gz` preserves the same HEAD/uncommitted state and baseline `17201a15e36b06bb3a9ac8adba0a98c61b3dc925c4ce1f118a39d26bce26cf05`. `count-baseline.json.gz` isolates fix_line_positions_with_metrics: selected_distinct321 queries/321 computations (643 total), ordinary_control320/zero. `context_stabilization.rs:14–17,660–663` drops the Session handle; `paged.rs:337–345,563–572` leaves None and reconciles with fresh metrics. The canonical owner `00_nucleo/prompts/infra/shaper/reconciliation.md` already permits an internal borrowed-metrics helper; equations and public ABI are unchanged.

Transport a clone with the accepted complete selected paged candidate. Prepared/prepare are pipeline-private; their lifetime expresses existing World/Source borrows. Only the existing settled branch transports metrics, including the accepted terminal result under proven nonconvergence. Preserve acceptance, errors, contributions, sinks and budgets. Unsettled results, HTML and bundles transport no handle. End lifetime in the paged coordinator. No extra execution/layout, frame/snapshot cache or cross-compilation retention. Output/position/diagnostic changes or no proven work/time reduction refute the change.

`lab/results/construction/diagnostics/contexto-cache-bidi-2026-10-05/before-state.json.gz` preserves the same HEAD/tree and binary `c35dc34f0f24466b0b7339fcb856d5fc59c6f8701308f54963a4a1704f7746b8`. count-initial.json.gz isolates bidi between reorder_bidi_document and shape_document: selected_rtl_distinct2057 queries/299 computations, ordinary_rtl2057/137. paged.rs:565–568 allocates fresh metrics despite an available handle; layout_bidi.rs:23–29,128–173 already consumes FontMetrics for words. Reusing layout widths is an inference refuted by unchanged work or changed output.

Borrow the selected handle for both fix_line_positions_with_metrics and the existing pre-shaping bidi call. This supersedes the temporary reconciliation-only isolation, while ordinary compilation retains its handle for both. No bidi algorithm/signature/phase/regions/callback/content/introspection change. Respect the pinned compilation-metric-cache. Require a cfg(test) sentinel RED on the selected fresh branch, A/B work/time measurement without attributing every CLI difference to bidi, and preserved text/positions/raster/diagnostics; PDF byte equality is not required.

layout_with_realized_math_callbacks_with_metrics may be pub(super) for selected-session use; other wrappers keep signatures and fresh defaults. A cfg(test) probe may count allocations only in the absent shared_metrics branch, never production. After the session uses explicit metrics, rg finds layout_with_realized_math_callbacks only at paged.rs:592 and its unused pipeline.rs:90 import: remove that private unused facade/import, preserving explicit-metrics and bundle-seed helpers, public reexports/signatures and remaining defaults.

## Certified bidi context

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{initial,probe}.json.gz` preserves bilateral/Unicode refutation sources/state/binaries. layout_bidi.rs:73–82,227–240 redetects base/invents separators; shaper/bidi.rs:34 redetects fragments, losing the ratified full input. This internal parity correction introduces no external public signature/phase.

For certified spans use local owning registries for auxiliary strings/maps per input Arc, then ordinary borrowed BidiInfo views, without unsafe/self-reference. Virtual LRE/RLE/PDF embeddings occur only where segment direction differs from base; retain real spaces and nonjustified Linebreak LF (justify=true is uncertified). Keep author controls. Resolve once per input/registry; paged reorder and shape share registry/views. Existing public wrappers may create a local registry.

Reorder uses real line intervals, exact segment/range intersections and end-of-line levels, never midpoint/invented spaces. Split mixed Text at original boundaries with style/subspan. Unknown sources/nontext items gain no certificate. Preserve the L1 physical anchor and real widths/advances; no suffix/coalescing fallback for certified context. Shape full-input runs for the original span, preserve sequential subrun offsets and exact TextShaped subspans; never redetect fragment direction or add context to TextStyle (cache already keys direction). Reconciliation uses the input base when known, not any RTL run. Keep prior None fallback. RED→GREEN covers 456 789 refutation, overrides/neutrals, extraction without auxiliary controls and bilateral measurement.

## Table insertion evidence, approval and remaining scope

Historical evidence in `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted state, UTC, status/diffstat, sources/binaries, argv/results and upstream pin `9f2b6e8715237cb086899a42873660fe744622e8`; oracle SHA `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt SHA `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, :1500–1530 rejects conflicts in order, :1665–1703 creates empty cells, :1945–1988 derives breakable. Tekt grid_placement.rs:91–112 prioritizes explicit cells and :262–270 knows only GridCell; realize/mod.rs:160–200 visits recipes bottom-up; table.rs:127–152 dumps wrappers without attributes. `order` measures zero cell effects when the table is replaced and zero heading effects when hidden by the cell; `aliases` measures LOCAL a, OUTER LOCAL a, NEW a. These are language observations, not mechanical/byte/Rust equality requirements. Unknown fields, hidden effects or wrong order refute the hypothesis; cache/pagination equivalence is not inferred.

The exact insertion package is ratified: `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3` records the owner's `comfirmo`; manifest SHA `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. This satisfies its gate for five public fields, two public carriers, one facade and the logical phase before Cell show. Isolated-cell defaults and fresh show-cell-reconstruct generation are separately ratified in `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, owner `confirmo`, manifest SHA `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. Preserve their exact scope; cell-set/style defaults remain incomplete. General Table defaults/fields (table-root-equality), rich functional styles/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA remain uncertified. Do not change legacy extra casts from the earlier grid package, syntax, flags or the global nonlocatable policy.

Acceptance remains RED→GREEN, suites/build/lint, restricted lineage maintenance, and D08A/B2, D05/D06/D07/query collections. Judge language, effects and real geometry; PDF bytes and whitespace normalization cannot close render acceptance.

Collections/substitutions/reconciliation consume only the prepared visual contribution. Preserve its plan/causal source without indexing hidden origins. No Table Engine in layout or extra source pass; existing callback/math/page budgets remain.

## Transparent origins and borrowed context discovery

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/tree/UTC/source/argv/binaries at historical9f2b6e8715237cb086899a42873660fe744622e8. The owner's transparent operation mirrors ShowRecipe/Located/Realized; Source must not become observable. Owner `confirmo` on2026-10-07 ratified `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. workspace-seventh/independent_table refutes treating Source as impure unknown: the positive page-independence proof delegates only to body; a span is not geometry. Unknown body remains fail-closed; no weakened observation comparison or effect independence.

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks-deep.json.gz` records binary/HEAD/tree/source/interposer/commands/UTC and sampling one in64 allocations. Under expand_realized_outline_contexts, TableElem::map_content appears in902 local-set-1600 and300 block-no-set-1600 samples. paged.rs:185–204 reconstructs twice just to compare ContextBlock identity, then discards both when no new producer. outline-context-red-gate.json.gz records 957331 requests at the stated HEAD/uncommitted state/command/UTC. A borrowed visitor saving copies without changing ordered producers/reexpansion is a refutable inference.

Replace only these two discoveries with a private read-only visitor preserving map_content projection: Source transparent; Located suppresses body-root callback but visits descendants; Realized visits visual only; ShowRecipe/Styled body; Sequence in order; resolved Table cells/caption, raw Table children/header/footer/caption; TableCell standalone/resolved visual or body. Emit a terminal ContextBlock exactly when its own callback would be visited. Other variants fallback via map_content, never assume unknown subtrees empty. Preserve ordered Arc::ptr_eq identity and replacement of old producers only when new ones exist. No public API/mode/phase/causal identity change. Compare old/new ordered IDs/identities in nested trees, language/PDF/raster, allocation gate and paired timing; continuous flow.

## Page-reference traversal without destinations

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/page-ref-empty-red-gate.json.gz` preserves UTC/tree/diffstat/commands/sources/fixtures/interposer and release `14d023d36fb765eb041f24f440eaad3bc5d67183325b611e4fd98502bd26815e` at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b` uncommitted. The≤539000-glibc-request gate failed at541535 local-set-1600; no-fill410264. cpu-style-current.json.gz has62/36 page_reference_pages samples. paged.rs:987–1004 reconstructs Content although empty extracted_label_pages makes every query negative. Exclusive cost attribution remains an inference refuted by GREEN without fewer allocations or no paired time gain.

If doc.extracted_label_pages is empty, page_reference_pages returns an empty set before traversal. Otherwise retain traversal, Page forms, Label resolution and page set exactly. This private L3 optimization preserves API/phase/default/reference semantics. Require allocation RED→GREEN, Page references with/without destinations and Normal form controls, equivalent output and paired timing; remove repeatable time regressions.

## Private adapters and Rust representation

paged.rs:340 and `lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/before.json.gz` preserve owner/source/HEAD/tree/UTC/diffstat; references-before.json.gz audits calls/imports/modules, prior suite/inventory hashes and pre-edit test list. Dead_code alone proves no functional uselessness. compile_to_paged_document_full_error is a pub(super) facade imported only under cfg(test) by its ancestor and used by pipeline/tests: gate its definition cfg(test), preserving test signature, Features::default and delegation. and_features/with_introspector remain productive with current features/context. Describe the adapter accurately: backends call the parameterized path.

paged.rs:373 (C097), `lab/results/construction/diagnostics/type-complexity-2026-10-15/before.json.gz` and red-clippy.log preserve state/source/owner hashes and RED. This measures positional Rust representation, not an AI benefit/functional bug/new parity. The internal success uses pipeline-private PagedCompilation with named document/introspector. Keep SourceResult separate from warnings with identical results/errors/timings; document-only facade projects .document, other callers .introspector. No public type/phase/resource change. Aliases require proven identical expansion; incompatibility/new public type needs another gate. Nominal type_complexity exceptions apply only to the three justified items of that batch, superseding earlier generic bans only there; no module/impl/crate suppression, raised threshold or suppression for other repair cases.

paged.rs:807 clippy::map_clone, `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` and initial-diagnostics.json preserve source/L0/HEAD/tree/status/diffstat/UTC/suggestions/spans. Replace shared_metrics.map(Clone::clone) with shared_metrics.cloned(), preserving lazy unwrap_or_else, absent-cache counter branch, same shared instance/cycle/World/callback budget and absence behavior. No new parity/performance claim.

These internal continuous-flow repairs add no public API/entity/field/dependency/default/phase/lint suppression. L0 precedes code; restrict lineage maintenance. Keep existing test lists/expectations/inputs/tolerances; selected diagnostics must go RED→GREEN, with unfulfilled_lint_expectations denied where relevant. Run full suites/workspace, lib/all-targets build, Clippy all-targets, Rustdoc/fmt/lineage and total inventory. Do not add tests mirroring cfg, removed bindings, aliases, field names or Rust syntax.

## Heading supplement before queries

`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/{final-freeze,target-controls}.json.gz` records HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/UTC/diffstat/source/argv at current pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. heading-raw-constructor-func passes baseline/vanilla but fails candidate: transport Styled changes func; query sees temporary causal wrapper; HTML ignores callback panic. walk.rs:930–945, paged.rs:419–427, html.rs:101–106 use pure walk setup. These are language values/errors, not upstream algorithm/structure requirements. The owner approved `lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`, SHA `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

Compose setup through the private runtime helper; preserve the existing private Paged/empty-features facade and add a private explicit-target/features variant for the contextual session. On initial snapshots, including realization snapshots, call only L1 Heading before prepare(context), propagating errors/warnings. Do not advance other callbacks. Preserve later complete runtime pass, metrics/layout/version/options/pagination/public APIs. Require ratified H1–H8 and bilateral RED before code. No public contract beyond Heading is authorized; five State helpers are separately approved. No public fields. Ordinary Styled/other callbacks retain boundaries. Validate workspace/doctests/build/Clippy/Rustdoc/fmt, V1/V5/V15/V26/reverse, no V16 increase; 12GiB/High8GiB/swap0/jobs1, sequential heavy commands. RAM benefit requires measurement.


## Figure runtime candidate diagnostic ownership

Current Figure implementation evidence `candidate-target-controls-2/receipt.json` and functional `integration-tests-8` bind the same sourced Hebrew-zero warning published twice for one demanded caption. The ordinary coordinator appends runtime introspection warnings at paged.rs:474 and again at :599 while replacing the candidate introspector. These are separate executions, but only the accepted candidate's runtime contribution is retained. The orchestrator approved this private integration repair under ADR-0127 continuous flow; no public signature, phase, fold algorithm or diagnostic policy changes.

Hold the complete runtime warning Vec beside the candidate snapshot. Each successful or failed replacement runtime computation replaces that generation's warning buffer; never append discarded validation-generation warnings. Publish the current buffer on ordinary branch success or its error exit, preserving original diagnostics and ordinary non-runtime warning contributions. Keep all runtime folds and callback execution, their source spans/traces/messages, five-attempt/page convergence, metrics lifetimes and selected-session path unchanged. No warning-text filtering, new dedup algorithm, result/effect cache or already-published flag. Require functional warning RED→GREEN, two independent compilations both warning, unread no-caption silence, and two same-text warnings at distinct real source spans both retained.

## Explicit contextual math-cancel callback execution

The unchanged full-suite RED in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/validation-final-3/tests-wiring.log`
and focused frozen rerun in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-focused-red/state.json.gz`
measure three legitimate `angle: a => context ... text.size` failures at the
context-known guard. Existing effective-style and script controls require the
sealed request snapshot (19pt and reduced 14pt), not isolated defaults.

Source evidence: paged.rs:941 constructs EvalContext::new with in_context=false;
:974–988 explicitly recognizes the returned ContextBlock and executes its delayed
closure without entering contextual evaluation. The existing fields.rs guard
correctly rejects text.size/lang where context is not known. This omitted caller
state became visible after that guard was repaired; weakening the guard would
violate actual outside-context Figure callback controls. The pinned
`00_nucleo/prompts/_nuclei/math/callback-realization.toml` pipeline-realization
claim already requires request execution through Engine/captured style outside
pure layout. This is an internal correction within that established boundary,
with no new phase, request/public field, API or callback budget.

Only while executing the explicitly returned ContextBlock's closure, set the
existing EvalContext.in_context flag to true. Restore the previous value before
propagating either success or error. Keep the outer ordinary Angle callback at
its existing noncontextual state. Preserve request.styles, span anchoring,
request order, exactly the existing closure invocations, Angle validation,
complete-store checks and all warning/error paths. Do not infer or invent a
current Location, introspector data, counter history or contextual request state.
Do not alter field guards, generic apply_func or pure layout.

Require unchanged effective-style, reduced-script and cross/background callback
regressions to pass after their actual RED. Independently measure guard controls
against the pinned oracle before deciding their expectations, including an
explicit contextual result followed by an ordinary style read to detect flag
leakage, plus genuine outside-context Figure callback rejection. Preserve
invalid-return/callback error spans, restoration and every prior callback/store
contract; no suppressed or weakened assertion is authorized.
