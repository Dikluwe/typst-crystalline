# L0 — text
Hash do Código: 8cb95f0b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:b556a9f22e29d478ae1afac3e31427aeea3fa58f44a4aa04e46c86b90fb5779c
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/text.rs:20`, `01_core/src/compiler/layout/text.rs:209`, `01_core/src/compiler/layout/text.rs:237`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Historical pins do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new run.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and handler inspection precede separator transport; public carrier approval in `00_nucleo/prompts/core/entities/layout_types.md` was received 2026-09-28/recorded in materialization provenance.

`lab/results/construction/diagnostics/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` records 32 inputs, state in initial.json/before/, at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. At 140pt/margin 8, Simple/Optimized choose different eta placement; justified 240pt Optimized shrinks spaces. Tekt discards strategy (`eval/rules.rs:3070-3100`, `stdlib/structural/par.rs:131-150`, `text.rs:41`, `word_layout.rs:29`); pinned inline/mod.rs:194-196 resolves auto from justification and linebreak raw_ratio/raw_cost plus model/par Limits::SPACING_DEFAULT define shrink/costs. Adjacent styles/Literal edges/opaque content are separate probes. This establishes observed syntax/geometry, not intent. Original preparation scope-outs explain deferred implementation but are superseded only within certified preparation below.

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/{before/results.json,initial.json}` preserves exact state/sources/pinned binaries/no artificial newline: empty Text has missing typographic height, terminal blank lines wrongly extend height and Literal-only blank paragraphs do not close. `text.rs:37-86`, `html_frame.rs:72`, `line_commit.rs:280-286`, `parbreak.rs:40-85` and `page_geometry.rs:180-201` locate missing empty leaf and conflated visual/causal predicates; pinned inline/linebreak.rs:695-699 permits mandatory empty-text breakpoint, without intent/mechanics inference.

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{probe.json.gz,initial.json.gz,inventory.json}` refutes isolated 456 789 resolution and preserves source/state/binaries. Human ratification of immutable API/three optional fields is manifest 1ae7513e.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/{pagination-block,block-freeze}.json.gz` preserves state/UTC/HEAD/sources/binaries/argv/boxes for candidate `c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea` against oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3` (binary `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). It regains two IEEE200×90pt pages but loses continuation inset; corrected 70pt fixture reveals prefix orphaned before body admission. `region_progress.rs:75–76` resets margin and `paragraph_pagination.rs:69,87–105` can legitimately group three lines for widow/orphan. Prefix must accompany actual first line, not an invalid FIRSTWORD-first-page requirement or forced greedy/disabled protection. First prefix paint remains before body paint after admission, never changed eval/show realization order. Orphan prefix/lost inset/double emission/new effect/changed strategy/Block-column-subframe regression refutes correction.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Own only `01_core/src/compiler/layout/text.rs`. Effective TextStyle resolution belongs exclusively to `00_nucleo/prompts/core/compiler/layout/text/style.md`; delegate with unchanged field-specific typed/custom precedence. This owner temporarily applies style and lays out words. Preserve family/variants/weight/direction/language/variation/edges/leading/tracking/highlight/math/cramped/script provenance. Carry math_text_item independently of math, semantic_strong/emph only from morphological provenance rather than arbitrary bold/italic appearance.

Preserve script fallback sizing 0.65/baseline offsets, SmallCaps lowercase→uppercase run rendering 0.8 with temporary size restoration. Split ordinary text at Literal spaces, dispatch words/chunks through Cursor and preserve interpart space opportunities. Every Literal separator delegates to internal Space producer with Literal origin, unchanged advance/opportunities and no immediate flush; no duplicated recording logic. Restore outer style; no font I/O/installed-font selection/show reevaluation or replacement of SpaceFromNewline adapter.

Unless guard suppresses it, try isolated Text preparation through paragraph_plan before incremental loop, and emit plan only on complete certification. Remainder/SmallCaps keep prior path. Initial global certificate: Latin Text/Space/SpaceFromNewline/Empty/Sequence/typographic Styled/Strong/Emph, hyphenation off/LTR/no tracking-math-smallcaps/finite constant width/outside columns-cells-floats. Certify entire input first; any refused unit keeps old path. Hyphenation/full UAX14/links-objects-Linebreak/variable width/custom limits/remaining incremental selection remain explicitly incomplete, not state-of-art/full language optimization.

Explicit empty Text retains one zero-advance leaf with effective style/baseline position, establishing typographic height without paint/width/tracking boundary. In an already-material line it preserves word positions/text extraction. Its direct-leaf path consumes pending bibliography prefix after text_baseline::admit and before the leaf. Nonempty Text stays on word_layout, with no additional preparation/eval call.

