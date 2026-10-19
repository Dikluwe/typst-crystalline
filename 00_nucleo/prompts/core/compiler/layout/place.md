# L0 — place
Hash do Código: 2d2290bc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/place.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/place.rs:22`, `01_core/src/compiler/layout/place.rs:141`, `01_core/src/compiler/layout/place.rs:154`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distinguishes regular GIDs from Oblique selection. The human confirmed public TextShaped.font: Option<Font> in `00_nucleo/prompts/core/entities/layout_types.md` (SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`). This round's snapshot/inventory are in `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

fragment_link_deferred_float_retains_its_lexical_target failed in extended.log: FLOAT moved y17.7→18.8 while BODY stayed fixed. extended-candidate.json.gz preserves source/L0. `place.rs:268-294` descends into Semantic and mistakes the marker's zero Shape for physical origin 0, replacing actual text origin.

`place-origem-local-2026-10-14/{before,extra-before}/results.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/diffstat/UTC/sources/binaries/argv and historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`. Box-place shape y37.75pt versus 28pt on 480×400 canvas (approximate raster bbox); source SVG for marginless tiling-corpus Box has local y14.834 versus 5. Offset grows with font. `placement.rs:463` forbids geometric origin in any subframe,`:533` retains local ascender; `place.rs:96–131` adds text top-edge even with explicit Top. Root-text shape differs 29.5/22.25pt; Box-text32/22.25; Grid/Align controls pass. Pinned `flow/distribute.rs:703–715` separates explicit align_y from flow offset. This is language geometry, not Rust buffers; lost text/VAlign/scope/fixup/checkpoint coordinates after a single origin refute correction.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` records the same HEAD, uncommitted tree/diffstat/UTC/sources/binaries/argv and historical pin. All 24 root/Box pairs after 12 prefixes differ: Linebreak/Parbreak require y22.25pt, Shape y27pt, Block/Grid/VSpace20pt y35pt, Tekt 15pt. Rotate also overlaps independently. `line_commit.rs:721–733` prepares next baseline; `shape.rs:196–206` adds below/cap; `block.rs:465` separates boundary/margin; `place.rs:91–99` recognizes still-pending line only. Pinned flow/distribute.rs:703–715 uses physical flow offset for axis-unspecified Place. Weak margins/overlap/new page/rollback/subframe leakage can refute the causal-end hypothesis.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserves same HEAD, dirty tree/diffstat/UTC/sources/binaries. Only 2 of 24 bilateral bboxes match against pin `9f2b6e8715237cb086899a42873660fe744622e8`: inset3/top Tekt(20,15) versus vanilla(20,18); bottom/right(71,37) versus(62,44); clip deletes Tekt marker. `block.rs:90` ignores inset.right;`:220` installs width as body;`:275` uses Place's zero advance as clip height;`:290` groups absolute children without rebase. `placement.rs:505` uses page/subframe without Block region. Pinned `typst-layout/src/flow/block.rs:66-89,223-234` forces inner size, adds inset then clips;`:278-280` subtracts inset from region. Geometry is language/buffers mechanics. Nesting/text/auto/clipping/sibling leakage refute local-region correction.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [408, 415]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the Place adapter plus float fitting, reservation and final emission. Merge missing alignment axes from the cell context. Non-float placement delegates geometry to `00_nucleo/prompts/core/compiler/layout/placement.md`; without explicit vertical alignment it follows the current ordinary flow origin, using the causal flow-origin contract below rather than always the physical page top. Apply that correction locally to this occurrence's items, decorations and vertical fixups; retain out-of-flow cursor behavior.

For a float, lay out/measure the body in the active isolated subframe, resolve clearance against effective font size and queue the existing DeferredFloat with dimensions, decoration and orphan alignment metadata. The language's default clearance arrives from its constructor as relative 1.5em; the adapter's absent-field fallback remains zero. The current DeferredFloat does not store dx/dy/scope, which are not applied by this float path: do not claim their support or broaden the private struct here.

Admit only an ordered fitting prefix using effective usable region and current material flow. Include adjacent clearance when flow or an earlier admitted float exists; retain the existing defensive admission of the first float on an empty region. Once realized, top/bottom reservation includes future-flow clearance. Anchor top floats at the top and accumulate downward, bottom floats at the physical bottom and accumulate upward; auto height retains its current top-placement fallback. Body, orphan paths and decoration share the final reference frame. Preserve existing top-reserve translations and bottom effective-region reduction, without applying clearance twice.

Ask `00_nucleo/prompts/core/compiler/layout/cursor.md` for region progress and `00_nucleo/prompts/core/compiler/layout/flow_suffix.md`/`00_nucleo/prompts/core/compiler/layout/mod.md` for full suffix recomposition; do not emit in-flow at the cursor, inspect a future Content or duplicate replayed occurrences. Direct private HTML markers are non-material for fitting but remain geometrically transported. Oversize/unmeasured float combinations are not full parity guarantees.

Transport TextSeparator position, consumed advance, style and origin without new paint/baseline/advance; preserve child order and exclude separators from measured vertical origin.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children preserving text, order, coordinates and existing calculations, without a new algorithm or advance. The pinned text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Migrate only this owner's shaped-face references: legacy constructors use None, transformations preserve font, and shaped comparison/join discriminates face/axes. The shaper owner's emission provides effective-font clone also on cache hit, without changing semantics/style to induce selection. Collectors/subset/CMap/PDF bind the same original face with style coordinates; PNG/SVG prepare local bindings and instantiate once per face/request before render, keeping original resource index distinct from instantiated bytes. None retains legacy resolution; existing public API/FontKey remain. No artificial fonts, unsafe, globals or new phase. Private functions may receive explicit font/bindings.

Shared shaped-face-identity invariants govern these cross-owner boundaries: associate original face bytes plus axis request rather than name/slot. Export-local tables preserve supplied resource order and append missing faces; multifont distributes items once by binding before per-font collection, never sharing global GIDs/CMap across faces. This owner remains 1:1 and materializes only its own references; the shared nucleus owns common claims.

Float frames retain lexical boundaries captured in their subframe; later emission does not inherit the draining main flow's link. Close/reopen ambient committed segment around float insertion without changing order/fit/geometry/fixups. Ignore direct private markers in item_origin before classifying items; preserve other wrappers/objects and translation formula, and keep original geometric expectation.

The nonfloating adapter follows a pending line only when effective_alignment.v is None; explicit vertical alignment does not acquire the flow offset. Do not add text top-edge: geometric body installs its own. Apply one common delta to occurrence items/decorations/vertical fixups without advancing cursor; floats remain unchanged.

Own private methods registering an end with current cursor and querying only a certificate whose cursor witness still matches. With no VAlign, prefer pending line then certified end; no paint scan or premature flush. Invalid witness retains the legacy path and is pending rather than parity credit.

Current local-region refinement passes optional flow_y to the private Place helper, which applies origin in the common formula while preserving cursor/floats. This supersedes earlier root margin.top=cursor_y borrowing and subframe posttranslation cursor_y-sub_frame_origin_y, as well as the root-only geometric-body restriction. Preserve margin restoration where required by the retained paths, without reintroducing superseded origin formulas. Only incompatible region/width/clip formulas are superseded; other repairs need measurement.

Private state required by the causal boundary is authorized, superseding earlier extraction-only no-state prohibitions to that extent; LIFO save/restore is mandatory.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Group destructure/recompose captures and reinserts paint_root unchanged; never reset true→false in projection, marker removal or rebasing.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection. Shaped-face acceptance covers RED→GREEN/bilateral miss/hit/VF without promised speed gain.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

Local-origin and causal-end corrections follow internal ADR-0127: L0 first, RED→GREEN/restricted resealing without new public field/trait/signature/flag/default/phase. Compare before/after SVG/PDF and preserve textual/mixed bodies, auto pages, clipping, observable positions and floats; validate build/workspace/Rustdoc/fmt/lineage/warning inventory, full suite, HTML/context and geometry controls.

Geometric body origin and postcommitted-flow anchor are distinct. Certified-anchor scope (historical B1) covers line/shape/Block/Align/Grid/Raw/VSpace, not the whole system. Local surviving Block region/clipping (historical B2a) does not complete Block fragmentation/dimensions/outset or metadata/decorator integration (historical B2b), nor late inline Box/Transform baseline and uncertified Heading/List/Stack/Equation/Columns producers (historical B3). Those remain explicit completion boundaries; do not certify universal Place while open. Initial statements that tiling awaited approval are superseded only within the recorded transport approval, not native production certification.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.

## V16 closed-domain evidence and correction

Measured in the uncommitted working tree at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:02:56.055664+00:00`; exact changed-file diffstat and source/L0 SHA-256 are in `lab/results/v16-corrections-2026-10-19/batch-d/agents/layout/audit-before.json`. `place.rs:289–311` projects Option<HAlign> to horizontal free-space placement, clamping only after projection; `place.rs:324–520` applies its target to items, orphan paths and decorations. DeferredFloat at `mod.rs:798–843` has no dx/dy/scope fields.

This is observed local source behavior, not a new bilateral result or inferred upstream intent. Identity, text, quote choice, geometry and supplement semantics are language obligations; the match representation is mechanics. A caller admitting a supposedly excluded payload, duplicate event, changed text/geometry/request or changed diagnostic refutes preservation.

Close the existing optional horizontal decision explicitly: Some(Center) uses half the available-minus-body width, Some(Right/End) uses all of it, and None/Some(Left/Start) uses zero. Retain the nonnegative clamp and physical Start/End interpretation, including bodies wider than the region. Do not certify RTL resolution, dx/dy/scope or clipping beyond existing contracts. Translate body, orphan alignment coordinates and active decoration segments in the same frame, retaining insertion indices and lexical links. A new HAlign must fail exhaustiveness inside Some at this exact match.

ADR-0127 classification: continuous-flow internal nominal coverage with no public signature, default, phase or compatibility change. Add behavior-preservation controls before implementation and require baseline/final controls plus disposable future-variant compile-negative evidence at the exact selected decision. Current runtime controls may already pass; the structural RED is failure to reject a future variant at that decision. No new suppression or open catchall is authorized.
