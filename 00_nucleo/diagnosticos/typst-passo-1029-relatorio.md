# Passo 1029 — Relatório de fecho dos 22 achados graves de Bloco 3 (família math)

**Data:** 2026-08-13  
**HEAD:** `0d041e2f7` (árvore limpa à partida; ficheiros de materialização `typst-passo-1027.md` e `typst-passo-1028.md` continuam não acompanhados)  
**Vanilla ratificado:** `/usr/local/bin/typst` e `lab/typst-original/target/release/typst` — ambos `typst 0.15.1 (e0e8ca4d)`  
**Cristalino:** `./target/release/typst` reconstruído no HEAD actual  
**Objectivo:** percorrer os 22 achados graves da família math, adicionar citações ou corrigir L0, e classificar os que exigem passo próprio. Nenhum código L1–L4 foi alterado neste passo.

---

## Execução do corpus `00_nucleo/corpus-docs/math/`

A pedido explícito (reparo aos relatórios 1027/1028), o corpus math/ foi compilado com os dois binários no início do passo.

| Binário | Resultado |
|---|---|
| Vanilla ratificado (`/usr/local/bin/typst`) | 19/20 ficheiros compilaram; `primes.typ:20` falha com `expected integer, found content` (problema do corpus, não do cristalino); `00-index.typ` emite warning de fonte ausente (`Pennstander Math`). |
| Cristalino (`./target/release/typst`) | 14/20 ficheiros compilaram; falhas esperadas por scope-outs conhecidos fora da família math deste passo: `cancel()` não suporta `length`/`inverted`/`cross`/`angle`/`stroke` (ADR-0054), `box(width: content)` ainda não implementado, modificadores de símbolo `v`/`c`/`harpoons` ainda não suportados, `none.where()` não suportado, `set` target vazio para `math.mat`/`math.cases`/`math.vec` ainda não suportado. Os ficheiros directamente relevantes para os 22 achados (`attach.typ`, `op.typ`, `class.typ`, `underover.typ`, `mat.typ`, `cases.typ`, `variants.typ`, `sizes.typ`) compilaram nos dois lados, com as advertências de `set` target vazio no cristalino. |

Os PDFs gerados encontram-se em `temp/p1029/` (`v_*.pdf` e `c_*.pdf`). Nenhuma comparação pixel-a-pixel foi feita neste passo porque o objectivo é fechar achados de documentação/citação, não medir novas divergências visuais.

---

## Resumo executivo

| Classificação | Contagem |
|---|---|
| Afirmação correcta — citação acrescentada (corpus/tests.rs/vanilla file:line) | 19 |
| Não verificável ao nível da linguagem / decisão de implementação (geometria OpenType MATH) | 3 |
| L0 corrigido (factual, sem mudar comportamento) | 0 |
| Achado a escalar para passo próprio | 0 |
| **Total** | **22** |

Nenhum achado revelou divergência real de código; todos foram fechados com citação ou classificação justificada.

## Tabela de achados

