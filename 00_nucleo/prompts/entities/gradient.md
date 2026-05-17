# Prompt L0 — `entities/gradient`
Hash do Código: 38ce26d6

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

---

## Anotação cumulativa P267 — Conic variant materializada (cluster Gradient 3/3 completo)

**Data**: 2026-05-15.

Subset Conic materializado per ADR-0089 (Opção α — ADR nova
dedicada paridade pattern N=5 P261/P262/P264). **Cluster
Gradient L1+stdlib completo 3/3** (Linear + Radial + Conic).

### Tipos adicionados

```rust
pub struct Conic {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle:  Angle,
}

impl Conic {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear/Radial
    pub fn sample(&self, t: f32) -> Color;        // paridade (Oklab)
}
```

### Enum Gradient expandido (3/3 variants completo)

```rust
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),
    Conic(Arc<Conic>),    // P267 — descomentado
}

impl Gradient {
    pub fn linear(stops, angle) -> Self;
    pub fn radial(stops, center, radius) -> Self;  // P264
    pub fn conic(stops, center, angle) -> Self;    // P267
    pub fn first_stop_color(&self) -> Color;       // pattern-match 3-arm
}
```

### Scope-outs P267 (per ADR-0089)

- `space` (Oklab fixo — paridade P262/P264).
- `relative` (bbox-local — paridade P262/P264).
- `anti_alias` (true assumed — paridade P262/P264).
- **PDF emit Conic fallback Solid** até **P268 dedicado**.
- **Sem `focal_*` scope-out** — não existe em ConicGradient
  vanilla (exclusivo Radial).

### Cross-references P267

- ADR-0089 — Gradient Conic-only (IMPLEMENTADO P267).
- ADR-0088 §"variants não materializados" parcialmente
  revogado por este passo (Conic activado; `focal_*` Radial
  preservado).
- ADR-0086 — Paint::Gradient automaticamente absorve Conic
  (zero cascade refactor).
