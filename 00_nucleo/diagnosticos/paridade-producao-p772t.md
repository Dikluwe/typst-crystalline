# P772t — Reconfirmação de `lacuna-inventario` (segunda rodada) e decisão de continuidade

> **Passo:** 772t
> **Data:** 2026-07-17
> **Commit-base:** working tree após P772s (não commitado no início deste passo).
> **Dependência:** P772e (primeira reconfirmação, metodologia).

---

## 1. Reconfirmação da lista `lacuna-inventario`

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

**Total de itens na lista:** 400 (inalterado — a lista-fonte não mudou desde P772e).

**Itens tratados desde P765a** (soma directa das linhas dos módulos cobertos, confirmada por `grep -v` de exclusão contra o mesmo comando, dando o mesmo total pelas duas vias):

| Módulo | Itens |
|---|---:|
| `foundations::calc` | 45 |
| `foundations::ops` | 20 |
| `math::style` | 32 |
| `layout::grid::resolve` | 24 |
| `diag` | 24 |
| `image::raster` | 13 |
| `pdf::accessibility` | 12 |
| `syntax::span` | 10 |
| `syntax::package` | 8 |
| `image::svg` | 7 |
| `foundations::scope` | 7 |
| `text::font::variations` | 5 |
| `text::font::metrics` | 5 |
| `text::font::info` | 5 |
| `text::font::book` | 3 |
| `text::font::exceptions` | 2 |
| **Total tratado** | **222** |

**Itens restantes:** 400 − 222 = **178**, em **66** módulos distintos (de 82 originais — 16 módulos já
totalmente tratados).

### Módulos restantes (topo, por contagem)

| Módulo | Itens restantes | Risco observável (ADR-0107: mecanismo vs linguagem) |
|---|---:|---|
| `typst_utils` | 14 | Baixo — infraestrutura Rust pura, sem exposição na linguagem |
| `typst` (root) | 8 | Médio — CLI/entrypoints |
| `typst_syntax::reparser` | 7 | Baixo — reparsing incremental é mecanismo (ADR-0107), não linguagem |
| `typst_library` (misc) | 7 | Variado |
| `typst_syntax::node` | 6 | Baixo — CST, mecanismo |
| `typst_syntax::ast` | 6 | Baixo — AST, mecanismo |
| `typst_library::foundations::target_` | 6 | Médio-Alto — link targets (`<label>`, refs) |
| `typst_utils::pico` | 5 | Baixo |
| `typst_syntax::lines` | 5 | Baixo |
| `typst_syntax::highlight` | 5 | Baixo (syntax highlighting, fora do compilador) |
| `typst_library::routines` | 5 | Baixo — infraestrutura de despacho |
| `typst_library::foundations::plugin_` | 5 | Médio — plugins WASM |
| `typst_library::visualize::image::pdf` | 4 | Médio-Alto — embutir PDF como imagem |
| `typst_library::math` | 4 | Médio |
| `typst_library::layout::frame` | 4 | Alto — núcleo de layout |
| `typst_library::foundations::fields` | 4 | Baixo-Médio |
| (resto, ≤3 cada) | ~52 | Maioritariamente baixo — `typst_eval::*` remanescente, `pico::*`, `loading::*` formatos pontuais, `layout::{em,abs,axes,corners,fragment}` (tipos utilitários) |

A maioria dos 178 itens restantes cai nas categorias `typst_utils`, `typst_syntax::{reparser,node,ast,lines,
highlight,kind}` — **mecanismo de parsing/CST/utilitário Rust**, explicitamente fora do critério de paridade
por ADR-0107 (paridade é com a linguagem — semântica/sintaxe/morfologia — não com a mecânica de execução).
Apenas um subconjunto pequeno tem risco de linguagem plausível: `foundations::target_` (6), `plugin_` (5),
`image::pdf` (4), `layout::frame` (4), `math` (4) — **~23 itens em 5 módulos**, muito menor que os ~60 itens
de 4 módulos identificados como próximo alvo em P772e.

---

## 2. Débito técnico acumulado (fora do inventário original)

