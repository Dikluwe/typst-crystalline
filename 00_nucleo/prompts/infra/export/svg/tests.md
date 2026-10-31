# L0 — svg/tests
Hash do Código: 8e1c6a21

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/tests.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Regression suite ownership

Own the existing cfg(test) SVG suite, fixtures, names/assertions, supplied fonts and
small resvg rasterizations. Exercise public output and private helpers via existing
restricted access. Expected productive behavior belongs to the specialized owners;
this suite does not redeclare paint/clip/context algorithms. Check morphology and
graph/reference closure; byte assertions are local regressions, not new vanilla parity.

## Ratified bidi and shaped-face fixtures

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
This unit's before-source/syntax-nodes anchors: [855, 912, 920, 1463]. layout_types.rs:531–539
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

## Nominal hyperlink interaction boundary regression

The before measurement and source/L0 state in `lab/results/remaining-parity-2026-10-22/agents/repeat/link-overflow-source-before.json` establish an actually overflowing painted child that incorrectly inherits the outer URL in emitted SVG. Exercise the exporter through a real Page/Link/translated Group/opaque Shape graph: nominal anchor bounds are(8,8,10,10), while the child's actual paint is at(28,8)–(33,13). Verify that only the nominal hit rectangle descends from that anchor, and that the unchanged painted child remains present before it. This tests hyperlink activation ancestry, rather than declaring arbitrary XML order a language requirement. Productive behavior remains owned exclusively by `00_nucleo/prompts/infra/export/svg/navigation.md`. Preserve prior target/transform/decorative and font tests; no fixture or observer weakening.

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
