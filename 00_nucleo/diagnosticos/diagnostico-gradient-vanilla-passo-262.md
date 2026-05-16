# Diagnóstico Gradient vanilla — Passo 262 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável (**primeiro consumo directo** pós-P260) +
ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-262.md` (spec).
**Análogo estrutural**: `diagnostico-color-vanilla-passo-257.md`
(P257) + `diagnostico-paint-vanilla-passo-261.md` (P261).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0085 §"Propriedades obrigatórias".

---

## §1 — Estrutura literal vanilla Gradient

### §1.1 — Enum Gradient (vanilla `gradient.rs:178`)

```rust
#[ty(scope, cast)]
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Gradient {
    Linear(Arc<LinearGradient>),
    Radial(Arc<RadialGradient>),
    Conic(Arc<ConicGradient>),
}
```

**3 variants**: `Linear` + `Radial` + `Conic`. **`Arc<T>`
wrapper** em cada variant — clone O(1) preservado.

### §1.2 — LinearGradient (vanilla `gradient.rs:1001`)

```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LinearGradient {
    pub stops: Vec<(Color, Ratio)>,      // tuples — não GradientStop directo
    pub angle: Angle,
    pub space: ColorSpace,               // default Oklab
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

**Campo `stops: Vec<(Color, Ratio)>`** — tuple (`Color`,
`Ratio`); NÃO usa `GradientStop` directo. `GradientStop` é
usado apenas para cast/parsing de input (com Option<Ratio>).

### §1.3 — GradientStop (vanilla `gradient.rs:1217`)

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GradientStop {
    pub color: Color,
    pub offset: Option<Ratio>,           // Option permite auto-spacing
}

impl GradientStop {
    pub fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset: Some(offset) }
    }
}

cast! {
    GradientStop,
    color: Color => Self { color, offset: None },
    array: Array => { ... cast (Color, Ratio) ... }
}
```

**`offset: Option<Ratio>`** — `None` = espaçamento automático
(uniformemente distribuído entre stops adjacentes com offset
explícito ou nos extremos).

### §1.4 — Métodos públicos Gradient

```rust
impl Gradient {
    pub fn linear(args: &mut Args, ...) -> SourceResult<Gradient>;
    pub fn radial(...) -> SourceResult<Gradient>;
    pub fn conic(...) -> SourceResult<Gradient>;
    pub fn sample(...) -> Color;
    pub fn space(&self) -> ColorSpace;
    pub fn relative(&self) -> Smart<RelativeTo>;
    pub fn angle(&self) -> Option<Angle>;
    pub fn stops(&self) -> Vec<GradientStop>;
    // + outros métodos auxiliares
}
```

### §1.5 — RelativeTo enum (vanilla `gradient.rs:1211`)

```rust
pub enum RelativeTo {
    Self_,
    Parent,
}
```

Default: `Smart::Auto` (resolve em layout).

---

## §2 — Consumers cristalino (impacto P262)

### §2.1 — Stroke.paint (P261)

```bash
$ grep -rn "Paint::Solid\|paint: Paint\|stroke.paint" \
  01_core/src/ 03_infra/src/ | head -10
```

- `Stroke.paint: Paint` (P261) — already wraps via Paint enum.
- **Sem refactor cascade adicional P262** — `Paint::Gradient(g)`
  activa via `impl From<Gradient> for Paint` (descomentar +
  add From).

### §2.2 — PDF exporter consumers actuais

```bash
$ grep -n "s.paint.to_color()" 03_infra/src/export.rs
863, 1125, 1371, 1553  (4 sítios P261)
```

**Adaptação P262**: cada sítio precisa branch:
- `Paint::Solid(c)` → emit `c.to_srgb()` + `rg` PDF operator
  (literal preservado).
- `Paint::Gradient(g)` → emit `/Pattern cs` + `/P<id> scn` PDF
  operators; pattern referenciado em `/Resources /Pattern`.

### §2.3 — Value::Gradient activação

```bash
$ grep -n "Gradient" 01_core/src/entities/value.rs
83: // Gradient(Gradient),       // gradiente  (comentado pré-P262)
```

**Activação P262**: descomenta linha 83 + adiciona
`Value::Gradient(Gradient)` variant + `type_name() => "gradient"`
+ `impl From<Gradient> for Value`.

### §2.4 — Color::to_rgba_f32 reutilizado

`Color::to_rgba_f32()` (color.rs:204) converte Oklab → linear
RGB → sRGB. **Reutilizado em P262** para amostragem PDF:
- Interpolação L1 em Oklab (paridade vanilla).
- Amostragem N pontos uniformemente.
- Conversão amostras → sRGB via `to_rgba_f32()`.
- Emit PDF Function Type 3 stitching com Type 2 segments
  (linear interpolation em sRGB entre amostras).

---

## §3 — PDF shading pattern arquitectura

### §3.1 — Estrutura PDF objects novos

PDF 32000 §8.7 Shading Patterns:
- **`/ShadingType 2`** (axial — linear gradient).
- **`/ColorSpace /DeviceRGB`** (sRGB; Oklab interpolation
  acontece L1 antes amostragem).
- **`/Coords [x0 y0 x1 y1]`** — endpoints axis (em local
  coordinates do shape).
- **`/Function`** — sub-objecto interpolação stops:
  - Function Type 3 (stitching) — combinar N Function Type 2.
  - Function Type 2 (exponential) com `/N 1.0` (linear) para
    cada par sucessivo de stops.
- **`/Extend [false false]`** ou `[true true]` (clamp/repeat
  outside `[0, 1]`).

### §3.2 — Pattern Dict

PDF 32000 §8.7.2:
- **`/PatternType 2`** (shading pattern).
- **`/Shading <obj-id>`**.
- **`/Matrix [a b c d e f]`** — opcional; identidade default.

### §3.3 — Cálculo Coords a partir de Angle + bbox

Generalização (decisão local L3 — exporter conhece bbox):

```text
angle 0° (rad) → coords horizontais left→right
angle π/2 (90°) → coords verticais bottom→top
angle θ generic:
  cx, cy = centro da bbox
  half_diagonal = sqrt(w² + h²) / 2
  dx = half_diagonal * cos(θ)
  dy = half_diagonal * sin(θ)
  x0, y0 = cx - dx, cy - dy
  x1, y1 = cx + dx, cy + dy
