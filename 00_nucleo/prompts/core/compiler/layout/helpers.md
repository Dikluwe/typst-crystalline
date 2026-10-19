# L0 — helpers
Hash do Código: 03a04b58

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/helpers.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/helpers.rs:18`, `01_core/src/compiler/layout/helpers.rs:54`, `01_core/src/compiler/layout/helpers.rs:94`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

`01_core/src/compiler/layout/helpers.rs:412` measure_content handles Shape/Sequence with zero fallback otherwise. Inspection finds the projection gap demonstrated by restricted show_grid320 in `lab/results/construction/diagnostics/dupla-avaliacao-auditoria-2026-10-05/geometry-measurement-red.json.gz`. Applying it to legacy measurement is a structural inference refutable by a generated-shape-under-transport test.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distinguishes regular GIDs from Oblique selection. The human confirmed public TextShaped.font: Option<Font> in `00_nucleo/prompts/core/entities/layout_types.md` (SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`). This round's snapshot/inventory are in `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized branches contain the corresponding transparent operation; Source must not leak as observable content. Human approval `confirmo` was received 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

`01_core/src/compiler/layout/helpers.rs:119` has Clippy inventory items. Sources/owners/references/UTC/status/diffstat at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, are in `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. Usage search/productive paths precede classification; no new parity/performance proof.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [260]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the existing pure FrameItem position/extent/translation/offset/path-slice helpers, the narrow legacy shape/sequence measurement fallback, heading-scale mapping and dimension projection. For a Semantic wrapper, horizontal position is the minimum non-marker child x and width is rightmost child end minus that same left edge; empty/nonvisual extent is zero. Keep existing vertical conventions and Group/Link behavior, without child reorder or automatic render semantics.

Exclude direct private context markers from visual right/bottom extents but preserve their point under all translation/offset helpers. Descendant path-slice traversal supports Group, Link and Semantic, rejects invalid/overflowing paths, and does not create geometry. Heading scales remain level one 1.4, level two 1.2, other levels 1.0.

Retain the source's existing translation/projection conventions rather than claiming fully general affine bounds or clipping: Semantic translation uses its first-child anchor; item_bottom_y retains its descender approximation; Image reconstruction preserves and translates clip_rect as specified below. Those limitations require a separate behavior decision, not silent cleanup repairs.

Transport TextSeparator position, consumed advance, style and origin without new paint/baseline/advance; preserve child order. Use advance only in horizontal transport; exclude separators from visual right/bottom bounds.

Image translation preserves the clip rectangle and moves its x/y by the same delta as the image position, retaining width/height. Reconstruction must not discard cover clipping.

The dedicated cover_clip_tracks_every_translation_without_changing_extent refutator also fails after reconstruction because offset_frame_item and shift_frame_item_x/y move only the image, leaving its clip behind (`helpers.rs:285`, `helpers.rs:315`, `helpers.rs:431`). All three helpers must transport the clip by the same respective delta, including recursive Semantic/Link paths; extent remains invariant. RED is recorded in the correction diagnostic.

TextReplacement position/width/height/shifts/offset/translate descend into children, reconstructing unchanged text. Test translation without identity change and resource visitor versus extraction. The shared nucleus does not transfer ownership.

Project Realized to visual and Located/ShowRecipe to body before legacy measurement. Do not measure consumed original, apply show or change identity. Preserve other-arm approximations. Existing transparent ShowRecipe.body delegation also accepts Source.body without separate origin execution, public carrier or assigned Location; preserve this unit's classification/preparation/serialization.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Migrate only this owner's shaped-face references: legacy constructors use None, transformations preserve font, and shaped comparison/join discriminates face/axes. The shaper owner's emission provides effective-font clone also on cache hit, without changing semantics/style to induce selection. Collectors/subset/CMap/PDF bind the same original face with style coordinates; PNG/SVG prepare local bindings and instantiate once per face/request before render, keeping original resource index distinct from instantiated bytes. None retains legacy resolution; existing public API/FontKey remain. No artificial fonts, unsafe, globals or new phase. Private functions may receive explicit font/bindings.

Shared shaped-face-identity invariants govern these cross-owner boundaries: associate original face bytes plus axis request rather than name/slot. Export-local tables preserve supplied resource order and append missing faces; multifont distributes items once by binding before per-font collection, never sharing global GIDs/CMap across faces. This owner remains 1:1 and materializes only its own references; the shared nucleus owns common claims.

Compile internal line_content_bottom only under #[cfg(test)], preserving signature/body/recursion for existing separator-exclusion test; other geometry helpers remain productive. Preserve productive routes, existing test names/bodies, external APIs/defaults/phase.

When reconstructing Group in translate_frame_item, transport paint_root without deriving it from clip/matrix. Shifts/offset preserve metadata in local groups, with identical dimensions/order.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection. Translation/legacy projection require RED→GREEN with observable generated-shape dimensions and no new signature/phase (ADR-0127). Shaped-face controls include miss/hit/VF without promised timing gain.

Preserve exclusive ownership/static dispatch without I/O or a new signature/trait/flag or eval↔layout phase. Source-origin tests/C2/C/macro collections/builds/lineage are required before certification.

Revalidate build/check for every workspace target, core suite, test inventory, Clippy and lineage; do not remove expectations to pass.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.


## Batch D V16 measurement (2026-10-19)

Evidence precedes the decisions below. The exact HEAD, uncommitted file list,
UTC, source bytes, argv, source/output/binary SHA-256 and pinned oracle recipe
are recorded in `lab/results/v16-corrections-2026-10-19/batch-d/agents/measurement/probes/png-before.json`.
The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
SVG geometry is evidence of occupied layout space, not a byte-equality contract.
These observations establish behavior; they do not infer upstream intent.
`transform.rs:45-67` first lays out the body and calls this narrow helper only
when both produced width and height are zero. `helpers.rs:450-483` projects
Source/ShowRecipe/Located bodies and Realized.visual, measures Shape/Sequence,
and returns zero otherwise. The pinned shape/empty/metadata transform sources
compile in both products; shape extent exists and empty/metadata do not paint.

### Decision and refuters

Retain this bounded fallback, with exhaustive nominal Content coverage at its
existing match. Empty and nonvisual markers are genuine zero contributions;
text, math, image, other containers and unknown visual families are unsupported
approximations here, not assertions that their language extent is zero. The
real-layout-first caller is the reason this is not a general measurement API.
Keep positive transport branches and original-consumption semantics unchanged.
A source transform whose body paints but reaches this fallback and loses its
extent refutes this classification and requires a causal repair. Preserve the
existing narrow approximation until that evidence exists; do not rerun recipes,
callbacks or allocate a location in this helper. Future Content variants must
force a decision at this exact match. ADR-0127 continuous internal flow applies.
