# L0 — stream/draw
Hash do Código: 15b337b4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:3d2d44a53e9fb6a6c8d59ddee7b362412ae0ab05fac0e37e50ddf731d987c132
- 00_nucleo/prompts/_nuclei/pdf-group-local-space.toml sha256:94c2f27db514ce909100e929a459524e08ebeb09583f4a39173f53f457c770cd
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:afd5a809b6e3f2ac1f9a752fb1f6c6d704139eb1f71b085d430d9417f826b04e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff



**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/draw.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/draw.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Recursive painting

Own exhaustive static draw_item_top/draw_item_local dispatch and recursive handling
of Text/TextShaped/Glyph, Line/Image/Shape/Group/Link/Semantic. Preserve paint order.
Top-level uses page_height - y; top-level Link children use that same convention.
Group applies position/matrix and clip once, then uses local drawing. Local text
uses the existing reflection envelope q / 1 0 0 -1 0 0 cm / Q with (x,-y), restoring
the text helpers' coordinate precondition; line/shape/image keep their own conventions.
Images compose resolved size/position with EXIF 1-8 once at the existing precision.

With tags disabled, Semantic is visually transparent. With tags enabled, draw
Formula/Lbl/TH/TD from supplied MCIDs and balance BDC/EMC; alt is never painted.
Artifact Other uses BMC. Header emits `/Artifact << /Attached [/Top] /Subtype /Header
/Type /Pagination >> BDC`; Background emits its existing Type Background property
list `/Artifact << /Type /Background >> BDC`. Other artifact variants retain BMC. These branches do not select properties
by PDF version and consume no MCID. Do not manufacture a StructElem here. In UA1, meaningful leaves
use Span/Figure while geometric decorations use Artifact Layout. Boundary carriers
paint no empty content. Recurse through each child exactly once.
Delegate text and geometry to their children; resource IDs and ParentTree belong
outside this owner. Unsupported artifact-version/AT behavior remains unproven.

## Verification

Check top/local coordinate composition, nested reflection, clips, asymmetric EXIF
images, balanced semantic/artifact envelopes and UA1 boundary/leaf behavior.

## Effective image clipping

Bilateral image.typ with1×1 JPEG/80×80pt target has identical extracted XObject bytes but321 differing right/bottom-edge pixels at144dpi. Historical pinned vanilla emits `80 0 0 80 8 40 cm` without clip; Tekt precedes it with `8 40 80 80 re W n` at draw.rs:284–293. L1 retains cover clip_rect even when transformed image exactly fits. Redundant-clip cause is refuted by unchanged raster or real-crop regression.

Omit Image clip only when the transformed image rectangle is entirely contained in clip_rect. Any exceeding edge retains clipping, with exact point geometry and no epsilon erasing small crops. Same top/local decision; preserve EXIF matrix/position/resources/alt/tags/layout/public carrier/default fit/phase/JPEG bytes. Require bilateral no-crop raster, positive cover-crop test and EXIF controls.

## Text identity dispatch

RED source→PDF probes/handler audit in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` preceded layout_types public approval 2026-09-28. Delegate TextSeparator to `00_nucleo/prompts/infra/export/stream/separator.md` in top/local coordinates with existing local reflection, no added advance.

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` measures eight U+002D/SHY equal-geometry controls; approved-before/owner confirmation precedes this change. Delegate ActualText opening/closing to `00_nucleo/prompts/infra/export/stream/text_replacement.md`, paint children with same dispatcher, no envelope MCID/new resource/advance/algorithm. Preserve author text/order/coordinates/calculation; require text/boxes/causal/recursive/literal controls, not byte/new-selection claims.



`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/envelope-probe/` refutes outer scope: EMC before Q preserves boxes, after Q loses Poppler position; opening before/after cm/Tm does not change this. No codepoint-count closure or Q-text search/rewrite. Keep existing signatures and offer private deferred graphic-tail variants: emit text/ET, return whether Q is pending without restoring. Type1 considers only faux-bold q; Verbose embedded fonts have own q; bitmap no textual tail. Draw requests deferred form only for TextReplacement's last visual child, then closes leaf marks→ActualText→pending Q. Earlier/ordinary children retain emission. Modes keep operators/transforms with no mode switch/operator-text heuristic. text owns formatting/tail, draw dispatch/leaf marks, `00_nucleo/prompts/infra/export/stream/text_replacement.md` UTF-16/BDC/EMC. Private parity, no public API/phase/flag; require original-position top/local/both-mode extraction, literal coexistence/balanced scopes/raster.

