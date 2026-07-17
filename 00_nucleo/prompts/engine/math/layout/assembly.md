# Prompt L0 — `math/layout/assembly` — assembly de delimitadores grandes
Hash do Código: 0f8a62a3

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/assembly.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

Assembly por partes para delimitadores grandes. Consome `GlyphAssembly` via
`self.metrics.vertical_glyph_assembly(c)` (P255 §2 item 2; `assembly.rs:14,20`).

**Critério**: `assembly.rs` constrói assembly de partes para delimitadores.
