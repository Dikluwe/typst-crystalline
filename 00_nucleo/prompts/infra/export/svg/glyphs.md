# L0 — svg/glyphs
Hash do Código: 88c4c45a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/glyphs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417

## Consumer evidence

`03_infra/src/export/svg/glyphs.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font context and glyph definitions

Own GlyphFontRequest/SvgGlyphFontContext, immutable request normalization/lookup,
GlyphDefs, outline extraction/cache/path builder, direct/shaped glyph painting and
the existing HTML textual writer. Follow the pinned glyph-font-context nucleus;
its shared resolution invariants are not redeclared by the façade.

Use the resolved FontKey's supplied bytes for glyph_id, verifying the key exists
in the collection. Deduplicate definitions by font identity and GID. Apply size /
units_per_em, position and Y inversion once; preserve shaped advances/offsets,
fill/alpha and supplied styles without reshaping or moving the layout cursor.
Keep OutlineBuilder methods as trait methods and current visibility.
Absent/ambiguous mapping, invalid face, zero units_per_em or missing outline remains
Unknown with the existing fallback, not a fabricated glyph. Fontless wrappers do
not synthesize text/outlines or probe incidental/system fonts or network.

## Verification

Exercise reordered fonts, same-GID distinct fonts, mathematical assemblies,
fill/alpha, transforms and missing/invalid/fontless contexts.


## Immutable coordinate and effective outline inputs

### Evidence

`avisos-clippy-tekt-2026-10-15/before.json.gz` retains sources/owners/HEAD/tree/UTC/
diffstat; selected-red.json records glyphs.rs:319 mutable y without reassignment.
This is equivalent Rust expression maintenance, no new parity. Preserve y value/
offsets/advances/Y inversion/fill/definition identity, make only y immutable.
Continuous ADR-0127: no public/default/phase, suppression or syntax-mirror tests;
selected GREEN, existing suite/build/all-target Clippy/fmt/doc/lineage/scoped seals.

`too-many-arguments-2026-10-15/before.json.gz`/classification.json retains full
source/prior L0/signatures/hashes/UTC/status/diffstat, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, dirty tree captured
2026-10-15T16:39:18.873477+00:00, reconfirmed spans/contracts. render_text_shaped
:286 has nine arguments (threshold seven), historical C066; outline-only leaf
ignores _text, HTML textual route is chosen earlier with separate writer. This
is private structure, no functional-defect/new parity claim.
clippy-aridade-interna-2026-10-16/before.json.gz retains checked sources/callers/L0/
spans/28-warning inventory/hashes/HEAD/tree/UTC/diffstat. It materializes the six
named corrections, superseding earlier pending status only for them.

### Current private correction and acceptance

Remove only _text from pub(super) outline helper and its dispatcher argument,
keeping eight effective face/outline/scale/definition inputs and unchanged body.
Then only this leaf may use
`#[expect(clippy::too_many_arguments, reason="C066: Leaf SVG usa oito inputs efetivos de contorno, face, escala e definições; texto HTML segue writer separado")]`.
Never remove text from FrameItem/public API/HTML writer; preserve upem==0/fallback/
GIDs/defs/scale/offsets/advances. Residual expectation depends on dead-argument
removal, no unconditional suppression. Item/lint only, no module/impl/crate/global
threshold, other lint or unfulfilled suppression; remove/review if no longer needed.

Continuous ADR-0127 no external public/default/phase/dependency/semantics change.
New carriers/constructors private; generic bans are superseded only for the two
historical conditional C047/C066 residuals after _text removal. Other five correction
arities remain visible. L0 first/caller-L0 before code/scoped seals. Public/external
compatibility needs fresh concrete approval. Real selected RED before code/GREEN
after, preserve all functional fixtures/expectations/tolerances and values/calls/
PNG-SVG-PDF/preference/diagnostic/resource/MCID behavior, no global mutation or
carrier-mirror tests. Full workspace suite/build/all-target Clippy/unfulfilled
expectations/Rustdoc/fmt/strict+reverse lineage under12GiB/zero swap/one job.


