# Prompt L0 — Color (espaços de cor vanilla paridade)
Hash do Código: 2701d418

## Módulo
`01_core/src/entities/color.rs`

## Camada
L1 (puro; sem I/O; sem estado global; Copy + Clone).

## Propósito

Representar cores em múltiplos espaços de cor com paridade
estrutural vanilla. Substitui o `Color { Rgb, Rgba }`
simplificado de `entities/layout_types.rs` (P25) per ADR-0029
§"Diagnosticar primeiro" + §"Simplificações aceites apenas
com ADR explícita".

P257 materializa 8 variantes correspondendo aos 8 espaços
vanilla. Scope-outs formalizados em **ADR-0083 PROPOSTO**:
PDF native CMYK + operadores cor + ColorSpace runtime
introspection + constantes nomeadas extras.

## Tipo exportado

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    /// sRGB color space (paridade vanilla `Rgb`).
    Srgb { r: f32, g: f32, b: f32, a: f32 },
    /// D65 grayscale (paridade vanilla `Luma`).
    Luma { l: f32, a: f32 },
    /// Linear RGB color space.
    LinearRgb { r: f32, g: f32, b: f32, a: f32 },
    /// Oklab perceptual color space.
    Oklab { l: f32, a: f32, b: f32, alpha: f32 },
    /// Oklch (Oklab polar coordinates).
    Oklch { l: f32, c: f32, h: f32, alpha: f32 },
    /// HSL color space.
    Hsl { h: f32, s: f32, l: f32, a: f32 },
    /// HSV color space.
    Hsv { h: f32, s: f32, v: f32, a: f32 },
    /// CMYK color space (print).
    Cmyk { c: f32, m: f32, y: f32, k: f32 },
}
```

8 variantes; representação interna `f32` (paridade vanilla).
`f32` exacto via bitwise equality em derived `PartialEq`
(per ADR-0028 regra herdada "sem tolerância em produção").

## Métodos públicos

### Construtores

- `Color::rgb(r: u8, g: u8, b: u8) -> Self` — sRGB com alpha=1.0
  (paridade cristalino existente; f32 normalizado `r as f32 / 255.0`).
- `Color::rgba(r: u8, g: u8, b: u8, a: u8) -> Self` — sRGB com
  alpha explícito (paridade existente).
- `Color::srgb_f32(r: f32, g: f32, b: f32, a: f32) -> Self` —
  sRGB direct f32 (sem normalização).
- `Color::luma(l: f32) -> Self` — luma com alpha=1.0.
- `Color::linear_rgb(r: f32, g: f32, b: f32, a: f32) -> Self`.
- `Color::oklab(l: f32, a: f32, b: f32, alpha: f32) -> Self`.
- `Color::oklch(l: f32, c: f32, h: f32, alpha: f32) -> Self`.
- `Color::hsl(h: f32, s: f32, l: f32, a: f32) -> Self` —
  h em graus.
- `Color::hsv(h: f32, s: f32, v: f32, a: f32) -> Self` —
  h em graus.
- `Color::cmyk(c: f32, m: f32, y: f32, k: f32) -> Self` —
  componentes [0.0, 1.0].

### Conversões

- `to_srgb(&self) -> (u8, u8, u8, u8)` — conversão para sRGB
  byte (consumer PDF exporter; 4 caminhos `to_rgba_f32`).
  Algoritmos:
  - `Srgb` → identidade (u8 normalizado).
  - `Luma { l }` → `(l*255, l*255, l*255, 255)`.
  - `LinearRgb` → gamma 2.2 inversa.
  - `Oklab` → matriz LMS + linear RGB + gamma.
  - `Oklch` → Oklab + polar→cartesiano.
  - `Hsl`/`Hsv` → algoritmo standard.
  - `Cmyk` → `(1-c)(1-k), (1-m)(1-k), (1-y)(1-k)` (CMY→RGB).
- `to_rgba_f32(&self) -> (f32, f32, f32, f32)` — conversão para
  sRGB normalizado [0.0, 1.0] (preservado para compatibilidade
  hot path PDF exporter cristalino existente).

## Comportamento

- **Pureza L1**: sem I/O; sem estado global; `Copy + Clone +
  PartialEq` derivados.
- **Paridade observable estricta**: `Color::rgb(255, 0, 0)`
  produz mesmos bytes PDF antes e depois de P257.
- **PDF exporter intocado estructuralmente**: 4 caminhos
  `to_rgba_f32` preservados; novos espaços convertem para
  sRGB transparentemente.
- **`PartialEq` exacto** per ADR-0028 regra herdada: f32
  bitwise equality (sem tolerância).

## Critérios de verificação

Por cada espaço materializado, ≥2 tests:

- **sRGB**:
  - `Color::rgb(255, 0, 0)` → `Srgb { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }`.
  - `Color::rgb(255, 0, 0).to_srgb() == (255, 0, 0, 255)`.
- **Luma**:
  - `Color::luma(0.5)` → `Luma { l: 0.5, a: 1.0 }`.
  - `Color::luma(0.5).to_srgb()` → `(127, 127, 127, 255)`.
- **LinearRgb**:
  - Construtor preserva valores f32.
  - Conversão `LinearRgb { 0.5, 0.5, 0.5, 1.0 }` → sRGB ≠
    `(127, 127, 127)` (gamma inversa aplicada).
- **Oklab**:
  - `Color::oklab(1.0, 0.0, 0.0, 1.0).to_srgb()` ≈
    `(255, 255, 255, 255)` (L=1 → branco).
  - `Color::oklab(0.0, 0.0, 0.0, 1.0).to_srgb()` ≈
    `(0, 0, 0, 255)` (L=0 → preto).
- **Oklch**:
  - Construtor preserva (L, c, h).
  - `Color::oklch(0.5, 0.0, 0.0, 1.0)` → sRGB gris (c=0 →
    sem chroma).
- **Hsl/Hsv**:
  - `Color::hsl(0.0, 0.0, 0.5, 1.0).to_srgb()` ≈ `(127, 127, 127, 255)`.
- **Cmyk**:
  - `Color::cmyk(0.0, 0.0, 0.0, 0.0).to_srgb()` →
    `(255, 255, 255, 255)` (CMYK zero → branco).
  - `Color::cmyk(0.0, 0.0, 0.0, 1.0).to_srgb()` →
    `(0, 0, 0, 255)` (K=1 → preto).

## Localização e re-export

- Ficheiro: `01_core/src/entities/color.rs` (~250-350 LoC).
- Re-export: `01_core/src/entities/mod.rs` — `pub mod color;`
  + `pub use color::Color;` (paridade pattern outros entities).
- Remoção: `01_core/src/entities/layout_types.rs:638-654` —
  `pub enum Color { Rgb, Rgba }` removido (migração).

## Operadores de cor (P476 — ADR-0083 §"Operadores cor" parcialmente revogado)

4 métodos adicionados em P476:

```rust
impl Color {
    /// Aumenta luminância por `amount` [0.0, 1.0] via Oklch.
    pub fn lighten(self, amount: f32) -> Self;

