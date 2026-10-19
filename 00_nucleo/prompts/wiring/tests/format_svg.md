# L0 — wiring/tests/format_svg
Hash do Código: 1da30030

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/format_svg.rs`

## Current evidence

- `04_wiring/tests/format_svg.rs:20` — `TempDir`.
- `04_wiring/tests/format_svg.rs:25` — `new`.
- `04_wiring/tests/format_svg.rs:42` — `drop`.
- `04_wiring/tests/format_svg.rs:48` — `ProcessResult`.
- `04_wiring/tests/format_svg.rs:55` — `Compilation`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/format_svg.rs`.

## Owned contract

- Own format.svg.pretty external-process tests: compact default, true/false/root/empty-rule cascade, explicit caller precedence, rejected containers/unknown named/type/direct construction.
- Check valid XML and unchanged semantic morphology between compact/pretty, without complete SVG hashes. Keep PNG/PDF/HTML/frame neutrality.
- Without html, sibling format.html produces its public feature gate; with html, only existence belongs here and its detailed surface belongs to `00_nucleo/prompts/wiring/tests/format_html.md`. Use isolated bounded fixtures; no administrative oracles.

## Verification and boundary

Run `cargo test -p typst-wiring --test format_svg -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Internal observation expressions

`lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz`
retains sources/L0/HEAD/uncommitted tree/status/diffstat/UTC;
initial-diagnostics.json retains suggestions/spans at format_svg.rs:454
(clippy::sliced_string_as_bytes) and :515 (clippy::question_mark). Uses/types were
inspected first. This is internal Rust representation, no new parity/performance.

Use &source.as_bytes()[start..end] in the XML observer: ASCII-tag find operations
on UTF-8 str supply valid boundaries. Preserve bounds/errors. Use ? only for
artifact extraction after require_success, retaining failure recording and None.
Continuous ADR-0127: no public/default/phase/dependency/suppression change. L0 first,
scoped resealing, unchanged inputs/results/tolerances; selected-lint RED→GREEN,
workspace suite/build/all-target Clippy/fmt/Rustdoc/lineage. No syntax-mirror tests.

## Harness boundary

### Evidence

`lab/results/construction/diagnostics/tekt-v12-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC,
sources/L0/configuration/lock/hashes/status/diffstat. Its 2077 hashes match the
final FrameItem checkpoint: the prior suite ran 8622 tests with zero failures
and seven ignored at that same state. gate-baseline-result.json records real RED
for 19 V12 before L0/code changes. items.json contains complete AST items and
classification.json records responsibility per unit. These are historical
measurements, not new parity or upstream-intent claims.

The linter identifies top-level enums/inherent impls in L4, including test support.
Scenarios, language fixtures and final product assertions stay in L4; independent
subprocess/filesystem/observation helpers belong to L3 test support. Do not move
the whole suite, hide declarations inside an L4 module to exploit shallow parsing,
exclude tests from lint, convert impls solely to evade the rule or change severity.

### Exclusive responsibility

Use `03_infra/tests/support/wiring/format_svg.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/format_svg.md`, for TempDir, Checks, their existing impls and Drop.
Keep all scenarios and final observations in this L4 consumer. Import only
Checks, TempDir through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/format_svg.rs:18 static_item NEXT_TEMP; 04_wiring/tests/format_svg.rs:20 struct_item TempDir; 04_wiring/tests/format_svg.rs:24 impl_item TempDir; 04_wiring/tests/format_svg.rs:41 impl_item TempDir; 04_wiring/tests/format_svg.rs:63 struct_item Checks; 04_wiring/tests/format_svg.rs:67 impl_item Checks.

### Acceptance and limits

This internal ADR-0127 harness separation does not put support into the L3
production library or CLI. pub(super) grants access only inside the test crate.
No public product API/dependency/flag/default/phase changes. The separately
ratified Published contract grants no other contracts. No new derives,
suppressions, hooks, cleanup policy, tolerance or verification fallback.

Preserve moved item bodies except minimum visibility for the parent test module;
Checks fields remain private where present. L3 owns its std imports, with no
use super::* or dependency on L4 logic. BIN, subprocess invocation/options,
scenarios/expectations and #[test] remain in L4; support does not read
CARGO_BIN_EXE_typst. Retain legitimate per-harness differences in prefixes,
messages, creation atomicity and cleanup.

Check restricted transfer replay, identical test functions/attributes/expectations,
valid 1:1 graph, no support in the productive library, V12 GREEN zero and complete
SARIF without new warnings. The original batch required workspace suite with
explicit /dev/shm TMPDIR and historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
That historical receipt does not certify the current pin; subsequent runs use the
ratified current oracle after independent recipe/source/tree/SHA verification.
Preserve all expectations/tolerances. Validate cargo build, workspace/all-target
Clippy -D warnings/-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings/four
indexes, strict V1/V5/V15/V26/reverse lineage and diff --check. L0 first; resealing
limited to authorized owners/code. Heavy jobs sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1; freeze sources/L0/configuration during commands.
No import-syntax mirror tests, unmeasured RAM/AI-gain claims or release publication.
