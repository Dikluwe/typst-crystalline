# Passo 392 — relatório: materialização de `panic(msg)`

**Tipo:** materialização (L1 — stdlib helper puro; zero tipo novo; zero I/O; zero layout).
**Data:** 2026-06-22. **HEAD:** `08fed76d3`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se `panic(msg)` como helper de aborto de avaliação. Recebe `Str`, devolve
`Err(vec![SourceDiagnostic::error(..., msg)])`, reutilizando o mecanismo de erro existente.

- `01_core/src/engine/stdlib/panic.rs` — novo `native_panic(msg)`.
- `01_core/src/engine/stdlib/mod.rs` — adiciona `mod panic`, re-exporta `native_panic`; adiciona
  4 testes unitários.
- `01_core/src/engine/eval/mod.rs` — importa e regista `"panic"` em `make_stdlib`.
- `00_nucleo/prompts/engine/stdlib/panic.md` — L0 novo.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` — `panic(msg)` reclassificado
  de `ausente` para `implementado` (A.3 e B.5).

`cargo build --workspace` + `crystalline-lint .` verdes; **4** testes novos passam.

## Protocolo de Nucleação cumprido

1. Redigiu-se o L0 (`panic.md`) e propagaram-se hashes.
2. Implementou-se `native_panic` (TDD).
3. Linhagem `@prompt`/`@prompt-hash` actualizada via `crystalline-lint --fix-hashes`.

## Decisão de engenharia

`panic` é aborto de avaliação com mensagem. Em vez de criar novo tipo de erro, `native_panic`
constrói um `SourceDiagnostic` com a mensagem do utilizador e devolve `Err`, seguindo o mesmo
caminho de `native_assert` quando a condição é falsa. A paridade (ADR-0107) é semântica: aborta
e reporta a mensagem; não é necessário reproduzir o tipo Rust de erro do vanilla.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `panic("fail")` | eval aborta; mensagem "fail" | ✓ |
| `panic("")` | eval aborta; mensagem vazia | ✓ |
| `panic(123)` | erro de tipo | ✓ |
| `panic("x", foo: 1)` | erro de arg nomeado | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `panic("fail")` aborta eval; mensagem "fail" no diagnóstico | ✓ |
| 2 | Zero tipo/variant novo; zero I/O | ✓ |
| 3 | Tests verdes; lint zero; hashes propagados | ✓ 4/4; `✓ No violations` |
| 4 | Inventário 148 actualizado | ✓ A.3 + B.5 |
| 5 | L0 salvo e hashado antes do código | ✓ `panic.md` |

## Artefactos

- Código: `01_core/src/engine/stdlib/panic.rs` (novo), `stdlib/mod.rs`, `eval/mod.rs`.
- L0: `00_nucleo/prompts/engine/stdlib/panic.md`.
- Inventário 148 — `panic(msg)` implementado.
- este relatório.

## Nota sobre o Tekt

Com `square` (P390), `lorem` (P391) e `panic` (P392), fecha o bloco de três XS da fila limpa.
O próximo passo sobe para S/M (`#show regex`, `eval`, Model).
