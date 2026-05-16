# Diagnóstico Gradient Conic vanilla — Passo 267 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0029 §"Diagnosticar primeiro" + ADR-0085
diagnóstico imutável (**terceiro consumo directo vanilla**
pós-P262/P264; **quarto consumo directo geral** pós-P266
audit Fase A formal) + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `typst-passo-267.md` (spec).
**Análogo estrutural directo**:
`diagnostico-gradient-vanilla-passo-262.md` (Linear) +
`diagnostico-gradient-radial-vanilla-passo-264.md` (Radial).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0085.

---

## §A.1 — Vanilla ConicGradient shape literal

```rust
// lab/typst-original/.../visualize/gradient.rs:1145
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConicGradient {
    pub stops: Vec<(Color, Ratio)>,
    pub angle: Angle,
    pub center: Axes<Ratio>,
    pub space: ColorSpace,
    pub relative: Smart<RelativeTo>,
    pub anti_alias: bool,
}
```

**6 campos vanilla**:
- `stops: Vec<(Color, Ratio)>` — tuple (não GradientStop directo).
- `angle: Angle` — ângulo inicial.
- `center: Axes<Ratio>` — centro do cone.
- `space: ColorSpace` — default Oklab.
- `relative: Smart<RelativeTo>` — placement.
- `anti_alias: bool` — anti-aliasing PDF.

**Defaults legíveis no Repr** (`gradient.rs:1160-1205`):
- `angle`: 0deg → omit do Repr.
- `center`: `(50%, 50%)` → omit.
- `space`: Oklab → omit.
- `relative`: Smart::Auto → omit (apenas Custom é renderizado).

**Spec P267 §1 critério paridade** mencionou `focal_center` +
`focal_radius` como scope-out — **CORREÇÃO FACTUAL**:
ConicGradient vanilla **NÃO tem `focal_*` campos**. Esses são
exclusivos de `RadialGradient` (`gradient.rs:1063`).

**Stops Ratio vs Angle no Repr**: storage interno é `Ratio`
(0..1, fração da circumferência); Repr renderiza como `Angle::deg(offset
* 360.0)` — paridade vanilla user-facing aceita ambas formas
input mas storage normaliza para Ratio.

---

## §A.2 — Vanilla stdlib `gradient.conic`

Como `gradient.linear` e `gradient.radial`, o construtor é
materializado via `#[func]` macro vanilla em `impl Gradient`
(`gradient.rs:186` scope). Assinatura esperada (paridade
Linear/Radial):
- Variadic positional: `stops: Vec<Spanned<GradientStop>>`.
- Named: `angle`, `center`, `space`, `relative`.

(Não materialização literal extracta — vanilla usa procmacro
`#[func]`; assinatura cristalino paridade `native_gradient_linear`/
`native_gradient_radial`.)

---

## §A.3 — Cristalino Gradient enum estado

```bash
$ grep -n "pub enum Gradient" 01_core/src/entities/gradient.rs
196:pub enum Gradient {
197:    Linear(Arc<Linear>),
198:    Radial(Arc<Radial>),   // P264 — descomentado per ADR-0088
199:    // Conic(Arc<Conic>),    // P-Gradient-Conic — comentário reserva
200:}
```

Pre-P267 estado: 2/3 variants materializados (Linear P262 +
Radial P264). `Gradient::Conic(Arc<Conic>)` comentário reserva
em linha 199. **Zero cascade refactor consumers esperado** —
Paint/Value já preparados P262 absorvem Conic automaticamente.

---

## §A.4 — Cristalino stdlib actual

```bash
$ grep "linear\|radial" 01_core/src/rules/stdlib/gradients.rs
make_gradient_module dict entries: "linear" + "radial"
native_gradient_linear + native_gradient_radial functions
```

Stdlib actual: 2/3 binds (linear + radial). Conic ausente.

---

## §A.5 — Gap a fechar P267

