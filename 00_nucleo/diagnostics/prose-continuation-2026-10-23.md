# Prose continuation — batch C, 2026-10-23

**Subsequent R2 update:** the [explicit-hyphenation correction](hyphenation-continuation-r2-2026-10-23.md) resolves the two original supplemental `hyphenate:true`/zero-tracking cases. The C measurements below remain historical; additional justification, word-boundary and tracking limits are recorded in R2.

The measured cause is an incomplete preparation certificate: ASCII colon was
rejected, bypassing the existing widow/orphan grouping for otherwise ordinary
Latin prose. The correction admits colon as part of a measured word; it changes
neither the pagination algorithm nor page counts through compensation.

## Evidence and source identity

All evidence is under
[`lab/results/prose-continuation-2026-10-23`](../../lab/results/prose-continuation-2026-10-23/).
HEAD is `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**.
`baseline.json`, `baseline-source.json.gz`, `red-source.json.gz`,
`source-final.json.gz` and `final-validation.json` retain exact dirty stat,
source/config hashes, UTC and commands. The baseline archive was recovered
from the frozen B source after a measurement-worker rerun overwrote its first
metadata copy; unchanged inputs were verified against that archive.
`environment.json` records that recovery, failed observer attempts, font hashes,
toolchain and the one-job, 12GiB maximum/8GiB high/zero-swap resource limits.

The pre-C binary is frozen B, SHA-256
`093a5d9984d21f4d9d9f1b75d60491e81757be3c85b1970341f11abcb1b2dc53`;
its build/source receipt remains under `init-output-2026-10-23`.
The oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
No upstream resync occurred. The corrected executable is frozen as `tekt-debug`;
its SHA and source-bound build are in `final-validation.json`.

## Causal reduction and correction

`before/receipt.json` reproduces the original full prose and reduces its first
incorrect transition to the first seven paragraphs. The fourth page holds an
extra penultimate line, leaving one widow on page five. The minimal four-word
case gives 3+1 lines versus vanilla 2+2. Replacing only the textual colon with a
period yields 2+2 in both and matching full-prose page membership.

`paragraph_prepare::text` rejected colon before preparation/selection, so
`paragraph_pagination` never received the complete selected paragraph.
The pinned oracle's `flow/collect.rs:196–229` protects the last two lines.
This is language-visible continuation, not a demand for upstream Rust structure.

The exclusive owning L0 was updated before tests and implementation:
[`paragraph_prepare`](../prompts/core/compiler/layout/paragraph_prepare.md).
Two new tests actually failed (`red.log`, exit 101): colon preparation failed,
and page membership was 3+1 instead of 2+2. The production change adds `:` to the
existing Latin punctuation certificate. The same tests then pass, including
zero widow cost retaining 3+1, fragment adjacency, internal/numeric colons,
exact text and non-Latin refusal. ADR-0127 continuous flow applies; no public
contract, default policy, pipeline phase or compatibility change was introduced.
Only this consumer and its owning L0 differ from the preserved B source;
`change-vs-dirty-baseline.patch` isolates those changes. Nucleus pins are intact.

## Validation and limits

`after/receipt.json` qualifies complete text and line/page membership in all
16 primary bilateral cases: original prose, seven-paragraph reduction, period
controls, attached/internal/numeric/standalone/adjacent colons, zero costs,
leading, paragraph spacing, small-page overflow and Simple/Optimized selection.
The original prose retains all 1,800 words on 12 pages, with the vanilla word
membership on every page. `geometry.json` retains every complete SVG glyph
origin: maximum delta 1.0000036354540498e-9pt, below the unchanged 0.001pt bound.
PDF xMin/yMin/yMax also pass that bound. Eight prior weak/strong/page-run,
unpainted-material, position and context controls pass in
`prose-layout-preservation/receipt.json`.

The first complete attempt passed tests and Clippy but failed rustfmt on the
widened punctuation expression. `candidate1/` preserves its binary, source,
measurements and failure logs. Rustfmt was applied, whitespace-only equivalence
was checked, reverse lineage was resealed, and complete validation was restarted.
The final `completion.json` and `final-validation.json` certify 9,836 passing
Rust tests (9,576 domain/infra, 255 wiring, five doctests), zero failures and
seven existing ignored tests. Python CI, workspace and all-target builds,
Clippy with `-D warnings -D unfulfilled_lint_expectations`, rustfmt, rustdoc with
`-D warnings`, structural checks, strict and reverse lineage, full
`crystalline-lint .` and `git diff --check` pass. Reverse lineage covers 1,003
consumers with zero errors; full lint has zero errors/warnings, retaining its
existing informational notes. All frozen productive/L0/config inputs remained
unchanged throughout final validation. No commit, push or upstream resync was
performed.

The final frozen executable SHA-256 is
`c63a1efdab3870f883c7a8511afccabc4fcb92a1e354151dabf7253571af4fe6`.
Supplemental controls were measured on candidate1; archived source comparison
confirms that the final source differs only in whitespace. The primary 16
bilateral cases, geometry and eight preservation controls were all rerun on the
final executable.

The PDF-box comparison retains the original 0.001pt threshold. The unchanged
period control already has a 0.010227pt maximum xMax discrepancy. That failure
must remain visible alongside independent complete SVG glyph-origin comparison;
correct continuation does not close batch F's PDF-derived-width precision debt.
All 16 PDF full-box comparisons still fail xMax (maximum 0.030941pt in the
small-word controls). `width-preservation.json` proves every global word's
derived width is exactly unchanged from the frozen baseline, after pairing only
identical complete word sequences. No tolerance was raised and no coordinate
discarded from the raw receipts.

Three supplemental controls preserve identical Tekt text and every PDF word
box before/after (`candidate1/supplemental-preservation.json`). Bold preserves bilateral
line/page membership. Hyphenate=true and hyphenate=true/tracking=0pt retain a
pre-existing mismatch: `paragraph_hyphen::prepare` still rejects colon-bearing
suffixes, returning to incremental layout. That explicit prior certificate
limit is preserved, not counted as bilateral success or newly closed debt.

This correction certifies colon only inside the existing preparation subset.
Arbitrary punctuation, variable metrics and refused incremental/column/note
contexts are not newly qualified. Batches D–H remain separate work.
