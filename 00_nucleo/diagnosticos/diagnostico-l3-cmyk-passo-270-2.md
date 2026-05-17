# Diagnóstico — L3 emit CMYK directo `/DeviceCMYK` (P270.2.A)

**Status**: imutável após criação (per ADR-0085).
**Data**: 2026-05-17.
**Passo**: P270.2 (L3 CMYK directo; fecha cluster Gradient L3 8/8 spaces).
**Décimo consumo directo de fonte** vanilla (P262/P264/P267/P268/P268.1/P268.2/P269/P270/P270.1 + **P270.2 vanilla CMYK emit literal**).
**Origem**: spec `00_nucleo/materialization/typst-passo-270.2.md` §1.

---

## §A.1 — Cristalino Color CMYK API

`01_core/src/entities/color.rs`:

```rust
// Variant
pub enum Color {
    // ... (P257 8 variants)
    Cmyk { c: f32, m: f32, y: f32, k: f32 },
}

// Construtor
pub fn cmyk(c: f32, m: f32, y: f32, k: f32) -> Self {
    Self::Cmyk { c, m, y, k }
}

// to_rgba_f32 (P257; CMYK → sRGB conversão forward)
Self::Cmyk { c, m, y, k } => {
    let r = (1.0 - c) * (1.0 - k);
    let g = (1.0 - m) * (1.0 - k);
    let b = (1.0 - y) * (1.0 - k);
    (r, g, b, 1.0)
}
```

**`Color::to_cmyk_f32` não existe como método público**. Para extrair
componentes CMYK directos, cristalino usa:
- `gradient::to_cmyk_components(c)` (P270 helper L1; gradient.rs:353).
  Lossless quando `c` já é Color::Cmyk; lossy via sRGB intermediate
  caso contrário.

**Acessibilidade L3**: `to_cmyk_components` é `fn` privada em
gradient.rs. **Gap**: precisa ser `pub` ou L3 precisa caminho
alternativo.

**Decisão arquitectural §A.1**: padrão análogo P270.1 — usar
`<variant>.sample(t)` que retorna `Color` no space escolhido (arm
CMYK do dispatcher P270). Para extrair CMYK directo da `Color`
result, fazer pattern-match em `Color::Cmyk { c, m, y, k }` em L3.
Se Color é outra variant (sample no space arm CMYK *deveria*
retornar Color::Cmyk, mas precaução): fallback via
`to_rgba_f32()` → conversão sRGB→CMYK inline.

**Alternativa cleaner**: promover `to_cmyk_components` a `pub`
(precedente P268.2 `color_to_oklab_with_alpha`); ~4 caracteres
L1 change.

**Decisão final**: promover `to_cmyk_components` a `pub` em
`gradient.rs` (paridade P268.2 pattern; gap minimal).

---

## §A.2 — Cristalino L3 actual CMYK comportamento (pré-P270.2)

`03_infra/src/export.rs` linhas 1209-1245 (Linear branch),
1247-1268 (Radial branch), 1269+ (Conic branch):

Pipeline actual:
1. `multispace_sample_stops_<variant>(...)` retorna `Vec<(f32, f32, f32)>`
   sRGB normalizado.
2. `emit_function_dict(stops, ...)` produz `/FunctionType 2`/`3` com
   `/C0 [r g b]` `/C1 [r g b]` 3-component.
3. Shading dict emit com `/ColorSpace /DeviceRGB`.

**Comportamento CMYK pré-P270.2**: `<variant>.space = Cmyk` produz
`Color::Cmyk` via dispatcher P270 → `to_rgba_f32()` converte para
sRGB → emit DeviceRGB. **Gama CMYK perdida**.

§A.2 confirma divergência observable de vanilla user-intent.

---

## §A.3 — Cristalino dispatcher `interpolate_in_space` arm Cmyk

`01_core/src/entities/gradient.rs` linha 432:

```rust
fn interpolate_cmyk(c0: Color, c1: Color, t: f32) -> Color {
    let (c0_, m0, y0, k0) = to_cmyk_components(c0);
    let (c1_, m1, y1, k1) = to_cmyk_components(c1);
    Color::cmyk(lerp(c0_, c1_, t), lerp(m0, m1, t), lerp(y0, y1, t), lerp(k0, k1, t))
}
```

Arm Cmyk do dispatcher chama `interpolate_cmyk` (P270 — linha 350).
Retorna `Color::Cmyk { c, m, y, k }` interpolado.

§A.3 confirma — dispatcher funcional. **Sem gap**.

---

