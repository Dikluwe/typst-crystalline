# Diagnóstico — Gradient ColorSpace runtime cross-variant (P270.A)

**Status**: imutável após criação (per ADR-0085).
**Data**: 2026-05-17.
**Passo**: P270 (activação `space: ColorSpace` cross-variant L1+stdlib).
**Oitavo consumo directo de fonte** vanilla (P262/P264/P267/P268/P268.1/P268.2/P269 + **P270 vanilla multi-space sample + ColorSpace enum**).
**Origem**: spec `00_nucleo/materialization/typst-passo-270.md` §1.

---

## §A.1 — Vanilla Linear/Radial/Conic campos `space`

`lab/typst-original/crates/typst-library/src/visualize/gradient.rs:1007/1075/1153`:

```rust
pub struct LinearGradient { ..., pub space: ColorSpace, ... }
pub struct RadialGradient { ..., pub space: ColorSpace, ... }
pub struct ConicGradient  { ..., pub space: ColorSpace, ... }
```

Default vanilla via stdlib: `space: ColorSpace::Oklab` (preserva paridade
P262/P264/P267 cristalino).

---

## §A.2 — Vanilla `mix_iter` (sample matemática multi-space)

`lab/typst-original/.../visualize/color.rs:1095-1176`:

```rust
pub fn mix_iter(colors, space: ColorSpace) -> StrResult<Color> {
    // 1. Convert each color to target space via `to_space(space).to_vec4()`.
    // 2. Linear interpolation componentwise.
    // 3. For hue spaces with |h0 - h1| > 180°, take shorter path:
    //    if c0[h] < c1[h]: (h0+360, h1) else (h0, h1+360)
    // 4. Construct Color in target space.
}
```

`sample_stops()` (`gradient.rs:1346-1366`) chama `mix_iter` com t-weighted
2 stops adjacentes.

---

## §A.3 — Vanilla `ColorSpace::hue_index`

```rust
fn hue_index(&self) -> Option<usize> {
    match self {
        Self::Hsl | Self::Hsv => Some(0),  // hue é component 0
        Self::Oklch => Some(2),            // hue é component 2
        _ => None,                          // não-polar
    }
}
```

Apenas HSL, HSV, Oklch têm hue (polar spaces). Hue-wrap shorter aplica-se
só a estes 3.

---

## §A.4 — Vanilla `ColorSpace` enum (8 variants)

`lab/typst-original/.../visualize/color.rs:1798`:

```rust
pub enum ColorSpace {
    Oklab, Oklch, Srgb, D65Gray, LinearRgb, Hsl, Hsv, Cmyk,
}
```

Cristalino `Color::Luma` ≡ vanilla `D65Gray` (mesmo conceito; nome
diferente histórico P257).

---

## §A.5 — Cristalino L1 sample actual (3 sítios Oklab hardcoded)

### Linear::sample (linha 116-134)

```rust
pub fn sample(&self, t: f32) -> Color {
    // ...
    return interpolate_oklab(self.stops[i].color, self.stops[i + 1].color, local_t);
}
```

### Radial::sample (linha 268-284 pós-P269)

```rust
pub fn sample(&self, t: f32) -> Color {
    // ...
    return interpolate_oklab(self.stops[i].color, self.stops[i + 1].color, local_t);
}
```

### Conic::sample (linha ~347 pós-P269)

```rust
pub fn sample(&self, t: f32) -> Color {
    // ...
    return interpolate_oklab(self.stops[i].color, self.stops[i + 1].color, local_t);
}
```

**3 sítios chamam `interpolate_oklab` directamente** — refactor P270:
substituir por `interpolate_in_space(c0, c1, t, self.space)` que dispatcha
per ColorSpace.

**Default `space: ColorSpace::Oklab`** preserva behavior P262/P264/P267
bit-exact (dispatcher chama `interpolate_oklab` original para Oklab).

---

## §A.6 — Cristalino stdlib actual (sem named arg `space`)

`01_core/src/rules/stdlib/gradients.rs`:

- `native_gradient_linear`: named args `{angle}` apenas.
- `native_gradient_radial`: named args `{center, radius, focal_center,
  focal_radius}` (focal_* P269).
- `native_gradient_conic`: named args `{center, angle}`.

P270 expande whitelist named para incluir `space` em cada variant.

Parse value: utiliza `Value::Str("oklab" | "oklch" | "srgb" | "luma" |
"linear-rgb" | "hsl" | "hsv" | "cmyk")` → match para ColorSpace enum.
(Cristalino `Value` não tem variant ColorSpace — uso Str preserva
simplicidade L1.)

