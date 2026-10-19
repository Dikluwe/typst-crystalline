# L0 — core/compiler/eval/math_accent_observation_tests
Hash do Código: 534ae4bd

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_accent_observation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_accent_observation_tests.rs:19` — `struct NullWorld {`.
- `01_core/src/compiler/eval/math_accent_observation_tests.rs:48` — `fn id`.
- `01_core/src/compiler/eval/math_accent_observation_tests.rs:52` — `fn carrier`.
- `01_core/src/compiler/eval/math_accent_observation_tests.rs:73` — `fn p1429_observation_relation_discrimina_cada_causa_simetricamente`.

## Contract

Own independent MathAccent observation tests. Construct carriers through the
public boundary without reimplementing the observed predicate. Identity and clone
are Same. Isolated changes in normalized accent, size, dotless, size presence or
dotless presence are symmetrically Different. Missing binding, ambiguous carrier
or opaque execution is Unknown, never reinterpreted as Same. Productive comparison
belongs to 00_nucleo/prompts/core/compiler/eval/observation/math.md.
