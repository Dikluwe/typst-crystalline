# Diagnóstico Color vanilla — Passo 257 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0034
canónico + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: discussão pré-P257 + ADR-0029 §"Sobre o
código do Passo 25".
**Imutável após criação** per ADR-0034.

---

## §1 — Estrutura literal vanilla

**Ficheiro**: `lab/typst-original/crates/typst-library/src/visualize/color.rs`
(1996 linhas).

**Enum `Color`** (linha 194):

```rust
#[derive(Copy, Clone)]
pub enum Color {
    Luma(Luma),
    Oklab(Oklab),
    Oklch(Oklch),
    Rgb(Rgb),
    LinearRgb(LinearRgb),
    Cmyk(Cmyk),
    Hsl(Hsl),
    Hsv(Hsv),
}
```

**8 variantes** representando 8 espaços de cor. Cada variante
delega a um tipo dedicado (`Luma`, `Oklab`, etc.) com campos
internos f32 (a, b, c, d).

**Enum `ColorSpace`** (linha 1798):

```rust
pub enum ColorSpace {
    Oklab,
    Oklch,
    Srgb,
    D65Gray,
    LinearRgb,
    Hsl,
    Hsv,
    Cmyk,
}
```

8 valores enumerados (mesma cardinalidade que `Color`).

**Constantes nomeadas Color** (linhas 218-235):
`BLACK`, `GRAY`, `WHITE`, `SILVER`, `NAVY`, `BLUE`, `AQUA`,
`TEAL`, `EASTERN`, `PURPLE`, `FUCHSIA`, `MAROON`, `RED`, `ORANGE`,
`YELLOW`, `OLIVE`, `GREEN`, `LIME` (mais outras pós linha 235).
Cada constante construída como `Self::Luma(Luma::new(...))` ou
`Self::Rgb(Rgb::new(...))` com valores f32 normalizados [0.0, 1.0].

---

## §2 — Funções stdlib vanilla relacionadas

Por convenção vanilla, cada espaço tem função stdlib homónima:

| Função | Espaço | Assinatura aproximada |
|--------|--------|-----------------------|
| `rgb(...)` | sRGB | `rgb(red, green, blue[, alpha])` ou `rgb(hex)` |
| `luma(...)` | D65Gray | `luma(lightness[, alpha])` |
| `cmyk(...)` | CMYK | `cmyk(c, m, y, k)` |
| `oklab(...)` | Oklab | `oklab(L, a, b[, alpha])` |
| `oklch(...)` | Oklch | `oklch(L, c, h[, alpha])` |
| `color.hsl(...)` | HSL | `hsl(h, s, l[, alpha])` |
| `color.hsv(...)` | HSV | `hsv(h, s, v[, alpha])` |
| `color.linear-rgb(...)` | LinearRgb | `linear-rgb(r, g, b[, alpha])` |
| `color.html(...)` | sRGB hex | `html(string)` (alias) |

---

## §3 — Estado cristalino actual

**`01_core/src/entities/layout_types.rs:638-654`** `Color`:

```rust
pub enum Color {
    Rgb  { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8)          -> Self { Self::Rgb { r, g, b } }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8)  -> Self { Self::Rgba { r, g, b, a } }
    pub fn to_rgba_f32(self) -> (f32, f32, f32, f32) { ... }
}
```

**2 variantes** (vs 8 vanilla); representação interna `u8`
(vs `f32` vanilla); zero conversão entre espaços.

**Consumers cristalino**:

- `entities/geometry.rs:7,82,86` — `Stroke.paint: Color`.
- `entities/layout_types.rs:113,218` — `FrameItem::Text.fill`
  + `FrameItem::Shape.fill`/`Shape.stroke`.
- `entities/style.rs:21,129` — `Style::Fill(Color)`.
- `entities/value.rs:341,345` — `Value::Color(Color)`.
- `entities/style_chain.rs:403+` — tests style chain com Color.
- `rules/stdlib/foundations.rs:48,62,65` — `native_rgb` +
  `native_rgba` parsers.
- `rules/stdlib/shapes.rs:14,31-35,67,103,154` —
  `parse_color` named colors (red/green/blue/black/white) +
  default stroke colors.
- `rules/eval/*` — possíveis usos em eval (a auditar pós-P257.C
  se necessário).

