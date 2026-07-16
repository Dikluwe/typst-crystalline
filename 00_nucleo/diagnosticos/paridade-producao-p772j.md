---
# Diagnóstico — P772j: L0 + reimplementação do alinhamento efectivo per-célula
em `grid()`

> **Passo:** 772j
> **Data:** 2026-07-16
> **Commit-base:** `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b` (working tree com
> alterações não commitadas).
> **Tipo:** L0 + Implementação (reversão do código órfão + reimplementação verificada
> contra o vanilla).

---

## Passo 0 — Reversão do código órfão

Confirmado o âmbito exacto com `git diff HEAD -- grid.rs` antes de reverter: o único
bloco não-explicado era o comentário `// P772f — aplicar align efectivo da célula...`
envolvendo o corpo num `Content::Place`. Revertido **manualmente** (não `git checkout
--`, que teria também revertido a linha `@prompt-hash` legitimamente actualizada por
P772g). `cargo test --workspace --release` confirmado verde imediatamente após a
reversão (4163 testes, mesma contagem de antes — o código órfão não estava a corrigir
nada por acidente).

---

## Sonda — mecanismo e precedência reais do vanilla

### Mecanismo: `Content::Align`, nunca `Content::Place`

`lab/typst-original/crates/typst-layout/src/rules.rs`:

```rust
const GRID_CELL_RULE: ShowFn<GridCell> = |elem, _, styles| {
    show_cell(elem.body.clone(), elem.inset.get(styles), elem.align.get(styles))
};

fn show_cell(mut body: Content, inset: .., align: Smart<Alignment>) -> SourceResult<Content> {
    if inset != Sides::default() { body = body.padded(inset); }
    if let Smart::Custom(alignment) = align { body = body.aligned(alignment); }
    Ok(body)
}
```

Confirmado por grep (`grep -rn ".align" typst-layout/src/grid/`): a camada de LAYOUT do
grid **não referencia `.align` de todo** — a resolução acontece antes, num show-rule
(eval-time), envolvendo o corpo em `Align` (`.aligned()`), nunca em `Place`. Isto
confirma que o código órfão (que usava `Place`) estava architecturalmente errado, não
só numericamente divergente.

### Precedência: fold por eixo, não substituição total

Repro `#grid(rows: 2cm, align: horizon, grid.cell(align: left)[Hello])` no vanilla
(`mutool trace`): a posição Y corresponde a **Horizon** (herdado do grid), não Top
(o default se `left` substituísse o `Align2D` inteiro). Confirma fold por eixo — H e V
resolvidos independentemente. O cristalino usava `cell_align.or(self.cell_align)`
(`.or()` do `Option<Align2D>` inteiro), que descartava o V do grid sempre que a célula
especificava qualquer eixo — divergência de precedência real, não só mecânica.

---

## Implementação

### `grid.rs` — precedência por eixo

```rust
let effective_align = match (cell_align, self.cell_align) {
    (None, None) => None,
    (Some(c), None) => Some(c),
    (None, Some(g)) => Some(g),
    (Some(c), Some(g)) => Some(Align2D { h: c.h.or(g.h), v: c.v.or(g.v) }),
};
```

### `grid.rs` — envolver em `Content::Align`, não `Content::Place`

Quando `effective_align` é `Some`, o corpo da célula é envolvido em
`Content::Align { alignment: effective_align, body: cell.clone() }` antes de
`layout_sub_frame`. Reutiliza directamente a disciplina de "consumidor absoluto"
estabelecida em P772g — nenhuma alteração adicional a `placement.rs::layout_align` foi
necessária para este mecanismo especificamente (já compõe correctamente quando
invocado a partir desta posição).

### Achado adicional, corrigido por bloquear directamente a validação: `measure_content`
sem braço para `Content::Text`

Durante a validação, `layout_align` continuava a produzir uma posição claramente
errada (deslocamento de ~13.5pt, idêntico ao valor do código órfão revertido) mesmo
com o mecanismo já corrigido. Instrumentação directa revelou a causa: `content_w`, em
`layout_align`, vinha do helper `measure_content` (`helpers.rs`) — uma função **sem
acesso a métricas de fonte** (não é método do `Layouter`), que só tem braços para
`Content::Shape`/`Content::Sequence`; para `Content::Text` cai no catch-all
`_ => (0.0, 0.0)`. Com `content_w=0`, `resolve_alignment` centra como se o conteúdo
tivesse largura zero (desloca por `avail_w/2` em vez de `(avail_w - largura_real)/2`).

Esta é a **mesma classe** de bug já corrigida em P772f (bug A, `measure_content_constrained`)
mas numa função irmã diferente (`measure_content`, sem métricas), e **pré-existe a este
passo** — afecta qualquer `align()` com texto directo, não só alinhamento de célula de
grid. Note-se uma correcção anterior (P772f, relatório §2.3) que reportou "20.247 vs
49.546" para um repro de `align(center,[Hello])` sem grid-level align — **essa medição
estava errada**; reexecutada agora, dá o mesmo `x=33.772` que o bug aqui descrito
produz. Regisco este erro de proveniência explicitamente (regra de registo de
proveniência do CLAUDE.md): a medição original de P772f não foi reproduzida
correctamente antes de ser escrita no relatório.

