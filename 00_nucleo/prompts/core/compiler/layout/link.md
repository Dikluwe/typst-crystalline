# L0 — link
Hash do Código: dae3001e

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
