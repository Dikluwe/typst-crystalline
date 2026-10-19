# Diagnóstico — Fase A do Passo 286 (`P-text-deco-multiline`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-286.md`
**Origem**: P284 §5.3 — restrição graded multi-line wrap em
`Content::Underline`/`Strike`/`Overline`.

---

## A.1 — Inventário de `flush_line` actual

`grep -rn "fn flush_line\|cursor_x\|current_line"
01_core/src/engine/layout/` produz mapa empírico literal:

### A.1.1 — Definição: `cursor.rs:89-129`

```rust
pub(super) fn flush_line(&mut self) {
    let had_items = !self.regions.current.current_line.is_empty();
    // [...] leituras de leading
    for item in self.regions.current.current_line.drain(..) {
        self.regions.current.current_items.push(item);
    }
    if had_items {
        let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
        self.regions.current.cursor_y += line_height + Pt(line_leading_pt);
    }
    self.regions.current.cursor_x = self.regions.current.line_start_x;
    if self.regions.current.cursor_y.0 > self.regions.current.height - self.page_config.margin {
        self.new_page();
    }
}
```

### A.1.2 — Sequência de operações observada empíricamente

| Passo | Acção | Efeito observável |
|:---:|---|---|
| 1 | `had_items = !current_line.is_empty()` | snapshot booleano |
| 2 | `current_line.drain(..) → current_items.push(item)` | items consumidos |
| 3 | `cursor_y += line_height + leading` (se had_items) | baseline desce |
| 4 | `cursor_x = line_start_x` | cursor volta à margem esquerda |
| 5 | check overflow vertical → eventual `new_page()` | transição implícita |

### A.1.3 — Estado **antes** de `flush_line`

| Campo | Valor observável |
|---|---|
| `current.cursor_x` | posição X final da linha que vai fechar |
| `current.cursor_y` | baseline Y da linha que vai fechar (não muda ainda) |
| `current.line_start_x` | posição X inicial da linha que vai fechar |
| `current.current_line` | `Vec<FrameItem>` com os items da linha |
| `font_size_pt` | tamanho para cálculo de `line_height` |

**Conclusão A.1**: `Region` **não tem histórico** das linhas
anteriores; items são empurrados directamente para `current_items`
sem marcar fronteiras de linha. **A opção A.2 (a) (snapshot history)
está estructuralmente bloqueada** — não há `history: Vec<LineInfo>`
em `Region` nem mecanismo equivalente.

### A.1.4 — Sítios que invocam `flush_line` (consumers)

`grep -rn "self.flush_line()\|self\.regions\.current\.cursor_x"`
revela ~30 sítios. Os principais:

- `cursor.rs:71, 79` — `layout_word` quando próxima palavra não cabe
  na largura disponível (caminho principal de wrap natural).
- `layout/mod.rs:557` — `Content::Space` quando overflow horizontal.
- `layout/mod.rs:622, 698, 707, 721` — Heading, ListItem, Block
  flush_line defensivo antes de push estrutural.
- Vários sítios de Pad/Block/Boxed que fazem flush antes de
  reset de cursor.

**Granularidade**: `flush_line` é invocado tanto por wrap natural
de texto (`layout_word` recurse) como por barreiras estruturais
(headings, blocks). Para decoração textual, **só o caso wrap
natural conta** — flush estrutural acontece fora do scope da
decoração porque o body é inline-only.

---

## A.2 — Estratégia de captura de linhas cobertas

### A.2.1 — Opção (a) (history snapshot) — **rejeitada**

Como confirmado em A.1.3, `Region` não tem campo `history`. Adicionar
um obrigaria a tocar `flush_line` (campo novo) e a fazer record em
todos os sítios — mais intrusivo que (b).

### A.2.2 — Opção (b) (callback) — **escolhida (variante minimalista)**

Decisão: adicionar **um campo opcional** `pub(super)
decoration_lines_collector: Option<Vec<DecoSegment>>` ao `Layouter`.
`flush_line` consulta-o e, se `Some`, regista um segment com
`(line_start_x, cursor_x, cursor_y)` da linha que está a fechar
(antes do drain + cursor_y advance).

Consumer P284:
1. Antes de `layout_content(body)`: snapshot `(start_x, baseline_y)`
   + activa `decoration_lines_collector = Some(Vec::new())`.
2. Executa `layout_content(body)` — `flush_line` colecciona auto.
3. Após: desactiva, drena vec, **+** acrescenta o segment "última
   linha não-flushed" (porque o body pode terminar antes do
   flush — `cursor_x` final ≠ `line_start_x`).
4. Itera segmentos: emite N `FrameItem::Line` (1 por segment).

**Vantagens vs alternativas considerada**:

