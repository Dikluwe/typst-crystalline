# Prompt L0 — geometry
Hash do Código: 37897031

## Módulo
`01_core/src/entities/geometry.rs`

## Propósito
Tipos geométricos primitivos para o sistema de layout vectorial (Passo 76).
Puramente declarativos — sem I/O, sem métricas de fonte.
Depende de `Color` de `layout_types` — sem dependências externas adicionais.

## Tipos

### `Stroke`
Contorno de uma forma: cor e espessura em pontos.
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
}
```
Usa `Color` de `layout_types` (já existente em L1) — sem tipo `RgbaColor` separado.

### `ShapeKind`
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind {
    Rect,
    Ellipse,
    Line { dx: f64, dy: f64 },
}
```
`Line::dx`/`dy`: deslocamentos no sistema de layout (Y positivo = baixo).
Bounding box de `Line` usa `abs()` dos deltas.

`Ellipse`: scaffolding presente; exportador PDF emite rectângulo placeholder
com comentário `TODO` apontando DEBT-31.

## Exposição em `entities/mod.rs`
```rust
pub mod geometry;
```
Tipos reutilizados nos módulos que os precisam via caminhos explícitos.

## Critérios de verificação
- Zero dependências externas além de `Color` de `layout_types`.
- `ShapeKind` e `Stroke` derivam `Debug`, `Clone`, `PartialEq`.
