# L0 — space
Hash do Código: 0037b18e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Inspection of `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` and `space.rs:162` found, respectively, a textual boundary, baseline admission and an initial-space integration test owned by other responsibilities. Exact state, sources and prior prompts are in `lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json` and `before/`. The previously measured correction is in `lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`; this separation preserves its results and limits. Positions, text and styles are language observables; function placement is mechanical. Inspection establishes neither historical intent nor new bilateral parity.

`lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/{before-pdf/results.json,summary.json,initial.json,before-pdf/provenance.json}` preserves 30-source/90-PDF state/commands/sources and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Repeated/nested equal-looking scopes and Hide/Box expose missing lexical producer identity, not a requirement to copy PartialEq/chain representation. `text_tracking.rs:50`, pinned inline/collect.rs:287-295/shaping.rs:1269-1295 and hide.rs:21-24/boxed.rs:157-208 locate boundaries.

`lab/results/construction/diagnostics/correcao-tracking-rtl-2026-09-29/{before-pdf/results.json,summary.json,provenance.json,initial.json}` preserves 66 inputs/198 PDFs, full source/commands/binaries/HEAD/tree against that historical pin. Explicit Latin/Arabic RTL show signed missing interunit tracking while 0pt controls discriminate independent Hebrew/initial-Literal-space defects. `text_tracking.rs:65`, `word_layout.rs:34,72`, `space.rs:25`, `cursor.rs:150-186` and pinned shaping.rs:1269-1295 locate rejection, signed application and real-right-edge anchoring. Ligatures/mixed direction/global pagination remain unmeasured; these observations do not establish intent/general RTL parity.

