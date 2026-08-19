# L0 — Passo 1086: Investigação — Espaçamento Entre Blocos de Equação Consecutivos

**Gate**: `ADR-0127` se a investigação confirmar necessidade de correcção —
mudança de comportamento por defeito. Este passo é só investigação.

**Base**: achado registado em nota externa (2026-08-12, seção 30) — 3 blocos de
equação consecutivos (`$ $`), transições com troca de sinal:
`cabeçalho→eq1: +1.96pt`, `eq1→eq2: −2.18pt`, `eq2→eq3: −2.17pt`. Ruído
intra-bloco (posição de sub/sobrescrito) é desprezível (0.00-0.09pt) — o
problema é especificamente o espaço **entre** blocos, correlacionado com a
altura de cada bloco (blocos com sub/sobrescrito mais alto, ex. `∇f(x*)+...`,
têm caixa delimitadora mais alta).

**Nota operacional herdada**: o registo original perdeu acesso ao repositório
`decalque` por reinício de sandbox. Os PDFs (`sec_30_crystalline.pdf`,
`sec_30_oracle.pdf`, `sec_30_vanilla.pdf`) não estão neste filesystem — se a
Parte 3 (remedição) for necessária, esses arquivos (ou equivalentes) precisam
de ser re-enviados.

---

## 1. Hipótese principal — mecanismo já documentado nesta conversa

`compiler/layout.md` (já visto nesta conversa, secção sobre `metrics.rs`)
documenta o mecanismo `FontMetrics::text_ink_bounds` (P813):

> "devolve (ascent, descent) ... medidos da união das bounding boxes reais dos
> glyphs ... Tem implementação **default conservadora** (`cap_height(size,
> style)`, `Pt(0.0)`) para métricas sem acesso a bboxes (`FixedMetrics`, stubs
> de teste); a implementação L3 com fonte real sobrescreve com
> `glyph_index + glyph_bounding_box`. Consumidor actual:
> `MathLayouter::layout_equation_measured` (extent da equação para
> **centragem/espaçamento de bloco**)."

Isto é exactamente o mecanismo que calcularia a "altura do bloco" mencionada no
pedido original. Hipótese a confirmar, não assumida: o `FontMetrics` real (L3,
`03_infra`) está a devolver ink bounds correctos para glifos simples, mas algo
no cálculo de extent de sub/sobrescrito (`x*`, `∇f(x*)+`) cai no fallback
conservador em vez do caminho real — ou o caminho real tem um erro específico a
glifos combinados (sub+base+sup na mesma caixa).

## 2. Ler antes de investigar mais

- `00_nucleo/prompts/compiler/layout/equation.md` (citado directamente no
  `layout.md` como o prompt do consumidor).
- `01_core/src/compiler/layout/equation.rs` (ou `math/layout/mod.rs`,
  dependendo de onde `layout_equation_measured` realmente vive — não presumir
  o caminho exacto sem confirmar).
- `03_infra/src/font_metrics.rs` — implementação real de `text_ink_bounds`
  (`FallbackFontMetrics`), especificamente para conteúdo com múltiplos glifos
  de tamanhos diferentes (base + sub/sobrescrito).

## 3. O que verificar

1. `layout_equation_measured` usa `text_ink_bounds` directamente, ou passa por
   outra camada que poderia estar a usar o fallback conservador por engano
   (ex.: `FixedMetrics` em vez de `FallbackFontMetrics` nalgum caminho de
   medição antecipada, mesmo padrão de bug já visto nesta conversa em
   `measure_content` vs `measure_content_real`, P904 Item 3)?
2. O cálculo do espaço **entre** blocos consecutivos usa o ink bound de cada
   bloco individualmente (`max(below_A, above_B)`, mesmo padrão do P1061) ou
   soma/subtrai de forma diferente? Confirmar a fórmula real antes de assumir
   que é o mesmo mecanismo de colapso já corrigido em P1059-1063 (que cobriu
   parágrafo/bloco/heading, não equação).
3. Reproduzir o sinal da troca: comparar o ink bound calculado pelo cristalino
   para os 3 blocos da seção 30 contra o que o vanilla usaria — se a correlação
   com sub/sobrescrito for confirmada, identificar o ponto exacto onde o
   cálculo diverge.

## 4. Scope — isto é equação, não o que já foi corrigido

P1059-1063 cobriram parágrafo↔bloco, bloco↔bloco, parágrafo↔heading. Bloco de
equação (`Content::Equation{block:true}`) pode ou não passar pelo mesmo
mecanismo `prev_block_below_pending`/`block_chain_active` — confirmar, não
assumir que já está coberto pela correcção anterior só porque o padrão
"colapso de margem" é semelhante.

## Critério de conclusão

- Arquivos do §2 lidos.
- Caminho real de `layout_equation_measured`/`text_ink_bounds` confirmado
  (fallback conservador vs `FallbackFontMetrics` real).
- Causa identificada com citação de código, não suposição.
- Se remedição for necessária: pedir os PDFs/documento de teste da seção 30
  (ou equivalente) antes de propor correcção, dado que o acesso original foi
  perdido.
