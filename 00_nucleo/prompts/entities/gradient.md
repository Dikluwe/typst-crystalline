# Prompt L0 — `entities/gradient`
Hash do Código: ebc84366

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

**Anotação P272**: Conic L3 emit **estratégia unificada Coons** —
`emit_conic_coons_stream_rgb` (P270.3 helper activado + extension
**N=stops*4** patches angulares) + `emit_conic_coons_stream_cmyk`
(P270.4 preserved literal). Dispatcher Conic **único** `/ShadingType 6`
para 8/8 spaces (Decode 5 pares RGB ou 6 pares CMYK; Function Type 2
N=1 identity 3 ou 4 components). Corner colors interpolated via
`Conic::sample(t)` dispatcher P270 (dispatches `interpolate_in_space`
per `conic.space` automaticamente). **Helpers REMOVED P272**:
`emit_conic_gouraud_stream` + `compute_adaptive_n_conic` +
`oklab_delta_e` (P268+P268.2). **20 tests P268+P268.2 REMOVED** + ~18
tests P272 ADDED (net -2). **ADR-0090 REVOGADO P272** (Type 4 Gouraud
descontinuado); **ADR-0092 expandida cumulativamente**. Sub-padrão
"ADR REVOGADO + substituta" N=2 → **N=3 cumulativo**. Sub-padrão
"Aplicação meta-ADR (ADR-0093 + ADR-0094)" **N=1 inaugural cada** —
primeira aplicação prática pós-formalização P271. **Cluster Gradient
L3 emit estratégia única Coons feature-complete 24/24 simplificado**.
Ver ADR-0092 §"Anotação cumulativa P272 — Decisão Cenário A revisado
FINAL".

**Anotação P273**: cluster Gradient cross-variant ganha campo
`relative: Option<RelativeTo>` (default `None` = Auto = Self_;
preserva P272 literal). Enum `RelativeTo { Self_, Parent }` com
`Default = Self_`; ADR-0064 §Caso A pattern (`Smart<T>` → `Option<T>`
cristalino). Stdlib named arg `"relative"` cross-variant
("self"/"parent"/"auto"; whitelist estendida em 3 fns). L3 dispatcher
dual:
- `Self_` (default) → pipeline P272 preserved literal (page-relative
  coords).
- `Parent` → `apply_parent_transform(local, parent_bbox)` em Rust
  nativo (paridade vanilla `correct_transform`; PDF `/Matrix`
  identity). `parent_bbox = None` em callsites P273 (estrutural) →
  identity; futuro callsite poderá passar bbox real.

**Cluster Gradient cross-variant runtime fields canónica 3/3**:
focal_* (P269) + space (P270) + **relative (P273)**.

Industry research consolidada P273 — Cairo/Inkscape/vanilla usam
transform Rust nativo (paridade cristalino); PDF `/Matrix`
rejeitado por todos. Sub-padrão "Anotação cumulativa em vez de ADR
nova" N=12; "Reutilização literal helpers" N=12; "Cap LOC hard/soft"
N=6; "Fase A com industry research proactiva" N=5 (limiar
formalização clara muito ultrapassado); "Aplicação meta-ADR
(ADR-0094)" N=2 cumulativo. Ver ADR-0091 §"Anotação cumulativa P273
— Cross-variant runtime fields".

**Anotação P274**: Adaptive N multispace refino qualitativo para
Linear+Radial RGB-family + perceptual. Pré-amostragem N=16 fixo
substituída por `adaptive_n_for_stops(stops, space)` (threshold
based; 16/32/64 níveis baseados em max_pair_delta_e Oklab). Cap
N_max=64 (4× N_base). Conic preserved P272 literal (Coons N=stops*4);
CMYK preserved P270.2 directo. Novo helper L3 privado
`perceptual_distance_in_space(c0, c1, space)` — genérico cross-space
desacoplado (`color_to_oklab_with_alpha` P262 reused 4ª vez literal).
Defaults pastel low-contrast preservam N=16 (paridade P270.1 emit
literal); red→blue high-contrast escala para N=64 deterministic.
Tests P270.1 reproducibility (`pdf1==pdf2`) preserved zero regressão.
Sub-padrão "Reutilização literal helpers cross-passos" N=12 → N=13
consolidação clara persistente; "Anotação cumulativa em vez de ADR
nova" N=12 → N=13 consolidação clara persistente; "Cap LOC hard/soft"
N=6 → N=7 consolidação total; "Fase A com industry research
proactiva" N=5 → N=6 (4 fontes W3C/Ottosson/Skia/Cairo); "Aplicação
meta-ADR (ADR-0093)" N=1 → N=2; "Aplicação meta-ADR (ADR-0094)" N=2
→ N=3. Ver ADR-0091 §"Anotação cumulativa P274".

