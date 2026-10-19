# L0 — infra/export/render
Hash do Código: 4bb8e999

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/render.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/render.rs:40` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Raster contract

Own RenderOptions, FontKey, raster entrypoints, rendering state, recursive item
dispatch, glyph/image/shape rasterization and PNG encoding. Inputs are resolved Page
or PagedDocument values and caller-supplied fonts; outputs are in-memory PNG bytes.
Do not read files, fetch fonts, interpret CLI/PPI or run layout.

RenderOptions defaults to pixel_per_pt=2.0 and render_bleed=false. The scale is
pixels per typographic point, not PPI; the caller resolves physical units.
Use max(pixel_per_pt, 0.01), round each scaled dimension and require at least one
pixel. Without bleed use the logical page; with bleed use the physical canvas and
translate by bleed.left/top once. Auto fill is white, None retains transparency,
and Paint uses the existing to_color projection. Paint background, body and foreground
in order. The current allocation path may panic with "tamanho validado acima";
no independent maximum allocation bound is implemented here.

TextShaped renders only with a matching FontList/FontVariant/FontVariations and a
valid supplied face. Preserve glyph advances, offsets, position, scale and alpha;
the current pixglyph path uses its existing raster/outline support. Plain Text and
direct Glyph are omitted by dispatch, even with fonts; do not promise tofu or
complete mathematical glyph coverage.

Line maps Butt/Round/Square to the corresponding tiny-skia cap without changing
thickness, color, transforms or coordinates. Shape consumes resolved ShapeKind<Pt>
and all four Corners<Pt> directly, never resolving Length in a neutral context.
Preserve paths, ellipse/rounded geometry and the prepared-paint support specified below.
The shape path consumes the transported NonZero/EvenOdd fill rule and all resolved
stroke geometry: cap, join, miter limit, dash lengths and phase. Odd dash arrays
repeat once to form even pairs; LineWidth entries resolve against thickness.

Images use existing decoding/texture preparation on the resolved box. EXIF
orientation remains transported but unconsumed by this renderer. Groups compose position and matrix and intersect clip_mask with the inherited
device-space mask before recursing, using the supplied inner dimensions.
Image clip_rect intersects in the current parent coordinates before image placement.
Empty clips paint nothing; unmasked siblings retain their original state.
Link and Semantic are visually transparent containers: recurse without painting
target, kind, placement or alt. No PDF tagged structure or accessible text layer
is produced by raster.

render_document_to_png uses the existing fontless per-page route, stacks successfully
decoded pages vertically with the rounded gap and white composite canvas, and keeps
its 1x1 allocation fallback for an empty/invalid composite. Encode straight RGBA from the premultiplied canvas, preserving alpha and existing
encoding failure behavior; do not claim compressed PNG bytes as language parity.

## Verification

Check IHDR dimensions, bleed/fill/layers, supported shaped text, image orientation,
resolved radii, and deterministic line endpoint coverage. Compare visible geometry
and color/alpha, not PNG compression bytes. Unsupported direct glyphs remain a recorded limitation, not validated equivalence. Clipping must apply to every currently rendered primitive, including
manual TextShaped glyph compositing, with antialiased intersection and no state leak.

## Textual carriers and effective face

Source→PDF RED probes/handler inspection in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` preceded layout_types public approval 2026-09-28. TextSeparator paints no pixels: advance is not ink. `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` preserves eight equal-geometry U+002D/SHY controls; approved-before records traversal inspection and owner confirmation. TextReplacement is visually transparent, recursing with identical text/order/coordinates/calculation and no new algorithm/advance. Shared claims remain pinned; require text/bbox/causal/recursive/literal-hyphen controls, no byte/new-selection claim.



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

## Raster masks, stroke geometry and alpha

