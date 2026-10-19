# L0 — wiring/tests/format_pdf_standard
Hash do Código: 90b72f8c

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/format_pdf_standard.rs`

## Current evidence

- `04_wiring/tests/format_pdf_standard.rs:30` — `ProcessResult`.
- `04_wiring/tests/format_pdf_standard.rs:37` — `success`.
- `04_wiring/tests/format_pdf_standard.rs:42` — `typst_binary`.
- `04_wiring/tests/format_pdf_standard.rs:48` — `shm_root`.
- `04_wiring/tests/format_pdf_standard.rs:58` — `fixture_paths`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/format_pdf_standard.rs`.

## Owned contract

- Own only standard language/version resolution and tagged/pages interaction in real-PDF subprocesses, not structural PDF/A/UA conformity or version-specific PDF rewrites.
- Check names/conflicts, explicit version, profile maximum version when absent, CLI over document and default 1.7. Examples: documentary 1.4 or a-1b gives header 1.4; a-2b gives 1.7; CLI 2.0/a-4 gives 2.0.
- Accessible standards with false tags preserve distinct documentary ``cannot set `pdf(tagged: false)` when exporting a {name} document`` versus precompile CLI `cannot disable PDF tags when exporting a PDF/A-1a document`. Ua1 plus CLI pages without explicit tagged uses the UA-1 variant and hint `using --pages implies --pdf-tagged=false`, exit 1.
- Conflicting CLI 1.4/1.7 fails before compilation with `PDF cannot conform to PDF 1.4 and PDF 1.7 at the same time`. Observe header prefix and diagnostics, never equality of the remaining bytes.
- Do not claim XMP profile identification, color/font/transparency rules or external conformity validation from these tests. Keep standalone other-format/tagged/pretty/pages suites separate. Use explicit /dev/shm TMPDIR without disk fallback, serial tests and finite process limits.

## Verification and boundary

Run `cargo test -p typst-wiring --test format_pdf_standard -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Useful subprocess failure observations

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`
records the warnings at `04_wiring/tests/format_pdf_standard.rs:32`, retaining
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

Use `03_infra/tests/support/wiring/format_pdf_standard.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/format_pdf_standard.md`, for ProcessResult, their existing impls and Drop.
Keep all scenarios and final observations in this L4 consumer. Import only
ProcessResult through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/format_pdf_standard.rs:30 struct_item ProcessResult; 04_wiring/tests/format_pdf_standard.rs:36 impl_item ProcessResult.

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
