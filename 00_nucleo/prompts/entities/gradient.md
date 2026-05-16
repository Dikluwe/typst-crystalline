# Prompt L0 — `entities/gradient`
Hash do Código: a237b047

## Módulo
`01_core/src/entities/gradient.rs`

## Camada
L1

## Propósito

Tipo `Gradient` enum + struct `Linear` + sub-componente
`GradientStop`. Permite preenchimentos/contornos com gradient
linear. Activa `Paint::Gradient(Gradient)` variant per ADR-0086
§"Critério revisão" cumprido por P262 (ADR-0087).

## Estrutura

```rust
use std::sync::Arc;
use crate::entities::color::Color;
use crate::entities::layout_types::{Angle, Ratio};

/// Sub-componente per ADR-0029 §exclusões.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    pub color:  Color,
    pub offset: Option<Ratio>,
}

impl GradientStop {
    pub fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset: Some(offset) }
    }
    pub fn unspaced(color: Color) -> Self {
        Self { color, offset: None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
}

impl Linear {
    /// Offsets efectivos com auto-spacing aplicado (paridade
    /// vanilla — stops com offset=None recebem distribuição
    /// uniforme entre stops com offset explícito).
    pub fn effective_offsets(&self) -> Vec<f32>;

    /// Amostra a cor interpolada em parâmetro t ∈ [0, 1].
    /// Interpolação em Oklab (paridade vanilla default).
    pub fn sample(&self, t: f32) -> Color;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    // Radial(Arc<Radial>),  // P-Gradient-Radial — comentário reserva
    // Conic(Arc<Conic>),    // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(stops: impl Into<Arc<[GradientStop]>>, angle: Angle) -> Self;

    /// Amostra primeiro stop (fallback para `Paint::to_color()`).
    pub fn first_stop_color(&self) -> Color;
}

// Activa Paint::Gradient — ADR-0086 §"Critério revisão".
impl From<Gradient> for Paint;
```

## Critérios de verificação

- `GradientStop::new(c, r).offset == Some(r)`.
- `GradientStop::unspaced(c).offset == None`.
- `Gradient::linear(stops, angle)` constrói `Gradient::Linear(Arc<Linear>)`.
- `Linear::effective_offsets()` auto-spacing correcto.
- `Linear::sample(0.0) == primeiro stop color` (após Oklab roundtrip).
- `Linear::sample(1.0) == último stop color`.
- `Gradient::first_stop_color()` retorna color do primeiro stop.
- Clone Arc é O(1).
- `PartialEq` derivado.

## Sobre paridade vanilla

Vanilla `Gradient` tem 3 variants (Linear/Radial/Conic).
P262 materializa **Linear only**; Radial/Conic são **comentários
reserva** no enum, não unit placeholders. Expansão consumer-
driven em P-Gradient-Radial/Conic per ADR-0087 §"Critério
revisão".

**ColorSpace fixo Oklab** (paridade vanilla default per user
decision P262 Q2). Interpolação em Oklab via `Color::oklab` +
Oklab linear interpolation; conversão final para PDF via
`Color::to_rgba_f32()` (linear path Oklab → linear RGB → sRGB).

**Scope-outs vanilla** (per ADR-0087):
- `space: ColorSpace` — Oklab fixo.
- `relative: Smart<RelativeTo>` — bbox-relative assumido.
- `anti_alias: bool` — assume true.
- `Gradient::sample()` user-facing — futuro.
- `Gradient::stops()` getter — futuro.

## Sobre ADR-0086 (Paint wrapper) — §"Critério revisão" cumprido

`Paint::Gradient(Gradient)` variant activada (era comentário
reserva pré-P262). `Copy` removido de Paint (Gradient não é
Copy via Arc). `Paint::to_color()` fallback retorna primeiro
stop color.

## Sobre ADR-0039 (TextStyle SR — preservado)

`TextStyle.fill: Option<Color>` permanece **literal preservado**.
P262 **não migra** TextStyle.fill para Option<Paint>. Decisão
preservada de P261.

## Exposição em `entities/mod.rs`

```rust
pub mod gradient;
pub use gradient::{Gradient, GradientStop, Linear};
```

## Cross-references

- `entities/color.md` — `Color` tipo base (8 variants P257) com
  `Color::oklab` + `to_rgba_f32()` consumidos.
- `entities/paint.md` — Paint wrapper P261; `Paint::Gradient`
  variant activada.
- ADR-0087 — Gradient Linear-only (IMPLEMENTADO P262).
- ADR-0086 — Paint wrapper Solid only (P261; §"Critério revisão"
  cumprido).
- ADR-0083 — Color paridade vanilla (template N=2 do pattern).
- ADR-0039 — TextStyle SR (preservado).
- Vanilla `lab/typst-original/.../visualize/gradient.rs` (1366
  linhas; 3 variants).

---

## Anotação cumulativa P264 — Radial variant materializada

**Data**: 2026-05-15.

Subset Radial materializado per ADR-0088 (Opção α — ADR nova
dedicada paridade pattern N=2 P261/P262).

### Tipos adicionados

```rust
use crate::entities::axes::Axes;

pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
}

impl Radial {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear
    pub fn sample(&self, t: f32) -> Color;        // paridade Linear (Oklab)
}
```

### Enum Gradient expandido

```rust
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),  // P264 — descomentado
    // Conic(Arc<Conic>),   // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(stops, angle) -> Self;
    pub fn radial(stops, center, radius) -> Self;  // P264
    pub fn first_stop_color(&self) -> Color;  // pattern-match expandido
}
```

### Scope-outs P264 (per ADR-0088)

- `focal_center` (default = center; consumer raro).
- `focal_radius` (default 0%; consumer raro).
- `space` (Oklab fixo — paridade P262).
- `relative` (bbox-local — paridade P262).
- `anti_alias` (true assumed — paridade P262).
- **PDF emit Radial fallback Solid** até **P265 dedicado**
  (`/ShadingType 3`).

### Cross-references P264

- `entities/axes.md` — `Axes<T>` minimal criado P264.
- ADR-0088 — Gradient Radial-only (IMPLEMENTADO P264).
- ADR-0087 §"Critério revisão" cumprido parcialmente
  (Conic continua scope-out).
- ADR-0086 — Paint::Gradient automaticamente absorve Radial
  (zero cascade refactor).
- P262 — Linear precedente; helpers Oklab reutilizados literal.
- P265 (futuro) — PDF emit Radial dedicado (replica P263
  template).
