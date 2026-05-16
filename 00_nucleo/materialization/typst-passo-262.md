# Passo 262 — Gradient Linear (activa Paint::Gradient; P259 Cenário B2 Opção 1 sub-passo 2)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada **M**
(M+ cap; ~3-4h se decisão minimalista Linear-only; estoiro
para M+ se PDF shading exporter for mais complexo).
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- ADR-0029 (EM VIGOR — obriga diagnóstico vanilla + ADR
  explícita para scope-outs).
- ADR-0033 (paridade observable).
- ADR-0034 (diagnóstico canónico — paralelo com ADR-0085
  para diagnósticos imutáveis).
- ADR-0054 (perfil graded — scope-outs Radial/Conic aceites).
- ADR-0065 (inventariar primeiro).
- **ADR-0083** (Color paridade vanilla — P257; modelo análogo).
- **ADR-0084** (auditoria condicional EM VIGOR P260 — não
  obrigatória aqui pois P262 é materialização, não audit).
- **ADR-0085** (diagnóstico imutável EM VIGOR P260 —
  diagnóstico vanilla Gradient cumpre forma análoga;
  **primeiro consumo directo** pós-P260).
- **ADR-0086** (Paint wrapper Solid only IMPLEMENTADO P261 —
  §"Critério revisão" aponta literalmente para este passo).