| Item | Origem | Natureza | Tamanho estimado |
|---|---|---|---|
| Span em `Args` — 326 pontos em funções auxiliares sem `Args` na assinatura | P772s | Gap de precisão de diagnóstico (não é bug de comportamento) | Grande — threading em ~243 funções |
| Span por-argumento individual (paridade completa com vanilla `Arg.span`) | P772s (débito desde P740C) | Gap de precisão de diagnóstico | Muito grande — ~1200 pontos |
| §2.6 — avisos de depreciação de símbolos | P772l | Gap de feature | Pequeno, mas bloqueado — falta fonte de dados de depreciação |
| Colapso de espaço em Cantarell-VF (CFF2/HVAR) | P772o | **Bug confirmado** (não gap) — medido, reproduzido, causa não isolada | Médio — precisa instrumentar `shaper.rs` |
| Suporte real a SVG (7 itens de `image::svg`) | P772k | Gap de feature — já não é silencioso (P772p faz `native_image` erroar "SVG images are not supported yet" em vez de omitir) | Grande — nova dependência L3 |
| `table()` header/footer (variante do gap resolvido em P772i para `grid()`) | P772i | Gap de feature — extensão directa de padrão já implementado | Médio — campos novos em `TableElem` |
| Repeat-across-páginas de `grid.header`/`grid.footer` | P772i | Gap de feature | Médio-grande |

Diferença chave dos itens de `lacuna-inventario` restante: estes sete itens **já estão confirmados e
escopados** (não precisam de mais sonda para "descobrir" — precisam de decisão de implementação). O
`lacuna-inventario` restante, ao contrário, ainda está em fase de descoberta (a maior parte nunca foi
classificada item a item).

---

## 3. Rendimento acumulado (P765a → P772s)

| Módulo/item | Itens | Bugs reais (contagem conservadora — sem achados adjacentes) | Passo(s) |
|---|---:|---:|---|
| `foundations::calc`/`ops` | 65 | 2 | P765a |
| `diag` | 24 | 0 | P765a |
| `math::style` | 32 | 4 | P765b |
| `image::raster` | 13 | 2 | P772 (série original) |
| `pdf::accessibility` | 12 | 0 | P772a |
| `syntax::span` | 10 | 2 (cross-file + I/O) | P772b, P772d |
| `syntax::package` | 8 | 0 | P772c |
| `layout::grid::resolve` | 24 | 3 (place duplicado, header/footer, align per-cell) | P772f-j |
| `image::svg` | 7 | 0 (7 gaps classificados, 0 corrigidos — decisão consciente) | P772k |
| `foundations::scope` | 7 | 5 (fuga de âmbito, mutação de constante, captura, hint) | P772l, n, q, r |
| `text::font::*` | 20 | 1 confirmado+corrigido (Ubuntu Sans); +1 parcial (Cantarell, não contado aqui — ver débito §2) | P772m, o |
| **Total** | **222** | **19** | — |

**Achados fora do inventário original, não contabilizados na taxa** (mesma convenção usada em P772e para
P772d): P772p (formato de imagem inválido não era reportado — corrigido) e P772s (span em `Args` ausente —
parcialmente corrigido, débito registado em §2).

### Métricas

| Métrica | P772e (baseline) | P772t (agora) |
|---|---:|---:|
| Bugs por item | 9 / 164 = 5,5% | 19 / 222 = **8,6%** |
| Módulos com ≥1 bug | 4 / 8 = 50% | 7 / 11 = **63,6%** |

A taxa **subiu**, não caiu — a série continua a encontrar bugs reais a um ritmo igual ou maior que no início.
Isto não é, por si só, sinal para encerrar.

---

## 4. Decisão

**Opção escolhida: Mudar o foco para o débito técnico acumulado, com um resíduo pequeno e definido do
`lacuna-inventario` original mantido como candidato pontual (não sweep sistemático).**

Justificativa, com os números desta secção:

1. A taxa de bugs por item **subiu** de 5,5% para 8,6% — não há sinal de rendimento decrescente que
   justifique encerrar a série por si só.