| # | L0 | Secção/linhas | Classificação | Prova / citação adicionada | Notas |
|---|---|---|---|---|---|
| 1 | `compiler/math/layout/attach.md` | `is_limits`, l. 31–34 | Citação | Adicionada citação a `docs.typst.app/reference/math/attach/#functions-limits` e `.../#functions-scripts`; corpus `attach.typ:18-24`; guardas `tests.rs:180` e `:221`. | Afirmação correcta; só faltava citação de língua. |
| 2 | `compiler/math/layout/attach.md` | P992, l. 64–70 | Citação | Adicionada citação a corpus `attach.typ:26-28` e guardas `tests.rs:7560-7760`. | Afirmação correcta; só faltava citação. |
| 3 | `compiler/math/layout/attach.md` | P914, l. 124–130 | Decisão de implementação + citação | Adicionada nota de que é geometria OpenType MATH e citação a guardas `tests.rs:2963-3145`. | O observável é posição de glifo no PDF; não há construção equivalente na linguagem Typst. |
| 4 | `compiler/math/layout/cases.md` | P912, l. 26–29 | Citação | Adicionada citação a guarda `tests.rs:4766` (`p945_grid_delim_target_du_e_altura_vezes_1_1`). | Afirmação correcta e já fechada; reforçada com citação de guarda. |
| 5 | `compiler/math/layout/delimited.md` | P912, l. 14–20 | Citação | Adicionada citação exacta ao vanilla `fenced.rs:93-99` e guarda `tests.rs:4071`. | Afirmação correcta; agora tem file:line do vanilla. |
| 6 | `compiler/math/layout/matrix.md` | P912, l. 68–71 | Citação | Adicionadas citações a `tests.rs:4766` e `:3981`. | Afirmação correcta e já fechada; reforçada com citações. |
| 7 | `compiler/math/layout/matrix.md` | `delim`, l. 112–117 | Citação | Adicionada citação a `docs.typst.app/reference/math/mat/#parameters-delim` e corpus `mat.typ:16-24`. | Afirmação correcta; só faltava citação de língua. |
| 8 | `compiler/math/layout/op.md` | l. 20–24 | Citação | Adicionada citação a `docs.typst.app/reference/math/op/#parameters-limits` e corpus `op.typ:24-27`; guardas `tests.rs:180` e `:7560`. | Afirmação correcta; só faltava citação. |
| 9 | `compiler/math/layout/op.md` | l. 23–24 | Citação | Mesma citação acima; o mecanismo de `is_limits` está em `attach.md` e `symbols.md`. | Afirmação correcta; coberta pela citação de limits e pelas guardas de attach. |
| 10 | `compiler/math/layout/spacing.md` | várias secções | Citação | Adicionadas citações a `docs.typst.app/reference/math/class/`, corpus `class.typ:8-50`, guardas `tests.rs:206-224`, `:1799`, `:2939-3145`, `:6720-6750`, `:7765`, `:5114-5610`. | Afirmação correcta; reforçada com citações de língua e de guarda. |
| 11 | `compiler/math/layout/underover.md` | P920/P922/P984, l. 92–97 e 155–157 | Citação | Adicionadas citações a corpus `underover.typ:15-21`, vanilla `resolve.rs:1277-1472`/`accent.rs:56-71`, guardas `tests.rs:2200-2500` e `:4101-4272`. | Afirmação correcta; só faltava citação. |
| 12 | `compiler/math/symbols.md` | `is_math_function`, l. 76–79 | Citação | Adicionada citação a `docs.typst.app/reference/math/op/` e corpus `op.typ:13-17`; guardas `symbols.rs:270-348`. | Afirmação correcta; só faltava citação. |
| 13 | `compiler/math/symbols.md` | `is_single_letter_var`, l. 83–84 | Citação | Adicionada citação a `docs.typst.app/reference/math/` e guardas `tests.rs:416`, `symbols.rs:349-367`. | Convenção tipográfica padrão; confirmada por medições P809/P311a. |
| 14 | `compiler/math/symbols.md` | `is_large_operator`, l. 95–99 | Citação | Adicionada citação a `docs.typst.app/reference/math/class/` e corpus `class.typ:37-38`; lista medida contra vanilla `sym.rs`/`typst-utils`; guardas `symbols.rs:368-418` e `tests.rs:180`. | Afirmação correcta; só faltava citação. |
| 15 | `entities/elements/math_limits_override.md` | Contexto, l. 17–22 | Citação | Adicionada citação a `docs.typst.app/reference/math/attach/#functions-scripts` e `.../#functions-limits`; corpus `attach.typ:18-28`; guardas `tests.rs:7560-7760`. | Afirmação correcta; só faltava citação. |
| 16 | `entities/elements/math_limits_override.md` | Contexto, l. 25–27 | Citação | Adicionada citação a vanilla `attach.rs` (`LimitsElem::set_limits`) e `resolve.rs` (`resolve_limits`); guardas `tests.rs:7632-7683`. | Afirmação correcta; agora tem file:line do vanilla. |
| 17 | `entities/elements/math_limits_override.md` | Contexto, l. 38–40 | Citação | Adicionada citação a vanilla `resolve.rs` (`resolve_scripts`/`resolve_limits`); guardas `tests.rs:7765` e `:7736`. | Afirmação correcta; só faltava citação. |
| 18 | `entities/elements/math_op.md` | Cabeçalho, l. 7–8 | Citação | Adicionada citação a `docs.typst.app/reference/math/op/`, corpus `op.typ:7-27`, vanilla `op.rs` e `resolve.rs`. | Afirmação correcta; só faltava citação. |
| 19 | `entities/elements/math_op.md` | `impl Element`, l. 30 | Citação / esclarecimento | Adicionada nota de que "layout-only" se refere à entidade; o consumo real está em `attach.rs:78`. | Não é uma afirmação sobre a linguagem Typst, mas sobre a separação entidade/layout do cristalino. |
| 20 | `entities/math-class.md` | Notas de implementação, l. 170–173 | Citação | Adicionada citação a vanilla `typst-utils/src/lib.rs` (`default_math_class`) e guardas `math_class.rs:163-266`. | Afirmação correcta; só faltava citação. |
| 21 | `entities/math_constants.md` | P990, l. 241–256 | Citação | Adicionada citação a vanilla `fraction.rs:33-52` e guardas `tests.rs:7192-7257`. | Constantes OpenType MATH; afirmação correcta e agora com file:line do vanilla. |
| 22 | `entities/math_style.md` | `map_glyph`, l. 71–81 | Citação | Adicionada citação aos passos P311b.1/P809/P812-C/P964 e guardas `math_style.rs:248-420`. | Algoritmo Unicode; validado por medições byte-a-byte. |

---

## L0s alterados

- `00_nucleo/prompts/compiler/math/layout/attach.md`
- `00_nucleo/prompts/compiler/math/layout/cases.md`
- `00_nucleo/prompts/compiler/math/layout/delimited.md`
- `00_nucleo/prompts/compiler/math/layout/matrix.md`
- `00_nucleo/prompts/compiler/math/layout/op.md`
- `00_nucleo/prompts/compiler/math/layout/spacing.md`
- `00_nucleo/prompts/compiler/math/layout/underover.md`
- `00_nucleo/prompts/compiler/math/symbols.md`
- `00_nucleo/prompts/entities/elements/math_limits_override.md`
- `00_nucleo/prompts/entities/elements/math_op.md`
- `00_nucleo/prompts/entities/math-class.md`
- `00_nucleo/prompts/entities/math_constants.md`
- `00_nucleo/prompts/entities/math_style.md`

Os hashes dos correspondentes ficheiros L1 foram resselados com `crystalline-lint --fix-hashes .`.

---

## Validação final

```
crystalline-lint .
cargo test --workspace
```

**Resultado:**

- `crystalline-lint .`: ✅ 0 violações (apenas 3 warnings V7 de prompts órfãos pré-existentes, fora do âmbito deste passo).
- `cargo test --workspace`: ✅ 5848 testes passaram, 0 falharam.
  - `typst-core`: 4977 passed
  - `typst-infra`: 789 passed
  - `typst-shell`: 41 passed
  - `typst-wiring`: 37 passed
  - outros: 2 + 2 + doc-tests ignorados

Zero regressão.