Initialize a missing hanging scope from current chain before planning/emission; parent-provided scope wins. Preserve empty Text/logical origin/existing selection.

Ratified logical transport uses private Cursor, Arc input and exact spans prepared for the whole paragraph under certified lexical base before emission. Preserve visit-order offsets, not string matching. Sequence/Par create scope and nested Sequence inherits; checkpoint retains cursor/current span and subframe isolates/restores. Text supplies word/Literal-space subspans; Space consumes its segment even when weak; Linebreak consumes boundary; Word emits span only for exact untransformed text. Initial scope is incremental Simple outside generic cells/columns/subframes, without tracking/hyphenation/justification/smallcaps and Text newline/tab/language segmentation. Refused regions have None; old global preparation remains for no-context paragraphs. Certified RTL-base/RTL-character/opposite-override subset uses incremental without changing permitted breaks; explicitly selected Optimized is untouched. `00_nucleo/prompts/core/compiler/layout/bidi_origin.md` owns preparation/validation; each consumer performs only its own role. Uncertified constructors use None and read patterns `..`, preserving pinned origin transforms without broadening API/phase.

## Acceptance and limits

Preserve current rendered content/morphology/causal positions-pages/styles/diagnostics and documented limits under static descendant interfaces; no unapproved public signature/default/compatibility/phase change or unresolved proposal materialization. Internal ADR-0127 corrections require L0 before RED→GREEN, valid ownership/pins and restricted resealing.

Empty-text and selector corrections require causal owner tests/bilateral matrix/prior regressions/error-free build/lint, owners restricted to manifest. Language geometry/text/morphology are criteria, not PDF bytes or historical-intent inference.

Require occurrence/UTF8/restoration/parbreak tests and bilateral wraps for logical transport. Tracking corrections require RED→GREEN, bilateral text/position/line controls for both directions/signed values and preserved LTR/uniform/terminal controls, error-free build/lint and restricted resealing; all other recorded limits stay open.

Prefix controls require RED→GREEN same actual prefix/body page, continuation inset and OUTSIDE, Simple/Optimized and prior controls. Measure PDF90/150/250, RTL, prefix-only/formatted/color/regressions; build/suites/doctests/Clippy/Rustdoc/fmt/strict-reverse lineage/differential Tekt under MemoryMax12GiB/High8GiB/swap0/jobs1, without publication. Generic hanging/overrides and multiple bibliographies retain their own open scopes.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