**Cobertura cristalina actual**: 2/8 espaços = 25% paridade
estrutural.

---

## §4 — Análise de paridade

| Espaço vanilla | Representação vanilla | Conversão para sRGB | Materializar P257? |
|----------------|----------------------|---------------------|---------------------|
| sRGB | `Rgb { r,g,b,a: f32 }` | identidade | **✓ refactor preserva u8 paridade** |
| Linear RGB | `LinearRgb { r,g,b,a: f32 }` | gamma 2.2 inversa | **✓ materializar** |
| Oklab | `Oklab { l,a,b,alpha: f32 }` | matriz LMS + linear RGB + gamma | **✓ materializar** |
| Oklch | `Oklch { l,c,h,alpha: f32 }` | Oklab + polar→cartesiano | **✓ materializar** |
| HSL | `Hsl { h:f32(deg),s,l,a: f32 }` | algoritmo HSL→RGB | **✓ materializar** |
| HSV | `Hsv { h,s,v,a: f32 }` | algoritmo HSV→RGB | **✓ materializar** |
| CMYK | `Cmyk { c,m,y,k: f32 }` | `(1-c)(1-k)` ... | **✓ materializar (com scope-out PDF native)** |
| Luma | `Luma { l,a: f32 }` | `r=g=b=l` | **✓ materializar (separar de Rgb cinza)** |

**Decisão preliminar**: **materializar 8 variantes** com paridade
estrutural literal vanilla; conversões para sRGB no PDF exporter
para todos os 7 espaços não-sRGB. CMYK PDF native (`/DeviceCMYK`)
**scope-out formal** documentado.

---

## §5 — Componentes auxiliares vanilla

Sub-componentes vanilla internos a `Color` (linhas 1093+):

- `Luma`, `Oklab`, `Oklch`, `Rgb`, `LinearRgb`, `Cmyk`, `Hsl`,
  `Hsv` — tipos newtype f32 por espaço.
- `WeightedColor` — usado em gradientes; **fora de scope P257**
  (gradient é tipo separado per ADR-0029 §enumeração).
- `RatioComponent` — usado internamente em mixagem; idem
  scope-out.

Per ADR-0029 §exclusões, sub-componentes internos a `Color`
não precisam de prompt L0 dedicado — são detalhes implementação.

---

## §6 — Conversões e operadores

Operadores vanilla disponíveis (per `impl Color` linha 1093+):

- `to_rgb()`, `to_oklab()`, etc. — conversão entre espaços.
- `lighten(ratio)`, `darken(ratio)` — manipulação luminance.
- `saturate(ratio)`, `desaturate(ratio)` — manipulação saturation.
- `mix(other, weight, space)` — interpolação entre cores.
- `negate(space)` — inverter cor num espaço.
- `components()` — extrair componentes f32.
- `space()` — retorna `ColorSpace` da variante.

**Decisão P257**: materializar **subset minimal** (`to_srgb()`
por espaço; `space()` para introspecção); operadores
`lighten`/`darken`/`mix`/etc. **scope-out formal**
(materializam-se quando uso real surgir; ADR dedicada por refino
futuro).

---

## §7 — Impacto exportador PDF

**PDF nativos**: `/DeviceRGB`, `/DeviceCMYK`, `/DeviceGray`.

**Exportador cristalino actual** (`03_infra/src/export.rs:857-1553`):

Sítios `to_rgba_f32()` (4 caminhos cumulativos):
- Linha 857 — Helvetica fill text.
- Linha 863 — Helvetica stroke text.
- Linha 1121 — CIDFont fill text.
- Linha 1125 — CIDFont stroke text.
- Linha 1367 — Shape fill.
- Linha 1371 — Shape stroke.
- Linha 1549 — variant Shape stroke.
- Linha 1553 — variant Shape stroke.

**Output PDF actual**: apenas `/DeviceRGB` (via `to_rgba_f32` →
operadores `rg`/`RG` para fill/stroke).

**Plano pós-P257**: todos os espaços não-sRGB convertem para
sRGB via `Color::to_srgb()` antes de emitir bytes PDF. **CMYK
nativo `/DeviceCMYK`** scope-out formal documentado em ADR-0067
(refino futuro quando export print for prioritário).

