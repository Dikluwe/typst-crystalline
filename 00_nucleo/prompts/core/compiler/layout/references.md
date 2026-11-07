# L0 — references
Hash do Código: dfd2cd50

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/footnote-numbering.toml sha256:35a0a18aa4a83222cca7b82e79a77ec9bf254700b7e133b68d34e9907c4e5cad
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/references/unique-label-target.toml sha256:ecf43b59f502d687be7b2f4d2cc7aeb4dc7383a4c0b393f88181c770b95e4764
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:b19634c84ea14e71d8e352c96bbe5097a02a074b8c5a3009e80d761d1a90a182

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/references.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/references.rs:77`, `01_core/src/compiler/layout/references.rs:283`, `01_core/src/compiler/layout/references.rs:30`, `01_core/src/compiler/layout/references.rs:54`, `01_core/src/compiler/layout/references.rs:249`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`box-contexto-2026-10-14/probe.json.gz`preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Label and Ref layout, not introspection stores or language tables. Label renders its body transparently, then records the current physical page and its captured point in runtime maps. Ref to an existing bibliography key delegates to `00_nucleo/prompts/core/compiler/layout/cite.md` with the supplement.

For ordinary references, preserve current validation precedence: unreferencable kind diagnostics before generic existence checks, then heading/equation numbering checks and numeric/legacy resolution. Emit the exact current errors and hints, with detached spans because the element lacks a span. Numeric counter-key/location formatting takes precedence; legacy figure/resolved-label data remains fallback. Missing labels are diagnosed, not silently changed back to a question mark. Preserve internal destinations and observable fragment placement; collection is specified by the frontier correction below.

An explicit supplement, including empty Content, overrides the implicit one. Equation defaults consume its causal realized supplement; figure defaults and legacy figure fallback delegate to the language owner. Heading defaults first extract the causal prefix from resolved label text only when it ends in ASCII space plus the same formatted number; otherwise use the language owner's deterministic current-style fallback. Do not add bilingual tables, kind-label catalogs or infer a target language that was not carried. A non-empty prefix joins number with U+00A0 once.

For Page form, preserve existing known-label/page lookup, missing-numbering diagnostic/hint, explicit-versus-PageStore supplement precedence and internal link. Function numbering consumes PageStore.reference_numbering without Func execution. The current Pattern branch still formats the physical page index locally; its conflict with the older shared unary logical-store promise is unresolved. A not-yet-known page retains the existing linked question-mark path rather than fabricated numbering. This cleanup does not authorize changes to numbering views or labels.

Label keeps capture of the pre-body point and post-body page write; delegate capture/write to the position helper. Carrier shares the enclosing body's geometric result; projection applies its coordinate system before publication. Do not change selection/counters/effect order/Ref formatting.

Both Ref text emissions (ordinary and Page/fallback) use the same private internal-destination scope. Preserve resolution/error precedence/supplement/numbering/labels, without Func execution. Remove stale-length drains rather than replacing them with a content-losing clamp. This is internal ADR-0127 without a new public API/field, flag/default or phase.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

L0 precedes RED→GREEN on coordinates, no fictitious paint/height, identity/order/visual clones/rollback; revalidate HTML, geometry, context/queries and full suite.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

## V16 closed-domain evidence and correction

Measured in the uncommitted working tree at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:02:56.055664+00:00`; exact changed-file diffstat and source/L0 SHA-256 are in `lab/results/v16-corrections-2026-10-19/batch-d/agents/layout/audit-before.json`. `references.rs:278–315` gives explicit supplements (including empty Content) and causal heading/equation fields precedence. `references.rs:349–371` recognizes Heading/Equation/Table kind selectors and figure:-prefixed strings only. `counter.rs:22–26`, `selector.rs:27–69` and `element_kind.rs:16–97` expose independently closed nested domains.

This is observed local source behavior, not a new bilateral result or inferred upstream intent. Identity, text, quote choice, geometry and supplement semantics are language obligations; the match representation is mechanics. A caller admitting a supposedly excluded payload, duplicate event, changed text/geometry/request or changed diagnostic refutes preservation.