`lab/results/construction/diagnostics/raster-mascaras-2026-10-14/{before-source.json.gz,before-explicit-ppi/results.json.gz}` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diffstat/source/argv/binary SHA, historical oracle9f2b6e8715237cb086899a42873660fe744622e8. At explicit144PPI on120×100pt: PNG clip19200 red pixels versus3744 SVG/local+oracle; image-cover2048 versus768; oversized tiling cell14300 versus4800. render.rs:229/584 ignores clip_mask, :208 clip_rect, :443 forces Winding, :454–458 Butt/Miter without dash, :619 sends premultiplied bytes to straight ImageRgba8. The interrupted initial implicit-PPI probe generated17280×14400; do not copy that default because Tekt already resolves PPI/72. curve(stroke:none/0pt) fails before raster and remains a constructor refuter, never exporter success.

Historical typst-render/src/lib.rs:226–262 transforms clips then intersects masks; shape.rs:55–77,131–137 preserves fill-rule/stroke/dash pairs. Local tiny-skia0.12.0/mask.rs:253, 346 provides device-pixel coverage/intersection. Masks belong to canvas, not child box. Nested transforms/empty clips/glyph/image/line/unclipped siblings/EvenOdd holes/dash/cap/join/alpha recomposition refute incomplete fixes.

Private State carries Option<&Mask> with transform; mask is recursion-local/canvas-sized. Compose T(pos)*matrix before Group clip; use full path/inner dimensions, no approximate bbox. Antialiased intersection preserves earlier clips; empty/invalid clips never release child painting. Image clip_rect remains parent-relative without adding bitmap position twice. Forward same mask to Shape/Line/Image tiny-skia calls; multiply manual rasterized glyph coverage by corresponding mask value, preserving unmasked route. No sibling/frame leakage. Map FillRule/all geometric Stroke fields; resolve LineWidth against current thickness, repeat odd dash arrays, preserve phase. Encoder unpremultiplies RGB for straight PNG RGBA, alpha unchanged.

At this mask slice declarative Paint/native tiling was intentionally unchanged; the later approved native/gradient contracts below supersede only that limitation. Internal parity adds no public API/flag/default/phase. L0 first/RED→GREEN: actual clip visibility/composition/isolation/fill-rule/stroke/alpha pixels with fixed local fonts; explicit-resolution bilateral CLI PNG/SVG, infra/build/doc/fmt/lineage/inventory, no tolerance widening. Masks are a prerequisite for native tiling, not certification alone. Direct Glyph/EXIF/curve constructors/color-bitmap fonts and complete math remain open; affine supported-glyph handling is separately specified below.

After first validation, render_document_to_png:154–180 decodes straight RGBA then treats it as premultiplied Pixmap. The first full suite8537/0/7 in first-pass did not test translucent recomposition. Explicitly premultiply decoded pixels before Pixmap; preserve white canvas/dimensions/gaps/order. Two translucent pages+opaque gap RED→GREEN and fresh final-state validation are required.

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

Consume FramePaint/FrameStroke in Shape. Render a prepared cell to a transparent periodic texture with local clip; fold negative-spacing overlap in paint order, Repeat-shade fill and stroke while preserving cap/join/dash/alpha/masks. Derive resolution from transformed pitch; subpixel reduction uses premultiplied area mean, never representative color. Propagate affine root-relative transform, reset at paint_root; each cell is its own root. Reuse fonts/images, no grid clones proportional to target area.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

native-v1/dense-open reveals indiscriminate supersampling blur: a2px cell was enlarged to16 then Triangle-reduced. Render periods≥1px directly; only subpixel periods supersample and use premultiplied area mean. Do not blur aligned integral edges through redundant reduction. Use pinned libm::hypotf for matrix-column(sx,ky)/(kx,sy) norms once per axis, preserving ceil/subpixel threshold. Pass updated logical root through State to render_group without extra bool, retaining physical transform/masks. No raised limit/allow; before/after suites/renders/fmt/doc/lineage.

## Position-sampled raster gradients

