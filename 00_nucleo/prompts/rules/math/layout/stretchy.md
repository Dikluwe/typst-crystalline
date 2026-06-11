# Prompt L0 — `math/layout/stretchy` — operadores extensíveis
Hash do Código: a0c870f3

**Camada**: L1 · **Alvo**: `01_core/src/rules/math/layout/stretchy.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

Operadores extensíveis. Consome `GlyphVariants` via
`self.metrics.vertical_glyph_variants(c)` com `.select(min_advance)`
(P255 §2 item 2; `stretchy.rs:22`).

**Critério**: `stretchy.rs` selecciona variant via `select(min_advance)`.
