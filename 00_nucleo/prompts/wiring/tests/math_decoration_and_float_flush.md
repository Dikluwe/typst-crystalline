# L0 — wiring/tests/math_decoration_and_float_flush
Hash do Código: 9a14f202

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/math_decoration_and_float_flush.rs`

## Current evidence

- `04_wiring/tests/math_decoration_and_float_flush.rs:16` — `Scratch`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:19` — `new`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:32` — `drop`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:37` — `eval`.
- `04_wiring/tests/math_decoration_and_float_flush.rs:44` — `diagnostic`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/math_decoration_and_float_flush.rs`.

## Owned contract

- Own the black-box language/geometry suite only. Use isolated fixtures, propagate subprocess channels on failure and never turn Unknown into success.
- Preserve callbacks/cross/background/spans/no-provisional-output controls. Observe identity, morphology, dimensions, positions, presence and order; JSON/SVG/PDF are transports, not total-byte oracles.
- math.underline equality with underline must be JSON boolean false. SVG automatic-page geometry applies every ancestor translate/matrix in affine order once; no glyph/family/fixture-specific constants or defs/id-order comparisons.
- measure tests execute contextual measurement by rendering a uniquely colored shape, separating finite and height-auto regions; query/metadata/root are not execution substitutes. Preserve delimiter/alignment/repr/default/error controls.
- place.flush observes page, bbox and unique ordered tokens with explicit failure if pdftotext is absent. Cover prefix/suffix, no-flush, top/bottom floats, active-layouter nesting and no-float no-op. Keep tokens out of flow and remove fixtures.

## Verification and boundary

Run `cargo test -p typst-wiring --test math_decoration_and_float_flush -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Explicit code-mode alignment in math fixtures — 2026-10-13

### Measurement before decision

`04_wiring/tests/math_decoration_and_float_flush.rs:706` uses `align:left` in math syntax. Both
the verified vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 and Tekt reject
it with `unknown variable left`; `align:#left` succeeds in both. Sources,
binary hashes and outputs are in
`lab/results/construction/diagnostics/correcoes-producao-2026-10-13/math-bilateral.json`.

The alignment-placement fixture must use `#left`/`#right` as code-mode values
inside math. Preserve the differing-placement assertion and all geometry/repr
controls. Add an explicit negative syntax control so invalid bare alignment
does not become accepted to satisfy the fixture. This repairs the test's input,
not the public language. Internal ADR-0127 continuous correction.


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

Use `03_infra/tests/support/wiring/math_decoration_and_float_flush.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/math_decoration_and_float_flush.md`, for Scratch, SvgTransform, their existing impls and Drop.
Keep all scenarios and final observations in this L4 consumer. Import only
Scratch, SvgTransform through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/math_decoration_and_float_flush.rs:16 struct_item Scratch; 04_wiring/tests/math_decoration_and_float_flush.rs:18 impl_item Scratch; 04_wiring/tests/math_decoration_and_float_flush.rs:31 impl_item Scratch; 04_wiring/tests/math_decoration_and_float_flush.rs:143 struct_item SvgTransform; 04_wiring/tests/math_decoration_and_float_flush.rs:152 impl_item SvgTransform.

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