`lab/results/construction/diagnostics/correcao-auto-espacos-2026-09-29/{before/results.json,provenance.json,initial.json}` records 30 sources/60 PDFs/font-path/commands/UTC/HEAD/tree/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Literal-only spaces at tracking 0/±0.2 lose auto dimensions in Tekt; terminal Literal width disappears while initial-with-text matches. Empty text/inkless markup and terminal newline reveal separate unresolved height issues. `space.rs:22-35` lacks initial Literal 0 carrier; page_geometry.rs:104-135 uses visual line_content_right excluding separators for correct RTL alignment; builder/exporter owns physical PDF minimum 3×3. This justifies separate causal auto extent, not a changed RTL ink bound or inferred historical intent.

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{probe.json.gz,initial.json.gz,inventory.json}` refutes isolated 456 789 resolution and preserves source/state/binaries. Human ratification of immutable API/three optional fields is manifest 1ae7513e.

verification.json.gz records 3.4966pt opposite-edge drift in rtl_terminal_space/rtl_wrap and final DEF in ltr_wrap. Weak Space at Styled end and after wrapper both advance in space.rs:31. Pinned `typst-realize/src/spaces.rs:25–31,68–99,128–132` retains first grouped-space style. First corrected suite exposed an old mechanical assertion expecting second weak space to cause wrap; fixture now inserts another B before testing wrap, retaining break-after-material obligation. Test-only 1e-9pt tolerance covers measured~1e-14 float sum, not altered geometry.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Own layout/advance for Markup/Newline/Literal, only in `01_core/src/compiler/layout/space.rs`. Markup/Newline resolve effective style via `00_nucleo/prompts/core/compiler/layout/text/style.md`; Literal reuses resolved style. Measure nominal advance through text_measure::space.

layout suppresses initial Markup/Newline or after only direct HTML markers. Preserve logical-segment consumption even when weak/suppressed. For visual output, emit carrier when material exists, and for every Literal from its first space including tracking 0. This supersedes initial-Literal-zero no-carrier limitation only. Apply State::boundary before carrier once when prior eligible producer exists; record after emitting TextSeparator. Suppressed/no-carrier advance records no producer. Keep signed boundary in explicit RTL without reversing sign or adding an exporter interval.

TextSeparator is append-only origin/style/position/nominal advance; tracking never becomes advance or is summed again by exporter. Advance cursor by nominal width and record justification opportunity. Initial y uses cursor_y plus carrier-style baseline offset without baseline demand; `00_nucleo/prompts/core/compiler/layout/text_baseline.md` later aligns when material text arrives. Preserve resolved Literal style for auto projection, never infer space from distance/adjacent text, add globals or coded counts.

After consuming logical segment, a weak new Space following a final TextSeparator Markup/Newline does not emit/advance a duplicate visual occurrence. Preserve first surviving carrier/style/span, while omitted weak spans stay in logical input and are not reassigned. Never collapse Literal, use string equality or merge spans.

layout flushes only after crossing effective paragraph final edge when par.justify is not true; with justify true, next material unit chooses continuation. Literal advance itself does not flush immediately. Preserve origin/duplication/nominal-advance rules and never manufacture spacing for hanging indent.

Ratified logical transport uses private Cursor, Arc input and exact spans prepared for the whole paragraph under certified lexical base before emission. Preserve visit-order offsets, not string matching. Sequence/Par create scope and nested Sequence inherits; checkpoint retains cursor/current span and subframe isolates/restores. Text supplies word/Literal-space subspans; Space consumes its segment even when weak; Linebreak consumes boundary; Word emits span only for exact untransformed text. Initial scope is incremental Simple outside generic cells/columns/subframes, without tracking/hyphenation/justification/smallcaps and Text newline/tab/language segmentation. Refused regions have None; old global preparation remains for no-context paragraphs. Certified RTL-base/RTL-character/opposite-override subset uses incremental without changing permitted breaks; explicitly selected Optimized is untouched. `00_nucleo/prompts/core/compiler/layout/bidi_origin.md` owns preparation/validation; each consumer performs only its own role. Uncertified constructors use None and read patterns `..`, preserving pinned origin transforms without broadening API/phase.

## Acceptance and limits

Preserve causal tests/pre-separation text/positions/styles/pages/diagnostics,1:1 ownership/valid pins/build/lint/static descendant movement and all tracking mixed-style/global-selection/cluster limits recorded by their owner. Internal parity follows continuous-flow ADR-0127 without unapproved public/default/compatibility/phase change.

Require occurrence/UTF8/restoration/parbreak tests and bilateral wraps for logical transport. Tracking corrections require RED→GREEN, bilateral text/position/line controls for both directions/signed values and preserved LTR/uniform/terminal controls, error-free build/lint and restricted resealing; all other recorded limits stay open.

Literal 0/± and PDF-minimum RED→GREEN require actual bilateral causal/corpus controls; physical minimum remains exporter-owned and explicit-empty/terminal-newline issues are separate. Weak adjacency RED→GREEN covers Markup/Newline plus Literal controls with identical widths/fonts/scopes; preserve after-material wrap test rather than obsolete double-weak assumption. No byte-equality or general RTL claim.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

## Weak-space closure retains the actual terminal causal bottom

### Measurement before decision

The source-bound `lab/results/nested-fractions-repeat-2026-10-21/prototype-3/`
receipts show equal glyph world origins/text but auto page height43.86484375
versus pinned vanilla28.357421875 for nested fixed Repeat and unequal
fractional siblings. The exact current source/L0 and dirty HEAD/stat/UTC
are preserved in `agents/box/auto-page-height-investigation/` there.
`repeat-auto-height-red/receipt.json.gz` records two genuine source-equivalent
failures with declared metrics: actual page42.55 versus terminal baseline17.7
plus bottom margin10. The pre-finish state is cursor32.55, previous baseline17.7,
no recorded bottom, flush advance14.85, no remaining causal line and flow end
(17.7,32.55). Source inspection locates the cause at this owner's edge-crossing
weak-space flush: a full certified line plus trailing weak newline closes
before finish, leaving only a predicted following baseline as page geometry.
The same RED occurs in ordinary dispatch; it is not a physical phase defect.

### Decision and invariant

Preserve weak whitespace advance, grouping, logical consumption, edge-crossing
flush and future wrap behavior exactly. After this owner actually flushes a
causal line on a weak-space edge crossing, retain its just-committed terminal
baseline as last_block_descent_y only when the normal close did not already
publish a bottom. Preserve a normal inline descent bottom or existing closed
frame decision. Do not publish anything for marker-only or otherwise noncausal
input, and never use current cursor (the next predicted baseline) as its bottom.
Later real content closes normally and replaces this prior bottom; an earlier
weak-space closure cannot freeze subsequent auto height. No wrapper materiality,
paint, phase, baseline coefficient, public contract or comparison bound changes.

### Acceptance

Require the two source-equivalent nested/fractional Repeat cases in ordinary
and planned dispatch, subsequent actual text replacing the prior terminal
bottom, and existing after-material wrap/initial-marker/adjacent-weak/Literal
controls. Final bilateral sources, full tests, lineage and hashes remain gates.


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

An immediately pending authored literal-script LF permits exactly the next original markup separator at the fresh line origin, retaining that separator's actual Markup/Newline provenance and effective outer style. Preserve its carrier and advance without creating a baseline by itself; consume the pending cause once. Keep ordinary initial/marker-only suppression, adjacent weak collapse, signed tracking, wrapping and terminal-bottom behavior unchanged.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.