math-large-identidade-2026-09-30 uses historical pinned vanilla/NewCMMath DisplayOperatorMinHeight3000du. Wrapping three ∫ pieces outside graphics states extracts the character but Verbose Poppler box has zero height and -layout places it before∑; Compact also reverses order. Temporary-PDF experiment with identical pieces/positions/GIDs refutes outer scope: BDC after first-piece BT and EMC before final-piece ET restores positive bbox and∑, ∫ order in both modes. Never infer text from GIDs. Internal marked Glyph variants accept causal opening/closing from draw and fully reuse normal selection/remap/color/position. Only TextReplacement-piece BDC/EMC insertion points differ. Keep ordinary Glyph format/default/Type1, no invisible text. Marking encoding remains text_replacement's ownership.

Draw keeps exactly one marked scope when TextReplacement has only positioned Glyph children: its causal text opens after first BT/closes before last ET. Paint each once with original GID/position/style and applicable leaf marks/MCIDs. Other replacements, including Text/TextShaped discretionary hyphens, keep generic route. Test same three-part∫ through simple/-layout extraction/positive boxes/raster in both modes; an ActualText token alone is insufficient. No GID heuristic/post-stream scan.

## Ratified bidi and effective face


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

## Certified terminal literal-space boundary


### Evidence

`lab/results/construction/diagnostics/espaco-terminal-2026-10-06/before.json`
retains working tree/HEAD/code/L0/binaries. literal-terminal-space in the bidi-
identity batch positions glyphs correctly, but extracted 123 bbox ends at
121.49581pt versus oracle 125.000002pt. PDF emits the space carrier at 121.50342pt;
separator.rs:10–14 emits U+0020, which Poppler drops at run end. Historical vanilla
9f2b6e871 uses ActualText 0020202C to close the export embedding. pdf-boundary-
probe.json shows 0020 alone fails while 0020202C retains bbox to 125.00142pt without
glyph/advance changes. word-envelope-refutation.json refutes wrapping preceding
text plus space across graphic restorations: extracted line changes. These are
observable text identity/geometry, not byte/operator equality requirements. Lost/
duplicated spaces, changed ink or markup/LTR-base regression refutes the correction.

### Contract and acceptance

Internal ADR-0127: no public field/API/phase/flag/dependency/unsafe addition.
U+202C is derived only in PDF ActualText to close the embedding certified by span;
it is not author content/carrier/glyph/bidi input/global ToUnicode. No virtual
control in author representation; preserve the paragraph-bidi-origin L0 intact.

Private emit receives explicit origin and optional BidiSpan. Certify only Literal
with exact U+0020 span at the end of a nonempty Text segment with preceding prefix,
LTR segment direction and RTL input base. Use original segment/range. None,
markup/newline/other ranges/directions and Space without Text segment retain old
emission; never infer boundary from distance/font/visualization.

For the certificate, open ActualText space+PDF, emit original U+0020 exactly once
through text helpers with deferred graphic restoration, close EMC before pending
Q. Compact/Verbose preserve identity through their own helpers. Font/x/baseline/
advance remain unchanged; no document mutation/previous-text clone/reframing.
Positive/negative certificate and EMC-before-Q tests plus raw PDF extraction/bbox/
raster/top/local/markup/LTR controls are required.
For this draw owner, both TextSeparator dispatches pass origin and bidi.as_ref() to the separator owner, preserving x/PDF baseline/local reflection. Certification/ActualText helpers stay with separator; do not certify here or insert virtual marks/text into document. Body paints once with unchanged streams/scopes/tagging.

## Nested affine groups and clip scope

`lab/results/construction/diagnostics/grupos-aninhados-2026-10-13/before.json.gz` preserves prior source/L0/HEAD/tree/status/diffstat/UTC/binary. draw.rs:530–565 applies page Group position/matrix/clip; :947–952 local path discards all three, contrary to existing contract. Historical export/tests/mod.rs:8306 checks text presence only. Independent conic-cobertura-2026-10-13/probe.json.gz initiated investigation; Pattern placement is a distinct open defect here.

