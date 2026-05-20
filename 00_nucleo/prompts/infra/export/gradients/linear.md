# Prompt L0 — `infra/export/gradients/linear` — Linear gradient coords + stops
Hash do Código: 5bcf4266

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/linear.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0087 (gradient linear PDF Shading Pattern)

---

## Contexto

Helpers para emit de Linear gradients como PDF Shading Type 2:
- `compute_axial_coords` — endpoints axiais a partir de angle + bbox.
- `multispace_sample_stops` — amostra N stops intermédios em sRGB.

## Restrições estruturais

- L3 puro. Lê `Linear` de `typst_core::entities::gradient`.
- `pub(crate)` — chamado por `super::cmyk` (que reusa pipeline RGB antes da conversão) e `super::super::builder`.
- Sem I/O. Sem stateful caches (cálculo puro).

## Interface

```rust
pub(crate) fn compute_axial_coords(angle_rad: f64, x0: f64, y0: f64, w: f64, h: f64)
    -> (f64, f64, f64, f64);
pub(crate) fn multispace_sample_stops(gradient: &Linear, n: usize) -> Vec<(f32, f32, f32)>;
```

## Invariantes

- `compute_axial_coords` produz endpoints simétricos em torno do centro do bbox.
- `multispace_sample_stops` amostra em espaço perceptual (Oklab) per `gradient.space`, depois converte para sRGB no output.
- Stops normalizados em [0.0, 1.0] no PDF (operadores RGB).

## Critérios de verificação

- `compute_axial_coords(0.0, 0, 0, 100, 50)` → linha horizontal através do centro.
- `compute_axial_coords(PI/2, 0, 0, 100, 50)` → linha vertical.
- Stops para 2-cor → 2 entradas; para N-cor + n=16 → 16 entradas amostradas.

Tests `p263_*`, `p268_multispace_sample_stops_*` em `super::super::tests`.