**Anotação P273.5**: Fecho pendência P273 `apply_parent_transform`
em `#[allow(dead_code)]` via callsite real L3 emit gradient.
Decisão 3γ híbrida (Fase A): callsite L3 passa `page_bbox` como
fallback parent_bbox (3γ.1 materializado) — cobre semântica vanilla
"shape top-level com relative=Parent ancora à página". Refino futuro
3γ.2 (Block/Boxed/Group save/restore real) é pendência preservada
incremental per ADR-0054 graded; estructuralmente preparado via
campo `parent_bbox: Option<Rect>` L1 Layouter. Novo `Rect` struct
em L1 paridade `Point`/`Size` (em `entities/layout_types.rs`).
Padrão DEBT-37 P84.6 `cell_origin_*: Option<f64>` reused
estructuralmente — campo opcional Layouter para contexto pai;
sub-padrão emergente "Pattern DEBT-37 replicado" N=1 → N=2 (meio
caminho limiar formalização N=3-4). `#[allow(dead_code)]` removido
de `apply_parent_transform` — função tem callsites L3 reais.
Defaults `relative: None/Some(Self_)` preservam pipeline P272+P273+P274
bit-exact. Sub-padrão "Anotação cumulativa em vez de ADR nova" N=13
→ N=14 (sétima anotação consecutiva ADR-0091); "Reutilização literal
helpers cross-passos" N=13 → N=14 consolidação clara persistente.
Ver ADR-0091 §"Anotação cumulativa P273.5".

**Anotação P273.6**: Fecho 3γ.2 — `parent_bbox` Layouter ganha
consumer real via save/restore em `Content::Block` arm + propagação
ao L3 via `FrameItem::Shape.parent_bbox_at_emit: Option<Rect>` (cascade
~86 sites bulk-patched). Decisão 1 (3γ.2.γ): popular apenas quando
`width.is_some() && height.is_some()` (Decisão Fase A). Decisão 2
(Prop-A revisitada): `FrameItem::Shape` + `GradientObject` ambos
ganham `parent_bbox_at_emit: Option<Rect>`. Decisão 3 (escopo
contentores): {Block} apenas; Boxed difere P273.7; outros scope-out
per ADR-0054 graded. **`#[allow(dead_code)]` Layouter `parent_bbox`
removed** — campo consumed por Block arm (write) + emit shape (read).
Pattern DEBT-37 `cell_origin_*` replicado N=3 cumulativo (P84.6 +
P273.5 estrutural + P273.6 com consumer real) — atinge limiar
formalização ADR meta N=3-4. Defaults `relative: None/Some(Self_)` +
Block sem dimensions literais preservam P262-P273.5 bit-exact (cai
no fallback page_bbox L3 P273.5). Sub-padrão "Anotação cumulativa em
vez de ADR nova" N=14 → N=15 (oitava anotação consecutiva ADR-0091);
"Reutilização literal helpers cross-passos" N=14 → N=15; "Cap LOC
hard vs soft explícito" N=8 → N=9 consolidação total. Ver ADR-0091
§"Anotação cumulativa P273.6".

