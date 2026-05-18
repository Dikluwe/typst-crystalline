# Diagnóstico imutável P273.A — Gradient relative RelativeTo cross-variant

**Data**: 2026-05-17.
**Passo**: typst-passo-273.
**Magnitude**: M (cap composto L1 hard 80 + stdlib hard 50 + L3 hard 150 + testes hard 30).
**Cluster**: Visualize / Gradient (activação feature cross-variant).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Décimo quarto consumo directo de fonte** (cristalino + vanilla
`relative: Smart<RelativeTo>` literal + Cairo/Inkscape transform paridade).

---

## §A.1 — Vanilla `relative: Smart<RelativeTo>` literal

`lab/typst-original/crates/typst-library/src/visualize/gradient.rs`:

- L221: `LinearGradient { ... relative: Smart<RelativeTo>, ... }`.
- L309: `RadialGradient { ... relative: Smart<RelativeTo>, ... }`.
- L429: `SweepGradient (Conic) { ... relative: Smart<RelativeTo>, ... }`.
- L1009/1077/1155: 3 vanilla wrapper structs declaram campo.
- L1209: `pub enum RelativeTo { Self_, Parent }` com `#[derive(Cast)]`.

**Confirmação**: vanilla expõe **`relative: Smart<RelativeTo>`** literal cross-variant
(Linear/Radial/Conic).

---

## §A.2 — Vanilla `unwrap_relative` default resolution

`gradient.rs:965-969`:

```rust
pub fn unwrap_relative(&self, on_text: bool) -> RelativeTo {
    self.relative().unwrap_or_else(|| {
        if on_text { RelativeTo::Parent } else { RelativeTo::Self_ }
    })
}
```

**Resolve**:
- `Smart::Custom(rel)` → `rel`.
- `Smart::Auto` + `on_text` → `Parent`.
- `Smart::Auto` + `!on_text` → `Self_` (default em maioria dos contextos).

**Cristalino simplificação**: defaults `None` (Auto) → `Self_` consistente
(divergência intencional: ignorar contexto `on_text` por enquanto;
materializável futuro se necessário). Pattern ADR-0064 §Caso A
(`Smart<T>` → `Option<T>` cristalino).

---

## §A.3 — Vanilla L3 emit usa transform Rust (não PDF /Matrix)

`lab/typst-original/crates/typst-pdf/src/paint.rs:175-181`:

```rust
let (size, offset) = match gradient.unwrap_relative(on_text) {
    RelativeTo::Self_ => (size, offset),
    RelativeTo::Parent => (state.container_size(), Point::zero()),
};
let base_transform = correct_transform(state, gradient.unwrap_relative(on_text));
```

E `correct_transform` (linhas 383+) calcula `Transform` Rust nativo
(não PDF `/Matrix` shading dictionary).

**Confirmação revisada vs spec**: vanilla usa **transform Rust** (paridade
cristalino decisão utilizador). Cristalino **paridade vanilla** — não
divergência. Sub-padrão "Fase A com industry research proactiva" P273
ainda válido pois consolidou Cairo/Inkscape APIs PDF /Matrix existence
(decisão arquitectural informada antes de spec).

Comentário spec §A.3 actualizado: cristalino **paridade vanilla** transform
Rust; PDF `/Matrix` é caminho rejeitado por ambos.

---

## §A.4 — Cristalino L1 estado pré-P273

`01_core/src/entities/gradient.rs`:

```rust
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    pub space: ColorSpace,  // P270
}

pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,  // P269
    pub focal_radius: Ratio,         // P269
    pub space: ColorSpace,           // P270
}

pub struct Conic {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle: Angle,
    pub space: ColorSpace,  // P270
}
```

**Sem campo `relative`**; aditivo cross-variant P273.

---

## §A.5 — Cristalino stdlib named args pré-P273

`01_core/src/rules/stdlib/gradients.rs`:

- `gradient.linear`: `angle`, `space` (+ stops posicionais).
- `gradient.radial`: `center`, `radius`, `focal_center`, `focal_radius`,
  `space` (+ stops).
- `gradient.conic`: `center`, `angle`, `space` (+ stops).

Whitelist named args per fn em `args.named.keys()` loop. Erro para
chave inesperada literal.

**Adicionar named arg `"relative"`** cross-variant P273; whitelist
estendida em 3 funções.

---