- P262/P264 — precedentes directos; helpers Oklab reutilizados
  literal (subpadrão "Reutilização literal helpers
  cross-passos" N=1 → N=2).
- P268 (futuro) — PDF emit Conic dedicado (replica P263/P265
  template).

---

## Anotação P268 — PDF Conic shading (`/ShadingType 4` Gouraud)

**Data**: 2026-05-15.

PDF render Conic materializado per ADR-0089 §"Anotação
cumulativa P268". **Promessa P267 fechada**: 3 sítios
pattern-match `Gradient::Conic(_) => continue/fallback` em
`03_infra/src/export.rs` substituídos por emit real.

### Estratégia: Type 4 Free-Form Gouraud Triangle Mesh

PDF Spec ISO 32000 §7.5.7. Triangulação manual do disco em
N=32 fatias (sem dependência crate externa krilla — vanilla
delega para krilla::SweepGradient não autorizada cristalino).

Estrutura PDF Conic Shading:
- `/ShadingType 4` (Gouraud free-form triangle mesh).
- `/ColorSpace /DeviceRGB`.
- `/BitsPerCoordinate 8` (256 níveis bbox unit).
- `/BitsPerComponent 8` (RGB sRGB).
- `/BitsPerFlag 8` (flag=0 todos os triângulos).
- `/Decode [0 1 0 1 0 1 0 1 0 1]`.
- Stream binary: 96 vertices × (1 flag + 1 x + 1 y + 3 RGB)
  = 576 bytes para N=32 triangles.

Cor central = primeiro stop (paridade fallback). Cores em
edges = `Conic::sample(i/N)`. Helpers Oklab P262/P263/P265
reutilizados literal.

Cluster Gradient L1+stdlib+PDF **3/3 completo** pós-P268.

**Anotação P268.1**: divergência arquitectural Type 4 cristalino
vs estratégia vanilla actual desconhecida (krilla `SweepGradient`
interno opaco; Typst original pré-krilla era Type 6 Coons per
blog 2023 — Part 7 #5420 transitou para krilla) formalizada em
ADR-0090 EM VIGOR; convenção cor central = primeiro stop
confirmada como industry standard Cairo/PDF (não decisão
arbitrária P268). Cristalino Type 4 alinhado com Cairo
(Type 6/7) / Inkscape (Type 7) / Typst original (Type 6 Coons)
— todos família mesh-based; divergência intra-família mesh
(Type 4 vs Type 6). ADR-0018 preservado (krilla não autorizada).
Refino qualidade visual pendente P268.2 (adaptive N hybrid;
spec futura).

**Anotação P268.2**: `emit_conic_gouraud_stream` callsite production
usa N adaptive hybrid 1+2 (número de stops + contraste Oklab ΔE) em
vez de N=32 fixo; fórmula §A.6 `diagnostico-adaptive-n-passo-268-2.md`
com `factor_delta = 256.0` calibrado para Oklab canónico (Björn
Ottosson; W3C CSS Color 4). ADR-0090 preservada literal (estratégia
Type 4 intocada). Helper privado `color_to_oklab_with_alpha`
promovido a `pub fn` para acessibilidade cross-crate L3 (function
body preservada literal); wrapper L3 `oklab_delta_e` ~8 LOC.
Assinatura `emit_conic_gouraud_stream(conic, n_slices)` preservada
literal — adaptive N entra apenas no callsite, zero regressão dos
6 tests P268 originais. Cluster Gradient PDF cristalino transita
para qualidade visual **industry-grade**.

---

## Anotação cumulativa P269 — Radial focal_center + focal_radius activados

**Data**: 2026-05-15.

Subset Radial estendido per ADR-0088 §"Anotação cumulativa P269"
(anotação cumulativa N=7; subpadrão "ADR scope-out revogado
parcialmente" N=1 → N=2 — P267 Conic + **P269 focal_***).

### Tipos estendidos

```rust
pub struct Radial {
    pub stops:        Arc<[GradientStop]>,
    pub center:       Axes<Ratio>,
    pub radius:       Ratio,
    pub focal_center: Axes<Ratio>,  // P269 — novo campo; default via construtor = center
    pub focal_radius: Ratio,        // P269 — novo campo; default via construtor = Ratio(0.0)
}

impl Gradient {
    pub fn radial(stops, center, radius) -> Self;                // P264; default focal=(center, 0)
    pub fn radial_with_focal(stops, center, radius,              // P269 — focal explícito
                             focal_center, focal_radius) -> Self;
}
```

### Stdlib estendido

```text
#gradient.radial(red, blue, focal_center: (30%, 40%), focal_radius: 10%)
```

Named args novos: `focal_center: Array [Ratio, Ratio]` (default =
center), `focal_radius: Ratio` (default `0%`).

### Validações stdlib portadas vanilla

- `focal_radius > radius` → erro.
- `dist(focal_center, center)² >= (radius - focal_radius)²` → erro
  (focal circle deve estar dentro do outer circle).

### Defaults preservam P264 — zero regressão

- `Gradient::radial(stops, center, radius)` sem focal → focal=(center, 0).
- Stdlib `gradient.radial(...)` sem named focal_* → idem.
- L3 `/Coords [cx cy 0 cx cy r]` (idêntico P265 para defaults).
- 16 tests P264/P265 preservados verdes literal (assertions intactas;
  struct literal sites recebem 2 campos novos com valores trivial).

### `Radial::sample(t)` inalterado

`Radial::sample(t)` permanece 1D pura (interpolação Oklab sobre
offsets dos stops; não usa coordenadas 2D nem focal). Focal só
afecta L3 PDF emit (`/Coords` Type 3 nativo).

### Scope-outs preservados pós-P269

- `space: ColorSpace` — Oklab fixo (paridade P262/P264).
- `relative: Smart<RelativeTo>` — bbox-local.
- `anti_alias: bool` — true assumed.

### Cross-references P269

- ADR-0088 §"Anotação cumulativa P269" — fórmula completa.
- ADR-0054 anotação P269 — perfil graded preservado.
- `00_nucleo/diagnosticos/diagnostico-gradient-focal-passo-269.md`
  — diagnóstico imutável (sétimo consumo directo de fonte vanilla).
- P264 — Radial L1+stdlib subset 3 campos (precedente directo).
- P265 — PDF Radial /ShadingType 3 (template emit; `compute_radial_coords`
  estendido).
- P267 — Conic activado (precedente "ADR scope-out revogado
  parcialmente" N=1).

---

## ColorSpace runtime (P270 — cross-variant)

Cada variant (Linear/Radial/Conic) tem campo `space: ColorSpace`:

- Default = `ColorSpace::Oklab` (preserva P262/P264/P267 behavior
  bit-exact).
- `sample(t)` interpola no space escolhido via dispatcher
  `interpolate_in_space(c0, c1, t, space)`.
- Hue-wrap shorter default para HSL/Oklch/HSV (CSS standard; vanilla
  paridade literal).

### `ColorSpace` enum criado P270

```rust
pub enum ColorSpace {
    Oklab, Oklch, Srgb, Luma, LinearRgb, Hsl, Hsv, Cmyk,
}
```

8 variants paridade vanilla (Luma ≡ D65Gray nome cristalino histórico).
Localização: `01_core/src/entities/color.rs`.

### Construtores estendidos

```rust
impl Gradient {
    // Defaults Oklab preservam P262/P264/P267 behavior.
    pub fn linear(stops, angle) -> Self;
    pub fn radial(stops, center, radius) -> Self;            // P269 defaults focal
    pub fn conic(stops, center, angle) -> Self;

    // P270 — explicit space cross-variant.
    pub fn linear_with_space(stops, angle, space) -> Self;
    pub fn radial_with_space(stops, center, radius, space) -> Self;
    pub fn conic_with_space(stops, center, angle, space) -> Self;

    // P269 (preservado).
    pub fn radial_with_focal(stops, center, radius, fc, fr) -> Self;
}
```

### Stdlib user-facing

```text
#gradient.linear(red, blue, space: "hsl")
#gradient.radial(red, blue, space: "oklch")
#gradient.conic(red, blue, space: "srgb")
```

Named arg `space: Str` aceita `"oklab" | "oklch" | "srgb" | "luma" |
"linear-rgb" | "hsl" | "hsv" | "cmyk"`. Default sem named arg → Oklab.

### Hue-wrap shorter (HSL/Oklch/HSV)

Para polar spaces, hue interpolada via caminho mais curto (CSS
standard; vanilla paridade literal portada de `mix_iter` linha
1126-1136). Wrap se `|h1 - h0| > 180°`.

### L3 emit preservado P270 (refactor adiado P270.1+P270.2)

L3 PDF emit ainda Oklab pipeline P263/P265/P268. Multi-space emit:
- P270.1 (futuro M+): 7 spaces RGB-family + perceptual via Oklab
  pipeline N=16 → DeviceRGB.
- P270.2 (futuro S+): CMYK directo `/DeviceCMYK` (revoga ADR-0083 §CMYK).

Cluster Gradient L1+stdlib **feature-complete em 3 variants × 8 spaces**
pós-P270.

### Cross-references P270

- ADR-0091 — Gradient ColorSpace runtime + CMYK strategy (criada
  PROPOSTO+IMPLEMENTADO P270).
- ADR-0083 — Color paridade (anotada cumulativa P270; §ColorSpace
  runtime revogado parcialmente).
- ADR-0054 — Perfil graded (anotada cumulativa P270).
- ADR-0087/0088/0089/0090 — Variant strategies (anotadas cumulativa
  P270).
- `00_nucleo/diagnosticos/diagnostico-gradient-space-passo-270.md`
  — diagnóstico imutável (oitavo consumo directo de fonte vanilla).
- P262/P264/P267 — Linear/Radial/Conic L1+stdlib (precedentes
  directos extendidos).
- P269 — Radial focal_* (preservado; campo space adicional).

**Anotação P270.1**: pipeline L3 emit ganha consciência de space —
helpers L3 renomeados `oklab_sample_stops_*` → `multispace_sample_stops_*`
(`03_infra/src/export.rs`). **Body literal preserved** porque
`<variant>.sample(t)` despacha via P270 `interpolate_in_space`
automaticamente — P270 já passou L3 multi-space implicitamente.
7 spaces materializados L3 emit (Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV);
CMYK preservado via pipeline natural CMYK→sRGB sub-óptimo (P270.2
materializa `/DeviceCMYK` directo). Defaults Oklab preservam bytes
pré-P270.1 bit-exact (2500 baseline preservados). Ver ADR-0091
§"Anotação cumulativa P270.1".

**Anotação P270.2**: CMYK emit branch directo `/DeviceCMYK`
materializado em L3 para **Linear+Radial** (Cenário B). Pipeline
dual em `emit_gradient_objects`: `space == Cmyk` → shading
`/ColorSpace /DeviceCMYK` + Function 4-component
(`/Range [0 1 0 1 0 1 0 1]`); senão P270.1 pipeline literal
preserved. Helpers L3 novos: `multispace_sample_stops_<variant>_cmyk(n)`
retorna `Vec<(c, m, y, k)>` via `Color::Cmyk` pattern-match
(dispatcher P270 arm Cmyk). **Conic CMYK preserved scope-out**
P270.2 (§A.8 diagnóstico Cenário B — vanilla suporte/reader
compatibility incertos; candidato P-Gradient-Conic-CMYK futuro);
Conic com `space: Cmyk` usa pipeline P270.1 fallback sub-óptimo
(CMYK→sRGB). **Bug vanilla #4422 resolvido por construção**
(cristalino emit `/DeviceCMYK` correcto). **ICC profiles scope-out
preserved** (candidato P-Gradient-CMYK-ICC futuro). ADR-0083
§DeviceCMYK revogado parcialmente. Cluster L3 emit Linear+Radial
**8/8 spaces**; Conic 7/8 + CMYK fallback. Ver ADR-0091 §"Anotação
cumulativa P270.2".

**Anotação P270.3**: infra-estrutura **Type 6 Coons Patch Mesh**
materializada como estratégia adicional Conic L3 emit (preparação
CMYK P270.4 via ADR-0092 EM VIGOR). **Conic ganha 2 emit paths
coexistentes** — Type 4 Gouraud (P268+P268.2 preserved; RGB
default) + Type 6 Coons (P270.3 infra; P270.4 activa para
`space == Cmyk`). **Dispatcher opt-in flag interno** (não user-facing;
default OFF P270.3; ON P270.4). **Strategy "1 patch per stop"**
(paridade Typst original blog 2023; N stops → N patches angulares).
**Matemática Bezier cúbico arc círculo** (Stanislaw Adaszewski):
offset = r·(4/3)·tan(angle/4). Helpers L3 novos:
`bezier_control_points_for_arc`, `compute_coons_patches_n_stops`,
`emit_conic_coons_stream` (37 bytes per patch: 1 flag + 12 control
points × 2 coord + 4 corner colors × 3 RGB). **`#[allow(dead_code)]`**
marcações P270.3 (helpers serão usados P270.4). ADR-0090 §Type 6
scope-out **revogado parcialmente** (Type 6 sai; Type 7 preserved).
Sub-padrão "ADR scope-out revogado parcialmente" N=4 → **N=5
cumulativo limiar formalização clara muito ultrapassado** —
candidato meta-ADR URGENTE. 2545 baseline preserved bit-exact
(flag default OFF). Ver ADR-0092 EM VIGOR.

**Anotação P270.4**: opt-in flag Coons CMYK **activado** (cluster L3
24/24 absoluto). `emit_conic_coons_stream_cmyk` variant materializado
(corner colors 4-component CMYK; **41 bytes/patch** vs 37 RGB).
Dispatcher Conic em `emit_gradient_objects`:
- `space == Cmyk` → `/ShadingType 6 /ColorSpace /DeviceCMYK` Coons
  (Decode `[0 1 0 1 0 1 0 1 0 1 0 1]` 6 pares x,y,c,m,y,k).
- senão → `/ShadingType 4 /ColorSpace /DeviceRGB` Gouraud P268+P268.2
  literal preserved.
Helpers Coons P270.3 perdem `#[allow(dead_code)]` (agora em uso).
**Adaptive N NÃO se aplica a Coons** (strategy 1 patch per stop;
N = stops.len()). Sub-decisão prévia recalibrar factor_delta CMYK
preservada reserva (P-Gradient-Adaptive-Multispace candidato futuro).
**Bug vanilla #4422 resolvido por construção** absoluto (3 variants ×
CMYK). **ADR-0091 §Conic CMYK scope-out** revogado **final**.
**ADR-0083 §DeviceCMYK PDF** revogado **definitivo**. Sub-padrão
"ADR scope-out revogado parcialmente" N=5 → **N=6 cumulativo limiar
ainda mais ultrapassado** — meta-ADR URGENTE FINAL. **Cluster
Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto** —
marco arquitectural máximo. Série P270 completa (P270 + P270.1 +
P270.2 + P270.3 + P270.4). Ver ADR-0092 §"Anotação cumulativa
P270.4".
