# L0 — core/compiler/stdlib/structural/math/math_limits_tests
Hash do Código: 46cccca5

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/math_limits_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/math/math_limits_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Own only structural tests derived from `00_nucleo/prompts/core/compiler/stdlib/structural/math.md`, `00_nucleo/prompts/core/entities/elements/math_limits_override.md` and `00_nucleo/prompts/core/compiler/eval/repr.md`. Verify closed-module native limits with public name limits; omitted/inline:true/inline:false all have limits:true while inline_explicit distinguishes omission. Verify body-first repr: limits(body: [x]), limits(body: [x], inline: true), limits(body: [x], inline: false). Preserve the five diagnostic fragments for missing/wrong body, unknown named, extra positional and named body. Equality distinguishes omission from explicit default; plain_text/is_empty/map_content/map_text preserve variant and all three controls. scripts remains limits:false, inline:true, presence false, repr scripts(body: [x]). No candidate inspection, Rust structural equality substituted for language observations or external artifacts.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