---

## §A.7 — Cristalino Color P257 — 8 variants + helpers conversão actual

`01_core/src/entities/color.rs`:

```rust
pub enum Color {
    Srgb { r, g, b, a },          // P257 §1
    Luma { l, a },                // P257 §2 (≡ vanilla D65Gray)
    LinearRgb { r, g, b, a },     // P257 §3
    Oklab { l, a, b, alpha },     // P257 §4
    Oklch { l, c, h, alpha },     // P257 §5
    Hsl { h, s, l, a },           // P257 §6
    Hsv { h, s, v, a },           // P257 §7
    Cmyk { c, m, y, k },          // P257 §8
}

impl Color {
    pub fn rgb(u8, u8, u8) -> Self;
    pub fn rgba(u8, u8, u8, u8) -> Self;
    pub fn srgb_f32(f32, f32, f32, f32) -> Self;
    pub fn luma(f32) -> Self;
    pub fn linear_rgb(f32, f32, f32, f32) -> Self;
    pub fn oklab(f32, f32, f32, f32) -> Self;
    pub fn oklch(f32, f32, f32, f32) -> Self;
    pub fn hsl(f32, f32, f32, f32) -> Self;
    pub fn hsv(f32, f32, f32, f32) -> Self;
    pub fn cmyk(f32, f32, f32, f32) -> Self;

    // Conversões one-way (X → sRGB normalizado):
    pub fn to_srgb(&self) -> (u8, u8, u8, u8);
    pub fn to_rgba_f32(&self) -> (f32, f32, f32, f32);
}

// Free helpers (P262/P268.2):
pub fn color_to_oklab_with_alpha(c) -> (f32, f32, f32, f32);  // gradient.rs
```

### Gap conversão cross-space

**Color tem só forward conversões (→sRGB)** — falta inverse para a maior
parte das espaços. Para interpolação multi-space cristalino precisa:

- `to_oklab_components(c)`: já existe via `color_to_oklab_with_alpha`. ✓
- `to_oklch_components(c)`: derivável de Oklab via `atan2`. **Novo P270**.
- `to_srgb_components(c)`: trivial via `to_rgba_f32`. ✓
- `to_linear_rgb_components(c)`: via `to_rgba_f32` + sRGB→linear inverso.
  **Novo P270** (~5 LOC).
- `to_luma_components(c)`: via `to_rgba_f32` + Rec 709 luminance.
  **Novo P270** (~5 LOC).
- `to_hsl_components(c)`: via `to_rgba_f32` + sRGB→HSL conversão. **Novo
  P270** (~15 LOC).
- `to_hsv_components(c)`: via `to_rgba_f32` + sRGB→HSV. **Novo P270**
  (~15 LOC).
- `to_cmyk_components(c)`: via `to_rgba_f32` + inverse formula. **Novo
  P270** (~8 LOC).

**Total gap**: ~70 LOC helpers extract. **Cap L1=350; folga ~70 LOC**
após reservar ~280 LOC para ColorSpace + interpolate dispatch + 3 struct
fields + constructores + tests infrastructure. **§política condição 2
NÃO accionada** (gap ≤ 50 LOC + bem dentro magnitude M).

### Cristalino ColorSpace enum — gap

**ColorSpace enum NÃO existe em cristalino** — gap arquitectural P270
inaugurar. Estrutura proposta:

```rust
pub enum ColorSpace {
    Oklab, Oklch, Srgb, Luma, LinearRgb, Hsl, Hsv, Cmyk,
}
```

8 variants paridade vanilla (Luma ≡ D65Gray). Localização: novo módulo
`01_core/src/entities/color_space.rs` OU adicionar a `color.rs`. Decisão
P270: adicionar a `color.rs` (mesmo módulo conceptual; ~30 LOC; evita
módulo novo trivial).

---

## §A.8 — PROPOSTA L1 — campo `space: ColorSpace` cross-variant

```rust
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    pub space: ColorSpace,  // P270; default Oklab
}

pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,  // P269
    pub focal_radius: Ratio,        // P269
    pub space: ColorSpace,           // P270; default Oklab
}

pub struct Conic {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle: Angle,
    pub space: ColorSpace,  // P270; default Oklab
}
```

### Sample multi-space