`lab/results/construction/diagnostics/png-gradientes-2026-10-14/before-final/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty status/diffstat/UTC/source+binaries/commands/28 bilateral144PPI controls at historical9f2b6e8715237cb086899a42873660fe744622e8. Solid matches; counts above16 maximum-channel difference: linear4440, radial4785, conic4426, Parent-cell4200, background44800. Threshold describes inventory, not acceptance tolerance. before/before-valid keeps invalid fixtures rejected by both, not raster-defect proof. render.rs:515–517 and:120 reduce gradients to solid, State:56–60 lacks root dimensions. Pinned typst-render/paint.rs:135–248 and visualize/gradient.rs:969–1057 sample normalized position/aspect/center/focus/angle/colors with Self bbox/Parent rigid root; shape.rs:55–77 includes stroke in gradient bbox. These are color/geometry observables, not bytes.

Internal parity supersedes only Shape/PageFill's solid Gradient projection; layout still prepares cells. Position-sample Linear/Radial/Conic using existing L1 offsets/color-space/alpha, never representative color or premature alpha byte. Linear preserves aspect/orientation; radial both circles/focus/focal radius and zero-radius center without NaN; conic center/angle/aspect/turn. Use libm trig. State retains root size/sampling densities. Page/Group.paint_root/prepared cell create roots; technical groups only compose matrix. Parent uses root↔shape affine inverse including position, never clip-based root inference. Self uses geometric bounds including stroke, curve extrema/stroke rather than control points/full canvas. Negative rectangles mirror orientation with positive raster bounds. Keep dash/cap/join/miter/fill-rule/masks. Fill/stroke are independent; one absent texture must not block the other.

Gradient textures cover bbox/root at explicit resolution with Pad/Nearest, never Repeat. Respect anti_alias, sample integer pixel coordinates, convert float RGBA→gamut clamp→premultiply→bytes. Background uses physical canvas with bleed, content logical root. Tiling retains Repeat/period composition/resources; cell bodies may contain gradients without fallback. No global cache/public L1 interpolation change/new API/flag/default/phase.

Require pixel RED→GREEN for three geometries/alpha/Self/Parent/group root/tiling/stroke/background/negative rectangles. Repeat bilateral negative-angle/displaced-focus/stops/sharp/transform/clip/overlap corpus without tolerance increase or realignment; separately record prior layout/interpolation failures. Revalidate suites/build/doc/fmt/direct-reverse lineage/warning inventory. These controls certify neither direct fonts, ICC/CMYK, allocation bounds, performance nor all PNG parity.

png_gradients_tiling_root_and_anisotropic_sampling in anisotropic-red.log/anisotropic-red-source.rs.txt measures(149,0,106) where geometry requires(128,0,128): raw pixel aspect changes45° under unequal densities. Use (width_px/density_x)/(height_px/density_y). Density is sampling, never author angle/center; isotropic-only success is insufficient.

isolation/results.json.gz and svg-controls/results.json use first-pass/candidate-final.json.gz: Round-solid has 396 pixels above16 in PNG, zero SVG at144PPI. svg/geometry.rs:78–99/current L0 defines external radii with centerline max(radius−thickness/2,0); raster failed to inset. Do not repair geometry through gradient sampling. For stroked RoundedRect build closed fill and stroke path open back to origin, subtract half thickness from all radii/clamp to smaller semiaxis. No-stroke/clips retain original radii. Stroke gradient bbox uses that path. Test declared corner coverage/asymmetric/dash/cap/clip controls, scoped lineage and fresh full suite.

## Affine outlines for supported shaped glyphs

`lab/results/construction/diagnostics/transformacoes-2026-10-14/refuters-third/results.json.gz` preserves sources/commands/HEAD/diffstat/UTC/binaries. reflow-wrap-rotate matches SVG, diverges PNG; PDF in refuters-pdf-first. render.rs:402–405 uses sy ppem and only sx/sy origin, ignores kx/ky and leaves uniform shape. Pinned typst-render/text.rs:53–91 uses full outline matrix when uniform raster is insufficient. This measures language position/shape, not PNG bytes/cache mechanics; existing transport authorizes internal correction without new API/phase.

Keep supported TextShaped advances/offsets/prepared face/alpha. For rotation/shear/nonuniform scale/reflection render same-face outlines with full group transform→glyph origin→font-unit scale→vertical inversion, preserving accumulated device mask. Positive uniform route may remain pixglyph; large sizes may use outlines. Close open contours including CFF2. Missing outline creates no invented ink. Direct Glyph/plain unshaped Text/color-bitmap fonts/complete math remain uncertified. Refuters: asymmetric glyph90°/nonuniform/shear/reflection against geometric coverage distribution, masked siblings unchanged; uniform controls unchanged, source→PNG follows SVG/PDF/pinned geometry without realignment.

## Exact leaf arity exceptions and dead transport

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` preserves audited source/prior L0/signatures/hashes/HEAD/tree/status/diffstat/UTC2026-10-15T16:39:18.873477+00:00. render_text_shaped:349 has 9 arguments(C047), render_glyph:388 has 8(C048), render_shape:839 has 10(C049), threshold7. Glyph receives resolved target/state/face-GID/position/size/color; Shape resolved geometry/fill-stroke-rule/state/texture fonts. All are real leaf inputs; regrouping globally or repeating selection/layout violates boundaries. No new parity/functional-defect claim.

