# Prompt L0 — entities/value (stub)

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/value.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval/typst-library)

## Contexto

`Value` é o tipo de valor em tempo de avaliação do Typst. No original
(`typst-library/src/foundations/value.rs`), é um enum com 40+ variantes
incluindo `Content`, `Module`, `Func`, `Args`, `Type`, `Gradient`,
`Tiling` — todos com dependências externas não ainda migradas.

Este ficheiro define apenas um stub opaco. O interior será definido
quando `typst-library/src/foundations/` for analisada e `Value` real
for migrado (ver ADR-0017).

## Interface pública (stub)

```rust
pub struct Value(());
```

## Critérios de Verificação

```
Dado Value stub
Quando compilado
Então compila sem erros e satisfaz V2 (testes presentes)
```
