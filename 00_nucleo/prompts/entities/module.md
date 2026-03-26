# Prompt L0 — entities/module (stub)

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/module.rs`
**ADRs relevantes**: ADR-0016 (adiamento eval/typst-library)

## Contexto

`Module` é o resultado de avaliar um ficheiro Typst. No original
(`typst-library/src/foundations/module.rs`), contém `Scope`, `Content`,
e `EcoString` — todos com dependências externas que ainda não foram
migradas.

Este ficheiro define apenas um stub opaco. O interior será definido
quando `typst-library/src/foundations/` for analisada e `Module` real
for migrado (ver ADR-0016).

## Interface pública (stub)

```rust
pub struct Module(());
```

## Critérios de Verificação

```
Dado Module stub
Quando compilado
Então compila sem erros e satisfaz V2 (testes presentes)
```
