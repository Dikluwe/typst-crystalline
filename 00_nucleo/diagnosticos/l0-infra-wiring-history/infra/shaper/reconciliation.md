# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/reconciliation.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `89d7bd8ac2d31ba7516fae087bf893d57d414cb7e0025dece54bb98cef8e5e42`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 6146b980
prompt: infra/shaper/reconciliation
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

# Prompt L0 — infra/shaper/reconciliation

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:2404-2637` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Possuir fix_line_positions e helpers. Agrupar baselines com tolerância de 0.5pt e ordenar por x físico. Preservar gaps e borda direita em RTL e diferenças acumuladas em LTR. Precedência da medição atual: ignorar a linha inteira que contenha TextShaped com style.math ou Glyph, como faz o consumer antes da extração. O texto histórico P975 abaixo não autoriza mudar esse bypass para tratamento por item nesta refatoração. Não recursar nem ampliar os casos tratados.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

## P1140.12 — reconciliação RTL preserva gaps e borda direita

Depois de shaping, `fix_line_positions` pode substituir larguras estimadas por
advances reais, mas não pode recalcular decisões de layout. Para uma linha RTL
fisicamente ordenada por x, captura antes da mutação:

```text
gap[i] = x[i+1] - (x[i] + w_est[i])
right_edge = x[last] + w_est[last]
```

e reconstrói da direita para a esquerda:

```text
x_real[last] = right_edge - w_real[last]
x_real[i] = x_real[i+1] - gap[i] - w_real[i]
```

Isso preserva a borda direita e os gaps normais ou justificados decididos em
L1. Valores posicionais medidos são apenas oracles e nunca constantes. Math
continua excluído conforme P975; item não textual usa largura estável.


## §P582 — Redistribuição de posições x após shaping

**Data:** 2026-07-06

Corrige posições x de todos os itens de uma linha usando os advances reais dos glyphs obtidos no shaper. O Layouter usa `FallbackFontMetrics` (estimativa) para calcular o cursor_x de cada palavra; o shaper usa a fonte real (rustybuzz). Quando as duas fontes diferem (ex.: bold resolve face diferente), as posições ficam descasadas. Esta passagem redistribui as posições dentro de cada linha acumulando a diferença (width_real - width_est) de forma a manter todos os espaçamentos e layouts relativos intactos.

### Chamada na pipeline (`03_infra/src/pipeline.rs`)

```rust
let doc = crate::shaper::shape_document(world, doc);
let doc = crate::shaper::fix_line_positions(world, doc);
```

### Comportamento

1. **Agrupamento por linha**: Para cada página, percorre `page.items` e agrupa os itens por baseline y (dentro de tolerância 0.5pt).
2. **Redistribuição de x**: Para cada linha, ordena os itens por x original (esquerda para direita).
   - Se a linha for RTL (se algum item tiver direção RTL):
     - Ancoramos o item mais à direita (o início da linha RTL) na sua coordenada original.
     - Percorremos a linha de trás para frente (da direita para a esquerda) aplicando `x_new = x_orig + shift`.
     - O shift acumula negativamente a diferença: `shift -= w_real - w_est` do item à esquerda.
   - Caso contrário (linha LTR normal):
     - Ancoramos o primeiro item (mais à esquerda) na sua coordenada original.
     - Percorremos a linha da esquerda para a direita aplicando `x_new = x_orig + shift`.
     - O shift acumula positivamente a diferença: `shift += w_real - w_est` de cada item.

### Testes adicionados P582

- `p582_single_item_unchanged`: item único — posição inalterada.
- `p582_two_items_redistributed`: dois itens — segundo recebe o shift cumulativo de largura.
- `p582_two_lines_independent`: itens em linhas diferentes redistribuídos independentemente.
- `p582_zero_glyphs_zero_advance`: item com glyphs vazios não altera a posição do seguinte.
- `p582_y_unchanged`: coordenada y mantida inalterada.
- `p582_rtl_redistributed`: linha RTL com âncora à direita e shift para a esquerda.

---


## P975 — `fix_line_positions` não reconcilia itens de matemática (`style.math`)

**Medição** (`typst-passo-975` Fase B): com a IC incluída no advance math
(`infra/font_metrics.md` §P975), o `fix_line_positions` (P582) movia os
itens seguintes a uma letra itálica math para trás exactamente pela IC
(ex.: `tau(G)`: o `(` voltava de 5.929pt para 4.806pt após o 𝜏). A
divergência `w_real − w_est` desses itens é **intencional**: a IC é um
termo de posicionamento do layout math (o glifo desenhado mantém o
advance puro), não uma discrepância de fallback de fonte. A reconciliação
de P582 existe para prosa (estimativa L1 vs largura shaped real); itens
math são posicionados com métricas reais desde P544/P893.

**Decisão**: itens com `style.math == true` não contribuem `w_real −
w_est` para o acumulador de shift (tratados como `(0.0, 0.0)` no cálculo),
mas continuam a **receber** o shift acumulado por itens de prosa na mesma
linha (math inline em prosa partilha a linha). Sem o termo de IC (prosa
ou math sem IC), o comportamento é bit-a-bit o de antes.