1. **L1**: `Conic` struct novo + `Gradient::Conic(Arc<Conic>)`
   variant activado + `Gradient::conic(...)` construtor +
   `first_stop_color` expand match.
2. **L1**: `Conic::effective_offsets` + `Conic::sample(t)` paridade
   Linear/Radial (helpers Oklab reutilizados).
3. **L1**: pattern-match em `03_infra/src/export.rs` (3 sítios)
   expand para tratar Radial — adiar PDF emit para P268
   (fallback Solid `first_stop_color` paridade pré-P263 Linear
   state).
4. **L1**: tests gradient.rs Conic (~9-10 tests; paridade
   Radial P264).
5. **Stdlib**: `native_gradient_conic` + entrada `conic` em
   `make_gradient_module()` + helpers parsing.
6. **Stdlib**: tests stdlib (~5 tests; paridade Radial P264).
7. **L0**: anotação cumulativa `entities/gradient.md` secção
   P267 (Conic variant + scope-outs).
8. **ADR-0089** novo PROPOSTO → IMPLEMENTADO mesmo passo
   (paridade P257/P261/P262/P264 N=5 cumulativo).
9. **ADR-0054** anotação cumulativa (cluster Gradient 3/3 L1+
   stdlib completo).
10. **ADR-0088** §"scope-outs" Conic riscado (revogação
    parcial; focal_* preservado per Radial only).

---

## §A.6 — Cenário detectado

☑ **B2 Sub-passo dedicado (cluster expansion)**.

Não é audit Fase A clássico (módulo grande) — P267 é
materialização cluster expansion paridade P264 (Radial L1+
stdlib seguido por P265 PDF). **Cenário pattern emergente
"dividir granularidade L1+stdlib / L3 dedicado" N=3 → N=4**
(P262/P263 + P264/P265 + **P267/P268**).

---

## §A.7 — Cobertura empírica pré-P267 cluster Gradient

| Variant | Pré-P267 | Pós-P267 esperado |
|---------|----------|--------------------|
| Linear | implementado+stdlib+render (P262+P263) | preservado |
| Radial | implementado+stdlib+render (P264+P265) | preservado |
| Conic | ausente (comentário reserva) | **implementado+stdlib** |

**Cobertura cluster**: 2/3 → **3/3 L1+stdlib** pós-P267
(PDF Conic ainda fallback Solid até P268).

**Cobertura Visualize agregada**: ~73% (P265) → ~75-76%
esperado (+2-3pp; Conic L1+stdlib promovido ausente →
`implementado+stdlib`).

---

## §A.8 — Decisão arquitectural

**Subset materializado P267** (paridade P262 Linear + P264
Radial):

```rust
pub struct Conic {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle:  Angle,
}
```

**3 campos materializados**: stops + center + angle.

**Scope-outs documentados** (paridade ADR-0087/0088):
- `space: ColorSpace` — Oklab fixo (paridade pattern).
- `relative: Smart<RelativeTo>` — bbox-local assumido.
- `anti_alias: bool` — true assumed.

**Sem `focal_*` scope-out** (não existe em ConicGradient
vanilla — correcção spec P267 §1).

**Storage de stops**: paridade `Linear`/`Radial` — usa
`Arc<[GradientStop]>` (não tuple `Vec<(Color, Ratio)>` vanilla).
GradientStop {color, offset: Option<Ratio>} reutilizado
literal.

**Construtor `Gradient::conic(stops, center, angle)`**:
- Stops `impl Into<Arc<[GradientStop]>>`.
- Center `Axes<Ratio>` (default `(50%, 50%)` aplicado em
  stdlib).
- Angle `Angle` (default `0deg` aplicado em stdlib).

**Interpolação `Conic::sample(t)`**: paridade `Linear::sample`/
`Radial::sample` (Oklab via helpers reutilizados literal de
P262).

**effective_offsets**: paridade pattern.

