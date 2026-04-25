# Paridade — Passo 150 (2026-04-25)

**Primeira matriz agregada (Passo 150)**. Esta iteração entrega **cristalino-only baseline**: cada ficheiro do corpus é compilado em cristalino e contado como sucesso/falha. **Comparação contra vanilla** está pendente em **DEBT-53** (candidato): integração do pipeline vanilla `lab/typst-original/crates/typst::compile` exige World adapter (vanilla `World` ≠ cristalino `World`) e materializar `from_vanilla` em `frame_dto.rs`. As colunas `text_content`, `structural` e `geometric` ficam `N/A` até DEBT-53 ser resolvido — a infraestrutura (DTO + matriz + render) está pronta e validada.

## Matriz

| Categoria | Total | Compila (cristalino) | text_content | structural | geometric (experimental) |
|-----------|------:|---------------------:|-------------:|-----------:|:------------------------:|
| code | 2 | 2/2 | N/A | N/A | N/A |
| markup | 6 | 6/6 | N/A | N/A | N/A |
| math | 2 | 2/2 | N/A | N/A | N/A |
| visual | 9 | 9/9 | N/A | N/A | N/A |
| **Total** | **19** | **19/19** | **N/A** | **N/A** | — |

## Notas

- **`geometric` é experimental** (per `typst-paridade-definicoes.md` §P3, classe introduzida no Passo 150). Os números brutos são registados para calibração futura mas **não contam para a % agregada**: cristalino usa `FixedMetrics` (~0.6×size por char, monoespaçado) enquanto vanilla usa `FontBookMetrics` (proporcional via `ttf-parser`). Divergência geométrica é **estrutural**, não defeito (ADR-0054 perfil observacional graded cobre).
- **Cobertura declarada** (per inventário 148, pós-Passo 149): user-facing 54%, arquitectural 72%.
- **Esta matriz mede paridade observacional** contra vanilla para o subconjunto declarado como suportado pelo cristalino.
- **Coluna `Compila (cristalino)`** é baseline inicial enquanto a integração vanilla está pendente. Quando vanilla integration estiver em produção (DEBT-53 candidato), `text_content` e `structural` substituem `N/A` por contagens reais.
