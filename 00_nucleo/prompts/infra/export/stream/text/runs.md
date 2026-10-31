# L0 — stream/text/runs
Hash do Código: 6964b6db

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/word-joiner-unicode.toml sha256:3fbe26365aae1904cecc40eb2cb61bd6c73ea0f6a4d84ca9ba7a8820f028075f
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:d286c7b15a39f05d3792706ade43933abbab24ec298e92998fe3151f41c7c340
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text/runs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Before extraction, `03_infra/src/export/stream/text.rs:621,655,696,767`
contained envelope compatibility, run-boundary selection, run emission and
rounded inter-item TJ cursor compensation. The source snapshot and tests are
preserved in `lab/results/construction/diagnostics/atomizacao-separadores-2026-09-28/`.
The bilateral separator probes in
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` reproduce a text
extraction divergence against vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`. They do not establish total PDF
byte equality, general reader behavior or the earliest historical failing build.

## Ownership and contract

Own compatibility, boundaries and emission of coalesced shaped prose runs.
Receive FrameItems and the prepared FontScenario/PageContext from the ancestor.
Keep the existing static interface within export::stream; allocate no font
resources and perform no shaping, I/O, instancing or language evaluation.

Coalesce only consecutive shaped prose with equal baseline and complete
emission envelope: font identity and variations, size, fill, weight,
bold/italic, tracking, direction, math flags/size and units_per_em. Empty
glyphs, incompatible envelopes, nontext, group/link scopes, math and bitmap
leaves terminate or prevent runs. UA1 call-site policy belongs to the page
owner; this unit does not independently decide tagging policy.

Emit balanced verbose envelopes through the text owner's prefix/render-mode/
tracking helpers. Select the prepared single-font or multifont mappings by
complete style. Preserve the supplied nominal/actual advances, offsets and
old-to-new GIDs. Every inter-item TJ boundary compensates from the effectively
rounded cursor to the next absolute x position; include per-glyph rounding
in that cursor. Do not replace absolute positioning with nominal advances.

## Acceptance and limit

This extraction preserves current text, geometry, scope boundaries, diagnostics
and rendering operators. Existing P979 and text-operator regressions verify
compatible grouping, envelope boundaries and rounded cursor positioning.
The missing separator provenance is unresolved by this structural extraction;
this L0 does not authorize inventing spaces from positive geometric gaps.

