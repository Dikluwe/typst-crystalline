# Prompt L0 — `math/layout/cancel` — `MathCancel`
Hash do Código: 8201214e

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/cancel.rs`
**Origem**: fatiado de `math/layout/mod.rs` em **P909**, completando o padrão de fatiamento
iniciado em P314 (ADR-0104) para `frac`/`root`/`stretchy`/`assembly`/`matrix`/`cases`/
`delimited`. `layout_cancel` foi adicionado em **P296**, depois de P314, e nunca tinha sido
movido. Núcleo partilhado: ver `math/layout/_comum.md`.

---

`MathCancel` — layout do `body` seguido de uma linha diagonal sobre a bbox. Heurística minimal
per ADR-0054 graded:
- Diagonal default (canto inferior-esquerdo `(0, h)` → canto superior-direito `(width, 0)`;
  "rising", ângulo padrão do vanilla).
- Sem `inverted`/`cross`/`angle`/`stroke` cosméticos — scope-out para passo futuro dedicado a
  `MathCancel`.

`ascent`/`descent`/`width` do `MathBox` resultante são os do `body` — a linha não afecta as
métricas de caixa (é um item adicional sobreposto, não expande a caixa).

**Critério**: `MathCancel { body }` → `MathBox` com os items do `body` mais um `FrameItem::Line`
diagonal de `(0, h)` a `(width, 0)`, sem alterar `width`/`ascent`/`descent` do `body`.

## P986 — a linha usa a convenção baseline-relativa (quarto caso da família)

**Medição** (achado §7.3 da auditoria 2026-08-06; doc canónico secção 10,
`cancel(a + b)`, 600dpi): no vanilla a diagonal cruza POR DENTRO do texto
(início 0.68pt abaixo do topo, fim 10.3pt abaixo — efeito de risco); no
cristalino ficava INTEIRA abaixo do texto (8.8pt→17.3pt — efeito de
sublinhado), deslocamento de ~8pt ≈ `ascent+descent` do corpo.

**Causa**: a linha era emitida em coords "topo do MathBox" (`(0, h)` →
`(width, 0)`), mas os items de um `MathBox` vivem na convenção
**baseline-relativa** (`y=0` = baseline própria, negativo para cima —
ADR-0123, mesma família de erro de P901/P906/P919/P972 — quarto caso). Em
coords baseline-relativas, `(0, h)`→`(width, 0)` lê-se "da baseline até
`h` abaixo dela" — exactamente o sublinhado observado.

**Vanilla** (`typst-layout/src/math/cancel.rs:43-45,108-115`): a linha é
construída com o ponto médio no CENTRO do frame do corpo e estende-se pela
diagonal do frame — canto inferior-esquerdo → canto superior-direito da
**tinta do corpo**. Em coords baseline-relativas do cristalino:
`start = (0, body.descent)`, `end = (body.width, −body.ascent)`.

**Correcção**: `layout_cancel` emite a `FrameItem::Line` com esses
endpoints; o texto do critério original acima ("de `(0, h)` a `(width, 0)`")
fica **revogado** — era a descrição da convenção errada. `width`/`ascent`/
`descent` do `MathBox` continuam os do `body` (a linha não expande a caixa —
igual ao vanilla, que sobrepõe o frame da linha).

**Critério**: para um corpo com `ascent`/`descent` conhecidos, a linha vai de
`(0, descent)` a `(width, −ascent)`; no documento canónico, a diagonal cruza
o texto (risco), com início/fim ≈ vanilla (0.68/10.3pt abaixo do topo).
