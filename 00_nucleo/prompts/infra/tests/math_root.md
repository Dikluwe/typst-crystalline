# L0 — Public math root integration tests
Hash do Código: 1f3c3317

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/math_root.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9

## Current evidence

- `03_infra/tests/math_root.rs:59`, `:68`, `:80`, `:97` — root convergence, index states, diagnostics and sqrt controls.
- `00_nucleo/prompts/core/entities/elements/math_root.md` and `00_nucleo/prompts/core/compiler/eval/math.md` — observed productive contracts.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only the public integration test, not pipeline implementation. Compile real programs to prove causal/morphological convergence between root(3, x) and math.root([3], [x]). Distinguish absent sqrt index, explicit none and empty index. Preserve the existing eight public negative diagnostic cases in the consumer.

Bare sqrt and math.sqrt remain controls with absent index/false explicit-presence state, without inventing explicit index:none. Keep route-specific children: qualified [x] is Text content, while bare x is now native Symbol under the approved batch E declaration. Both represent as [x]; the differing element identity remains observable. Convergence means the same causal state and square-root semantics without collapsing those native variants.

Use isolated temporary worlds. Identity errors, timeout, crash, OOM, truncated output or ambiguous extraction are Unknown and block acceptance. Do not turn such outcomes into evidence of parity.

## Verification boundary

The prompt's missing legacy code-seal metadata is recorded in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md`; sanitation does not invent a seal or alter the test header.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
The pinned oracle and candidate1 math_root failure show that bare sqrt now retains a native Symbol x, with repr [x]. Update that representation expectation only; absent-index state, qualified Text morphology and all diagnostics remain controls.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
