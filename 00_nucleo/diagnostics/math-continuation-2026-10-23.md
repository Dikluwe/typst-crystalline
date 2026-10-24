# Display mathematics continuation — batch D, 2026-10-23

**Subsequent R3 update:** [R3 geometry correction](math-geometry-r3-2026-10-23.md) resolves the seven additional geometry differences across all 19 original render probes, with full PDF/SVG checks at 0.001 pt and preservation of A–H/R1/R2. Measurements below remain their historical snapshot; additional baseline limits are documented in R3.

The measured continuation defect is in the handoff of a display equation's
already measured extent to whole-line admission. The block's full descent was
missing there, and migration installed a text strut instead of its raw ascent.
After migration, equation spacing and numbering also retained an old baseline.

## Evidence and source identity

Evidence is retained under
[`lab/results/math-continuation-2026-10-23`](../../lab/results/math-continuation-2026-10-23/).
HEAD is `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**.
`baseline.json`, `baseline-source.json.gz`, both RED archives,
`source-final.json.gz` and `final-validation.json` retain exact source/config,
UTC, dirty stat, command and executable provenance. `environment.json` records
font bytes, toolchain, failed attempts and resource limits. Heavy jobs run
sequentially with one Cargo job, 12GiB maximum, 8GiB high and no swap.

The pre-D executable is frozen C, SHA-256
`c63a1efdab3870f883c7a8511afccabc4fcb92a1e354151dabf7253571af4fe6`;
its source/build receipt remains under `prose-continuation-2026-10-23`.
The unchanged oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
The corrected binary is frozen as `tekt-debug`; its SHA is in the final receipt.

## Reduction, specification and correction

`before/receipt.json` reproduces 35 expressions on six pages, distributed
6/7/6/6/6/4 versus vanilla 6/6/6/6/6/5. The first expression on page two is
6.72358pt too high according to the PDF observer. Thirteen expressions retain
that transition; four simple fractions on 100pt pages give 3+1 versus 2+2.
The single-expression and first-page controls preserve the mathematical body.
A separate explicit-break control reuses the old region's baseline despite
correct words/pages. These are language geometry and continuation defects,
not a requirement to reproduce upstream data structures.

The exclusive [equation L0](../prompts/core/compiler/layout/equation.md) was
updated before code. The owner now submits the raw block ascent/descent to the
existing line aggregator and records the ascent actually installed. Inline
normalization by 0.7 leading is unchanged. After closure, block spacing and
number placement use the settled baseline even if closure migrated the formula.
An actually empty root region uses its geometric top; existing physical-empty
witnesses preserve unpainted/deferred material and deliberate vertical flow.
No duplicate admission algorithm, new public API/state, glyph rescanning or
body/callback re-execution was introduced. The existing installed-line-ascent
Nucleus is pinned in canonical order; its bytes and all other owners remain
unchanged. ADR-0127 continuous flow applies.

Both initial RED tests failed. The first candidate fixed frame bounds, but its
following-text test incorrectly assumed 12pt while the lexical default was
11pt. `candidate1/` preserves that failure. The fixture now explicitly sets
12pt, retaining the 22.8pt expectation and tolerance. Both corrected tests were
rerun against the exact old productive source and failed, then passed with the
correction. `red-explicit-size*` retains this genuine RED. `candidate2-pre-pin/`
retains the passing 16-case precursor before the shared Nucleus pin was added.

## Validation and limits

The final 16-case matrix matches the oracle's complete text and line/page
membership, including all 35 original expressions at 6/6/6/6/6/5. It covers
fraction/root combinations, inline math, numbering, leading, explicit gaps,
text boundaries, explicit page breaks and oversized frames. `geometry.json`
compares all SVG glyph origins and rule endpoints/widths at the unchanged
0.001pt bound: maximum deltas are 1.000017846308765e-9pt and
5.684341886080802e-14pt. The PDF xMin/yMin/yMax pass in all cases; full boxes
pass in 15 cases.

The prefix-text control still fails xMax at 0.002191pt. Its BEFORE/AFTER text
carriers retain exactly the same derived widths before and after the fix;
`prefix-width-preservation.json` records the original failure and pairing.
This remains separate PDF precision debt, not a passed or relaxed comparison.
All coordinates are retained.

Sixteen prose line/page controls and eight weak/strong/page-run, unpainted
material, position and context controls pass under `prose-preservation/` and
`layout-preservation/`.

`completion.json` and `final-validation.json` certify 9,838 passing Rust tests
(9,578 domain/infra, 255 wiring, five doctests), zero failures and seven existing
ignored tests. Python CI, workspace and all-target builds, Clippy with
`-D warnings -D unfulfilled_lint_expectations`, rustfmt, rustdoc with
`-D warnings`, structural checks, strict and reverse lineage, full
`crystalline-lint .` and `git diff --check` pass. Reverse lineage covers 1,003
consumers with zero errors. Full lint has zero errors/warnings; informational
notes remain visible in its full log. Frozen productive/L0/config inputs stayed
unchanged throughout final validation. Only equation.rs and its exclusive L0
differ from the preserved productive baseline; `change-vs-dirty-baseline.patch`
isolates this batch. No commit, push or upstream resync was performed.

The final frozen executable SHA-256 is
`7d31f0f8c49d5797dc4e57211ff551dd5bb89211f1a7bb2059736d8740be3daf`.
The final source body is exactly equal to the passing pre-pin candidate after
removing only the forward lineage header; every primary/preservation case was
nevertheless rerun on the final executable. The original math fixture's first
page and the single-expression control preserve every PDF word box exactly
against the frozen pre-D product.

The scope is whole display frames and their existing line/region protocols.
General splitting of one multiline equation, new layout phases, new math
constructors and unrelated symbol-carrier or PDF precision debts are not
certified by this correction. Existing limits from batches C and F remain
separate and must not be counted as newly passing tests.
