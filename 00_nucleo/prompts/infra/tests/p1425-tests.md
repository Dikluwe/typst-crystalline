# L0 — Public math limits integration tests
Hash do Código: 98eb2458

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/p1425_math_limits.rs`

## Current evidence

- `03_infra/tests/p1425_math_limits.rs` — public limits/scripts cases.
- `00_nucleo/prompts/compiler/stdlib/structural/math.md`, `00_nucleo/prompts/compiler/eval/math.md` and `00_nucleo/prompts/compiler/eval/repr.md` — productive owners observed by the test.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Bare syntax and qualified math.limits calls converge in public repr for omitted inline, inline:true and inline:false, preserving explicit presence of the default. Qualified calls retain all five existing signature diagnostics. Bare and qualified scripts remain convergent and do not acquire a named inline parameter.

Use HTML only to expose the public string. DOM structure, bytes, fonts and HTML rendering are not acceptance. Failure, ambiguous or unreadable output receives no credit. No productive or external-artifact contract is owned by this test.
