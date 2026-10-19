# L0 — slicing
Hash do Código: 073ce792

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/slicing.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/slicing.rs:39`, `01_core/src/compiler/layout/slicing.rs:61`, `01_core/src/compiler/layout/slicing.rs:84`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distinguishes regular GIDs from Oblique selection. The human confirmed public TextShaped.font: Option<Font> in `00_nucleo/prompts/core/entities/layout_types.md` (SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`). This round's snapshot/inventory are in `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [170, 177, 346]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Partition items by their existing projected starting y against a height threshold, preserve order in both outputs, and rebase the tail by minus that threshold. Preserve the current projection conventions: Link projects to zero and Semantic to the maximum child start; this is not arbitrary geometric clipping of a wrapper crossing the threshold. Translate tail coordinates recursively through Semantic/Link, and shift only the origin of a Group whose children are local. Deferred fixup transport is coordinated by callers, not a second registry here. The current Image rebase reconstructs clip_rect as None; preserving clipped-image masks across slicing remains an unresolved discrepancy rather than an authorized behavior change.

Transport TextSeparator position, consumed advance, style and origin without new paint/baseline/advance; preserve child order. Slice by carrier baseline and rebase position only. Recursive TextReplacement clipping preserves text/clipped children; metrics consult children.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Migrate only this owner's shaped-face references: legacy constructors use None, transformations preserve font, and shaped comparison/join discriminates face/axes. The shaper owner's emission provides effective-font clone also on cache hit, without changing semantics/style to induce selection. Collectors/subset/CMap/PDF bind the same original face with style coordinates; PNG/SVG prepare local bindings and instantiate once per face/request before render, keeping original resource index distinct from instantiated bytes. None retains legacy resolution; existing public API/FontKey remain. No artificial fonts, unsafe, globals or new phase. Private functions may receive explicit font/bindings.

Shared shaped-face-identity invariants govern these cross-owner boundaries: associate original face bytes plus axis request rather than name/slot. Export-local tables preserve supplied resource order and append missing faces; multifont distributes items once by binding before per-font collection, never sharing global GIDs/CMap across faces. This owner remains 1:1 and materializes only its own references; the shared nucleus owns common claims.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. In Group destructure/recompose paths, capture paint_root and reinsert unchanged. Never redefine true as false during projection, marker removal or rebasing.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection. Shaped-face acceptance requires RED→GREEN and bilateral miss/hit/VF controls, without promised timing gain.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.
