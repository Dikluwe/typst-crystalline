# L0 — infra/tests/support/wiring/format_svg
Hash do Código: 0e5d8532

**Layer:** L3, test-only.
**Exclusive consumer:** `03_infra/tests/support/wiring/format_svg.rs`.
**Governance:** ADR-0107/0108/0127/0129/0130.


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

Only this prompt owns this L3 consumer. The independently owned L4 caller
(`00_nucleo/prompts/wiring/tests/format_svg.md`) governs scenarios without shared
ownership. Types, fields needed by the parent and used inherent members may be
pub(super); internal functions, Checks state and statics remain private. No
reexport in typst_infra.

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

## Unit operations

new preserves NEXT_TEMP.fetch_add(1, Relaxed), suite-specific prefix, pid and
label. create_dir remains exclusive: AlreadyExists retries; any other error fails
with unchanged path/error text. Drop attempts remove_dir_all on the same root.
No nonexclusive acquisition or directory fallback.

require accumulates message.into only when condition is false. finish consumes
the aggregate and requires empty failures, with its unchanged suite-specific
prefix/message join. Existing expect_json checks status, stderr and stdout in
the same order, retaining legitimate per-suite differences. Never ignore failures
or change trimming, strings or tolerances.
