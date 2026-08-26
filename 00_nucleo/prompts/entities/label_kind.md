# Prompt L0 — `entities/label_kind` — classe de label não referenciável
Hash do Código: aa5bec2e


**Camada:** L1
**Ficheiro proprietário:** `01_core/src/entities/label_kind.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:5dde6e7f84150a121e671dab0d174798e8b56f24355147c1bcc471d6babd69ed

## Contrato

O enum público fechado `UnreferencableKind` distingue `Text`, `Raw`,
`EquationWithoutNumbering` e `Other`, com `Debug`, `Clone`, `Copy`,
`PartialEq`, `Eq` e `Hash`. Não contém algoritmo de layout nem texto completo
de diagnóstico.
