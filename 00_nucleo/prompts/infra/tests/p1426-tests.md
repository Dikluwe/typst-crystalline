# L0 — Public math root integration tests
Hash do Código: 28c357f4

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/p1426_math_root.rs`

## Current evidence

- `03_infra/tests/p1426_math_root.rs:59`, `:68`, `:80`, `:97` — root convergence, index states, diagnostics and sqrt controls.
- `00_nucleo/prompts/entities/elements/math_root.md` and `00_nucleo/prompts/compiler/eval/math.md` — observed productive contracts.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only the public integration test, not pipeline implementation. Compile real programs to prove causal/morphological convergence between root(3, x) and math.root([3], [x]). Distinguish absent sqrt index, explicit none and empty index. Preserve the existing eight public negative diagnostic cases in the consumer.

Bare sqrt and math.sqrt remain controls with absent index/false explicit-presence state, without inventing explicit index:none. Keep route-specific children: qualified [x] content versus a bare math identifier x. Convergence means the same causal state and square-root semantics, not forced textual identity between already distinct radicand representations.

Use isolated temporary worlds. Identity errors, timeout, crash, OOM, truncated output or ambiguous extraction are Unknown and block acceptance. Do not turn such outcomes into evidence of parity.

## Verification boundary

The prompt's missing legacy code-seal metadata is recorded in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md`; sanitation does not invent a seal or alter the test header.