```

**Decisão Fase A.3**: ☑ **L3 (exporter)** — exporter conhece
bbox real do shape; L1 fica puro em representação angular.

Helper privado `compute_axial_coords(angle, bbox) -> [f64; 4]`
em `03_infra/src/export.rs`.

### §3.4 — Dedup gradients

Paridade pattern image P73 (`Arc::as_ptr` dedup):
- `pattern_resources: HashMap<usize, (PatternResource, ObjectId)>`
- Key: `Arc::as_ptr(linear)` para Gradient::Linear.
- Reuso entre múltiplos shapes que partilham a mesma instância
  Arc<LinearGradient>.

### §3.5 — Resources /Pattern dict

```text
/Resources <<
    /Pattern <<
        /P1 <obj-id-1>
        /P2 <obj-id-2>
        ...
    >>
    /ColorSpace ...
    /Font ...
>>
```

`/P<n>` resource names allocated per pattern dedupado.

---

## §4 — Decisão forma cristalina

### §4.1 — Estrutura proposta P262

Per user decisions (P262 pre-flight Q1+Q2+Q3):

```rust
// 01_core/src/entities/gradient.rs

use std::sync::Arc;
use crate::entities::color::Color;
use crate::entities::layout_types::{Angle, Ratio};

/// Sub-componente per ADR-0029 §exclusões.
///
/// `offset: Option<Ratio>` per **decisão user P262 Q3**:
/// paridade vanilla com auto-spacing quando offset = None.
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

/// Linear gradient — paridade vanilla `LinearGradient`.
///
/// **ColorSpace fixo Oklab** per decisão user P262 Q2 (paridade
/// vanilla default). Interpolação em Oklab acontece L1 via
/// helper `sample(t)`; amostras sRGB para PDF emit.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    // space: ColorSpace,     // scope-out ADR-0087 — Oklab fixo
    // relative: RelativeTo,  // scope-out ADR-0087 — bbox-relative
    // anti_alias: bool,      // scope-out ADR-0087 — assume true
}

/// Gradient — enum tagged paridade vanilla.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    // Radial(Arc<Radial>),  // P-Gradient-Radial — comentário reserva
    // Conic(Arc<Conic>),    // P-Gradient-Conic — comentário reserva
}
```

### §4.2 — Decisão GradientStop sub-componente

☑ **Opção β (inline)** — GradientStop é literalmente
sub-componente Gradient per ADR-0029 §exclusões. Análogo a
WeightedColor/Cmyk vanilla (sub-components em ficheiro principal).

### §4.3 — Decisão Coords L1 vs L3

☑ **L3 (exporter)** — helper `compute_axial_coords(angle, bbox)`
em `03_infra/src/export.rs`. L1 fica puro em representação
angular.

### §4.4 — Scope-outs documentados

| Scope-out | Razão | Resolução prevista |
|-----------|-------|---------------------|
| `Gradient::Radial` | Sem consumer real P262 | P-Gradient-Radial dedicado |
| `Gradient::Conic` | Baixa prioridade | P-Gradient-Conic dedicado |
| `LinearGradient.space: ColorSpace` | Oklab fixo (paridade vanilla default) | Refino futuro se sRGB explícito for prioritário |
| `LinearGradient.relative: Smart<RelativeTo>` | bbox-relative (vanilla default) | "self-relative" diferido P-Gradient-Relative |
| `LinearGradient.anti_alias` | Assume true (PDF default) | Refino futuro se controlo necessário |
| `Gradient::sample()` user-facing | API auxiliar vanilla | Futuro se consumer real exigir |

### §4.5 — Stops auto-spacing logic

Algoritmo per user decision Q3:

```text
Dados N stops com offsets opcionais:
1. Identificar runs consecutivos de offset = None.
2. Cada run é delimitado por offset explícito (ou extremos
   implícitos 0% / 100%).