## §A.6 — Cristalino L3 dispatcher pós-P272 (estado para extender)

`03_infra/src/export.rs:1497-1622`:

- `fn emit_gradient_objects(grad_objs, page_dimensions, next_sub_id)`.
- 3 callers (lines 1198, 1323, 1480) — passam `page_dimensions` (Vec).
- Dispatcher branch Linear/Radial/Conic + sub-branch CMYK/RGB.
- Coordinates calculadas via `compute_axial_coords(angle, 0, 0, page_w, page_h)`
  (Linear) / `compute_radial_coords(..., page_w, page_h)` (Radial) /
  unit-space Coons patches (Conic; transformados pelo PDF /Matrix do
  Pattern).

**Coords actuais baseados em page dimensions** = comportamento "Parent" implícito
(o "parent" maior é a página). Aditivo P273: dispatcher considera
`relative` para resolver entre Self (shape bbox) vs Parent (page/container).

---

## §A.7 — emit_gradient_objects callers (3 sítios)

- Linha 1198: text paint flush.
- Linha 1323: shape fill.
- Linha 1480: shape stroke.

**Refactor surface area**: 3 callsites + 1 fn signature. Param adicional
`parent_bbox: Option<Rect>` aceitável (cap L3 hard 150 com folga).
Callers actuais passam `None` (default behavior preserved P272).

---

## §A.8 — PROPOSTA L1 enum RelativeTo + Option<RelativeTo>

ADR-0064 §Caso A: vanilla `Smart<T>` → cristalino `Option<T>`
(None = Auto, Some(t) = Custom).

```rust
/// Define a que bounding box o gradient é relativo (paridade vanilla
/// `RelativeTo`). Default `Self_` (consistente entre contextos não-text;
/// vanilla `on_text` → `Parent` divergência intencional cristalino
/// adiada).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelativeTo {
    #[default]
    Self_,   // Self é palavra reservada Rust; trailing underscore.
    Parent,
}
```

Cada variant ganha campo `relative: Option<RelativeTo>`:

```rust
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,  // P273; None = Auto = Self_
}
// Análogo Radial + Conic.
```

---

## §A.9 — PROPOSTA stdlib named arg `"relative"`

```rust
fn parse_relative_named(args: &Args, fn_name: &str)
    -> SourceResult<Option<RelativeTo>>
{
    match args.named.get("relative") {
        None => Ok(None),  // Auto
        Some(Value::Str(s)) => match s.as_str() {
            "self"   => Ok(Some(RelativeTo::Self_)),
            "parent" => Ok(Some(RelativeTo::Parent)),
            "auto"   => Ok(None),
            other => Err(...),
        },
        Some(other) => Err(...),
    }
}
```

Whitelist estendida cross-variant; 3 callsites adicionam `"relative"`.

---

## §A.10 — PROPOSTA contexto Rust parent bbox

`emit_gradient_objects` mantém assinatura actual (mantém compatibilidade
3 callsites preservada). Helper `apply_parent_transform(coords,
parent_bbox)` é estrutural — quando `parent_bbox = None`, retorna
coords inalteradas (= Self behavior).

Materialização L3 minimal: structural — dispatcher resolve `relative`
mas com `parent_bbox = None` (futuro callsite poderá passar bbox real;
estrutura preparada).

---

## §A.11 — PROPOSTA L3 dispatcher refactor

```rust
// L3 export.rs P273
fn resolve_relative(relative: Option<RelativeTo>) -> RelativeTo {
    relative.unwrap_or_default()  // None → Self_
}

// Helper apply transform (structural; parent_bbox = None ⇒ identity)
fn apply_parent_transform(
    local: (f32, f32, f32, f32),
    parent_bbox: Option<(f32, f32, f32, f32)>,
) -> (f32, f32, f32, f32) {
    match parent_bbox {
        Some((px0, py0, px1, py1)) => {
            // Escalar local (unit space) para parent bbox.
            let dx = px1 - px0;
            let dy = py1 - py0;
            (px0 + local.0 * dx, py0 + local.1 * dy,
             px0 + local.2 * dx, py0 + local.3 * dy)
        }
        None => local,
    }
}

// Dispatcher Linear branch (paridade Radial/Conic structural):
let relative = resolve_relative(linear.relative);
let parent_bbox: Option<(f32, f32, f32, f32)> = None;  // P273 structural; future callsite preenche.
let (x0, y0, x1, y1) = compute_axial_coords(
    linear.angle.to_rad(), 0.0, 0.0, page_w, page_h);
let (x0, y0, x1, y1) = match relative {
    RelativeTo::Self_  => (x0, y0, x1, y1),  // P272 literal preserved.
    RelativeTo::Parent => apply_parent_transform((x0 as f32, y0 as f32, x1 as f32, y1 as f32), parent_bbox)
        .into(),  // None parent_bbox → identity.
};
```

