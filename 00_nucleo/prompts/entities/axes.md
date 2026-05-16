# Prompt L0 — `entities/axes`
Hash do Código: c942a18a

## Módulo
`01_core/src/entities/axes.rs`

## Camada
L1

## Propósito

Tipo genérico `Axes<T>` representando par 2D (x, y).
Reutilizável para coordenadas / Ratios / Lengths / etc. Tipo
minimal (~25 LoC) criado em P264 para `Radial.center: Axes<Ratio>`
(paridade vanilla `Axes<T>`).

## Estrutura

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axes<T> {
    pub x: T,
    pub y: T,
}

impl<T> Axes<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Eq> Eq for Axes<T> {}
```

## Critérios de verificação

- `Axes::new(x, y).x == x` e `Axes::new(x, y).y == y`.
- `Copy` derivado se `T: Copy`.
- `PartialEq` derivado.

## Sobre paridade vanilla

Vanilla `Axes<T>` em `lab/typst-original/crates/typst-library/
src/layout/axes.rs` tem campos `x`/`y` + métodos auxiliares
(map, with, etc.). P264 materializa apenas struct base + `new()`
constructor — paridade minimal per ADR-0080 §"L0 minimal para
refactors aditivos".

## Exposição em `entities/mod.rs`

```rust
pub mod axes;
pub use axes::Axes;
```

## Cross-references

- `entities/gradient.md` — `Radial.center: Axes<Ratio>` consumer
  P264.
- ADR-0088 — Gradient Radial-only (IMPLEMENTADO P264).
- ADR-0080 — L0 minimal para refactors aditivos.
