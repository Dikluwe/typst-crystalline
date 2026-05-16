# Passo 264 — Gradient Radial L1+stdlib (replica template P262; PDF dedicado P265)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada **M**
(M+ cap; ~2-3h se decisão minimalista L1+stdlib only, paridade
P262).
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- ADR-0029 (EM VIGOR — obriga diagnóstico vanilla + ADR
  explícita para scope-outs).
- ADR-0033 (paridade observable).
- ADR-0054 (perfil graded — scope-outs space/relative aceites).
- ADR-0065 (inventariar primeiro).
- **ADR-0083** (Color paridade vanilla — precedente N=2).
- **ADR-0084, ADR-0085** (P260 — auditoria condicional +
  diagnóstico imutável; diagnóstico vanilla cumpre forma
  análoga ADR-0085, paridade P262).
- **ADR-0086** (Paint wrapper IMPLEMENTADO P261; `Paint::Gradient`
  variant já activa per P262; este passo adiciona variant
  Gradient::Radial sem tocar Paint).
- **ADR-0087** (Gradient Linear-only IMPLEMENTADO P262 +
  anotação cumulativa P263 PDF shading complete; **este
  passo vai criar ADR nova ou anotar ADR-0087** — decisão
  granularidade Fase A).
- Relatórios precedentes:
  - **P262** (Gradient Linear L1+stdlib — **template directo
    deste passo**).
  - **P263** (Gradient Linear PDF — template directo de P265
    futuro).
  - P257 (Color paridade — precedente N=2 pattern
    "PROPOSTO+IMPLEMENTADO mesmo passo").
  - P261 (Paint wrapper — precedente N=3 mesmo pattern).

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
  (Fase A diagnóstico vanilla per ADR-0029 §"Diagnosticar
  primeiro"; imutável per ADR-0085 — segundo consumo directo
  pós-P262 que foi N=5).
- ADR nova (provável **ADR-0088**) — "Gradient Radial
  materializado; Conic scope-out preservado" per ADR-0029
  §"Simplificações aceites apenas com ADR explícita".
  - **Alternativa**: anotação cumulativa ADR-0087 se decisão
    de granularidade Fase A unificar Linear+Radial no mesmo
    ADR (decisão preliminar: **ADR nova**, paridade pattern
    P262 vs P261 — cada subset materialização tem ADR
    própria).
- Prompt L0 actualizado: `00_nucleo/prompts/entities/gradient.md`
  ganha secção "Anotação cumulativa P264 — Radial variant
  materializada" (paridade pattern P261→P262 anotação aditiva).
- Código L1 actualizado:
  - `01_core/src/entities/gradient.rs` ganha `Radial` struct +
    activa `Gradient::Radial(Arc<Radial>)` variant (era
    comentário reserva).
- Stdlib actualizado: `01_core/src/rules/stdlib/gradients.rs`
  ganha `native_gradient_radial` + registo namespace
  `gradient.radial`.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-264-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

0. **Vanilla read-first explicitamente autorizado** —
   `lab/typst-original/` está disponível no filesystem;
   Claude Code **deve** ler literal qualquer estrutura
   vanilla antes de decisões arquitecturais. Especificamente
   neste passo:
   - `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
     — Radial struct + campos + métodos (Fase A §A.1).
   - `lab/typst-original/crates/typst-library/src/layout/axes.rs`
     (ou similar) — `Axes<T>` se existir vanilla (Fase A §A.6).
   - Outros ficheiros conforme Fase A revele dependências.

   Palpites informados na spec preliminar (e.g. `focal_center:
   Smart<Axes<Ratio>>` em §A.1) **devem ser substituídos por
   evidência literal**. Se vanilla diverge dos palpites, o
   diagnóstico imutável regista o vanilla real e a decisão
   arquitectural P264 ajusta-se.

1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Ordem: diagnóstico vanilla → ADR → prompt L0
   actualizado → fix-hashes → testes-primeiro → código L1 →
   stdlib.
2. **Decisão minimalista declarada (paridade P262 pattern)** —
   Radial materializa; **Conic continua comentário reserva**.
   Expansão consumer-driven futura (P-Gradient-Conic).
3. **P264 NÃO toca PDF exporter** — pattern "P262/P263 dividir
   granularidade" replicado: P264 = L1+stdlib; **P265 = PDF
   shading Radial dedicado** com `/ShadingType 3`. Subpadrão
   "P262/P263 dividir granularidade" cresce **N=1 → N=2**.
4. **ADR-0087 §"Critério revisão" cumprido parcialmente** —
   `Gradient::Radial(Arc<Radial>)` variant activada (era
   comentário reserva). Conic continua scope-out.
5. **ADR-0029 §"Simplificações aceites apenas com ADR
   explícita"** — Radial materializa subset (provavelmente
   scope-out `space`/`relative`/`anti_alias`/`focal_*`
   conforme Fase A); ADR-0088 documenta:
   - Diferença vanilla vs cristalino.
   - Custo semântico scope-outs.
   - Critério revisão (passos específicos).
6. **Ordem testes-primeiro** — para cada código novo: testes
   antes de implementação.
7. **`crystalline-lint .`** zero violations no fim.
8. **Tests workspace** sem regressão (baseline 2369 pós-P263).
   Esperado **+10-15** (paridade P262 +20; P264 escopo
   ligeiramente menor pois Paint+Value já preparados).
9. **Política "sem novas reservas"** preservada — Conic continua
   comentário reserva em gradient.rs; ADR-0088 §"Critério
   revisão" aponta para P-Gradient-Conic.
10. **ADR-0039 preservada literal** (`TextStyle.fill: Option<Color>`).
11. **Materialization é leitura proibida por iniciativa
    própria**.

---

## §1 — Sub-passo P264.A: Fase A diagnóstico Radial vanilla obrigatório

**Objectivo**: produzir inventário literal de Radial vanilla +
analisar diferenças face a Linear + decidir forma cristalina.

**Materialização**: zero código novo. Apenas leitura e
diagnóstico imutável per ADR-0085 (segundo consumo directo
pós-P262).

### Acções obrigatórias

#### A.1 — Leitura literal vanilla

```bash
# Estrutura Radial vanilla
grep -A 30 "^\s*pub struct RadialGradient\b\|^\s*pub struct Radial\b" \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# Campos esperados (a confirmar): stops, center, focal_center,
# focal_radius, radius, space, relative, anti_alias
grep -n "pub center\|pub radius\|pub focal_\|pub stops" \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# Construtor / método linear (template já existente)
grep -A 15 "fn radial\|gradient.radial" \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# Stdlib registo
grep -rn "gradient\.radial\|fn radial" \
  lab/typst-original/crates/typst-library/src/visualize/ | head -10