## R10 — source-preserving raster glyph fallback

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,bitmap-evidence.json,font-tables.json}` binds received R9 code/dirty stat/UTC and fixed bd5ebec2 binaries. Nimbus/Liberation mixed123☃ selects Noto Color Emoji. The received PDF raster route emits only image Do, losing the source character in extraction; SVG glyphs.rs:100-105 requires an outline and emits no raster. Pinned paired PDFs/SVGs preserve source text and the PNG. Font-family names are evidence, never runtime branches; supported actual raster data authenticates the correction. Source character, physical position/advance, PNG pixels and geometry are language observations. Object numbering, Type3 versus CID structures and byte encoding are mechanics.

Classification: private font/export parity correction under ADR-0127 continuous flow, with no new public method/entity/phase/mode/dependency. The previously approved FontMetrics method remains separately bounded. No invisible duplicate source text, fake outline, font-name coefficient, or oracle-byte copying. Build a1016-byte self-authored portable bitmap font fixture from the preserved fontTools generator; it is test data, not a font copied from the oracle or a productive dependency. Authenticate its bytes explicitly in the receipt.

### Exclusive owning obligation

Extend only authenticated missing-outline PNG raster glyph definitions. Extract actual raster data/bearings/ppem from supplied font bytes, retain identity key by font/GID/scale, and embed PNG data in the definition with a single scale/translation and Y inversion coherent with existing shaped offsets. Do not lose advance on absent/unsupported data; preserve current outline/cache/font binding behavior. No system font lookup. Require portable missing-outline raster RED→GREEN and actual paired image geometry/pixels/complete text-source controls.
Require module RED→GREEN before implementation, fixed-pin actual fallback PDF/SVG paint/text/geometry controls, previous A–H/R1–R9, full workspace/lineage/resource gates and byte preservation of received unrelated inputs.


## R10 — fractional tracking paint correction

Measurement before decision: candidate3-build/receipt.json/source.json.gz and
candidate3-matrix-svg/receipt.json under the R10 evidence root bind exact source,
dirty state, UTC and binary9ac2dcfaf0bee19a94317e1f09da8861d21fd52ede9f791934fc899b3debf071.
All69 script PDF controls and the R8/R9 corpus pass. Twelve signed-tracking SVG
controls fail by0.003125–0.016pt. render_text_shaped advances x using only stored
integer font-unit advances, whereas PDF stream/text and shaper/tracking::width
already add the authoritative tracking-owner residual. This loses an actual
fractional interval, including in the ordinary Alpha prefix, not a font-name or
PDF-byte discrepancy. These are physical glyph positions (language/render),
not a required serializer algorithm. No tolerance or test input may change.

Delegate fractional per-glyph advance compensation to the existing
crate::shaper::tracking::residual in both successful and unsupported-definition
advances. Preserve same-cluster suppression, signed values, terminal boundaries,
font/scale/position/raster identities and no tracking duplication. Require a
portable actual-SVG coordinate RED with different clusters sharing a glyph run,
signed values and paint sizes, plus all69 script SVG/PDF and original R8/R9,
full workspace/Clippy/fmt/Rustdoc/lineage/resource gates. This is internal parity,
no new public/default/phase or formula copied away from its owning unit.


### R10 — Clippy test construction correction

Measurement before decision: validation-complete2/clippy.log under the R10
evidence root rejects glyphs.rs:513 with regex_creation_in_loops. All tests
passed; the immutable SVG-use parser is rebuilt for each size/tracking pair.
clippy-correction.json and before-clippy-source.json.gz bind the exact state
and UTC. This is test construction mechanics, not a product parity change.
Construct that identical parser once before both loops; retain every input,
parsed coordinate, assertion and tolerance. Require the focused test, denied
warning Clippy and full frozen validation again; no lint suppression.

## Variable outline geometry — evidence before decision

physical11-build.json/source.json.gz binds exact source/dirty HEAD212aed0449,
UTC/diffstat and binary. physical11-optical-visual/receipt.json records original
font assets and source; comparison.json keeps variable17 SVG20575/5709 pixels
negative despite PDF/word boxes matching. For its A at17pt, native first quadratic
control delta is0.8925pt while product is0.901pt; subsequent coordinates also
differ. This differs from the tiny serif11 case, whose110 ordered outlines and
origins match within2.842170943040401e-14pt (source-geometry inspection only).

font_binding.rs:159-175 materializes each complete FontKey into parallel prepared
bytes. glyphs.rs:106 then outlines those prepared bytes. Thus the earlier
missing-axis inference was incomplete: axes are already applied at preparation.
Hypothesis: static-program materialization quantizes variable outline geometry
before SVG extraction. An actual exported contour equal to an independently
configured original face at its effective coordinates would refute this cause.
Require module REDs using original UbuntuSans-Variable.ttf,400/700 and explicit
wght override, both shaped and mapped direct Glyph plus static/raster controls.
The reference configures the font directly; it must not call the resource
instancer or the proposed original-view accessor. Compare actual referenced
contour geometry, not defs ids/order/XML spelling as a vanilla oracle.

Bounded correction, only after RED: expose a private original FontKey view through
the same complete-request/original-face lookup. SVG outlines read that original
face and apply the key's effective variant plus explicit axes in RAM before
outline extraction; the per-export index still denotes the complete instance.
Keep legacy prepared resource view/instancing for its existing consumers; no
World lookup, reshaping, cursor/position/advance/fill mutation or source effects.
This supersedes only SVG outline readers' requirement to return materialized
bytes. Preserve original/prepared association/order and missing/ambiguous None.
No public API, flag/default mode, compiler phase/order or dependency changes.
Under ADR-0127 this is an internal render parity correction with L0 first and
scoped resealing. Require RED→GREEN, actual PDF/SVG/PNG and all historical
controls,12 warning-strict gates with a fresh documentation output directory
if necessary, resources and benchmark gates unchanged. No small-AA closure or
normalization follows from fixing the variable contours.

svg-variable-red.json/source.json.gz confirms three actual exported-contour
failures (shaped700, mixed400/700/explicit700 and direct Glyph700) with unchanged
inputs; static inapplicable-axis control passes. Font asset hashes are retained
in svg-variable-red-assets.json. Implement the bounded original-face view and
effective-coordinate extraction now. Retain the materialized get/fonts view for
the PNG renderer and existing users; SVG's separate original view changes no
public signature, compiler phase or font selection. The original table index
remains complete variant/axes identity, so the existing per-export GID/scale
cache key remains valid. Preserve missing/ambiguous behavior and raster fallback.