Local Group saves graphics state, concatenates affine matrix with pos+(tx,ty), clips full inner-size geometry via emit_shape_path_local, visits children/restores. Page already established Y-down: no second reflection, bbox substitution or lost rotation/reflection/scale/shear. Clip stays inside q/Q, no sibling leak. Preserve order/contextual bbox/semantic-text envelopes/page dispatcher. Internal omission repair, no public field/signature/mode/flag/phase/default. Before code independent-operator-interpreter RED tests check transformed points/arithmetic, clip-before-paint, sibling restore, Compact/Verbose, noncommuting two-level scale/rotate/position; GREEN unchanged. Infra/build/fmt/lineage; cm/text presence/test count proves no geometry. first-validation.json.gz records test interpreter return tuple type_complexity at:1246: use private test struct points/rectangles with identical operations/fixtures/expectations/coverage, no suppression/productive type change.

## Unpainted paths and conic occurrence transport

block-regiao-2026-10-14/open-pdf/results.json.gz shows extra blue Block(outset:4pt) backing without fill/stroke only in local PDF, visually confirmed. `lab/results/construction/diagnostics/pdf-caminho-vazio-2026-10-14/before-full/results.json.gz` preserves HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty status/diffstat/UTC/source/commands/binaries. Invisible Rect/Ellipse/RoundedRect then blue marker paints from(20,20) locally in both modes; local SVG/historical upstream 9f2b6e8715237cb086899a42873660fe744622e8 paints marker only(70,60). draw.rs:513–517,922–926 builds a path then emits nothing for absent fill/stroke before Q; pending-path reuse is an inference refuted by unchanged subsequent paint/clip. polygon(stroke:none) fails preexport and stays separate, no parity credit/intent/byte inference.

Top/local dispatch must terminate without painting a constructed Shape path when both paints are absent, keeping graphics/semantic envelopes/order/resources/tags/transforms/primitives/real fill-stroke paths. Do not contaminate next paint/mask even after Group, discard documents, turn absence black or alter producer layout. Independent local PDF path-state interpreter tests painted/clipped rectangle geometry, not exact string/operator if another implementation meets geometry. Cover top/local/Group escape/both modes/fill/stroke/both/later clip; bilateral PDF/SVG/positive/no-paint/qpdf/regressions/full suites/build/doc/fmt/check/inventories/scoped lineage. Internal parity no public field/API/default/flag/dependency/phase/unsafe. Block outset/overflow, underline thickness and polygon None stay their own owners/open evidence.

For conic occurrence lookup, gradients/mod.rs:224–264 loses RGB/CMYK placement, builder/resources.rs:564–574 identity matrix, draw.rs:522–555,939–965 composes groups while Pattern is initial-stream-relative, page.rs:45–49 adds bleed. red-vs-conic(red,red) before/probe and `lab/results/construction/diagnostics/conic-posicionamento-2026-10-14/placement-before.json.gz` preserve source/L0/HEAD/UTC/status/diffstat. Pass current Shape FrameItem reference to top/local fill/stroke helpers; preserve coordinates/order/group transforms/clips, Pattern matrix prepared elsewhere. Require position/size/reused-Arc/page/fill-stroke/nested-affine/bleed/parent/resource/both-mode RED→GREEN and full uncropped raster. Positioning alone does not complete author-center/corner/CMYK-offset geometry; conic-geometria-amostragem owns completion and no inscribed-circle certification follows here.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [523,934,1299,1314,1353,1361,1445,1497]. layout_types.rs:531–539
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

paint-root-2026-10-14/first-render-delta.json and after-first/results.json.gz preserve block-auto-gradient-parent/PDF regression:3840 pixels differ>16 from baseline/increased oracle distance, other 35 local renders unchanged. Legacy collector replaces parent_bbox_override per Group; new root envelope exposes lack of affine frame while Shape paints remain declarative. This is an open failure, never accepted reference/tolerance. Prepared/native integration must close cause; isolated metadata transport proves no render/production parity. Retain failing sample/matching controls and upstream quarter PDF divergence.

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

