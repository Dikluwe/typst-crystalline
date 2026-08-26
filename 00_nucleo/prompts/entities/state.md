# Prompt L0 — `entities/state` — valor documental State

Hash do Código: 0984caf6

**Camada:** L1
**Ficheiro proprietário:** `01_core/src/entities/state.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:27acb21a5e0b2e0cb3b65de61bba5266158f3e9828392fe92a1a3b160e9d61a4

## Contrato

`State` conserva chave e valor inicial, oferece construção e acesso aos dados,
e mantém igualdade/hash/repr coerentes com sua identidade linguística. Não
executa métodos nativos, introspecção ou I/O; esses pertencem a outros owners.