**Anotação P273.7**: Estende Decisão 3 P273.6 de `{Block}` para
`{Block, Boxed}` aplicando o template save/restore literal ao arm
`Content::Boxed`. **Decisão 1 (semântica bbox.y inline fixada
`3γ.2.γ-inline-baseline-y`)**: `bbox.y = self.regions.current.cursor_y`
literal (baseline-relative). Aproximação aceitável — coerente com
limitação consciente pré-existente P156H "height em contexto inline
alteraria line_height — refino futuro"; refino topo-exacto fica
registado como `P273.X-bis2` per ADR-0054 graded. **Decisões 2/3/4
herdadas P273.6 literal**: 3γ.2.γ W/H (popular apenas com
width+height literais); Prop-A revisitada inalterada (emit shape
sites do Boxed já populam `parent_bbox_at_emit` desde P273.6); escopo
contentores estendido para `{Block, Boxed}` (Stack/Pad/Group/Grid cell
scope-out per ADR-0054 graded). **Sem cascade novo**: cascade ~86
sites já feito P273.6; L3 GradientObject + dispatcher inalterados.
Defaults `relative: None/Some(Self_)` + Boxed sem dimensions literais
preservam P262-P273.6 bit-exact. Pattern DEBT-37 N=3 cumulativo
preserved (P273.7 é extensão da 3ª aplicação, não 4ª). Sub-padrão
"Anotação cumulativa em vez de ADR nova" N=15 → N=16 (nona anotação
consecutiva ADR-0091); "Template-passo replicado literal" N=0 → N=1
emergente (save/restore P273.6 replicado a Boxed com bbox.y semantic
inline-baseline-y). Ver ADR-0091 §"Anotação cumulativa P273.7".

**Anotação P273.9**: Estende Decisão 3 de `{Block, Boxed}` para
`{Block, Boxed, Grid cell, Stack, Pad}` — escopo 1γ M magnitude
(Decisão utilizador). **Decisão 2 fixada (Grid bbox)**:
`Rect { x: body_x, y: body_y, w: body_w, h: body_h }` exacto cell —
todos 4 disponíveis pré-body via track resolution + insets aplicados.
**Decisão 3 fixada (Stack + Pad bbox)**: medição inline via
`measure_content_constrained` pre-layout. Stack — vertical
`max_w × sum_h`; horizontal `sum_w × max_h`; bbox no cursor
pós-flush_line. Pad — bbox INNER (body region, sem insets) paralela
a Block; `(body_w, body_h) = measure_content_constrained(body,
available_inner)`. **Sem cascade novo**; L3 dispatcher inalterado
desde P273.6. **Layout duplo arquitectural aceite** — sub-padrão
N=0 → N=1 inaugural emergente; custo perf ~1.5-2× **apenas em
pipelines com gradient `relative=parent`** (defaults Self_/None
preservam zero overhead). **Pattern DEBT-37 `cell_origin_*`
replicado N=3 → N=4 cumulativo** atinge limiar formalização
N=3-4 com folga consolidada (P84.6 + P273.5 + P273.6 + **P273.9**
Grid cell paralelo a `cell_origin_*`). **Template-passo replicado
literal N=1 → N=2 cumulativo** (Grid replica template Block/Boxed
literal; Stack/Pad replicam com adaptação layout duplo). Defaults
rigorosos (`body_w > 0 && body_h > 0`) preservam P262-P273.8
bit-exact. Sub-padrão "Anotação cumulativa em vez de ADR nova"
N=16 → N=17 (décima anotação consecutiva ADR-0091). Ver ADR-0091
§"Anotação cumulativa P273.9".

**Anotação P273.10**: Estende cobertura `parent_bbox` para
`FrameItem::Group` via **mecanismo L3 puro** (zero touch Layouter
L1). Inaugura sub-padrão "L3-only parent_bbox" N=1 — contentores
post-layout cuja bbox é conhecida apenas em L3 emit-time usam L3
dispatcher override em vez de L1 save/restore. **Decisão 1α**
(parameter threading): `scan_all_gradients` ganha helper recursivo
interno com `parent_bbox_override: Option<Rect>`; Group arm constrói
`group_bbox = Rect { pos, inner_width, inner_height }` e recurse;
Shape arm aplica **Inner-wins** via `parent_bbox_at_emit.or(override)`.
**Decisão 2α** (bbox): geometric exact em coords cristalino (sem
Y-inversion). **Decisão 3α** (precedence): Inner wins — Shapes com
campo populated pelos 5 containers Layouter (Block/Boxed/Grid/Stack/Pad)
mantêm-no; override Group ignorado nesse caso. **Scope creep arquitectural**:
`pattern_resources_for_page` também ganha recursão symmetric — bug
latent pré-existente onde gradients dentro de Groups não eram
registados/listados é corrigido em paralelo (sem o scope creep, a
feature não produz observable behavior). Defaults `relative:
None/Some(Self_)` + Shapes com campo populated preservam P262-P273.9
bit-exact. Sub-padrão "L3-only parent_bbox" distingue de Pattern
DEBT-37 (L1 save/restore) e Layout duplo arquitectural aceite (L1
`measure_content_constrained`). Sub-padrão "Anotação cumulativa em
vez de ADR nova" N=17 → N=18 (décima primeira anotação consecutiva
ADR-0091); "Sub-passos consecutivos do mesmo cluster" N=5 → N=6
emergente. Ver ADR-0091 §"Anotação cumulativa P273.10".

