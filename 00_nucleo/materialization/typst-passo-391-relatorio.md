# Passo 391 — relatório: materialização de `lorem(n)`

**Tipo:** materialização (L1 — stdlib helper puro; zero tipo novo; zero I/O; zero layout).
**Data:** 2026-06-22. **HEAD:** `08fed76d3`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se `lorem(n)` como helper puro `Int → Str`, gerador de texto dummy (Lorem Ipsum).
Zero tipo novo, zero I/O, zero layout.

- `01_core/src/rules/stdlib/text.rs` — novo `native_lorem(n)`.
- `01_core/src/rules/stdlib/mod.rs` — re-exporta `native_lorem`; adiciona 7 testes unitários.
- `01_core/src/rules/eval/mod.rs` — regista `"lorem"` em `make_stdlib`.
- `00_nucleo/prompts/rules/stdlib/text.md` — L0 actualizado com secção `lorem(n)`.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` — `lorem` reclassificado
de `ausente` para `implementado` (A.2 e B.5).

`cargo build --workspace` + `crystalline-lint .` verdes; **7** testes novos passam.

## Protocolo de Nucleação cumprido

1. Redigiu-se a secção L0 para `lorem(n)` no prompt existente `text.md` e propagaram-se hashes.
2. Implementou-se `native_lorem` (TDD).
3. Linhagem `@prompt`/`@prompt-hash` actualizada via `crystalline-lint --fix-hashes`.

## Decisão de engenharia

A paridade (ADR-0107) é semântica: `lorem(n)` devolve `Str` com exactamente `n` palavras de
texto dummy. O texto exacto não precisa de ser byte-identical ao vanilla. O vocabulário Lorem
Ipsum está embeddado no código; cicla/repete até atingir `n` palavras.

**Nota sobre o L0:** o Passo 391 previa inicialmente um ficheiro `lorem.md` separado. Durante a
validação, `crystalline-lint` sinalizou `text.md` como prompt órfão (`V7`) porque `text.rs` já o
referenciava; manter dois prompts no mesmo ficheiro-fonte faria o prompt existente perder a
referência. Optou-se por integrar a spec de `lorem` no `text.md` existente, mantendo a linhagem e
zero violations.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `lorem(0)` | `""` | ✓ |
| `lorem(5)` | 5 palavras | ✓ |
| `lorem(100)` | 100 palavras | ✓ |
| `lorem(-1)` | erro | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `lorem(5)` devolve 5 palavras; `lorem(0)` devolve `""`; `lorem(-1)` erro | ✓ |
| 2 | Zero tipo/variant novo; zero I/O | ✓ |
| 3 | Tests verdes; lint zero; hashes propagados | ✓ 7/7; `✓ No violations` |
| 4 | Inventário 148 actualizado | ✓ A.2 + B.5 |
| 5 | L0 salvo e hashado antes do código | ✓ secção em `text.md` |

## Artefactos

- Código: `01_core/src/rules/stdlib/text.rs`, `stdlib/mod.rs`, `eval/mod.rs`.
- L0: `00_nucleo/prompts/rules/stdlib/text.md` (actualizado).
- Inventário 148 — `lorem` implementado.
- este relatório.
