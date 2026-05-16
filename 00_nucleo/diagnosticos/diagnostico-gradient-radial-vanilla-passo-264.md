# Diagnóstico Gradient Radial vanilla — Passo 264 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável (**segundo consumo directo** pós-P262) +
ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-264.md` (spec).
**Análogo estrutural directo**:
`diagnostico-gradient-vanilla-passo-262.md` (P262 Linear).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0085.

---

## §1 — Estrutura literal vanilla Radial

```rust
// lab/typst-original/.../visualize/gradient.rs:1063
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

**8 campos vanilla**. Diferenças face spec preliminar §A.1
(palpites informados):

| Spec preliminar | Vanilla literal | Diferença |
|-----------------|-----------------|-----------|
| `focal_center: Smart<Axes<Ratio>>` | `focal_center: Axes<Ratio>` | **NÃO Smart** — directo, default = center via repr |
| `focal_radius: Smart<Ratio>` | `focal_radius: Ratio` | **NÃO Smart** — directo, default 0% |
| `radius: Ratio` | `radius: Ratio` | ✓ |
| `center: Axes<Ratio>` | `center: Axes<Ratio>` | ✓ |

**Defaults vanilla (do `Repr::repr`)**:
- `center` default `(50%, 50%)` (centro do bbox).
- `radius` default `50%`.
- `focal_center` default = `center` (omit se igual).
- `focal_radius` default `0%`.

**Wrapper enum**: `Gradient::Radial(Arc<RadialGradient>)` (paridade
Linear).

---

## §2 — Comparação Linear vs Radial vanilla

| Campo | Linear | Radial | Notas |
|-------|--------|--------|-------|
| `stops` | `Vec<(Color, Ratio)>` | `Vec<(Color, Ratio)>` | comum |
| `angle` | `Angle` | — | Linear-specific |
| `center` | — | `Axes<Ratio>` | Radial-specific (default 50%, 50%) |
| `radius` | — | `Ratio` | Radial-specific (default 50%) |
| `focal_center` | — | `Axes<Ratio>` | Radial-specific (default = center) |
| `focal_radius` | — | `Ratio` | Radial-specific (default 0%) |
| `space` | `ColorSpace` | `ColorSpace` | comum (Oklab default) |
| `relative` | `Smart<RelativeTo>` | `Smart<RelativeTo>` | comum |
| `anti_alias` | `bool` | `bool` | comum |

**Total**: 4 campos comuns + 1 Linear-only (angle) + 4
Radial-only (center, radius, focal_center, focal_radius).

---

## §3 — Consumers cristalino (zero cascade — Paint/Value indiferente a variant)

### §3.1 — Paint::Gradient já activa P262

```bash
$ grep -rn "Paint::Gradient" 01_core/src/ 03_infra/src/
01_core/src/entities/paint.rs:31:    Gradient(Gradient),
01_core/src/entities/paint.rs:50:            Paint::Gradient(g) => g.first_stop_color(),
01_core/src/entities/paint.rs:64:    fn from(g: Gradient) -> Self { Paint::Gradient(g) }
03_infra/src/export.rs: (4 sítios branching emit P263)
```

→ `Paint::Gradient(Gradient)` é enum wrapper indiferente a
variant interno. Aceita `Gradient::Radial(Arc<Radial>)`
automaticamente. **Zero cascade refactor**.

### §3.2 — Value::Gradient já activa P262

```bash
$ grep -n "Value::Gradient" 01_core/src/entities/value.rs
70:    Gradient(crate::entities::gradient::Gradient),
137:            Self::Gradient(_)  => "gradient",
```

→ `Value::Gradient(Gradient)` accept Radial automaticamente.
**Zero cascade refactor**.

### §3.3 — `Gradient::Radial` comentado em gradient.rs

```bash
$ grep -n "Radial" 01_core/src/entities/gradient.rs
194:    // Radial(Arc<Radial>),  // P-Gradient-Radial — comentário reserva
```

→ **Descomentar** + adicionar `Radial` struct + impl.

### §3.4 — Pattern-match `Gradient::Linear` em codebase

```bash
$ grep -rn "Gradient::Linear" 01_core/src/ 03_infra/src/
01_core/src/entities/gradient.rs: (5 sítios em código + tests)
03_infra/src/export.rs: (4 sítios — scan_all_gradients, pattern_resources_for_page, emit_stroke_paint)
```