**Anotação P273.12**: Fecha limitação documentada P273.6 §9 quarto
bullet ("gradient com mesmo Arc usado em contextos distintos:
actualmente primeiro wins") preservada em todos relatórios P273.6-P273.11.
Refino arquitectural L3-puro: chave dedup expandida de `Arc::as_ptr`
para `DedupKey { arc_ptr: usize, bbox: Option<RectKey> }` com
`RectKey(i32, i32, i32, i32)` em milipontos quantizados (resolve
problemas de `f64` em HashMap key + preserva precisão sub-typográfica).
**Decisão 2β scan-side**: `scan_all_gradients.walk` +
`pattern_resources_for_page.walk` computam `effective_bbox =
parent_bbox_at_emit.or(parent_bbox_override)` e usam-no como dedup
key; `emit_stroke_paint` ganha `effective_bbox` param para construir
DedupKey lookup. **Decisão 3α**: pat_ptr_to_idx permanece global ao
documento. Trade-off PDF size: pior caso N callsites mesmo Arc + N
bboxes distintos → N patterns (semântica correcta); caso comum
preserved. Defaults preservam P262-P273.11 bit-exact:
`relative=self/None` (bbox=None) → DedupKey factorizes a singleton
key per Arc; Arc-único context idem. Apenas Arc com bboxes effective
distintos em N contexts produz N PDF patterns (vs primeira-wins
pre-P273.12). Sub-padrão **"Dedup Arc::as_ptr resources" N=2 → N=3
cumulativo crossing limiar formalização N=3-4** (P73 image + P263
pattern + **P273.12 pattern bbox-aware**); candidato meta-ADR
formalização NÃO reservado. Sub-padrão **"Bug arquitectural
intencional corrigido" N=0 → N=1 inaugural emergente** — limitação
corrigida 6 sub-passos depois com refino deliberado; distingue de
"Bug latent corrigido em scope creep". "Anotação cumulativa em vez
de ADR nova" N=18 → N=19 (décima segunda anotação consecutiva
ADR-0091). Ver ADR-0091 §"Anotação cumulativa P273.12".

**Anotação P273.13**: Fecha pendência P263 §8 #3 (2026-05-16) +
P273.12 §9 quarto bullet — `draw_item_local` (recursão Group em
build_page_stream_*) usava fallback solid color via
`s.paint.to_color()` em vez de consumir o pattern dict registado
em P273.10/P273.12. **Decisão 1α** (parameter threading): adiciona
3 params (`parent_bbox_override`, `pat_ptr_to_idx`, `pat_refs`)
paralelo ao P273.10 `scan_all_gradients.walk`. **Decisão 2α**
(Group bbox source): construção literal-equivalente a
`scan_all_gradients.walk` + `pattern_resources_for_page.walk` para
garantir `dedup_key_for` produzir chave idêntica → lookup encontra
pattern. **Decisão 3α** (coords): cristalino (Y-down) paridade scan.
**Scope creep aceito**: arm Group novo em `draw_item_local` corrige
bug pre-existente (nested Groups silenciosamente descartados via
`_ => {}`) + suporta recurse paralelo scan. **Sub-padrão "L3-only
parent_bbox" N=1 → N=2 cumulativo** — P273.10 inaugural
(scan_all_gradients) + **P273.13 reaplicação (draw_item_local)**;
padrão consolidado mas longe do limiar formalização N=3-4. Sub-padrão
**"Triplicação Group bbox" N=0 → N=1 emergente** — 3 sítios constroem
mesmo Rect literal (scan + pattern_resources + draw_item_local);
candidato extract helper P273.X-bis-helper-group-bbox NÃO reservado.
Defaults preservam P262-P273.12 bit-exact (top-level Shapes
unaffected; Group Shapes sem gradient solid preserved). "Anotação
cumulativa em vez de ADR nova" N=19 → N=20 (décima terceira anotação
consecutiva ADR-0091); "Sub-passos consecutivos do mesmo cluster"
N=8 → N=9 emergente. Ver ADR-0091 §"Anotação cumulativa P273.13".

