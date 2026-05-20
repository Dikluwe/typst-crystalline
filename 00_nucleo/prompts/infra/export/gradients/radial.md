# Prompt L0 — `infra/export/gradients/radial` — Radial gradient coords + stops
Hash do Código: 6e889301

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/radial.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0088 (gradient radial PDF Shading Type 3)

---

## Contexto

Helpers para emit de Radial gradients como PDF Shading Type 3:
- `compute_radial_coords` — 6 valores `Coords [c0x c0y r0 c1x c1y r1]`.
- `multispace_sample_stops_radial` — amostra N stops intermédios em sRGB.

Suporta focal points (P269) via `Radial.focal_center` + `Radial.focal_radius`.

## Restrições estruturais

- L3 puro. Lê `Radial`, `Axes`, `Ratio` de `typst_core::entities`.
- `pub(crate)` — chamado por `super::cmyk` (reuso pipeline RGB).
- Sem I/O.

## Interface

```rust
pub(crate) fn compute_radial_coords(radial: &Radial, x0: f64, y0: f64, w: f64, h: f64)
    -> (f64, f64, f64, f64, f64, f64);
pub(crate) fn multispace_sample_stops_radial(gradient: &Radial, n: usize) -> Vec<(f32, f32, f32)>;
```

## Invariantes

- `compute_radial_coords` resolve `Axes<Ratio>` relativo a `(w, h)`.
- Focal point opcional via `Radial.focal_center`; default = center (paridade simples).
- Stops em [0.0, 1.0] no PDF.

## Critérios de verificação

- `compute_radial_coords` para focal=center → c0=c1, r0=0, r1=raio.
- Tests `p265_*`, `p269_*` em `super::super::tests`.
