# L0 — wiring/tests/format_pdf_pages
Hash do Código: 7e659d14

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/format_pdf_pages.rs`

## Current evidence

- `04_wiring/tests/format_pdf_pages.rs:30` — `ProcessResult`.
- `04_wiring/tests/format_pdf_pages.rs:37` — `success`.
- `04_wiring/tests/format_pdf_pages.rs:42` — `typst_binary`.
- `04_wiring/tests/format_pdf_pages.rs:48` — `shm_root`.
- `04_wiring/tests/format_pdf_pages.rs:58` — `fixture_paths`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/format_pdf_pages.rs`.

## Owned contract

- Own real-PDF page-selection integration: documentary integer/range, CLI list and explicit precedence, filtered page count and interactions with tagging. A beyond-end selection must not die by signal; its count is not fixed by this contract.
- Documentary explicit true with pages fails with `cannot enable tagged PDF and export a page range`; CLI true plus pages fails before compilation with distinct `cannot enable PDF tags when exporting a page range`, exit 1.
- Documentary Auto plus pages emits `exporting a page range disables PDF tagging` and the two canonical inaccessibility/silencing hints, producing no struct tree. Explicit false remains silent.
- Included headings survive native outline filtering; excluded headings disappear and included descendants reparent to the nearest included ancestor. An included outline must not become empty merely because pages are selected. Links to removed pages lose their annotation instead of redirecting; keep no-selection control.
- Inspect parsed counts/tree/titles/annotations, not full PDF bytes. Standard/conformity/PageLabels and PNG/SVG multi-output are outside this suite. Use explicit /dev/shm TMPDIR, no disk fallback, serial execution and bounded subprocesses.

## Verification and boundary

Run `cargo test -p typst-wiring --test format_pdf_pages -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Useful subprocess failure observations

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`
records the warnings at `04_wiring/tests/format_pdf_pages.rs:32`, retaining
sources/L0/hashes/HEAD/uncommitted tree/UTC/diffstat. Consumer/use inspection
preceded the decision; selected Clippy gate must fail before implementation.
This is Rust observation/representation maintenance, no new upstream intent/parity.

Keep ProcessResult.stdout and capture it; include stdout/stderr in existing
success-assertion diagnostics. Do not count derived Debug as a useful read or
replace canonical stderr/status/PDF/precedence/fixture/process/timeout checks.
This useful observation removes dead_code without suppression or lost failure data.
Continuous ADR-0127: no new public/default/phase contract; L0 first, scoped
resealing, no new lint suppression. Require exact inventory reduction/no new
warnings, Clippy RED→GREEN/unfulfilled_lint_expectations as error, workspace
suite/build/fmt/Rustdoc/lineage under 12 GiB, zero swap, one job. No tests merely
mirroring derives/Entry/module placement. The historical bbox case requires real
observation as specified; other controls retain fixtures/expectations.

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

Use `03_infra/tests/support/wiring/format_pdf_pages.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/format_pdf_pages.md`, for ProcessResult, their existing impls and Drop.
Keep all scenarios and final observations in this L4 consumer. Import only
ProcessResult through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/format_pdf_pages.rs:30 struct_item ProcessResult; 04_wiring/tests/format_pdf_pages.rs:36 impl_item ProcessResult.

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
