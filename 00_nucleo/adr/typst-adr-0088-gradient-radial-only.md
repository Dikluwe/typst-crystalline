# ⚖️ ADR-0088: Gradient Radial materializado; Conic scope-out preservado

**Status**: `IMPLEMENTADO`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 264.B (criação PROPOSTO) → Passo 264.D
(promoção `IMPLEMENTADO` pós-materialização L1+stdlib;
PDF shading scope-out para P265 dedicado paridade P262/P263).
**Aplicação**:
`00_nucleo/materialization/typst-passo-264-relatorio.md`.
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
(imutável per ADR-0085 — **segundo consumo directo** pós-P262).
**Análogo estrutural directo**: ADR-0087 (Gradient Linear-only;
N=3 do pattern PROPOSTO+IMPLEMENTADO mesmo passo; este ADR é
N=4 cumulativo).

---

## Contexto

Visualize vanilla define:

```rust
// lab/typst-original/.../visualize/gradient.rs:1063
pub struct RadialGradient {
    pub stops: Vec<(Color, Ratio)>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,
    pub focal_radius: Ratio,
    pub space: ColorSpace,
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

8 campos vanilla. P262 materializou `Gradient::Linear`; P263
materializou PDF shading Linear. ADR-0087 §"Critério revisão"
explicitamente apontou para activação `Gradient::Radial`
variant. **P264 cumpre parcialmente** (Conic continua scope-out;
PDF emit Radial adiado P265).

P259 Cenário B2 Opção 1 sub-passo 2 (extensão) → spec preliminar
deste passo. P264 executa Gradient Radial L1+stdlib; P265
candidato L3 PDF shading Radial dedicado (`/ShadingType 3`).

ADR-0029 §"Simplificações aceites apenas com ADR explícita"
obriga ADR para subset materializado vs vanilla full. Paridade
pattern N=3 → **N=4** cumulativo com ADR-0083 (Color) + ADR-0086
(Paint) + ADR-0087 (Gradient Linear).

---

## Decisão

### Subset materializado P264 — Radial 3 campos

```rust
// 01_core/src/entities/gradient.rs (delta sobre P262)

use crate::entities::axes::Axes;
use crate::entities::layout_types::Ratio;

#[derive(Debug, Clone, PartialEq)]
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    // focal_center: Axes<Ratio>,   // scope-out — default = center
    // focal_radius: Ratio,         // scope-out — default 0%
    // space: ColorSpace,            // scope-out — Oklab fixo
    // relative: Smart<RelativeTo>,  // scope-out — bbox-relative
    // anti_alias: bool,             // scope-out — true assumed
}

pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),     // P264 — descomentado
    // Conic(Arc<Conic>),     // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn radial(
        stops: impl Into<Arc<[GradientStop]>>,
        center: Axes<Ratio>,
        radius: Ratio,
    ) -> Self;
}

impl Radial {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear
    pub fn sample(&self, t: f32) -> Color;        // paridade Linear (mesma Oklab interp)
}
```

**Subset 3 campos** (stops + center + radius). Magnitude
controlada.

### Tipo novo `Axes<T>` minimal

```rust
// 01_core/src/entities/axes.rs (novo P264)

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axes<T> {
    pub x: T,
    pub y: T,
}

impl<T> Axes<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
}
```

L0 prompt `entities/axes.md` novo. Re-export em `entities/mod.rs`.

### Stdlib `native_gradient_radial`

```rust
// 01_core/src/rules/stdlib/gradients.rs

pub fn native_gradient_radial(args, ...) -> SourceResult<Value>;
```

- Variadic positional stops (paridade `native_gradient_linear`).
- Named: `center: Axes<Ratio>` (default `(50%, 50%)`); `radius:
  Ratio` (default 50%).
- Auto-spacing via `effective_offsets` paridade Linear.

`make_gradient_module()` ganha entrada `radial`. User-facing:
`#gradient.radial(red, blue, center: (40%, 60%), radius: 70%)`.

### Activação Gradient::Radial em consumers existentes

`Paint::Gradient(Gradient)` (P261/P262) e
`Value::Gradient(Gradient)` (P262) são enum wrappers
**indiferentes a variant interno**. Aceitam Radial
automaticamente. **Zero cascade refactor**.