## Shaped tracking: authoritative advances and serialized cursor

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/tracking-controls.json` measured plain-verbose maximum deviation2.652pt versus historical pinned vanilla, compact0.348pt, with state/binaries/commands in its receipt. `lab/results/construction/diagnostics/tracking-pdf-2026-09-28/{initial.json,before/}` preserves the new state/matrix. shaping.rs:319–326 adds tracking to advances, text.rs:639,668 and runs.rs:107 emit Tc again; the run cursor accounts for advances/TJ, not extra Tc. Historical upstream inline/shaping.rs:1269–1292 applies tracking in glyphs. This is observable spacing, not operator/byte equality; unadjusted advances or no extra Tc refute the cause.

CIDFont/Multifont TextShaped carries authoritative adjusted advances: tracking comes from those advances alone. Both single/coalesced emission explicitly set Tc=0 inside BT before TJ even with absent/zero tracking, eliminating inherited extra spacing. This is not a font/GID/fixture/width exception. Never subtract tracking from advances or infer cause from text. Text/Type1 Tj keep existing tracking; bitmap stays image drawing. No public API/flag/default/phase change. Layout measurement/ligature tracking remain independent; the later measured fractional correction below supersedes only the former preservation of that rounding limit.

`lab/results/construction/diagnostics/tracking-residual-2026-09-29/{initial,results,summary,causal-checks}.json` records 36 entries/108 PDFs, pinned oracle/source/UTC/HEAD/full tree. An isolated word accumulates+0.052pt at0.2pt tracking, while adjacency loses one interval, one literal space two, three spaces four. ±1.2pt removes quantization but not boundaries. shaping.rs:225–232,319–326 rounds to i32, text.rs:49–83 splits words, space.rs:15–33 uses nominal advance. Preimplementation state: correcao-tracking-2026-09-29/initial.json. For the coalesced cursor use the single leaf's fractional correction: nonzero-tracking TJ boundaries serialize five decimals against the actual serialized cursor; other runs retain integer rounding. Preserve absolute positions/nominal/GID/offset/envelopes/compatibility. Acceptance: boundary/selection RED→GREEN, independent Tc/TJ interpretation, bilateral ±/zero/repeated/adjacent/wrap controls, build/lint zero errors and scoped lineage maintenance; continuous internal parity only.

For coalesced emission use ancestor push_shaped_spacing instead of push_tc_if_tracking. Cursor remains nominal+shaped advance+rounded TJ. Keep compatibility/boundaries/absolute compensation. Causal emit_verbose_text_run tests live in ancestor text, without shared productive ownership.

## Ratified effective face binding


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

## Export-local validated face memoization

first-benchmark.json in the face-binding round measures+25.36ms grid320, +14ms rtl160/mixed160. font_binding.rs:index compares full bytes each query; runs.rs:verbose_run_compatible compares Font bytes per adjacency. Causal benefit remains an A/B inference. Per resource table/export cache only validated positives: keep an Arc Font clone alive and its index after first full bytes/request validation; a hit requires identical slice address+length and complete request. Distinct allocations require full equality initially. Never serialize pointers/create public pointer identity, cache misses in an append-capable binder, use global/cross-session storage or memoize Content across Worlds. Positive appended indices stay stable. Merge may shortcut identical slices but compares bytes across distinct allocations.

## Effective variable weight and synthetic stroke

variable-axes-after.pdf has correct wght250 hmtx/glyf but `2 Tr`/`0.800 w` from semantic weight700. At 600dpi extra ink is visible despite unchanged word bbox; text.rs:274/shaped callers and runs.rs:105 reapply faux-bold over realized wght. Preserve distinction between source axes and semantic style.

Only TextShaped with certified Some Font whose fvar declares real wght uses fill-only0 Tr in verbose single/coalesced emission. The shared private text helper checks real axes only when style would request stroke. Keep Text/Type1/None/static fonts as before, without rewriting semantic weight/bold, new API/phase/unsafe/I/O. Compact shaped already avoids stroke. Static faux-bold/other formats remain outside this correction. Require weight700/wght250 single+coalesced RED→GREEN, None controls and full bilateral raster/boxes.

## Unused local binding

runs.rs:72 and `lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/{before.json.gz,references-before.json.gz}` preserve owner/source/HEAD/tree/UTC/diffstat/calls/imports/modules and verified old inventory/suite hashes/pre-edit test list. Private calls/compilation were audited; dead_code alone is insufficient. Remove only the unused font binding in verbose_run_end's initial style.math test; preserve face lookup/bindings in multifont/other actual font-selection/comparison branches. Run bounds/bitmap/math/baseline/effective identity remain unchanged. Continuous flow adds no public field/API/default/phase/dependency/suppression. Selected diagnostics RED→GREEN are not parity proof. Preserve test list/expectations, run full suite/lib/all-target build/Rustdoc/fmt/lineage/total inventory, scoped resealing; no cfg/binding-mirror tests.

## Fractional PDF font-space precision (batch F)

### Measurement before classification

`lab/results/pdf-bbox-precision-2026-10-23/baseline.json` freezes the unchanged
validated E working tree, source hashes, HEAD, diffstat, UTC and binary identities.
`before.json` reproduces Hello maximum xMax error0.003652pt and Table maximum
xMax error0.007417pt at the ratified bd5ebec2 oracle; `causal-width-only.json`
keeps the failed width-only PDF mutation. This is diagnostic output, not an
accepted binary. Source fonts.rs:223 quantizes /W; text.rs:223–245 quantizes
nominal-minus-shaped advance; runs.rs:207–248 quantizes boundary and glyph cursor.
The retained oracle Hello content stream has fractional TJ58.59375 versus local59,
and fractional /W751.9531 versus local752. These are measured serialization
losses; neither byte identity nor a guessed upstream intention is acceptance.

### Correction and acceptance

For this owner's existing PDF responsibility, supersede earlier preservation of
integer text-unit rounding. Preserve effective face fractional widths, shaped
advance differences, x offsets, tracking residue and absolute run boundaries;
use the actually serialized fractional advance for the coalesced cursor. Retain
existing units, missing metrics fallbacks, GID remapping, subset/face identity,
operators, Tc=0, graphic scopes, bitmap and Type1 policies. No layout adjustment,
new public contract, default, phase, flag or font exception. Existing rounded
models are historical evidence, not current requirements.

Inference: combined width/TJ precision explains the strict box failures; residual
PDF box or independent painted-origin errors would refute completeness. Require
real module RED→GREEN with non1000 UPEM, remapping, offsets, fractional boundary
and tracking controls, then unchanged0.001pt word/page boxes on Hello/Table,
prose/math preservation, independent PDF/SVG placements and higher-resolution
painting. Keep original failed receipts. Do not round measured coordinates,
widen tolerance or substitute raster agreement for text-selection acceptance.
Reseal lineage and pass all workspace validation plus full crystalline lint.


## R8 fractional font-size precision

Evidence before decision: source-bound candidate2/receipt.json under
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24 records
7.7pt painted versus resolved7.69677734375pt and9.3pt versus9.35pt. The existing
Tf format at text.rs:113,430,674,825 and child runs.rs:124 rounds to one decimal,
while layout/shaping retain full size. Geometry is observable; decimal spelling
is mechanics. Preserve the resolved size to five decimals in every compact/verbose
Text/TextShaped/Glyph and coalesced-run Tf operator. Keep exact one-decimal values
in their existing spelling, delegating the common pure formatter to the text
owner (runs owns its delegation). No byte identity, new public contract, product
mode or source/shaping phase change. ADR-0127 continuous flow. Tests must show
actual emitted fractional font-size RED→GREEN, preserve ordinary integral output,
verify measured/painted glyph geometry and all prior PDF/SVG controls, and pass
full workspace and lineage gates without relaxing0.001pt.


## Readiness: Word Joiner and space CID collision (2026-10-28)

### Evidence before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/before.json`,
raw-outline-build.json and raw-outline-build-source.json.gz bind exact HEAD
212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diffstat/UTC, source hashes
and release. Oracle remains64c5e8c02fe0394d913ead36e42ef8c1334d2107, binary
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
raw-outline-candidate/comparison.json records identical outline PDF/SVG paint,
but failed words. Its control-outline-tekt.qdf:5568 maps space CID0001 to2060.
unicode-before/receipt.json reproduces ALPHA/U+2060/BETA/U+2060/GAMMA without
outline, before and after the joiner and inside Box; ligatures are a control.
`builder/fonts.rs:534-549,1040-1054` prioritizes shaped cluster strings before
used U+0020. The shaper can reuse the space glyph with zero advance for2060.
This proves a font-global extraction collision, not a need to remove an
invisible language character. Font resource order/GID machinery is mechanical;
source words and occurrence Unicode are observable. Altered paint, lost joiner,
changed ligatures or an overridden authored replacement refute the correction.

