# Prompt L0 — `math/layout/attach` — `MathAttach`
Hash do Código: 4c91feee

**Camada**: L1 · **Alvo**: `01_core/src/rules/math/layout/attach.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado (MathLayouter, despacho): ver `math/layout/_comum.md`.

---

`MathAttach` — subscripts/superscripts/primes merged via eval. Consome
`MathGlyphKern` em todos os **4 quadrantes** (top-left, bottom-left, top-right,
bottom-right) via `self.metrics.math_kern(c)` (P255 §2 item 1; geometria
correcta sem `.abs()`, kern negativo permitido — `attach.rs:49-208`).

Recebe os `MathPrimes` (resolvidos em eval; ver `_comum.md`) pelo arm
superscript regular — não há arm dedicado `MathPrimes`.

**Critério**: `attach.rs` consome `math_kern` em todos os 4 quadrantes.