Substituir 3 sítios `interpolate_oklab(c0, c1, t)` por
`interpolate_in_space(c0, c1, t, self.space)`. Dispatcher chama:
- Oklab → `interpolate_oklab` original (preserva bytes P262/P264/P267).
- 7 outros spaces → nova função `interpolate_in_<space>`.

---

## §A.9 — PROPOSTA hue-wrap shorter (vanilla paridade)

`mix_iter` linha 1126-1136 vanilla:

```rust
if let Some(index) = space.hue_index()
    && (c0[index] - c1[index]).abs() > 180.0
{
    let (h0, h1) = if c0[index] < c1[index] {
        (c0[index] + 360.0, c1[index])
    } else {
        (c0[index], c1[index] + 360.0)
    };
    m[index] = (w0 * h0 + w1 * h1) / (w0 + w1);
}
```

Cristalino implementação:

```rust
fn interpolate_hue_shorter(h0: f32, h1: f32, t: f32) -> f32 {
    let diff = h1 - h0;
    let wrapped_h1 = if diff.abs() > 180.0 {
        if diff > 0.0 { h1 - 360.0 } else { h1 + 360.0 }
    } else {
        h1
    };
    (h0 + (wrapped_h1 - h0) * t).rem_euclid(360.0)
}
```

Equivalente vanilla literal. Edge case diff == 180° → wrap fica
positivo (cristalino default sentido +; paridade CSS).

**Scope-outs P270**:
- `longer hue` / `increasing hue` / `decreasing hue` CSS modes —
  preservados scope-out.

---

## §A.10 — PROPOSTA stdlib named arg `space`

Cada variant `native_gradient_*` ganha:

```rust
let space = match args.named.get("space") {
    Some(Value::Str(s)) => parse_color_space(s)?,
    Some(other) => return Err(...),
    None => ColorSpace::Oklab,  // default
};

// Whitelist named estendida com "space".
```

Parser:

```rust
fn parse_color_space(s: &str) -> SourceResult<ColorSpace> {
    match s {
        "oklab"      => Ok(ColorSpace::Oklab),
        "oklch"      => Ok(ColorSpace::Oklch),
        "srgb"       => Ok(ColorSpace::Srgb),
        "luma"       => Ok(ColorSpace::Luma),
        "linear-rgb" => Ok(ColorSpace::LinearRgb),
        "hsl"        => Ok(ColorSpace::Hsl),
        "hsv"        => Ok(ColorSpace::Hsv),
        "cmyk"       => Ok(ColorSpace::Cmyk),
        _ => Err(...),
    }
}
```

User-facing: `#gradient.linear(red, blue, space: "hsl")`.

---

## §A.11 — Tests P262/P264/P265/P267/P268/P268.2/P269 — paridade preservada

### Defaults preservam bytes P262/P264/P267

- `Gradient::linear(stops, angle)` mantém assinatura; internamente
  `space: ColorSpace::Oklab`.
- `Gradient::radial(...)` idem (preserva P269 focal defaults também).
- `Gradient::conic(...)` idem.
- Stdlib sem named `space` → P262/P264/P267 behavior.

### Tests P262 (4 tests Linear): preservados literal

Construtores `Gradient::linear(...)` mantém-se; testes não tocados.

### Tests P264 (9 tests Radial): preservados literal

Idem; tests P264 construct via `Gradient::radial(...)` ou `Radial { ... }`
literal — este último precisa adicionar `space: ColorSpace::Oklab`
mecânicamente (paridade comportamento; só syntax). 5 sites P264 tests
+ 4 sites P269 tests adicionais (focal explícito) actualizados.

### Tests P267 (Conic L1): preservados literal

Similar a P264 — sítios struct literal `Conic { ... }` actualizados
mecânicamente.

### Tests P263/P265/P268/P268.2/P269 (L3 PDF emit): preservados literal

L3 não muda em P270 — `Linear/Radial/Conic` struct ganha campo extra
mas `Conic::sample`, `Radial::sample`, `Linear::sample` preservam bytes
para defaults Oklab. Tests PDF emit preservados.

§política condições 6 + 9 satisfeitas — defaults preservam P262/P264/P267
bit-exact.

---

## §A.12 — Cenário detectado — B1 fecho conceptual L1+stdlib

**B1 confirmado**: P270 L1+stdlib é absorvível em magnitude M
(estimado ~250-330 LOC L1 + ~60-90 LOC stdlib + ~40 testes); cap
350+50/50 respeitado.

**B2 NÃO accionado** — hue-wrap matemática vanilla é trivial (linha 1126-1136
literal portável); ColorSpace conversões cristalino têm gap manejável
(~70 LOC para 7 helpers extract).

