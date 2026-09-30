# L0 — Unreferencable label classification
Hash do Código: aa5bec2e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/label_kind.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:5dde6e7f84150a121e671dab0d174798e8b56f24355147c1bcc471d6babd69ed

## Contract

The closed public enum `UnreferencableKind` distinguishes `Text`, `Raw`,
`EquationWithoutNumbering` and `Other`, with `Debug`, `Clone`, `Copy`,
`PartialEq`, `Eq` and `Hash`. It contains no layout algorithm or full diagnostic
text. Diagnostic production belongs to its own owner.