**Anotação P273.14**: CMYK-ICC paridade — **SCOPE-OUT-RECONFIRMED**
via NO-GO Fase A binária. Razão tríplice: (1) Caminho 1 crate
externa requer ADR nova revogando invariante L0 export.md "sem
crates externas de PDF" — decisão arquitectural fora de escopo;
(2) Caminho 2 profile bytes hardcoded inviável — zero profiles
CMYK royalty-free industry-recognized para redistribuição (todos
proprietários Adobe/ECI/IDEAlliance; "generic CMYK no-profile"
royalty-free não existe per ICC.org Tech Note 7); (3) Caminho 3
scope-out preserved reconfirmado por evidência empírica P273.14
factual. `/DeviceCMYK` preserved como caminho actual; PDF/A
compliance pendência inalterada. Pendência **P-Gradient-CMYK-ICC**
permanece formal aberta com **trabalho prévio externo identificado**
(3 itens em `typst-passo-273-14-trabalho-previo-externo.md`).
Sub-padrão emergente inaugural **"Scope-out reconfirmado por Fase A"
N=0 → N=1** — passo executado até critério binário; NO-GO honesto
é output legítimo per ADR-0054 graded; trabalho de diagnóstico
preserved como documento de pendência + pré-requisitos. Distingue
de "Bug arquitectural intencional corrigido" P273.12 (limitação
fechável vs dependente de trabalho externo). Zero alterações código
L1/L3. "Anotação cumulativa em vez de ADR nova" N=20 → N=21 (décima
quarta anotação consecutiva ADR-0091); "Sub-passos consecutivos do
mesmo cluster" N=9 → N=10 cumulativo emergente. Ver ADR-0091
§"Anotação cumulativa P273.14".

**Anotação P273.15**: Bbox medido pós-layout — **SCOPE-OUT-RECONFIRMED**
via NO-GO Fase A factual. Razão quádrupla: (1) §A.1 confirma **zero
demanda empírica** registada em 8 sub-passos consecutivos
(P273.6-P273.13; grep verificação em 20 documentos
`00_nucleo/`); (2) Caminho 1 (eager) custo perf inaceitável —
`measure_content_constrained` em todos Blocks sem dimensions, pior
caso O(N²) onde antes era O(N); (3) Caminho 2 (lazy) custo impl
desproporcional (walker novo ~60-100 LOC + manutenção sem demanda);
(4) 3γ.2.γ aceito por ADR-0054 graded — "menor mudança suficiente"
preserved. Decisão P273.6 §A.3 (3γ.2.γ) preserved literal — 8
sub-passos sem contraproba. Block sem dimensions continua a cair no
fallback page_bbox L3 P273.5 (identity transform; comportamento
aceito). Pendência **P273.X-bis-bbox-medido-pos-layout** permanece
formal aberta com **2 pré-requisitos identificados** (em
`typst-passo-273-15-trabalho-previo-externo.md`): caso empírico
concreto + decisão executiva custo perf. Sub-padrão "Scope-out
reconfirmado por Fase A" N=1 → N=2 cumulativo emergente — primeira
reaplicação do padrão inaugurado P273.14; mecânica consolidada
(Fase A factual + decisão binária + NO-GO output legítimo + zero
código). Distingue de P273.14 (constraints externas: licensing +
crate) por NO-GO ser por **ausência de demanda + custo perf**. Zero
alterações código L1/L3. "Anotação cumulativa em vez de ADR nova"
N=21 → N=22 (décima quinta anotação consecutiva ADR-0091);
"Sub-passos consecutivos do mesmo cluster" N=10 → N=11 cumulativo
emergente — folga máxima sobre limiar formalização N=3-4 preservada.
Ver ADR-0091 §"Anotação cumulativa P273.15".

