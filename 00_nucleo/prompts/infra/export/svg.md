# L0 — svg
Hash do Código: 066b77c6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:199640afc0a7243ca509288840d9c9e0c431f850f67919148fc8dac3b1cb33a0
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130


## Consumer evidence

`03_infra/src/export/svg.rs:40` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own SvgOptions, default pretty=false/render_bleed=false, FontKey/GlyphFontIdentity
aliases, compatibility reexports and all existing standalone/context/font/HTML-frame
entrypoints and methods. Preserve signatures and private context fields. The pinned
context nuclei define the shared identity/transport invariants; specialized context
implementation belongs to glyphs and navigation.

Own the root XmlWriter, page canvas/viewBox and bleed translation. Standalone without
render_bleed crops to the logical page; with it use the physical canvas. Auto page
fill is white and None transparent. Paint background/body/foreground in order and
emit definitions through child writers. Keep exhaustive static FrameItem dispatch;
Semantic is visually transparent and never paints alt or invents accessibility.

The HTML-frame variant consumes the supplied Page with the same canvas, fill and
layer rules; it does not independently remove backgrounds or convert dimensions to
em. Its root style is `display: block; overflow: visible`. Intrinsic transparent
frame construction belongs to the HTML layout caller, not this wrapper. Reuse child
writers, including the HTML textual path, without textual paint/palette substitution
or duplicating SVG in HTML.
Pretty selects writer indentation only; do not alter the graph or child algorithms.
Bundle/file routing stays outside this page exporter. No filesystem or new layout.

## Delegated owners

- `00_nucleo/prompts/infra/export/svg/geometry.md` owns `03_infra/src/export/svg/geometry.rs`.
- `00_nucleo/prompts/infra/export/svg/glyphs.md` owns `03_infra/src/export/svg/glyphs.rs`.
- `00_nucleo/prompts/infra/export/svg/groups.md` owns `03_infra/src/export/svg/groups.rs`.
- `00_nucleo/prompts/infra/export/svg/images.md` owns `03_infra/src/export/svg/images.rs`.
- `00_nucleo/prompts/infra/export/svg/navigation.md` owns `03_infra/src/export/svg/navigation.rs`.
- `00_nucleo/prompts/infra/export/svg/pretty_serialization_tests.md` owns `03_infra/src/export/svg/pretty_serialization_tests.rs`.
- `00_nucleo/prompts/infra/export/svg/paints.md` owns `03_infra/src/export/svg/paints.rs`.
- `00_nucleo/prompts/infra/export/svg/serialization.md` owns `03_infra/src/export/svg/serialization.rs`.
- `00_nucleo/prompts/infra/export/svg/tests.md` owns `03_infra/src/export/svg/tests.rs`.

## Text identity carriers

This FrameItem consumer was inspected before the RED source→PDF probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/`; public layout_types approval was received2026-09-28 and recorded. TextSeparator has no visual outline; HTML-frame emits U+0020 in the same text coordinate system.

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` records U+002D versus SHY with coincident geometry in eight bilateral controls; approved-before archives traversal inspection, and owner confirmed layout_types. TextReplacement is visually transparent: descend children preserving text/order/coordinates/current calculation without new algorithm/advance. Pinned text-replacement owns shared claims, not this consumer. Require text/boxes/causal/recursive/literal-hyphen controls, not PDF byte equality or a new selection policy.

## Ratified bidi and effective face binding


`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
records unit references/1:1 owners; confirmed manifesto 1ae7513e requires None for
uncertified constructors and `..` read patterns, preserving language observations
under the pinned origin nucleus without phase/API expansion.

`identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}`
distinguishes regular GIDs from Oblique selection. Human approval of public
TextShaped.font: Option<Font> is manifesto
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`;
implementation state/inventory: identidade-fonte-shaped-implementacao-2026-10-05/.

Migrate only this owner's references: legacy constructors None, transforms preserve
Font, shaped comparison/join distinguishes face/axes. The shaper supplies the actual
Font clone even on hits without semantic/style changes to force selection. Collectors/
subset/CMap/PDF bind the same original face and style coordinates. PNG/SVG prepare
local bindings and instance once per face/request before render, keeping original
resource index separate from instanced bytes; None follows legacy resolution.
Preserve public APIs/FontKey, no fabricated fonts/unsafe/globals/new phase; private
functions may receive Font/bindings explicitly.

