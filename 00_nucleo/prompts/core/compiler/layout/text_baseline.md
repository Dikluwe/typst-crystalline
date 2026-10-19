# L0 — text_baseline
Hash do Código: 1992d265

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_baseline.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Inspection of `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` and `space.rs:162` found, respectively, a textual boundary, baseline admission and an initial-space integration test owned by other responsibilities. Exact state, sources and prior prompts are in `lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json` and `before/`. The previously measured correction is in `lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`; this separation preserves its results and limits. Positions, text and styles are language observables; function placement is mechanical. Inspection establishes neither historical intent nor new bilateral parity.

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/after/results.json` measures two 12pt space paragraphs at 30.206pt Tekt versus 31.536pt vanilla; size-probe.log for 10–18pt grows 1.914pt/pt versus 2.628pt/pt. `text_baseline.rs:14-16` admits through restored outer layouter.style, although Literal carriers retain effective style. This supports a carrier-style Literal-only admission helper, not a new state/global.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

mixed-style-metrics-2026-10-18 first-after/first-bib-after/first-regions-after/candidate1-sources preserve first candidate changing only line_commit with seven passing tests. audit-candidate1 finds matching SVG heading y in six bibliography cases and italic→roman, but forced still differs 0.156. linebreak.rs:46–58 appends ExplicitLinebreakBoundary with empty exterior-style Text: causal for bidi, not material face. Twenty regional PDF controls preserve same binaries/pin/sources as before-sources: at 39.44pt Tekt sends ITALIC to page 2 versus vanilla page 1;55.14pt forced 2 versus 1 pages;70pt wrap 3+1 versus 2+2 lines. Initial fix corrects wrapped y but not regional decisions. Invalid multipage SVG-without-template attempt stays in regions-before as a failed command, not SVG parity proof; use valid PDF and future regional script requests PDF only.

region_progress.rs:83–88 installs one-face baseline without updating assumed ascent;`:143–152` admits forecast before actual text. paragraph_pagination.rs:43–59 rejects differing edges and disables positive-cost grouping despite known selected ranges. Pinned flow/collect.rs:196–239 protects first/last actual-height frames; distribute.rs:343–364 advances group only if it fits next region. Language geometry/words/pages, not Profile shape/SVG IDs. Premature jump/recount/new callbacks/Group-based metrics/width-selection-leading changes or column/note/float/NoTypography regression refute correction.

## Exclusive ownership and current contract

Own parent-private admit(layouter) only. Before admission record whether line is nonempty but lacks visual material according to html_frame::items_have_material. Request textual baseline once through private ensure_text_baseline facade, the later effective-text refinement of the prior ensure_initial_baseline call. Only when carriers existed without material, align each TextSeparator y to admitted cursor_y plus baseline_offset resolved with that carrier's own style size; preserve x/advance/style/origin/order/other items.

Word/Chunk call admit before emission and again after wrap. Space alone creates no vertical demand. Prepared path retains its own admission without introducing this separator alignment/tracking. This owner owns no regional-admission/line-close/text-measure formulas; existing adapters retain them. Initial-space test remains here exercising real Space/Word.

A private Literal-only line helper admits with largest-size Literal carrier style, restores exterior style afterwards and reuses admit without new state/global. Mixed content keeps Word/Chunk path. Later mixed-face refinement does not alter this Literal-only path.

After vertical admission, apply pending horizontal origin once, shifting earlier Literal separators by same delta without vertical demand. New region reopens application without restarting logical line. No local metrics calculation or changed indent-application/admission point.

## Acceptance, certification and limits

Preserve causal tests and pre-separation text/positions/styles/pages/diagnostics,1:1 ownership/valid pins/error-free build/lint/static descendant movement. Existing mixed-style/RTL/global-selection/cluster-counting tracking limits stay in correction diagnostics, refined only by their own owners.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

Mixed-face refinement is private continuous-flow ADR-0127 without public/product-flag/default/phase/callback-order change. Preserve prior refused/scalar paths. Regional certificate expands prepared text only at same size/leading, zero baseline_offset, finite nonnegative edges and faces with different tops/descents; variable sizes/leading remain outside certificate, without complete-parity claim. Require RED→GREEN for four added regressions/prior controls,30+20 regional PDF matrix,132+94+21 corpus, broad suite/Clippy/Rustdoc/lineage under 12GiB/High8GiB/swap0/jobs1. Record failed attempts rather than hiding them.
