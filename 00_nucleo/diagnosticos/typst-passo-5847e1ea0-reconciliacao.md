# Reconciliação Retroativa — `5847e1ea0` · `axis_height` em delimitadores + `STRETCHY_BASES`

**Tipo:** Relatório retroativo (continuação directa do Achado A de P911/P912, sem relatório próprio)
**Commit:** `5847e1ea030b555ef663edd164813d721eb05ebb`
**Data do commit:** 2026-07-25 23:56:26 -0300
**Detectado em:** P916 (Parte C) · 2026-07-26

---

## Contexto

P912 corrigiu a causa raiz do Achado A de P911: `covering()` passou a preferir fontes
com tabela OpenType `MATH` para glifos matemáticos. Com essa correcção, as variantes
de delimitadores passaram a ser seleccionadas — mas o alinhamento vertical estava
errado: os delimitadores apareciam com `ascent=total_height, descent=0`, ou seja,
todo o delimitador acima do baseline, não centrado no eixo matemático.

Este commit corrigiu esse segundo problema, que só se tornou visível após P912.

## O que foi alterado

### `01_core/src/engine/math/layout/assembly.rs`

**Antes:** `ascent=total_height, descent=0.0` — delimitador inteiramente acima do baseline.

**Depois:** centralização no eixo matemático:
```rust
let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
let half_h = total_height / 2.0;
let ascent = axis_pt + half_h;
let descent = (half_h - axis_pt).max(0.0);
let shift_y = axis_pt - half_h;
```
Cada peça de assemblagem deslocada por `shift_y` em Y.

### `01_core/src/engine/math/layout/stretchy.rs`

Mesmo cálculo de `axis_pt`/`half_h`/`shift_y` aplicado tanto ao caminho Text
(mapeamento Unicode via `layout_text_node`) como ao caminho Glyph (sem mapeamento).

### `03_infra/src/font_metrics.rs` — `STRETCHY_BASES`

**Antes:** `['(', ')', '[', ']', '{', '}', '|', '√']` (8 chars)

**Depois:** adiciona `‖`, `∥`, `⌊`, `⌋`, `⌈`, `⌉`, `⟨`, `⟩`, `〈`, `〉`, `/`, `\`,
`↑`, `↓`, `↕`, `⇑`, `⇓`, `⇕`.

**Motivo:** `STRETCHY_BASES` determina para que chars se constrói o mapa reverso
`glyph_id → char`. Sem inclusão, `glyph_to_char()` retorna `None`, forçando o caminho
Glyph sem ToUnicode em vez do caminho Text com mapeamento correcto.

### L0 actualizado

`00_nucleo/prompts/engine/math/layout/stretchy.md` — hash actualizado para `ac81e392`
(novo `§P914 — Centralização no Eixo Matemático`). Incluído no commit `dcff44bd2`.

## Por que ficou fora de P912

P912 foi commitado antes deste alinhamento vertical ter sido identificado como
problema separado. Só após P912 se tornou claro que a selecção correcta da fonte
não era suficiente — o posicionamento vertical estava errado. A correcção foi feita
na mesma sessão, mas como commit separado, sem relatório porque foi tratada como
"fixup" de P912.

## Validação

- `cargo test --workspace → 0 falhas` (commit `dcff44bd2`, 4781+740+41=5562)
- `crystalline-lint . → 0 V5` (hash stretchy.md sincronizado)

## Achado aberto de P916 (relacionado)

As medições de mutool trace de P916 (Parte A) mostram que os delimitadores `(` e `)` em
`(1/2)`, `(1/2/3/4)`, `(1/2/3/4/5/6/7/8)` e `mat(2×2)` ainda não crescem (glyph
fixo `1` e `2` vs. vanilla que usa glyph `1`, `4`, `8`, `12`). Este é um achado
novo — ver `typst-passo-916-relatorio.md`.
