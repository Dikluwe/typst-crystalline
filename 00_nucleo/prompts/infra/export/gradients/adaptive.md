# Prompt L0 — `infra/export/gradients/adaptive` — Adaptive N multispace
Hash do Código: fab5f533

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/adaptive.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0091 §"Anotação cumulativa P274" Opção 1B

---

## Contexto

Refino qualitativo do número N de stops amostrados (P274):
- N=16 fixo (P270.1) substituído por adaptive baseado em ΔE Oklab.
- Aplicável a Linear+Radial RGB-family + perceptual; Conic preserved P272.

Helpers:
- `perceptual_distance_in_space` — ΔE entre duas cores num space dado.
- `adaptive_n_for_stops` — N adaptativo (16/32/64) baseado em max pair ΔE.

## Fórmula (P274)

| max ΔE | N |
|---:|---:|
| < 0.05 | 16 |
| < 0.30 | 32 |
| ≥ 0.30 | 64 (cap) |

## Restrições estruturais

- L3 puro. Lê `GradientStop`, `Color`, `ColorSpace`.
- `pub(crate)` — chamado por `super::super::builder::emit_gradient_objects`.
- Sem I/O. Cálculo puro.

## Interface

```rust
pub(crate) fn perceptual_distance_in_space(c1: Color, c2: Color, space: ColorSpace) -> f64;
pub(crate) fn adaptive_n_for_stops(stops: &[GradientStop], space: ColorSpace) -> usize;
```

## Invariantes

- N retornado é sempre potência de 2 entre 16 e 64.
- `perceptual_distance_in_space` em Oklab native (sem coerção a sRGB ΔE).

## Critérios de verificação

Tests `p274_*` (14 testes) em `super::super::tests`.
