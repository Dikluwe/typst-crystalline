# L0 — Layout engine coordination
Hash do Código: 33ca17c3




**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/mod.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:5c68aee638a285b125399ae9198883a90ae27a0c3a89f68e0765b86fa84b8197
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:b556a9f22e29d478ae1afac3e31427aeea3fa58f44a4aa04e46c86b90fb5779c
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:4a93d5c55bf975b506b6f02ae484a91fdf7449716f9248dea2fcaa6b7b0738b6
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:029c2929c9e294efceb796a5358a6f425f02ae91c519314080f413374f4f4403
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:5c733e4756935ef5e4841991852a5f60f0f00e2b76c988c6af340b52cc0ce977
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/footnote-entry-settings.toml sha256:69e358bd5dd06f821f96981bef9ab5c6b6d35b3bc592d2ee41d9f7ecf767698a
- 00_nucleo/prompts/_nuclei/layout/horizontal-space-paragraph.toml sha256:bbcb7c79c93e3d2da4580dfbc5586174a9698f02e3d3579ea886a1ff2608ed4b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2
- 00_nucleo/prompts/_nuclei/layout/page-number-baseline.toml sha256:0c492a09d2660de765df65c22e703cc8c3e63cc663ee4d9ac2dd612afc80dfa3
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:ec315f38498aa37ead497fdcb72cdd7e401cd00752baf5a535c2b5fae69374ed
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:68203377765579bc29327c7d8fded6f4f831d11682b6b7c9abbf2dbf65b28bad
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/declarative-alignment.toml sha256:011aad6ca3199e192d8eb8eecd55d718b4d7ce7097f7987f982cb7a5f12f9a10


## Evidence before specification