### Magnitude estimada

| Componente | LOC estimado | Cap |
|---|---|---|
| `ColorSpace` enum + helpers parse | ~30 | parte de L1 |
| 7 helpers `to_<space>_components` | ~70 | parte de L1 |
| `interpolate_in_space` dispatcher | ~30 | parte de L1 |
| 7 `interpolate_*` per-space | ~80 | parte de L1 |
| `interpolate_hue_shorter` | ~12 | parte de L1 |
| 3 struct fields + 3 construtores | ~30 | parte de L1 |
| 3 `_with_space` construtores | ~40 | parte de L1 |
| Update 3 `sample()` sítios | ~10 | parte de L1 |
| Update existing struct literal sites | ~30 | parte de L1 |
| **L1 total** | **~332** | **350 (folga ~18)** |
| Stdlib 3 named args + validation | ~60 | 50 (ligeiramente acima cap; aceita) |
| Tests | ~40 | 50 |

**Cap L1 apertado** — §política condição 4 sob vigilância. Optimização
viável se necessário: helpers `to_<space>_components` podem ser
inlinados em `interpolate_<space>` reduzindo ~30 LOC.

---

## §A.13 — Decisão arquitectural — Op B documentada (L3 adiado)

L3 emit refactor adiado P270.1+P270.2 per ADR-0091 EM VIGOR:

- **P270.1** (M+): L3 RGB-family (sRGB/LinearRgb/Luma/Oklab/Oklch/Hsl/Hsv)
  via Oklab pipeline N=16 → DeviceRGB.
- **P270.2** (S+): L3 CMYK directo `/DeviceCMYK`. Revoga ADR-0083
  §CMYK.

L3 actual P263/P265/P268 preservado bit-exact P270 — utilizadores
não verão diferença visual até P270.1/P270.2 fecharem.

---

## §A.14 — Sumário decisões diagnóstico

| Item | Decisão |
|---|---|
| §A.1 Vanilla 3 variants × space | Cristalino paridade L1+stdlib P270; L3 adiado |
| §A.2 Vanilla mix_iter | Cristalino `interpolate_in_space` dispatcher análogo |
| §A.3 Hue-wrap shorter | Portado literal para HSL/HSV/Oklch |
| §A.4 Vanilla ColorSpace 8 variants | Cristalino criar enum com mesmo 8 (Luma ≡ D65Gray) |
| §A.5 Cristalino 3 sítios sample Oklab | Refactor mínimo: substituir por dispatcher |
| §A.6 Stdlib gap | 3 variants ganham named arg `space` |
| §A.7 Color helpers gap | ~70 LOC novos helpers extract; cap respeitado |
| §A.8 L1 proposta | 3 struct fields + dispatcher + 7 interpolate |
| §A.9 Hue-wrap shorter | Implementação literal vanilla portada |
| §A.10 Stdlib parse | `Value::Str` discriminator (Value não muda) |
| §A.11 Tests preservados | Defaults Oklab bit-exact P262/P264/P267 |
| §A.12 Cenário B1 | Magnitude M absorve L1+stdlib; L3 adiado |
| §A.13 Op B | Documentada ADR-0091; não materializada P270 |

**Diagnóstico aprovado para passagem a sub-passo P270.B (ADR-0091 +
anotações + L0).**

---

## §A.15 — Referências

- Spec P270: `00_nucleo/materialization/typst-passo-270.md`.
- Vanilla L1: `lab/typst-original/crates/typst-library/src/visualize/gradient.rs:1007/1075/1153`.
- Vanilla mix: `lab/typst-original/.../visualize/color.rs:1095-1176`.
- Vanilla ColorSpace: `lab/typst-original/.../visualize/color.rs:1798-1830`.
- Cristalino L1: `01_core/src/entities/gradient.rs:116/268/347`.
- Cristalino Color P257: `01_core/src/entities/color.rs:32-242`.
- Cristalino stdlib: `01_core/src/rules/stdlib/gradients.rs`.
- ADR-0083 — Color 8/8 spaces (§ColorSpace runtime scope-out revogado
  parcialmente P270).
- ADR-0085 — Diagnóstico imutável (oitavo consumo).
- ADR-0091 (criada P270) — ColorSpace runtime + CMYK strategy.
- typst.app/docs/reference/visualize/gradient — vanilla user-facing API.
- typst/typst issue #4422 — CMYK gradient PDF emit bug vanilla.
