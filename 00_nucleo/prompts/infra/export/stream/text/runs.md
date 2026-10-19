# L0 — stream/text/runs
Hash do Código: bc9faca5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:decdb0b923135236d590767fe6075281006841b5da32f79aad375ebb58f84fe0
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
