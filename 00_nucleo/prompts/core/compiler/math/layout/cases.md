# L0 — math/layout/cases
Hash do Código: 4a2a200a


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/cases.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/cases.rs:23` locates `pub(super) fn layout_cases(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Cases carrier and rows

Consume complete MathCasesElem, forming one row for each child in order before
late splitting at MathAlignPoint. Preserve children and public identity.
Resolve delimiter pair, reverse and gap independently by explicit >
math.cases StyleChain > defaults, without setting presence bits during layout.
Resolve relative gap against the outer style's font size.

Children use shared denominator style (Display→Text without size reduction,
then discrete script descent, always cramped).
Reverse=false uses only the optional left delimiter; true uses only the
optional right delimiter. None omits the glyph, not a NUL stand-in in the
public carrier. The common grid helper owns row/column placement and the
matrix module provides the existing alignment-boundary helpers; neither
takes ownership of cases behavior.

Use natural class spacing at ampersand boundaries, not extra column gaps.
The grid's 1.1 height target and shortfall determine delimiter selection.
Center the grid on the axis before merging the already-centered delimiter.
Do not insert redundant 0.1em horizontal padding or top-align components.
Final extents include both grid and delimiter maxima.

Acceptance covers row order, late alignment splitting, denominator style,
both orientations, optional sides, relative gap and explicit/cascade/default
precedence, with no public carrier mutation.

## Semantic delimiter identity

The .typ/sec_09.typ probe in typs-teste-2026-09-30 extracts an assembled brace as
construction glyphs (`⎧`, `⎨`, `⎩` and bar), although its language sign is the
opening brace. When composing cases, retain the delimiter's visual items in
one TextReplacement with the character requested by the carrier. Do not alter
shared selection/stretching policy or grid geometry.

## Unused local wrapper boundary

### Evidence before decision

compiler/math/layout/cases.rs:23 contains the historical Clippy inventory item.
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
retains sources, owners, references, UTC, status and diffstat. Call-site research
and productive paths precede classification; this is neither new parity nor
performance evidence.

### Contract and acceptance

Compile internal layout_cases only under `#[cfg(test)]`, preserving signature
and body for existing tests. Productive dispatch uses layout_cases_elem; the
rows/delim/reverse/gap wrapper is test-only. Preserve productive routes, existing
test names/bodies, external APIs, defaults and phase. ADR-0127 internal continuous
flow applies. Revalidate all workspace-target build/check, core suite, test
inventory, Clippy and lineage; do not remove expectations to pass the gate.