**Sítios pattern-match exhaustivos sobre Gradient** que precisam
de branch `Radial`:
- `gradient.rs:Gradient::first_stop_color` (1 sítio code + tests).
- `gradient.rs::tests::gradient_clone_arc_o1` (1 sítio test).
- `gradient.rs::tests::gradient_linear_construcao_2_stops` (1 sítio test; usa Gradient::Linear match).
- `export.rs::scan_all_gradients` (1 sítio — `let Gradient::Linear(linear) = g`).
- `export.rs::pattern_resources_for_page` (1 sítio).
- `export.rs::emit_stroke_paint` (1 sítio).

**Total sítios pattern-match codigo ~3 + tests ~3 = ~6**. Magnitude
controlada.

---

## §4 — Decisão forma cristalina

| Campo | Status P264 | Justificação |
|-------|-------------|--------------|
| `stops: Arc<[GradientStop]>` | Materializar | Paridade Linear P262 |
| `center: Axes<Ratio>` | Materializar | Vanilla literal; Axes<T> ausente → criar minimal |
| `radius: Ratio` | Materializar | Vanilla literal |
| `focal_center` | **Scope-out** | Default = center; consumer raro |
| `focal_radius` | **Scope-out** | Default 0%; consumer raro |
| `space` | **Scope-out** | Oklab fixo (paridade ADR-0087) |
| `relative` | **Scope-out** | bbox-relative (paridade ADR-0087) |
| `anti_alias` | **Scope-out** | true assumed (paridade ADR-0087) |

**Subset materializado P264** = 3 campos (stops + center + radius).
Magnitude controlada.

### §4.1 — Decisões Q1-Q5

| Q | Decisão | Justificação |
|---|---------|--------------|
| **Q1** — Materializar tudo ou L1+stdlib? | **L1+stdlib only** | Pattern P262/P263 dividir granularidade N=2 (P265 PDF Radial dedicado) |
| **Q2** — Interpolação Oklab? | **Sim** | Paridade P262 + ADR-0087; reutiliza helpers `interpolate_oklab`/`linear_rgb_to_oklab`/`srgb_to_linear`/`color_to_oklab_with_alpha` |
| **Q3** — GradientStop offset `Option<Ratio>`? | **Sim** | Paridade P262; auto-spacing reutilizado |
| **Q4** — Focal point? | **Scope-out** | Default `center + 0% radius`; raramente usado |
| **Q5** — `Axes<Ratio>` para center? | **Criar Axes<T> minimal** | Vanilla usa amplamente; tuple `(Ratio, Ratio)` perde semântica clara |

---

## §5 — Decisão granularidade ADR

☑ **Opção α — ADR-0088 nova**.
☐ Opção β — Anotação cumulativa ADR-0087.

**Decisão escolhida**: **Opção α**.

**Justificação**:
- Paridade pattern N=2 (P261 ADR-0086 + P262 ADR-0087) — cada
  subset materializado tem ADR própria.
- ADR-0088 documenta scope-outs específicos Radial (focal_*,
  center default 50%50%, radius default 50%) distintos de
  Linear scope-outs (angle).
- +1 ADR aceitável (total 74 → 75); subpadrão "ADR
  PROPOSTO+IMPLEMENTADO mesmo passo" N=3 → N=4 (P257+P261+
  P262+**P264**) — **limiar formalização clara excedido**.

---

## §6 — Axes<Ratio> disponibilidade cristalino

```bash
$ grep -rn "pub struct Axes\b" 01_core/src/entities/
(zero hits)
```

→ **`Axes<T>` ausente em cristalino**.

### §6.1 — Decisão: criar `Axes<T>` minimal

Per spec §C.3.1, criar `01_core/src/entities/axes.rs` minimal:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axes<T> {
    pub x: T,
    pub y: T,
}

