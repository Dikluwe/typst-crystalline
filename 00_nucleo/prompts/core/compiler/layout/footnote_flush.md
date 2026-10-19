# L0 — core/compiler/layout/footnote_flush
Hash do Código: 047c22db

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


## Ownership and evidence

Layer: L1. Sole productive consumer:
`01_core/src/compiler/layout/footnote_flush.rs` (lineage header at `:2`).
The flush body at `:40`, safe boundary at `:91`, greedy fitting at `:128`
and placement at `:188` implement the current behavior.
`01_core/src/compiler/layout/mod.rs:2187` forwards warning strings;
`01_core/src/entities/layout_types.rs:989` declares the existing document
warning carrier; `03_infra/src/pipeline/paged.rs:439` converts it to diagnostics.
Those files are evidence of integration, not additional consumers owned here.

Column placement/translation is specified in
`00_nucleo/prompts/core/compiler/layout/columns.md`. Layouter state, document fields and
pipeline warning conversion retain their own L0s; this prompt does not authorize
new fields or changes to those modules.

## Additional supporting evidence

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distinguishes regular GIDs from Oblique selection. The human confirmed public TextShaped.font: Option<Font> in `00_nucleo/prompts/core/entities/layout_types.md` (SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`). This round's snapshot/inventory are in `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [293, 300]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Flush and bounded overflow

Preserve flush_pending_footnote_bodies(bottom_y: Option<f64>).
It consumes pending numbered bodies, measures their marker plus body in a local
sub-frame of the current usable page/column width, and emits fitting bodies in
order. bottom_y optionally supplies the container's footnote-area bottom;
otherwise use the region height minus bottom margin.

Compute top_safe as the maximum of top margin, current cursor_y, the current
positioned-item Y values and the open line's item Y values, using the existing
FrameItem projection (including Semantic children).
available_h is max(area_bot - top_safe, 0).
This measured projection is not a general bounding-box/ink-extent proof.

Fit greedily by cumulative measured height. At the first non-fitting body,
defer it and subsequent bodies in order for the caller's next flush.
Force the first body only when its own measured height exceeds the full usable
page/column height, not merely the remaining current space.
This defensive escape prevents non-progress on a body that cannot fit anywhere.

Place accepted bodies top-down from
max(area_bot - accumulated_height, top_safe), compensating the sub-frame's
ascender in physical item translation. Supply the actual left origin to the
sub-frame and do not add it again as a full horizontal translation.
Retain decoration and orphan alignment-fixup rebasing, distinguishing logical
origins from ascender-adjusted physical baselines. Preserve causal numbering,
remainder order and pending-note coordination by the caller.

## Warning and purity contract

On forced emission, append the existing pure string, substituting the number:

```text
footnote body [N] exceeds available column/page height and may be truncated or overlap content
```

L1 writes no Sink, file, environment or clock. Existing Layouter warnings move
to PagedDocument.layout_warnings; the paged pipeline drains them into its
returned warnings vector as SourceDiagnostic::warning with detached span.
This consumer owns warning production, not the carrier declaration or conversion.

The user must be able to recover the note or be warned of placement failure;
mere presence of bytes in a PDF is not success. A fitting/deferred note must
not silently disappear. The forced oversized fallback explicitly permits
truncation/overlap with a warning: do not promise impossible no-overlap or
vanilla-style note splitting/continuation for that case.

## Current transport and lexical isolation

Transport TextSeparator position, consumed advance, style and origin without new paint/baseline/advance; preserve child order and exclude it from safe vertical bound.

TextReplacement's safe bound consults visual children; translation preserves text and shifts children by placement delta without new advance. The shared nucleus supplies invariants only, never extra ownership.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Migrate only this owner's shaped-face references: legacy constructors use None, transformations preserve font, and shaped comparison/join discriminates face/axes. The shaper owner's emission provides effective-font clone also on cache hit, without changing semantics/style to induce selection. Collectors/subset/CMap/PDF bind the same original face with style coordinates; PNG/SVG prepare local bindings and instantiate once per face/request before render, keeping original resource index distinct from instantiated bytes. None retains legacy resolution; existing public API/FontKey remain. No artificial fonts, unsafe, globals or new phase. Private functions may receive explicit font/bindings.

Shared shaped-face-identity invariants govern these cross-owner boundaries: associate original face bytes plus axis request rather than name/slot. Export-local tables preserve supplied resource order and append missing faces; multifont distributes items once by binding before per-font collection, never sharing global GIDs/CMap across faces. This owner remains 1:1 and materializes only its own references; the shared nucleus owns common claims.

Deferred note emission must not inherit the flushing main-flow link. Suspend ambient scopes and close/reopen their committed segment around emission, retaining explicit links inside the note. Do not change note splitting/overflow policy.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Group reconstruction transports paint_root unchanged, never true→false during projection/marker removal/rebase.

## Acceptance and boundaries

Verify empty queues, notes that fit, deferral when only remaining space is too
small, inclusion of unflushed line/cursor state in top_safe, oversized-body
termination and exact warning production, optional bottom_y, and single-origin
translation with nested placement/alignment fixups.
Integration checks cover preserved warning forwarding and existing small-note
column placement. No new source edits are implied by these checks.

The old quarantined-vanilla observation did not establish that current vanilla
lacks page(columns:). Its incomplete provenance is historical, not an exception
to the active parity target or a new default. Relevant history and outstanding
evidence limits are in
`lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.