Association uses original face bytes plus axes, never names/slots alone. Table
lifetime is export-local; preserve supplied resource order and append missing faces.
For multifont distribute items once before per-font collectors; no cross-face
GID/CMap reuse. References never share ownership. Require RED→GREEN/bilateral
miss/hit/variable-font controls without a promised speedup. Shared invariants remain
in pinned shaped-face-identity, no second productive owner.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [323]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Consume prepared Shape paints. PaintDefs owns local_to_root: paint_root Group resets base, technical Group concatenates T(pos)*matrix then restores on exit. Page/cell are roots. Render paint definitions with identical font bindings. Inside cells links/semantics are decorative, with no annotations/destinations but all visual items preserved. Other dispatch/context behavior stays.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Private SVG render session and exact maintenance scope

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` records full source/prior L0/signatures/hashes/UTC/status/diffstat at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree2026-10-15T16:39:18.873477+00:00. svg.rs:233 render_item has 9 arguments (threshold7, historical C071). Audited contracts/spans support internal maintenance, not a functional defect/new parity. `lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/before.json.gz` records sources/callers/contracts/spans/28-warning inventory; its six nominal changes removed only text from the outline-helper call while preserving author text extraction/render_html_text in html_frame. Conditional residual exceptions C047/C066 require their dead-argument removal first; the other five correction arities stay visible. No raised threshold/module/impl/crate silence. Preserve public signatures/fields/values, PNG/SVG/PDF/precedence/diagnostics/resources/MCIDs and no global mutation.

`lab/results/construction/diagnostics/clippy-aridade-final-2026-10-16/before.json.gz` and callers.txt preserve checked source/L0/HEAD/tree/UTC/status/diffstat/global hashes/workspace fixtures; MathLayouter homonyms are another engine. Inventory22 warnings/five arities. This follow-up materializes only the specified SVG session, superseding its former audit-only pending status.

Own one private borrowed SvgRenderSession per export: fonts Option<&PreparedFonts>, glyph_defs &mut GlyphDefs, paint_defs &mut PaintDefs, clip_next_id &mut usize, destinations &SvgDestinationContext, glyph_fonts &SvgGlyphFontContext, html_frame bool. render_item(xml,item,session) retains exhaustive static match, all leaves and background/body/foreground order. Groups/links/paint cells reborrow the same caches/IDs, no clones/globals/vtables/new public API. Group passes existing Size with Pt(inner_width/height), saves/restores local_to_root. Decorative Link temporarily disables html_frame for its children, restoring it for siblings. Preserve reference frames, target/navigation contexts and private call behavior.

svg.rs:47 and `lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz` preserve source/L0/hashes/HEAD/tree/UTC/diffstat and consumer/use audit. Derive existing public SvgOptions Default without changing fields or pretty=false/render_bleed=false; no added/removed Rust contract. No new lint suppression. Verify exact inventory reduction/no new warnings, selected lint RED→GREEN and denied unfulfilled_lint_expectations; existing tests retain fixtures/expectations and bbox cases require real observation, no derive/Entry/module-position mirror tests.

All maintenance uses continuous flow with unchanged external contracts/defaults/phases/compatibility/layers/static dispatch. L0 and private-caller L0 precede code, scoped lineage maintenance. New public/compatibility needs a separate concrete ADR-0127 gate. Require real too_many_arguments RED→GREEN, no new suppression/threshold changes, previous expectations fulfilled, body/value transport review and existing Grid/Table/outline/SVG/tiling/integration/call controls before/after without relaxed fixtures/tolerances or private-carrier mirror tests. Full workspace/build/Clippy all-targets/expectations/Rustdoc/fmt/strict reverse lineage/Tekt no new warnings, 12GiB/swap0/jobs1. No global parity/PDF-UA certification or unmeasured benefit.

## Owned test module declarations

This consumer declares separately owned `pretty_serialization_tests`; its name describes its responsibility. Delegate test bodies without absorbing their L0 ownership.