## §A.4 — Vanilla typst CMYK emit literal

Vanilla typst PDF emit usa krilla:
- `/ColorSpace /DeviceCMYK` no shading dictionary.
- `/Function FunctionType 2`/`3` com 4 outputs.
- `/Range [0 1 0 1 0 1 0 1]` (8 values; 4 pares c/m/y/k).
- `/C0 [c m y k]` `/C1 [c m y k]` 4-component.

**Bug vanilla #4422**: dictionary `/ColorSpace /DeviceRGB` quando
deveria ser `/DeviceCMYK` em casos específicos (krilla ou wrapper
intermediário). pdfkit #532 análogo confirma causa raiz universal.

**Cristalino implementação correcta resolve por construção** — emit
`/DeviceCMYK` directo no shading dictionary.

---

## §A.5 — Vanilla Conic CMYK comportamento

Vanilla Conic via krilla `SweepGradient` (ADR-0090 §"Pesquisa
empírica industry"). Krilla actual estratégia interna opaca; CMYK
support em Conic não verificado literal.

PDF spec `/ShadingType 4` com `/ColorSpace /DeviceCMYK` é
**permitido** mas **suporte reader pode variar**:
- Adobe Reader: suporta.
- pdf.js (Firefox): suporta Type 4 RGB; CMYK incerto.
- mupdf/pdftoppm: suporta.

**Cenário B decidido**: Conic CMYK preserved scope-out P270.2;
candidato futuro P-Gradient-Conic-CMYK. Justificativas:
- Conic Type 4 Gouraud + CMYK adiciona complexidade (stream
  binary 4 bytes/vertex; `/Decode` array 5 pares vs 4).
- Suporte reader incerto.
- Linear + Radial CMYK cobrem maioria dos use cases user-facing.

Conic com `space: Cmyk` em P270.2 mantém pipeline P270.1 fallback
(sample CMYK convert para sRGB via `to_rgba_f32` no helper
`multispace_sample_stops_conic`). Sub-óptimo mas funcional.

---

## §A.6 — PROPOSTA L3 estrutura CMYK branch (Cenário B)

```rust
// Helpers samplers CMYK 4-component (2 variants: Linear+Radial)
fn multispace_sample_stops_linear_cmyk(linear: &Linear, n: usize)
    -> Vec<(f32, f32, f32, f32)>
{
    let n = n.max(2);
    (0..n).map(|i| {
        let t = i as f32 / (n - 1) as f32;
        let c = linear.sample(t);  // P270 dispatcher arm Cmyk
        let (cy, m, y, k) = match c {
            Color::Cmyk { c, m, y, k } => (c, m, y, k),
            // Fallback se dispatcher retorna outra variant (precaução)
            _ => {
                let (r, g, b, _) = c.to_rgba_f32();
                rgb_to_cmyk(r, g, b)
            }
        };
        (cy.clamp(0.0, 1.0), m.clamp(0.0, 1.0),
         y.clamp(0.0, 1.0), k.clamp(0.0, 1.0))
    }).collect()
}

// Análogo multispace_sample_stops_radial_cmyk
```

**rgb_to_cmyk inline** (helper privado):
```rust
fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> (f32, f32, f32, f32) {
    let k = 1.0 - r.max(g).max(b);
    if k >= 1.0 - 1e-6 {
        (0.0, 0.0, 0.0, 1.0)
    } else {
        let denom = 1.0 - k;
        let c = (1.0 - r - k) / denom;
        let m = (1.0 - g - k) / denom;
        let y = (1.0 - b - k) / denom;
        (c, m, y, k)
    }
}
```

**`emit_function_dict_cmyk`** novo (4-component):
```rust
fn emit_function_dict_cmyk(
    stops: &[(f32, f32, f32, f32)],
    function_id: usize,
    sub_first_id: &mut usize,
) -> (String, Vec<(usize, String)>) {
    // Análogo emit_function_dict mas 4-component:
    // /C0 [c m y k] /C1 [c m y k]
    // /Range [0 1 0 1 0 1 0 1] (8 values; 4 pares)
    // ...
}
```

**Dispatcher dual em `emit_gradient_objects`**:
```rust
GradientObjectKind::Linear(linear) => {
    if linear.space == ColorSpace::Cmyk {
        // P270.2 CMYK branch novo
        let stops = multispace_sample_stops_linear_cmyk(linear, 16);
        let (func_dict, sub_objs) = emit_function_dict_cmyk(&stops, ...);
        let shading_dict = format!(
            "<< /ShadingType 2 /ColorSpace /DeviceCMYK ...>>", ...);
        // ...
    } else {
        // P270.1 pipeline literal preserved
        let stops = multispace_sample_stops(linear, 16);
        let (func_dict, sub_objs) = emit_function_dict(&stops, ...);
        let shading_dict = format!(
            "<< /ShadingType 2 /ColorSpace /DeviceRGB ...>>", ...);
        // ...
    }
}

// Análogo Radial.
// Conic: preserved P270.1 literal (sub-óptimo CMYK fallback;
// scope-out preserved P-Gradient-Conic-CMYK).
```

---

## §A.7 — Bug #4422 e pdfkit #532 — causa raiz factual

**Causa raiz**: PDF shading dictionary `/ColorSpace` errado
(`/DeviceRGB` em vez de `/DeviceCMYK`) quando stops são CMYK.

**Cristalino implementação correcta por construção**:
- `<variant>.space == ColorSpace::Cmyk` → emit
  `/ColorSpace /DeviceCMYK` literal.
- Function 4-component output (`/Range [0 1 0 1 0 1 0 1]`).
- Sample stops preservados como `(c, m, y, k)` sem conversão sRGB
  intermediate.

**Sem ICC profiles** — cristalino preserva scope-out P-Gradient-CMYK-ICC
(refino futuro candidato; paridade krilla custom ICC profiles).

---

## §A.8 — Decisão Conic CMYK — Cenário B (scope-out preserved)

**Cenário B confirmado** (justificado §A.5):
- Linear CMYK materializado P270.2.
- Radial CMYK materializado P270.2.
- **Conic CMYK preserved scope-out P270.2** — candidato futuro
  P-Gradient-Conic-CMYK.

Razões:
1. Vanilla Conic CMYK suporte incerto (krilla opaco).
2. PDF reader compatibility Type 4 Gouraud + CMYK incerto
   (Adobe sim; pdf.js incerto).
3. Complexidade stream binary 4 bytes/vertex + Decode array
   5 pares adiciona ~50 LOC L3.
4. Linear + Radial cobrem maioria dos use cases user-facing
   (Conic é menos comum).

**Conic com `space: Cmyk` em P270.2**: pipeline P270.1 fallback
preservado (sample CMYK convert para sRGB via `Conic::sample(t)` +
`to_rgba_f32()`). Sub-óptimo mas funcional; user pode usar
gradient mas gama CMYK perdida no emit.

---

## §A.9 — Estimativa cap LOC P270.2 (Cenário B)

| Componente | LOC estimado | Cap hard | Cap soft |
|---|---|---|---|
| Promover `to_cmyk_components` a `pub` | 1 keyword L1 | — | — |
| `rgb_to_cmyk` helper L3 | ~12 | 250 | 150 |
| `multispace_sample_stops_linear_cmyk` | ~18 | | |
| `multispace_sample_stops_radial_cmyk` | ~18 | | |
| `emit_function_dict_cmyk` | ~40 | | |
| Dispatcher Linear branch CMYK | ~25 | | |
| Dispatcher Radial branch CMYK | ~25 | | |
| **L3 total** | **~138** | **250 (folga 45%)** | **150 (folga 8%)** |
| Tests (12) | ~150-200 | 35 testes | 25 testes |

L3 production ~138 LOC. **Cap hard L3 250 respeitado**. **Cap
soft L3 150 ligeiramente acima** (~8 LOC sobre); regista no
relatório (sub-padrão "Cap LOC hard vs soft" P270.1 inaugurou).

Tests 12 (Cenário B). **Cap hard testes 35 respeitado** (folga
66%). **Cap soft testes 25 respeitado** (folga 52%).

---

## §A.10 — Defaults preservam P270.1 bit-exact

- `space != ColorSpace::Cmyk`: dispatcher dual entra arm "else" →
  pipeline P270.1 literal preservado.
- `space == ColorSpace::Cmyk`: branch CMYK novo (não-default;
  apenas users que escolherem explícito).
- 2533 baseline tests P262-P270.1 preservados.

§política condições 4 + 7 + 9 satisfeitas.

---

## §A.11 — Cenário detectado: **B confirmado**

**Cenário B**: Linear+Radial CMYK materializados; Conic CMYK
scope-out preserved.

- Cluster L3 emit pós-P270.2: 7/8 spaces full + Linear/Radial CMYK
  + Conic CMYK fallback sub-óptimo. Approximadamente "8/8
  conceptual" mas Conic CMYK não tem `/DeviceCMYK` directo.
- ADR-0083 §"DeviceCMYK PDF" revogação parcial (não final 100%) —
  documentar explicitamente cobertura Linear+Radial vs Conic
  scope-out.

§política condição 10 não accionada (Cenário B é decisão
arquitectural consciente; não failure mode).

---

## §A.12 — Decisão arquitectural — DeviceCMYK directo sem ICC

ADR-0091 §"Scope-outs preserved" preservado:
- ICC profiles para CMYK PDF/A compliance: scope-out P270.2.
- Cristalino emit `/DeviceCMYK` directo sem ICC profile.
- Refino futuro candidato P-Gradient-CMYK-ICC (paridade krilla
  custom ICC profiles).

§política condição 12 (PDF/A compliance) não accionada
explicitamente — cristalino não declara PDF/A compliance per ADR-0090.

---

## §A.13 — ADR-0083 §DeviceCMYK revogação **parcial** (não final 100%)

Revisão da spec original P270.2:
- Spec assumiu Cenário A (revogação final 100% para Linear+Radial+Conic).
- Diagnóstico §A.8 decide Cenário B — Conic CMYK preserved scope-out.
- Consequência: ADR-0083 §"DeviceCMYK PDF" revogação **parcial**
  (Linear+Radial materializados; Conic preserved scope-out).
- ADR-0083 atualização: §"DeviceCMYK PDF" → "Linear+Radial
  materializados P270.2; Conic CMYK scope-out preserved P-Gradient-Conic-CMYK
  futuro".

**Sub-padrão "ADR scope-out revogado parcialmente"** N=3 → N=4
cumulativo conta P270.2 (mesmo sendo "parcial" do que originalmente
planeado; semanticamente é revogação substantiva — 2/3 variants
materializados em CMYK).

---

## §A.14 — Sumário decisões diagnóstico

| Item | Decisão |
|---|---|
| §A.1 `Color::to_cmyk_f32` API | Usar `Color::Cmyk` pattern-match + `to_cmyk_components` promovido a `pub` |
| §A.2 L3 actual CMYK | Convertido sRGB sub-óptimo (P270.1 baseline) |
| §A.3 Dispatcher arm Cmyk | Funcional sem gap (P270 already provides) |
| §A.4 Vanilla CMYK emit | Padrão `/DeviceCMYK` + Function 4-component |
| §A.5 Conic CMYK vanilla | Suporte reader incerto |
| §A.6 PROPOSTA L3 | Helpers CMYK 4-component + dispatcher dual |
| §A.7 Bug #4422 causa raiz | `/ColorSpace` errado; cristalino resolve por construção |
| §A.8 Conic CMYK Cenário B | Scope-out preserved (P-Gradient-Conic-CMYK futuro) |
| §A.9 Cap LOC | ~138 LOC L3 (cap hard 250; folga 45%) |
| §A.10 Defaults preservam P270.1 | Verificado via space != Cmyk arm "else" |
| §A.11 Cenário B | Confirmado |
| §A.12 ICC scope-out | Preserved P270.2; refino futuro candidato |
| §A.13 ADR-0083 revogação | **Parcial** (Linear+Radial; Conic preserved) |

**Diagnóstico aprovado para passagem a sub-passo P270.2.B (anotações
cumulativas).**

---

## §A.15 — Referências

- Spec P270.2: `00_nucleo/materialization/typst-passo-270.2.md`.
- Cristalino L1 P257 Color: `01_core/src/entities/color.rs:32-262`.
- Cristalino L1 P270 dispatcher: `01_core/src/entities/gradient.rs:332-440`
  (interpolate_in_space + 7 helpers + interpolate_cmyk).
- Cristalino L3 P270.1: `03_infra/src/export.rs:458-543` (3 helpers
  multispace_sample_stops_*).
- Cristalino L3 emit_function_dict: `03_infra/src/export.rs:734-777`.
- Cristalino L3 emit_gradient_objects: `03_infra/src/export.rs:1198+`.
- Vanilla typst PDF emit: `lab/typst-original/crates/typst-pdf/src/`
  (CMYK via krilla).
- typst/typst issue #4422 — CMYK gradient bug vanilla causa raiz
  dictionary errado.
- pdfkit issue #532 análogo confirma causa raiz universal.
- ADR-0091 — Gradient ColorSpace runtime + CMYK strategy
  (§"Decisão L3 (materializada P270.1+P270.2)" pendente).
- ADR-0083 — Color paridade (§"DeviceCMYK PDF" revogação parcial
  P270.2 pendente).
- ADR-0085 — Diagnóstico imutável (décimo consumo).
