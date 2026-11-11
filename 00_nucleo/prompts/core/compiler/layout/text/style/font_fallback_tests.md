# L0 — Effective font fallback regressions
Hash do Código: ce62f163

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:24f0b69fa1db115ff13a786121c80cb7be3abfa2028a9a473075374a6815cfd4
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00

Layer L1. Exclusive consumer: `01_core/src/compiler/layout/text/style/font_fallback_tests.rs`.

## Evidence before specification

Before the fallback change, `01_core/src/compiler/layout/text/style.rs:127-261`
decodes custom family arrays, `:262-273` determines nearest typed/custom font
precedence, and `:296-300` selects custom or active fallback. The current owner
preserves rich family identity, resolved size, lexical direction and script
provenance. `font_list.rs:28-30` makes Covers uninhabited; only None is representable.

## Contract

Test the actual private from_parts and resolve_parts using real StyleChain and
TextStyle. Nearer typed font wins outer custom; nearer custom wins outer typed;
same-node typed wins custom. Missing, empty, invalid or nonarray custom retains
the complete active Option, including None. Preserve ordered literal/regex family
names, variants, variant, weight, style, representable covers=None, and every
family of the typed fallback. A valid decoded list succeeds even with no active
font. The output owns independent list storage without mutating active style or
chain; pointer nonalias checks verify internal ownership, not language parity.
Preserve other render fields, affine resolved size, absent/explicit language,
direction, variations, mathematical/script and semantic Strong/Emph provenance.
Private cfg(test) only; no instrumentation, public API, dependency or default.
Semantic expectations do not depend on eager versus lazy allocation or a fixed
allocation count. These regressions do not certify general language parity;
outline divergence against vanilla64c5e8c02fe0394d913ead36e42ef8c1334d2107 stays open.
