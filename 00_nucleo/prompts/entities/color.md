# Prompt L0 — Color (espaços de cor vanilla paridade)
Hash do Código: 2abf697a

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
  - `Hsl`/`Hsv` → hexcone standard, o mesmo modelo do `palette` usado pelo vanilla.

> **Fonte de paridade dos espaços de cor (P1031)** — vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/visualize/color.rs`:
>
> - **`Hsl`/`Hsv` — "algoritmo standard" tem nome e origem.** `color.rs:26-27`:
>   ```rust
>   pub type Hsl = palette::hsl::Hsla<encoding::Srgb, f32>;
>   pub type Hsv = palette::hsv::Hsva<encoding::Srgb, f32>;
>   ```
>   O vanilla delega na crate `palette` sobre encoding sRGB — não define algoritmo próprio.
>   A afirmação do L0 é correcta e agora tem origem; "standard" = o hexcone de `palette`
>   com encoding sRGB. Doc de superfície: `color.rs:686-702`
>   (`#[func(title = "HSL", since = "0.9.0")] pub fn hsl(…)`), publicado em
>   `typst.app/docs/reference/visualize/color/#definitions-hsl`.
> - **`Luma` — "Rec.709" não aparece na fonte.** O vanilla declara
>   `pub type Luma = palette::luma::Lumaa<encoding::Srgb, f32>` (`color.rs:28`) e converte
>   via `Luma::from_color(...)` (`color.rs:1771-1774`). Os coeficientes de luminância são,
>   portanto, os de `palette` para encoding sRGB — que **são** os primários Rec.709/sRGB,
>   mas a atribuição "Rec.709" é **inferência** do cristalino a partir do encoding, não uma
>   citação. Refutável por comparação numérica dos coeficientes com os de `palette`. Marcada
>   como tal; a medição §P744 abaixo mostra que o resultado observável coincide.
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

## Operadores de cor (P476/P477, **semântica corrigida em P742**)

**P742 — medição ADR-0108 (refuta a semântica P476/P477).** A sonda contra
o vanilla 0.15.0 (969087ec) mediu, para `red` = `rgb(1.0, 0.254902, 0.211765)`
(o `red` vanilla **não** é `#ff0000` — `visualize/color.rs:311`):

| Expressão | Vanilla (medido) |
|---|---|
| `red.lighten(20%)` | `rgb("#ff675e")` |
| `red.darken(20%)` | `rgb("#cc342b")` |
| `red.negate()` | `rgb("#004b74")` |
| `red.rotate(90deg)` | `rgb("#87a100")` |
| `red.mix(blue)` | `oklab(61.08%, 0.075, -0.031)` |
| `red.saturate(20%)` | `rgb("#ff372b")` |
| `red.desaturate(20%)` | `rgb("#ff675e")` |

Todas as fórmulas foram reproduzidas num scratch com `palette 0.7.6`
(a crate que o vanilla usa — `Cargo.lock`) e bateram **exactamente** nos
bytes medidos. A semântica P476/P477 (lighten/darken via Oklch, negate =
complemento sRGB, saturate = chroma Oklch) estava **errada** e é substituída.

### Semântica vanilla (fonte: `visualize/color.rs:1549-1653` + palette 0.7.6)

Fórmula base do palette (`macros/lighten_saturate.rs`), por componente `c`
com máximo 1.0 e factor `f`:

```text
increase(c, f) = clamp(c + (1 - c) * f, 0, 1)   se f >= 0
                 clamp(c + c * f, 0, 1)          se f < 0
```

- **`lighten(f)`** — `increase` aplicado **no espaço da própria cor**:
  Srgb/LinearRgb (r, g, b), Luma (l), Oklab (l), Oklch (l), Hsl (l), Hsv (v).
  Cmyk (tipo próprio do vanilla, `color.rs:2235`): `clamp(u - u*f, 0, 1)`
  por componente (c, m, y, k).
