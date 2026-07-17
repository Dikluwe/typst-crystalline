# Diagnóstico — Fase A do Passo 284 (`P-text-deco-emit`)

**Data**: 2026-05-18
**Spec mãe**: `00_nucleo/materialization/typst-passo-284.md`
**Inventário-fonte**: `lab/typst-original/crates/typst-library/src/text/deco.rs`
**Inspeção L1/L3**: `01_core/src/entities/{content.rs,layout_types.rs}`,
`01_core/src/engine/layout/mod.rs`, `03_infra/src/export.rs`.

---

## A.1 — Scope dos atributos vanilla

Vanilla `text/deco.rs` declara `UnderlineElem` (linhas 14-73), `OverlineElem`
(82-147) e `StrikeElem` (156-206). Atributos comparados, com decisão por bucket
(per spec §A.1):

| Atributo | `underline` | `strike` | `overline` | Bucket | Justificação |
|----------|:---:|:---:|:---:|:---:|---|
| `body: Content` (required) | ✓ | ✓ | ✓ | **1 (materializar)** | base — sem ela não há decoração |
| `stroke: Smart<Stroke>` | ✓ | ✓ | ✓ | **1 simplificado** | aceitar `Option<Color>` (paint puro); **rejeitar** o objecto `Stroke` rico (paint+thickness+cap+dash). Justificação: A.7 linha 201 da cobertura lista `stroke(...)` como `parcial`; passo dedicado resolve. Default cristalino: `Color::rgb(0, 0, 0)` |
| `offset: Smart<Length>` | ✓ | ✓ | ✓ | **1** | override do offset Y default; resolvido em pt via `Length::resolve_pt(font_size_pt)` |
| `extent: Length` | ✓ | ✓ | ✓ | **1** | extensão horizontal além do body (positiva ou negativa); default 0 |
| `evade: bool` | ✓ | — | ✓ | **scope-out semântico** | descender skipping requer cálculo glifo-a-glifo; passo dedicado per ADR-0054 graded. **Strike não tem** este atributo em vanilla (assimetria intencional) |
| `background: bool` | ✓ | ✓ | ✓ | **scope-out cosmético** | z-order; baixo valor visível neste passo. ADR-0054 graded |

**Decisão final** — três variants partilham `(body, stroke?, offset?, extent?)`,
todos com sub-fold sobre tipos já materializados em L1 (Color RGB, Length).

### Observação sobre N≥3 padrão emergente (per spec §7 risco secundário)

`Underline + Strike + Overline` é a 4ª aplicação consecutiva (após Block,
Boxed, Stack em P156G/H/I) do padrão "variant rico com `body` + atributos
cosméticos opcionais". Patamar N=4 atinge 80% do gatilho histórico ADR-0065
(N=5). Registado para promoção a ADR meta caso o próximo passo (qualquer
variant rico com `body` + cosméticos) ultrapasse N=5. **Não é objectivo de
P284 resolver isto.**

---

## A.2 — Helper único vs três em export.rs

### Descoberta empírica

`FrameItem::Line { start: Point, end: Point, thickness: f64 }` **já existe**
em `01_core/src/entities/layout_types.rs:184-188` e tem emit PDF
`q {w} w {x1} {y1} m {x2} {y2} l S Q` em `03_infra/src/export.rs:2256-2264`.

Precedente: linha de fracção matemática (Passo 38) e linha geométrica
(Passo 78) já emitem via este variant.

### Implicação na decisão A.2

As três opções do spec partem do pressuposto **falso** que `export.rs` precisa
de função nova. Reformulação:

- A diferença visual entre `underline/strike/overline` é **apenas** a
  coordenada Y da linha (em pt no espaço do Layouter).
- A largura, espessura e ponto de partida da linha são idênticos.
- O Layouter já tem `cursor_x`, `cursor_y`, `font_size_pt` — material
  suficiente para calcular `line_y` em função de uma constante por kind.

**Decisão**: variante prática da opção **(c)** mas sem helper novo em
export.rs — o "helper parametrizado por offset_em" é o próprio
**consumer no Layouter** que calcula `line_y` e emite `FrameItem::Line`. O
export.rs **não é tocado** (preserva hash `bc7b8b95` per spec §5 e §1
não-objectivo).

