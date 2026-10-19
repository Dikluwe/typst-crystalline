# L0 — Unreferencable label classification
Hash do Código: f91e98da

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/label_kind.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:b19634c84ea14e71d8e352c96bbe5097a02a074b8c5a3009e80d761d1a90a182

## Contract

The closed public enum `UnreferencableKind` distinguishes `Text`, `Raw`,
`EquationWithoutNumbering` and `Other`, with `Debug`, `Clone`, `Copy`,
`PartialEq`, `Eq` and `Hash`. It contains no layout algorithm or full diagnostic
text. Diagnostic production belongs to its own owner.