```

**Output esperado** (com base ADR-0029 §enumeração + conhecimento
geral; **a confirmar literal**):

```rust
pub struct RadialGradient {
    pub stops: Vec<(Color, Ratio)>,
    pub center: Axes<Ratio>,
    pub focal_center: Smart<Axes<Ratio>>,
    pub focal_radius: Smart<Ratio>,
    pub radius: Ratio,
    pub space: ColorSpace,
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

#### A.2 — Comparação Linear vs Radial vanilla

```bash
# Diff campos Linear vs Radial
grep -A 20 "pub struct LinearGradient" \
  lab/typst-original/crates/typst-library/src/visualize/gradient.rs
# (já lido em P262.A; comparar)
```

**Diferenças esperadas**:
- Linear: `angle: Angle`.
- Radial: `center`, `focal_center`, `focal_radius`, `radius`
  (4 campos coordenadas em vez de 1 angle).
- Campos comuns: `stops`, `space`, `relative`, `anti_alias`.

#### A.3 — Consumers cristalino — sem impacto cross-cutting

```bash
# Paint::Gradient já activa P262 — não toca consumers
grep -rn "Paint::Gradient\|Gradient::Linear" 01_core/src/ 03_infra/src/

# Value::Gradient já activa P262
grep -n "Value::Gradient" 01_core/src/entities/value.rs

# `Gradient::Radial` actualmente comentário reserva em gradient.rs
grep -n "Radial" 01_core/src/entities/gradient.rs
```

**Output esperado**:
- `Paint::Gradient(Gradient)` activo P262 — accept Radial automaticamente.
- `Value::Gradient(Gradient)` activo P262 — accept Radial automaticamente.
- `Gradient::Radial(...)` comentado em gradient.rs — descomentar.
- **Zero cascade refactor consumers** — Paint/Value são
  enum-wrapper indiferente a variant interno Gradient.

#### A.4 — Decisão forma cristalina + scope-outs

**Decisão preliminar P264** (a confirmar Fase A):

```rust
// 01_core/src/entities/gradient.rs (delta sobre P262)

pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,           // novo campo (vs Linear angle)
    pub radius: Ratio,                  // novo campo
    // pub focal_center: Smart<Axes<Ratio>>,  // scope-out — vanilla = center fallback
    // pub focal_radius: Smart<Ratio>,        // scope-out — vanilla = 0% fallback
    // space, relative, anti_alias: scope-outs paridade P262
}

pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),  // P264 — descomentado; ADR-0087 §"Critério revisão" cumprido
    // Conic(Arc<Conic>),  // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(...) -> Self;  // P262 preserved
    pub fn radial(
        stops: impl Into<Arc<[GradientStop]>>,
        center: Axes<Ratio>,
        radius: Ratio,
    ) -> Self;
    pub fn first_stop_color(&self) -> Color;  // P262 preserved; expand match
}

impl Radial {
    pub fn effective_offsets(&self) -> Vec<f32>;  // paridade Linear
    pub fn sample(&self, t: f32) -> Color;        // paridade Linear (mesma Oklab interp)
}

// `From<Gradient> for Paint` continua a cobrir Radial via wrapper enum.
```

**Decisão pre-flight equivalente P262 Q1-Q3**:

| Q | Decisão | Justificação |
|---|---------|--------------|
| **Q1** — Materializar tudo ou L1+stdlib? | **L1+stdlib only** | Pattern P262/P263 dividido; **PDF P265 dedicado**. |
| **Q2** — Interpolação Oklab? | **Sim** (paridade P262 + ADR-0087). | Reutiliza `linear_rgb_to_oklab` + `interpolate_oklab` helpers L1. |
| **Q3** — GradientStop offset `Option<Ratio>`? | **Sim** (paridade P262). | Auto-spacing reutilizado via `effective_offsets()`. |
| **Q4** — Focal point? | **Scope-out** (default `center` + `radius=0`). | Vanilla default behaviour; raramente usado user-facing. |
| **Q5** — `Axes<Ratio>` para center? | **A confirmar** Fase A. | Cristalino pode ter `Axes<Ratio>` materializado ou não; se ausente, decisão local: `(Ratio, Ratio)` tuple vs criar `Axes<Ratio>`. |

#### A.5 — Decisão granularidade ADR

**Opção α — ADR-0088 nova**:
- Análoga a ADR-0083 (Color) + ADR-0086 (Paint) + ADR-0087
  (Linear).
- Foco preservado em Radial específico.
- +1 ADR (total 74 → 75).
- **Recomendação preliminar α** — paridade pattern N=2 (P261
  ADR-0086 + P262 ADR-0087 cada um com ADR dedicada).

**Opção β — Anotação cumulativa ADR-0087**:
- ADR-0087 ganha secção "Anotação P264 — Radial activado".
- Total ADRs preservado em 74.
- Mistura âmbitos (Linear + Radial scope-outs distintos).

**Decisão final** registada em Fase A §A.5 do diagnóstico
imutável.

#### A.6 — `Axes<Ratio>` disponibilidade

```bash
# Confirmar se Axes<T> existe em cristalino
grep -rn "pub struct Axes\b\|Axes<" 01_core/src/entities/ | head -10
```

**Critério**:
- Se `Axes<Ratio>` existe → reutilizar literal.
- Se não existe → decisão local: criar minimalista
  `entities/axes.rs` OR usar tuple `(Ratio, Ratio)` no Radial
  struct directo. Recomendação: criar `Axes<T>` minimal se
  vanilla o usar amplamente (paridade ADR-0029 §enumeração).

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
com estrutura análoga
`diagnostico-gradient-vanilla-passo-262.md` (P262):

```markdown
# Diagnóstico Gradient Radial vanilla — Passo 264 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável (segundo consumo directo pós-P262) +
ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-264.md` (spec).
**Análogo estrutural directo**:
`diagnostico-gradient-vanilla-passo-262.md` (P262 Linear).

---

## §1 — Estrutura literal vanilla Radial

(Colar output A.1.)

## §2 — Comparação Linear vs Radial vanilla

(Tabela diff campos com Linear lido P262.)

## §3 — Consumers cristalino (zero cascade — Paint/Value indiferente a variant)

(Output A.3.)

## §4 — Decisão forma cristalina

(Tabela A.4 com decisão por campo + scope-outs documentados
focal_center/focal_radius/space/relative/anti_alias.)

## §5 — Decisão granularidade ADR

☐ Opção α — ADR-0088 nova.
☐ Opção β — Anotação cumulativa ADR-0087.
Decisão escolhida: _.
Justificação: _.

## §6 — Axes<Ratio> disponibilidade cristalino

(Output A.6 — presente vs ausente vs criar minimalista.)

## §7 — Plano materialização P264.C

(Cross-references sub-passos B/C/D.)

## §8 — Limitações conscientes

- Radial only entre 2 pendentes (Conic comentário reserva).
- `focal_*` scope-out — default `center + 0% radius`.
- `space`/`relative`/`anti_alias` scope-outs paridade P262.
- PDF emit Radial adiado P265 (granularidade ADR-0061 +
  subpadrão "P262/P263 dividir granularidade" N=2).

## §9 — Referências

(ADR-0029, ADR-0033, ADR-0083, ADR-0085, ADR-0086, ADR-0087,
P262 precedente directo, P263 template PDF futuro.)
```

### Critério de aceitação P264.A

- Ficheiro
  `diagnostico-gradient-radial-vanilla-passo-264.md` criado
  em `00_nucleo/diagnosticos/`.
- §1-§9 preenchidos com conteúdo literal.
- Decisão Opção α/β em §5 explicitada.
- Decisão `Axes<Ratio>` em §6 explicitada.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0, ADRs ou DEBT.md.

---

## §2 — Sub-passo P264.B: ADR explícita (conforme decisão §A.5)

**Objectivo**: cumprir ADR-0029 §"Simplificações aceites
apenas com ADR explícita".

### Acções (conforme Opção α ou β escolhida em Fase A §5)

#### B.1 (Opção α) — Criar ADR-0088

Ficheiro novo
`00_nucleo/adr/typst-adr-0088-gradient-radial-only.md`:

- **Status**: `PROPOSTO` (transita `IMPLEMENTADO` em P264.D
  pós-materialização — subpadrão "ADR PROPOSTO+IMPLEMENTADO
  mesmo passo" cresce **N=3 → N=4**).
- **Contexto**: Gradient vanilla 3 variants (Linear/Radial/
  Conic); P262 materializou Linear; P264 materializa Radial;
  Conic continua scope-out. ADR-0087 §"Critério revisão"
  cumprido parcialmente.
- **Decisão**:
  - `Gradient::Radial(Arc<Radial>)` materializa.
  - `Radial { stops, center, radius }` — subset minimal vs
    vanilla (focal_* scope-out).
  - Stdlib `native_gradient_radial`.
  - Conic continua comentário reserva.
- **Análise paridade**:
  - Paridade Radial: subset (4 scope-outs documentados +
    focal scope-out).
  - Paridade Conic: scope-out documentado.
- **Scope-outs**:
  - **`focal_center` + `focal_radius`** → default center + 0%
    radius (vanilla behaviour padrão; revisão pós-consumer
    real).
  - **`space` ColorSpace** → Oklab fixo (paridade ADR-0087).
  - **`relative` placement** → assume bounding-box (paridade
    ADR-0087).
  - **`anti_alias`** → assume true (PDF default).
  - **Gradient::Conic** → P-Gradient-Conic dedicado futuro.
  - **PDF emit Radial** → P265 dedicado (subpadrão dividir
    granularidade N=2).
- **Consequências**:
  - **Positivas**: user-facing `gradient.radial(...)` funcional;
    Paint::Gradient já preparado P262 absorve Radial sem
    cascade extra; cobertura Visualize +5pp.
  - **Negativas**: PDF render Radial inicialmente fallback
    Solid (igual P262 pré-P263); P265 fecha promessa.
  - **Neutras**: ADR-0086 Paint inalterado; ADR-0039 inalterado.
- **Alternativas**:
  - α1 — Radial completo (com focal_*; rejeitada — magnitude
    M+; raramente usado user-facing).
  - α2 — Radial subset (escolhida).
  - α3 — Linear+Radial unificados sem `Radial` struct
    dedicado (rejeitada — paridade pattern P262 `Linear`
    struct dedicado; consistência).
- **Critério revisão**:
  - PDF emit → **P265 PDF shading Radial** (S-M dedicado).
  - Conic → **P-Gradient-Conic** futuro.
  - Focal point → passo dedicado se consumer real exigir.
- **Subpadrões aplicados**:
  - "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=3 → **N=4**
    (P257+P261+P262+P264 — limiar formalização clara
    excedido).
  - "Decisão minimalista (subset materializado) com
    variants comentário reserva" N=3 → **N=4** (P257 Color +
    P261 Paint + P262 Gradient Linear + P264 Gradient Radial).
  - "P262/P263 dividir granularidade L1+stdlib / L3" N=1 →
    **N=2 com sub-passos P264/P265 replicando**.
  - "Diagnóstico imutável precedente à acção" N=5 → **N=6
    segundo consumo directo** pós-P260 ADR-0085.
- **Referências**: ADR-0083, ADR-0086, ADR-0087, ADR-0029,
  ADR-0085, P262 precedente directo, P263 template PDF
  futuro.

#### B.1 (Opção β) — Anotação cumulativa ADR-0087

Editar ADR-0087 acrescentando secção análoga "Anotação
cumulativa P264 — Radial variant activada" com mesmo conteúdo
B.1 Opção α adaptado.

### B.2 — README ADRs

Se Opção α:
- Distribuição: PROPOSTO 11 → **12** transitório (entra
  P264.B, sai P264.D); IMPLEMENTADO 27 → **28** em P264.D.
- Total 74 → **75**.

Se Opção β:
- Sem alteração contagens.

### Critério de aceitação P264.B

- Decisão Opção α/β concretizada.
- Se α: ADR-0088 criada PROPOSTO; README actualizado.
- Se β: ADR-0087 anotada; sem novas ADRs.
- Zero violations lint.

---

## §3 — Sub-passo P264.C: Materialização L1+stdlib

**Ordem obrigatória — testes primeiro per CLAUDE.md**.

### C.1 — Actualizar L0 prompt `entities/gradient.md`

`00_nucleo/prompts/entities/gradient.md` ganha secção:

```markdown
## Anotação cumulativa P264 — Radial variant materializada

(Data 2026-05-15)

Subset Radial materializado per ADR-0088 (ou anotação ADR-0087
conforme decisão Fase A §5).

### Tipos adicionados

```rust
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,  // OU (Ratio, Ratio) conforme Fase A §6
    pub radius: Ratio,
}

impl Radial {
    pub fn effective_offsets(&self) -> Vec<f32>;
    pub fn sample(&self, t: f32) -> Color;  // paridade Linear Oklab
}
```

### Enum Gradient expandido

```rust
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),  // P264 activado
    // Conic(Arc<Conic>),  // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn radial(stops, center, radius) -> Self;
}
```

### Scope-outs

- `focal_center`/`focal_radius` (default center + 0% radius).
- `space` (Oklab fixo, paridade P262).
- `relative`/`anti_alias` (paridade P262).

### Cross-references

- ADR-0088 (criada P264; ou anotação ADR-0087).
- ADR-0086 (Paint::Gradient automaticamente absorve Radial).
- ADR-0039 (preservada literal).
- P262 (Linear precedente; helpers Oklab reutilizados).
- P265 (PDF emit futuro; pattern dividir granularidade).
```

Propagar hash via `--fix-hashes`.

### C.2 — Testes primeiro

```rust
// 01_core/src/entities/gradient.rs (tests submodule)

#[test]
fn radial_construcao_2_stops() {
    let r = Gradient::radial(
        vec![
            GradientStop { color: Color::rgb(255, 0, 0), offset: Some(Ratio(0.0)) },
            GradientStop { color: Color::rgb(0, 0, 255), offset: Some(Ratio(1.0)) },
        ],
        Axes::new(Ratio(0.5), Ratio(0.5)),  // OU (Ratio(0.5), Ratio(0.5)) tuple
        Ratio(0.5),
    );
    match r {
        Gradient::Radial(rad) => {
            assert_eq!(rad.stops.len(), 2);
            assert_eq!(rad.center.x, Ratio(0.5));
            assert_eq!(rad.radius, Ratio(0.5));
        }
        _ => panic!("expected Radial"),
    }
}

#[test]
fn radial_first_stop_color() {
    let r = Gradient::radial(vec![...], Axes::new(...), Ratio(...));
    assert_eq!(r.first_stop_color(), Color::rgb(255, 0, 0));
}

#[test]
fn radial_to_paint_via_from() {
    let r = Gradient::radial(...);
    let p: Paint = r.into();
    matches!(p, Paint::Gradient(Gradient::Radial(_)));
}

#[test]
fn radial_partial_eq() { ... }

#[test]
fn radial_effective_offsets_auto_spacing() { ... }

#[test]
fn radial_sample_oklab() { ... }

#[test]
fn radial_to_value_via_from() {
    let r = Gradient::radial(...);
    let v: Value = r.into();
    assert_eq!(v.type_name(), "gradient");
}

// +5-8 tests cobrindo edge cases (radius 0, stops 1, Axes
// boundaries, etc.).
```

Executar `cargo test gradient::radial` — verificar falham.

### C.3 — Implementação L1

#### C.3.1 — Decidir `Axes<Ratio>`

Se Fase A §6 confirmou `Axes<T>` presente:
- Reutilizar literal.

Se ausente:
- Criar `01_core/src/entities/axes.rs` minimal:
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct Axes<T> {
      pub x: T,
      pub y: T,
  }
  impl<T> Axes<T> {
      pub fn new(x: T, y: T) -> Self { Self { x, y } }
  }
  ```
- Prompt L0 novo `entities/axes.md`.
- Re-export em `entities/mod.rs`.

**Alternativa**: tuple `(Ratio, Ratio)` directo no Radial. Se
vanilla usa Axes<Ratio> amplamente, criar Axes<T>; se isolado,
tuple aceitável.

#### C.3.2 — `Radial` struct + impl

```rust
// 01_core/src/entities/gradient.rs (delta sobre P262)

use crate::entities::axes::Axes;  // se criado §C.3.1

#[derive(Debug, Clone, PartialEq)]
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
}

impl Radial {
    pub fn effective_offsets(&self) -> Vec<f32> {
        // Paridade Linear::effective_offsets (P262)
        // Auto-spacing para stops com offset None
        // ...
    }

    pub fn sample(&self, t: f32) -> Color {
        // Paridade Linear::sample (P262):
        // 1. Encontrar par adjacente (offset_i, offset_{i+1}) tal que offset_i <= t <= offset_{i+1}.
        // 2. t_local = (t - offset_i) / (offset_{i+1} - offset_i).
        // 3. interpolate_oklab(color_i, color_{i+1}, t_local).
        // 4. Retornar Color em sRGB.
        // ...
    }
}
```

#### C.3.3 — Activar `Gradient::Radial(Arc<Radial>)` variant

```rust
// 01_core/src/entities/gradient.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),  // P264 — descomentado
    // Conic(Arc<Conic>),  // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(...) -> Self { ... }  // P262 preserved

    pub fn radial(
        stops: impl Into<Arc<[GradientStop]>>,
        center: Axes<Ratio>,
        radius: Ratio,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
        }))
    }

    pub fn first_stop_color(&self) -> Color {
        match self {
            Gradient::Linear(l) => l.stops[0].color,
            Gradient::Radial(r) => r.stops[0].color,
        }
    }
}
```

**Pattern-match cobertura**: cada match exhaustivo sobre
`Gradient` na codebase ganha branch `Radial`.

```bash
# Identificar sítios pattern-match Gradient
grep -rn "match.*Gradient\|Gradient::Linear" 01_core/src/ 03_infra/src/
```

**Esperado**: poucos sítios (cobertura limitada P262). Cobertura
exhaustiva pattern-match obrigatória per padrão arquitectural.

#### C.3.4 — Helpers Oklab reutilizados de P262

Zero código novo helpers — `linear_rgb_to_oklab`,
`interpolate_oklab`, `srgb_to_linear`, `color_to_oklab_with_alpha`
todos privados em `gradient.rs` já existentes P262. `Radial::sample`
reutiliza-os literal.

### C.4 — Stdlib `native_gradient_radial`

```rust
// 01_core/src/rules/stdlib/gradients.rs

pub fn native_gradient_radial(
    args: &[Value],
    named: &[(String, Value)],
) -> Result<Value> {
    // stops: array positional obrigatório (paridade native_gradient_linear)
    let stops_val = args.first().ok_or_else(|| err("gradient.radial espera array de stops"))?;
    let stops_arr = stops_val.cast_array().ok_or_else(|| err("stops deve ser array"))?;
    let stops = parse_stops(stops_arr)?;

    if stops.is_empty() {
        return Err(err("gradient.radial: pelo menos 1 stop requerido"));
    }

    // center: named (default Axes::new(Ratio(0.5), Ratio(0.5)) — centro)
    let center = extract_axes_ratio(named, "center")?
        .unwrap_or_else(|| Axes::new(Ratio(0.5), Ratio(0.5)));

    // radius: named (default Ratio(0.5) — metade)
    let radius = extract_ratio(named, "radius")?
        .unwrap_or(Ratio(0.5));

    Ok(Value::Gradient(Gradient::radial(stops, center, radius)))
}
```

Adicionar a `make_gradient_module()` em `eval/mod.rs`:
```rust
module.define("radial", native_gradient_radial);
```

Helpers `extract_axes_ratio`, `extract_ratio` privados — verificar
se existem; criar se ausentes.

### C.5 — Validações stdlib

`native_gradient_radial`:
- Stops vazios → erro hard.
- Stop com offset fora de [0, 1] → erro hard.
- Radius fora de [0, 1] → erro hard (paridade vanilla Ratio).
- Center.x ou center.y fora de [0, 1] → warning ou erro
  (decisão local; recomendação: aceitar fora-bounds para
  paridade vanilla — clamping é responsabilidade L3 emit).

### C.6 — Verificação intermediária

```bash
cargo build --workspace
# Esperado: verde
RUST_MIN_STACK=33554432 cargo test --workspace --release
# Esperado: 2369 → 2379-2384 (+10-15 P264)
cargo run -p crystalline-lint -- --fix-hashes .
# Esperado: 1-2 hashes propagados (gradient.md atualizado;
# axes.md novo se criado)
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
```

### Critério de aceitação P264.C

- `entities/gradient.rs` ganha `Radial` struct + `Gradient::Radial`
  activado.
- Eventual `entities/axes.rs` minimal (se §C.3.1 decisão criar).
- Stdlib `native_gradient_radial` registado.
- Tests workspace **2369 → 2379-2384** (+10-15).
- Zero violations.
- Hashes propagados.

---

## §4 — Sub-passo P264.D: Promoção ADR + relatório

### D.1 — Promover ADR-0088 (se Opção α)

`00_nucleo/adr/typst-adr-0088-gradient-radial-only.md`:
- Status: `PROPOSTO` → **`IMPLEMENTADO`**.
- Linha **Validado**: P264.
- Secção **Aplicação**: referência a relatório.

**Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo"**:
N=3 → **N=4** (P257+P261+P262+P264). **Limiar formalização
clara excedido**. Candidato a meta-ADR futuro — **improvável**
(padrão auto-documentado).

### D.2 — Actualizar README ADRs

Se Opção α:
- Distribuição: PROPOSTO 12 → **11** (ADR-0088 promovida);
  IMPLEMENTADO 27 → **28**.
- Total **75** preservado.

Se Opção β:
- Sem alteração contagens.

### D.3 — Subpadrões cumulativos

**"P262/P263 dividir granularidade" N=1 → N=2**:
- N=1 P262 (L1+stdlib) → P263 (L3 PDF).
- **N=2 P264 (L1+stdlib) → P265 (L3 PDF; futuro)**.

**Patamar N=2 reforça pattern**. Próxima aplicação candidata:
P-Gradient-Conic L1+stdlib + L3 PDF se materializar.

**"Decisão minimalista (subset materializado)" N=3 → N=4**:
- N=1 P257 Color (8/8 + 4 scope-outs).
- N=2 P261 Paint Solid only.
- N=3 P262 Gradient Linear only.
- **N=4 P264 Gradient Radial subset**.

**"Diagnóstico imutável precedente à acção" N=5 → N=6**.

### D.4 — Relatório

`00_nucleo/materialization/typst-passo-264-relatorio.md`
estrutura canónica:

- §1 Sumário executivo.
- §2 P264.A — diagnóstico Radial vanilla.
- §3 P264.B — ADR-0088 (ou anotação ADR-0087).
- §4 P264.C — código L1+stdlib materializado.
- §5 P264.D — ADR promovida; README actualizado.
- §6 Padrões metodológicos — subpadrões cumulativos N.
- §7 Cobertura — Visualize ~63% → ~68% (+5pp via Radial L1+stdlib;
  PDF emit Radial fica para P265).
- §8 Limitações e trabalho futuro — P265 PDF dedicated;
  P-Gradient-Conic; focal_* scope-outs.
- §9 Critério de aceitação global — Checklist final.
- §10 Referências.

### Critério de aceitação P264.D

- ADR-0088 IMPLEMENTADO (se Opção α) OU ADR-0087 anotada
  (se Opção β).
- README ADRs actualizado.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P264

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [ ] `cargo test --workspace --release` retorna ≥ 2369 +
  10-15 = 2379-2384.
- [ ] `diagnostico-gradient-radial-vanilla-passo-264.md` existe
  com §1-§9 preenchidos.
- [ ] ADR-0088 criada PROPOSTO P264.B → IMPLEMENTADO P264.D
  (se Opção α) OU ADR-0087 anotada (se Opção β).
- [ ] `entities/gradient.md` L0 actualizado com secção P264.
- [ ] `entities/axes.md` L0 criado (se §C.3.1 decidiu criar).
- [ ] `entities/gradient.rs` `Gradient::Radial(Arc<Radial>)`
  activado; struct Radial materializado.
- [ ] Stdlib `native_gradient_radial` + `make_gradient_module`
  expandido.
- [ ] `scope.define("gradient", ...)` em eval/mod.rs cobre
  Radial via namespace.
- [ ] **ADR-0039 preservada literal** (TextStyle.fill: Color
  inalterado).
- [ ] **Paint::Gradient + Value::Gradient absorvem Radial
  automaticamente** (zero cascade refactor).
- [ ] **PDF render Radial** — fallback first_stop_color até
  P265 (paridade pré-P263 state Linear).
- [ ] Hashes propagados.
- [ ] README ADRs actualizado.
- [ ] Relatório criado.
- [ ] Paridade observable parcial: user-facing
  `gradient.radial(...)` funcional; PDF mostra fallback até
  P265.

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, ADR-0029/0033/0083/0085/0086/0087,
   relatórios P262 + P263.
2. **Reportar** estado inicial: tests 2369 + lint baseline +
   ADRs 74.
3. **P264.A** — Executar comandos Fase A; criar diagnóstico
   Radial imutável; decisões Q1-Q5 + α/β + Axes explícitas.
4. **P264.B** — Criar ADR-0088 PROPOSTO (ou anotar ADR-0087);
   README actualizado.
5. **P264.C** — Actualizar L0 prompt `gradient.md` + (eventual
   `axes.md`); `--fix-hashes`; testes primeiro; implementação
   L1 (Radial struct + activar Gradient::Radial variant) +
   stdlib (native_gradient_radial).
6. **P264.D** — Promover ADR-0088 IMPLEMENTADO; actualizar
   README ADRs; criar relatório.
7. **Verificação final** — checklist §5 satisfeito.
8. **Reportar** ao utilizador: ADR criada+promovida, tests
   delta, ficheiros criados/editados, recomendação P265 (PDF
   Radial dedicado).

---

## §7 — Política de paragem

**Nota preliminar**: a spec contém palpites informados sobre
vanilla (e.g. campos `focal_center: Smart<Axes<Ratio>>`).
**Discrepância palpite-vs-vanilla não é gatilho de paragem
por si** — Fase A regista o vanilla literal e a decisão
arquitectural ajusta-se. Política de paragem aplica-se a
**decisões arquitecturais não-óbvias**, não a confirmações
factuais rotineiras.

Claude Code **deve parar e perguntar ao utilizador** se:

- P264.A revela que **vanilla `RadialGradient` tem mais campos
  do que esperado** (e.g. `direction` adicional) — exigir
  decisão local sobre subset.
- P264.A revela que **vanilla `RadialGradient` não usa
  `Axes<T>` para center** (e.g. usa `Point` directo ou tuple
  separado) — alterar decisão arquitectural.
- P264.A revela que **`focal_center`/`focal_radius` não são
  Smart<>** (e.g. são obrigatórios) — re-considerar scope-out.
- P264.A revela que **`Axes<Ratio>` em cristalino tem
  semântica diferente do esperado** (e.g. é `Axes<Length>`
  apenas) — decisão local: tuple vs criar tipo novo.
- P264.B descobre que slot ADR-0088 está ocupado por outro
  tópico — re-numerar (análogo P160A).
- P264.C revela que **expansão pattern-match `Gradient` em
  consumers** estoira ~10 sítios (vs ~3 esperado) — magnitude
  real M+ → considerar adiar.
- P264.C revela que **`Radial::sample(t)` semântica radial**
  diverge significativamente de Linear (e.g. amostragem 2D
  vs 1D) — re-pensar helpers Oklab reuse.
- Decisão de granularidade entre **ADR-0088 nova** vs
  **anotação ADR-0087** é ambígua.
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.
- Magnitude real estoira M+ — considerar dividir P264 em
  P264 (L1+stdlib core) + P264-X (axes types) ou similar.

Em qualquer paragem, registar contexto no relatório parcial e
aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P262 (Linear L1+stdlib) e P263 (Linear PDF)

P262 = template directo deste passo (L1+stdlib only).
P263 = template directo de P265 futuro (L3 PDF dedicado).

P264 é **réplica intencional do pattern P262/P263 dividido** —
subpadrão "P262/P263 dividir granularidade" cresce N=1 → N=2.

### Subpadrão "P262/P263 dividir granularidade" cresce N=1 → N=2

Cumulativo:
- N=1 P262 (Linear L1+stdlib) → P263 (Linear PDF).
- **N=2 P264 (Radial L1+stdlib) → P265 (Radial PDF; futuro)**.

**Patamar N=2 reforça pattern**. Promoção formalização adiada
per política N=3-4.

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=3 → N=4

Cumulativo:
- N=1 P257 ADR-0083.
- N=2 P261 ADR-0086.
- N=3 P262 ADR-0087.
- **N=4 P264** (ADR-0088; se Opção α).

**Patamar N=4 excede limiar formalização clara**. Candidato a
meta-ADR — **improvável e desnecessário** (padrão auto-documentado
em cada ADR individual; análogo P156K self-documentation).

### Subpadrão "Decisão minimalista (subset materializado)" N=3 → N=4

Cumulativo:
- N=1 P257 Color (8/8 + 4 scope-outs).
- N=2 P261 Paint (Solid only; Gradient/Tiling reserva).
- N=3 P262 Gradient (Linear only; Radial/Conic reserva).
- **N=4 P264 Gradient Radial subset** (focal_* scope-out;
  Conic continua reserva).

**Pattern emergente sólido** confirma: cada tipo wrapper
materializa subset minimal + comentários reserva activáveis em
passos dedicados per ADR §"Critério revisão".

### Política "sem novas reservas"

Preservada. Conic continua comentário reserva em
`entities/gradient.rs`; PDF emit Radial fica em ADR-0088
§"Critério revisão" → P265 dedicado.

### Pós-P264 — sequência lógica recomendada

1. **P265 — Gradient Radial PDF shading** (S-M dedicado;
   replica P263 template; `/ShadingType 3`; fecha promessa
   P264).
2. **OU outras Opções P259 alternativas**:
   - DEBT-33 Bézier bbox + Stroke<Length>.
   - Curve variant + Polygon estrutural.
3. **OU Text audit** (segundo consumo directo ADR-0084 + 0085).
4. **OU P-Footnote-N** refino M (Model pendência).
5. **OU P-Gradient-Conic** L1+stdlib (replica P264 pattern).
6. **OU Tiling activação** (Paint::Tiling — análogo P262/P263
   estrutural).

**Recomendação preliminar pós-P264**: **P265** (paridade
sequência P262→P263 fechou promessa coerentemente; precedente
mais recente bem-sucedido).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0027 — PDF objects estrutura (precedente para P265
  futuro `/ShadingType 3`).
- ADR-0029, ADR-0033, ADR-0034, ADR-0054, ADR-0065 —
  metodologia.
- ADR-0039 — TextStyle SR (preservado literal).
- **ADR-0083** (Color paridade; precedente N=2 pattern
  PROPOSTO+IMPLEMENTADO).
- **ADR-0084, ADR-0085** (P260 — metodologia formalizada;
  segundo consumo directo aqui).
- **ADR-0086** (Paint wrapper; Paint::Gradient activo absorve
  Radial sem cascade).
- **ADR-0087** (Gradient Linear-only; este passo cumpre
  §"Critério revisão" Radial activado).
- **ADR-0088** (criada por este passo; se Opção α).
- DEBT-1 — Fechado P142 (preservado).
- ISO 32000-1 §7.5.7 — Shading patterns (referência PDF futura
  P265).
- P25 — Color simplificado original (REVOGADO via P257).
- P252 — Stroke `overhang` (precedente N=1 cross-cutting; não
  toca P264).
- P257 — Color paridade 8/8 (precedente N=2 do pattern).
- P261 — Paint wrapper Solid only (precedente N=3 do pattern;
  Paint::Gradient activo desde P262 absorve Radial).
- **P262** — Gradient L1+stdlib (precedente directo N=1 do
  subpadrão "dividir granularidade"; **template literal P264**).
- **P263** — Gradient Linear PDF (template literal P265 futuro).
- P260 — ADRs meta (formaliza ADR-0084/0085 consumidos
  metodologicamente).
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico Linear precedente directo.
- `00_nucleo/diagnosticos/diagnostico-paint-vanilla-passo-261.md`
  — diagnóstico Paint precedente.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (1366 linhas; 3 variants; Linear lido P262).