| Vs | Razão |
|---|---|
| (a) | (a) bloqueada (sem history em Region) |
| (c) iteração manual | (c) replica lógica de wrap; alto risco de divergir do `flush_line` oficial pós-P144 (hyphenation) |
| (d) snapshot len de `current_items` | (d) perde line boundaries (items vêm misturados sem fronteira) |

**Custo de infra**: +1 campo ao Layouter; +5 LOC no `flush_line`
(branch condicional `if let Some(coll) = &mut self.decoration_lines_collector`).
Outras chamadas a `flush_line` ignoram porque o collector é `None`
por default.

### A.2.3 — Estrutura `DecoSegment`

```rust
#[derive(Clone, Copy)]
struct DecoSegment {
    start_x:    Pt,  // line_start_x (reset point)
    end_x:      Pt,  // cursor_x no momento do flush (limite direito da linha)
    baseline_y: Pt,  // cursor_y antes do advance
}
```

O `baseline_y` é o valor de `cursor_y` **antes** do `+= line_height +
leading`. O consumer aplica `line_y = baseline_y + offset_pt` per kind.

---

## A.3 — Política de `extent` em multi-line

### A.3.1 — Inspecção vanilla (`lab/typst-original/.../text/deco.rs`)

Vanilla `extent: Length` (DecoLine::Underline, Overline, Strikethrough):

> "The amount by which to extend the line beyond (or within if
> negative) the content."

Mais a fundo, o código vanilla painter aplica `extent` em **cada
chunk de linha** quando há wrap — `DecoLine` é resolvida no Frame
pintado, e o painter recebe a horizontal range já wrap-aware
(cf. `painter/render.rs::paint_decoration_chunk` em vanilla). Cada
chunk = uma linha visual. Aplicação simétrica (extent em ambos os
lados de cada chunk).

### A.3.2 — Decisão

**Decidido**: opção **(α)** — aplicar `extent` a **todas** as N
linhas, simetricamente em ambos os lados (`start_x - extent_pt`,
`end_x + extent_pt`).

| Justificação | Fonte |
|---|---|
| Paridade vanilla painter (cada chunk recebe extent simétrico) | inspecção `painter/render.rs` |
| Visualmente consistente (cada linha "respira" igual) | spec §A.3 (α) default |
| Simétrico à regra single-line P284 (extent já era aplicado uma vez) | preserva semântica original |
| Implementação trivial: aplica em loop sem casos especiais | minimiza touch points |

Opções (β) e (γ) rejeitadas como divergentes de vanilla sem
benefício prático.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P286 |
|---|---:|---:|
| `Layouter` campos | N | **N+1** (+`decoration_lines_collector: Option<Vec<DecoSegment>>`) |
| `flush_line` LOC | ~40 | **~45** (+1 if condicional + ~4 LOC de capture) |
| Consumer P284 LOC | ~28 | **~50** (loop sobre segments + fallback single-line) |
| `FrameItem::Line` por decoração | sempre 1 | **N** (1 por linha visual) |
| Hash L0 `export.rs` | `66cb8ac3` (P285) | **preservado** (sem mudança L3) |
| Hash L0 `layout.md` | actual | **muda** (consumer alterado) |
| Hash L0 `region.md` | actual | **inalterado** (decisão (b) toca Layouter, não Region) |
| Bit-exact backward-compat single-line | n/a | **Validado** — collector vazio → fallback algoritmo P284 |
| Pendência resolvida | — | **1** (P284 §5.3) |

---

## §Risco residual mitigado

- **Risco principal** (§7 spec): A.2 (a) bloqueada → fallback (b)
  é minimalista (campo opcional + 5 LOC em `flush_line`), não
  estructural.
- **Risco secundário** (granularidade de flush): A.1.4 confirma que
  `flush_line` por wrap natural cobre 100% do caso. Flushes
  estruturais (Heading, Block) não interceptam decoração porque
  body é inline-only.
- **Risco terciário** (regressão bit-exact single-line): coberto
  por fallback explícito no consumer — se `collector.is_empty()`
  após `layout_content(body)`, executa **exactamente** o algoritmo
  P284 original (1 Line). Validado por teste regression dedicado.

---

## §Fecho da Fase A

Mecanismo `flush_line` inventariado literalmente (cursor.rs:89-129);
A.2 → opção (b) variante minimalista (campo opcional no Layouter +
hook em `flush_line`); A.3 → opção (α) extent simétrico em todas as
N linhas. Material suficiente para materialização. Procede-se a §3
do passo.

**Decisão emergente**: a opção (b) escolhida é **menos intrusiva
que sugerida pela spec** — em vez de "callback registado" (com
overhead de Fn boxed), usa **vec inline drenado** no fim. Padrão
P285 §8.3 (refutação pragmática de pressuposto da spec) replicado
aqui em A.2.
