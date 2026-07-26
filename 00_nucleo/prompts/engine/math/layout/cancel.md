# Prompt L0 — `math/layout/cancel` — `MathCancel`
Hash do Código: b26bb1e2

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