### Exclusive responsibility and acceptance

Treat a shaped source run containing U+2060 as a boundary of the existing
coalesced path, so its occurrence ActualText is emitted by the ordinary text
owner. Preserve all other envelope compatibility, rounded absolute TJ/offset
formulas and ownership of page/tag policy. Test the page stream's logical
ActualText output for adjacent joiner/ordinary text, not only a boundary index.

This is ADR-0127 continuous internal parity work: no public field/signature,
mode/default/phase/compatibility change. Existing no-repair/boundary clauses
are superseded only for this collision. V15/V26 must precede resealing; require
runtime RED→GREEN, original outline/code-data and Unicode-first/last/Box and
ligature controls, geometry0.001pt/independent PDF/SVG paint, prior corpus,
full suites/build/Clippy/Rustdoc/fmt/forward-reverse lineage, and preserved JPEG
resource gates. Do not infer global Unicode/PDF-UA or readiness completion.


## Readiness: source joiner prefix beside a causal separator (2026-10-28)

### Measurement before decision

readiness-fixes-2026-10-28/fraction-{candidate,extras}/comparison.json retain
identical original outline PDF/SVG paint after the allocation correction but
failed exact word strings. The immutable list-body-extras zero-inset PDF and
its source-bound build are retained independently. outline-scope-experiment/
receipt.json binds exact QDF/PDF/source/tool/worker hashes, HEAD/dirty stat,
UTC, commands, frozen compiler inputs and independent rasters. Repacked control
and a single wide ActualText remain negative. Two separate scopes pass exact
words and0.001pt boxes (maximum0.00001pt), with identical paint: the existing
space carries space+U+2060 and the original shaped run carries only its suffix.
Both inside-BT and deferred-restoration variants pass; copying a vanilla BT
arrangement is not required. This is observable text identity/bounds, not byte
or operator parity.

wj-before/receipt.json separately binds the new joined/split DejaVu dot/suffix
sources, compiler/oracle bytes and all source hashes. Tekt extracts separate
"." and "U+2060+1" words; pinned oracle extracts ". SPACE U+2060+1" as one word,
then TAIL. Bundled/system DejaVuSans.ttf bytes both hash
 ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280.
Oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree
483bd6f1d8ac8810db47b65336d402647955fd17, binary
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The earlier glyph-global collision and pure-joiner scale corrections remain
intact. No original source or expected word is normalized to grant a pass.

### Exclusive responsibility and acceptance

Keep all productive coalescing eligibility and run emission unchanged. The draw/page pair is a separate occurrence protocol, not permission to merge a joiner run into preceding or following prose. Existing U+2060 boundaries and adjacent-prose controls stay in force in both stream modes and all font scenarios.

ADR-0127 continuous internal parity work: no public field/signature/trait,
phase/default/mode/compatibility change. L0 and meaningful real-font PDF RED
precede implementation. Preserve actual source concatenation, all earlier
Unicode/ALT/ligature/space/bidi/UA1 controls and independent PDF/SVG painting.
Require V15/V26 before resealing, forward/reverse lineage, full technical gates
and final historical/JPEG resource validation. This does not certify general
Unicode extraction, PDF-UA, HTML, bibliography or overall product readiness.
