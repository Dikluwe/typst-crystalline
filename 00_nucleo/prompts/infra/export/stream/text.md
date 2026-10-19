# L0 — stream/text
Hash do Código: 244705cd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:decdb0b923135236d590767fe6075281006841b5da32f79aad375ebb58f84fe0
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
