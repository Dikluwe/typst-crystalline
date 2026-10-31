# L0 — stream/text
Hash do Código: ac4d28b7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/word-joiner-unicode.toml sha256:3fbe26365aae1904cecc40eb2cb61bd6c73ea0f6a4d84ca9ba7a8820f028075f
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:d286c7b15a39f05d3792706ade43933abbab24ec298e92998fe3151f41c7c340
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:afd5a809b6e3f2ac1f9a752fb1f6c6d704139eb1f71b085d430d9417f826b04e



**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/text.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Text and glyph operators

Own font lookup by family/variant/variations and Text/TextShaped/Glyph emitters in
Compact/Verbose. Type1 retains WinAnsi fallback. CIDFont/Multifont use supplied
old-to-new GID mappings; direct mathematical glyphs use their per-font reverse map.
Paint bitmap glyphs through prepared XObjects and placement metadata, not outline
fonts. Do not perform shaping, font instancing or resource allocation.

TJ uses (nominal - x_advance) / units_per_em * 1000 and compensates x_offset with
the existing paired adjustments, rounding and units. Preserve the current y_offset
scope without introducing new operators. Verbose uses balanced q/cm/color/BT/Tr/Tf/
Tc/Tm/TJ-or-Tj/ET/Q envelopes, explicit rendering mode, faux bold, tracking and spot
colors. Compact retains its current operator form. These helpers serve both top
and local dispatchers, whose coordinate composition is owned by draw.

Coalesced-run compatibility, boundary selection and rounded inter-item TJ
compensation belong to `00_nucleo/prompts/infra/export/stream/text/runs.md`.
Declare the descendant runs module and preserve the internal reexports used
by the stream/page dispatcher. This owner retains single-item text/glyph
operators and their shared envelope helpers; do not duplicate run decisions.

## Verification

Exercise kerning, offsets, remapping, same-GID multifont identity, bitmap placement,
mode envelopes. Run boundaries and mathematical/UA1 noncoalescing are
verified through the runs owner and existing integration regressions.