3. Interpolar offsets em [prev_offset, next_offset] distribuição
   uniforme.

Exemplo: [(red, None), (green, None), (blue, None)]
→ red @ 0%, green @ 50%, blue @ 100%.

Exemplo: [(red, None), (green, Some(70%)), (blue, None)]
→ red @ 0%, green @ 70%, blue @ 100%.
```

Implementação `Linear::effective_offsets(&self) -> Vec<f32>`
em L1 (puro; sem I/O).

---

## §5 — Validações stdlib `native_gradient_linear`

```rust
pub fn native_gradient_linear(args: &Args) -> SourceResult<Value> {
    // Stops: variadic positional (paridade vanilla #[variadic])
    // Aceita:
    //   - Color directo (offset = None)
    //   - Array de 2: [Color, Ratio]
    //   - Dict { color, offset }  (futuro)
    //
    // Validações:
    //   - Pelo menos 1 stop.
    //   - Offsets explícitos em [0%, 100%].
    //   - Offsets explícitos ordenados (warning se não).
    //
    // Named:
    //   - angle: Angle  (default Angle::deg(0))
    //
    // Returns: Value::Gradient(Gradient::linear(stops, angle)).
}
```

---

## §6 — Plano materialização P262.C

### Sequência sub-passos

1. **C.1** — L0 prompt `entities/gradient.md` novo.
2. **C.2** — `entities/gradient.rs` (tests primeiro + impl —
   GradientStop + Linear + Gradient + sample(t) em Oklab +
   effective_offsets).
3. **C.3** — `entities/mod.rs` re-export.
4. **C.4** — `entities/paint.rs` activar `Paint::Gradient(Gradient)`
   variant; `Copy` removido (Arc não é Copy); add `From<Gradient>
   for Paint`; ajustar `Paint::to_color()` para Gradient fallback.
5. **C.5** — `entities/value.rs` activar `Value::Gradient(Gradient)`
   variant + `type_name()` + `From<Gradient> for Value`.
6. **C.6** — Stdlib `native_gradient_linear` em novo
   `01_core/src/rules/stdlib/gradients.rs` + registo em
   `rules/eval/mod.rs` (scope.define("gradient", ...)).
7. **C.7** — PDF exporter:
   - Helper `compute_axial_coords(angle, bbox)`.
   - Helper `sample_gradient_to_srgb_stops(gradient, n)` — amostra
     N pontos em Oklab, converte para sRGB.
   - Emit `/Function Type 3` stitching + Type 2 segments.
   - Emit `/Shading` + `/Pattern` resources.
   - Adaptar 4 sítios `s.paint.to_color().to_rgba_f32()`
     branching Solid vs Gradient.
   - Dedup via `Arc::as_ptr(linear)`.

### Magnitude esperada

- L1 gradient.rs: ~150-200 LoC + 10-15 tests.
- L1 paint.rs + value.rs activações: ~30 LoC + 2-3 tests.
- L2 stdlib gradients.rs: ~80-120 LoC + 5-7 tests.
- L3 PDF shading: ~200-300 LoC + 5-8 tests E2E.
- **Total magnitude M-M+** (~3-5h).

---

## §7 — Limitações conscientes

- **Linear only** (Radial/Conic comentários reserva no enum).
- **ColorSpace fixo Oklab** (paridade vanilla default;
  scope-out per ADR-0087).
- **Relative assume bbox-local** ("self-relative" scope-out).
- **Anti-alias assume true** (PDF default).
- **Sem `Gradient::sample()` user-facing** (futuro consumer real).
- **`Paint::to_color()` Gradient fallback**: primeiro stop (não
  mid-gradient sample) — uso documentado como "Solid fallback;
  Gradient renderiza via L3 shading separado".

---

## §8 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório.
- ADR-0033 — Paridade observable vanilla.
- ADR-0083 — Color paridade vanilla (precedente N=2 do pattern).
- **ADR-0085** — Diagnóstico imutável (**primeiro consumo
  directo** P262).
- **ADR-0086** §"Critério revisão" — aponta literal para este
  passo.
- **ADR-0087** — Gradient Linear-only (a criar P262.B).
- ADR-0027 — PDF objects estrutura (precedente shading).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded.
- ADR-0065 — Inventariar primeiro (cumprido aqui).
- P257 — Color paridade vanilla 8/8.
- P259 §3 Opção 1 sub-passo 2 — spec preliminar deste passo.
- P260 — ADRs meta (ADR-0085 consumido directamente).
- P261 — Paint wrapper Solid only (pré-requisito; ADR-0086).
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (1366 linhas; 3 variants Linear/Radial/Conic;
  LinearGradient com 5 campos; GradientStop sub-componente
  com Option<Ratio>; RelativeTo enum).
