# Prompt L0 — geometry
Hash do Código: a1be19a3

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

## `ShapeKind::RoundedRect` — Passo 242 (M9d/M7+5)

```rust
ShapeKind::RoundedRect {
    radii: crate::entities::corners::Corners<Length>,
}
```

Rectângulo com cantos arredondados (paridade vanilla
`layout/shape.rs::RoundedRect`). Co-existe com `Rect` / `Ellipse`
/ `Line` / `Path`. Consumer principal: `Content::Block.radius` +
`Content::Boxed.radius` (refino P231 → P242 `Option<Length>` →
`Corners<Length>`).

**Degeneração**: quando todos os 4 radii são zero, semantic é
equivalente a `Rect` mas a distinção estrutural é preservada (não
normaliza para `Rect` — preserva distinguibilidade). PDF exporter
P242 emite Bezier 4 corners path mesmo quando radii zero (output
correcto via geometria degenerada).

**Bezier kappa**: `0.552_284_749_831` (paridade `ShapeKind::Ellipse`
mesmo ficheiro). Quarto de círculo aproximado com 2 control points
por canto. Output PDF: `m` + `l` × 4 + `c` × N + `h` (N = #cantos
não-zero).

**Layout integration**: `Content::Block.clip == true` + `radius
!= zero` → Layouter emite `FrameItem::Group` com `clip_mask:
Some(ShapeKind::RoundedRect { radii: radius })`. PDF exporter
desenha clip path via `emit_rounded_rect_ops`.

**Bounding box**: radii não afecta bounding box (paridade Rect;
clamp interno em `emit_rounded_rect_ops` para `radii ≤ min(w,h)/2`
evita overflow geométrico). `measure_content` arms tratam
RoundedRect identicamente a Rect.