## Occurrence substitution inside graphic state

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/envelope-probe/` refutes outer scope: EMC before Q preserves boxes, after Q loses Poppler position; opening before/after cm/Tm does not change this. No codepoint-count closure or Q-text search/rewrite. Keep existing signatures and offer private deferred graphic-tail variants: emit text/ET, return whether Q is pending without restoring. Type1 considers only faux-bold q; Verbose embedded fonts have own q; bitmap no textual tail. Draw requests deferred form only for TextReplacement's last visual child, then closes leaf marks→ActualText→pending Q. Earlier/ordinary children retain emission. Modes keep operators/transforms with no mode switch/operator-text heuristic. text owns formatting/tail, draw dispatch/leaf marks, `00_nucleo/prompts/infra/export/stream/text_replacement.md` UTF-16/BDC/EMC. Private parity, no public API/phase/flag; require original-position top/local/both-mode extraction, literal coexistence/balanced scopes/raster.

math-large-identidade-2026-09-30 uses historical pinned vanilla/NewCMMath DisplayOperatorMinHeight3000du. Wrapping three ∫ pieces outside graphics states extracts the character but Verbose Poppler box has zero height and -layout places it before∑; Compact also reverses order. Temporary-PDF experiment with identical pieces/positions/GIDs refutes outer scope: BDC after first-piece BT and EMC before final-piece ET restores positive bbox and∑, ∫ order in both modes. Never infer text from GIDs. Internal marked Glyph variants accept causal opening/closing from draw and fully reuse normal selection/remap/color/position. Only TextReplacement-piece BDC/EMC insertion points differ. Keep ordinary Glyph format/default/Type1, no invisible text. Marking encoding remains text_replacement's ownership.

## Shaped tracking: authoritative advances and serialized cursor


`lab/results/construction/diagnostics/shy-autoral-2026-09-28/tracking-controls.json` measured plain-verbose maximum deviation2.652pt versus historical pinned vanilla, compact0.348pt, with state/binaries/commands in its receipt. `lab/results/construction/diagnostics/tracking-pdf-2026-09-28/{initial.json,before/}` preserves the new state/matrix. shaping.rs:319–326 adds tracking to advances, text.rs:639,668 and runs.rs:107 emit Tc again; the run cursor accounts for advances/TJ, not extra Tc. Historical upstream inline/shaping.rs:1269–1292 applies tracking in glyphs. This is observable spacing, not operator/byte equality; unadjusted advances or no extra Tc refute the cause.

CIDFont/Multifont TextShaped carries authoritative adjusted advances: tracking comes from those advances alone. Both single/coalesced emission explicitly set Tc=0 inside BT before TJ even with absent/zero tracking, eliminating inherited extra spacing. This is not a font/GID/fixture/width exception. Never subtract tracking from advances or infer cause from text. Text/Type1 Tj keep existing tracking; bitmap stays image drawing. No public API/flag/default/phase change. Layout measurement/ligature tracking remain independent; the later measured fractional correction below supersedes only the former preservation of that rounding limit.

`lab/results/construction/diagnostics/tracking-residual-2026-09-29/{initial,results,summary,causal-checks}.json` records 36 entries/108 PDFs, pinned oracle/source/UTC/HEAD/full tree. An isolated word accumulates+0.052pt at0.2pt tracking, while adjacency loses one interval, one literal space two, three spaces four. ±1.2pt removes quantization but not boundaries. shaping.rs:225–232,319–326 rounds to i32, text.rs:49–83 splits words, space.rs:15–33 uses nominal advance. Preimplementation state: correcao-tracking-2026-09-29/initial.json. For the coalesced cursor use the single leaf's fractional correction: nonzero-tracking TJ boundaries serialize five decimals against the actual serialized cursor; other runs retain integer rounding. Preserve absolute positions/nominal/GID/offset/envelopes/compatibility. Acceptance: boundary/selection RED→GREEN, independent Tc/TJ interpretation, bilateral ±/zero/repeated/adjacent/wrap controls, build/lint zero errors and scoped lineage maintenance; continuous internal parity only.

Own private push_shaped_spacing emitting Tc=0, called inside Compact/Verbose CIDFont/Multifont TextShaped BT including deferred variants; child runs shares it. Keep TJ deltas/offsets/remapping/scopes/transforms/bitmap. Independent minimal Tc/TJ interpreter tests each glyph with prior nonzero Tc, ±/zero/absent tracking, CIDFont/Multifont, absolute merged boundaries and Text/Type1 tracking. PDF probes preserve Compact/Verbose visuals and lexical SHY/U+002D; measure oracle progress without rounding normalization/wider tolerance.

Single-shaped/bitmap emission uses fractional residue derived by tracking owner. TJ retains old integer kerning delta/rounding and adds only tracking residue, five decimals when present. X-offset/zero tracking keep prior format. Private TJ helper may accept TextStyle, never infer tracking from GID/text. Bitmap adds same residue to cursor with no new text operator. Tc=0 remains mandatory.

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

## Effective variable weight and synthetic stroke


variable-axes-after.pdf has correct wght250 hmtx/glyf but `2 Tr`/`0.800 w` from semantic weight700. At 600dpi extra ink is visible despite unchanged word bbox; text.rs:274/shaped callers and runs.rs:105 reapply faux-bold over realized wght. Preserve distinction between source axes and semantic style.

Only TextShaped with certified Some Font whose fvar declares real wght uses fill-only0 Tr in verbose single/coalesced emission. The shared private text helper checks real axes only when style would request stroke. Keep Text/Type1/None/static fonts as before, without rewriting semantic weight/bold, new API/phase/unsafe/I/O. Compact shaped already avoids stroke. Static faux-bold/other formats remain outside this correction. Require weight700/wght250 single+coalesced RED→GREEN, None controls and full bilateral raster/boxes.

## Private imports and equivalent observations

text.rs:9 and `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz` preserve source/owner/spans/compiler output/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted status/diffstat/UTC and productive/test/comment-use audit. Remove only proven unused private imports, retain required public/child/test aliases; no formula/default/phase/API/acceptance/suppression change. Initial all-target compile check-initial-failed.log finds PageContext in internal tests at candidate1018/1225/1344: restore import cfg(test), never remove test dependency or restore normal-library import. Revalidate all targets/test names/assertions/build/workspace Clippy/Rustdoc/lineage, no import/comment mirror tests.

text.rs:25(C050) and `lab/results/construction/diagnostics/type-complexity-2026-10-15/{before.json.gz,red-clippy.log}` preserve state/source/owners/hashes/UTC and RED. Use existing export::FontKey in font_index_for_style fonts with identical expansion; remove only obsolete FontList/FontVariant/FontVariations imports. Preserve family/variant/axis selection/fallback/style/PDF operations. No public/default/phase change; aliases require identical expansion and new/incompatible public types another gate. Nominal type_complexity exceptions cover only three justified batch items, no module/impl/crate silence/raised threshold/other repair suppression. No alias/name-mirror tests.

text.rs:1073 clippy::double_ended_iterator_last and `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/{before.json.gz,initial-diagnostics.json}` preserve source/L0/HEAD/tree/status/diffstat/UTC/suggestions/spans and type/use audit. In test rendered_positions select last Tc using next_back on double-ended filtered lines. strip_suffix is pure; preserve parse/zero fallback/independent position calculation/all TJ expectations; emitter unchanged. This Rust maintenance proves no parity/performance. Continuous flow/L0 first/scoped lineage, selected lint RED→GREEN/unchanged controls/full workspace/build/all-target Clippy/expectations/Rustdoc/fmt/lineage; no syntax-only tests.

## Solid textual opacity


`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserves source/owner/HEAD/UTC/tree. tiling-nativo-2026-10-14/final-renders and remaining-analysis.md show cell8×8/pitch4×4/alpha50%: PDF opaque red, SVG one contribution, PNG four(255,15,15). pdf_tiling.rs:184 uses BBox=size, svg/paints.rs:148 overflow visible, shape.rs:155/text.rs:275 drop alpha. Upstream also clips overlap and is not a requirement for that defect. The explicit-period source-over hypothesis must preserve no-overlap controls and analytical four-contribution composition.

