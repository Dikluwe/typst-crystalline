# R7 — Legacy symbol identity gate — 2026-10-24

## Measurement before decision

The [received live probe](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/received-probe.json)
exits 1: 198 cases, 162 MATCH, 24 EXTENSION and 12 EXTENSION_DRIFT.
All twelve bare legacy extensions already expose native `func=symbol` with the
expected glyphs and warnings. The obsolete classifier required `func=text` in
`lab/parity/tools/p1495_symbols_probe.py`; productive identity is `symbol` in
`01_core/src/entities/content/identity.rs:181`, and symbol-producing math returns
`Content::Symbol` in `01_core/src/compiler/eval/math/expression.rs:179`.

The audited [content L0](../prompts/core/entities/content.md),
[math expression L0](../prompts/core/compiler/eval/math/expression.md),
[identity L0](../prompts/core/entities/content/identity.md),
[observation L0](../prompts/core/compiler/eval/observation/content.md) and
[shared Symbol Nucleus](../prompts/_nuclei/content/symbol-identity.toml) already
require native identity, approved in batch E. The [classification receipt](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/classification.json)
records their exact hashes, source evidence and passing V5/V15/V26 audit.

Decision: repair the diagnostic extension expectation only, from `text` to
`symbol`. This is measurement-policy maintenance against existing L0, not a
product contract/default/phase/compatibility change (ADR-0127). `lab` is excluded
by `crystalline.toml`; creating a materializable L0 for this diagnostic Python
would violate productive 1:1 ownership (ADR-0129). Productive L0 and seals remain
current and unchanged. Glyphs, qualified representation, deprecation warnings,
expected oracle rejection, extension classification and all other gates stay.

## RED → GREEN and live verification

Eight binary-independent classifier tests use literal witnesses for all twelve
legacy glyphs. The [actual RED](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/red-tests.json)
records 52 assertion/subtest failures, with no harness errors. The
[GREEN](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/green-tests.json)
passes the same eight methods with identical test-source hash. Negative controls
still reject Text with the same glyph, missing/malformed/wrong payloads, altered
warnings, failed product results and unexpected oracle outcomes. Ordinary parity
still compares full native identity; qualified extensions retain their repr and
namespace-specific warning policy.

Fresh [release](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/symbols-release.json)
and [debug](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/symbols-debug.json)
probes each exit 0: **162 MATCH, 36 EXTENSION, 198 cases**. Every raw oracle and
Crystalline observation is identical to the received run; exactly twelve
`EXTENSION_DRIFT` verdicts become `EXTENSION`, never MATCH. These are `join` and
its three modifiers, and the four `gt.tri` and four `lt.tri` forms. Their absence
from vanilla remains explicit. Historical exit-1 receipts are preserved.

## Provenance and preservation

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, branch
`codex/tekt-mature-layout`, **uncommitted working tree**. The
[received baseline](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/baseline.json)
records UTC, exact `git diff HEAD --stat`, status and all 2162 productive/config/L0
hashes. The [received source archive](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/received-source.json.gz)
retains their bytes. The [reuse receipt](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/reuse-receipt.json)
binds the three changed lab files and the unchanged R6 binaries to the actual R6
build receipt and source archive, copied verbatim. This is artifact reuse, not a
new R7 release compilation. Fresh workspace compilation is validated separately.

Release SHA-256 `6ccf6b10b1dfb7d4443dda2ce9dd4d82837264e8fec2fdd1aec7762ba5f61d86`;
debug `0ff7d7dc689a71f5caea518f7fc6f8a676251fc1112f070f69ba4e6a60c9caaa`.
Oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
No oracle resync occurred. The [execution receipt](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/complete-stages.json)
records exact commands, timestamps, worker/log hashes and source guards.

R7 changes only the classifier, its CI tests, CI documentation and current
diagnostic summaries. It does not close prior note-layout, hyphenation-boundary,
additional math/constructor, painting or resource limits. R6 performance results
remain attributed to R6; identical product bytes do not constitute a new R7
performance improvement.

## Complete validation

The fresh [12-gate validation](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/validation-complete/validation-commands.json)
passes: 42 Python tests; layout and strict/reverse lineage; workspace build;
domain/infra and wiring tests; doctests; all-target build; Clippy with
`-D warnings -D unfulfilled_lint_expectations`; formatting; Rustdoc with
`-D warnings` and all four crate indexes. Rust totals are **9878 passed,
7 ignored, zero failed**. The
[source freeze](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/validation-complete/validation-freeze.json)
records no changed inputs. Cargo ran offline/locked on Rust 1.92.0, sequentially,
with one job. The [effective resource caps](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/effective-caps.txt)
are MemoryMax 12 GiB, MemoryHigh 8 GiB and swap zero.

All [21 fresh preservation stages](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/replays.json)
pass across A–H/R1–R5, including complete PDF words/bounds, symbol reflection,
HTML stdout, full font inventories, normalized executable size, mathematical
rendering, PDF/SVG painting, compact output and refused controls. R6 resource
changes are preserved through exact source and executable identity. Full
`crystalline-lint . --fail-on warning` and `git diff --check` also exit 0.

The additional [debug controls and final audit](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/post-stages.json)
also pass. The [preservation audit](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/audit.json)
confirms all 2162 productive/config/L0 files and 1853 font/reference assets are
unchanged, including known residual classifications. Its source revisions and
all measurement workers are archived; timestamps and hashes distinguish every
measurement state from the received dirty tree.

Received state: `2026-10-24T12:24:54.368773+00:00`. Final executable/acceptance audit completed: `2026-10-24T12:48:03.119879+00:00`. The final completion receipt records the documentation seal separately.

The [final completion receipt](../../lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/completion.json)
seals current documents and evidence. All 8524 inherited R6 evidence files retain
their original hashes and sizes. The initial sealing adapter compared manifest
objects to digest strings; the corrected object-field check and the initial
attempt are retained in the evidence directory. All product acceptance and
validation results were already passing before that receipt-parser correction.
