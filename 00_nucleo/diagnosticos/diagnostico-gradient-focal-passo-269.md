# Diagnóstico — Gradient Radial focal_* activação (P269.A)

**Status**: imutável após criação (per ADR-0085).
**Data**: 2026-05-15.
**Passo**: P269 (activação focal_center + focal_radius Radial L1+stdlib+PDF).
**Sétimo consumo directo de fonte** vanilla (P262 Linear + P264 Radial + P267 Conic + P268 PDF Conic + P268.1 web industry + P268.2 literatura perceptual + **P269 vanilla Radial focal**).
**Origem**: spec `00_nucleo/materialization/typst-passo-269.md` §1.

---

## §A.1 — Vanilla RadialGradient shape (campos exactos)

`lab/typst-original/crates/typst-library/src/visualize/gradient.rs:1063` —
struct `RadialGradient` com 8 campos:

```rust
pub struct RadialGradient {
    pub stops:        Vec<(Color, Ratio)>,
    pub center:       Axes<Ratio>,
    pub radius:       Ratio,
    pub focal_center: Axes<Ratio>,        // L:334 Smart<Axes<Ratio>>; resolvido Auto → center
    pub focal_radius: Ratio,              // L:343 Spanned<Ratio>; default 0%
    pub space:        ColorSpace,
    pub relative:     Smart<RelativeTo>,
    pub anti_alias:   bool,
}
```

### Validações vanilla §330-370

1. **`focal_radius.v > radius.v` → erro** (`"focal radius must be smaller than gradient radius"`).
2. **`(focal_center - center).hypot() >= (radius - focal_radius)` → erro**
   (`"focal circle must be inside the gradient circle"`).
3. `focal_center: Smart<Axes<Ratio>>` resolvido `unwrap_or(center)` literal.
4. `focal_radius` default `Ratio(0.0)`.

---

## §A.2 — Vanilla stdlib `gradient.radial` named args

`lab/typst-original/.../visualize/gradient.rs:334-343`:

```rust
focal_center: Smart<Axes<Ratio>>,    // default Auto → resolved center
focal_radius: Spanned<Ratio>,        // default 0%
```

Demais: `center: Smart<Axes<Ratio>>` (default Auto → `(50%, 50%)`),
`radius: Smart<Ratio>` (default Auto → `50%`).

---

## §A.3 — Vanilla PDF Radial Type 3 emit literal

`lab/typst-original/crates/typst-pdf/src/paint.rs:220-240`:

```rust
Gradient::Radial(radial) => {
    let radial = RadialGradient {
        fx: radial.focal_center.x.get() as f32,
        fy: radial.focal_center.y.get() as f32,
        fr: radial.focal_radius.get() as f32,
        cx: radial.center.x.get() as f32,
        cy: radial.center.y.get() as f32,
        cr: radial.radius.get() as f32,
        ...
    };
    (radial.into(), 255)
}
```

Krilla `RadialGradient` é abstracção que produz `/ShadingType 3`
PDF nativo com `/Coords [fx fy fr cx cy cr]` (focal_circle, outer_circle).

---

## §A.4 — Cristalino L1 Radial actual (P264; 3 campos)

`01_core/src/entities/gradient.rs:210-216`:

```rust
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    // focal_center: Axes<Ratio>,   // scope-out ADR-0088 — default = center
    // focal_radius: Ratio,         // scope-out — default 0%
}
```

**focal_* estão COMENTADOS como scope-out** (linhas 214-215);
não existem como `Option<>` nem como campos vazios. P269 activa-os
como campos pub `Axes<Ratio>` + `Ratio` directos (não-opcional;
default vem do construtor).

---

## §A.5 — Cristalino L1 `Radial::sample(t)` (matemática 1D)

`01_core/src/entities/gradient.rs:260-276` — `sample(t: f32) → Color`
é **interpolação Oklab 1D pura sobre offsets dos stops**; NÃO usa
focal nem coordenadas 2D.

### Comparação com vanilla

Vanilla tem **dois métodos** sample:
1. `sample(RatioOrAngle)` — 1D análogo cristalino (sem focal).
2. `sample_at(x, y, w, h)` — 2D point→t conversion (`gradient.rs:911-932`);
   usa focal para conversão 2-circle (linhas 913-931).

Cristalino **só implementa `sample(t)` 1D** (paridade vanilla método 1).
**Cristalino NÃO implementa `sample_at(x, y)`** — não é necessário
porque PDF emit usa `/ShadingType 3` nativo (o reader PDF computa a
conversão 2-circle internamente via /Coords).

**Decisão arquitectural §A.5**: `Radial::sample(t)` **NÃO muda em
P269**. Focal só afecta:
- L3 PDF /Coords (Type 3 nativo).
- (Hipotético) `Radial::sample_at(x, y)` futuro — fora do scope P269.

**§política condição 2 não accionada** — sample 1D preservado literal
P264.

---

## §A.6 — Cristalino stdlib `native_gradient_radial` actual

`01_core/src/rules/stdlib/gradients.rs:146-203` — named args:
- `center: Array [Ratio, Ratio]` (default `(50%, 50%)`).
- `radius: Ratio` (default `50%`).