`Gradient::first_stop_color()` pattern-match expande para cobrir
Radial:

```rust
pub fn first_stop_color(&self) -> Color {
    match self {
        Gradient::Linear(l) => l.stops.first().map(|s| s.color)
            .unwrap_or(Color::rgb(0, 0, 0)),
        Gradient::Radial(r) => r.stops.first().map(|s| s.color)
            .unwrap_or(Color::rgb(0, 0, 0)),
    }
}
```

PDF exporter (P263) pattern-match em
`scan_all_gradients` / `pattern_resources_for_page` /
`emit_stroke_paint`: 3 sítios `let Gradient::Linear(l) = g`
expandem para tratar Radial. **Decisão P264**: Radial fallback
Solid no PDF (igual P262 pré-P263 state Linear) — PDF shading
Radial completo (`/ShadingType 3`) adiado para **P265**.

### Preservações arquitecturais

- **ADR-0039 SR-Struct Resolvido**: `TextStyle.fill:
  Option<Color>` **inalterado** literal. P264 não migra
  TextStyle.fill para Option<Paint>.
- **ADR-0086 Paint wrapper Solid only**: status `IMPLEMENTADO`
  preservado; `Paint::Gradient(Gradient)` activa P262 cobre
  Radial.
- **ADR-0087 Gradient Linear-only**: status `IMPLEMENTADO`
  preservado; §"Critério revisão" cumprido parcialmente (Radial
  activado; Conic continua scope-out).
- **DEBT-1** (fechado P142): preservado.

### Scope-outs documentados

| Scope-out | Razão | Resolução prevista |
|-----------|-------|---------------------|
| `Gradient::Conic(Conic)` | Baixa prioridade; nenhum consumer real | **P-Gradient-Conic** dedicado futuro |
| `RadialGradient.focal_center: Axes<Ratio>` | Default = center; consumer raro | Refino futuro se Gradient focal real exigir |
| `RadialGradient.focal_radius: Ratio` | Default 0%; consumer raro | Refino futuro |
| `RadialGradient.space: ColorSpace` | Oklab fixo (paridade ADR-0087) | Refino futuro |
| `RadialGradient.relative: Smart<RelativeTo>` | bbox-relative (paridade ADR-0087) | "self-relative" diferido |
| `RadialGradient.anti_alias` | true assumed (PDF default) | Refino se controlo necessário |
| **PDF Radial shading `/ShadingType 3`** | Pattern P262/P263 dividir granularidade N=2 | **P265 dedicado** (S-M; replica P263 template) |
| `Gradient::sample()` user-facing Radial | API auxiliar vanilla | Futuro se consumer real exigir |

---

## Consequências

### Positivas

- **User-facing `gradient.radial(...)` funcional** parcialmente
  (parsing + L1; PDF render Solid fallback até P265).
