# Prompt L0 — `infra/export/gradients/cmyk` — CMYK conversão + stops
Hash do Código: 6d95916a

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/cmyk.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0091 (gradient space runtime + CMYK strategy), ADR-0092 (conic Coons CMYK)

---

## Contexto

Cluster CMYK para gradients quando `gradient.space == Cmyk`:
- `rgb_to_cmyk` — conversão simples sRGB → CMYK (sem ICC profile).
- `multispace_sample_stops_linear_cmyk` — paralelo CMYK de `super::linear::multispace_sample_stops`.
- `multispace_sample_stops_radial_cmyk` — paralelo CMYK de `super::radial::multispace_sample_stops_radial`.

Convenção P270.2 ("Cenário B"): Linear + Radial em CMYK; Conic
preservado em RGB Coons Patch (P272).

## Restrições estruturais

- L3 puro. Lê `Linear`, `Radial`, `Color` de `typst_core::entities`.
- `pub(crate)` — `rgb_to_cmyk` chamado por `super::conic` para emit Coons CMYK.
- Conversão sRGB→CMYK simples: `K = 1 - max(R,G,B)`, depois `(R,G,B,K)` resolvido.

## Interface

```rust
pub(crate) fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> (f32, f32, f32, f32);
pub(crate) fn multispace_sample_stops_linear_cmyk(g: &Linear, n: usize) -> Vec<(f32, f32, f32, f32)>;
pub(crate) fn multispace_sample_stops_radial_cmyk(g: &Radial, n: usize) -> Vec<(f32, f32, f32, f32)>;
```

## Invariantes

- `rgb_to_cmyk(0,0,0)` → `(0,0,0,1)` (preto puro).
- `rgb_to_cmyk(1,1,1)` → `(0,0,0,0)` (branco — sem tinta).
- Stops em [0.0, 1.0] para cada canal CMYK.

## Critérios de verificação

Tests `p270_*` (69 testes) em `super::super::tests`.