- **`darken(f)`** — `lighten(-f)` para as variantes palette; Cmyk:
  `clamp(u + (1-u)*f, 0, 1)` por componente.
- **`saturate(f)` / `desaturate(f)`** — Luma → **erro**
  ("cannot saturate/desaturate grayscale color"); Hsl/Hsv → `increase`
  sobre a saturação; **restantes** (Srgb, LinearRgb, Oklab, Oklch, Cmyk) →
  converte para Hsv, `increase` sobre `s`, converte **de volta ao espaço
  original**. `desaturate(f) = saturate(-f)`.
- **`negate(space: Option<ColorSpace>)`** — espaço default **Oklab**: converte
  para o espaço indicado, aplica `(1-l, -a, -b)` em Oklab, converte de volta
  ao espaço original de `self`. `space` pode ser qualquer um dos 8 espaços
  (medido: `negate(space: rgb)` → `#00bec9`).
- **`rotate(deg, space: Option<ColorSpace>)`** — espaço default **Oklch**:
  converte para o espaço indicado, soma `deg` ao hue, converte de volta ao
  espaço original de `self`. Só espaços com hue são válidos: **Oklch, Hsl,
  Hsv**; os restantes produzem erro "this color space does not support hue
  rotation" (medido: `rotate(90deg, space: rgb)` → erro).
- **`mix(other, weight, space: Option<ColorSpace>)`** — interpolação linear no
  espaço indicado; default **Oklab**. O resultado fica no espaço indicado
  (não no espaço original de `self`): `red.mix(blue, space: rgb)` → sRGB;
  `red.mix(blue, space: oklab)` → Oklab (medido).

### Novos métodos de domínio (P742)

```rust
impl Color {
    // Operadores reescritos (assinaturas P476/P477 mantidas, excepto
    // saturate/desaturate que passam a Option — Luma é erro no vanilla):
    pub fn lighten(self, factor: f32) -> Self;
    pub fn darken(self, factor: f32) -> Self;
    pub fn saturate(self, factor: f32) -> Option<Self>;   // None = grayscale
    pub fn desaturate(self, factor: f32) -> Option<Self>; // None = grayscale
    pub fn negate(self, space: Option<ColorSpace>) -> Self;
    pub fn rotate(self, angle_deg: f32, space: Option<ColorSpace>) -> Self;
    pub fn mix(self, other: Self, weight: f32, space: Option<ColorSpace>) -> Self;

    /// Espaço da cor (mapeamento 1:1 variante → ColorSpace).
    pub fn space(self) -> ColorSpace;

    /// Conversão para outro espaço. Hub sRGB: `to_rgba_f32` + conversão
    /// para o destino (Hsv/Hsl hexcone standard = palette; Luma = luminância
    /// linear re-codificada sRGB, Rec.709; Cmyk = naive `k = 1-max`,
    /// `c = (1-r-k)/(1-k)` — o ICC do vanilla é scope-out ADR-0083).
    pub fn to_space(self, target: ColorSpace) -> Color;

    /// Componentes da cor para `components()`.
    pub fn components(self, include_alpha: bool) -> Vec<ColorComponent>;
```

> **Fonte da afirmação sobre CMYK (P1031)** — o scope-out ADR-0083 estava correcto e agora
> tem `file:line`. O vanilla **usa mesmo um perfil ICC**, não uma fórmula:
> `crates/typst-library/src/visualize/color.rs:30-38`, citação literal:
>
> ```rust
> /// The ICC profile used to convert from CMYK to RGB.
> ///
> /// This is a minimal CMYK profile that only contains the necessary information
> /// to convert from CMYK to RGB. It is based on the CGATS TR 001-1995
> /// specification. See
> /// <https://github.com/saucecontrol/Compact-ICC-Profiles#cmyk>.
> static CMYK_TO_XYZ: LazyLock<ColorProfile> = LazyLock::new(|| {
>     ColorProfile::new_from_slice(typst_assets::icc::CMYK_TO_XYZ).unwrap()
> });
> ```
>
> O perfil é **CGATS TR 001-1995**, carregado de `typst_assets::icc::CMYK_TO_XYZ` — precisão
> que o L0 não tinha. A aproximação naive do cristalino é divergência **declarada e visível
> no observável** (as cores CMYK não coincidem numericamente), não mecânica interna;
> continua coberta pelo scope-out ADR-0083, mas fica registada como divergência de
> resultado, não de implementação. Nota adicional do vanilla (`color.rs:640-641`):
> *"Note that CMYK colors are not currently supported when PDF/A output is enabled."*