- ADR-0027 (CIDFont/Identity-H — PDF exporter; contexto).
- DEBT-1 (fechado P142; preservado).
- Relatórios precedentes: P257 (Color paridade — N=2 do
  pattern "ADR PROPOSTO+IMPLEMENTADO mesmo passo"), P261
  (Paint wrapper — N=3 "Refactor cross-cutting entity
  primitivo"; activa Gradient consumer).

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  (Fase A diagnóstico vanilla per ADR-0029 §"Diagnosticar
  primeiro"; imutável per ADR-0085 — primeiro consumo
  directo).
- ADR nova (provável **ADR-0087**) — "Gradient Linear materializado;
  Radial/Conic scope-out" per ADR-0029 §"Simplificações
  aceites apenas com ADR explícita".
- Prompts L0 novos:
  - `00_nucleo/prompts/entities/gradient.md` (estrutura
    análoga `entities/color.md`).
  - `00_nucleo/prompts/entities/gradient_stop.md` (sub-componente).
- Código L1 novo:
  - `01_core/src/entities/gradient.rs` (enum + Linear struct
    + tests).
  - `01_core/src/entities/gradient_stop.rs` ou inline em
    `gradient.rs` (decisão Fase A).
- Código L1 actualizado:
  - `01_core/src/entities/paint.rs` (descomenta
    `Gradient(Gradient)` variant + From<Gradient>).
  - `01_core/src/entities/value.rs` (adiciona
    `Value::Gradient(Gradient)`; per restrição "comentadas no
    código — NÃO implementar sem ADR" agora autorizada por
    ADR-0087).
- Stdlib nova: `native_gradient_linear(stops, angle: ?, ...)`
  em `01_core/src/rules/stdlib/shapes.rs` ou novo
  `stdlib/gradients.rs`.
- Código L3 actualizado: `03_infra/src/export.rs` ganha
  emit de shading pattern (`/Pattern`, `/Shading`,
  `/ShadingType 2` axial).
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-262-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Ordem: diagnóstico vanilla → ADR → prompts L0
   → fix-hashes → testes-primeiro → código L1 → código L3.
2. **Decisão minimalista declarada (paridade P25 → P257 →
   P261 pattern)** — Linear materializa; Radial/Conic como
   **comentários reserva** no enum, não unit placeholders.
   Expansão consumer-driven em passos futuros.
3. **ADR-0086 §"Critério revisão" cumprido** — `Paint::Gradient(Gradient)`
   variant activada (era comentário reserva).
4. **ADR-0029 §"Simplificações aceites apenas com ADR
   explícita"** — Linear-only obriga ADR-0087 com:
   - Diferença vanilla vs cristalino (3 variants → 1).
   - Custo semântico de Radial/Conic ausentes.
   - Critério revisão (passos específicos futuros).
5. **Ordem testes-primeiro** — para cada código novo: testes
   antes de implementação.
6. **`crystalline-lint .`** zero violations no fim do passo.
7. **Tests workspace** sem regressão (contagem ≥ baseline
   2341 pós-P261). Esperado **+15-20** (estimativa P259
   Opção 1 sub-passo 2).
8. **Materialization é leitura proibida por iniciativa
   própria**.
9. **Política "sem novas reservas"** preservada — variants
   Radial/Conic como comentários no enum são **roadmap
   visual**, não DEBT/ADR novo. Roadmap real fica em
   ADR-0087 §"Critério revisão".
10. **ADR-0039 preservado literal** — `TextStyle.fill: Option<Color>`
    NÃO migra para `Option<Paint>` neste passo (preservação
    P261 confirmada).

---

## §1 — Sub-passo P262.A: Fase A diagnóstico vanilla obrigatório

**Objectivo**: produzir inventário literal de Gradient vanilla
+ analisar impacto cross-cutting + decidir forma cristalina.

**Materialização**: zero código novo. Apenas leitura e
diagnóstico imutável per ADR-0085 (**primeiro consumo
directo** pós-P260).

### Acções obrigatórias

#### A.1 — Leitura literal vanilla

```bash
# Estrutura Gradient vanilla
view lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# Variants Gradient enum
grep -n "^\s*pub enum Gradient\|impl Gradient " \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# Struct Linear
grep -A 20 "^\s*pub struct Linear\b\|^\s*pub struct LinearGradient" \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# GradientStop
grep -A 10 "^\s*pub struct.*GradientStop\|^\s*pub struct Stop" \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# Conversões + métodos públicos
grep -n "From<\|impl.*for Gradient\|pub fn " \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs | head -30

# Stdlib registo
grep -rn "gradient\.linear\|fn linear" \
  lab/typst-original/crates/typst-library/src/visualize/ | head -10
```

**Output esperado**:
- `enum Gradient { Linear(LinearGradient), Radial(RadialGradient), Conic(ConicGradient) }`.
- `LinearGradient { stops: Vec<GradientStop>, angle: Angle, space: ColorSpace, relative, anti_alias }`.
- `GradientStop { color: Color, offset: Ratio }`.
- Funções `gradient.linear(...)`, `gradient.radial(...)`, etc.

#### A.2 — Consumers cristalino (impacto P262)

```bash
# Onde Paint::Solid é usado hoje (consumers que beneficiam
# de Gradient automático via From<Gradient>)
grep -rn "Paint::Solid\|paint: Paint::Solid\|: Paint\b" \
  01_core/src/ 03_infra/src/

# Stroke construções (post-P261 — usa Paint via From<Color>)
grep -rn "Stroke\s*{\|stroke.paint" 01_core/src/ 03_infra/src/

# PDF exporter actual — emit de fill operator
grep -n "rg\b\|RG\b\|s.paint.to_color" 03_infra/src/export.rs | head -20

# Value enum — variants comentadas/não implementadas
grep -n "Gradient" 01_core/src/entities/value.rs
```

**Output esperado**:
- `Stroke.paint: Paint` already (P261); From<Gradient>
  automático sem refactor cascade.
- PDF exporter ~4 sítios `s.paint.to_color().to_rgba_f32()`
  preservados para Solid; **branch novo** para Gradient.
- `Value::Gradient` actualmente comentado em `value.rs`
  (per ADR-0029); este passo activa.

#### A.3 — PDF shading pattern requisitos

```bash
# Inspeccionar caminhos export PDF actuais
grep -n "fn build_page_stream\|/Resources\|/Pattern" \
  03_infra/src/export.rs

# Estrutura objectos PDF actual
grep -n "Catalog\|Pages\|font_id" 03_infra/src/export.rs | head -20
```

**PDF axial shading specification** (referência ISO 32000):
- `/ShadingType 2` (axial).
- `/ColorSpace /DeviceRGB` (consistente com P257 Color
  to_srgb).
- `/Coords [x0 y0 x1 y1]` (endpoints da direcção do gradient).
- `/Function` (interpolação stops).
- `/Extend [false false]` ou `[true true]`.

**Cálculo `Coords` a partir de `angle` + bounding box**:
- Angle 0° (rad) → x0=left, y0=center, x1=right, y1=center.
- Angle 90° → x0=center, y0=bottom, x1=center, y1=top.
- Generalização: projecção do angle no rectangle.

Decisão arquitectural Fase A.3: **cálculo Coords a partir de
angle é responsabilidade L1** (entities/gradient.rs ou
helper) ou **L3** (exporter, conhece bounding box).
Recomendação preliminar: **L3** — exporter conhece bbox
real do shape; L1 fica puro em representação angular.

#### A.4 — Decisão forma cristalina

**Decisão preliminar P262** (a confirmar Fase A):

```rust
// 01_core/src/entities/gradient.rs

pub struct GradientStop {
    pub color:  Color,
    pub offset: Ratio,
}

pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    // space: ColorSpace,    // scope-out — usa sRGB sempre (per ADR-0083 §scope-out #3)
    // relative: Relative,   // scope-out — assume "bounding-box"
    // anti_alias: bool,     // scope-out — assume true
}

pub enum Gradient {
    Linear(Linear),
    // Radial(Radial),  // P-Gradient-Radial — comentário reserva
    // Conic(Conic),    // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(stops: impl Into<Arc<[GradientStop]>>, angle: Angle) -> Self;
}

impl From<Gradient> for Paint;  // Paint::Gradient(g)
```

**Decisão sub-componente GradientStop**:
- **Opção α**: ficheiro próprio `entities/gradient_stop.rs`
  (precedente subpadrão "Tipo entity em ficheiro próprio" N=5
  per P159C).
- **Opção β**: inline em `gradient.rs` (mais compacto;
  GradientStop está em ADR-0029 §exclusões como sub-componente).

**Recomendação preliminar**: **Opção β (inline)** — GradientStop
é literalmente sub-componente Gradient per ADR-0029 §exclusões.
Reaviva subpadrão "sub-componente inline" análogo a `Cmyk`/
`WeightedColor` (per ADR-0029 §exclusões precedente).

#### A.5 — Validações obrigatórias stdlib

`native_gradient_linear`:
- Stops vazios → erro hard.
- Stop com offset fora de [0, 1] → erro hard.
- Stops não-ordenados por offset → ordenar ou erro? Decisão
  local (recomendação: ordenar com warning).
- Angle aceita tanto `Angle` quanto `Float` (radianos) per
  precedente `native_rotate` P78.

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
com a seguinte estrutura (imutável após criação per
ADR-0085 — **primeiro consumo directo** pós-P260):

```markdown
# Diagnóstico Gradient vanilla — Passo 262 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável (primeiro consumo directo pós-P260) +
ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-262.md` (spec).
**Análogo estrutural**: `diagnostico-color-vanilla-passo-257.md`
(P257) + `diagnostico-paint-vanilla-passo-261.md` (P261).

---

## §1 — Estrutura literal vanilla Gradient

(Colar output A.1 — enum vanilla + Linear struct +
GradientStop + métodos/conversões.)

## §2 — Consumers cristalino (impacto P262)

(Colar output A.2 + análise — Stroke.paint via From<Gradient>
automatic; PDF exporter branch novo; Value::Gradient activar.)

## §3 — PDF shading pattern arquitectura

(Output A.3 — cálculo Coords axial em L3 vs L1; estrutura
objectos PDF; recursos /Pattern dict.)

## §4 — Decisão forma cristalina

(Tabela A.4 com decisão por campo + Opção α/β GradientStop
sub-componente + scope-outs space/relative/anti_alias.)

## §5 — Validações stdlib

(Output A.5 — regras `native_gradient_linear`.)

## §6 — Plano materialização P262.C

(Cross-references sub-passos B/C/D.)

## §7 — Limitações conscientes

- Linear only (Radial/Conic comentários reserva).
- ColorSpace fixo sRGB (scope-out).
- Anti-alias assume true (scope-out).
- Relative assume "bounding-box" (scope-out).
- Stops ColorSpace assume todos sRGB.

## §8 — Referências

(ADR-0029, ADR-0033, ADR-0083, ADR-0085, ADR-0086 §"Critério
revisão", P257, P261 precedentes.)
```

### Critério de aceitação P262.A

- Ficheiro
  `diagnostico-gradient-vanilla-passo-262.md` criado em
  `00_nucleo/diagnosticos/`.
- §1-§8 preenchidos com conteúdo literal.
- Decisão Opção α/β em §4 explicitada.
- Decisão Coords L1 vs L3 em §3 explicitada.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0, ADRs ou DEBT.md.

---

## §2 — Sub-passo P262.B: Criar ADR-0087

**Objectivo**: cumprir ADR-0029 §"Simplificações aceites
apenas com ADR explícita".

### Estrutura ADR-0087

Ficheiro novo
`00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`:

- **Status**: `PROPOSTO` (transita `IMPLEMENTADO` em
  P262.D pós-materialização; paridade pattern P257
  ADR-0083 + P261 ADR-0086 — **N=2 → N=3** subpadrão "ADR
  PROPOSTO+IMPLEMENTADO mesmo passo"; **atinge limiar
  formalização**).
- **Contexto**: Gradient vanilla 3 variants
  (Linear/Radial/Conic); cristalino actual zero hits
  (P259 confirmou); ADR-0086 §"Critério revisão" aponta para
  este passo; sequência arquitectural P259 Cenário B2
  Opção 1 sub-passo 2.
- **Decisão**:
  - `Gradient::Linear(Linear)` materializa.
  - Radial/Conic comentários reserva.
  - `GradientStop` sub-componente per Opção α/β (Fase A).
  - PDF shading via `/ShadingType 2` axial.
- **Análise paridade**:
  - Paridade Linear: completa (stops + angle).
  - Paridade Radial/Conic: scope-out documentado.
- **Scope-outs**:
  - **Radial gradient** → P-Gradient-Radial (passo futuro
    quando consumer real exigir).
  - **Conic gradient** → P-Gradient-Conic (passo futuro;
    baixa prioridade).
  - **`space` ColorSpace** → preservar sRGB sempre
    (scope-out documentado; revisão pós-Color operadores).
  - **`relative` placement** → assume "bounding-box"
    (vanilla default; scope-out "self-relative" diferido).
  - **`anti_alias`** → assume true (PDF default).
- **Consequências**:
  - **Positivas**: activa `Paint::Gradient` variant;
    consumer real Gradient em Stroke + Fill futuro;
    user-facing `#gradient.linear(...)` funcional;
    PDF shading pattern primeiro uso.
  - **Negativas**: PDF exporter ganha caminho complexo
    (shading objects + Function dict + Pattern dict).
  - **Neutras**: ADR-0086 §"Critério revisão" cumprido;
    Paint enum 1 variant adicional activa.
- **Alternativas**:
  - α1 — Linear only (escolhida).
  - α2 — Linear + Radial juntos (rejeitada — magnitude
    M+ vs M; passo dedicado preferível).
  - α3 — Stubs unit Radial/Conic (rejeitada per pattern
    P257/P261 — variants vazios poluem enum).
- **Critério revisão**:
  - Radial → P-Gradient-Radial (passo dedicado M; activar
    `Gradient::Radial(Radial)`).
  - Conic → P-Gradient-Conic (passo dedicado M; activar
    `Gradient::Conic(Conic)`).
- **Subpadrões aplicados**:
  - "ADR PROPOSTO+IMPLEMENTADO mesmo passo via Cenário
    B1/B2" N=2 → **N=3** (P257 + P261 + P262 — limiar
    formalização atingido).
  - "Refactor cross-cutting entity primitivo" N=3 → **N=4**
    se PDF exporter expand for cross-cutting substantivo.
  - "Diagnóstico imutável precedente à acção" N=4 → **N=5
    primeiro consumo directo** pós-P260 ADR-0085.
- **Referências**: ADR-0083 (Color), ADR-0086 (Paint),
  ADR-0029 (regra), ADR-0085 (diagnóstico imutável),
  ADR-0027 (PDF objects estrutura precedente).

### Critério de aceitação P262.B

- ADR-0087 criada `PROPOSTO`.
- README ADRs distribuição: PROPOSTO 11 → **12** transitório
  (entra em P262.B, sai em P262.D); total 73 → **74**.
- Cross-references explícitas a ADR-0086 §"Critério revisão".
- Zero violations lint.

---

## §3 — Sub-passo P262.C: Materialização

### C.1 — Criar prompts L0

#### C.1.1 — `entities/gradient.md`

Análogo a `entities/color.md` (P257) + `entities/paint.md`
(P261).

Conteúdo mínimo:
- Módulo `01_core/src/entities/gradient.rs`.
- Camada L1.
- Propósito (referência ADR-0087).
- Tipo `Gradient` enum + `Linear` struct.
- Sub-componente `GradientStop` (inline ou ficheiro próprio
  per decisão Fase A §4).
- Conversões `From<Gradient> for Paint`.
- Critérios verificação.
- Notas paridade vanilla (3 variants → 1; scope-outs).
- Cross-references ADR-0086/ADR-0087/ADR-0083.

#### C.1.2 — `entities/gradient_stop.md` (apenas se Opção α)

Análogo (sub-componente).

### C.2 — Materializar `01_core/src/entities/gradient.rs`

**Ordem obrigatória — testes primeiro per CLAUDE.md**.

#### C.2.1 — Testes

```rust
// 01_core/src/entities/gradient.rs (tests submodule)

#[test]
fn gradient_stop_construcao() {
    let s = GradientStop {
        color: Color::rgb(255, 0, 0),
        offset: Ratio(0.5),
    };
    assert_eq!(s.color, Color::rgb(255, 0, 0));
}

#[test]
fn linear_construcao_2_stops() {
    let g = Gradient::linear(
        vec![
            GradientStop { color: Color::rgb(255, 0, 0), offset: Ratio(0.0) },
            GradientStop { color: Color::rgb(0, 0, 255), offset: Ratio(1.0) },
        ],
        Angle::deg(0.0),
    );
    match g {
        Gradient::Linear(l) => {
            assert_eq!(l.stops.len(), 2);
            assert_eq!(l.angle, Angle::deg(0.0));
        }
    }
}

#[test]
fn linear_construcao_multi_stops() { ... }

#[test]
fn gradient_to_paint_via_from() {
    let g = Gradient::linear(...);
    let p: Paint = g.into();
    matches!(p, Paint::Gradient(_));
}

#[test]
fn gradient_partial_eq() { ... }

#[test]
fn gradient_clone_arc_O1() { ... }  // Arc<[T]> clone verifica O(1)

// +10 tests adicionais cobrindo edge cases (Ratio bounds,
// Angle radianos vs graus, stops ordering, etc.)
```

Executar `cargo test gradient::` — verificar falham.

#### C.2.2 — Implementação L1

```rust
// 01_core/src/entities/gradient.rs

use std::sync::Arc;
use crate::entities::color::Color;
use crate::entities::layout_types::{Angle, Ratio};
use crate::entities::paint::Paint;

/// Sub-componente per ADR-0029 §exclusões.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    pub color:  Color,
    pub offset: Ratio,
}

/// Linear gradient — paridade vanilla LinearGradient.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
}

/// Gradient — enum tagged paridade vanilla.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Linear),
    // Radial(Radial),  // P-Gradient-Radial — comentário reserva
    // Conic(Conic),    // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(
        stops: impl Into<Arc<[GradientStop]>>,
        angle: Angle,
    ) -> Self {
        Gradient::Linear(Linear {
            stops: stops.into(),
            angle,
        })
    }
}

// Activa Paint::Gradient — ADR-0086 §"Critério revisão" cumprido.
impl From<Gradient> for Paint {
    fn from(g: Gradient) -> Self {
        Paint::Gradient(g)
    }
}
```

#### C.2.3 — Activar `Paint::Gradient` em `entities/paint.rs`

```rust
// 01_core/src/entities/paint.rs

use crate::entities::gradient::Gradient;

#[derive(Debug, Clone, PartialEq)]  // Copy removido — Gradient não é Copy
pub enum Paint {
    Solid(Color),
    Gradient(Gradient),    // P262 — descomentado; ADR-0086 §"Critério revisão" cumprido
    // Tiling(Tiling),      // futuro — comentário reserva
}

impl Paint {
    pub fn solid(c: Color) -> Self { Paint::Solid(c) }

    pub fn to_color(&self) -> Color {
        match self {
            Paint::Solid(c) => *c,
            // Para Gradient: PDF emit fica para L3; helper auxiliar
            // pode devolver primeiro stop como fallback.
            Paint::Gradient(g) => g.first_stop_color(),  // ver §C.4
        }
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self { Paint::Solid(c) }
}

impl From<Gradient> for Paint {
    fn from(g: Gradient) -> Self { Paint::Gradient(g) }
}
```

**Decisão crítica: `Paint::to_color()` para Gradient**:
- **Opção 1**: retornar primeiro stop como fallback (perde
  informação gradient; preserva API).
- **Opção 2**: tornar `to_color()` retornar `Option<Color>`
  (refactor consumers).
- **Opção 3**: novo método `Paint::as_solid() -> Option<Color>`
  + manter `to_color()` para Solid only via panic.

**Recomendação preliminar**: **Opção 1** com helper `first_stop_color`
em Gradient retornando primeiro stop — preserva API P261; uso de
`to_color()` em consumers que precisam de Color real
(e.g. fallback PDF) fica documentado como "fallback Solid;
Gradient renderiza via L3 shading separadamente".

#### C.2.4 — Activar `Value::Gradient` em `entities/value.rs`

```rust
// 01_core/src/entities/value.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // ... existing variants
    Color(Color),
    // P262 — Gradient activado per ADR-0087.
    Gradient(Gradient),
    // ... outras comentadas continuam comentadas
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            // ...
            Value::Color(_) => "color",
            Value::Gradient(_) => "gradient",   // P262
            // ...
        }
    }
}

impl From<Gradient> for Value {
    fn from(g: Gradient) -> Self { Value::Gradient(g) }
}
```

### C.3 — Stdlib func `native_gradient_linear`

#### C.3.1 — Decisão localização

Opções:
- **Opção α**: novo ficheiro `stdlib/gradients.rs`.
- **Opção β**: inline em `stdlib/shapes.rs` (precedente —
  Color funcs em shapes per P25; Paint em geometry).

**Recomendação preliminar**: **Opção α (novo ficheiro)** —
gradients são família substantiva; precedente subpadrão
"módulo stdlib por domínio" (P156K-pre); permite Radial/Conic
futuros sem conflito.

#### C.3.2 — Materialização

```rust
// 01_core/src/rules/stdlib/gradients.rs

use crate::entities::color::Color;
use crate::entities::gradient::{Gradient, GradientStop, Linear};
use crate::entities::layout_types::{Angle, Ratio};
use crate::entities::value::Value;
use crate::rules::stdlib::err;

pub fn native_gradient_linear(args: &[Value], named: &[(String, Value)]) -> Result<Value> {
    // stops: array positional obrigatório
    // angle: named opcional, default Angle::deg(0)

    let stops_val = args.first().ok_or_else(|| err("gradient.linear espera array de stops"))?;
    let stops_arr = stops_val.cast_array().ok_or_else(|| err("stops deve ser array"))?;

    let mut stops: Vec<GradientStop> = Vec::with_capacity(stops_arr.len());
    for (i, stop_val) in stops_arr.iter().enumerate() {
        // Aceita: Color directo (offset implícito uniforme)
        //         OR (Color, Ratio) tuple/array de 2
        //         OR struct-like (color: ..., offset: ...) — decisão A.5
        let stop = parse_stop(stop_val, i, stops_arr.len())?;
        stops.push(stop);
    }

    if stops.is_empty() {
        return Err(err("gradient.linear: pelo menos 1 stop requerido"));
    }

    let angle = extract_angle(named, "angle")?
        .unwrap_or(Angle::deg(0.0));

    Ok(Value::Gradient(Gradient::linear(stops, angle)))
}
```

Helpers privados:
- `parse_stop(val, idx, total) -> Result<GradientStop>`.
- `extract_angle(named, key) -> Result<Option<Angle>>` (já
  existe? — verificar; criar se não).

### C.4 — PDF exporter shading pattern

#### C.4.1 — Estrutura objectos novos

`03_infra/src/export.rs` ganha:

1. **`pattern_resources: HashMap<usize, PatternResource>`** —
   mapa de gradients deduplicados por `Arc::as_ptr(stops)`
   (paridade pattern image P73).

2. **Função `emit_axial_shading_pattern(coords, stops) -> ObjectId`** —
   emite:
   - `/ShadingType 2` axial.
   - `/ColorSpace /DeviceRGB`.
   - `/Coords [x0 y0 x1 y1]`.
   - `/Function` — sub-objecto interpolação (Type 2 ou Type 3).
   - `/Extend [false false]`.

3. **Função `emit_pattern_dict(shading_id) -> ObjectId`** —
   wrapper `/Pattern /PatternType 2 /Shading shading_id`.

4. **`build_page_stream_*`** adapta:
   - Quando `paint: Paint::Solid(c)` → `c.to_srgb()` + `rg`.
   - Quando `paint: Paint::Gradient(g)` →
     - `/Pattern cs` (set colour space to Pattern).
     - `/P1 scn` (apply pattern).
     - Pattern referenciado em `/Resources /Pattern`.

5. **Cálculo `Coords`** (decisão Fase A.3):
   - Recomendação L3: helper `compute_axial_coords(angle, bbox)`
     em `export.rs`.
   - `bbox` é local frame do shape (rect/path).

#### C.4.2 — Cobertura PDF spec

Restrições conservadoras P262:
- `/ShadingType 2` (axial) only — Radial seria `/ShadingType 3`.
- `/ColorSpace /DeviceRGB` — sRGB sempre (per scope-out
  ADR-0087).
- Function Type 3 (stitching) para múltiplos stops; Type 2
  (exponential) para 2 stops only.

#### C.4.3 — Tests E2E (esperado ~5-8)

```rust
// 03_infra/tests/gradient_export.rs ou tests inline

#[test]
fn export_pdf_gradient_linear_2_stops_emits_shading() { ... }
#[test]
fn export_pdf_gradient_linear_angle_0_horizontal() { ... }
#[test]
fn export_pdf_gradient_linear_angle_90_vertical() { ... }
#[test]
fn export_pdf_gradient_deduplication() { ... } // Arc::as_ptr
#[test]
fn export_pdf_gradient_in_stroke() { ... } // user-facing E2E
```

### C.5 — Verificação intermediária

```bash
cargo build --workspace
# Esperado: verde após todos os ficheiros editados
RUST_MIN_STACK=33554432 cargo test --workspace --release
# Esperado: 2341 → 2356-2361 (+15-20 P262)
cargo run -p crystalline-lint -- --fix-hashes .
# Esperado: 2-3 hashes propagados (gradient.md + paint.md + value.md)
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
```

### Critério de aceitação P262.C

- `entities/gradient.rs` materializado com 15+ tests verdes
  (incluindo GradientStop + Linear + From<Gradient> for
  Paint).
- `entities/paint.rs` `Paint::Gradient(Gradient)` activado;
  Copy removido se aplicável.
- `entities/value.rs` `Value::Gradient(Gradient)` activado;
  `type_name() == "gradient"`.
- `entities/mod.rs` re-export Gradient adicionado.
- Stdlib `native_gradient_linear` registado.
- PDF exporter emite `/Pattern /Shading /ShadingType 2`
  correctamente.
- Tests workspace **2341 → 2356-2361** (+15-20).
- Zero violations linter.
- Hashes propagados.

---

## §4 — Sub-passo P262.D: Promoção ADR + relatório

### D.1 — Promover ADR-0087 PROPOSTO → IMPLEMENTADO

`00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`:
- Status: `PROPOSTO` → **`IMPLEMENTADO`**.
- Adicionar linha **Validado**: P262.
- Adicionar secção **Aplicação**: referência a
  `00_nucleo/materialization/typst-passo-262-relatorio.md`.

**Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo via
Cenário B1/B2"**: N=2 → **N=3** (P257 + P261 + **P262**).
**Patamar N=3 atinge limiar formalização clara**. Candidato a
meta-ADR futuro (improvável — padrão auto-documentado em
cada aplicação per pattern P156K).

### D.2 — Actualizar README ADRs

- Distribuição: PROPOSTO 12 → **11** (ADR-0087 promovida);
  IMPLEMENTADO 26 → **27**.
- Total **74** preservado.
- Tabela: entrada ADR-0087 IMPLEMENTADO P262.
- Passos-chave: entrada P262 ~50 linhas descritivas paridade
  P257/P261.

### D.3 — Subpadrões cumulativos

**"Refactor cross-cutting entity primitivo"** (P252 + P257 +
P261):
- N=3 → **N=4 se PDF exporter expand for cross-cutting
  substantivo** (provável; +/Pattern resources + emit branch).
- Patamar N=4 reforça formalização.

**"Diagnóstico imutável precedente à acção"** (P255/P257/
P258/P259):
- N=4 → **N=5 primeiro consumo directo ADR-0085** pós-P260.
- Patamar N=5 valida formalização P260 retroactivamente.

**"ADR PROPOSTO+IMPLEMENTADO mesmo passo"** (P257 + P261):
- N=2 → **N=3 limiar formalização clara**.

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-262-relatorio.md`
estrutura canónica:

- **§1 Sumário executivo** — Fase A confirmada; ADR-0087
  criada+promovida; tests delta (+15-20); ADRs distribuição
  (73 → 74; IMPLEMENTADO 26 → 27).
- **§2 P262.A** — diagnóstico Gradient vanilla resumido;
  primeiro consumo directo ADR-0085.
- **§3 P262.B** — ADR-0087 PROPOSTO criada.
- **§4 P262.C** — código L1 + L3 materializado.
- **§5 P262.D** — ADR-0087 promovida; READMR actualizado.
- **§6 Padrões metodológicos** — subpadrões cumulativos N
  (cross-cutting; diagnóstico imutável; ADR PROPOSTO+IMPL).
- **§7 Cobertura** — Visualize ~53% → ~61% (+8pp via Gradient
  Linear); subsistema F.1 (Gradient Linear) promovido ausente
  → implementado.
- **§8 Limitações e trabalho futuro** — Radial/Conic
  scope-outs; ColorSpace fixo sRGB; relative/anti_alias
  scope-outs.
- **§9 Critério de aceitação global P262 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P262.D

- ADR-0087 IMPLEMENTADO.
- README ADRs actualizado (distribuição correcta).
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P262

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [ ] `cargo test --workspace` retorna contagem ≥ baseline
  2341 + 15-20 (sem regressão; +15-20 P262 esperado).
- [ ] `diagnostico-gradient-vanilla-passo-262.md` existe com
  §1-§8 preenchidos.
- [ ] ADR-0087 criada PROPOSTO P262.B → IMPLEMENTADO P262.D.
- [ ] `entities/gradient.md` criado.
- [ ] `entities/gradient.rs` materializado.
- [ ] `entities/paint.rs` `Paint::Gradient(Gradient)` activado.
- [ ] `entities/value.rs` `Value::Gradient(Gradient)` activado.
- [ ] `entities/mod.rs` re-export Gradient adicionado.
- [ ] Stdlib `native_gradient_linear` registado e funcional.
- [ ] PDF exporter emite `/ShadingType 2` correctamente.
- [ ] Tests E2E confirmam `gradient.linear(...)` user-facing.
- [ ] **ADR-0039 preservado literal** (TextStyle.fill: Color
  inalterado).
- [ ] **Paint::Solid + Paint::Gradient ambos funcionais** em
  Stroke.paint.
- [ ] Hashes propagados.
- [ ] README ADRs actualizado.
- [ ] Relatório do passo criado.
- [ ] Paridade observable confirmada (PDFs com gradients
  renderizam visualmente correctos).

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, ADR-0029, ADR-0033, ADR-0083,
   ADR-0085 (primeiro consumo directo), ADR-0086 §"Critério
   revisão", relatórios P257 + P261.
2. **Reportar** estado inicial: tests count (esperado 2341
   pós-P261) + lint baseline + ADRs 73.
3. **P262.A** — Executar comandos Fase A (A.1-A.5); criar
   diagnóstico Gradient vanilla imutável; decisões Opção α/β +
   Coords L1 vs L3 explícitas.
4. **P262.B** — Criar ADR-0087 PROPOSTO conforme estrutura §2.
5. **P262.C** — Criar L0 prompts; `--fix-hashes`; testes
   primeiro; implementação L1; activar Paint::Gradient +
   Value::Gradient; stdlib; PDF exporter.
6. **P262.D** — Promover ADR-0087 IMPLEMENTADO; actualizar
   README ADRs; criar relatório.
7. **Verificação final** — checklist §5 satisfeito.
8. **Reportar** ao utilizador: ADR criada+promovida, tests
   delta, ficheiros criados/editados, recomendação P263+
   pós-P262.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P262.A revela que vanilla Gradient tem mais variants do
  que esperado (e.g. Pattern/Mesh adicionais).
- P262.A revela que **vanilla Linear não usa Angle** (usa
  Direction ou Coords directos) — invalidaria decisão
  arquitectural.
- P262.A revela que **GradientStop tem mais campos** que
  `{ color, offset }` (e.g. opacity per stop) — exige
  decisão local.
- P262.A revela que **`relative` placement** tem semântica
  cruzada com Layouter que exige refactor maior (e.g.
  "self-relative" requer bounding box do parent — cross-module).
- P262.B descobre que ADR-0087 slot está ocupado por outro
  tópico (re-numerar análogo P160A).
- P262.C revela que **`Paint::to_color()` Gradient fallback**
  é arquiteturalmente incoerente (e.g. consumers assumem
  Color real, não primeiro stop) — refactor consumers
  obrigatório.
- P262.C revela que **PDF shading exporter** exige refactor
  estrutural maior do esperado (e.g. resource manager
  redesign) — magnitude real estoira M+ → L.
- P262.C revela que **stops array vazio** ou **angle inválido**
  têm semântica não-óbvia per vanilla — decisão local.
- Decisão de granularidade entre **stdlib/gradients.rs novo**
  vs **stdlib/shapes.rs inline** é ambígua (Opção α/β §C.3.1).
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.
- Magnitude real estoira M+ — considerar adiar Radial/Conic
  para passos dedicados (já é a decisão minimalista; estoiro
  significaria refactor exporter maior).

Em qualquer paragem, registar contexto no relatório parcial
e aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P261 (Paint) + P259 (audit)

P259 Cenário B2 Opção 1 sub-passo 2 → spec preliminar de
Gradient Linear. P261 materializou sub-passo 1 (Paint enum
Solid only). P262 executa sub-passo 2 — sequência arquitectural
completa Opção 1 pós-P261.

### Primeiro consumo directo ADR-0085

P260 formalizou ADR-0084 (auditoria condicional) + ADR-0085
(diagnóstico imutável). P261 foi **consumo indirecto**
(diagnóstico Paint vanilla cumpre forma análoga).

**P262 é primeiro consumo directo** — diagnóstico Gradient
vanilla é diagnóstico imutável per ADR-0085 literal,
producido por materialização per ADR-0029 §"Diagnosticar
primeiro". Valida formalização P260 retroactivamente.

### Subpadrão "Refactor cross-cutting entity primitivo" N=3 → N=4

Cumulativo:
- N=1 P252 (Stroke `overhang`).
- N=2 P257 (Color expansão).
- N=3 P261 (Paint wrapper).
- **N=4 P262** (Gradient + PDF exporter expand — se cascade
  exporter contar como cross-cutting substantivo).

**Patamar N=4 reforça formalização**. Próximas aplicações
candidatas: Stroke<T> (Length unit) ou Tiling activação.

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=2 → N=3

Cumulativo:
- N=1 P257 (ADR-0083 Color).
- N=2 P261 (ADR-0086 Paint).
- **N=3 P262** (ADR-0087 Gradient).

**Patamar N=3 atinge limiar formalização clara**. Candidato
a meta-ADR futuro — **improvável** (padrão auto-documentado
em cada ADR individual).

### Subpadrão "Diagnóstico imutável precedente à acção" N=4 → N=5

Cumulativo:
- N=1-4 P255/P257/P258/P259 (audit Fase A).
- **N=5 P262** (diagnóstico vanilla per ADR-0029; primeiro
  consumo directo pós-P260).

**Patamar N=5 valida formalização P260 ADR-0085** —
estabelece precedente para diagnósticos vanilla de tipos
serem tratados análogos a diagnósticos Fase A audit.

### Política "sem novas reservas"

Preservada. Radial/Conic como comentários no enum são
**roadmap visual**, não DEBT/ADR novo. Roadmap real fica em
ADR-0087 §"Critério revisão".

### Pós-P262 — sequência lógica recomendada

1. **P-Gradient-Radial** (M; activa `Gradient::Radial`;
   PDF `/ShadingType 3`).
2. **OU outras Opções P259 alternativas**:
   - DEBT-33 Bézier bbox + Stroke<Length>.
   - Curve variant + Polygon estrutural.
3. **OU Text audit** (consumo directo ADR-0084 + 0085).
4. **OU P-Footnote-N** refino (Model pendência).
5. **OU Tiling activação** (Paint::Tiling — análogo P262
   estrutural).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0027 — PDF objects estrutura (precedente para shading
  pattern emit).
- ADR-0029, ADR-0033, ADR-0034, ADR-0054, ADR-0065 —
  metodologia.
- ADR-0039 (TextStyle SR; preservado literal).
- **ADR-0083** (Color paridade vanilla; precedente N=2 do
  pattern).
- **ADR-0084, ADR-0085** (P260 — auditoria condicional +
  diagnóstico imutável; **primeiro consumo directo P262**).
- **ADR-0086** (Paint wrapper Solid only IMPLEMENTADO P261;
  §"Critério revisão" cumprido por este passo).
- **ADR-0087** (criada por este passo).
- DEBT-1 (fechado P142; preservado).
- `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica vanilla.
- P25 — Color simplificado original (REVOGADO via P257).
- P252 — Stroke `overhang` cross-cutting (precedente N=1).
- P257 — Color paridade vanilla 8/8 (precedente N=2;
  ADR-0083 template).
- P259 — Visualize Fase A audit (Cenário B2 Opção 1 sub-passo
  2 spec preliminar).
- P260 — ADRs meta (formaliza ADR-0084/0085 consumidos
  directamente).
- P261 — Paint wrapper Solid only (precedente N=3 +
  pré-requisito).
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  — diagnóstico imutável precedente.
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  — audit Visualize cobertura agregada.
