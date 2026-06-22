# Passo 390 — relatório: materialização de `square(...)`

**Tipo:** materialização (L1 — stdlib helper; zero tipo novo; zero I/O). **Data:** 2026-06-22.
**HEAD:** `08fed76d3`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se `square(...)` como helper sintático sobre `Rect` (`ShapeKind::Rect`),
fechando a dívida genuína acidental XS identificada na sonda do Passo 389. Não se criou
`ShapeKind::Square` nem novo variant `Value`/`Content`.

- `01_core/src/rules/stdlib/shapes.rs` — novo `native_square(width, height: auto, fill?, stroke?)`.
- `01_core/src/rules/stdlib/mod.rs` — re-exporta `native_square`; adiciona 5 testes unitários.
- `01_core/src/rules/eval/mod.rs` — regista `"square"` em `make_stdlib`.
- `00_nucleo/prompts/rules/stdlib/square.md` — L0 novo.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` — `square(...)` reclassificado
de `ausente` para `implementado` (A.7 e B.5).

`cargo build --workspace` + `crystalline-lint .` verdes; **5** testes novos passam.

## Protocolo de Nucleação cumprido

1. Redigiu-se o L0 (`square.md`) e propagaram-se hashes.
2. Implementou-se `native_square` (TDD): testes primeiro, código depois.
3. Linhagem `@prompt`/`@prompt-hash` atualizada via `crystalline-lint --fix-hashes`.

## Decisão de engenharia

`square` é **morfologia sobre `Rect`** (ADR-0107). `square(w)` constrói
`Content::shape(ShapeKind::Rect, width=w, height=w)`; `square(w, height: h)` aceita
`h != w` como fallback vanilla. Reaproveitou-se o parsing de `fill`/`stroke` e o fallback de
stroke preta 1pt de `native_rect`.

## Paridade

| Caso | Equivalente morfológico |
|------|-------------------------|
| `square(1cm)` | `rect(width: 1cm, height: 1cm)` |
| `square(1cm, height: 2cm)` | `rect(width: 1cm, height: 2cm)` |

A paridade é morfológica (mesma forma `Rect`), não byte-diff de saída renderizada.

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `square(1cm)` produz saída morfologicamente idêntica a `rect(1cm, 1cm)` | ✓ |
| 2 | Zero tipo/variant novo; zero I/O | ✓ |
| 3 | Tests verdes; lint zero; hashes propagados | ✓ 5/5; `✓ No violations` |
| 4 | Inventário 148 actualizado | ✓ A.7 + B.5 |
| 5 | L0 salvo e hashado antes do código | ✓ `square.md` |

## Artefactos

- Código: `01_core/src/rules/stdlib/shapes.rs`, `stdlib/mod.rs`, `eval/mod.rs`.
- L0: `00_nucleo/prompts/rules/stdlib/square.md`.
- Inventário 148 — `square(...)` implementado.
- este relatório.