**Anotação P273.16**: Bbox.y topo-exacto inline —
**SCOPE-OUT-RECONFIRMED** via NO-GO Fase A factual; **terceira
aplicação consolidando padrão "Scope-out reconfirmado por Fase A"
crossing limiar formalização ADR meta N=3-4**. Descoberta empírica
importante: spec premissa "DEBT-56 EM ABERTO" factualmente
desactualizada — verificação literal em `DEBT.md:535` confirma
DEBT-56 ENCERRADO P221 (2026-05-12). Fase A empírica prevalece
sobre premissa da spec — cumprimento honesto critério "verificar
empíricamente". Razão NO-GO quádrupla: (1) zero demanda empírica
em 9 sub-passos consecutivos (P273.7-P273.15); (2) Caminho 1
refactor inline line_height fora de escopo P273.16 (magnitude L+
vs S-M cluster Gradient; Fase 4 multi-region scope-out per
ADR-0078 §"Decisão" sub-fase (b)); (3) Caminho 2 font_metrics
ad-hoc cria dívida invisível sem demanda; (4) 3γ.2.γ-inline-baseline-y
P273.7 aceito por ADR-0054 graded + coerente com **P156H
limitação consciente** preserved literal. Bloqueador real
identificado (substituindo DEBT-56 fechado): P156H limitação
consciente + ADR-0078 §"Decisão" sub-fase (b) Fase 4 scope-out.
Decisão P273.7 §A.3 (3γ.2.γ-inline-baseline-y) preserved literal —
9 sub-passos sem contraproba. Pendência
**P273.X-bis2-bbox-y-topo-exacto-inline** permanece formal aberta
com 3 pré-requisitos identificados. Pendência candidata XS nova
**P273.X-bis-content-md-debt56-update** descoberta empíricamente
(L0 referência DEBT-56 fechado desactualizada; NÃO reservado).
Sub-padrão "Scope-out reconfirmado por Fase A" N=2 → N=3
cumulativo **crossing limiar formalização ADR meta N=3-4** —
terceira aplicação consolida com 3 razões NO-GO distintas
(externa P273.14 + interna P273.15 + estrutural aceita P273.16).
Candidato meta-ADR formalização NÃO reservado. **Cluster Gradient
declarável feature-complete pós-P273.16** — sequência "terminar
cluster Gradient" esgotada com 12 sub-passos consecutivos
(P273.5-P273.16) — **caminho mais longo documentado no projecto
cristalino**. Zero alterações código L1/L3. "Anotação cumulativa
em vez de ADR nova" N=22 → N=23 (décima sexta anotação consecutiva
ADR-0091); "Sub-passos consecutivos do mesmo cluster" N=11 → N=12
cumulativo emergente. Ver ADR-0091 §"Anotação cumulativa P273.16".

**Anotação P273.17**: Passo administrativo S+ — reflexão metodológica
formal cluster Gradient + 3 ADRs meta novas EM VIGOR. **ADRs criadas
directamente** (paridade P271 ADR-0093/0094): **ADR-0095** "Dedup
`Arc::as_ptr` resources" (N=3 cumulativo: P73 image + P263 pattern
+ P273.12 bbox-aware); **ADR-0096** "Pattern DEBT-37 campo Layouter
consumer-pending" (N=4 com folga: P84.6 Grid + P273.5 + P273.6 +
P273.9); **ADR-0097** "Scope-out reconfirmado por Fase A" (N=3 com
3 razões NO-GO distintas: P273.14 externa + P273.15 interna +
P273.16 estrutural aceita). **Documento reflexão** standalone
`typst-cluster-gradient-reflexao.md` com 10 secções (trajectória +
sub-padrões + limiares + descobertas + pendências + trade-offs +
anti-padrões + reflexão final). **Cluster Gradient encerrado
definitivamente** — **13 sub-passos consecutivos P273.5-P273.17**
caminho mais longo de sub-passos consecutivos do mesmo cluster
documentado no projecto cristalino. ADRs vigentes 81 → 84 (+3 EM
VIGOR). Sub-padrões NÃO formalizados (preserved emergentes N=1-2):
L3-only parent_bbox, Template-passo replicado, Layout duplo, Extract
helper inline, Triplicação Group bbox, Bug arquitectural intencional,
Bug latent scope creep, Cleanup XS derivado — anti-padrão
over-formalização explícito. Sub-padrão meta-meta "Passo
administrativo XS/S criar ADRs meta" N=3 cumulativo (P156K + P271 +
P273.17) NÃO formalizado por mesmo anti-padrão. Zero alterações
código L1/L3 (ADR-0029 preserved absoluto). "Anotação cumulativa em
vez de ADR nova" N=22 → N=23 (décima sétima anotação consecutiva
ADR-0091); "Sub-passos consecutivos do mesmo cluster" N=12 → N=13
cumulativo emergente. Ver ADR-0091 §"Anotação cumulativa P273.17".