```rust

    /// **P744** — hex string da cor em sRGB (com alpha quando < 1.0).
    /// Paridade vanilla `to_hex`: `red.to_hex()` → `"#ff4136"`;
    /// `rgb("#ff413680").to_hex()` → `"#ff413680"`.
    pub fn to_hex(self) -> String;

    /// **P744** — diminui opacidade por `factor` (Ratio): `alpha' = alpha * (1 - factor)`.
    /// Paridade vanilla medido: `rgb("#ff413680").transparentize(50%)` → `#ff413640`.
    pub fn transparentize(self, factor: f32) -> Self;

    /// **P744** — aumenta opacidade por `factor` (Ratio): `alpha' = alpha + factor * (1 - alpha)`.
    /// Paridade vanilla medido: `rgb("#ff413680").opacify(50%)` → `#ff4136c0`.
    pub fn opacify(self, factor: f32) -> Self;
}

/// Componente heterogéneo de `Color::components` (Ratio/Float/Angle —
/// o domínio não conhece `Value`; a camada rules mapeia).
pub enum ColorComponent {
    Ratio(f32),  // componente [0,1] — imprime como percentagem
    Float(f32),  // a/b de Oklab, chroma de Oklch
    Angle(f32),  // hue em graus (rem_euclid 360 — paridade `hue_angle`)
}
```

`components` por variante (paridade `color.rs:1455-1522`; Cmyk ignora
alpha; as restantes omitem o alpha quando `include_alpha == false`):

- Srgb/LinearRgb: `[Ratio r, g, b, (a)]`
- Luma: `[Ratio l, (a)]`
- Oklab: `[Ratio l, Float a, Float b, (alpha)]`
- Oklch: `[Ratio l, Float c, Angle h, (alpha)]`
- Hsl: `[Angle h, Ratio s, Ratio l, (a)]`
- Hsv: `[Angle h, Ratio s, Ratio v, (a)]`
- Cmyk: `[Ratio c, m, y, k]`

**Critérios P742** (substituem os critérios P476/P477 errados):
- `Color::rgb(0xFF, 0x41, 0x36).lighten(0.2).to_srgb()` → `(255, 103, 94, 255)`.
- `…darken(0.2)` → `(204, 52, 43, 255)`.
- `…negate()` → `(0, 75, 116, 255)`.
- `…rotate(90.0)` → `(135, 161, 0, 255)`.
- `…saturate(0.2)` → `(255, 55, 43, 255)`; `…desaturate(0.2)` → `(255, 103, 94, 255)`.
- `Color::luma(0.5).saturate(0.2)` → `None` (erro grayscale no chamador).
- `lighten(0.0)` / `darken(0.0)` → cor idêntica (mesma variante).
- `saturate(0.0)` / `desaturate(0.0)` → cor idêntica.
- `mix` — critérios P476 mantidos (paridade confirmada).
- **P744**:
  - `Color::rgb(255, 65, 54).negate(Some(ColorSpace::Oklab)).to_hex()` → `"#004b74"`.
  - `Color::rgb(255, 65, 54).rotate(90.0, Some(ColorSpace::Oklch)).to_hex()` → `"#87a100"`.
  - `Color::rgb(255, 65, 54).mix(Color::rgb(0, 116, 217), 0.5, Some(ColorSpace::Oklab)).to_hex()` → `"#a37095"`.
  - `Color::rgb(255, 65, 54).mix(Color::rgb(0, 116, 217), 0.5, Some(ColorSpace::Srgb)).to_hex()` → `"#805a88"` (paridade exata vanilla confirmada no P1076).
  - `Color::rgba(255, 65, 54, 128).transparentize(0.5).to_hex()` → `"#ff413640"`.
  - `Color::rgba(255, 65, 54, 128).opacify(0.5).to_hex()` → `"#ff4136c0"`.
  - `Color::rgb(255, 65, 54).to_hex()` → `"#ff4136"`.

> **Proveniência dos critérios P744 (P1031) — medidos agora, e dois estavam errados.**
>
> A rubrica P744 alegava *"Paridade vanilla medido"* sem comando nem resultado. Medição
> directa em 2026-08-13, com o documento
>
> ```typ
> #let c = rgb(255, 65, 54)
> #let d = rgb(0, 116, 217)
> 1:#c.negate(space: oklab).to-hex() \
> 2:#c.rotate(90deg, space: oklch).to-hex() \
> 3:#c.mix(d, space: oklab).to-hex() \
> 4:#c.mix(d, space: rgb).to-hex() \
> 5:#rgb(255,65,54,128).transparentize(50%).to-hex() \
> 6:#rgb(255,65,54,128).opacify(50%).to-hex() \
> 7:#c.to-hex()
> ```
>
> Vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) vs cristalino
> `target/release/typst` (fonte em HEAD `4f64e4e69`, árvore só com edições em
> `00_nucleo/prompts/**`), lido por `pdftotext`:
>
> | # | Operação | Vanilla | Cristalino | Redacção P744 anterior |
> |---|---|---|---|---|
> | 1 | `negate(oklab)` | `#004b74` | `#004b74` ✅ | `#004b74` ✅ |
> | 2 | `rotate(90deg, oklch)` | `#87a100` | `#87a100` ✅ | `#87a100` ✅ |
> | 3 | `mix(d, oklab)` | `#a37095` | `#a37095` ✅ | `#805b87` ❌ |
> | 4 | `mix(d, rgb)` | `#805a88` | `#805a88` ✅ | `#805b87` ❌ |
> | 5 | `transparentize(50%)` | `#ff413640` | `#ff413640` ✅ | ✅ |
> | 6 | `opacify(50%)` | `#ff4136c0` | `#ff4136c0` ✅ | ✅ |
> | 7 | `to-hex()` | `#ff4136` | `#ff4136` ✅ | ✅ |
>
> **Duas conclusões.** (a) Os dois critérios de `mix` estavam errados no L0 — davam o mesmo
> valor para Oklab e sRGB, o que a medição refuta; corrigidos acima para os valores do
> vanilla. (b) Cinco das sete operações estão em paridade exacta, incluindo `mix` em Oklab,
> o que também valida indirectamente a cadeia Oklab e a re-codificação de luminância
> discutida em §"Fonte de paridade dos espaços de cor".
>
> **ACHADO ESCALADO (prioridade baixa) — `mix` em sRGB difere numa unidade no verde**:
> vanilla `#805a88`, cristalino `#805b88` (0x5a = 90 vs 0x5b = 91). Um passo em 255 num só
> canal, e só no espaço sRGB — provável diferença de arredondamento na interpolação. Gate
> ADR-0127 por ser mudança de comportamento; passo próprio. **Não implementado aqui.**
>
> **Lacuna observada na mesma medição, fora deste achado**: o cristalino não aceita a forma
> de peso da linguagem `c.mix((d, 50%), space: rgb)` — erro
> `color.mix: argumento 'col2' deve ser Color, recebeu array`, enquanto o vanilla devolve
> `#aa526c`. Registada no relatório do P1031.

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