In the shared verbose prefix, including merged runs, apply textual-color ExtGState inside existing q before BT. Dispatcher isolates Compact/direct Glyph. Preserve coalescing/metrics/ActualText/position; fill/stroke alpha are equal for faux-bold. This is internal within the approved tiling package, no new public API/phase/flag/default. L0 first/RED→GREEN, analytical composition including distinct-color order, simple/nested/rotated/text/image and extraction/page-selection controls; workspace/build/fmt/Rustdoc/lineage. Variable-gradient alpha/PNG quality remain incomplete, so no production certification with these open.

## Exact text-emitter arity exceptions

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` preserves checked source/prior L0/signatures/hashes/HEAD/tree/UTC/status/diffstat2026-10-15T16:39:18.873477+00:00. text.rs:342 emit_shaped_pdf(C051,9), :367 deferred(C052,9), :392 inner(C053,10), :533 emit_glyph_pdf_marked(C054,9), :719 verbose(C055,9), :744 verbose_deferred(C056,9), :769 verbose_inner(C057,10), :895 emit_glyph_pdf_verbose_marked(C058,9), threshold7. Compact receives author text for Type1 fallback separately from glyphs/effective face/style/PDF position/scale/resource scenario, avoiding reshaping. Deferred variants return pending Q to close ActualText first. Inner variants add explicit restore, sharing normal/deferred emission. FontScenario/face cannot be replaced by name, nor text inferred from GIDs. Marked glyph opens/closes causal Large occurrence independently across first/last pieces; Verbose retains BT/ET/nominal-remapped linkage. Format belongs to this owner and never authorizes reconstructed text/font.

clippy-locais-2026-10-16/clippy-inventory.json records 79 warnings; clippy-excecoes-2026-10-16/before.json.gz preserves source/spans/reasons/signatures/hashes/HEAD/tree/UTC/diffstat. Nominal follow-up materializes only these eight direct expectations, superseding audit-only pending status, not correction cases. Preserve signature/body/calls/cfg/order/other attributes/semantics and exact function-local literals:

- `#[expect(clippy::too_many_arguments, reason="C051: Emissor PDF Compact recebe texto autoral para fallback Type1, glyphs e face efetiva separados, estilo, posição PDF, escala e cenário de recursos")]`
- `#[expect(clippy::too_many_arguments, reason="C052: Variante Compact diferida possui os mesmos inputs causais e retorna o Q pendente para fechar ActualText antes da restauração gráfica")]`
- `#[expect(clippy::too_many_arguments, reason="C053: Núcleo Compact acrescenta restore à operação shaped; o controle distingue tail normal de diferido sem duplicar emissão")]`
- `#[expect(clippy::too_many_arguments, reason="C054: Emissor de glifo marcado recebe abertura e fecho causais de ActualText independentemente")]`
- `#[expect(clippy::too_many_arguments, reason="C055: Emissor Verbose preserva a mesma identidade shaped, posição PDF, escala e recursos com envelope específico")]`
- `#[expect(clippy::too_many_arguments, reason="C056: Variante Verbose diferida transporta os mesmos inputs e informa restauração Q pendente")]`
- `#[expect(clippy::too_many_arguments, reason="C057: Núcleo Verbose recebe restore explicitamente para compartilhar a implementação normal/diferida; os demais inputs são os operandos reais do texto e seu cenário de fontes")]`
- `#[expect(clippy::too_many_arguments, reason="C058: Variante Verbose do glifo marcado conserva abertura/fecho de ocorrência Large dentro de BT/ET e o cenário nominal/remapeado")]`
- `#[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")]`

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

