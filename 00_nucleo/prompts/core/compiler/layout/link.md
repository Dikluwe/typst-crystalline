# L0 — link
Hash do Código: 42df7c90

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425




**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/link.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/link.rs:18`, `01_core/src/compiler/layout/link.rs:51`, `01_core/src/compiler/layout/link.rs:141`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

`links-captura-2026-10-14/probes/results.json.gz`, nested case, records exterior rectangle through descending y 29.78pt while second unlinked SUFFIX line occupies y 21.792..34.332pt. `link.rs:63-75` treats second vertical_metrics return as descender and adds ascender; `metrics.rs:28-37` explicitly declares (ascender, line_height), followed by FixedMetrics/L3. Snapshot: bbox-before.json.gz.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Render the rich body once under its lexical capture scope, and wrap each final fragment in the existing URL Link without changing style to mandatory blue/underline. Preserve nested content and current cursor behavior. Own link_bbox: measure text via injected advance/vertical metrics, glyphs via their current advance/size projection, shape/image/line extents and Group/Link bounds; recurse through Semantic. An empty measured set has zero bounds. Ignore direct private HTML context markers for bounds but retain them among children. Internal destination links are assembled by `00_nucleo/prompts/core/compiler/layout/outline.md`/`00_nucleo/prompts/core/compiler/layout/references.md` using this helper, not by adding a new LinkElem target here. The old pending-tail drain limitation is superseded by the measured frontier correction below. QuadPoints remain outside this correction.

Exclude TextSeparator from link bounding box while retaining its carrier among children.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children preserving text, order, coordinates and existing calculations, without a new algorithm or advance. The pinned text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Replace capture by stale buffer lengths with private scope around one body execution. Preserve URL destination/style/link_bbox, also serving internal destinations. Fragments remain in their own buffers/pages; never gather the entire body onto the last line. Bounds use final geometry per fragment. No new QuadPoints is claimed.

For Text/TextShaped use y=baseline-ascender and height=line_height, never add ascender again. Preserve advances/destinations/child geometry; the incorrect formula does not authorize covering the next line. This is internal without API/phase change.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage. Test both text variants with deterministic metrics and inspect PDF to ensure first-fragment rectangle excludes SUFFIX. Require neither byte equality nor exact vanilla height.

## Nominal inline frame link control (2026-10-22)

Before this control, `lab/results/remaining-parity-2026-10-22/agents/repeat/link-frame-before.json` preserves source/L0 hashes, exact dirty tree and UTC. The separate read-only `agents/repeat/link-observer-before/observation.json` under that result root observes the immutable decorated-link-nested baseline and pinned oracle: one identical URI each, but the baseline linked rectangle begins at the glyph baseline rather than the Box top and has the wrong nominal width. This is actual hit-area geometry, independent of paint comparison; the PDF serialized rectangles are supporting evidence without a new tolerance. Current `link.rs` measures Group nominal dimensions, whereas the current fixed-width Box composes a Group at its actual geometric top; the baseline therefore does not prove this current control must fail or establish a new Link-specific cause.

Require a full Content::Link containing one fixed-width 60pt, fixed-height 16pt Box with real text, zero inset and zero baseline shift. With an otherwise empty first line at geometric origin (10pt,10pt), preserve the nominal Box frame hit area (10pt,10pt,60pt,16pt), its exact target, body and text geometry. The clickable area follows the actual frame, rather than the first glyph ink or glyph baseline. This control does not authorize changing Box placement, rerouting source evaluation, adding public geometry or changing capture phases. Diagnose an existing GREEN control honestly; only a demonstrated current RED authorizes a further correction. Preserve the existing text-only bounds and nested destination/page/rollback contracts.

## C06: immutable glyph storage

Evidence at baseline `0a15ccebf861e3cc0ea16d0397d55dc057664556`: `layout_types.rs:411` uses Vec glyph storage at this baseline. Fixture constructions in this consumer require adaptation to the newly authorized SharedGlyphs field. Wrapper ownership remains `00_nucleo/prompts/core/entities/shared_glyphs.md`; no other productive behavior change is authorized here.


Only test fixture compatibility is authorized here: convert finished Vec fixtures with .into(), add explicit Vec collection types when necessary, and explicitly copy to Vec in test helpers. For the existing malformed-glyph negative test, thaw a private test copy, perform the same mutations, then replace the complete frozen payload. Preserve every assertion, input, tolerance and cfg.


## Laboratory link width follows the injected text-width contract, 2026-11-12

### Measured evidence before decision

The frozen linkscope release and pinned vanilla64c5e8c02fe0394d913ead36e42ef8c1334d2107 in `lab/results/construction/diagnostics/watch-residual-2026-11-12/controlprimary-linkwidth-before/receipt.json` bind source, ELF, input and UTC. At x20pt, a link containing only U+2060 ends at26.60pt versus20pt in vanilla; A+U2060+B ends40.71pt versus34.113pt; AVATAR63.13pt versus59.435pt; ABC with2pt tracking41.22pt versus45.219pt. Ordinary ABC agrees to existing serialization precision. These are wrong clickable horizontal extents, not a byte-parity requirement; vertical extents remain a separate existing difference.

The controlprimary partial implementation makes isolated control measurement catalog-lazy but its whole-outline test remains RED (3passed/1failed). `controlprimary-catalog-trace/{receipt.json,trace.log}` locates the remaining full acquisition below link_bbox→advance→primary_for_text_advance. `compiler/layout/metrics.rs` explicitly defines text_width as the authoritative word-level injected operation, including the default shaped advance/fallback and tracking; `text_measure.rs` implements that formula. `link.rs:54,61` instead calls unshaped advance again, ignoring both shaped width and tracking and querying fallback fonts for a zero-width control. The proposed correction is refuted by bypassing an injected text_width override, changing text/paint/vertical geometry, or changing destination/lexical membership.

### Exclusive bounded parity correction

Laboratory only, no integration. In the existing Text and TextShaped arms of link_bbox, obtain the horizontal extent through metrics.text_width(text, style.size, style) rather than direct advance. Preserve both distinct carriers and all current positional/vertical-metric, Group/Link nominal-bound, Glyph, shape/image/line, wrapper recursion and marker policies. Do not shape/rewrite/move carriers in this owner, inspect font names or Unicode categories, fabricate widths, modify the trait default, change fallback or cache lifetimes, or move phases. A zero-width capture may retain its body without a positive clickable rectangle through the existing wrap guard. Prior instructions to preserve direct advance are superseded only by this authoritative-width dispatch; there is no public API/default/compatibility change. ADR-0127 continuous internal language-parity flow applies.

RED must prove both carrier arms ignore the injected width today; GREEN requires the same assertions after dispatch correction. Include an independent default text_width shaped-plus-tracking result and its None→advance fallback. Preserve all existing bounds/source/pagination/nested/repeat/rollback tests and run full core/infra/build/check/Clippy/docs/fmt/lint. Re-run the unchanged whole-outline loader refuter from the partial candidate. Actual release controls must conserve text/geometry/raster and destination membership, while horizontal hit rectangles may correct the measured kerning/tracking/control defects. Keep preexisting group-transform/footer/outline geometry debts explicit; do not waive unrelated annotation differences. Measure CPU only after exact source/ELF freeze.

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
