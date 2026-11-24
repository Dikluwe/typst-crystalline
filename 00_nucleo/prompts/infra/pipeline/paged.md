# L0 — infra/pipeline/paged
Hash do Código: eb75424c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:966b8ac90f477d71d8a442f4139239da77c75c9f3e24e2c097a1609433cd7227
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/fonts/complete-shaping-products.toml sha256:d612a88c4d14b13ea8e783dc06a984e26cc2c9978983a93dbabc6fac1277c2e3
- 00_nucleo/prompts/_nuclei/fonts/raw-shaping-cache.toml sha256:ba89868d23ef2304111b39efef27f781b39b74141f68b4abdc048c0088e0ae61
- 00_nucleo/prompts/_nuclei/lab-legacy-physical-certificate.toml sha256:286d3ec27c85463e62b086abd2df51219197c0502164bac70a631898d7490868
- 00_nucleo/prompts/_nuclei/layout/physical-number-outline-eligibility.toml sha256:ab006ebec30a3716a79ce2f93bc1716f7ed6f09a49ed370450865897e613de56
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-product-reuse.toml sha256:69dd4f797e17336227f56e038f9d833a0c6eff81bfc8e6beef4cf268fec27430
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896
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

## Math-cancel callbacks receive known styles without Location

The unchanged full-suite failure log
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/validation-final-3/tests-wiring.log`
and focused frozen rerun
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-focused-red/state.json.gz`
expose three existing Tekt callback compatibility regressions at text.size's
context-known guard. Their raw math `a => context ...` spelling is not vanilla
syntax acceptance: pinned vanilla reports an unknown context variable. Preserve
the established Tekt delayed-ContextBlock unwrap behavior without advertising
that extension as vanilla compatibility.

Independent valid code-inserted plain callbacks are measured in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-context-oracle-v2/receipt.json`
and `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-context-language-review.json`.
Pinned `lab/typst-original/crates/typst-layout/src/math/cancel.rs:100–105` invokes
the outer callback with Context::new(None, Some(styles)). This establishes known
styles and no Location for the outer callback itself. The separate valid-script
measurement in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-context-oracle-v4/receipt.json`
observes lexical20pt in vanilla, while the established Tekt test requires reduced
14pt. Preserve that existing Tekt requirement in this bounded guard correction;
the conflict remains an unresolved parity debt requiring a separate decision,
not a newly ratified intentional divergence. Valid explicit context returns
Content and is rejected as an Angle by vanilla; keep that separate from Tekt's
existing unwrap extension.

