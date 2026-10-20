# L0 — references
Hash do Código: 9faa66ca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
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