    /// Diminui luminância por `amount` [0.0, 1.0] via Oklch.
    pub fn darken(self, amount: f32) -> Self;

    /// Interpolação linear entre `self` e `other` em Oklab.
    /// `weight` [0.0, 1.0]: 0.0 = self; 1.0 = other.
    pub fn mix(self, other: Self, weight: f32) -> Self;

    /// Negação: complementar em sRGB (1-r, 1-g, 1-b). Alpha preservado.
    pub fn negate(self) -> Self;
}
```

**Helpers privados** (duplicados de `gradient.rs` — circular dep impede import):
- `srgb_to_linear_p476`, `linear_rgb_to_oklab_p476`, `to_oklab_p476`, `to_oklch_p476`.

**Critérios P476**:
- `negate(red)` → `(0.0, 1.0, 1.0, 1.0)` (ciano).
- `lighten(c, 0.0)` → Oklch idêntico ao original.
- `lighten(c, 1.0)` → l clamped a 1.0.
- `darken(c, 1.0)` → l clamped a 0.0.
- `mix(red, blue, 0.0)` → oklab idêntico a red.
- `mix(red, blue, 1.0)` → oklab idêntico a blue.
- `mix(red, blue, 0.5)` → l médio entre red e blue.

**P477 — `saturate` e `desaturate`** — ADR-0083 §"Operadores cor" TOTALMENTE FECHADO (6/6):

```rust
impl Color {
    /// Aumenta saturação por `amount` (chroma Oklch). Clamp mínimo 0.0.
    pub fn saturate(self, amount: f32) -> Self;

    /// Diminui saturação por `amount` (chroma Oklch). Clamp mínimo 0.0.
    pub fn desaturate(self, amount: f32) -> Self;
}
```

Critérios P477:
- `saturate(0.0)` → chroma inalterada.
- `saturate(0.1)` → chroma aumenta.
- `desaturate(1.0)` → chroma clamped a 0.0 (cinzento).
- l e h preservados em ambos.

**ADR-0083 §"Operadores cor": TOTALMENTE FECHADO pós-P477** (6/6).

## Constantes de cor nomeadas em `parse_color` (P477)

`parse_color` em `rules/stdlib/shapes.rs` alargada de 5 para 18 cores:

**Originais (P-base):** `red`, `green`, `blue`, `black`, `white`.

**Novas P477 (13 + 2 aliases):**
`yellow`, `cyan`, `magenta`, `orange`, `purple`, `gray`/`grey` (alias), `silver`,
`maroon`, `navy`, `olive`, `teal`, `lime`, `aqua` (alias de cyan).

Aliases: `gray` == `grey` == `rgb(128,128,128)`; `aqua` == `cyan` == `rgb(0,255,255)`.

ADR-0083 §"Constantes nomeadas": parcialmente revogado (CSS basic colors cobertas).

## Sobre paridade vanilla (ADR-0083)

Referência: `lab/typst-original/crates/typst-library/src/visualize/color.rs`
linha 194 (enum `Color` com 8 variantes) + `ColorSpace` linha
1798 (8 valores enumerados).

**Scope-outs P257 documentados em ADR-0083 PROPOSTO**:

1. PDF native `/DeviceCMYK` — CMYK converte para sRGB no
   exporter; refino futuro **P-Color-CMYK-PDF**.
2. Operadores cor — **P477 TOTALMENTE FECHADO** (6/6: lighten/darken/mix/negate P476 + saturate/desaturate P477).
3. `ColorSpace` enum runtime — não materializado; match
   exhaustive em consumers.
4. Constantes nomeadas extras — refino incremental via
   ADR-0080 (sem ADR dedicada).