Local source `03_infra/src/pipeline/paged.rs:941–988` constructs an EvalContext
with unknown context and invokes both callbacks against captured request.styles.
The context-known field guard is correct; the owning runtime caller omitted its
known-style state. The former outer-noncontextual hypothesis and interim patch
are preserved and refuted in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/realize/math-cancel-context-boundary/refuted-outer-false-hypothesis.json`.
ADR-0127 continuous flow permits this internal parity restoration inside the
existing request/Engine phase. The pinned math callback Nucleus remains unchanged.

Initialize the helper-local EvalContext with in_context=true for outer Angle
callbacks and the existing delayed ContextBlock unwrap. It has no caller context
to restore and must not escape this helper. Keep current_location absent: do not
invent a Location from Equation identity, a request span, counter history or
introspector state. Preserve effective captured request.styles, request order,
existing invocation count, span rewriting, Angle validation, complete-store and
transcript checks, budgets and warning/error propagation. Preserve the existing
Tekt Content::ContextBlock result path without broadening accepted result types.

Do not change field guards, generic apply_func, Figure callback context-none,
public requests/functions, pure layout or phase order. Require unchanged legacy
Tekt compatibility tests to pass after their recorded RED. Independently validate
the valid plain code-inserted top-level callback observing19pt, ordinary code
outside-context text.size/lang rejection and Figure no-context rejection, with
these language expectations measured against the pinned oracle. Retain the
existing Tekt reduced-script14pt assertion unchanged without claiming that it
matches the separately measured vanilla lexical20pt behavior. Do not change
style snapshots or script sizing in this correction. Preserve all
invalid-return, source-span, callback/store and prior validation controls; no
suppression, fabricated negative or weakened assertion is authorized.


## Closed conservative reuse proofs

Measurement before decision: clean baseline4382891f0d5f0839669597cb5c90a98f24cfb8bd paged.rs:15–20 accepts text.font Str or an all-Str Array; entities/value.rs:83–219 contains39 alternatives. paged.rs:55–123 proves24 physical/recursive Content classes and leaves96 unproved; entities/content.rs:233–1309 contains120 alternatives. paged.rs:214–237 compares pointers only for Sequence, Styled, Located and Realized root pairs before ordinary prefix observation. Exact immutable inputs and real compiler diagnostics are retained in lab/results/v16-corrections-2026-10-19/batch-g/structural-baseline-2/. These are mechanical sufficient proofs, not Typst equality or new parity claims.

The font predicate explicitly rejects the other37 Value alternatives. Preserve empty/all-Str Array acceptance and mixed-array rejection without catalog lookup, font resolution, evaluation or I/O. The arbitrary-string custom-key fallback remains unchanged. The initial-layout predicate explicitly leaves the other96 Content alternatives unproved. Retain every true/recursive case, transparent Source, both original and visual for Realized, raw/resolved Table/Grid/cell slots and SetPage marginal restrictions. False preserves the full layout/context/numbering cycle; it does not reject valid language or infer an empty render. The producer collector's visual-only projection is not this proof's contract.

The immutable-outline continuity proof keeps exactly four same-kind Arc::ptr_eq arms first. Its finite negative complement explicitly lists all120 current Content alternatives independently on both tuple axes, returning false after those four accepted pairs. No enum wildcard, structural/hash equality or extra qualified carrier is allowed. New-left/old-right, old-left/new-right and new/new inputs each require an attributable missing-decision witness. Rebuilt equal roots do not qualify through pointer identity. Preserve general prefix observation, optional second run, errors and causal warning order. This proof is a private immutable-carrier optimization, not language identity.

Acceptance preserves Str/empty/all-Str/mixed/nonfamily font inputs with inert Func/Content decoys; Source, both Realized branches, Table/Grid/SetPage restrictions; ordinary full-cycle output for valid unproved content; and the four shared outline carriers plus unqualified general-prefix handling. Existing rebuilt/prefix-change/error/warning controls remain binding. Independent enum-domain and tuple-axis compiler witnesses supplement runtime controls; grouped-arm execution cannot certify every nominal alternative. Keep PageStore, metrics lifetime, Source origins, known-style/no-Location math callbacks and modern counter.final guards unchanged. This scope is continuous internal exhaustiveness under ADR-0127 with no API, default, phase, compatibility, proof-whitelist or Nucleus expansion.


## Approved physical running-content candidate integration

Evidence before this decision is the immutable source-qualified current-pin
SetPage baseline and ambient20/30 measurements under
lab/results/set-page-header-2026-10-21. Header-only/repeated/footer/context/style
and duplicate-label failures are language observations; the twelve existing gates
passing is structural preservation only. Actual seven semantic REDs and one inert
template control are recorded in
lab/results/set-page-header-implementation-2026-10-21/initial-red/receipt.json.
Human approval of review SHA256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
is recorded in that implementation directory's approval.json. ADR-0127 items1/3
are satisfied for exactly the frozen five L1 facades/types and their phase/order.
This section supersedes earlier preserve-phase/default boilerplate only for the
approved physical channel. It adds no public L3 API, source reevaluation, new
solver/budget, generic observation/equality rule, or unrelated behavior change.

Use the approved owned Content/source plan and immutable request/resolution/trace
facades as opaque L1 transports. L3 coordinates the actual World, Engine, retained
arena frames, existing Context Session and candidate diagnostic sinks; no selector,
Figure-kind, semantic child-edge, temporal action, style-rebase or render business
logic moves to L3. The pure layout renderer executes no language closures. The
initial canonical body identity comes from the existing reached realization and
its protected operational templates; preparation publishes no invented body index
or PageStore. Only a matching actual trace/store pair builds the complete Paged
source-derived index. Global Bundle input uses actual ordered target metadata,
features and document seeds; HTML has no physical requests.

Keep the Session alive through physical discovery, including initially unselected
body producers. Actual Pending requests select the approved physical contribution
channel, without fabricating filtered/position reads or running unselected source
producers a second time. Raw operational SetPage/PageRun templates are excluded
from body discovery/substitution; realize only requested actual insertions in one
retained frame per actual request lineage/key. Inspecting a dormant template for
allocator reservation is not execution, query/counter admission or a new producer.
Register actual generated Context descendants in the same Session, preserve their
source/style/target/features, and rebind expanded Content through the approved L1
function under the shared frame. No synthetic operational Located wrapper/Context
query element or page-derived numeric Location is allowed.

Reuse a body plan only with unchanged actual retained contribution versions and
immutable source provenance. Content Eq/hash/Debug, equal counters/page counts or
freshly reconstructed closure equality are not evidence. New source generations
invalidate prior plan/request authority; pure layout reflow of the same retained
plan keeps its authority. Candidate-local stores/transcripts must be complete and
agree with current geometry/effective page environment; no partial candidate may
reach shaping/export. Maintain the existing five contextual/page attempts and
separate eight math/par.line request attempts, no hidden sixth execution or second
context solver. Discovery/replay/replaced sinks remain discarded; publish only the
retained final transactions through existing Sink policy, preserving source error
precedence and independently sourced identical warnings.

Own only this consumer's integration. Its existing private helpers and a private
descendant page_running module may coordinate the protocol; each new productive
file has its own canonical owner. Other owner's clauses do not authorize bodies
here. Preserve legacy facade contracts, non-running paths, PageRun lexical restore,
page snapshots and numbering, source parents/original morphology, HTML semantics,
Bundle validation/resource/write policy and all unrelated F/G debts. Require actual
focused GREEN and frozen CLI/counter/query/style/error/alias/dormant/template and
rollback controls, followed by full twelve-gate validation, canonical paths,
strict forward/reverse lineage, ownership/Nucleus pins/DAG and complete diagnostic
comparison. No suppression, test relaxation, opaque catchall or convenience global
hash reseal is authorized.

## Approved ordinary Repeat physical membership integration

The source-qualified paired measurement and exact human-approved review SHA-256 `2609d1a9231b1b8a665f2d8284b7343366a1bc44b0471fdd6008b0f180a52bb2` (`lab/results/nested-fractions-repeat-2026-10-21/phase-approval.json`) extend the existing physical running-content integration only to actual paginated Repeat contributions lacking marginals. Reuse the retained Session and existing source-plan/trace/store facades; semantic membership for these contributions follows actual accepted layout before stable export. Keep prior unselected/non-Repeat behavior, HTML semantics, mixed Bundle target metadata, source once, contexts, counters, labels, callbacks, numbering, sinks and attempts. A completed body with no marginal requests still needs its matching trace-derived source index. No public interface, generic equality, inactive-paint heuristic or source reevaluation is added. Require integrated real-source RED→GREEN and all prior phase/resource/diagnostic/lineage gates.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

## Final query snapshot after Stack fragmentation (2026-10-27)

Measurement before decision: `lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/pipeline-red.log`, with its sealed source and state, records twenty Context producers mapped to Locations0,2,...,38 in the expanded topology, while returned metadata still retains Here Locations0,1,...,19 and captured page1 from the initial pre-layout expansion. The twentieth physical context is correctly on page2. At `paged.rs:878`, intr_for_positions is cloned before the ordinary fixed point; `:1001-1013` constructs a fresh runtime candidate but updates only layout_intr, while `:1047-1050` copies merely page stores/counters to the initial snapshot. Queries therefore mix metadata from the initial generation with positions of the accepted document. Candidate1 already exhibits the stale captured page without the later child-local-index collision; keep both independent findings. This is observable query identity/page semantics.

Decision: within the existing ordinary loop, retain the fresh successful runtime introspection candidate as intr_for_positions before it is moved into realization/layout. Accepted final semantic metadata/Context identities must come from the same generation as the final layout content. Keep final positions, pages/counters and query publication on the existing path. Do not reexecute callbacks for publication, change the existing five-attempt/convergence rule, selected path, warning ownership, public facade, phase, metrics or defaults. Preserve existing extracted heading handling; any newly observed heading/ref/counter regression refutes the change and must be investigated, not weakened. ADR-0127 continuous internal parity repair applies.

Acceptance: integrated twenty-child RED→GREEN requires source-order metadata and both captured/final Here pages19×1 then2; independent JPEG, reverse, spacing and wrapper sources must agree with pinned vanilla. Full context/counter/state/ref/heading and all validation/resource gates remain mandatory.

## Approved readiness physical eligibility — plain numbering and heading Outline

### Measurement and human decision before code

readiness-fixes-2026-10-28/{number2-build.json,number2-build-source.json.gz,
number2-extras/comparison.json,article-route-before/receipt.json,
outline-route-debugger/receipt.json} bind exact binary/source/fixture hashes,
HEAD212aed0449150dd296ceea13fc2be665cc5db01a dirty stat and UTC. Product
92c6346f8562438618e2cd7ba57166423316ee26fc08aa597ac357c482859fd7;
oracle64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17,
executable22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The terminal17pt/DejaVu page loses lexical style after restoration; the selected
Ref source bypasses ordinary Outline realization and paints an incorrectly
numbered title without fill. Actual debugger function entries/piles distinguish
both routes and full compiles, rather than inferring them from appearance.
Readiness review documents exact source anchors and independent remaining debts.
These are language/style/source observations, not upstream Rust-mechanics parity.

The human directly approved “Aprovar a ampliação proposta” in this chat;
physical-number-outline-approval.json records authority, UTC and frozen review
SHA-2569d52d041904a94221cae76e4d9d394afd3a758be31e0e4b8845c6839a0dcdc87
with its owner/receipt manifest. ADR-0127 phase gate is satisfied ONLY for that
reviewed extension. This section supersedes earlier preserve-phase/noneligible
clauses only for its named heading-Outline/plain-number eligibility; prior
Repeat approval is not used as authorization. Existing APIs and other phases
remain unchanged. The approval does not certify tests, rendering or readiness.

### Exclusive implementation and acceptance

Consume the existing complete selected physical result for the newly approved eligibility exactly as for current rich-marginal/Repeat candidates. Keep accepted source/index/metrics lifetime and final transforms/format/query/diagnostic publication; do not evaluate the source or run ordinary Outline realization again over that completed result. Existing noneligible/HTML/Bundle entrypoints and their target-specific contracts remain unchanged. This owner owns coordination only; pure eligibility/language/render policy remains in its current descendants/L1 owners.

Stage real current-API REDs before code. Revalidate the approved exact source
variants, source words/pages/0.001pt and separate PDF/SVG, actual styles/query/
Location/effects once, causal errors/warnings, aliases/dormant/hidden producers,
None/Content, all old marginals/Repeat/rollback/notes/callback/Bundle/HTML gates.
Retain strict authority, coverage, Location and reflexive/current observation
proof; no extra attempt, normalization, suppression or weakened expectation.
V15/V26 before resealing, forward/reverse lineage, twelve technical gates,
full original historic corpus and JPEG time/RSS preservation remain mandatory.
Unresolved numbered-fill, justification, explicit-footer/bold-PDF and all other
readiness debts keep their separate receipts and acceptance; no general claim.

## Approved issue 8954 — exclusive carrier reader migration

The human-approved schema is recorded in approval.json, patch SHA
18e5019618e3c08e71c3a9d2f51a4f50d9b4a2e624fc70ab6d8de6c4aac6ee20
under lab/results/construction/diagnostics/issue-8954-2026-10-31/.
carrier-audit-before.json.gz retains exact owner/source/UTC/dirty state. Manual
Table readers currently inspect only legacy children/header/footer: with the
new ordered stream this incorrectly certifies empty/context-free raw subtrees.
In this owner's existing traversal/certificate use Complete visual cells, else
authoritative Some(items), else legacy children/header/footer, and caption once.
No parallel visitation, new allocation, context evaluation, phase or public API.
Retain existing policies and negative certificates for all other carriers.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Inject a private World-aware SVG preparation adapter implementing the extended ImageSizer port; delegate raster size/dpi/orientation to existing ImageSizeImageSizer. Preserve the public unit raster adapter and pipeline signatures. Preparation receives current compile World/font catalog, honors resource root/package restrictions, and does not reopen files in export. Do not introduce a product flag or default-mode change.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.

## Raw shaping reuse: evidence before decision

The shaping-input-overlap-lab-20261111 census on clean HEAD 48e5a2570159c6fc325babe883bac7d1c1fbd4b2 compared complete inputs and independently replayed 3,788 calls. In layout.typ, 111/111 emission misses had an earlier equal measurement among 113 measurement calls; math admitted only 2/9. Tracking coincidence applied to raw buffers, not final adjusted carriers. Counts do not prove CPU/memory benefit. See the preserved external census and the correction's measurements for exact states/times/ELFs.

Internal correction authorized by the owner; ADR-0127 continuous flow, no public/default/phase/compatibility change. The pinned raw-shaping-cache nucleus supplies shared invariants; this owner authorizes only its following responsibility.

After bidi, select the existing ordinary or accepted-planned metrics handle for its private raw-sharing adapter. The None branch invokes the existing fresh path. Preserve eval/context/layout/bidi/shaping/reconciliation/export ordering, all errors and metadata; do not integrate prior eval-session or font-freshness experiments.

Keep regression tests first and preserve source/frame/glyph/position/font/fallback semantics. Directed missing-key and bypass-reuse mutants must fail; compare allocation traffic, peak and CPU in the real pipeline including retention/key costs, without an arbitrary target. Compile only in own /dev/shm and preserve all existing checkout changes. Outline's semantic divergence and prior identity debts remain explicit. The retained eval/font-session prototype is a lab measurement base, not integrated by this correction.


## Laboratory subtree evaluation connection — 2026-11-12

Measured source: pipeline.rs:295–344 evaluates and validates assets; paged.rs:765
starts root evaluation; pdf.rs:385–416 carries PDF options into paged compilation;
compile.rs:126/469 and watch.rs:65–85 establish session lifetime and dispatch.
These are mechanics, not language requirements. Source freeze in subtree-validation.
User explicitly waives approval for laboratory experiments; no integration.

Add a private optional-session companion to the existing full-feature introspector facade. Only the root eval adapter receives this session. Introspection, stabilization, layout, metrics, shaping and export run with current World and fresh compilation state. Compatibility facades pass None. Do not retain frames or move pipeline phases.


## Laboratory paragraph retention — 2026-11-12

Evidence first: subtree-layout-admission/receipt.json records102 preparations
and two refused complete plans per100-paragraph compilation at frozen subtree.
paragraph-validation records actual route geometry GREEN and cold-cache runtime
RED (22 vs11 calls). Prepared contains word/gap measurements and selectors,
without World, positions or callbacks. This experiment preserves all current
phase order and creates a bounded explicit product session. User waives gates
for laboratory work; no checkout integration.

The private session companion receives PdfCompileSession. Pass only evaluation
to the unchanged evaluation adapter. At the existing ordinary layout_metrics
construction, attach paragraphs using fresh FallbackFontMetrics. Current context
settlement/helper paths can remain cold; do not extend their certificates or
retain World/layout/introspection state. Existing raw store remains percompile.


## Laboratory direct-body and context product reuse — 2026-11-13

Evidence before decision: paragraph-layout-boundaries/receipt.json, bound to
freeze-paragraph, records60 refused try_parts calls but78 try_layout calls
(72 accepted) on each outline compilation. The older statement that all outline
plans refuse applied only to try_parts and is superseded by this wider census.
paragraph-native-phases/summary.json records29.509ms elapsed inside context
preparation, not exclusive CPU or a promised saving. paragraph_plan.rs:89-136
prepares/selects the direct body without using the product session;
context_stabilization.rs:923 constructs compilation-local metrics without it.
This is execution mechanics, not a new language rule. No gain is claimed until
paired release measurement. User permits laboratory work; no integration.

Pass only the optional pure paragraph session to the private paginated
context-preparation companion at the existing call site. Keep root evaluation,
phase order, context discovery/stabilization, selected-layout handoff and every
other helper unchanged. This supersedes only the previous experimental allowance
that context metrics remain disconnected. No retained final document/snapshot.


## Laboratory snapshot field extraction — 2026-11-13

Evidence before adaptation: snapshot-share-validation/adjusted-1-all-targets.log
reports E0507 atpaged.rs:600,725 because fields now reside behind the
Arc/COW payload owned by entities/introspector. These are fresh scratch results,
not a retained semantic generation. Extract the prefix observation map and realized ContextBlock-location map from their fresh unique scratch snapshots with mem::take. Make the scratch topology binding mutable only for that extraction. Preserve the same observed map, focal comparison/rerun and contextual snapshot injection; no extra clone, evaluation or changed traversal.
L0 precedes this mechanical change. Existing language controls/full gates apply;
user authorizes laboratory work, no integration.


## Laboratory complete shaping products — 2026-11-13

### Evidence before decision

The frozen fraction-demand release (f34f54de100fc63587f7611d042dc3d836b484f95283a2f5fc029c8afadce65e)
and fraction-demand-work-shift/causal-summary.json record 112 backend executions
per100-paragraph compilation and64 peroutline, unchanged by removing2004/2172
width queries. All three cycle kinds repeat this work. structural-summary.json
attributes18.39% inclusive samples to emitted shaping and5.31% to raw shaping
in100 paragraphs; these groups overlap and are not a speedup prediction.
font_metrics.rs:100-106 shares only raw buffers within a compilation;
shaper/traversal.rs:30-49 reconstructs faces/selection/output caches;
shaping.rs:262-346 rebuilds measured widths and:363-679 rebuilds emitted products.
world/fonts.rs:21,35-39,291 retains one immutable prefix owner across a font
session and revokes before full-catalog exposure. Current PdfCompileSession
at pipeline/pdf.rs:1133 has no shaping products. These are execution mechanics,
not a change to language semantics. The user explicitly authorizes laboratory
implementation/testing without further stops, including experimental APIs;
integration remains prohibited. No performance benefit is presumed.

### Exclusive responsibility

At existing ordinary metrics construction attach the explicit shaping session when present. At the existing final shaping point pass the same session, including the fresh-face fallback branch; never move shaping before bidi or layout, nor cache context execution. Selected context metric attachment must be handled by its own owner before any such extension is claimed.

The two runtime pipeline REDs require connecting both ordinary metrics and the contextual companion. Pass (paragraphs, shaping) only through its retained argument; keep all phases/selection unchanged. This supersedes the earlier temporary allowance for cold selected-context metrics.


## Laboratory: complete independent Block candidate — 2026-11-15

### Measured inputs before classification

pipeline-returns-2026-11-15/detail-build.json and paired-detail-block/record.json
identify source/ELF/UTC/input bytes; detail-controls-state.json identifies HEAD
and dirty tree. Tail executes two callback-layout bodies (3206658/2959777 ns,
instrumented inclusive, not nominal), 202 line commits, with ordinary snapshot
same=true and five pages before/after. Paragraph control has one layout. Equality
of page count is not the proof: paged.rs:91–256 rejects Block without examining
its body. entities/elements/block.rs:31–46 has only body as a Content dependency;
width/height/inset/outset/radius/clip/fill/stroke/spacing/above/below/sticky are
already materialized physical data. layout/block.rs:119–405 reads those fields,
current physical region/style and delegates BlockBody through its source plan;
there is no independent PageStore/query/counter/callback field in BlockElem.
Body effects and dependencies must still be recursively certified. No SourcePlan
or checkpoint is retained, and child rendering remains unchanged.

### Narrow extension of the existing positive proof

Only in this isolated laboratory variant, initial_layout_independent additionally
accepts Content::Block iff its complete body passes the same recursive proof.
This supersedes only the prior Block exclusion. Unknown descendants, contexts,
math/callbacks, references, outlines, counters, page-dependent styles and other
unlisted variants keep fail-closed behavior. Check BOTH original contextual and
final realized content, and require every final page numbering absent as before.
Do not weaken any PageStore/position/read comparison, callback budget, effects,
style predicate, PageRun policy or selected-path validation.

Reuse the already complete initial document only via the existing ordinary branch.
Still construct/realize PageStore, publish its warnings, inject positions/pages,
bind logical numbers, preserve formatting metadata and run bidi/shaping/export.
No new phase, default, public API or persistent physical result. Same width,
style, source plan and World are those of this compilation; a new compilation
still performs its first complete layout. No visual/hash/Arc equality certificate.

Tests: module RED on actual first-layout call count for multipage, nested/styled
and fixed/clip Blocks while text and logical page binding remain intact; numbered,
contextual and unproved math bodies must retain relayout. GREEN plus existing
pipeline tests and external warm/cold original-reference controls; block geometry,
styles/fonts/callback/numbering/marginal controls, exact diagnostics and PDF/SVG.
Paired nominal reference vs Unicode-only vs combined vs pinned vanilla measures
actual benefit; observed 2.96ms is not a promised saving. No integration.

Test correction before implementation: INITIAL_LAYOUT_CALLS observes only the ordinary callback route (paged.rs:1261), not the approved contextual physical protocol. Existing numbered_table_keeps_second_layout explicitly expects zero. The initial negative test assumption was invalid. Negative guards now check exclusion by the actual proof and visible numbering; external detail probes retain physical pass evidence. First-attempt RED is preserved; no implementation change has occurred.

## Laboratory: Pad complete-document reuse certificate — 2026-11-15

Measurement precedes decision: reuse-certificates-2026-11-15/preflight.json, Pad/detail, binds UTC and exact source. initial.json binds HEAD/dirty files and diagnostic ELF. Plain multipage Pad executes two callback layouts (4052744 and1015561 ns inclusive instrumented, not nominal saving), ordinary snapshot same=true, nine pages. entities/elements/pad.rs:23-26 has only body and physical Sides<Option<Length>>. compiler/layout/pad.rs:24-81 resolves sides against current font size, measures body bbox, dispatches PlanEdge::PadBody and restores geometry. Body measurement is intentional and is NOT removed; no independent query/store/closure field exists outside body. Read owning Pad L0 and preserve its source-edge and parent-bbox semantics.

Internal laboratory-only positive-proof refinement: additionally accept Content::Pad iff its full body is initial_layout_independent. Require BOTH contextual and realized content and absent page numbering exactly as existing guard. Unknown descendants, counters/state/context/math/outline/references and dynamic styles still reject. Reuse the existing complete initial document only within this compilation, after constructing snapshot, warning effects and positions/page injections; retain final binding, bidi, shaping and exports. New source/width/style/font compilation always composes anew; no persistent PagedDocument cache or key. No new API/default/phase or compatibility change.

RED before implementation: multipage Pad with asymmetric physical sides, em padding, nested Block/Pad must report two ordinary callbacks where one is required, while text/logical pages remain correct. Negative proof tests preserve rejection for context, state, math and numbering. GREEN full pipeline:: tests, permanent external counterexamples including known counter/state values and missing-font diagnostics; warm/cold reference controls, negative fallbacks, nominal paired baseline Block-only vs Pad refinement vs vanilla with no observers. No expansion of Align or unrelated variants and no unsound mutation flags. Keep first-pass body measurement intact.

## Laboratory: independent Align complete-document reuse — 2026-11-15

Evidence before classification: reuse-align-2026-11-15/measurement.json records exact source/UTC and original/previous/detail outputs; initial.json binds HEAD, dirty files and ELF SHA. Align40-paragraph document executes two callback layouts,5057441 and2321560ns inclusive instrumented, same ordinary snapshot,3pages. Geometry and extracted words equal in original and previous; empty-page extraction warning is preserved and not repaired by this proposal. No nominal saving inferred.

entities/elements/align.rs:23-26 has only alignment:Align2D and body:Content. compiler/layout/align.rs:10-15 delegates to placement.rs:237-551; it reads current physical regions/style/metrics, dispatches PlanEdge::AlignBody, preserves incoming-line and subframe placement and carries pending coordinate fixups to finish/new_page. Align2D introduces no separate introspection/closure/PageStore dependency; children retain all their own dependencies. Source/L0 equality across product/previous/candidate is checked in source-audit.json. Existing coordinate/physical-source contracts remain authoritative; do not copy or alter formulas or settle Pending early.

Only this laboratory proof is extended: Content::Align is independent iff its complete body passes initial_layout_independent. Supersede prior unlisted-Align exclusion and the Pad amendment's prohibition on expanding Align ONLY for this measured scope. Require BOTH original contextual and realized trees, absent final numbering and the existing fail-closed style/body rules. Every unlisted descendant (including Place, Box, context, references/outline, counter/state, math/callbacks) still rejects. Do not change positional fixups, source plan, ordinary snapshot construction, effects, logical binding, bidi/shaping/export or stability algorithms. Reuse only the complete initial document in the same compilation; every new compilation still performs its initial layout. No persisted document/cache key, no new public API/default/phase/compatibility, no product integration.

RED before implementation: finite centered multiparagraph Align, nested Align/Pad/Block, bottom/horizon, Auto page dimensions observe two ordinary compositions where one required; text and logical numbering intact. Negative tests for context, state/query, math, explicit numbering and unproved Place retain proof rejection. GREEN pipeline:: suite plus all nine permanent counterexamples, warm/cold comparisons including finite/Auto axes, nested styles/regions, contextual callbacks/numbering and references; PDF/SVG/raster guards. Measure original vs previous(Block+Pad) vs cumulative(Block+Pad+Align) vs pinned vanilla, two balanced paired campaigns, with uninstrumented executables. Record adverse controls and distinguish incremental from accumulated benefit. No uniform performance claim or full-pipeline equivalence.

## Laboratory: legacy complete-candidate certificate — 2026-11-15

Evidence before decision: reuse-proof-cost-2026-11-15/profiles.json records Align-context three ordinary physical compositions and about13.6ms inclusive layout vs5.47us proof in tail; final introspective snapshot changes then stabilizes (reuse-align/fallback-route-probe.json). paged.rs:943-1098 always rebuilds after the newly realized snapshot unless initial syntax-only proof passes. context_stabilization.rs:2237 creates a new legacy Session and discards its actual executions immediately after expansion; Execution already contains original ctx/read transcript/input/sink/Location and with_replay_engine validates in isolated sinks. World source contract allows external reads and SystemWorld.today reads a live date; context-read validation alone is insufficient. ContextBlock PartialEq only compares id and MUST NOT be used as a cache key. All evidence has HEAD/dirty/ELF/UTC in prior receipts and this initial.json.

Internal-only classification ADR-0127: add private per-compilation proof/World forwarding records; no exported signature/trait/entity changes, no eval/layout movement or target/default/compatibility changes. Keep existing expand_legacy facade and behavior for all other callers. Only its new private companion may retain a successful Session to certify the document composed from that exact expansion. Do not change Session selection, actual closure evaluation order, budgets, occurrence arena or existing warning/error publication. New World guard is used only in the companion, delegates ALL World methods unchanged, permits the established immutable library/book/main/font/candidate resources and rejects any observed other World operation. Source accesses require the immutable-source authority defined below; file/path/include/package/inputs/today/plugin accesses poison certificate even on error. No guarded value caching. Captured plugin calls remain governed by existing PluginFunc.call memoization; new plugin-host access rejects.

Paged owner generalizes the existing positive predicate internally to a borrowed ContextBlock authorization callback, preserving the old strict facade with callback always false. Only the retained session may authorize exact active producer Arc membership (a provenance check, never standalone validity/deduplication); ambiguous topology and unknown carriers reject. Verify original session content, substituted output and actual final realized content with this predicate. Every active producer must have successful retained execution, no poisoned World guard, matching candidate Location, reflexive actual-read validation and validation against candidate, using existing L1 facilities. Require full conservative pagination_stability(input,candidate) coverage/equality for each execution. Validation errors return false and publish nothing; ordinary path then proceeds unchanged.

No certificate for the initial untracked prepare result. After the ordinary snapshot/PageStore/numbering/position injection, require final pages have no numbering and a prior certificate for the CURRENT complete doc. If valid, preserve doc, intr/format data and existing finalization; skip only the next legacy expansion/introspection/outline/layout cycle. If not valid, drop old certificate BEFORE new expansion. The new companion returns content + opaque private certificate; attach it only after successful physical composition from that content and retain at most one Session. Never use snapshot_same alone, compare newly executed closures or remove Pending loops. Context/state/query outputs with effectful/unknown carriers reject. HTML/Bundle and selected physical discovery/replay remain untouched.

Tests before code: actual final page output correct and ordinary layouts3→2 for plain and Align/Page-final contexts, invalid first snapshot forces recomposition, known initial0 must never survive; negative World date/read bodies retain3 layouts and diagnostics, unknown math/numbering/Place/effect bodies retain fallback, nested/position/location changes do not borrow stale candidates. Full pipeline:: suite, nine permanent regressions, warm/cold edits and source/World invalidation, PDF/SVG/diagnostic guards and paired baseline/candidate/original/A-A nominal with memory. Reject candidate if no gain, output/effect divergence or disproportionate resource cost. No integration/commits.

## Laboratory immutable source authority — 2026-11-15

Evidence before decision: the rejected legacy-certificate report and World
forwarding audit bind its ELF to 16/19 forwarded methods. markup_stream.rs:121-134
calls World.source to recover the current content origin. Source is immutable
Arc<SourceInner> (source.rs:28-50); reparse produces another immutable revision.
Source Eq only compares id/hash and is not revision authority. SystemWorld
source slots cache the first result in OnceLock (world/cache.rs:37-96), allocate
monotonic ids and never replace a successfully published slot (cache.rs:98-174).
Generic World has no source stability promise. These are mechanics observations;
language acceptance remains text/content morphology/geometry/diagnostics.

Authorization/classification: user explicitly requested correcting World and
testing immutable source certification in the laboratory, without integration
or commits. The Source owner records the earlier user instruction "Para o
laboratório não precisa autorização". This ratifies the following laboratory
capability addition only; no productive contract or phase/default is changed.

Exact laboratory public trait addition:
    fn immutable_source(&self, id: FileId) -> Option<Source> { None }

Some(source) is an explicit authority promise: source has the requested id,
is an owned immutable revision, and every subsequent source(id) query on THIS
World during its lifetime returns that same successful revision without new
observable source-query effects. No guarantee is inferred from bytes, ids,
hashes, root addresses or ordinary source() success. None is conservative and
permits arbitrary World implementations. Never override another World's source
answer or cache file/date/path/plugin results. SystemWorld may return its
successful OnceLock-backed world_source(id); the pipeline asks only after a
successful actual source(id) query. New Worlds have independent source slots.

LegacyWorldGuard must explicitly forward ALL trait methods, including font
epoch/select/info and immutable_source, preserving owned/reference identity and
overrides. The source method first delegates exactly once and returns its exact
result. Permit only the compilation input id, the exact immutable SourceInner
revision kept alive by the borrowed input, and a matching explicit authority
from the SAME World. Root-address comparison is provenance under that authority,
never a standalone semantic key. Different revisions even with same bytes/id,
other ids/imports, errors, missing authority, date/file/path/plugin/inputs reject.
Record whether source was observed in each retained execution; recheck its
authority before candidate reuse. Replay uses the identical guard. No extra
ordinary source() calls merely to validate, and no queries in unguarded routes.

All existing complete-document pagination/Location/read/structural proofs,
first-layout/snapshot requirements, budgets, warning/error policies and local
certificate lifetimes remain. No Unicode cache, cross-compilation context
result cache or selected/HTML/Bundle changes. Two manifested lab builds may
toggle a private constant CERTIFY_IMMUTABLE_SOURCE: false is forwarding-only,
true enables this capability. This is no product mode/flag/default.

RED before implementation: SystemWorld markup totals in plain/Align/Pad retain
correct text but must drop physical compositions from3 to2; sentinel World
font overrides must survive wrapping. Negatives: generic World without authority,
fresh equal-text revision, mutable external reads/date and imports retain fallback.
Preserve existing tests; run full pipeline tests and lint, permanent regressions,
warm/cold/export/edit controls and two balanced paired nominal campaigns versus
prior Block/Pad/Align and forwarding-only, with an identical-ELF A/A alias.
Record CPU/RSS/HWM and compare all states, including references and context scale.

The exact capability in the latest section supersedes the prior prototype's blanket source rejection and no-trait-change statements, solely in this laboratory tree. Product integration remains excluded.


## Preserved project scope and prior obligations

## Contract categories — review 2026-11-13

This review changes documentation scope, not executable behavior. Existing public
signatures, phases, cache lifetimes and fallback behavior remain the current
implementation contract. Restrictions attached to a named correction delimit
that correction; they are not universal prohibitions on a separately specified
future design. Reuse requires complete relevant dependency validation, immutable
or independently owned results, and preservation of geometry, causal identity,
effects, warnings and errors. A storage pointer, equal output, or matching page
count is not sufficient. No laboratory candidate is integrated by this review.

The no-retention/no-frame-cache clauses belong to the existing borrowed metrics
handle, callback stores and bounded corrections described here. They continue to
forbid prolonging those objects without a new validity contract. They do not prove
that all immutable layout products must be rebuilt forever. A future cross-edit
product must state its inputs, dependency reads, invalidation, effect replay,
retention bound and fallback; it cannot merely extend this World borrow. Keeping
candidate effects isolated does not inherently require deep-copying immutable data.


## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.
