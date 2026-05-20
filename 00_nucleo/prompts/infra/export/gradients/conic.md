# Prompt L0 — `infra/export/gradients/conic` — Conic Type 6 Coons Patch Mesh
Hash do Código: ac19f896

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/conic.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0089 (gradient conic only), ADR-0090 (conic strategy Type 4 vs Type 1), ADR-0092 (conic Coons patches RGB+CMYK)

---

## Contexto

Emit de Conic gradients como PDF Shading Type 6 (Coons Patch Mesh).
Estratégia única decidida em P272: Type 6 patches construídos a partir
de N stops + bezier control points para arcs circulares.

Helpers:
- `multispace_sample_stops_conic` — amostra N stops em sRGB ao longo do ângulo conic.
- `bezier_control_points_for_arc` — control points para arco circular do conic patch.
- `compute_coons_patches_n_stops` — N base por número de stops.
- `compute_coons_patches_n_stops_extended` — N estendido com adaptive subdivision.
- `emit_conic_coons_stream_rgb` — stream binário PDF Type 6 RGB.
- `emit_conic_coons_stream_cmyk` — stream binário PDF Type 6 CMYK (via `super::cmyk::rgb_to_cmyk`).

## Restrições estruturais

- L3 puro. Lê `Conic`, `Color` de `typst_core::entities`.
- `pub(crate)` — chamado por `super::super::builder::emit_gradient_objects`.
- Cross-module dependency: usa `super::cmyk::rgb_to_cmyk` para variant CMYK.
- Stream binário emitido como `Vec<u8>` directamente (não String — bytes opcionais incluem `\xFF`).

## Interface

```rust
pub(crate) fn multispace_sample_stops_conic(g: &Conic, n: usize) -> Vec<(f32, f32, f32)>;
pub(crate) fn bezier_control_points_for_arc(...) -> (f64, f64, f64, f64);
pub(crate) fn compute_coons_patches_n_stops(conic: &Conic) -> usize;
pub(crate) fn compute_coons_patches_n_stops_extended(conic: &Conic) -> usize;
pub(crate) fn emit_conic_coons_stream_rgb(conic: &Conic) -> Vec<u8>;
pub(crate) fn emit_conic_coons_stream_cmyk(conic: &Conic) -> Vec<u8>;
```

## Invariantes

- Type 6 ShadingType produz patches contínuos sem hardcoded angle limits.
- Bezier control points respeitam `cos(θ/4) * tan(θ/4) * 4/3` formula.
- CMYK variant produz output diferente de RGB (verificado por testes).

## Excede limite 800 LOC

~300 LOC neste ficheiro. Sub-divisão futura (emit_streams.rs +
helpers.rs) possível mas baixo ROI dada coesão temática.

## Critérios de verificação

Tests `p272_*` (15 testes) + `p270_*` CMYK em `super::super::tests`.
