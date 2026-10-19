# L0 — core/compiler/eval/math_limits_override_observation_tests
Hash do Código: 1704cc7e

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_limits_override_observation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_limits_override_observation_tests.rs:11` — `fn carrier`.
- `01_core/src/compiler/eval/math_limits_override_observation_tests.rs:16` — `fn p1425_observation_relation_observa_presenca_inline`.

## Contract

Own only tests of the MathLimitsOverride observation relation. Construct complete
carriers with equal body/limits/inline and vary only inline_explicit. Two equal
omitted carriers and two equal explicit carriers are Same; omission versus explicit
inline:true is Different in both directions, never Same or Unproven. Observe the
closed private trait through descendant tests without extending a public API.
The normative productive relation belongs to
00_nucleo/prompts/core/compiler/eval/observation/math.md, not this test consumer.
Derived Rust equality, external artifacts or frozen rendered hashes are not oracles.
