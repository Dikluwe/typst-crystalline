---
# P772t — Reconfirmação de `lacuna-inventario` (segunda rodada) e decisão de continuidade

> **Passo:** 772t
> **Data:** 2026-07-17
> **Foco:** Segunda reconfirmação da lista `lacuna-inventario`, no mesmo espírito de P772e — desta vez depois de cobrir `image::raster` (P772), `pdf::accessibility` (P772a), `syntax::span`/`package` (P772b-d), `layout::grid::resolve` (P772f-j), `image::svg` (P772k), `foundations::scope` (P772l, P772n, P772q, P772r), `text::font::*` (P772m, P772o), e dois achados fora do inventário original mas dentro do mesmo espírito (formato de imagem inválido — P772p; span em `Args` — P772s). Medir o rendimento acumulado e decidir se a varredura sistemática continua, muda de forma, ou encerra.
> **Tipo:** Sonda + Decisão registada (regra 1).
> **Tamanho:** S.
> **ADR-0108 EM VIGOR** — decisão baseada em números, não em cansaço ou impressão.
> **Dependências:** P772e (primeira reconfirmação, metodologia), todos os lotes desde então.

---

## Sonda — reconfirmar a lista restante

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

Excluir todos os módulos já tratados (a lista completa está no cabeçalho acima) e listar o que resta.

### Considerar também o débito técnico acumulado, não só o inventário original

Ao contrário de P772e, esta rodada tem um segundo tipo de pendência: itens encontrados **fora** do inventário original de `lacuna-inventario`, mas com a mesma natureza de dívida de linguagem, ainda não corrigidos:

| Item | Origem | Tamanho estimado |
|---|---|---|
| Span em `Args` — 326 pontos em funções auxiliares sem `Args` na assinatura | P772s | Grande — threading em ~243 funções |
| Span por-argumento individual (paridade completa com vanilla `Arg.span`) | P772s (débito antigo, era P740C) | Muito grande — ~1200 pontos |
| §2.6 — avisos de depreciação de símbolos | P772l | Pequeno, mas precisa de fonte de dados de depreciação que não existe hoje |
| Colapso de espaço em Cantarell-VF (CFF2/HVAR) | P772o | Médio — precisa instrumentar `shaper.rs` |
| Suporte real a SVG (7 itens de `image::svg`) | P772k | Grande — nova dependência L3 |
| `table()` header/footer (variante do gap resolvido em P772i para `grid()`) | P772i | Médio — campos novos em `TableElem` |
| Repeat-across-páginas de `grid.header`/`grid.footer` | P772i | Médio-grande |

### Calcular o rendimento acumulado (todos os lotes desde P765a)

| Módulo/item | Itens (se aplicável) | Bugs reais | Observação |
|---|---:|---:|---|
| `foundations::calc`/`ops` | 65 | 2 | P765a |
| `diag` | 24 | 0 | P765a |
| `math::style` | 32 | 4 | P765b |
| `image::raster` | 13 | 2 (+desdobramentos DPI/rotação/clip/colorspace) | P772-P778 |
| `pdf::accessibility` | 12 | 0 | P772a |
| `syntax::span` | 10 | 2 (cross-file + I/O span) | P772b, P772d |
| `syntax::package` | 8 | 0 | P772c |
| `layout::grid::resolve` | 24 | 3+ (place duplicado, header/footer, align per-cell) | P772f-j |
| `image::svg` | 7 | 0 implementado, 1 achado adjacente corrigido (formato inválido) | P772k, P772p |
| `foundations::scope` | 7 | 5 (fuga de âmbito, mutação de constante, captura, hint) | P772l, n, q, r |
| `text::font::*` | ~20 | 1 confirmado e corrigido (Ubuntu Sans), 1 parcial (Cantarell) | P772m, o |

Somar para obter a taxa global actualizada e comparar com a taxa de P772e (9/164, 50% dos módulos).

---

## Decisão a registar

Com os números:

| Opção | Quando escolher |
|---|---|
| Continuar a varredura por módulo (mesmo padrão) | Se a taxa de bugs por módulo se mantiver alta e o restante do `lacuna-inventario` original ainda tiver itens de risco não cobertos |
| Mudar o foco para o débito técnico acumulado (span em Args, SVG, table header/footer, etc.) em vez de módulos novos | Se o inventário original estiver com rendimento decrescente mas os itens de débito acumulado tiverem impacto observável maior |
| Encerrar a varredura sistemática e consolidar um resumo final | Se ambos os sinais (inventário + débito) sugerirem que o retorno marginal não justifica mais passos dedicados |

Registar com os números concretos, não com impressão subjetiva.

---

## Critério de fecho do passo

- [ ] Lista `lacuna-inventario` restante reconfirmada por contagem directa.
- [ ] Débito técnico acumulado (tabela acima) confirmado e priorizado por impacto observável.
- [ ] Rendimento acumulado calculado (taxa global de bugs por módulo/item desde P765a).
- [ ] Decisão registada com base nos números: continuar por módulo, focar em débito acumulado, ou encerrar.
- [ ] Se continuar: próximo módulo/item identificado.
- [ ] Se encerrar: resumo da série completa (P765a-P772t), com o que foi coberto e o que ficou fora por decisão consciente.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772t.md`.

---

## Próximo passo

Conforme a decisão registada.