## Compact absolute coordinate precision

### Additional measurement before correction

`lab/results/pdf-bbox-precision-2026-10-23/compact-candidate1.json` keeps
four compact-vs-verbose controls from the frozen first candidate. Hello/Table/
prose pass0.001pt, but math has xMin0.0076pt and yMin0.0346pt differences.
text.rs:573,604 emits direct glyph Td with one decimal, unlike verbose cm's
five decimals; compact shaped Td uses three and plain Text one. These are
serialization losses on supplied absolute coordinates. This is a parity
correction under ADR-0127, no public/default/phase/compatibility change.

### Current obligation

Emit all compact text/shaped/direct-glyph Td origins with five decimals,
matching the default verbose positional precision; preserve operator choice,
font size policy, Type1/bitmap policy, graphics and ActualText scopes. The
prior paragraph's Type1 policy exclusion does not retain its one-decimal Td
loss. Require causal compact absolute-origin RED→GREEN for all text carriers,
then strict compact-vs-verbose0.001pt text/page/box controls on Hello/Table/
prose/math, plus complete final-state gates. Retain the candidate1 failed
compact measurement and interrupted validation; neither certifies final scope.


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


## R10 — source-preserving raster glyph fallback

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,bitmap-evidence.json,font-tables.json}` binds received R9 code/dirty stat/UTC and fixed bd5ebec2 binaries. Nimbus/Liberation mixed123☃ selects Noto Color Emoji. The received PDF raster route emits only image Do, losing the source character in extraction; SVG glyphs.rs:100-105 requires an outline and emits no raster. Pinned paired PDFs/SVGs preserve source text and the PNG. Font-family names are evidence, never runtime branches; supported actual raster data authenticates the correction. Source character, physical position/advance, PNG pixels and geometry are language observations. Object numbering, Type3 versus CID structures and byte encoding are mechanics.

Classification: private font/export parity correction under ADR-0127 continuous flow, with no new public method/entity/phase/mode/dependency. The previously approved FontMetrics method remains separately bounded. No invisible duplicate source text, fake outline, font-name coefficient, or oracle-byte copying. Build a1016-byte self-authored portable bitmap font fixture from the preserved fontTools generator; it is test data, not a font copied from the oracle or a productive dependency. Authenticate its bytes explicitly in the receipt.

### Exclusive owning obligation

When supplied bitmap references carry complete Type3 codes, emit source glyph text with the existing actual font resource and the builder's single-byte encoding, using original advances/offsets/tracking. Its CharProcs draw the prepared image; do not paint it twice or emit invisible synthetic text. Both Compact/Verbose, single/multifont and local coordinates preserve graphic state and accessibility/ActualText closure. Uncertified bitmap references retain original Do emission. No independent resource allocation or font selection here.
Require module RED→GREEN before implementation, fixed-pin actual fallback PDF/SVG paint/text/geometry controls, previous A–H/R1–R9, full workspace/lineage/resource gates and byte preservation of received unrelated inputs.


R10 closure decision after actual RED: red-svg-residual.json/log/source archive
in the R10 evidence root confirms the hypothesis for the portable bitmap.
Plain source bbox is(10,13.25,19,22.25); source-identical TextReplacement becomes
(10,20,11,20). The old helper closes Q before the existing ActualText protocol
can close its marked content. For the certified Type3 path, preserve the existing
restore/deferred-tail convention: open the helper q once, emit the actual text,
and let the caller close Q when restore=true or return a pending tail when
restore=false. The existing draw/replacement owner then closes its original
marks/ActualText before Q. Preserve normal/legacy image paths and all source
text, glyph positions, bytes of prepared PNGs, advances, offsets and bitmap
font selection. No new signature, mode, phase or invisible text. The expanded
plain/replacement test must pass in both modes and single/multifont, together
with SVG fractional tracking and all previous gates.


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

Use existing UTF-16 ActualText opening/closing for a shaped source run that
contains U+2060. Preserve the full original run text and glyph painting. Share
one private small wrapper across normal/deferred Compact/Verbose entry points;
close marked content before its corresponding graphical restoration, returning
any original deferred tail. No new public parameter, resource, shaping or
carrier. Runs without2060 retain their exact route. Test actual logical PDF
operators and balanced normal/deferred restores; enclosing replacement wins.

This is ADR-0127 continuous internal parity work: no public field/signature,
mode/default/phase/compatibility change. Existing no-repair/boundary clauses
are superseded only for this collision. V15/V26 must precede resealing; require
runtime RED→GREEN, original outline/code-data and Unicode-first/last/Box and
ligature controls, geometry0.001pt/independent PDF/SVG paint, prior corpus,
full suites/build/Clippy/Rustdoc/fmt/forward-reverse lineage, and preserved JPEG
resource gates. Do not infer global Unicode/PDF-UA or readiness completion.


### Zero-width alias refutes first candidate

unicode-green.json/log in the readiness-fixes evidence root records two scope
passes but the real-font extraction still returns A SPACE2060 newline B SPACE C.
Its exact source is unicode-candidate1-source.json.gz. The prior wrong global
space alias is repaired; ActualText alone does not eliminate the false break.
The injected run has x_advance0 and the actual DejaVu space glyph; its font has
a positive nominal space advance and no outline for that glyph. Inference:
reader word bounds still see the nominal blank glyph despite TJ cancellation.
A zero horizontal scale for only this provably empty, zero-advance alias run
must be tested; unchanged extraction or altered paint refutes that inference.

Before the body correction, test actual empty-space face versus ink glyph,
nonzero advance and absent face. Only a pure2060 run whose every glyph is the
actual supplied face's empty space glyph, zero advance/offset, and whose font
scenario is not bitmap, qualifies. Under its own saved graphical state insert
0 Tz inside the emitted BT; preserve source ActualText and exact source advance.
Restore graphics state, including deferred Compact/Verbose tails, without leaking
text scale to following prose. No shaping/resource lookup/field or global font
policy; absent/invalid face and other runs retain the previous route. This
supersedes the old no-new-text-operator clause only for this measured alias.
Keep the original strict extraction expectation, enclosing replacement and paint
gates. Legacy None-face scope is not newly certified. Continuous ADR-0127.

The draw owner may call the existing private shaped kernels within an authored
TextReplacement to suppress automatic nested ActualText, preserving original
Type1 fallback text and all painting/restoration operands. Kernels are visible
only to sibling stream modules; exported interfaces remain unchanged.

Write the zero-alias text-scale operand directly at the shaped kernel's BT
emission point, supplied by a private explicit flag from the occurrence wrapper.
Do not locate operators in serialized stream text or rewrite the emitted stream.
Draw's authored-replacement delegation supplies false: it preserves original
paint and leaves the outer occurrence scope authoritative. Keep exported
entry-point signatures and the existing extraction tests unchanged.


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

Preserve public entry signatures and all glyph kernels. Factor the existing marked wrapper so a private occurrence emitter may receive an explicit replacement separately from the ORIGINAL painted run text. Normal paths retain their previous conditional U+2060 replacement; only a draw-certified pair supplies the exact suffix, including an empty suffix. Force that explicit ActualText even without a remaining joiner so the original blank glyph cannot leak a space. Compute zero-alias qualification from the ORIGINAL glyphs/text/real face, keeping its existing0Tz state and normal/deferred restore protocol. Emit original glyphs/advances/font/style once, and close the replacement before its corresponding Q. Do not split/re-shape/rebase glyphs, inspect serialized BT/Q text, rewrite the output stream or place a wide replacement over the pair. Authored outer replacements retain their old raw-kernel route.

ADR-0127 continuous internal parity work: no public field/signature/trait,
phase/default/mode/compatibility change. L0 and meaningful real-font PDF RED
precede implementation. Preserve actual source concatenation, all earlier
Unicode/ALT/ligature/space/bidi/UA1 controls and independent PDF/SVG painting.
Require V15/V26 before resealing, forward/reverse lineage, full technical gates
and final historical/JPEG resource validation. This does not certify general
Unicode extraction, PDF-UA, HTML, bibliography or overall product readiness.

## Actual static face already realizing the requested weight

number-bold-stream-before.json/provenance bind number2 release/source archive,
full streams, actual font descriptors and raster bands at dirty212aed0449/UTC.
The real LibertinusSerif-Bold face receives2 Tr/0.440w; body differs1344 pixels
on first page although SVG matches. physical4-extras independently retains
PDF1514/1255 disagreement with matching geometry/SVG on both pages. The pinned
typst-pdf/src/text.rs:37-49 supplies only authored text stroke. This supports a
redundant-stroke hypothesis, requiring a real static-font renderer RED before
repair, rather than font-name or appearance classification.

Supersede the old keep-static-fonts clause ONLY when a supplied actual Font
parses and its static OS/2 weight equals the requested style.weight. In the
existing shaped_tr_ops helper, such a face uses fill-only0 Tr when faux stroke
would otherwise be requested. Preserve semantic style, actual font/glyphs, all
metrics and the existing variable wght certificate. Missing/invalid/nonmatching
static face, None, Text/Type1/Compact and authored graphical styles retain their
previous policy. Do not generalize to unmatched weight or choose by font name.
Require actual static face single/coalesced emitter RED→GREEN, real regular-face
and None preservation, existing variable tests and fresh paired PDF/SVG. This
is an internal export parity correction, with no new API/phase/default/flags.

Functional RED receipt: entry-text-weight-red.json/log and source archive retain
exact inputs/HEAD/dirty list/UTC and --no-fail-fast execution. Core records one
actual suffix at81.391pt instead of the regional right edge; four actual Entry
query tests and one real static-font emitter fail in infra. The hidden Entry
control and all prior readiness tests remain green. Preserve those logs rather
than treating later tests as historical RED.
