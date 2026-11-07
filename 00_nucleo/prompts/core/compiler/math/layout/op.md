# L0 — math/layout/op
Hash do Código: 4cf12f27


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/op.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/op.rs:21` locates `pub(super) fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Text operator layout

Own only the thin MathOp text-layout delegate. Return the ordinary math layout
of its text in the supplied style. Do not rewrite textual operators into
glyph variants. The Large spacing class is owned by
`00_nucleo/prompts/core/compiler/math/layout/spacing.md`; display-only use of
MathOp.limits is owned by `00_nucleo/prompts/core/compiler/math/layout/attach.md`.
Moving or duplicating those decisions here is not part of this contract.

Acceptance checks unchanged textual morphology/style and transparent geometry;
operator classification and attachment policy are integration checks of the
respective owners, not another implementation in this module.

## Explicit text-direction regression controls — #8990 residual (2026-10-31)

Before-decision evidence is preserved in
`lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/before-state.json`
and its bilateral before receipts at HEAD 3c6b0ce7 plus exact concurrent diffstat.
An explicitly styled operator loses canonical text.dir while typed text style
uses the existing external bridge. Pinned text/mod.rs:517-543 documents explicit
ltr/rtl; math/op.rs:28 retains the supplied text Content.

Keep the existing delegate unchanged. Own inline regression tests that pass
local Styled text to this operator delegate and inspect actual emitted text
and effective direction for ltr and rtl, with both Latin and Hebrew source.
The parent owner 00_nucleo/prompts/core/compiler/math/layout/mod.md exclusively
owns the routing correction; do not copy that classification algorithm here.
Defaults, math class/limits, morphology, outer style, APIs and phases stay
unchanged. The language-derived default and native-Hebrew default policy are
not authorized here and remain at the separate human ADR-0127 gate.

## Ratified A/B direction defaults — 2026-11-01

### Measurement and approval before code

Source anchors and runtime counterexamples are preserved in
`lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/`
before receipts, pinned-documentation.json and gate-default-direction-proposal.md.
Current source/owner bytes, exact concurrent diffstat and UTC are fixed in
`lab/results/construction/diagnostics/issue-8990-ab-2026-11-01/before-state.json`.
The same causal paths still read only explicit text.dir or discard Auto; the
operator delegate does not establish a local mathematical base. Those are
language defaults/semantic bidi inputs; font IDs/IR/byte equality are mechanics.
The human explicitly approved A and B on 2026-11-01, preserved in approval.json,
for proposal SHA-256 a2a9737c7960cb400b2899ac60c3b02e04f0823f6a9cc388ceee385663f214ca.
ADR-0127 default gate is satisfied for this bounded package only.

### Exclusive responsibility

Approved B supersedes the earlier unchanged-thin-delegate direction clause only. Clone the supplied TextStyle with a local Dir::LTR for operator layout, leaving input Content and outer style unchanged. Keep math fields, TextItem provenance, class, limits, scripts and metrics. Hebrew strong runs remain RTL within the local LTR base. Inner explicit Dir or Auto wins through the existing Styled/external bridge; outside document direction must not replace the operator default. Do not insert Unicode controls, change glyphs, wrap all content into new public elements or modify pipeline phases.

### Acceptance and limits

Require tests first and functional RED before implementation, then GREEN;
measure he/ar/en/absent language, explicit LTR/RTL, Auto reset, inherited
language and scope restoration, geometry/paint and existing RTL #8990 controls.
Keep the six prior physical RTL-Hebrew punctuation negatives separate; this
package does not ratify a new fallback-font policy or general RTL mathematics.
Revalidate core/math/shaper tests, RAM-only build, lint and paired lineage,
with exact source/UTC/diffstat provenance and restricted resealing after a
valid V15/V26 graph. Preserve all concurrent changes and every other owner
responsibility; no commit/push, new public API, Nucleus or phase is authorized.
