# Passo 1028 — `BoxedElem.width` não chega à largura desenhada

**Data**: 2026-08-13
**Estado**: **fechado**. Fases A, B, C e D feitas. Gate ADR-0127 categoria 2 aplicado e
aprovado via confirmação do dono; L0 actualizado antes do código.

---

## Resumo

O Passo 1026 encontrou que `#box(width: 40pt, …)` fora de math ignorava a largura pedida:
a caixa saía com a largura do conteúdo mais inset (~6,96 pt), independentemente do valor de
`width`. O problema reproduzia-se em texto corrido, logo não era específico de math nem de
`layout_external`.

A causa estava em `01_core/src/compiler/layout/boxed.rs`: `width` era usado para clampar
`regions.current.width` durante o layout do body (P243), mas a largura exterior desenhada
(`FrameItem::Shape.width`) era conduzida pelo avanço real do cursor
(`outer_w = cursor_x - start_x`). O campo `width` nunca era convertido em avanço horizontal
nem em largura de Shape.

A correcção força `outer_w` e o cursor final para
`outset_left + inset_left + width + inset_right + outset_right` quando `width` é `Some`.
Quando `width` é `None`, mantém-se o comportamento actual (largura natural do conteúdo).

---

## Proveniência

`HEAD = a5b23004c`, árvore de trabalho limpa (dois ficheiros de materialização não
acompanhados: `typst-passo-1027.md` e `typst-passo-1028.md`).

Binário cristalino reconstruído no HEAD actual (`cargo build --release -p typst-wiring`);
referência vanilla `/usr/local/bin/typst` e `lab/typst-original/target/release/typst`,
baseline ratificado `a51e02804`.

Documentos e scripts de medição em `temp/p1028/`:
- Vários `.typ` de verificação de `box(width:)`, `box(height:)` e conteúdo largo.

---

## Fase A — causa confirmada

`boxed.rs:68-75` aplica `width` durante o body layout via save/restore de
`regions.current.width`. No entanto, após o body:

- `cursor_x` avança apenas pelo conteúdo real + `inset_right` + `outset_right`.
- `outer_w = cursor_x - start_x` reflete esse avanço.
- `FrameItem::Shape` usa esse `outer_w`.

`width` nunca participa no cálculo final. A `height` funciona porque `outer_h` é calculado
a partir de `height` quando fornecido (`boxed.rs:187-189`); a largura não tinha tratamento
paralelo.

A comparação com o vanilla (`lab/typst-original/crates/typst-layout/src/inline/box.rs`)
confirmou o mecanismo esperado: o vanilla layouta o body num pod de tamanho fixo e depois
força o frame a esse tamanho (`frame.set_size(pod.expand.select(pod.size, frame.size()))`).

---

## Fase B — alcance

Verificou-se rapidamente `#block(width: 40pt, stroke: 1pt)[a]`:
- vanilla: 40,00 × 7,24 pt
- cristalino: 40,50 × 7,24 pt

A diferença de ~0,5 pt está na margem do stroke; o campo `width` é aplicado. O problema é
**isolado a `BoxedElem` / `boxed.rs`**.

Comportamento quando o conteúdo excede `width`:

| documento | vanilla | cristalino |
|---|---|---|
| `#box(width: 40pt, stroke: 1pt)[abcdefghijklmnopqrstuvwxyz]` | page 40,00 pt; texto truncado/quebrado (`abcdefgh`, xMax 40,58 pt) | page 134,02 pt; caixa acompanha o texto |

No vanilla, `width` define o tamanho da caixa e o conteúdo transborda/trunca. No cristalino,
a caixa perdia o tamanho fixo. A correcção de P1028 restaura o tamanho fixo da caixa;
o truncamento/quebra de palavras longas sem espaços é um problema de wrap de texto
separado — com espaços, o cristalino faz wrap corretamente dentro dos 40 pt.

---

## Fase C — gate ADR-0127 e L0

Mudança de output visual em qualquer documento com `box(width:)` → gate ADR-0127 categoria 2
(comportamento por defeito).

O Prompt L0 `00_nucleo/prompts/compiler/layout.md` §`BoxedElem.width` foi actualizado com as
medições da Fase A/B, a confirmação de que o problema é isolado a `BoxedElem`, e a decisão:
quando `width` é `Some`, forçar `outer_w` e o cursor final para
`outset_left + inset_left + w + inset_right + outset_right`. Hash resselado antes do código.

---

## Fase D — implementação e validação

### Código alterado

- `01_core/src/compiler/layout/boxed.rs` — cálculo de `outer_w` e posicionamento final de
  `cursor_x` passam a respeitar `width` quando fornecido.

### Testes adicionados

Em `01_core/src/compiler/layout/tests.rs`:

- `p1028_boxed_width_forca_largura_exterior`
- `p1028_boxed_width_none_mantem_largura_natural`

### Resultados

- `cargo test --workspace`: **5848 passed, 0 failed**.
- `crystalline-lint .`: **0 violações** (apenas 3 warnings V7 pré-existentes de prompts
  órfãos).
- Validação visual / medição:
  - `#box(width: 40pt, stroke: 1pt)[a]` → cristalino 40,50 × 7,24 pt vs vanilla
    40,00 × 7,24 pt. Largura corrigida (era 6,96 pt).
  - `#box(width: 40pt, stroke: 1pt)[a b c … z]` → cristalino faz wrap dentro de 40 pt,
    paridade com vanilla.
  - `#box(height: 40pt, stroke: 1pt)[a]` (sem width) → comportamento inalterado; a
    diferença de altura (~40 pt vs ~7 pt) é o débito pré-existente de `outer_h` já
    registado no L0.

### Commits

- `ab2fc2d8c` — docs(L0): decisão de correcção de `BoxedElem.width`.
- `a5b23004c` — fix(layout): P1028 — `BoxedElem.width` força largura exterior da caixa.

### Ressalva

Palavras longas sem espaços (ex.: `abcdefghijklmnopqrstuvwxyz`) continuam a transbordar a
caixa no cristalino, enquanto o vanilla trunca/quebra. Este comportamento não é alterado
pela correcção de P1028: trata-se de uma questão de quebra de palavras no layout inline,
separada do cálculo da largura exterior da caixa. Com espaços entre as palavras, o wrap
respeita `width` correctamente.