**Corrigido** (`placement.rs::layout_align`): `content_w` passa a ser medido a partir
dos `sub_items` já layoutados, via `FontMetrics::line_content_right` (o mesmo
mecanismo usado por `measure_content_real`), não do helper `measure_content` sem
métricas:

```rust
let sub_item_refs: Vec<&FrameItem> = sub_items.iter().collect();
let content_right_abs = self.metrics.line_content_right(&sub_item_refs);
let content_w = (content_right_abs - origin_x_abs).max(0.0);
```

`measure_content` (`helpers.rs`) fica **inalterado** — continua a servir
`Content::Place`/`Content::Transform`, fora do âmbito deste passo.

---

## Validação

### `#grid(align: center, [Hello], [World])` vs vanilla

| | x medido | vanilla |
|---|---|---|
| Antes (código órfão, `Place`) | 33.772 | 20.247 |
| Depois (fold + `Align` + fix de `measure_content`) | 21.622 (mesma fonte do sistema) / 21.995 (DejaVu Sans, fonte idêntica confirmada) | 20.247 |

Divergência residual ~1.4–1.75pt (era 13.5pt). **Causa da divergência residual,
medida, não deixada como incógnita:** `resolved_widths` (dimensionamento automático de
colunas, via `measure_content_constrained`) mede a largura do texto palavra-a-palavra
(aproximação), enquanto `content_w` (agora corrigido) mede a partir do resultado
**real** do shaping (`line_content_right`). As duas medições do "mesmo" texto diferem
ligeiramente — arquitectura de duas passagens (medir→colocar) já documentada e aceite
desde P233 (`grid.rs`, comentário "Two-pass measure→place... Resolução completa
min-content/max-content negotiation continua DEBT-34d-rest"), não introduzida nem
alargada por este passo. Confirmado com fonte idêntica em ambos os lados (DejaVu Sans,
`adv` de cada glifo idêntico byte-a-byte no `mutool trace`) — a divergência não é por
substituição de fonte. Classificado como **divergência mecânica aceite (ADR-0107)**:
paridade é com a língua (texto centra-se, não sai da coluna), não com o valor exacto
em pontos de uma medição de largura que o próprio motor já trata como aproximada numa
fase e exacta noutra.

### Precedência (fold por eixo)

`#grid(align: horizon, grid.cell(align: left)[Hello])`: X idêntico entre `align: top`
e `align: horizon` a nível de grid (13.498 em ambos, H fixo vindo da célula); Y difere
claramente entre os dois casos (78.734 vs 99.886, local) — confirma que V é herdado do
grid independentemente do H da célula. Comparação X directa contra vanilla:
crystalline `x=13.498`, vanilla `x=13.498313` — **essencialmente exacto**.

### Checklist de sub-layouts

`#columns(2, grid(columns:1, align: center, [Hello]))`: x=21.622, **idêntico** ao caso
sem `columns()` — confirma que o mecanismo compõe correctamente aninhado em `columns`
(consistente com a auditoria de P772g, que já validara `columns.rs` sem necessidade de
alteração).

### Testes de regressão novos (prefixo `p772j_`)

- `p772j_grid_align_center_nao_diverge_uma_coluna_inteira_do_vanilla` — falha se a
  divergência voltar a ser da ordem de uma coluna inteira (tolerância 5pt, não
  byte-exacta, dada a divergência mecânica residual documentada acima).
- `p772j_grid_align_fold_por_eixo_preserva_eixo_do_grid` — falha se o fold por eixo
  regredir para `.or()` do `Align2D` inteiro.

### Suite completa

- `cargo test --workspace --release`: **4165 passed (typst-core lib, +2 novos) + 644 +
  33 + 29 + 2 + 2, 0 failed**.
- `crystalline-lint .`: **0 violations** (mesmo warning V7 pré-existente e não
  relacionado).

---

## Critério de fecho — checklist

- [x] Código órfão revertido, escopo confirmado, testes continuam verdes.
- [x] Precedência de `align` em `grid()` confirmada contra o vanilla real (fold por
      eixo), não assumida.
- [x] L0 escrito antes do código (`00_nucleo/prompts/rules/layout.md` §"Alinhamento
      efectivo per-célula", hash `a214cd68`).
- [x] Implementação nova, reutilizando a disciplina de coordenadas de P772g.
- [x] `#grid(align: center, ...)` bate com o vanilla dentro de tolerância documentada
      (residual mecânico de medição de largura, não um bug de mecanismo/precedência).
- [x] Checklist de sub-layouts (`columns`).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772j.md`.

---

## Próximo passo

Com P772g e P772j fechados (P772i em curso em paralelo), a linha de achados extra de
P772f está resolvida. Retomar a varredura da stdlib: `visualize::image::svg` (7 itens),
`foundations::scope` (7 itens), `text::font::*` (~22 itens).

Achado residual não fechado neste passo (mecânico, não bloqueante): a discrepância
entre a medição aproximada de largura de coluna (`measure_content_constrained`,
palavra-a-palavra) e a medição real de largura de conteúdo alinhado (`line_content_right`,
pós-shaping) pode, em casos extremos, produzir alinhamento visualmente perceptível mas
pequeno. Não é um scope-out novo — é a mesma limitação de arquitectura de duas
passagens já registada desde P233; mencionado aqui só para não ficar "descoberto por
acidente" outra vez.