impl<T> Axes<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialEq + Eq> Eq for Axes<T> {}
```

L0 prompt `entities/axes.md` criado.
Re-export em `entities/mod.rs`.

**Justificação criar Axes<T>**:
- Vanilla usa amplamente para 2D coordinates.
- Tuple `(Ratio, Ratio)` perde semântica nomeada.
- Tipo genérico permite reuso futuro (e.g. `Axes<Length>`,
  `Axes<bool>`).
- Magnitude minimal (~20 LoC).

---

## §7 — Plano materialização P264.C

### Sequência sub-passos

1. **C.0** — Criar `entities/axes.md` L0 + `entities/axes.rs`
   minimal.
2. **C.1** — Actualizar L0 `entities/gradient.md` com secção
   "Anotação cumulativa P264".
3. **C.2** — Testes primeiro em `entities/gradient.rs` (10-12
   tests Radial).
4. **C.3** — Impl `Radial` struct + `Gradient::Radial(Arc<Radial>)`
   activado + `Gradient::radial(...)` construtor +
   `Gradient::first_stop_color` expand match.
5. **C.4** — Adaptar 3 sítios pattern-match em
   `03_infra/src/export.rs` (scan_all_gradients,
   pattern_resources_for_page, emit_stroke_paint) — paridade
   pattern Linear via `let Gradient::Linear(_) | Gradient::Radial(_)`.

   **PDF emit Radial inicial**: fallback shading com same
   Axial /ShadingType 2 (não real Radial /ShadingType 3) —
   adiado para P265 dedicado. **Decisão pragmática P264**:
   pattern_resources fallback Radial→Solid emit
   (`first_stop_color`); shading PDF dedicado P265.

   **Alternativa P264 mais simples**: scan_all_gradients
   apenas detecta Linear (Radial salta para fallback Solid em
   `emit_stroke_paint` via `Paint::to_color()` chain — já
   funciona graças à expansão Gradient::first_stop_color).

   **Decisão**: opção mais simples (Radial fallback Solid no
   PDF até P265). Magnitude P264 controlada.

6. **C.5** — Stdlib `native_gradient_radial` + `make_gradient_module`
   ganha `radial`.
7. **C.6** — Verificação: lint zero violations; tests
   workspace +10-15.

### Magnitude esperada

- L0 axes.md: ~30 LoC.
- L1 axes.rs: ~25 LoC + 3-5 tests.
- L0 gradient.md secção P264: ~50 LoC.
- L1 gradient.rs: ~50 LoC (Radial struct + impl + Gradient
  variant + construtor + first_stop_color expand) + ~10 tests.
- L2 stdlib gradients.rs: ~30 LoC + ~3 tests.
- L3 export.rs adaptação pattern-match (3 sítios): ~10 LoC
  modificadas.
- **Total magnitude M-** (~2h real).

---

## §8 — Limitações conscientes

- **Radial subset only** — focal_center/focal_radius scope-out.
  Default `center + 0% radius` (vanilla behaviour padrão).
- **Conic continua comentário reserva** — P-Gradient-Conic
  futuro.
- **ColorSpace fixo Oklab** — paridade ADR-0087 P262.
- **`space`/`relative`/`anti_alias` scope-outs** paridade
  ADR-0087.
- **PDF emit Radial fallback Solid** — `first_stop_color`
  até P265 dedicado (`/ShadingType 3`).
- **Sem `Gradient::sample()` user-facing** Radial — futuro.

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (regra principal cumprida).
- ADR-0033 — Paridade observable vanilla.
- ADR-0083 — Color paridade vanilla (precedente N=2 pattern).
- ADR-0084, ADR-0085 — P260 metodologia formalizada
  (**segundo consumo directo** P264.A).
- ADR-0086 — Paint wrapper (`Paint::Gradient` activo absorve
  Radial sem cascade).
- ADR-0087 — Gradient Linear-only (precedente directo N=3
  pattern; §"Critério revisão" cumprido parcialmente).
- ADR-0088 — Gradient Radial-only (a criar P264.B; Opção α).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded.
- ADR-0065 — Inventariar primeiro (cumprido aqui).
- P252 — Stroke `overhang` (precedente N=1 cross-cutting).
- P257 — Color paridade 8/8 (precedente N=2 pattern).
- P261 — Paint wrapper Solid only (precedente N=3 pattern).
- **P262** — Gradient L1+stdlib (precedente directo; **template
  literal P264**).
- **P263** — Gradient Linear PDF (template literal P265 futuro).
- P260 — ADRs meta.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  (1366 linhas; RadialGradient §1063-1080; 8 campos).