Validação rejeita named args fora de `{center, radius}` (linha 193-200).

**Alteração P269 esperada**:
- +2 named args `focal_center: Array [Ratio, Ratio]` (default = center).
- +1 named arg `focal_radius: Ratio` (default `0%`).
- Whitelist linha 193 estende para `{center, radius, focal_center, focal_radius}`.
- 2 validações vanilla §A.1 portadas: `focal_radius > radius` erro;
  `dist(focal_center, center) >= radius - focal_radius` erro.

Estimativa LOC: ~25-35 (≤ 40 cap).

---

## §A.7 — DECISÃO CENTRAL — Cristalino L3 emit Radial trivial vs estrutural

`03_infra/src/export.rs:486-497`:

```rust
fn compute_radial_coords(
    center: Axes<Ratio>,
    radius: Ratio,
    w: f64, h: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let cx = center.x.0 * w;
    let cy = center.y.0 * h;
    let r = radius.0 * w.min(h);
    // Subset: focal point pontual no center; target concêntrico.
    (cx, cy, 0.0, cx, cy, r)
}
```

`03_infra/src/export.rs:1216-1232` — callsite emit Radial:

```rust
GradientObjectKind::Radial(radial) => {
    let stops = oklab_sample_stops_radial(radial, 16);
    let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
        radial.center, radial.radius, page_w, page_h);
    let shading_dict = format!(
        "<< /ShadingType 3 /ColorSpace /DeviceRGB \
           /Coords [{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}] \
           /Function {} 0 R /Extend [true true] >>",
        x0, y0, r0, x1, y1, r1, function_id,
    );
    ...
}
```

### Cenário detectado: **B1 TRIVIAL**

- `compute_radial_coords` retorna `(cx, cy, 0.0, cx, cy, r)` literal.
- Alteração: aceitar `focal_center: Axes<Ratio>` + `focal_radius: Ratio`
  como parâmetros adicionais; computar `fx = focal_center.x.0 * w`,
  `fy = focal_center.y.0 * h`, `fr = focal_radius.0 * w.min(h)`.
- Resultado: `(fx, fy, fr, cx, cy, r)`.
- Callsite passa `radial.focal_center` + `radial.focal_radius`.

**Estimativa LOC**: ~15-25 (≤ 60 cap).

**§política condição 1 NÃO accionada** — divisão P269+P270
desnecessária; P269 absorve L1+stdlib+PDF em magnitude M total.

---

## §A.8 — Helpers Oklab cristalinos reutilizáveis literal

- `oklab_sample_stops_radial(radial, n)` (`03_infra/src/export.rs:502`)
  — reutiliza `Radial::sample(t)`; focal NÃO entra; intacto.
- `interpolate_oklab` (`01_core/src/entities/gradient.rs:142`) — usado
  por `Radial::sample(t)`; intacto.
- `color_to_oklab_with_alpha` (`01_core/src/entities/gradient.rs:163`,
  `pub` desde P268.2) — intacto.