clippy-locais-2026-10-16/clippy-inventory.json records 79 warnings; clippy-excecoes-2026-10-16/before.json.gz retains source/spans/reasons/signatures/hash/HEAD/tree/UTC/diffstat. Its direct nominal follow-up authorizes only C048/C049, not the conditional C047 before repair. clippy-aridade-interna-2026-10-16/before.json.gz then records checked current sources/callers/contracts/spans/28-warning inventory: remove only private _text parameter/argument and the unused dispatcher TextShaped text binding. Keep author FrameItem field, painting body/face/upem.max(1)/offsets/alpha/transforms/omitted paths; no invisible PNG text/HTML/PDF change. Only after confirming eight real inputs may the residual C047 expectation apply. This supersedes its pending status only with the dead transport removed.

Preserve signatures/order/semantics for direct exceptions and exact literals:

- `#[expect(clippy::too_many_arguments, reason="C048: Leaf raster recebe alvo, estado gráfico, face/GID, posição, tamanho e cor resolvidos")]`
- `#[expect(clippy::too_many_arguments, reason="C049: Leaf de Shape consome geometria resolvida, fill/stroke/fill-rule, estado e fontes para textura")]`
- `#[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")]`
- `#[expect(clippy::too_many_arguments, reason="C047: Leaf raster usa oito inputs efetivos de pintura; texto autoral não é transportado para PNG")]`

Use only the named function/lint;  no module/impl/crate/global application, threshold
increase or other lint. This narrowly supersedes generic suppression bans only
for named item/lint. If no longer over threshold remove/review the expectation;
never silence unfulfilled_lint_expectations. Conditional exceptions requiring
dead-argument removal still require that correction; pending correction cases
(including historical C047/C066) gain no annotation here.



Continuous ADR-0127 changes only lint metadata, no public contract/executable body/
cfg/default/phase/dependency/architecture. L0 first/scoped resealing. Removing added
attributes and lineage header must recover prior rustfmt-normalized source.
Measure real RED, then all-target inventory: historical batch exactly 46 arities
plus five test warnings satisfied by 49 expectations, no new warnings and
unfulfilled_lint_expectations as error; its 11 correction arities remain visible.
Keep owner functional/call controls without fixture/tolerance relaxation; private
signature corrections migrate callers/L0 before code and new public/compatibility
needs another ADR-0127 gate. Validate build/fmt/strict/reverse lineage/Tekt no new
errors/warnings, existing Counter/coverage tests under 12 GiB, zero swap, one job.
Prior full suite is evidence for identical executable code, not a fresh run;
no lint-attribute mirror tests or new performance comparison for metadata.

For the six-case private arity slice, only conditional C047/C066 residual expectations are permitted after their respective _text removals; five other correction arities remain visible. L0/private caller owners precede code, no external contract/default/phase/dependency/semantics/new global state. Preserve public fields/values, PNG/SVG/PDF/precedence/diagnostics/resources/MCIDs and unchanged fixtures/expectations/tolerances. Real selected RED→GREEN/denied expectations, workspace/build/all-target Clippy/Rustdoc/fmt/strict reverse lineage, 12GiB/swap0/jobs1; no private-carrier mirror tests or claimed performance from representation.