Close default_supplement_for_key across CounterKey, Selector and ElementKind at the current decision. Heading delegates its language owner; Equation/Table preserve existing language choice; a Str starting figure: delegates figure_supplement_for_lang for its exact suffix, including empty/unknown suffixes. Str without that prefix and Page receive no implicit supplement. Kind(Figure, Citation, Metadata, State, StateUpdate, Outline, OutlineEntry, Bibliography, CounterUpdate, List, Enum, Par, Link, Raw, Quote, Footnote, StateDisplay, CounterDisplay, ContextBlock) receive none. NativeElement, Element, Label, Location, And, Or, Regex, Where, Within, Before and After selectors receive none regardless of their payload; even a filter around Heading is not an unfiltered Heading key. Keep the prefix guard and its false branch visible. Do not invent labels, add language tables, erase explicit-empty precedence or alter the one-NBSP join. New variants at any nested domain require a decision at this match.

ADR-0127 classification: continuous-flow internal nominal coverage with no public signature, default, phase or compatibility change. Add behavior-preservation controls before implementation and require baseline/final controls plus disposable future-variant compile-negative evidence at the exact selected decision. Current runtime controls may already pass; the structural RED is failure to reject a future variant at that decision. No new suppression or open catchall is authorized.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Numbered Figure labels use the same typed key and target Location snapshot as captions, including captionless targets. Unnumbered target after numbered target cannot inherit its earlier prefix. Resolved target supplement including empty/None takes precedence; custom Name(table) never gets Element(table) localization. Preserve bibliography, Page and explicit reference supplement precedence; pure walk cannot freeze incomplete Func histories. No implicit HTML rendering certification.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


### Rich productive Figure reference supplement

The existing generic String resolver flattens Content supplements and cannot retain Strong morphology. For a prepared productive Figure only, render a Content sequence of explicit-or-resolved supplement, NBSP and the existing numeric Location projection through layout_link_capture. Preserve explicit empty/None precedence and exact target Location/history validation, without flattening the rich supplement. Heading/Equation/Page/bibliography/synthetic generic paths retain their existing owners. Add a bold Figure-reference emitted-text control. No new public signature, callback, string-key shadow or fabricated fallback.


## Approved reference contribution integration

Measured existing boundary before decision: `01_core/src/compiler/layout/references.rs:53 and :237` retains the old generic transport or legacy assertion behavior. Source inspection establishes ownership; the independently frozen actual L3 INNER_PREPARATION RED establishes the language omission.