Sub-padrão "Reutilização literal helpers cross-passos" N=4 → **N=5
cumulativo** (P265/P267/P268/P268.2/**P269**); estende sem mudar
helpers.

---

## §A.9 — Gap a fechar

### L1 (~30-45 LOC estimadas)

- `Radial` struct: 2 campos novos `pub focal_center: Axes<Ratio>` +
  `pub focal_radius: Ratio`.
- `Gradient::radial(stops, center, radius)` actualizado: fill
  `focal_center: center, focal_radius: Ratio(0.0)` (preserva P264).
- `Gradient::radial_with_focal(stops, center, radius, focal_center,
  focal_radius)` construtor novo.
- 4 sites internos `Radial { stops, center, radius }` em testes P264
  (`gradient.rs:688/705/724/751`) actualizados com `focal_center: center,
  focal_radius: Ratio(0.0)` adicionados (paridade comportamento; só
  syntax de construção).

### Stdlib (~25-35 LOC estimadas)

- `native_gradient_radial`: 2 named args + 2 validações vanilla
  portadas.
- Whitelist named args estendida.

### L3 (~20-30 LOC estimadas)

- `compute_radial_coords`: 2 args novos `focal_center` + `focal_radius`.
- Callsite `emit_gradient_objects` Radial branch passa
  `radial.focal_center` + `radial.focal_radius`.
- 5 sites de teste P265 em `export.rs:3086/3115/3172/3227/3460/3772`
  com `Radial { ... }` literal updated.

### Total estimado: ~75-110 LOC; cap 250 (150+40+60); folga ~140 LOC

---

## §A.10 — Cenário detectado: B1 — fecho conceptual

**B1 confirmado** (§A.7): PDF /Coords trivial-aware; absorve L1+stdlib+PDF
em P269. **B2 NÃO accionado** (§política condição 1 não dispara).

---

## §A.11 — Decisão arquitectural — defaults preservam P264

- `Gradient::radial(stops, center, radius)` mantém assinatura SEM
  focal_*; internamente seta `focal_center: center, focal_radius: 0`.
- Todos os call sites P264 produzem **bytes PDF idênticos** pós-P269
  (focal=center, fr=0 colapsa /Coords para `[cx cy 0 cx cy r]` igual
  P265).
- Stdlib `gradient.radial(...)` sem named args focal_* → P264 behavior.
- L3 `emit_gradient_objects` produz mesma shading dict para defaults.

**§política condições 9 + 11 satisfeitas** — regressão P264/P265 zero
esperada.

### Vanilla validation §A.12

- Cristalino default `focal_center = center` (struct field).
- Vanilla default `focal_center: Smart::Auto` resolvido `unwrap_or(center)`.
- Comportamento idêntico (`focal_center == center` ambos casos);
  **§política condição 12 não accionada**.

- Cristalino default `focal_radius = Ratio(0.0)`.
- Vanilla default `focal_radius: Ratio(0.0)`.
- Idêntico.

---

## §A.12 — Validações stdlib portadas

P269 stdlib porta 2 validações vanilla §A.1:

1. `focal_radius.0 > radius.0` → erro
   `"gradient.radial(focal_radius): {fr} > radius {r}"`.
2. `(focal_center.x - center.x)² + (focal_center.y - center.y)² >=
   (radius - focal_radius)²` → erro
   `"gradient.radial: focal circle must be inside outer circle"`.

L1 NÃO valida (Radial é struct dados; validação no consumer stdlib).

---

## §A.13 — Cristalino tests P264/P265 existentes (regressão obrigatória)

Tests P264 actuais (9 testes):
- `p264_radial_construcao_2_stops`
- `p264_radial_first_stop_color`
- `p264_radial_clone_arc_o1`
- `p264_radial_partial_eq`
- `p264_radial_effective_offsets_auto_spacing`
- `p264_radial_sample_extremos`
- `p264_radial_sample_clamp_above_1`
- `p264_gradient_radial_to_paint_via_from`
- `p264_radial_center_non_default`

Tests P265 actuais (7 testes):
- `p265_compute_radial_coords_center_default`
- `p265_compute_radial_coords_center_offset`
- `p265_compute_radial_coords_non_square_uses_min_dim`
- (e mais 4 E2E PDF tests)

**Total 16 tests P264/P265 devem permanecer verdes literal**
pós-activação focal_*.

**Adaptação necessária**: tests que constroem `Radial { ... }`
literal precisam adicionar `focal_center: center, focal_radius:
Ratio(0.0)` aos campos. Comportamento (assertions) preservado
literal — só syntax de construção muda. §política condição 9
satisfeita: 0 falhas de assertion.

---

## §A.14 — Sumário decisões diagnóstico

| Item | Decisão |
|---|---|
| §A.1 Vanilla 8 campos | Cristalino activa 2 (focal_center + focal_radius); demais (space/relative/anti_alias) preservam scope-out P264 |
| §A.5 `Radial::sample(t)` 1D | **Não muda em P269** (focal só afecta /Coords PDF) |
| §A.7 Cenário B1 trivial | PDF /Coords aceita focal natively → P269 absorve L1+stdlib+PDF |
| §A.9 Gap LOC | ~75-110 (cap 250; folga ~140) |
| §A.10 B1 confirmado | Sem divisão P269+P270 |
| §A.11 Defaults preservam P264 | Zero regressão esperada |
| §A.12 Validações stdlib | 2 erros (focal_radius>radius; focal fora outer) |
| §A.13 Tests P264/P265 | 16 tests preservados; só struct literal sites atualizados (mecânico) |
| §A.14 Magnitude P269 | M dentro do cap; viável absorver L1+stdlib+PDF |

**Diagnóstico aprovado para passagem a sub-passo P269.B (anotações
cumulativas + L0).**

---

## §A.15 — Referências

- Spec P269: `00_nucleo/materialization/typst-passo-269.md`.
- Vanilla L1: `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`.
- Vanilla L3 PDF: `lab/typst-original/crates/typst-pdf/src/paint.rs:220-240`.
- Cristalino L1: `01_core/src/entities/gradient.rs:210-216` (Radial actual).
- Cristalino stdlib: `01_core/src/rules/stdlib/gradients.rs:146-203`.
- Cristalino L3: `03_infra/src/export.rs:486-497` (`compute_radial_coords`)
  + `1216-1232` (callsite emit Radial).
- ADR-0088 — Gradient Radial-only L1+stdlib (anotação cumulativa
  P269 pendente §2 spec).
- ADR-0054 — Perfil graded DEBT-1 (anotação cumulativa P269 pendente).
- ADR-0085 — Diagnóstico imutável (este ficheiro produzido per
  ADR-0085; sétimo consumo directo de fonte vanilla).
- P264 — Gradient Radial L1+stdlib (precedente directo extendido).
- P265 — PDF Radial /ShadingType 3 (template emit).
- P267 — Gradient Conic (precedente "ADR scope-out revogado parcialmente";
  pattern reutilizado P269).
- P268/P268.2 — PDF Conic + adaptive N (preservados intactos).
