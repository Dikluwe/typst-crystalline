# Prompt L0 — `rules/math/layout` — ÍNDICE (fatiado em P314)
Hash do Código: (a calcular no checkpoint A.4 — P314)

**Este prompt foi fatiado em P314** (ADR-0104, atomicidade para agentes). Cada
`.rs` aponta agora para o seu prompt fino; este ficheiro é só o índice.

## Prompts finos que o substituem

| Prompt fino | Conteúdo | `.rs` que aponta |
|---|---|---|
| `math/layout/_comum.md` | `MathLayouter` struct, `MathBox`, interface, despacho, baseline x-height, MathPrimes, handler `MathStyled`, critérios gerais | `mod.rs`, `tests.rs` |
| `math/layout/attach.md` | `MathAttach` (sub/superscripts; `MathGlyphKern` 4 quadrantes) | `attach.rs` |
| `math/layout/root.md` | `MathRoot` (sqrt + n-th roots; radical consts) | `root.rs` |
| `math/layout/frac.md` | `MathFrac` (fracções; fraction consts) | `frac.rs` |
| `math/layout/matrix.md` | `MathMatrix` | `matrix.rs` |
| `math/layout/cases.md` | `MathCases` | `cases.rs` |
| `math/layout/stretchy.md` | operadores extensíveis (`GlyphVariants`) | `stretchy.rs` |
| `math/layout/assembly.md` | assembly de delimitadores grandes (`GlyphAssembly`) | `assembly.rs` |
| `math/layout/delimited.md` | `MathDelimited` (par fixo) | `delimited.rs` |