2. Mas o **motivo** dessa taxa alta foi a escolha de módulos de alto risco linguístico em P772e
   (`grid::resolve`, `scope`, `font::*`, `svg`) — precisamente os que restavam de maior risco. O que
   sobra agora (178 itens, 66 módulos) é **dominado por mecanismo** (`typst_utils`, `syntax::{reparser,
   node,ast,lines,highlight,kind}` somam ~50 dos 66 módulos restantes) — fora do critério de paridade por
   ADR-0107. Continuar a varrer estes por ordem de tamanho repetiria o padrão de P772a/P765a (`diag`,
   `pdf::accessibility`: 36 itens, 0 bugs) — módulos grandes de baixo risco linguístico.
3. Em paralelo, a série já acumulou **7 itens de débito técnico confirmado e escopado** (§2), incluindo
   um **bug confirmado** (Cantarell-VF) que não precisa de mais sonda — só de decisão de implementação.
   Ao contrário do resíduo do inventário (ainda por classificar item a item), este débito já passou pela
   fase de descoberta; o próximo passo é sempre "decidir corrigir", não "sondar se há bug".
4. Impacto observável directo do débito acumulado supera o impacto esperado do resíduo de baixo risco:
   um bug de tipografia confirmado (Cantarell) e dois gaps de feature bem delimitados (`table()` header/
   footer, extensão directa de P772i) têm efeito visível e imediato; o resíduo do inventário, à taxa actual
   ponderada pelo risco (~23 itens de risco plausível em 5 módulos, não 178), rende no máximo ~2 bugs
   adicionais estimados — mas exige nova fase de classificação item a item primeiro (custo de sonda alto
   por item de baixo volume).

**Não** é a opção "encerrar" — o resíduo de risco plausível (`foundations::target_`, `plugin_`,
`image::pdf`, `layout::frame`, `math` — 23 itens, 5 módulos) fica registado como candidato a **um** passo
pontual futuro, não descartado, mas subordinado ao débito de maior impacto já confirmado.

---

## 5. Próximo passo

**P772u** — corrigir o colapso de espaço em Cantarell-VF (CFF2/HVAR), item de maior prioridade do débito
acumulado por ser o único **bug confirmado** (não gap) da tabela §2, já escopado por P772o (precisa
instrumentar `shaper.rs`, mesma área de código já tocada em P772m/o — sem descoberta nova necessária).

Candidatos seguintes, em ordem de prioridade decidida por impacto/custo:

1. `table()` header/footer — extensão directa e de baixo risco do padrão já validado em P772i.
2. Resíduo de `lacuna-inventario` de risco plausível (`foundations::target_`, `plugin_`, `image::pdf`,
   `layout::frame`, `math` — 23 itens) — um passo de classificação, não um sweep amplo.
3. Repeat-across-páginas de `grid.header`/`grid.footer` — médio-grande, depende do mesmo `TableElem`/
   `GridElem` tocado no item 1.
4. Span em `Args` (326 pontos restantes) e span por-argumento (~1200) — maior volume, menor urgência
   (gap de precisão de diagnóstico, não de comportamento); candidato a passo dedicado só depois dos itens
   acima.
5. SVG real — maior custo (nova dependência L3) e, desde P772p, já não silencioso (erro explícito
   "SVG images are not supported yet"); prioridade mais baixa entre os gaps de feature.

---

## Critério de fecho do passo (`typst-passo-772t.md`)

- [x] Lista `lacuna-inventario` restante reconfirmada por contagem directa (178 itens, 66 módulos).
- [x] Débito técnico acumulado confirmado e priorizado por impacto observável (tabela §2, ordenado no §5).
- [x] Rendimento acumulado calculado (19 bugs / 222 itens = 8,6%; 7/11 módulos com ≥1 bug = 63,6%).
- [x] Decisão registada com base nos números: focar no débito técnico acumulado (Cantarell primeiro),
      mantendo um resíduo pequeno e definido do inventário original como candidato pontual, não sweep.
- [x] Próximo módulo/item identificado: P772u — Cantarell-VF (CFF2/HVAR).
