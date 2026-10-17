# L0 — compiler/eval/observation/math
Hash do Código: 4d2453e3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/math.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/math.rs:147` — `fn p1437_math_mid_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:154` — `fn p1436_math_lr_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:245` — `fn p1441_math_overline_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:412` — `fn p1450_math_underparen_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:433` — `fn p1451_math_undershell_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:441` — `fn p1449_math_underbracket_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:449` — `fn p1448_math_underbrace_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:493` — `fn p1439_math_overbrace_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:501` — `fn p1440_math_overbracket_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:508` — `fn p1442_math_overparen_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:515` — `fn p1443_math_overshell_observation_differs`.
- `01_core/src/compiler/eval/observation/math.rs:634` — `fn p1444_math_primes_observation_differs`.

## Contract

Own observation of mathematical carriers and the crate-internal structural
regression helpers defined in this consumer. Compare every causal field,
including explicit-presence bits, optional style, delimiters, annotations and
slots. Do not compare derived glyphs, frames or constants, resolve context or
perform layout. Distinct mathematical identities remain Different even when
their appearance coincides. Helpers remain crate-internal; comparisons must
retain the same relation in both directions.

The closed comparisons retain these family-specific obligations: limits includes
inline_explicit; root includes index/index_explicit/radicand; frac includes num,
denom, line, optional style and both deparenthesization bits; accent includes
normalized accent, size, dotless and their presence bits; cases/mat/vec retain
every stored delimiter, alignment, gap, augment, child/row and presence field.
MathLr includes body/size/size_explicit, MathMid and MathOverline are body-only,
MathPrimes compares count, MathStretch includes body/size/presence and annotated
over/under families include body/annotation/annotation_explicit. Never collapse
omitted/default/explicit-none/empty-content states or cross public identities.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