The human approved frozen proposal SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2, recorded in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-phase-approval.json`, including the separate no-child clarification. Actual L3 explicit/callback INNER_PREPARATION regressions failed on the unchanged pre-implementation source (`reference-phase-red-frozen`), authorizing RED-to-GREEN for this exact phase. Existing unrelated obligations remain.

A produced canonical Ref carries its actually realized selected supplement in the existing supplement field (Some Empty for no content). Consume that field without callbacks or target-template reuse; preserve current numeric typed Location prefix, link target and rich morphology. Unprepared non-Figure/page/bibliography passthrough keeps its prior None/default/explicit semantics; no fallback may execute an unrealized Figure template. Existing lexical nearest-source errors remain intact.

No additional public contract beyond the exact three approved transport functions, new target inference, second arena, shared result/effect cache, fake observation, weakened diagnostic or unrelated generic visitor change is authorized. Preserve five-attempt stabilization, canonical paths and scoped lineage; validate reached and hidden effects, forward/repeated references, aliases, missing/unnumbered/non-Figure/bibliography/page controls and mixed target behavior.


### Nearest lexical source for all Ref layout diagnostics

Measurement before decision: reference-layout-origin/receipt.json under agents/counters binds oracle and frozen build8 errors. Heading without numbering has the same original message/hint but oracle source2:0 versus detached product; Page without numbering is source2:1 versus detached product. Eight layout_ref diagnostics at references.rs:79/95/136/146/159/174/185/202 hardcode detached spans, while productive Figure diagnostics already use current_source_span. layout/mod.rs:1208–1230 and1247–1252 installs/restores nearest non-detached Source and checkpoint.rs retains it. Thus no new RefElem field or source inference is needed.

Supersede only the historical detached-span limitation: use layouter.current_source_span for missing page/ordinary labels, absent page/heading/equation numbering, Raw/text rejection and both existing Figure snapshot errors. Preserve every original message, hint, precedence and error transaction; bibliography still delegates its own owner. Source-free synthetic layout keeps detached through the existing initialized default. Require actual source error line/column regression and synthetic nearest-Source/default control. This is continuous mapping correction with no API, phase or semantic target expansion.

## Checked ordinary-label multiplicity

### Measured responsibility

At the implementation baseline, `references.rs:53–78` delegates bibliography
before ordinary lookup, and its later normal branch uses first-target lookup.
`entities/label_registry.rs:40–55` already supplies all Locations/count without
changing its public first-lookup compatibility. The frozen current-pin repeated
header reference in `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json`
requires `label `<header>` occurs multiple times in the document`; unchanged
product's missing-label refusal is not equivalent. This establishes a language
diagnostic requirement after actual physical membership is available.

The approved review is `lab/results/set-page-header-2026-10-21/review.md`,
SHA-256 cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039.
The direct human decision is preserved in
`lab/results/set-page-header-implementation-2026-10-21/approval.json`.
It authorizes only the frozen five public facades, opaque carriers and physical
page phase; it does not authorize additional public contracts or generic
observer/traversal changes. The implementation baseline is HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`. Exact owning source/L0 bytes,
SHA-256, UTC and working-tree state before these amendments are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/before.json`.

### Exclusive downstream validation

Preserve bibliography precedence. For the existing normal Ref downstream routes,
check actual distinct label Locations before selecting the first target or
formatting its supplement/number. More than one distinct actual target emits
`label `<NAME>` occurs multiple times in the document` at current_source_span;
synthetic source-free calls keep the existing detached default. Do not treat a
repeated prepared same-Location alias as a fresh target, mutate the registry,
execute callbacks or compare content/hash/text to determine occurrence identity.

Supersede only first-target acceptance of an ambiguous normal label. Unique and
missing target validation, unreferencable/numbering paths, explicit-empty/rich
supplements, Page form, internal destinations and geometry remain with their
existing contracts. The realization owner owns its own checked producer path;
layout is not a fallback place to execute physical-template callbacks. Require
normal unique/duplicate/alias/bibliography/source diagnostics and actual repeated
header RED→GREEN without weakening prior Figure/Ref or page-link controls.

## Source-planned label body descent

At the implementation baseline, `references.rs:30–42` renders the Label body
through ordinary layout_content before recording its label point/page. The
approved source plan stores this actual body as typed LabelBody, independently
of the enclosing label's root/post-child registration. Under the planned facade,
route exactly that source body through layout_planned_child(LabelBody, body),
keeping current chain, position capture and page write unchanged. Legacy calls
with no plan retain ordinary rendering through the same private adapter. Generated
reference strings/prefixes are not source children and may not borrow that edge.
This is the approved physical-plan routing boundary, not a new API or callback.

### Injected trait boundary for the multiplicity check

The source-current diagnostic build `compile-diagnostic-1` under the physical-page
implementation evidence rejects direct labels-field access at references.rs:72
with E0609: the injected introspector is the existing tracked Introspector trait,
not a concrete TagIntrospector. Use its existing query(Selector::Label) result to
inspect distinct Locations without a new trait method or concrete downcast.
Compare Location handles only; preserve intentional duplicate handles as one
target and retain the exact ambiguity diagnostic and all precedence above.

## Bibliography-selected Ref source identity

Evidence before decision: references.rs:59–69 delegates an actual bibliography
key to the Cite owner before normal label validation. The source plan reserves
a prospective Cite identity in that Ref packet's auxiliary_locations, while
ordinary Ref remains non-locatable; conditional children are admitted only after
actual bibliography selection. Exact pre-amendment bytes are retained beside
the citation routing evidence.

After the existing bibliography-key proof, observe only that packet's first
reserved auxiliary Cite Location and record its actual current page/point through
the existing position carrier. Scope current_location to the delegated citation
and restore the enclosing value afterward. Do not allocate a Location, create a
Located wrapper, promote generic Ref locatability or inspect dormant templates.
The selected supplement follows CitationSupplement through its existing Cite
owner; generated CSL text remains unplanned. Preserve bibliography precedence,
checked normal-label multiplicity, current forms and legacy no-plan rendering.

### Mixed generated/reference source parts

Source measurement before correction: the productive Figure branch constructs
Sequence[supplement, NBSP, number] and passes it to generic layout_link_capture,
which intentionally clears source routing. The source plan records a reached
PreparedReference node's selected child as PreparedReferenceSupplement; dropping
that cursor leaves real nested source packets uncovered. The exact prior bytes
are retained in agents/realization/prepared-reference-routing-before under the
physical-page implementation evidence.

For this branch only, recognize the current source packet's PreparedReference
role and attach its PreparedReferenceSupplement cursor to the actual nonempty
supplement part. Render the parts through the private mixed-source link helper;
NBSP and numeric suffix stay generated/unplanned. Unprepared legacy fallback
retains no borrowed source cursor. Preserve existing number/supplement precedence,
empty handling, chain, destination, link fragments and geometry. Do not infer a
new target, traverse a dormant target template or execute a callback in layout.


## Approved issue8890 Figure placement — 2026-10-31

### Measurement before decision

`lab/results/construction/diagnostics/issue-8890-2026-10-31/{analysis.json,svg-geometry.json,report.md}` preserves the exact HEAD3c6b0ce7, UTC, clean/dirty state, pinned vanilla64c5e8c0 executable/source/inputs and PDF/SVG measurements. Original Figure placement is rejected in Tekt at stdlib/figure_image.rs:342-360; vanilla loses D after deferring the float. The supported Place control preserves A–E but puts its float on page3 after continuation text, so it does not certify the original trigger. Current source/owner bytes and concurrent dirty state before this amendment are preserved under `lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/before/` and before.json. Geometry/presence/positions are language; queue/carrier mechanics need not copy vanilla.

The user explicitly approved the concrete public-contract proposal with “Aprovo” in this chat; approval.json binds its exact scope. This satisfies ADR-0127 for FigurePlacement and the two public fields only, with existing layout phase/default in-flow behavior retained. The prior placement scope-out is superseded only for this approved subset.

### Exclusive responsibility

Migrate only existing Figure fixture literals/destructures for the approved two-field addition: absent placement/resolved_placement for all legacy in-flow inputs. Preserve every prior expected language result and all unrelated implementation. No behavior redesign is authorized in this owner.

### Gates and limits

L0 and scoped reseal precede real RED, code and GREEN; valid V15/V26 precede repairs. Preserve all other ownership, previous clauses/ratifications and concurrent changes. Use only /dev/shm/typst-crystalline-8890-materialization-target for this amendment's compilation/test temporaries, one job, 12GiB virtual ceiling and zero swap. This target supersedes older task-specific target paths only for the present amendment. Record state/hash/UTC around every measurement. Revalidate original and link PDF/SVG, controls and finite progress with all A–E once; page count itself is not an oracle for a known vanilla defect. No new phase/default/compatibility change beyond the approved fields. Do not suppress lint, weaken tests, copy upstream defects, commit or claim global qualification from scoped checks.


## Issue 8935 residual — physical entry scope and original note marker

### Measurement before decision

`lab/results/construction/diagnostics/issue-8935-residual-2026-10-31/{before.json.gz,baseline-build.json.gz,baseline-tekt.json.gz,baseline-vanilla.json.gz,reference-geometry-before.json.gz}` preserves HEAD3c6b0ce7, the exact concurrent uncommitted changed-file list, UTC, source/L0 bytes, argv and executable SHA-256. Current oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17, binary22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34. Compact repeated headers retain one logical member from approved #8954, but entries distribute 0/1/2 while the oracle's unreferenced control has one per page. Narrow Box reproduces deferred/narrow entry outside Table, unlike ordinary prose. Alphabetic call/entry a has decimal ref:1 outside Table; vanilla ref:a. Footnote marker styling is language morphology, not a demand to copy upstream mechanics.

`footnote_fragments.rs:690-695` subtracts page margins from the local cell region. `sub_frame.rs:373-374,813` zeros/restores reserve while new captured notes remain queued, and refresh refuses is_sub_frame. `references.rs:376-382` formats other counters as decimal without consulting Footnote; pinned model/reference.rs:281-282 clones the original note into a reference and model/footnote.rs:117-121,148-154 uses its pattern/superscript display. These observations support a shared physical-scope loss for Table/Box and a distinct generic-projection loss for references; one arithmetic cause for both is not inferred. Correct text but wrong page/entry line or marker styling, repeated effects, changed counters or measurements refute closure.

### Exclusive internal obligation

For an already validated ordinary reference whose unique target is an actual Footnote and whose existing label-counter snapshot is available, format that snapshot with the original note's numbering pattern (default 1), and render the generated marker as the existing superscript Content inside the existing destination link. This reference creates no Footnote, note body, counter action, Location or source child. Preserve Page-form/bibliography routing, missing/duplicate errors and other reference kinds. Preserve the existing generic fallback if the synthetic/legacy target has no counter snapshot; do not invent a number. Explicit Ref supplement cannot replace/add to a Footnote marker, as measured upstream's into_ref path. Require pattern/control/morphology REDs and actual outside-Table/Table references.

ADR-0127 continuous flow: this restores measured note semantics/pagination/morphology within existing layout, without public entity/trait/signature, new mode/default, pipeline phase or compatibility change. Preserve all approved #8935 identity/measurement/header replay work and concurrent Figure/math changes. L0 first, scoped reseal after V15/V26, minimal module/integrated RED before implementation, then GREEN. Revalidate all seven original #8935 tests, ordinary/Table/Box, repeat:false, later real note/query/counter/duplicate/hidden controls, general note/column/continuation pagination and complete PDF text plus PDF/SVG painting. Existing one-logical-member semantics intentionally differ from vanilla's repeated-header counter defect. Use exclusively the residual task's RAM target, one compilation job, 12GiB ceiling; leave combined workspace qualification to the coordinator.


## Footnote column and lexical-numbering correction — 2026-11-01

### Measurement before decision

`lab/results/construction/diagnostics/footnote-columns-numbering-2026-11-01/{before.json.gz,baseline-build.json.gz,baseline-tekt.json.gz,baseline-vanilla.json.gz,general-before.json.gz}` binds HEAD3c6b0ce7, uncommitted tree/exact diffstat and source/L0 bytes, UTC, argv and executable SHA. Oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17, binary22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34. Global/alias/local set is ignored at the call (H1/ref1 instead of Ha/refa); query numbering field is missing. Short standalone columns already match; long explicit columns use three product pages versus two oracle pages, and Box notes occupy the wrong column while main rows ignore note area. Existing pending queues and page_column_notes distinguish ownership, but refresh refuses columns, flow selects fixed segments on any colbreak and final flow guesses page bottom. Native physical text/coordinates and lexical values govern acceptance; buffer/match shape is mechanics. No intent is inferred from source inspection alone.

### Exclusive obligation

For actual Footnote references prefer the existing original-occurrence resolved-label marker snapshot. Keep the existing explicit-pattern/counter formatting fallback for synthetic introspectors without that snapshot. Render the same superscript/link and no note or effect. Preserve all #8935 identity/duplicate and other reference routes.

This is internal convergence/restoration of existing language behavior under ADR-0127 continuous flow: no public entity/trait/signature, user mode/default, compatibility break or eval/layout phase change. Preserve the constructor's raw morphology and all #8935 identity, measurement and header replay corrections. Nucleus obligations are shared only through these full pins; each source retains its canonical sole owner. Require minimal real RED before production code, GREEN, prior14 #8935, footnote/subframe/column suites, bilingual source controls, full PDF extraction and SVG paint checks. Use only the task's exclusive /dev/shm target, one compilation job, 12GiB process limit, preserve concurrent work and do not commit/push. Global combined qualification remains separate. Any new public carrier or phase need stops at a concrete proposal for human approval.
