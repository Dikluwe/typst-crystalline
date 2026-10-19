# L0 — paragraph_indent
Hash do Código: f47f0cdc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_indent.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Evidence before specification

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

First functional RED→implementation leaves RTL Spec.dir LTR: cursor receives right100 while paragraph.rs:427–436 transports text.dir in custom chain rather than isolated TextStyle::from. Full lexical-trunk Value::Dir must supply direction, with typed/LTR fallback. Checkpoint controls must capture a committed drained-line boundary as suffix protocol does, not demand recovery of a prefix already moved between buffers by an artificial transaction; retain prefix-text proof/discarded-attempt exclusion/restored continuation indent.

The 12 SVG decoration pairs in bibliography-hanging-2026-10-18/deco-before.json.gz with corrected deco-inspection.json line/path read use the initial 9026-suite freeze/pinnedbd5ebec oracle. Overline h10 has second-line text x18/stroke x8 versus vanilla stroke x18; h0 already differs vertically/thickness/tail. decorations.rs:43–45,76–78, line_commit.rs:615–619 and sub_frame.rs:350–354 use global line starts. This measures horizontal language geometry, not required vanilla buffers. Negative preview can precede actual cursor before material; end>preview alone proves no segment. Double origin/early compensation consumption/baseline-end-thickness-text-region changes outside horizontal delta refute correction.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/paragraph_indent.rs`. Own private Copy Spec(initial/continued offset, Dir), local Paragraph/Inline context State with scope/line application/commit, and pure Shape(initial/continued widths). Shared nucleus supplies invariants. No I/O/eval/callback/location/public Content mutation.

Scope is complete lexical trunk: factor only a wrapper covering the entire substantive set, traversing Source/Located/Realized via already-ready view. Multiple siblings do not form value consensus. Resolve em through trunk TextStyle and Dir through trunk custom text.dir Value::Dir with typed/LTR fallback, never partial-run direction. Partial par.* declaration interrupts through `00_nucleo/prompts/core/compiler/layout/style_scope.md`; it is not a typographic fragment within a larger global preparation.

Classify container fragment through closed Content: Par/Parbreak/flow objects require Paragraph, simple inline does not. Recurse Sequence/Styled/transparent envelopes; Box/inline math/raw are exterior inline units with own body, Raw/Equation/Quote use block flag. Do not execute Dynamic/ContextBlock/ShowRecipe to predict form. Opaque input does not certify paragraph and uses explicit admission when materialized. Exhaustive nominal matches retain every variant distinction, no silent catchall; groups are deliberate layout classes, not all-alternative test proof.

Generic Paragraph uses Spec(0,h), Fragment uses Spec(0,0). CSL hanging may provide pending Spec(-h,0) compensation inside already-installed h inset. Read without consuming during prepare/select; consume when material unit receives origin. Retain initial shape until first logical line commits even after regional admission. No HSpace/text/Par/tags/Location. After first line, normal inset bounds retain all continuations/body substructures. Auxiliary subframes suspend exterior queue; checkpoint preserves it.

Apply cursor once per line from directional initial edge, moving initial Literal carriers by same horizontal delta as needed. Query effective bounds without rewriting global line_start_x/width. Pass initial/continued widths to both selectors. Physical break commits line, paragraph boundary clears scope, new region only reopens application. Nonmaterial presence creates no height. Negative/extreme offsets remain real, without clamping away semantics or first-overlong progress.

Dispatcher hooks remain thin: origin only for inline layout units; finalize scope after confirmed flow boundary or explicit pagebreak/colbreak. Classification/formula/policy stay here. No paint-based repositioning of semantic/Location origins.

Private read-only cursor_start previews only still-unapplied Spec/compensation delta, without scope creation/queue consumption/material admission. RTL gets no physical cursor delta and retains existing alignment; no-Spec geometric fallback is zero. Provide private read-only applied query for decorations. Decoration owner requires actual applied origin for final preview-shifted segment when none flushed; do not create scope/admission/paint/height to satisfy empty body.

## Acceptance and explicit limits

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin 1279707, under 12GiB/High 8GiB/swap 0/jobs 1. No publication.

Before implementation add real functional RED through current APIs, not a missing future helper. Cover set/constructor, zero/negative/absolute+em/partial fonts, first/continued lines, repeated/empty boundaries, Simple/Optimized/hyphenation, RTL, inline/Par containers, pages/checkpoint/subframe and bibliography/default/override. Then discriminate shape/cost inputs rather than mirror helper arms. Preserve logical text in extreme frames; cropped PDF proves no deletion/copy. Repeat prior corpus/bilateral matrix and declare prior metrics/realization debt open, without relaxed controls. V15/V26 precede reseal/implementation, full gates/12GiB/no publication; whole objective/R3-T1 remain open.

Decoration tests cover all three origins/signs, entry after break/subframe and empty body/metadata/empty Par with negative h. Preserve previous vertical differences; horizontal refinement certifies LTR only and full decoration/RTL geometry remains unmeasured. Keep original checkpoint boundary semantics while proving prefix text/discard/restored continuation.