---

## §8 — Plano materialização (Fase B)

**Decisão por espaço**:

| Espaço | Decisão | Justificação | ADR-required? |
|--------|---------|--------------|---------------|
| sRGB (Rgb) | **materializar** | refactor; preservar u8 paridade observable | — |
| Luma (D65Gray) | **materializar** | espaço minimal cinza; distinto de Rgb generic | — |
| Linear RGB | **materializar** | base para conversões Oklab; útil para sci/graphics | — |
| Oklab | **materializar** | espaço moderno; útil interpolação visual perceptual | — |
| Oklch | **materializar** | derivado Oklab (polar) | — |
| HSL | **materializar** | uso comum web | — |
| HSV | **materializar** | uso comum design | — |
| CMYK | **materializar (struct only)** | print-only; PDF native scope-out documentado | **ADR-0067 §"Scope-out PDF native CMYK"** |

**Sub-decisões scope-out** (ADR-0067):

1. **PDF native `/DeviceCMYK`** — scope-out: exporter actual
   converte CMYK para sRGB via `to_srgb()` antes de emitir.
   Refino futuro quando export print for prioritário.
2. **Operadores `lighten`/`darken`/`mix`/`saturate`/`negate`** —
   scope-out: materializam-se quando uso real surgir (sem
   feature em consumers actuais).
3. **Constantes nomeadas extras (NAVY/PURPLE/etc.)** —
   scope-out: cristalino actual usa Color literal em
   `stdlib/shapes.rs::parse_color`; expansão é refino
   incremental sem ADR (ADR-0080 EM VIGOR cobre refactors
   aditivos).
4. **`ColorSpace` enum runtime** — scope-out: cristalino
   actual não precisa de runtime introspection de espaço
   (consumer único é PDF exporter via match exhaustive).
   Refino futuro quando uso surgir.

---

## §9 — Localização do tipo

**Ficheiro novo**: `01_core/src/entities/color.rs` (~250-350
LoC esperadas para 8 variantes + conversões para sRGB +
helpers).

**Re-export em `01_core/src/entities/mod.rs`**: `pub mod color;`
+ `pub use color::Color`.

**Remoção em `01_core/src/entities/layout_types.rs`**: `pub
enum Color { Rgb, Rgba }` removido (Color migrado a ficheiro
próprio).

**Compatibilidade temporária**: `Color::rgb(r,g,b)` preservado
como construtor sRGB (paridade observable cristalino actual);
nova `Color::rgba(r,g,b,a)` preserva comportamento.

---

## §10 — Decisão arquitectural

**Decisões fixas pós-Fase A**:

1. **Enum tagged 8 variantes** (paridade estrutural vanilla
   literal) — não scope-out de espaços por minimização.
2. **Representação interna f32** (não u8) — paridade vanilla
   + conversões correctas + precisão sub-byte.
3. **Construtor `Color::rgb(r:u8,g:u8,b:u8)` preservado** —
   wrapper que constrói `Color::Srgb` com f32 normalizado
   `r as f32 / 255.0`. Paridade observable cristalino actual
   estricta.
4. **PDF exporter converte para sRGB via `Color::to_srgb()`**
   uniformemente; CMYK native `/DeviceCMYK` scope-out
   documentado.
5. **Operadores cor (lighten/darken/mix)** scope-out
   documentado.
6. **`PartialEq` exacto preservado** per ADR-0028 regra
   herdada — sem tolerância em produção (`f32` exacto via
   bitwise se necessário).
7. **ADR-0067 nova** (ou próximo número disponível) formaliza
   subset materializado + scope-outs.
8. **Stdlib funcs novas**: `native_oklab`, `native_oklch`,
   `native_linear_rgb`, `native_cmyk`, `native_hsl`,
   `native_hsv`, `native_luma` (separado de `native_rgb`).
   `native_rgb`/`native_rgba` preservados (paridade
   construtor existente).

**ADR-0067 necessária**: sim — formaliza scope-outs (PDF
native CMYK + operadores + ColorSpace runtime + constantes
nomeadas extras).

**Magnitude estimada P257.C**: **M (~3-5h)** — refactor
struct + cascade ~15-20 consumers + conversões + PDF exporter
+ stdlib novas funcs + tests +20-40.
