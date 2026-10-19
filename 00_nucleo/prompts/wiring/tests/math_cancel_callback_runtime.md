# L0 — wiring/tests/math_cancel_callback_runtime
Hash do Código: 08783350

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/math_cancel_callback_runtime.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
```

## Current evidence

- `04_wiring/tests/math_cancel_callback_runtime.rs:18` — `Scratch`.
- `04_wiring/tests/math_cancel_callback_runtime.rs:21` — `new`.
- `04_wiring/tests/math_cancel_callback_runtime.rs:34` — `drop`.
- `04_wiring/tests/math_cancel_callback_runtime.rs:39` — `diagnostic`.
- `04_wiring/tests/math_cancel_callback_runtime.rs:48` — `compile`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/math_cancel_callback_runtime.rs`.

## Owned contract

- Own only math callback black-box integration. Compile isolated fixtures and observe final geometry/diagnostics, not implementation internals or provisional documents.
- Angle callback agrees with explicit-angle control; lexical text and derived script math sizes remain observable. cross overrides inverted in two-line morphology. Invalid returns preserve call path/line, fail without panic and produce no output.
- Use local cleaned fixtures. Unknown is not success; vec/underline generalization is outside this suite. Keep the pinned callback-realization obligation without authorizing productive changes.

## Verification and boundary

Run `cargo test -p typst-wiring --test math_cancel_callback_runtime -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


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

Use `03_infra/tests/support/wiring/math_cancel_callback_runtime.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/math_cancel_callback_runtime.md`, for Scratch, their existing impls and Drop.
Keep all scenarios and final observations in this L4 consumer. Import only
Scratch through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/math_cancel_callback_runtime.rs:18 struct_item Scratch; 04_wiring/tests/math_cancel_callback_runtime.rs:20 impl_item Scratch; 04_wiring/tests/math_cancel_callback_runtime.rs:33 impl_item Scratch.

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