---

## §A.12 — ADR-0029 pureza física L1 verificação

- Campo `relative: Option<RelativeTo>` é pura metadata enum.
- `Linear::sample(t)` / `Radial::sample(t)` / `Conic::sample(t)`
  **não usa** o campo (interpolação em 1D só depende de stops + space).
- Sem I/O, sem chamadas externas, sem state mutável.

✓ **Pureza física L1 preserved** absoluta.

---

## §A.13 — Defaults preservam P272 bit-exact

- Construtores existentes (`linear`, `linear_with_space`, `radial`,
  `radial_with_focal`, `radial_with_space`, `conic`, `conic_with_space`)
  preservam assinatura literal — adicionam `relative: None` default.
- Stdlib parsing default `relative = None` quando arg omitido.
- L3 dispatcher: `resolve_relative(None) = Self_` → branch literal P272.
- `parent_bbox = None` (estrutural) → `apply_parent_transform` identity.

✓ **2557 baseline P272 preserved bit-exact**.

---

## §A.14 — Cenário detectado

**Cenário B1 fecho conceptual** — refino cross-variant trivial; pattern
N=3 cumulativo (P269 focal_*, P270 space, **P273 relative**); pipelines
L1+stdlib+L3 estendidos minimamente sem invadir lógica existente.

Cap LOC L1 hard 80 / soft 50: estimativa ~50-60 LOC (folga 25-37%).
Cap LOC stdlib hard 50 / soft 30: estimativa ~40-50 LOC (folga 0-20%).
Cap LOC L3 hard 150 / soft 100: estimativa ~60-90 LOC (folga 67-80%).

---

## §A.15 — Estimativa cap LOC

- **L1**: ~50-60 LOC (enum + 3 fields + 3 construtores helpers ou
  inline defaults).
- **Stdlib**: ~40-50 LOC (parse_relative_named + 3 chamadas + whitelist
  estendida + 1 default per fn).
- **L3**: ~60-90 LOC (resolve_relative + apply_parent_transform + 3
  dispatcher branches em Linear/Radial/Conic; CMYK/RGB já estão dentro
  de cada).
- **Tests**: ~20-25 tests (5 L1 + 6 stdlib + 5 L3 + 4 E2E ≈ 20).

---

## §A.16 — Decisão arquitectural

**Cluster Gradient ganha runtime field cross-variant `relative`**:

- L1 campo `relative: Option<RelativeTo>` cross-variant 3/3.
- Stdlib named arg `"relative"` paridade.
- L3 dispatcher resolve + apply_parent_transform (estrutural; None
  branch identity).

**Cristalino paridade vanilla strategy** (transform Rust; não PDF
/Matrix). Industry research P273 consolidada confirma:
- PDF `/Matrix` existe (iText/PDFTron APIs) mas usado raramente.
- Cairo/Inkscape e vanilla usam transform Rust nativo.
- Cristalino paridade simplifica auditoria pipeline.

**Sub-padrão "ADR REVOGADO + substituta" não aplicável P273** —
refino aditivo cross-variant; ADRs preservadas literal.

**Sub-padrão "Anotação cumulativa em vez de ADR nova" N=11 → N=12
cumulativo** — pattern canónico continua.

**Sub-padrão "Fase A com industry research proactiva" N=4 → N=5
cumulativo limiar formalização clara muito ultrapassado** — confirma
valor metodológico ADR-0094 Pattern 3.

**Sub-padrão "Aplicação meta-ADR (ADR-0094)" N=1 → N=2 cumulativo** —
segunda aplicação prática Cap LOC hard/soft Pattern 1 + Industry
research Pattern 3 pós-formalização P271.

---

*Diagnóstico imutável produzido em 2026-05-17 P273.A. Linhagem
empírica preservada como evidência ADR-0085 + auto-aplicação
ADR-0065. Décimo quarto consumo directo de fonte (vanilla +
cristalino + industry consolidação P273).*