**Decisões pre-flight Q1-Q5** (paridade P264):
- Q1 Materializar tudo ou L1+stdlib? **L1+stdlib only** —
  PDF P268 dedicado (pattern N=4 cumulativo "dividir
  granularidade").
- Q2 Interpolação Oklab? **Sim** (paridade P262/P264).
- Q3 GradientStop com `Option<Ratio>`? **Sim** (paridade).
- Q4 focal_*? **N/A** — não existe em Conic vanilla.
- Q5 Axes<Ratio> para center? **Sim** — Axes<T> criado P264.

---

## §A.9 — Helpers Oklab reutilização confirmada

```bash
$ grep "fn interpolate_oklab\|fn color_to_oklab_with_alpha\|fn srgb_to_linear\|fn linear_rgb_to_oklab" 01_core/src/entities/gradient.rs
```

Helpers privados P262/P264 presentes:
- `interpolate_oklab(c0, c1, t)`.
- `color_to_oklab_with_alpha(c)`.
- `srgb_to_linear(c)`.
- `linear_rgb_to_oklab(r, g, b)`.

**Conic reutiliza literal** — zero código novo helpers.

---

## §A.10 — PDF emit Conic adiado P268

**Pattern N=4 cumulativo "dividir granularidade"**:
- N=1: P262 (Linear L1+stdlib) → P263 (Linear PDF).
- N=2: P264 (Radial L1+stdlib) → P265 (Radial PDF).
- **N=3 atingido** P267 (Conic L1+stdlib) → **P268 (Conic
  PDF dedicado)** futuro.

Note: spec P267 mencionou "N=4 cumulativa" mas cluster
Gradient é 3 (não 4) divisões. Spec re-numeração interpretada
como N=3 cumulativo + 1 (este passo) = N=4 — manter
contagem spec por compat.

**P267.C**: 3 sítios pattern-match em `export.rs`
(scan_all_gradients + pattern_resources_for_page +
emit_stroke_paint) ganham branch `Gradient::Conic(_)` →
fallback Solid `first_stop_color` (paridade Radial pre-P265
state).

**P268 dedicado**: substitui fallback por `/ShadingType` real
(provavelmente custom shading function ou fall-back via
amostragem dense + Type 4-7 lattice; decisão local P268).

---

## §A.11 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório.
- ADR-0033 — Paridade observable vanilla.
- ADR-0083 — Color paridade (precedente N=2 pattern).
- ADR-0084 + ADR-0085 — Auditoria condicional + diagnóstico
  imutável (P260; consumido directamente este passo).
- ADR-0086 — Paint wrapper (Paint::Gradient activa absorve
  Conic sem cascade).
- ADR-0087 — Gradient Linear-only (precedente directo N=3
  pattern PROPOSTO+IMPL).
- **ADR-0088** — Gradient Radial-only (precedente directo
  N=4 pattern; §"scope-outs" Conic riscado por este passo;
  focal_* preservado per Radial-only).
- **ADR-0089** (a criar P267.B; N=5 cumulativo).
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (anotação cumulativa P267 reserva).
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P267/P268).
- ADR-0065 — Inventariar primeiro.
- DEBT-1 — Fechado P142 (preservado).
- P252 — Stroke `overhang` (precedente N=1 cross-cutting).
- P257 — Color 8/8 (precedente N=2 pattern PROPOSTO+IMPL).
- P261 — Paint wrapper Solid only (N=3 pattern).
- **P262** — Gradient Linear L1+stdlib (precedente directo
  N=4 pattern; template literal P267).
- **P263** — Gradient Linear PDF (template literal P268 futuro).
- **P264** — Gradient Radial L1+stdlib (precedente directo
  N=5; **template literal exacto P267**).
- **P265** — Gradient Radial PDF (template literal P268 futuro;
  divisão granularidade N=2).
- P266 — Text audit Fase A (primeiro consumo directo formal
  ADR-0084 + 0085 pós-P260).
- Vanilla `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  (1366 linhas; ConicGradient §1145-1158; 6 campos).
