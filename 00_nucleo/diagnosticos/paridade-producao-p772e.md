# Paridade de Produção — P772e: Reconfirmação de `lacuna-inventario` e Decisão de Continuidade

**Data:** 2026-07-16T12:53:16-03:00  
**Commit base:** `4b9c67a77302bed43dfcdd58f9f820171c28515c`  
**Estado da working tree:** existem modificações não commitadas além deste passo (ver `git diff HEAD --stat` no final).

---

## 1. Recontagem da lista `lacuna-inventario`

Comando usado:

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

**Total de itens na lista:** 400  
**Itens já tratados (P765a→P772d):** 164  
**Itens restantes:** 236

### Módulos restantes (topo)

| Módulo | Itens restantes | Risco observável |
|---|---|---|
| `typst_library::layout::grid::resolve` | 24 | Alto — layout renderizado |
| `typst_utils` | 14 | Baixo — infraestrutura Rust |
| `typst` (root) | 8 | Médio — CLI/entrypoints |
| `typst_syntax::reparser` | 7 | Baixo — parsing incremental |
| `typst_library::visualize::image::svg` | 7 | Alto — renderização de imagem |
| `typst_library::foundations::scope` | 7 | Alto — semântica de scopes/bindings |
| `typst_library` (misc) | 7 | Variado |
| `typst_syntax::node` | 6 | Baixo — AST/CST |
| `typst_syntax::ast` | 6 | Baixo — AST |
| `typst_library::foundations::target_` | 6 | Médio — link targets |
| `typst_library::text::font::*` | ~22 (variations, metrics, info, book, exceptions, case) | Alto — tipografia |
| `typst_library::routines` | 5 | Baixo |
| `typst_library::foundations::plugin_` | 5 | Médio — plugins WASM |
| ... | ... | ... |

Lista completa disponível no comando acima.

---

## 2. Rendimento até agora

| Módulo | Itens | Bugs reais encontrados |
|---|---:|---:|
| `foundations::calc` + `foundations::ops` | 65 | 2 (título, símbolo) |
| `diag` | 24 | 0 |
| `math::style` | 32 | 4 |
| `image::raster` | 13 | 2 (DPI/rotação) |
| `pdf::accessibility` | 12 | 0 |
| `syntax::span` | 10 | 1 (cross-file span) |
| `syntax::package` | 8 | 0 |
| **Total** | **164** | **9** |

Métricas:

- **Bugs por módulo:** 9 / 8 = **1,13**
- **Bugs por item:** 9 / 164 = **5,5%**
- **Módulos com pelo menos 1 bug:** 4 / 8 = **50%**

Nota: a correcção de P772d (span detached em erros de I/O de `#import`/`#include`) é um desdobramento da área `syntax::span`/`package`; não foi contabilizada como módulo adicional para não distorcer a taxa.

---

## 3. Decisão

**Opção escolhida:** **Continuar só para módulos com risco de efeito observável.**

Justificativa (dados):

- 50% dos módulos investigados renderam pelo menos 1 bug real — taxa alta o suficiente para não encerrar de imediato.
- No entanto, a maioria dos 236 itens restantes é infraestrutura Rust pura (`typst_utils`, `typst_syntax::reparser/node/ast`, `typst_eval::*`, `typst_library::routines`), onde o risco de efeito observável na linguagem é baixo.
- Os módulos de maior risco restantes são:
  1. `layout::grid::resolve` (24 itens) — layout renderizado;
  2. `visualize::image::svg` (7 itens) — renderização de imagem;
  3. `foundations::scope` (7 itens) — semântica de bindings;
  4. `text::font::*` (~22 itens) — tipografia e métricas.

Estes quatro módulos concentram ~60 itens de alto risco. Se a taxa de 5,5% se mantiver, esperam-se ~3 bugs adicionais. O esforço é justificado face ao potencial de retorno.

---

## 4. Próximo passo

Aberto `00_nucleo/materialization/typst-passo-772f.md` para varredura de `typst_library::layout::grid::resolve` (24 itens), o maior módulo restante de alto risco.

Se P772f, `image::svg`, `foundations::scope` e `text::font::*` não encontrarem bugs, reavaliar se encerra a série.

---

## 5. Estado do `git diff HEAD --stat`

```text
 00_nucleo/0.15.0.typ                     | 463 -------------------------------
 00_nucleo/testing/fontes-padrao-teste.md |  74 -----
 2 files changed, 537 deletions(-)
```

Nota: estes ficheiros já estavam modificados na working tree antes deste passo; não foram alterados por P772e.

---

## 6. Critério de fecho

- [x] Lista `lacuna-inventario` restante reconfirmada por contagem directa.
- [x] Rendimento até agora calculado (164 itens varridos, 9 bugs reais, 50% dos módulos com bug).
- [x] Decisão registada com base nos números: continuar selectivamente para módulos de alto risco observável.
- [x] Próximo módulo identificado (`layout::grid::resolve`) e P772f aberto.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772e.md`.