`01_core/src/compiler/layout/mod.rs:240,711,1129,1219,1418,1435,1455,1923,2218,2446` was inspected. This establishes behavior/ownership, not upstream intent or new bilateral parity. Text/semantics/syntax/morphology/geometry/errors govern acceptance; state/dispatch/buffers are mechanics. Historical 9f2b6e871 evidence does not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Complete chronology remains in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/mod.md`.

The separation/source evidence in sanitization-layout-history.md, correcao-falhas-2026-09-28/, regional-demand/terminal-note controls, paragraph-composition/preparation/hyphenation/widow-orphan measurements, tracking/baseline separation and lexical tracking receipts precede their private module/state boundaries. Realized-origin backtrace/RED supplies evidence for delegation and one-time explicit identity reservation. Direction, ratified logical bidi and cell-text refutators bound context storage without expanding certification.

SmartQuote before/fields-before/partial-and-html/proposal-controls receipts in `lab/results/construction/diagnostics/smartquote-2026-10-06/` precede human ratification 2026-10-06 of public double:Option<bool>/enabled:Option<bool>, pure context helper and glyph selection eval→target render. This supersedes only quote-family conflicts. Paged/HTML remain separate semantic targets. Source transport D08B3-C2 is ratified and adds no geometry/replay/identity event.

`lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz` preserves spans 556/698/875, uses, source/L0, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree, UTC/status/stat. box-composicao-2026-10-13/before.json.gz exposes subtracting external page margins in local regions; inline-extents/installed-ascent, posicoes-compostas and links-captura receipts precede their private storage. Place-local/committed-flow/Block-region, block-auto-width/alignment and transform relative-region/prefix-nesting evidence precede the later slots and invalidation rules. Source/HEAD/working-tree/UTC/stat/argv/binary provenance is retained in each causal diagnostic, not inferred from a final commit.

The arity inventory measures C017 at 1911: eight arguments, Clippy threshold 7. Its public seeded callback/LocatorSeed façade is contractually fixed. Later nominal-expectation receipt reconfirms only this justified direct expectation, with no conditional correction annotations.

Bibliography structure/plan and admitted-prefix/continuation evidence distinguish a private immutable plan from public cache/Content/Style and a local layout capability from global state. `lab/results/construction/diagnostics/bibliography-structure-2026-10-18/{pagination-block,block-freeze}.json.gz` preserves state/UTC/HEAD/sources/binaries/argv/boxes for candidate `c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea` against oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3` (binary `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). It regains two IEEE200×90pt pages but loses continuation inset; corrected 70pt fixture reveals prefix orphaned before body admission. `region_progress.rs:75–76` resets margin and `paragraph_pagination.rs:69,87–105` can legitimately group three lines for widow/orphan. Prefix must accompany actual first line, not an invalid FIRSTWORD-first-page requirement or forced greedy/disabled protection. First prefix paint remains before body paint after admission, never changed eval/show realization order. Orphan prefix/lost inset/double emission/new effect/changed strategy/Block-column-subframe regression refutes correction.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and preserved coordination

Native dispatch and state storage are specified here. Checkpoint capture/restore and terminal document composition belong to `00_nucleo/prompts/core/compiler/layout/checkpoint.md` and `00_nucleo/prompts/core/compiler/layout/finish.md`.

This is the exclusive owner of `01_core/src/compiler/layout/mod.rs`: Layouter construction and state, causal dispatch, public façade entrypoints, attempt coordination, and compatibility helpers actually defined here. Checkpoint and terminal composition behavior is delegated to the child owners named above. Child owners define their own layout behavior; importing or calling them does not transfer ownership to this prompt.

### State and injected dependencies

Keep the existing Layouter API, generic FontMetrics/ImageSizer injection, tracked introspector and runtime. No filesystem, network, clock, environment access or Func execution is introduced. The active region holds cursors, line start, emitted items, pending line, dimensions and horizontal fractions. PageConfig and region dimensions stay synchronized when a configuration is installed on an empty page.

Preserve the chain/cached TextStyle, locator seed/current Location, document language, page/column and cell contexts, parent bbox, read-only runtime, figure progress, enum/list grouping, smartquote and smallcaps state. Preserve existing deferred buffers for floats, cell tails, footnote bodies, attachments, page/line/equation numbering, alignment paths, repeat and fractional natural widths, decoration collectors, warnings and errors. The hub owns this storage and its lifetime; it does not re-specify child formulas.

Fractional Block allocation remains private optional state: absent allocation is not a zero allocation. Preserve the preflight re-entry guard, terminal fractional-tail flag, one-shot validated-prefix line arm, assumed ascent and inline ascent/descent accumulators. Child policies may set/consume these through the existing descendant interface; no public entity or alternative state machine is created.

### Causal dispatcher and transaction boundary

`layout_content_once` performs the single canonical `is_locatable` gate before its exhaustive Content match. An eligible occurrence advances Locator exactly once, makes current_location available to descendants and records its current physical page and point. Structural recursion and visual children do not allocate extra Locations by heuristic. Measurement does not add emission events.

Native dispatch remains static and exhaustive with thin descendant delegation in the render layer, as required by ADR-0109 form B. Keep the closed Content enum and the current explicit transparent/no-op branches for already-consumed metadata/state/counter updates, structural entry carriers, alignment points and HTML-only elements. Retain the existing Dynamic extension adapter without turning native dispatch into dyn/vtable/string/PropMap dispatch or moving render into entities.

When a stabilized float marker leaves a checkpoint pending, `layout_content` opens the transaction before the next occurrence and routes top-level dispatch to `00_nucleo/prompts/core/compiler/layout/flow_suffix.md`; nested dispatch remains part of the same atomic unit. The hub provides capture/restore, while `00_nucleo/prompts/core/compiler/layout/flow_suffix.md` owns acceptance/rejection/replay decisions and `00_nucleo/prompts/core/compiler/layout/cursor.md` owns regional progress.

### Checkpoint integrity

A FlowSuffixCheckpoint records the causal state and append-only tail boundaries of both active/cell regions, completed pages, collectors, floats, attachments, numbering/alignment/repeat buffers, warnings/errors, columns, and all associated geometry/style/runtime/locator state actually captured by this consumer. RegionTailCheckpoint restores cursor/line-start/dimensions and truncates emitted items, pending line and fraction boundaries together.

Rollback restores the complete speculative suffix, including already-emitted Place items and causal counters/positions, not merely current_line. It reconstructs Locator from the captured seed, restores callback-related numbering state and one-shot arms, truncates buffered candidates/fixups, and clears transaction/replay flags. Committed prefix floats and pre-boundary items remain outside rollback. Missing structural storage where a captured region/collector should still exist is a debug invariant violation, not invented replacement state. Do not copy the checkpoint into children, inspect future Content variants or apply generic translation outside rejected replay.

### Attempt coordination and public entrypoints

Preserve the existing public layout, introspector/metrics, callback-aware, locator-seeded, intrinsic HTML-frame and real-measurement façade paths and signatures. The metrics entrypoint returns SourceResult and diagnoses unresolved math callbacks rather than returning a placeholder document; the legacy FixedMetrics convenience entrypoint retains its current expect behavior.

The convergence child installs fresh math and par.line pass state for each candidate Layouter. Each attempt restores the same supplied LocatorSeed and uses the tracked introspector, injected metrics/sizer and current known label/figure/table/page stores. Carry forward extracted page maps until they stabilize within the existing retry limit of five attempts. The source currently runs this convergence comparison regardless of the computed has_outline flag: do not restore an obsolete “no outline means exactly one attempt” promise.

Return Complete only with a stable candidate and successfully consumed math/par.line transcripts; otherwise return Pending requests without exposing the tentative document. A rejected candidate's transcript is discarded, not merged into its successor. Exhausting the retry bound returns the existing empty Pending request sets, not a completed candidate. The mathematical callback protocol remains owned by its math prompt; the par.line protocol belongs to `00_nucleo/prompts/core/compiler/layout/par_line_callbacks.md`. No eval ↔ layout phase change or layout-time Func execution is introduced.

The convergence child discovers the first bibliography's supplied style/locale and constructs a pure CSL cache only for an explicitly named or already-resolved style. Retain cache-conversion diagnostics in each candidate and the local fallback when no style is requested. `00_nucleo/prompts/core/compiler/layout/bib_csl.md` owns cache interpretation; `00_nucleo/prompts/core/compiler/layout/bibliography.md`/`00_nucleo/prompts/core/compiler/layout/cite.md` own rendering. The BibEntry compatibility body formatter belongs to `00_nucleo/prompts/core/compiler/layout/bibliography.md`, preserving conditional field order and organization-as-publisher fallback.

### Terminal composition

`finish` closes the final line through `00_nucleo/prompts/core/compiler/layout/line_commit.md`, preserving its causal last baseline/bottom. Request finite auto dimensions from `00_nucleo/prompts/core/compiler/layout/page_geometry.md` before terminal float/footnote emission. Drain pending floats and footnote bodies through Cursor hooks, retaining the bounded extra-region loop for remaining note bodies.

Create the final Page under the existing material/empty-page-run conditions. Apply descendant line-number, equation and horizontal/vertical alignment fixups in their current order, delegate explicit running layers and canvas construction, preserve fill/bleed/supplement, and initialize Page.logical_number to the physical index. Numbering in an automatic margin uses the injected known logical number when available, otherwise the physical index. Function numbering consumes already-realized visible Content; it is never executed here.

Preserve the terminal-only coordination for compound page-number patterns: record the page index, chosen current number and marginal geometry, then format with the final known logical total or physical page total fallback after all pages exist. Intermediate assembly belongs to `00_nucleo/prompts/core/compiler/layout/page_commit.md`; the hub does not absorb its consumer or duplicate a general page-transition policy.

Transfer pages, document language, attachments, label pages/positions, sealed occurrence positions, figure/table page vectors, warnings and errors into PagedDocument. Do not mutate the injected introspector or fabricate logical overlays belonging to the pipeline.

### Delegation boundaries

Line closure/allocation/physical line numbering: `00_nucleo/prompts/core/compiler/layout/line_commit.md`. Baseline/wrap/regional transition and marker hooks: `00_nucleo/prompts/core/compiler/layout/cursor.md`. Sequence preflight/grouping/spacing: `00_nucleo/prompts/core/compiler/layout/sequence.md`. Isolated sublayout: `00_nucleo/prompts/core/compiler/layout/sub_frame.md`; constrained/intrinsic measurement: `00_nucleo/prompts/core/compiler/layout/measurement.md`. Geometry/fixups: `00_nucleo/prompts/core/compiler/layout/placement.md`, `00_nucleo/prompts/core/compiler/layout/equation.md`, `00_nucleo/prompts/core/compiler/layout/page_geometry.md`; page layers: `00_nucleo/prompts/core/compiler/layout/page_running.md`, `00_nucleo/prompts/core/compiler/layout/page_canvas.md`. Child element behavior is specified by its own exclusive prompt, including `00_nucleo/prompts/core/compiler/layout/style_scope.md`, `00_nucleo/prompts/core/compiler/layout/space.md`, `00_nucleo/prompts/core/compiler/layout/parbreak.md`, `00_nucleo/prompts/core/compiler/layout/align.md`, `00_nucleo/prompts/core/compiler/layout/context_block.md`, `00_nucleo/prompts/core/compiler/layout/state_display.md`, `00_nucleo/prompts/core/compiler/layout/counter_display.md`, `00_nucleo/prompts/core/compiler/layout/counter_display_callback.md`, `00_nucleo/prompts/core/compiler/layout/outline_entry.md`. Column behavior remains in `00_nucleo/prompts/core/compiler/layout/columns.md`.

The coordinate and math-callback nuclei pinned above remain the shared invariant authority; child calls are boundary references, not additional consumers or duplicated normative ownership.

## Private state, dispatch and boundary refinements

Register private sibling descendants text_measure, text_tracking, text_baseline, break_selection, word_layout, region_progress, paragraph, paragraph planning units and paragraph_hyphen with their own exclusive canonical owners. Hub contains neither their formulas nor dynamic dispatch. `paragraph_select_bounded` and `paragraph_select_forward` are cfg(test)-only experimental descendants; production planner/default is unchanged. Convergence exclusively owns candidate preparation/page-map comparison/Complete-Pending; fallback bibliography formatting exclusively belongs to bibliography. Above descriptions of called behavior are boundary criteria, not duplicated ownership.

Space/SpaceFromNewline delegate to Space with Markup/Newline origin respectively; keep thin exhaustive native match. Content::Par delegates paragraph. Source delegates body transparently without new sentinel/Location/geometry/replay and preserves styles. Located uses explicit Location without reallocating body; Realized delegates static realized::layout to register consumed-origin anchors at visual start and emit only visual. Visual occurrences register themselves. Pending carrier projects body and must be resolved before ordinary layout. Keep location allocation separate from body dispatch, reserve all explicit body Locations once per Layouter before generating ordinary IDs with the same pure service used by introspection; this reservation is not a temporal index. Independent sublayout instances retain their own flag/seed. Realized completeness/textual block-equation boundary belongs to `00_nucleo/prompts/core/compiler/layout/realized.md`.

Private terminal_paragraph_line starts false and is captured/restored. Finish captures it before an empty flush and uses already-closed baseline as terminal geometry; a last Literal-only separator line is causal too. Empty initial document produces one page from resolved PageConfig, including dimensions/margins/layers, never exporter defaults; preserve suppression of trailing empty page-run boundaries and explicit intentional breaks. A hypothetical next baseline alone cannot create a terminal page. Preserve causal cursor during Auto geometry/float flush; if material final line closed and pending notes need fitting, temporarily use max(causal cursor,predicted terminal clearance), retaining that clearance across deferred body/next_region. Restore causal cursor only if physical page/column did not change; after transition keep its new cursor. Each required next_region immediately drains notes with existing bottom override so a fitting deferred body does not need another transition. Preserve defensive body-count bound/forced-overlarge rule and no stored field/new note model.

Maintain private paragraph_planning_suppressed=false, checkpointed; no planner algorithm in hub. text_tracking::State starts Default, cloned/restored as whole; no identity allocation/continuity formula here. Paragraph active/pending direction are Option<Dir>=None; resolver belongs to paragraph. Ratified logical cursor/input/current-span state is private and checkpointed, created only by certified owner; uncertified contexts remain None with pinned transforms. cell_bidi_allowed starts false and is lexical emission capability, not speculative flow mutation. explicit_line_alignment:Option<HAlign>=None is installed by placement, calculated by cursor and inherited distinctly from direction.

SmartQuoteContext replaces independent alternating flags, fresh per isolated flow, shared among markup/programmatic/Styled/Strong/Emph in the same flow. Observe logically emitted text/separators/inline objects causally, delegating pairs to language quotes and glyph emission to smartquote; private hooks may wrap dispatch but never duplicate resolver. Checkpoint clones/restores context so measurement/rollback does not consume confirmed context twice. Independent sublayout has own context; parent sees object, not inner text prefix. Preserve bidi/Locator/notes/columns/floats/styles/budgets/resources; equal-looking glyphs do not widen reuse guards. No entity-layout logic, I/O/globals/vtable, new show evaluator/Engine/mode/unsafe/watch/math phase. HTML resolves in its own semantic target.

Keep positional constructor f64 argument as _font_size; initial style derives solely from default StyleChain, never restore literal 12pt. bidi_cursor and pending_repeats remain private to layout descendants/checkpoint, no moved types or external API change.

Inside is_sub_frame, available_width=max(width-line_start_x-margin.right,0), available_height/page_bottom_limit use local height without exterior page margins. Finite local region wins over exterior Auto dimension; ordinary page formulas unchanged. Preserve local boundary/parent-restoration ownership rather than embedding Box policy.

Store private last_sub_frame_inline_extents=(0,0); line_assumed_ascent:Option<f64>=None with Some(0) valid. position_marker owns registration/capture: append-only destinations starts empty; both canonical locatable/Located paths retain observe/next/page/point and delegate only write/capture. link_capture owns destination/active-scope vectors, initially empty, no Location change.

Store pub(super) sub_frame_origin_y:f64=0 (recomposition origin only), flow_placement_end:Option<(f64,f64)>=None (physical end/prepared cursor), private block_place_region:Option<Rect>=None, flow_frame_extent:(f64,f64)=(0,0), last_sub_frame_flow_width:f64=0, block_auto_width=false and relative_region_size=None at root. New PageConfig clears flow witness/Block region/Auto-width state. Keep last_sub_frame_width legacy. relative_region_size is actual percentage context, distinct from pagination sentinel. Private Place-auto-Y decoration ranges are associated with fixup index, stored here/checkpointed, isolated by Block/subframe and cleared by region transitions; placement registers occurrence ranges and Block applies same Y delta before ending scope. Later private-state authorizations supersede old extraction-only no-new-state restrictions only within these contracts.

Store text_segment_chain:Option<StyleChain>=None and geometric_text_lines=false; owners implement lexical/multiline policy. Store Option<Arc<bib_csl::plan::BibliographyPlan>>=None without altering public CSL cache or treating plan as Content/Style. pending_bib_prefix:Option<bibliography::PendingPrefix> and block_inline_region:Option<block::InlineRegion> start None and are local capabilities, no global/domain/custom Style/vtable. Register paragraph_indent and private State initialized Paragraph, pending_bib_escape:Option<Spec>=None; dispatcher only delegates before_content/after_content, all classification/application belongs to its owner.

Preserve C017 signature/order/semantics/body/calls/cfg/unrelated attributes and direct function-only `#[expect(clippy::too_many_arguments, reason="C017: Fachada pública seeded do layout; delega ao coordenador de convergência os oito inputs completos")]`. Do not annotate module/impl/crate, turn expect into allow, use a builder just to hide arity, change threshold, or annotate entries classified correction (including conditional C047/C066). Nominal expectations must become warnings if no longer exercised. No public-boundary repair is authorized by this mechanical cleanup.

## Acceptance and explicit limits

Preserve façade/public signatures/defaults/compatibility and static exhaustive form-B dispatch, causal identity/events, prefix/suffix atomic replay, exact checkpoint slots/lifetimes, terminal document text/geometry/styles/pages/errors, intentional empties/notes/float clearance, and all sibling-owner contracts. Require meaningful RED→GREEN for coordinates/neutral private carriers/no phantom height/identity/order/clones/rollback, configured empty page, finite/infinite roots, fitting terminal text/suffixes/notes, region transitions, columns/cells, lexical direction/tracking/bidi/quote state, local bounds and nested scope restoration. Use actual source→PDF/SVG/HTML/context/query observables, not flags/build alone; freeze provenance before numeric decisions. Run full workspace/core suites, all targets/build/Rustdoc/fmt/Clippy strict including unfulfilled expectations, strict/reverse lineage and differential inventories with valid V15/V26 before restricted resealing.

Place body origin/committed-flow boundary/Block local region are bounded separate corrections, not universal Place certification. Block fragmentation/late inline Box-Transform settlement, uncertified producers and separate rendering/tiling resource gates remain open under their canonical owners. Tiling public transport has an approval receipt but requires integrated render acceptance; no stale pending approval assertion or carrier-only production claim. Bibliography/hanging and bidi/cell scopes retain their measured certificates and fallback restrictions. This consolidation authorizes no new public API/default/phase/dependency or unresolved proposal.

Prefix controls require RED→GREEN same actual prefix/body page, continuation inset and OUTSIDE, Simple/Optimized and prior controls. Measure PDF90/150/250, RTL, prefix-only/formatted/color/regressions; build/suites/doctests/Clippy/Rustdoc/fmt/strict-reverse lineage/differential Tekt under MemoryMax12GiB/High8GiB/swap0/jobs1, without publication. Generic hanging/overrides and multiple bibliographies retain their own open scopes.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

## V16 closed-domain evidence and correction

Measured in the uncommitted working tree at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:02:56.055664+00:00`; exact changed-file diffstat and source/L0 SHA-256 are in `lab/results/v16-corrections-2026-10-19/batch-d/agents/layout/audit-before.json`. `mod.rs:1219–1263` separates Source/Located/Realized/ShowRecipe before the canonical location gate. `mod.rs:1265–1275` observes Text and the two space forms before exhaustive body dispatch. `mod.rs:1179–1196` publishes the ordinary occurrence identity; direct Located instead observes its explicit ID and dispatches its body without a new ID.

This is observed local source behavior, not a new bilateral result or inferred upstream intent. Identity, text, quote choice, geometry and supplement semantics are language obligations; the match representation is mechanics. A caller admitting a supposedly excluded payload, duplicate event, changed text/geometry/request or changed diagnostic refutes preservation.

At layout_content_once keep four distinct transport behaviors: Source recurses transparently; Located reserves/observes its explicit identity and invokes body dispatch; Realized delegates causal-original/visual handling; ShowRecipe delegates its body. Explicitly enumerate all other present Content variants as forwarding to the existing locator gate and body dispatch, not as ignored content. At the quote observer, Text observes its string and Space/SpaceFromNewline observe a single logical space. Explicitly classify every other present Content variant as having no additional event at this observer: wrappers let descendants speak, neutral metadata stays neutral, and object/quote/break policies remain in their existing delegates. Neither match may hide future Content variants behind an open remainder, move the decision to another helper or replace it with an if-let. Preserve location reservation, one causal occurrence, quote event ordering, static form-B delegation and rollback. New Content variants require independent identity and quote-observation decisions.

ADR-0127 classification: continuous-flow internal nominal coverage with no public signature, default, phase or compatibility change. Add behavior-preservation controls before implementation and require baseline/final controls plus disposable future-variant compile-negative evidence at the exact selected decision. Current runtime controls may already pass; the structural RED is failure to reject a future variant at that decision. No new suppression or open catchall is authorized.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Document and preserve figure_progress exclusively as unprepared synthetic Figure state. Prepared productive Figures use validated typed snapshot at current_location. Preserve current Source span for missing-snapshot diagnostics and child Location transport. No new productive progress map, public API or callback.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


Nearest Source span must survive the public layout_content entrypoint's source_body normalization, with scoped restoration after dispatch. Source(Located(prepared Figure)) lacking a verified snapshot reports that Source span, not detached. Preserve original transaction-content normalization and all marker selection.


### Returned styled occurrence root transport

`figure-kind-implementation/integration-tests-2/tests.log` records real RED: Located(Styled(prepared Figure)) changes Location 71 to 72. Direct Located(Figure) bypasses the ordinary gate, but Styled recursion reenters it. Add private scoped transported_root_location:Option<Location>, initially None. Located installs it only while descending transparent Source/Styled/ShowRecipe root carriers, restoring the previous slot afterwards. The first ordinary locatable root consumes the transported identity without allocation or duplicate position publication. An ordinary nonlocatable non-Styled container clears it before children; a nested Figure never borrows its parent's identity. Direct Located(Figure) needs no pending slot. Preserve standard style_scope behavior and all styles rather than copying its business logic. Checkpoint copies/restores this transient root context. Require exact-identity and nested-child regression, existing position/suffix tests unchanged.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Own the approved `layout_with_page_running` facade and private optional planned state on Layouter. Preserve every legacy public signature and legacy unplanned path. The new facade installs a PageRunningBody source authority and explicit root cursor, with no Content wrappers or changed language fields. Native dispatch remains exhaustive/form B. Planned source children descend through their declared typed field/index; normalization of Source/Located/Realized/ShowRecipe preserves exact plan edges and original-versus-visual ownership. Root and post-child work use the index owner's closed phase domain, including nonlocatable zero-paint contributions. Unplanned/generated-only calls cannot borrow the caller's cursor by accident. Keep selected running-template bindings and page-effective environment as private sidecars, never PageConfig fields. Install no Engine and execute no Func. The new approved facade is the only additional public function owned here; its declaration is exactly the frozen API package.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.


The existing FigureCaption and FootnoteEntry standalone arms remain zero-render. Their canonical source children retain the same introspection traversal as the existing walk, through exact typed child packets without painting or activating a new feature mode. Prospective bibliography children/identities require the same actual membership proof as the Ref owner. Deferred packet tokens reserve semantic callsite order independently of final Location geometry; the protocol owner defines reservation and rollback.


## Private deferred Note frame transport

### Measurement before correction

`lab/results/set-page-deferred-corrections-2026-10-21/red-2/tests.log` records
the actual missing Note-subtree admissions. The later root-controlled fitting
measurement and pinned oracle page-text evidence distinguish a complete Note frame
from its accepted prefix and continuing suffix. The oracle keeps NOTE TEXT/START
in the first page and emits the later Figure in the second; whole-body deferral
or moving the complete call does not establish that language behavior. Exact
before bytes/hashes/HEAD/dirty state for this owner are archived in
`lab/results/set-page-deferred-corrections-2026-10-21/agents/index/note-transport-before.json.gz`.
The root authorizes an internal layout parity correction, with no eval/layout
phase change, public contract, new identity source or attempt budget.

### Exclusive transport obligation

Store three layout-private carriers: DeferredFootnoteFrame owns captured
height/items/decorations/orphan alignments and certified FootnoteBreak values;
FootnoteBreak owns item_count/height; FootnoteCapture owns breaks/atomic_depth/
origin_y/enabled. The feature alone sets enabled from its exhaustive cut
certification; transport never grants eligibility. Keep the optional captured frame queue aligned with existing numbered
Content and exact source-cursor queues. Initialize frames empty, reserved height
zero and capture None. Register private `footnote_fragments`; its exclusive owner
implements certification, reservation and frame selection, not this hub.

Store private footnote_reserved_height independently of physical page dimensions.
The root page_bottom_limit subtracts that reserved height, while subframes keep
their local height and Auto remains infinite. Do not modify PageConfig or
regions.current.height to encode notes. The earlier terminal-only no-stored-field/
new-note-model restriction is superseded only for these private captured carriers.
No public Layouter/entity field, Content variant or public signature is added.

Validate actual split Note text/page/counter/query/identity and no-fit progress,
plus existing no-break notes, columns, subframes, rollback, floats and running
content controls. Preserve strict trace coverage/foreign authority, warnings,
canonical owner paths and forward/reverse lineage; no suppression or fixture
weakening. These transport declarations alone do not certify general pagination.


## Certified consumed note-spacing boundary

`lab/results/set-page-deferred-corrections-2026-10-21/break-5/receipt.json` and
`break-5/tests.log` prove a certified START cut6.6pt followed by actual source
VSpace180pt; preserving that entire trailing gap in the tail yields222.9pt and
incorrectly triggers the oversized warning. Current pinned upstream
`lab/typst-original/crates/typst-layout/src/flow/distribute.rs:219–227` consumes
absolute spacing in its current region, `:391–394` stops before the following
non-fitting block, and `:650` clamps the emitted frame size to that region.

Add only private FootnoteBreak.gap_start_height: Option<f64>. None identifies an
ordinary line/atomic boundary. Some retains the actual occupied prefix height
before a source-owned consumed VSpace ending at this boundary. Checkpoints and
queue/frame clones transport it exactly. The feature owner may clamp only this
certified trailing-gap prefix to actual remaining region height while consuming
the original source space once; it must not trim arbitrary paint or unknown gaps.


## Exact decoration ownership at a certified cut

`lab/results/set-page-deferred-corrections-2026-10-21/decoration-red-2/receipt.json`
and its test log establish the real split defect: the valid actual first-line
cut returns no head decoration where the original capture owns one. The earlier
`decoration-red` failed an invalid precondition equating preexisting decoration
and text baselines; that refuted premise and original test remain archived.
Preserve the original decoration baseline offset exactly.

Private FootnoteBreak.deco_count records the actual ambient collector length at
each source-owned committed line/atomic/spacing boundary. Partition the original
DecoSegment vector at that exact count, preserving order and segment identity;
shift only the tail's baseline by the same actual cut height as its painted line.
Tail cut witnesses subtract the consumed decoration count just as they subtract
the consumed item count. Never compare absolute decoration baselines to normalized
cut heights or change the decoration producer to conceal the mismatch. Preserve
the unchanged V2 causal guard and all original source/painter controls.


## Private assigned-width transport (2026-10-21)

### Measurement before specification

`lab/results/nested-fractions-repeat-2026-10-21/measurement-before-decision.json`
pins four fresh bilateral receipts and their UTC intervals, HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, exact uncommitted source inputs and
`git diff HEAD --stat`. Product SHA-256 `94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`
is compared with pinned vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
`boxed.rs:196–305` mixes child fraction boundaries into the exterior denominator.
Pinned `typst-layout/src/inline/line.rs:577–591` allocates the final Box width
before its actual body. `word_layout.rs:31–44` shows why translation of paint
prepared at a provisional width cannot repair general body wrapping.
These are observable language geometry/effect requirements; tree representation
is private mechanics, and source inspection alone does not certify parity.

### Exclusive transport contract

Store private `inline_allocation: Option<Arc<sequence::inline_allocation::Node>>`,
initially None. The immutable allocation tree carries only certified geometry
for the actual ordered source traversal; no Location, source identity or effect
authority is inferred from it. Sequence owns preparation and scoped consumption,
Box/h own use. None differs from an assigned zero. Preserve all existing public
fields/signatures, typed source edges, callback ordering and runtime contracts.

This is an internal layout parity correction under ADR-0127 continuous flow:
no public entity field, trait method, signature, flag/default, compatibility or
eval/realize/layout phase change. L0 precedes meaningful RED→GREEN and scoped
resealing. Require actual nested/weighted/zero-width/source-once/decoration,
subframe-restoration and rejected-suffix tests, all prior geometry controls,
bilateral parity, full workspace tests, Clippy/rustdoc/fmt, hashes and canonical
forward/reverse lineage, with every warning retained.


## Private first-flow baseline witness

### Measurement before correction

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/zero-source-after/measurement.json.gz`
binds actual source, candidate/oracle SHA-256, fonts, HEAD, uncommitted diffstat
and UTC. Zero-copy Repeat loses its prepared baseline, moving adjacent BEFORE/TAIL
while its body height remains real. Source inspection at pinned
`lab/typst-original/crates/typst-layout/src/repeat.rs:19–29`,
`inline/box.rs:32–47,79–89`, and `flow/distribute.rs:689–699` shows that
a prepared frame retains baseline even without copies, and the FIRST actual
in-flow frame decides baseline presence. The local Box paint scan cannot see it.
Exact preamendment owners/sources and state are archived in
`lab/results/nested-fractions-repeat-2026-10-21/baseline-witness-before.json`.
These are language geometry obligations; private carrier representation is mechanics.

### Exclusive private obligation

Own private flow_baseline and last_sub_frame_baseline, both Option<Option<f64>>, initially None. Outer None means no certified frame decision; Some(None) means an admitted first frame without a baseline; Some(Some(y)) carries its actual baseline. Do not invent paint, font metrics, dimensions or source actions. Preserve public entities/signatures and existing dispatch.

Require genuine RED→GREEN for empty Repeat baseline, positive copies, earlier Text,
earlier empty frame, nested insets/shift, Shape/empty controls, LIFO and checkpoint
restoration, then the same pinned bilateral SVG/PDF observations without relaxing
1e-8 geometry or semantic membership. Unknown legacy producers retain their
previous fallback unless a measured owning frame explicitly supplies a decision.
This is an ADR-0127 continuous private layout correction, with no public contract,
new product mode/default, compatibility break or eval/layout phase change.
L0 precedes implementation and scoped resealing; full tests/Clippy/rustdoc/fmt,
zero Tekt errors/warnings, paths and forward/reverse lineage remain mandatory.


## Measured nonexpanding Repeat alignment — 2026-10-22

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/repeat-refuters-before/measurement.json.gz`
retains exact source/font/binary/build inputs, UTC2026-10-22T12:02:34.973885+00:00
through12:02:51.345977+00:00, uncommitted HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227` and its dirty-file stat. Product
SHA-256 `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`
and ratified oracle commit`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
are pinned. The eight-source manifest and full matrix are in the adjacent
`agents/repeat/` evidence. Nested fixed-child center/right and unequal-width
multiline center/right fail; decorated multiline also loses line paint.
Direct Shape, natural text and an explicit inner-left override are GREEN.
Repeat clears inherited alignment before composing its piece; pinned upstream
`typst-layout/src/repeat.rs:19,55–59` preserves lexical styles and separately
places copies. `shape.rs:251–283` emits flow geometry without line alignment,
refuting global translation. Nested-only inheritance does not fix the measured
multiline defect. Language geometry/paint are required; data structures and
iteration mechanics are free under ADR-0107. Classification and pre-edit hashes
are preserved in `agents/repeat/classification-before-L0.json`.

### Private contract

Own optional private Repeat piece-alignment state initialized absent. Own the private optional line association on DecoSegment; it is not source identity, a packet, a Location or a public FrameItem field. State belongs to the current local geometry scope only.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


## Complete singleton bidi entry after statement whitespace correction

### Measurement before decision

`lab/results/remaining-parity-2026-10-22/baseline-source.json.gz` binds the
uncommitted source at HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`,
UTC `2026-10-22T11:48:34.231064+00:00`; the companion `baseline.json.gz`
retains the exact changed-file list, diffstat and product identity. Candidate-7's
`validation/tests-domain-infra.log:7934-7938,9512-9515` records the unchanged
source-context tests failing after the statement-whitespace correction. These
records establish a lost certified context, not general bidi parity.

The exact baseline comparison leaves paragraph/sequence/bidi-origin/style-scope
and their failing tests unchanged. `compiler/lexer/markup.rs:346-384` keeps a
space before an alphanumeric character inside Text. The new statement filter in
`compiler/eval/markup_stream.rs:85-91` correctly removes the post-statement
newline; its normalization/lexical-style fold now permits direct Styled(Text)
instead of relying on an incidental leading Empty to retain Sequence. Only
Sequence and explicit Par currently install complete logical context; Text
consumes an existing cursor, and Styled dispatches its actual body. Restoring
forbidden whitespace, inserting a virtual Sequence/Par, or independently
certifying a descendant of an opaque enclosing object would be incorrect.

### Exclusive dispatch integration

Delegate the existing content-body entry to the private complete-entry wrapper
owned by `00_nucleo/prompts/core/compiler/layout/bidi_origin.md`, then execute
the original body dispatcher once. Keep the dispatcher static/exhaustive and
thin; no certificate logic in the hub, new source edge/virtual tree, locator or
admission event, field, phase or public signature. Both normal and planned
Located body dispatch use this same boundary. Existing Sequence/Par authority,
opaque-root refusal, source spans, callbacks, transactions and route admission
must remain intact. The called owner governs temporary bidi-state restoration.


## Weak pagebreak parity correction — 2026-10-23

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/baseline.json` binds the uncommitted source/L0/config state to HEAD, dirty stat and UTC; `before/receipt.json` binds both verified executables, inputs, commands and observations. `pagebreak.rs:21-58` ignores weak and always requests transitions; `finish.rs:103-106` requires the newly opened page unless it is a PageRun boundary. `page_run.rs:126-131` unconditionally closes its body. The pinned source `lab/typst-original/crates/typst-library/src/layout/page.rs:554-557` explicitly defines skipping weak breaks on empty pages; its tests `tests/suite/layout/pagebreak.typ:15-24,61-76` document weak/material distinctions. Initial, repeated, terminal and strong/weak controls plus parity and explicit PageRun probes distinguish language intent from mechanics. Page membership, content and geometry are observables; the private successor flag is a chosen mechanism. The inference that an optional successor suffices is refuted by lost notes/placed content, changed strong/parity empties, or leaked rollback state.

### Decision and exclusive responsibility

Store a private weak-page-successor flag, initially false, distinct from the existing PageRun boundary. This is internal layout state, without public entity/trait/signature changes. Physical-page materiality must include closed page-column buffers when deciding whether a weak break can collapse.

This is an ADR-0127 continuous-flow language-parity correction: L0 first, reseal, RED→GREEN and complete revalidation. No public contract, default mode, pipeline phase or compatibility change is authorized. Existing nucleus obligations and unrelated limits remain binding.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Store a private page_has_unpainted_material witness, initially false, alongside weak-successor state. Only owning feature adapters record explicit unpainted page material; isolated subframes must not mark their parent page. This witness describes admitted language content, not guessed painting emptiness.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## R1 — retain the formatted entry marker (2026-10-23)

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/baseline.json` and `before/receipt.json` bind exact HEAD, uncommitted changed-file list, source archive, UTC, executable and fixed oracle identities. Successful note and note-successor probes expose generated `[1]` instead of `1`; custom exposes decimal bracket footers despite correctly formatted inline `a` and `(ii)`. Source `footnote.rs:45–48` formats the call; `footnote_fragments.rs:286` and `footnote_flush.rs:156` independently synthesize brackets. Oracle `typst-library/src/model/footnote.rs:305–338` realizes the original note numbering as superscript in its entry. This is visible language morphology; internal queue representation is mechanics.

Classification: ADR-0127 continuous parity correction, private layout metadata only, no public contract, default mode, pipeline phase or compatibility change.

Exclusive obligation: Store each pending footnote as a private tuple of diagnostic number, original boxed body and generated entry-prefix Content. Existing cursor/frame queues remain aligned. No public entity or API changes.

Acceptance: tests first must demonstrate actual marker failure before implementation and pass afterward. Cover default, custom and intentionally bracketed numbering, literal body brackets, captured and uncaptured emission, and checkpoint preservation. Reuse generated Content rather than export text replacement. Preserve the existing body styles and note-area model, source admissions, diagnostic behavior and unrelated tests. Full build, tests, lineage and linter gates are required. This does not certify broader entry typography, column placement, nested-note body completeness, call/body migration or oversized-note continuation; measured prior limits remain open.


## R8 private note geometry reference

Evidence precedes storage: the R8 source-bound before/receipt.json under
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24 and pinned
flow/compose.rs:892–900,979–989 distinguish85% entry text from outer-size gap0.5em
and clearance1em. Nested entry size stays at root entry size. Store only a private
outer_size: Pt in DeferredFootnoteFrame and FootnoteCapture, captured before entry
scaling and copied through splits/checkpoints. These are descendant-only layout
carriers, not public entities or APIs. The fragment owner calculates geometry;
the hub owns initialization/transport only. Require existing RED→GREEN, source
once, nesting/split/rollback controls and complete validation. ADR-0127 continuous.

R8 legacy capture dispatch: hooks currently run only from
page_running_requests.rs:522–542, while ordinary layout_content clears the source
cursor and bypasses those hooks. candidate2/split-{p}.svg places END at228.2788pt
on a100pt page. Wrap only ordinary captured-content dispatch with the same
feature-owned begin/end atomic and completed-spacing hooks, once per dispatch;
planned dispatch already owns its hooks and must not receive them twice. No new
source packet, source re-evaluation or phase. The note feature still exclusively
certifies boundaries. Source callbacks/counters and rollback remain unchanged.


## R8 horizontal superscript compensation is paint-only

Source evidence precedes this correction: pinned inline/line.rs:458–484 applies
script compensation by Frame::translate without changing advance. The approved
metrics tuple supplies this offset; current text.rs adds it to cursor_x, which
also shifts following text. The existing real font fixtures have zero X offset,
so injected nonzero provider controls are necessary refuters, not font-name cases.
Keep logical advance, wrapping, justification and source identity unchanged;
translate only the text/highlight paint of the actual script run. This is private
formula/state correction, no new public field/API/phase (ADR-0127 continuous).
Own a private text_horizontal_offset: Pt, initially zero; child text and word owners set/use it. No geometry formula in the hub.
Require nonzero injected offset and following-text/rollback/subframe RED→GREEN, actual-font controls and complete validation.


R8 continuation line-top certificate: svg-candidate3/receipt.json measures END
at the correct page/baseline but its separator4.675pt too high, exactly0.5em of
entry9.35pt. split_frame currently rebases tail items by the preceding source-gap
end, retaining the interline leading before the continuation. Store a private
FootnoteBreak.tail_start: Option<f64> certified by the actual next ordinary text
line's top after baseline admission, in the same normalized coordinate system.
The feature sets this once per actual boundary, only with enabled capture and
outside atomic depth; no arbitrary Y slicing or source identity inference. Use
that certified tail origin for item/deco/orphan/break rebasing and tail height;
head height/positive-source-gap capacity remain unchanged. Preserve None for
uncertified or script/math first-line geometry. The hub owns storage only.


R8 reentrant-drain evidence: candidate4/receipt.json and svg-candidate4/receipt.json
move END back onto page1 after terminal clearance is corrected. finish.rs drains
once, then next_region drains the same region again before commit. The newly
normalized short tail now fits below its own head on that repeated drain, which
is not a new page. Add only private DeferredFootnoteFrame.deferred_region recording
(pages.len(), current_column, segmented_par_line_column) for a split tail. The
flush owner marks the actual region at acceptance of the head and refuses that
tail and following queued notes while the same region is still active. A genuine
page/column sibling permits the tail; checkpoint/split transport preserves the
stamp. Initialize unmeasured/root captures with None. This stamp is physical
progress metadata, never source identity or a new pipeline phase/public contract.
The mod owner stores it; fitting remains exclusively in footnote_flush. Require
END on exactly the next region, no duplicate/lost body, reentry/rollback and
page-column controls plus all final gates.

## R8 private physical-region type name
Measurement before decision: validation-complete5/clippy.log reports type_complexity at DeferredFootnoteFrame.deferred_region. Name the same private tuple (physical page index, column index, optional segmented paragraph column) as FootnoteRegion and retain Option<FootnoteRegion> in the frame. No value, equality, dispatch, field visibility, public contract or runtime policy changes; no lint suppression. Preserve passing functional gates and rerun full final validation after the canonical reseal.


## R9 — long notes with explicit line breaks (2026-10-24)

Measurement precedes this correction: long-footnotes-r9-2026-10-24/baseline.json and before/receipt.json under lab/results/construction/diagnostics/ bind HEAD, exact dirty stat, UTC,2162 inputs and current R8/pinned oracle binaries. h80-n5 places1/2/2 body lines versus0/2/3; both-zero instead requires1/3/1; orphan-zero requires1/2/2. h80-n6 through n9 lose late words because finish.rs:75 bounds drain by queued note count. following-flow incorrectly reserves a non-admitted first fragment and sends TAIL to another page. Original bytes and failed controls remain immutable.
Pinned flow/collect.rs:196–239 resolves strictly positive widow/orphan costs, protects first two/last two and whole-three, ignoring empty neighboring lines. flow/compose.rs:650–730 budgets the first entry against current flow+separator+gap, while757–777 inserts already laid out future-region spill. These source structures are not mandated; physical words/pages, source effects and geometry are language observables.
Classification: private vanilla-parity/correctness correction, ADR-0127 continuous flow. No public field, trait, signature, mode/default, eval/layout phase or compatibility break. Preserve prior gates, callbacks once, static exhaustive dispatch, source anchors and pure injected metrics; no arbitrary painted-frame semantic inference or hard-coded font/page fixtures.

### Exclusive owning obligation

Own only private storage: DeferredFootnoteFrame gains source-line certification and continuation booleans, initially false, and FootnoteBreak an optional protected-group height, initially None. Fragment owner authenticates these from original source plus actual committed-line witnesses. Preserve them across clones/checkpoints and splits; root is not continuation, accepted certified line-tail is. Keep actual deferred_region stamps. No formulas/selection/admission in the hub. All public interfaces unchanged.

Acceptance: source-backed module RED→GREEN before implementation, independent positive/zero/negative cost refuters, complete long-note words/markers, no-progress warning escape, actual PDF/SVG physical geometry at unchanged0.001pt, R8 and A–H/R1–R7 preservation, resource recipe, complete workspace/Clippy/fmt/Rustdoc/lint and both lineage gates. Preserve failed attempts and all received unrelated changes.


## R9 — source page-column admission and drain

Evidence before decision: candidate2 and candidate3 receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve exact inputs, dirty stat, UTC and binaries. The first passes40/41; the wider68-case matrix refutes retaining context only at segmented exit: page-columns follows columns/flow.rs:119–126, which closes active columns before terminal draining. cols2-h80-n2 also measures Alpha atx20 versus155.2pt; cols2-h100-n3 unnecessarily defers a fitting whole-three entry. Pinned flow/compose.rs:650–777 measures admission at the source carrier and retains future column regions. These physical positions/pages are language, private state/dispatch is mechanics. This supersedes the refuted column follow-up saved in l0-column-followup.json, whose source changes were withdrawn without changing received code.

Classification: private parity/correctness, ADR-0127 continuous flow; no public field/signature/default/phase/compatibility change.

Exclusive owning obligation: Store a private page_column_notes boolean, initially false, distinguishing an active source page-column flow from an ordinary columns container. Keep source-line frame/break certificates as specified; discard the refuted queued column-context design.

Require actual carrier-column RED→GREEN before implementation, the wider68-case PDF/SVG contract at unchanged0.001pt, original R8 and A–H/R1–R7 controls, unchanged effects, resources, full workspace validation, both lineage gates and preservation audits.


## R10 — source superscript runs and mandatory breaks

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,baseline-audit.json,approved-metrics.md,approval.json}` binds received R9 source/UTC/dirty stat and pinned bd5ebec2 binaries. Six of fifteen controls fail the unchanged complete physical words/pages and0.001pt gate: three literal-newline cases, Nimbus hard break, and Nimbus/Liberation mixed fallback. Pinned inline/collect.rs:191-196 includes mandatory breaks in source script text; inline/shaping.rs:970-1004,1090-1107,1167-1217 retains outer size, actual face/run and full causal coverage. This is language geometry and source preservation, not a byte/algorithm obligation. Font-specific synthetic metrics cannot be guessed from the primary face after fallback. The exact additive FontMetrics::text_superscript_runs contract was approved at SHA65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9; every other public method/field/default/phase stays unchanged.

Classification: approved public method plus private parity integration under ADR-0127. No font I/O, callbacks, dynamic evaluation or font-name coefficients in L1. Require real module RED→GREEN before runtime code, default/override/borrowed contract tests, source partition and restoration, native/synthetic/mixed-face/warm-cold/explicit-size refuters; all original controls, R8/R9 and A–H/R1–R7 replays, PDF/SVG0.001pt and full workspace/lineage/resources validation. Preserve all historical observations and unrelated received input bytes.

### Exclusive owning obligation

Store only a private superscript_source_break boolean, initially false. The style-scope owner authenticates/restores it; text uses it only for a superscript request. The hub owns no new scan, rendering or font selection.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

Store an additional private literal_script_break_pending boolean, initially false, and delegate its source-boundary observation to the text owner at existing dispatch entry. No predicate/formula/selection in the hub and no new phase or source dispatch.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.


## R12 — shared-flow footnote entry settings

Measurement before decision: `lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/{baseline.json,received-source.json.gz,before/receipt.json,before-expanded/receipt.json,type-probes.json}` binds the received R11 dirty tree at HEAD212aed044, fixed oracle bd5ebec285505889e8ff1d4f68804ab147a699f3/tree cac836c07e4dec2a89219370428292393a3216b5/SHA9b237315..., sources, binaries, UTC and exact changed-file list. Original indent/gap are ignored; explicit separator:none retains a rule. The short clearance control alone is not an admission witness. Expanded54 sources include actual three-font settings, affine units, aliases, text/string/line separators, shared versus local and late scopes, and literal continuation. Clearance30pt at80pt page height yields three vanilla pages (carrier, empty region, complete entry); received yields one. Local block overrides and late settings do not change the shared entry configuration; a global2em indent also applies to an entry called under a local0pt override. Pinned rules.rs:400–410 resolves indent inside the85% entry; flow/mod.rs:275–282 resolves separator/clearance/gap from shared outer styles, and flow/compose.rs:645–677,794–833,977–994 uses actual separator height and gap for admission/placement. Integer lengths including0 are rejected, string separator accepted, positional and unknown settings rejected. These observed geometry/syntax/diagnostics are language; storage and dispatch mechanics need not mirror upstream.

Classification: internal parity correction under ADR-0127 continuous flow. Keep all public entity fields, traits, signatures, modes and pipeline phases unchanged. New state is private layout state only, not a new language setting. Preserve the exact received R11 source and historical evidence outside this declared scope.

Exclusive owning obligation: Own only the private shared entry-scope snapshot and captured-frame configuration fields with their initialization. Delegate source-prefix/settings decisions to footnote.rs, keeping the core dispatch lean. Snapshot the first genuine source flow before child/callback dispatch: transparent Source/Located/Styled/Sequence wrappers may expose the common leading style prefix; Empty, weak whitespace and SetPage transport are not material siblings. Stop at genuine siblings, blocks and explicit page boundaries rather than borrowing a local or late style. Ignore leading nonmaterial calls until a real flow snapshot can be established. No global mutable state, extra public contract, source rewriting, or layer reversal. Captured frames own resolved settings so ordinary style restoration cannot discard them.

Require source-bound assertional module RED→GREEN before runtime changes; actual PDF complete per-page words/page sizes and SVG glyph/rule geometry at unchanged0.001pt in debug/release; preserve original R8–R11 corpora and all A–H/R1–R7 language/render controls, full workspace/tests/Clippy/fmt/rustdoc, full lint and both lineage gates, resource/stress gates and received evidence. Configuration warnings and old unrelated limits remain explicit.

### R12 private visibility hygiene

candidate1-tests.log/build.log report private_interfaces for the new captured entry field: its parent-module pub(super) visibility extends to compiler, while its layout-only settings type is intentionally narrower. Keep the field private in its owning layout module; descendants can access it under ordinary Rust module privacy. No public contract is exposed or changed and no lint is suppressed.


R12 additional source boundary: further-scope/receipt.json under the R12 diagnostic group refutes retaining this private shared snapshot across the already-existing SetPage new_page transition after material. The set_page owner now exclusively invalidates it after flushing the old flow; this owner keeps its existing prefix/resolution logic and captured settings. Initial configuration and automatic pagination remain as received. Two new module REDs precede the hook, and final acceptance expands to58 positive PDF/SVG sources per build with fresh complete validation; first-scope-attempt retains the earlier valid source state.

## Image flow and horizontal paragraph correction — 2026-10-27

Measurement precedes this correction: `lab/results/construction/diagnostics/image-flow-cause-2026-10-27/{baseline.json,results.json,refuters.json,completion.json}` binds HEAD212aed044, uncommitted inputs/diffstat/UTC, pinned64c5e8c0/tree483bd6f1/SHA22897f93 and received Tekt SHA2941a5ba. Direct PDF streams show image1 at y5 and images2–16 at y13.2; vanilla has eight30x20pt images per page at step46.4. Removing h gives vanilla step33.2 but retains Tekt overlap. Margin30 still resets later images to13.2; a60x40pt image does too. Source `image.rs:198,325–335` advances cursor without publishing the physical end consumed by `cursor.rs:128–169`. Vanilla `rules.rs:771`, realization `lib.rs:1061–1087` and flow `collect.rs:162–188,240–285` distinguish image blocks and h-only paragraph gaps. These are language/geometry observations, not a requirement to copy buffers, PDF bytes or upstream execution structure. No upstream intent is inferred.

Classification: private parity repair, continuous flow ADR-0127. No public fields/signatures/trait methods, new product mode, phase change, dependency or compatibility break. Keep all received footnote/R11–R13/JPEG work. New shared obligations are pinned in the horizontal-space-paragraph Nucleus; this owner authorizes only its exclusive consumer.

### Exclusive correction

Own only an optional private pending horizontal-paragraph spacing value, initially None, and its storage declaration/initialization. All formula/selection/closure logic belongs to h_space.rs. No extra dispatch algorithm or business logic in the hub. The existing source/plan traversal and identity remain unchanged.

Require targeted RED→GREEN, bilateral PDF/SVG<=0.001pt, source order/complete image counts, all source-margin/size/spacing refuters, prior R8–R13/notes and A–H/R1–R7, full build/tests/Clippy/fmt/Rustdoc and full lint without warning/error. Preserve known tracking/paint limits; remeasure JPEG latency/RSS against received/legacy/oracle and retain R6 resource bounds. Snapshot provenance and preserve unrelated inputs/historical receipts. Validate V15/V26 before restricted resealing; no commit/push.

### Measured lexical horizontal scope refinement

The candidate2 frozen state and pilot/geometry.json in image-flow-fix-2026-10-27 show mixed-h-scopes and siblings-h-scopes differ by43.2pt at the third image, while sole-h-scope and wrapper-h-scopes match. Equal styled siblings do not establish a shared lexical paragraph wrapper. Resolve h-only paragraph spacing from the nearest actual common StyleChain ancestor of all producers. Preserve the original anchor and monotonically narrow to its ancestor; do not infer lexical scope from equal values. Existing raw_deltas supplies identity without a public contract change. The private pending state stores an Arc-backed source chain and ancestor depth; normal one-producer closure resolves directly, mixed scope may reconstruct only the ancestor suffix for resolution. Checkpoint clones, subframes save/clear/restore, region reset and real-line absorption apply to this complete state. No public API, source reevaluation, paint, dependency or phase change. Require RED→GREEN for mixed outer/sibling scope and sole/wrapped controls, then rerun full validation.

The horizontal paragraph state field is private to layout and its descendants. It must not advertise compiler-parent visibility wider than its h_space-owned private state type. No public contract change.


## Readiness: automatic number baseline and per-page style (2026-10-28)

### Measurement before decision

readiness-fixes-2026-10-28/{article-fields-before,article-fields-extra-before,
page-number-total-before}/receipt.json preserve exact source/font/compiler/oracle
hashes, current source-bound wj2 build/archive, HEAD212aed0449150dd296ceea13fc2be665cc5db01a,
dirty stat, UTC, commands and frozen productive/L0/fixture inputs. Oracle commit
64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree
483bd6f1d8ac8810db47b65336d402647955fd17, executable
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The default bottom number is7.238pt too high at11pt; at17pt its first-page
offset is11.186pt. Bold/DejaVu produce different measured top edges, refuting
a universal constant. The top unary/compound control already matches. A final
page loses the17pt/bold/DejaVu style, and compound patterns borrow11pt even for
the first17pt page. Explicit CUSTOM footer is a DIFFERENT measured negative;
it is not fixed or waived here. Body text and None-number controls are retained.

Source page_commit.rs:62-111 and finish.rs:123-174 emit Text directly at the
number-position anchor. finish.rs:247-275 later builds compound text using the
then-current chain. The retained page-effective chain already exists and is
used for running content; this correction consumes it rather than introducing
another source/style authority. Positions, styles, words and pages are language
observables; tuple/record representation and queue mechanics are not parity.

### Exclusive correction and acceptance

Replace only the existing private pending_page_numbering tuple element with the feature-owned private record type. Keep the same Vec, initialization, lifetime and checkpoint length/truncate protocol. Do not add a public entity field, PageConfig field, Layouter field, new facade or business logic to this hub. The record owner captures style and geometry; terminal/intermediate consumers keep their actual existing phase.

ADR-0127 continuous internal parity correction. L0/pins first, actual RED before
code, V15/V26 before resealing, exact source words/pages/0.001pt and independent
PDF/SVG paint. Preserve all previous running/source/callback/rollback/notes,
Unicode, layout, technical/lineage and historical/JPEG resource gates without
new tolerances, ignored tests or suppression. No overall readiness claim.

## Approved actual-glyph optical margin ports

Measure first: physical5-candidate/receipt.json and comparison provenance under
readiness-fixes-2026-10-28 bind bed9a095… source/build/UTC/dirty state and exact
article. Accepted justified lines are1.935998pt short at terminal period. Pinned
inline/line.rs:493-526 adds optical remainder only after line selection; :676-716
uses actual GID/face and advance. Do not modify candidate choices or costs.
The direct human approval in optical-overhang-metrics-approval.json binds review
SHA-25629d7f6fcd280ecfe5abd27b54c50b8326822504ea397ab978e62771de9c10ecb.
It supersedes no-new-API clauses ONLY for the two exact methods in that review.
All earlier APIs, phases, source/effect and validation boundaries remain.

Declare only the private optical_margin descendant; keep its constants/helpers
private to layout. No Layouter field, public facade, mode or dispatch changes.

Require real RED→GREEN, strict lineage/lint/full suites/build and fresh paired
PDF/SVG, keeping all historical negative and performance/resource obligations.

Keep a private optional final lexical text-overhang decision, recorded by
word_layout::push_text and reset by actual line flush. It certifies only a
terminal text occurrence admitted by the optical policy; missing evidence does
not fabricate an enabled policy. This prevents style restoration from changing
the already emitted terminal occurrence. No public carrier field is added.


## Approved declarative Table alignment — 2026-10-29

### Measurement and authorization before decision

readiness-fixes-2026-10-28/table-align-before/receipt.json and
 table-align-extra-before/receipt.json under lab/results/construction/diagnostics/
retain the exact physical12/pinned64c5e8c0 binaries, HEAD, dirty input/diffstat,
UTC, source hashes and bilateral geometry/errors/effect order. The signed review
review-table-column-alignment.md SHA256
1e73466f33e416cb14541575124b16244272079ac336e04159ac89ce1bdef66e
was approved by the human as “Aprovar o contrato proposto”; receipt
lab/results/construction/diagnostics/readiness-fixes-2026-10-28/table-column-alignment-approval.json.
This gate covers exactly TableAlignment and TableElem.align plus the explicit
runtime-none rejection and unsupported-function diagnostic. It adds no other
public signature, Grid field, trait, callback, default mode or phase.

### Exclusive amendment

Only if required by the concrete private fallback, hold and initialize scoped private alignment transport; preserve public signatures and all subframe/nested-grid restore boundaries. Do not add this state unless necessary; no new cache or render logic in this hub.

### Gates and bounded scope

This amendment supersedes only conflicting legacy scalar/None and name-based
Table-set clauses. General Table root reflection including .align remains
unsupported; functions, Grid column arrays, rich paints/insets, multipage spans
and PDF/UA remain negative scope. Require genuine source RED before code, GREEN
for arrays/cycle/auto/presence/aliases/late/local/condition/effects/errors/spans,
header/footer/padding/colspan and hidden/replaced Table/Cell, raw/prepared and
unchanged legacy controls. No relaxation of expected output/tolerance or cache
controls. Rebuild financial original and compare PDF/SVG, words/pages/0.001pt
and independent paint without normalization; compilation does not certify the
whole financial document. Final stable batch requires valid V15/V26, resealed
forward/reverse lineage, full warning-denying lint, twelve gates, historical
1720 flags and both original JPEG resource/timing campaigns under one job,
12GiB/High8GiB/swap0. Keep failures and unrelated readiness debt explicit.


## Automatic page width and divider — issue 8525 (2026-10-31)

### Measurement before decision

The independent agent-layout and agent-svg-math reports/receipts under
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/` reproduce
infinite SVG viewport/divider geometry and PDF Letter fallback/lost second text.
`01_core/src/compiler/layout/divider.rs:35` multiplies the unbounded-region
mechanism into visible geometry; `page_geometry.rs:62-69` then includes that
shape in intrinsic page extent. `finish.rs:44-48` and intermediate page assembly
consume the contaminated result. Exact received HEAD, dirty stat, UTC and
source/owner bytes are frozen in
`lab/results/construction/diagnostics/auto-page-width-8525-2026-10-31/before.json.gz`.
This is observed geometry/content semantics; it does not establish upstream
intent. The pinned vanilla zero-width divider is a separate defect and is not
an acceptance requirement. A later wider sibling, stale index, lost content,
changed fixed-page line or queue leakage on replay refutes the repair.

### Exclusive correction

Store and initialize only a private region-local pending divider queue containing exact emitted item indices and their original symmetric margin. Delegate geometry and queue consumption to the divider descendant. No new public fields or facade, source replay, eval/layout move, or exporter fallback.

Internal correctness correction under ADR-0127 continuous flow: L0 first,
restricted reseal, functional RED then GREEN, build and unfiltered lineage.
Keep public contracts, modes/defaults, compatibility and pipeline phases.
Acceptance requires finite content-derived page/SVG/PDF geometry, a nonzero
divider when body width is positive, and complete text before/after the rule.
Do not copy vanilla zero width or force a paper size. This does not certify
other relative-size shapes, asymmetric margins or general auto-page parity.
