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

---

## Anotação cumulativa P265 — PDF Radial shading complete materializado

**Data**: 2026-05-15.

PDF rendering real Gradient Radial materializado. Substitui
fallback `first_stop_color` introduzido em P264 (3 sítios
pattern-match adaptados). Status `IMPLEMENTADO` preservado
literal (paridade pattern P263 anotação ADR-0087 + ADR-0080
§"refactor aditivo").

### Componentes materializados em `03_infra/src/export.rs`

- **Tipo `GradientObjectKind` enum local** (novo P265):
  ```rust
  enum GradientObjectKind {
      Linear(Arc<Linear>),
      Radial(Arc<Radial>),
  }
  ```
  `GradientObject.linear: Arc<Linear>` (P263) generalizado para
  `GradientObject.kind: GradientObjectKind`.
- **`compute_radial_coords(center, radius, w, h)`** — 6 valores
  Coords axes locais; círculos concêntricos (focal_* scope-out
  preservado per ADR-0088).
- **`oklab_sample_stops_radial(radial, n=16)`** — paridade
  literal `oklab_sample_stops` P263 (apenas tipo Radial em vez
  de Linear).
- **`emit_gradient_objects` expandido** com branching match
  `GradientObjectKind::Linear` / `Radial`:
  - Linear: emit `/ShadingType 2 /Coords [x0 y0 x1 y1] /Extend
    [false false]`.
  - Radial: emit `/ShadingType 3 /Coords [x0 y0 r0 x1 y1 r1]
    /Extend [true true]` (paridade vanilla default radial).
- **`emit_stroke_paint` branch unificado** — pattern lookup
  funciona idêntico para Linear/Radial (chave `Arc::as_ptr`
  genérica); fallback Solid P264 substituído.
- **3 sítios pattern-match P264** substituídos:
  - `scan_all_gradients`: Linear+Radial ambos registados via
    enum kind.
  - `pattern_resources_for_page`: ambos emit resource entry.
  - `emit_stroke_paint`: lookup unificado (`Arc::as_ptr` Linear
    OU Radial).

### Reutilização literal de P263 (sem alteração)

- **`emit_function_dict`** (Type 2 / Type 3 stitching) — idêntico.
- **`emit_pattern_dict`** inline em `emit_gradient_objects` —
  idêntico (PatternType 2 wrapper).
- **`pattern_resources: HashMap<usize, PatternRef>`** —
  estrutura genérica preservada; chave `Arc::as_ptr` funciona
  para Linear OU Radial.
- **3 paths `build_helvetica/cidfont/multifont`** — sem
  modificação (branching unificado via `emit_stroke_paint`).
- **`Radial::sample(t)`** Oklab L1 (P264) — reutilizado em
  `oklab_sample_stops_radial`.

### Paridade observable cumprida pós-P265

`#gradient.radial(red, blue, center: (50%, 50%), radius: 60%)`
em Stroke renderiza radial real no PDF — círculos concêntricos
center → borda via `/ShadingType 3` + Function Type 3 stitching
(16 stops Oklab pre-sampled). Fallback Solid pré-P265
eliminado.

### Decisões D1-D5 P265.A

- **D1 Generalização sample_stops**: ☑ Opção α duplicação
  explícita (`oklab_sample_stops_radial` paridade literal).
- **D2 compute_radial_coords**: 6 valores; círculos concêntricos
  (foco pontual no center, target radius); subset materializado
  P264.
- **D3 Pattern_resources chave**: `Arc::as_ptr` genérico
  preservado paridade Linear.
- **D4 Cross-path cobertura**: branching unificado em
  `emit_stroke_paint` (helper P263 ganha branch Radial);
  zero modificação em 3 paths build_page_stream_*.
- **D5 Function reutilizada**: idêntico — `emit_function_dict`
  aceita stops genéricos (r, g, b).

### Tests adicionais P265 (+7 cumulativos)

**Unit helpers** (4):
- `p265_compute_radial_coords_center_default`.
- `p265_compute_radial_coords_center_offset`.
- `p265_compute_radial_coords_non_square_uses_min_dim`.
- `p265_oklab_sample_stops_radial_red_blue_endpoints`.

**E2E PDF** (3):
- `p265_export_pdf_radial_emits_shading_type_3` — confirma
  `/ShadingType 3`, `/PatternType 2`, `/FunctionType`, `/Coords`,
  `/Extend [true true]`, `/Pattern <<`, `SCN`.
- `p265_export_pdf_radial_dedup_arc_ptr` — 3 shapes com mesmo
  Arc<Radial> → 1 Shading dedup.
- `p265_export_pdf_linear_e_radial_coexistem` — Linear + Radial
  no mesmo doc emit ShadingType 2 + ShadingType 3 distintos.

### Cobertura Visualize agregada

- Pre-P265: ~68% (P264 Radial L1+stdlib +5pp).
- **Pós-P265: ~73%** (+5pp via PDF Radial real; F.2 promovido
  `implementado+stdlib` → `implementado+stdlib+render` paridade
  Linear).

### Scope-outs preservados

- `focal_center`/`focal_radius` → P-Gradient-Focal futuro.
- Conic continua comentário reserva em
  `entities/gradient.rs` → P-Gradient-Conic dedicado.
- `/ShadingType 1, 4-7` fora de scope.
- `draw_item_local` continua fallback Solid (scope-out P263
  preservado).

### Subpadrões cumulativos pós-P265

- **"P262/P263 dividir granularidade" N=2 → N=3 cumulativo
  atinge limiar formalização clara** (P262/P263 Linear +
  P264/P265 Radial — cluster Gradient completa duas divisões).
- **"Reutilização literal de helpers cross-passos" N=1
  inaugurado** — P265 reutiliza 4 helpers P263 inalterados
  + Radial::sample L1 P264; **~70% código L3 P265 é wiring
  + helpers específicos** (compute_radial_coords,
  oklab_sample_stops_radial, branching emit_gradient_objects).
- **"Anotação cumulativa em vez de ADR nova" reaplicada** —
  paridade pattern P263 anotação ADR-0087.
- **Status `IMPLEMENTADO` ADR-0088 preservado** — anotação
  cumulativa não muda status; refina aplicação (paridade
  ADR-0080 §"refactor aditivo").

Cross-references:
- L3 emit: `03_infra/src/export.rs` (~100-150 LoC novas;
  enum local `GradientObjectKind`; 3 helpers; expand
  emit_gradient_objects branching).
- L0 prompt: `00_nucleo/prompts/infra/export.md` secção
  "Suporte Gradient Radial via Shading Patterns (Passo 265)".
- Tests E2E: confirmam `/ShadingType 3`, `/Coords` 6 valores,
  `/Extend [true true]`, dedup `Arc::as_ptr<Radial>`,
  coexistência Linear + Radial.
- ADR-0087 anotação P263 (precedente directo template).
- P263 (template literal — paridade quase 1-para-1).
- P264 (origem da promessa fechada por este passo).
- P73 (template arquitectural `image_resources` dedup
  `Arc::as_ptr` cumulativamente N=2 — P263+P265 ambos
  aplicam).