Migrate FrameItem::Shape fixtures to FramePaint/FrameStroke with unchanged values/
semantic assertions. Tiling explicitly constructs a prepared cell; eval fixtures
remain declarative. No total productive conversion or weakened assertion.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Solid opacity scope


`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserves source/owner/HEAD/UTC/tree. tiling-nativo-2026-10-14/final-renders and remaining-analysis.md show cell8×8/pitch4×4/alpha50%: PDF opaque red, SVG one contribution, PNG four(255,15,15). pdf_tiling.rs:184 uses BBox=size, svg/paints.rs:148 overflow visible, shape.rs:155/text.rs:275 drop alpha. Upstream also clips overlap and is not a requirement for that defect. The explicit-period source-over hypothesis must preserve no-overlap controls and analytical four-contribution composition.

Open q/ExtGState only for translucent leaves and close after each without contaminating the next. Top/local retain ActualText: private pending-Q count includes this extra envelope and is discharged only after EMC, never operator-text scanning. Apply also to marked glyphs taking the direct path. Keep transforms/opaque markings. This is internal within the approved tiling package, no new public API/phase/flag/default. L0 first/RED→GREEN, analytical composition including distinct-color order, simple/nested/rotated/text/image and extraction/page-selection controls; workspace/build/fmt/Rustdoc/lineage. Variable-gradient alpha/PNG quality remain incomplete, so no production certification with these open.

## Proven local Rust expression changes

ci-helpers-testes-2026-10-14/before.json.gz/references-before.json.gz preserves source/owner/HEAD/tree/UTC/diffstat/call/module/import audit/prior suite hashes/pre-edit names at draw.rs:324,325,983,984. Remove only unused intrinsic_width/intrinsic_height Image bindings from top/local patterns, covered by existing `..`; retain carrier fields/resolved dimensions/position/orientation/clip without pixel-based recomputation/resource/matrix/branch changes. Audit calls/compilation, never dead_code alone. avisos-clippy-tekt-2026-10-15/before.json.gz and selected-red.json preserve source/owner/HEAD/tree/UTC/diffstat/RED at:647, 1205: pass already borrowed opening directly to push_str in both Semantic dispatches, with identical envelope/text/order/balance/UA1/top-local behavior. Internal continuous flow, no public/default/phase/dependency/suppression, L0 first/scoped lineage, selected RED→GREEN, unchanged names/assertions/full lib-all-target build/suites/Clippy/Rustdoc/fmt/lineage/inventory. No cfg/binding/syntax mirror tests or new parity claim.

## Exhaustive existing FrameItem decisions


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace ua1_leaf_opening's `_` at draw.rs:114 (historical F057) with FrameItem::TextReplacement/Semantic/Line/Shape/Group/Link.
Preserve complete fallback bodies/prior arms/guards/transforms/calls/returns/order.
Callbacks/effects/metadata/clipping/fonts/positions/ink/width follow current owners;
no expanded traversal/wrapper-leaf change/text-or-ActualText geometry replacement.
Tests retain fixtures/expectations/tolerances/counts/Unknown failures; V19 group
notes stay visible without pretending one arm covers all alternatives.

Continuous ADR-0127 adds no enum/field/trait method/public signature/default/
dependency/compatibility/phase and approves no historical pending proposal.
L0 first, scoped bidirectional resealing; no suppression/higher threshold or
match→if-let evasion. Before code run new FrameVisitor control/workspace suite
with unchanged expectations and confirm AST RED for all 70 family warnings.
Afterward zero scoped-family V16/no new warnings; outside V16 remains pending.
For productive core matches reached by cargo check -p typst-core, use a disposable
FrameItem sentinel variant to observe absent E0004 before/present after. That
probe covers only compiled sites, never unbuilt cfg(test) or blocked L3 sites.
For the full batch replay patterns/guards/bodies, verify enum/signatures unchanged,
real build/tests, no stub bodies.

Revalidate full workspace suite/cargo build/strict workspace-all-target Clippy
(-D warnings, -D unfulfilled_lint_expectations)/fmt/Rustdoc -D warnings/four HTML
indexes/strict V1/V5/V15/V26/reverse lineage/SARIF/diff --check. Freeze sources/L0/
configuration during runs; heavy commands sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1. Structural cleanup is not new global parity,
memory gain or measured AI benefit.
