# Prompt L0 — `entities/paint`
Hash do Código: 6a9cd487

## Módulo
`01_core/src/entities/paint.rs`

## Camada
L1

## Propósito

Wrapper enum sobre fontes de cor para preenchimentos e contornos.
Substitui `Color` directo em `Stroke.paint` (P261) para abrir
caminho a `Gradient`/`Tiling` (P262+ per ADR-0086).

## Estrutura

```rust
use crate::entities::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Paint {
    Solid(Color),
    // Gradient(Gradient),  // P262 — comentário reserva
    // Tiling(Tiling),      // futuro — comentário reserva
}

impl Paint {
    pub fn solid(c: Color) -> Self {
        Paint::Solid(c)
    }

    pub fn to_color(&self) -> Color {
        match self {
            Paint::Solid(c) => *c,
        }
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self {
        Paint::Solid(c)
    }
}
```

## Critérios de verificação

- `Paint::solid(c).to_color() == c` para qualquer `Color`.
- `Paint::from(c) == Paint::Solid(c)`.
- `PartialEq` + `Copy` derivados (porque `Color` é Copy).
- Zero dependências externas.

## Sobre paridade vanilla

Vanilla `Paint` tem 3 variants (`Solid` + `Gradient` + `Tiling`).
P261 materializa **Solid only**; `Gradient`/`Tiling` são
**comentários reserva** no enum, não unit placeholders. Expansão
consumer-driven em P262+ per ADR-0086 §"Critério revisão".

**Métodos vanilla não materializados** (scope-out ADR-0086):
- `unwrap_solid()` panicking — substituído por `to_color()`
  sem panic (Solid only garantido).
- `relative()` — específico Gradient/Tiling.
- `as_decoration()` — específico Gradient texto.

## Sobre ADR-0039 (TextStyle SR — preservado)

`TextStyle.fill: Option<Color>` permanece **literal preservado**.
P261 **não migra** TextStyle.fill para `Option<Paint>`. Apenas
`Stroke.paint` adapta. Refino futuro pode migrar TextStyle.fill
se Gradient para texto for prioritário (P262+ Gradient text como
extensão natural).

## Sobre Style::Fill (preservado)

`Style::Fill(Color)` (variant StyleChain) **inalterado**. P261
não toca este caminho.

## Sobre stdlib `native_rgb`/etc. (preservado)

Stdlib funcs cor continuam retornar `Value::Color`. P261 **não
introduz** `Value::Paint`. Paint::Solid é wrapper interno
cristalino, transparente para user-facing.

## Exposição em `entities/mod.rs`

```rust
pub mod paint;
pub use paint::Paint;
```

## Cross-references

- `entities/color.md` — `Color` tipo base (8 variants P257).
- `entities/geometry.md` — `Stroke.paint: Paint` consumer P261.
- ADR-0086 — Paint wrapper Solid only (IMPLEMENTADO P261).
- ADR-0083 — Color paridade vanilla (análogo estrutural).
- ADR-0039 — TextStyle SR (preservado).
- Vanilla `lab/typst-original/.../visualize/paint.rs` (3 variants).