## Source-scope refinement measurement (2026-10-21)

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/scope-refuters/measurement.json.gz`
records exact source/font bytes, UTC, HEAD, dirty state, commands, diagnostics,
PDF/SVG and source-bound prototype binary
`380224b96e7d12e15f7d8ee02ad2200d96664eb14f0bbe879fbd3eb265bd96d7`
against oracle commit bd5ebec285505889e8ff1d4f68804ab147a699f3, tree
cac836c07e4dec2a89219370428292393a3216b5, executable SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9. All six sources compile
without errors. Global-flow-tail paints joined PAB and Q at134.05884 in the
prototype, versus PA and BQ at215.79541 in the oracle. Fraction-rounding-residual
paints joined AB and Q at226.88379, versus A at8, B at36.686947 and Q at225.4.
These are source-language positions; PDF token grouping alone does not certify
source deletion. Zero-width controls retain the visible source body and fixed
sibling, including coincident A/Q; an empty zero Box has its own real baseline.

`eval/markup_stream.rs:305` adds `eval.flow.scope` to a complete lexical tail;
`layout/paragraph_prepare.rs:65–99` already positively classifies bounded text
fields/channels. Rejecting every custom delta propagates Blocked into such a
complete tail. The initial arithmetic model in
`agents/box/rounding-certificate-analysis.json` is withdrawn as a causal proof:
Python's compensated builtin sum did not model the current Rust ordered sum.
`agents/box/rounding-ordered-fold-correction.json` preserves that refutation
and a separate provisional ordered-fold analysis; it is not compiler evidence.
`lab/results/nested-fractions-repeat-2026-10-21/corrected-certificate-red/`
retains the exact input/source state, UTC, full test log and receipt. Its
corrected scope test fails at actual29.8 versus expected133.2, while fourteen
controls, including the
rounding geometry control, pass. Preserve their exact test inputs/tolerances.
No arithmetic correction follows from the refuted model or its passing control;
retain the existing normalized ratios and strict cursor<=right proof unchanged.

### Refinement decision and refuters

Expose a private effective_style_in(&StyleChain, &TextStyle) facade delegating
to the existing text/style owner's pure from_parts policy. It serves inert
certified lexical measurement without emitting content, admitting source,
changing state, choosing fonts or repeating script scaling. Existing emission
and effective_style(Layouter) callers keep their current single resolution.
Do not duplicate precedence or style formulas in this facade. This remains an
internal ADR-0127 correction with current source/evaluation phases unchanged.


## R8 — approved font-backed superscript integration

Evidence before decision: the source-bound `footnote-layout-r8-2026-10-24/before/receipt.json`,
`font-tables.json` and `red-geometry.json` under `lab/results/construction/diagnostics/`
record wrong 0.65 scaling/positive baseline and font-dependent pinned geometry at
bd5ebec285505889e8ff1d4f68804ab147a699f3. `approval.json` binds the human-approved
FontMetrics::text_superscript_metrics addition. OS/2/whole-run GSUB are text metrics;
MATH constants and fixture/font-name coefficients are not substitutes. This private
integration follows ADR-0127 continuous parity flow, superseding only superscript
scaling/feature assumptions below. Keep subscript behavior and unrelated folds.
Actual text emission uses the private actual-run resolver; install its style only
for that run and add horizontal compensation once at admission. Preserve original
Bidi spans, source identity and restoration. Empty text and ordinary prose retain
their existing paths. Do not reapply a script scale in measurement or paint.

Require actual RED→GREEN tests, fixed-pin PDF/SVG controls, unchanged geometry tolerance and complete workspace/lineage validation.


## R8 horizontal superscript compensation is paint-only

Source evidence precedes this correction: pinned inline/line.rs:458–484 applies
script compensation by Frame::translate without changing advance. The approved
metrics tuple supplies this offset; current text.rs adds it to cursor_x, which
also shifts following text. The existing real font fixtures have zero X offset,
so injected nonzero provider controls are necessary refuters, not font-name cases.
Keep logical advance, wrapping, justification and source identity unchanged;
translate only the text/highlight paint of the actual script run. This is private
formula/state correction, no new public field/API/phase (ADR-0127 continuous).
Install the actual-run horizontal offset as a scoped private paint offset and restore it with style; never add it to cursor_x. Preserve native/synthetic font size and absolute baseline shift.
Require nonzero injected offset and following-text/rollback/subframe RED→GREEN, actual-font controls and complete validation.


## R10 — source superscript runs and mandatory breaks

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,baseline-audit.json,approved-metrics.md,approval.json}` binds received R9 source/UTC/dirty stat and pinned bd5ebec2 binaries. Six of fifteen controls fail the unchanged complete physical words/pages and0.001pt gate: three literal-newline cases, Nimbus hard break, and Nimbus/Liberation mixed fallback. Pinned inline/collect.rs:191-196 includes mandatory breaks in source script text; inline/shaping.rs:970-1004,1090-1107,1167-1217 retains outer size, actual face/run and full causal coverage. This is language geometry and source preservation, not a byte/algorithm obligation. Font-specific synthetic metrics cannot be guessed from the primary face after fallback. The exact additive FontMetrics::text_superscript_runs contract was approved at SHA65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9; every other public method/field/default/phase stays unchanged.

Classification: approved public method plus private parity integration under ADR-0127. No font I/O, callbacks, dynamic evaluation or font-name coefficients in L1. Require real module RED→GREEN before runtime code, default/override/borrowed contract tests, source partition and restoration, native/synthetic/mixed-face/warm-cold/explicit-size refuters; all original controls, R8/R9 and A–H/R1–R7 replays, PDF/SVG0.001pt and full workspace/lineage/resources validation. Preserve all historical observations and unrelated received input bytes.

### Exclusive owning obligation

Validate the entire provider UTF-8 partition before emitting any run. Only script text uses the new method; ordinary text/planning remains intact. Apply returned effective style and paint-only offset once per actual run, retain source text/spans and restore all outer state. For script literal LF/CRLF, admit/flush original-flow lines through the existing linebreak owner rather than leaving multiline paint in a single word. Flush with the original outer line style, restore temporary bidi cursor around generated internal boundaries, and retain consecutive/initial/terminal empty-line causes. Invalid partitions preserve the previous single-run path, without partial admission or lost/duplicated text.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

For adjacent authenticated provider ranges on the same material line, delegate any generated style-change tracking interval to text_tracking once, before normal word admission; do not charge it across a source CR/LF. Preserve ordinary equal-style boundaries. Own a private one-boundary pending literal-script-break cause: generated LF sets it; actual following text consumes it; a static before-content observer retains it only through transparent Sequence/Styled/Strong/Emph/Source/Located/Realized/ShowRecipe/Empty and Space carriers, clearing it at every other original constructor. The hub only delegates this observer. Do not create an empty painted/semantic strut merely to retain a following space.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.