### Offsets default (constantes Y em em-units, espaço Layouter Y-down)

| Kind | offset_em (Y do Layouter) | Posição visual |
|---|---:|---|
| `underline` | `+0.10` (abaixo do baseline) | logo sob os glifos |
| `strike`    | `-0.25` (acima do baseline) | atravessa o x-height |
| `overline`  | `-0.80` (muito acima do baseline) | acima do cap-height |

Estes são offsets **cosméticos** — vanilla lê do `font.ttf` (UnderlinePosition,
StrikethroughPosition). Justificação per ADR-0054 graded: paridade exacta requer
shaping completo (não materializado); offsets aproximados são suficientes para
PDF visualmente correcto em casos típicos. Override via `offset:` argumento do
utilizador.

### Restrição graded reconhecida

Multi-line bodies: este passo trata **single-line** apenas (cursor_x não
recua entre `before` e `after` se o body fluir para linha nova). Sub-passo
P284.1 candidato para `flush_line`-aware decoration emission. Registado em
§5 do passo como graded.

---

## A.3 — Naming dos variants

**Decisão**: opção **(α) três variants distintos**: `Content::Underline`,
`Content::Strike`, `Content::Overline`.

### Justificações cumulativas

1. **Paridade vanilla**: três `Elem` separados em `text/deco.rs`.
2. **Padrão Layout Fase 2 mais recente**: P156G/H/I escolheu três variants
   distintos para Block/Boxed/Stack apesar de partilharem `body` +
   cosméticos. Consistência arquitectural.
3. **Atributos não são 100% idênticos**: `evade` existe em Underline/Overline
   mas **não** em Strike (vanilla, confirmado em diagnóstico A.1). Mesmo
   que `evade` seja scope-out neste passo, o sinal arquitectural está
   presente — variants tagged forçariam Strike a transportar campo morto.
4. **Sem colisão**: nenhum dos três nomes colide com stdlib Rust nem com
   variants `Content` existentes.

### Padrão cosmético partilhado

Todas as três variants têm a mesma assinatura interna:

```rust
Underline { body: Box<Content>, stroke: Option<Color>, offset: Option<Length>, extent: Option<Length> }
Strike    { body: Box<Content>, stroke: Option<Color>, offset: Option<Length>, extent: Option<Length> }
Overline  { body: Box<Content>, stroke: Option<Color>, offset: Option<Length>, extent: Option<Length> }
```

Justifica-se a duplicação estrutural pelo motivo arquitectural #3 (campos
não-totalmente-idênticos) e pelo precedente P156G/H/I.

---

## §Métricas Layouter disponíveis (mitigação risco §7)

Inventário do estado actual de `Layouter` em `01_core/src/engine/layout/mod.rs`:

| Campo | Tipo | Disponível? | Notas |
|---|---|:---:|---|
| `cursor_x` | `Pt` | ✓ | usado para start.x da linha |
| `cursor_y` | `Pt` | ✓ | baseline Y do Layouter; offset relativo via `font_size_pt` |
| `font_size_pt` | `Pt` | ✓ | escala dos offsets em-units |
| `current_line: Vec<FrameItem>` | — | ✓ | onde fazer push das linhas |

**Conclusão**: zero gap empírico. Sub-passo P284.1 (risco §7) **não é
necessário** — todos os campos requeridos estão disponíveis. Materialização
procede directa.

---

## §Não-tocar (per §5 do passo)

- Hash `export.rs` `bc7b8b95` — **preservado** (A.2 decidiu não tocar).
- `FrameItem` enum — **não estendido** (reutiliza `Line` existente).
- ADR-0054 graded — atributos scope-out documentados acima sem violação.

---

## §Fecho da Fase A

Scope materializável: 3 variants × 4 atributos cada (body required + 3 cosméticos
opcionais). Helper único derivado da existência de `FrameItem::Line` —
zero impacto em export.rs. Métricas Layouter completas — sem necessidade
de P284.1. Procede-se a §3 do passo.