- **Activa `Gradient::Radial` variant** (ADR-0087 §"Critério
  revisão" cumprido parcialmente).
- **Zero cascade refactor consumers** — Paint/Value já
  preparados P261/P262.
- **Cobertura Visualize** +5pp estructural (F.2 Radial ausente
  → implementado L1+stdlib).
- **Helpers Oklab reutilizados** literal de P262 (zero código
  duplicado).

### Negativas

- **PDF render Radial inicialmente fallback Solid** — paridade
  pré-P263 state Linear; P265 fecha promessa.
- **Magnitude controlada M-** (~2h) — minimal extension P262.

### Neutras

- **Variants `Conic` (comentários)** continuam roadmap visual.
- **`focal_*` scope-out** preservado per default vanilla
  (raramente usado).
- **Axes<T> minimal criado** — reutilizável futuro
  (e.g. `Axes<Length>`).

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| α1 — Radial completo (com focal_*) | Paridade vanilla 100% | Magnitude M+ vs M-; raramente usado user-facing |
| **α2 — Radial subset (escolhida)** | **Magnitude controlada; helpers L1 reutilizados** | **focal_* scope-out (refino futuro)** |
| α3 — Linear+Radial unificados sem `Radial` struct | -1 struct | Paridade pattern Linear quebrada; inconsistência |
| β — Anotação ADR-0087 sem ADR nova | -1 ADR | Mistura âmbitos (Linear scope-outs distintos de Radial scope-outs) |

**Decisão**: **α2 (Radial subset) + Opção α (ADR-0088 nova)**
per paridade ADR-0083 + ADR-0086 + ADR-0087.

---

## Critério revisão

ADR-0088 transita `IMPLEMENTADO` → expansão real quando:

1. **P265 PDF Radial shading** materializa `/ShadingType 3`
   (radial axial); substitui fallback `first_stop_color` por
   render real (paridade pattern P262 → P263).
2. **P-Gradient-Conic** materializa `Conic` struct + activa
   `Gradient::Conic(Arc<Conic>)` variant.
3. **P-Gradient-Focal** materializa focal_center/focal_radius
   campos (revoga scope-out).

Cada activação é **passo dedicado pequeno** (XS-M) per pattern
P262+; sem DEBT novo per política P158.

---

## Subpadrão "ADR PROPOSTO+IMPLEMENTADO no mesmo passo via Cenário B1/B2"

Cumulativo:
- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- N=3 P262 (ADR-0087 Gradient Linear-only).
- **N=4 P264** (ADR-0088 Gradient Radial-only; este passo).

**Patamar N=4 excede limiar formalização clara**. Candidato a
meta-ADR — **improvável e desnecessário** (padrão
auto-documentado em cada ADR individual).

---

## Subpadrão "P262/P263 dividir granularidade L1+stdlib / L3" N=1 → N=2

Cumulativo:
- N=1 P262 (Linear L1+stdlib) → P263 (Linear PDF).
- **N=2 P264** (Radial L1+stdlib) → P265 (Radial PDF; futuro).

**Patamar N=2 reforça pattern**. Próxima aplicação candidata:
P-Gradient-Conic L1+stdlib + L3 PDF se materializar.

---

## Subpadrão "Decisão minimalista (subset materializado)" N=3 → N=4

Cumulativo:
- N=1 P257 Color (8/8 + 4 scope-outs).
- N=2 P261 Paint (Solid only).
- N=3 P262 Gradient Linear only.
- **N=4 P264 Gradient Radial subset** (3 campos materializados;
  5 scope-outs).

**Pattern emergente sólido** confirma. Cada tipo wrapper
materializa subset minimal + comentários reserva activáveis em
passos dedicados futuros.

---

## Referências

- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (regra principal cumprida).
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico.
- **ADR-0083** — Color paridade vanilla (precedente N=2 pattern).
- **ADR-0084, ADR-0085** — Auditoria condicional + diagnóstico
  imutável (P260; **segundo consumo directo** P264.A).
- **ADR-0086** — Paint wrapper (Solid only IMPLEMENTADO P261;
  `Paint::Gradient` activa P262 absorve Radial sem cascade).
- **ADR-0087** — Gradient Linear-only (IMPLEMENTADO P262 +
  anotação cumulativa P263 PDF; §"Critério revisão" cumprido
  parcialmente por este passo; **precedente directo N=3**).
- ADR-0027 — PDF objects estrutura (precedente P265 futuro
  `/ShadingType 3`).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded.
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P264/P265).
- ADR-0065 — Inventariar primeiro.
- DEBT-1 — Fechado P142 (preservado).
- Aplicações precedentes do pattern:
  - P252 — Stroke `overhang` (precedente N=1 cross-cutting).
  - P257 — Color 8/8 (precedente N=2 pattern PROPOSTO+IMPL).
  - P261 — Paint wrapper Solid only (precedente N=3).
  - **P262** — Gradient L1+stdlib (precedente directo N=4;
    template literal P264).
  - **P263** — Gradient Linear PDF (template literal P265
    futuro).
- P259 §3 Opção 1 sub-passo 2 — spec preliminar (extensão).
- P260 — ADRs meta.
- `00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
  — diagnóstico imutável P264.A (segundo consumo directo).
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (1366 linhas; RadialGradient §1063-1080).

---

## Próximos passos

1. P264.C executa materialização L1+stdlib (axes.rs + Radial
   struct + Gradient::Radial activado + stdlib).
2. P264.D promove ADR-0088 → `IMPLEMENTADO`.
3. **P265** (futuro) — PDF Radial shading `/ShadingType 3`
   (replica P263 template; ~200-250 LoC L3).
4. **P-Gradient-Conic** (futuro) — activa `Gradient::Conic`
   variant.
