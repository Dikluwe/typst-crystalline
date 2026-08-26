# Prompt L0 — `testing/mod` — hub test-only

Hash do Código: b9cb23ff

**Camada:** L1, somente testes
**Ficheiro proprietário:** `01_core/src/testing/mod.rs`

## Contrato

O hub declara exclusivamente `pub(crate) mod math_oracle`. Não contém fórmulas,
I/O, lógica produtiva ou reexports públicos. O contrato das fórmulas pertence a
`testing/math_oracle.md`.

## Aceitação

O oráculo fica acessível aos testes internos da crate e ausente de builds de
produção; nenhuma responsabilidade do oráculo é duplicada neste owner.
