# Relatório P270.2 — L3 emit CMYK directo /DeviceCMYK Linear+Radial (Cenário B)

**Data**: 2026-05-17.
**Magnitude**: S+ (real ~138 LOC L3 + 12 testes).
**Cluster**: Visualize / Gradient / PDF export (refino L3 CMYK).
**Tipo**: sub-passo .2 da série P270.
**Spec**: `00_nucleo/materialization/typst-passo-270.2.md`.

---

## §1 — Sumário executivo

L3 emit CMYK directo via `/ColorSpace /DeviceCMYK` materializado para
**Linear+Radial** (Cenário B). **Conic CMYK preserved scope-out**
(P-Gradient-Conic-CMYK futuro). ADR-0091 e ADR-0083 anotadas
cumulativas; sem ADR nova.

### Marco arquitectural P270.2

**Cluster Gradient L3 emit Linear+Radial feature-complete em 8/8
spaces** (Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV + CMYK directo).
Conic 7/8 full + CMYK fallback sub-óptimo (preserved P270.1
pipeline; gama CMYK perdida no emit).

**Sub-padrão "ADR scope-out revogado parcialmente" N=3 → N=4
cumulativo limiar formalização clara** (P267 Conic + P269 focal_*
+ P270 ColorSpace + **P270.2 DeviceCMYK** parcial). Candidato
meta-ADR futura paridade P260 ADR-0084/0085.

### Bug #4422 resolvido por construção

Cristalino emit `/ColorSpace /DeviceCMYK` correcto vs vanilla bug
`/DeviceRGB` (causa raiz dictionary errado por wrapper intermediário;
pdfkit #532 análogo confirma universal). Cristalino implementação
directa sem wrapper evita o bug.

### Decisão Cenário B confirmado

§A.8/§A.11 diagnóstico decide:
- **Linear CMYK materializado** `/DeviceCMYK`.
- **Radial CMYK materializado** `/DeviceCMYK` (focal_* P269
  preservados).
- **Conic CMYK preserved scope-out** — candidato futuro
  P-Gradient-Conic-CMYK. Razões:
  - Vanilla Conic CMYK suporte incerto (krilla opaco).
  - PDF reader compatibility Type 4 Gouraud + CMYK incerto.
  - Complexidade extra (stream binary 4 bytes/vertex; `/Decode`
    array 5 pares vs 4) adiciona ~50 LOC L3.
  - Linear + Radial cobrem maioria use cases user-facing.

### Defaults preservam P270.1 — zero regressão

`space != Cmyk` → arm "else" dispatcher dual → pipeline P270.1
literal preservado. **2533 baseline tests preservados literal**.

§política condições 4 + 7 + 9 satisfeitas absolutas.

---

## §2 — Diff L3 antes/depois

### §2.1 — Helpers L3 novos (~138 LOC)

```rust
// rgb_to_cmyk — fallback precaução (~12 LOC)
fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> (f32, f32, f32, f32);

// multispace_sample_stops_linear_cmyk — amostragem 4-component (~20 LOC)
fn multispace_sample_stops_linear_cmyk(linear: &Linear, n: usize)
    -> Vec<(f32, f32, f32, f32)>;

// multispace_sample_stops_radial_cmyk — análogo (~20 LOC)
fn multispace_sample_stops_radial_cmyk(radial: &Radial, n: usize)
    -> Vec<(f32, f32, f32, f32)>;

// emit_function_dict_cmyk — Function dict 4-component (~50 LOC)
fn emit_function_dict_cmyk(
    stops: &[(f32, f32, f32, f32)],
    function_id: usize,
    sub_first_id: &mut usize,
) -> (String, Vec<(usize, String)>);
// /Range [0 1 0 1 0 1 0 1] (8 values; 4 pares)
// /C0 [c m y k] /C1 [c m y k]
```

### §2.2 — Dispatcher dual em `emit_gradient_objects`

```rust
GradientObjectKind::Linear(linear) => {
    let (x0, y0, x1, y1) = compute_axial_coords(...);
    // P270.2 — dispatcher dual
    if linear.space == ColorSpace::Cmyk {
        let stops_cmyk = multispace_sample_stops_linear_cmyk(linear, 16);
        let shading_dict = format!(
            "<< /ShadingType 2 /ColorSpace /DeviceCMYK ... >>"
        );
        let (func_dict, sub_objs) = emit_function_dict_cmyk(...);
        // ...
    } else {
        // P270.1 pipeline preserved literal
        let stops = multispace_sample_stops(linear, 16);
        let shading_dict = format!(
            "<< /ShadingType 2 /ColorSpace /DeviceRGB ... >>"
        );
        let (func_dict, sub_objs) = emit_function_dict(...);
        // ...
    }
}

// Análogo Radial (focal_* P269 preservados em ambos branches).
// Conic preservado P270.1 literal (sub-óptimo CMYK fallback).
```

### §2.3 — Comparação shading dict CMYK vs RGB-family

| Field | RGB-family (P270.1) | CMYK (P270.2) |
|---|---|---|
| `/ShadingType` | 2 (axial) / 3 (radial) | idem |
| `/ColorSpace` | `/DeviceRGB` | `/DeviceCMYK` |
| `/Coords` | `[x0 y0 x1 y1]` / 6-value | idem (preservado) |
| `/Function` | Type 2/3 3-component | Type 2/3 **4-component** |
| `/Range` (Function) | implícito `[0 1 0 1 0 1]` | **`[0 1 0 1 0 1 0 1]`** explicit |
| `/C0` `/C1` (Function) | `[r g b]` | **`[c m y k]`** |

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P270.2 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=8 → N=9 cumulativo** | + P270.2 anotada ADR-0091 |
| **ADR scope-out revogado parcialmente** | **N=3 → N=4 cumulativo (limiar formalização clara)** | + P270.2 ADR-0083 §DeviceCMYK parcial |
| Reutilização literal helpers cross-passos | **N=7 → N=8 cumulativo** | + P270.2 (dispatcher P270 arm Cmyk; helpers L3 P270.1 templates) |
| Diagnóstico imutável (décimo consumo) | **N=14 → N=15 cumulativo** | + P270.2 (vanilla CMYK emit + bug #4422 causa raiz) |
| Auditoria condicional (ADR-0084) | **N=13 → N=14 cumulativo** | + P270.2 |
| Auto-aplicação ADR-0065 inline | **N=12 → N=13 cumulativo** | + P270.2 |
| **Cap LOC hard vs soft explícito** | **N=1 → N=2 cumulativo** | P270.1 inaugurou; P270.2 segunda aplicação consolida |
| **Anotação cumulativa cross-ADR** | **N=2 → N=3 cumulativo** | P270 + P270.1 + **P270.2** (6 ADRs cada; terceira aplicação consolida) |

---

## §4 — Métricas finais

| Métrica | Pré-P270.2 | Pós-P270.2 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2533 | **2545** | +12 |
| Tests P270.2 novos | — | 12 | 5 unit pré-amostragem + 5 E2E + 2 snapshot |
| Tests P262-P270.1 originais (verdes) | 2533 | 2533 | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 (L0 gradient.md) | +1 |
| ADRs totais | 78 | 78 | **0 (sem ADR nova)** |
| LOC L3 adicionado | — | ~138 | cap hard 250 (folga 45%); cap soft 150 (ligeiramente acima ~8 LOC) |

### §política condições verificadas

- 1 (Color::Cmyk variant existe P257; sem gap L1 substantivo;
  `to_cmyk_components` ficou private — usei pattern-match em
  `Color::Cmyk` directo). ✓
- 2 (`interpolate_cmyk` arm dispatcher P270 funcional sem gap). ✓
- 3 (Vanilla Conic CMYK suporte ambíguo — Cenário B decidido). ✓
- 4 (Cap L3 hard 250 — real ~138; folga 45%). ✓
- 5 (Cap testes hard 35 — real 12; folga 66%). ✓
- 6 (Cristalino emit `/DeviceCMYK` correcto — verificado via
  test `_shading_devicecmyk`). ✓
- 7 (Snapshot bytes reproduzíveis — 2 tests determinísticos). ✓
- 8 (Lint zero pós `--fix-hashes`). ✓
- 9 (Zero regressão tests P262-P270.1 — 2533 baseline preserved). ✓
- 10 (Conic CMYK Type 4 reader compatibility resolvido Cenário B
  scope-out preserved). ✓
- 11 (PDF reader compatibility issues — não bloqueante; cristalino
  emit correcto per spec). ✓
- 12 (ICC profile requirement — scope-out preserved; refino
  futuro P-Gradient-CMYK-ICC). ✓

**12 condições §política verificadas — todas satisfeitas**.

---

## §5 — Verificação regressão zero P262-P270.1

**2533 tests preservados literal** (baseline P262-P270.1):

- typst-core: 2162 preserved.
- typst-infra: 324 → 336 (+12 P270.2 tests).
- Outros: preserved.

Mecânica: dispatcher dual entra arm "else" para `space != Cmyk` →
pipeline P270.1 literal preservado bit-exact. Linear/Radial com
defaults Oklab produzem mesmo `/ColorSpace /DeviceRGB` + Function
3-component idêntico a P270.1.

§política condições 4 + 7 + 9 satisfeitas absolutas.

---

## §6 — Anotações cumulativas materializadas

### §6.1 — ADR-0091 anotação cumulativa P270.2

`00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md`
estendida com §"Anotação cumulativa P270.2 — L3 emit CMYK directo
(fecha L3 Linear+Radial 8/8; Conic CMYK scope-out preserved)".

**Conteúdo essencial**: Cenário B confirmado; helpers samplers CMYK
4-component; emit_function_dict_cmyk; dispatcher dual; Conic preserved
scope-out; bug #4422 resolvido por construção; ICC scope-out
preserved; subpadrões cumulativos; status `IMPLEMENTADO` preservado.

### §6.2 — ADR-0083 anotação cumulativa P270.2 (revogação parcial)

`00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md`
estendida com §"Anotação cumulativa P270.2 — DeviceCMYK PDF
revogação parcial".

**§"DeviceCMYK PDF" revogado parcialmente P270.2**:
- Linear+Radial: materializado `/DeviceCMYK` directo.
- Conic: preserved scope-out (P-Gradient-Conic-CMYK futuro).

Sub-padrão "ADR scope-out revogado parcialmente" N=3 → **N=4
cumulativo limiar formalização clara**.

### §6.3 — ADR-0087/0088/0089/0090 anotações cumulativas P270.2

4 anotações cumulativas curtas (variant strategies):
- ADR-0087 Linear: CMYK emit branch directo.
- ADR-0088 Radial: CMYK emit branch directo (focal_* preservados).
- ADR-0089 Conic: CMYK scope-out preserved Cenário B.
- ADR-0090 Type 4 Gouraud: estratégia preservada; Conic CMYK scope-out.

Sub-padrão "Anotação cumulativa cross-ADR" N=2 → **N=3 cumulativo**.

### §6.4 — ADR-0054 anotação cumulativa P270.2

Cluster Gradient L3 emit feature-complete (Linear+Radial 8/8;
Conic 7/8 + CMYK fallback). Perfil graded DEBT-1 preservado.

### §6.5 — L0 `entities/gradient.md` anotação P270.2

Adicionada após anotação P270.1. Documenta CMYK emit branch +
dispatcher dual + Conic CMYK scope-out + bug #4422 resolvido por
construção + ICC scope-out preserved.

Hash propagado via `crystalline-lint --fix-hashes`.

---

## §7 — README ADRs distribuição

### Total ADRs

**78 preservado** (P270.2 sem ADR nova; 6 anotações cumulativas).

### Distribuição

| Status | Pré-P270.2 | Pós-P270.2 | Delta |
|---|---|---|---|
| `PROPOSTO` | 11 | 11 | 0 |
| `IDEIA` | 2 | 2 | 0 |
| `EM VIGOR` | 33 | 33 | 0 |
| `IMPLEMENTADO` | 30 | 30 | 0 |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| **Total** | **78** | **78** | **0** |

### Passos-chave

Nova entrada `- **Passo 270.2**` adicionada após Passo 270.1.
~120 linhas (fecho da série P270 com 7 anotações cumulativas).

### Cobertura Visualize agregada

~81-83% (P270.1) → **~83-85% pós-P270.2** (+1-2pp via CMYK
Linear+Radial directo; cluster Gradient cobertura L3 ~96% —
Linear+Radial 8/8 spaces; Conic 7/8 + CMYK fallback).

---

## §8 — Pendências preservadas pós-P270.2

### Refinos Gradient (candidatos)

- **P-Gradient-Conic-CMYK** (S+ futuro) — Conic Type 4 Gouraud +
  `/DeviceCMYK` directo. Revoga definitivamente ADR-0083 §DeviceCMYK
  (para Conic); resolve cenário "8/8 full" cluster.
- **P-Gradient-CMYK-ICC** (S-M futuro) — krilla paridade custom
  ICC profiles; PDF/A compliance.
- **P-Gradient-Adaptive-Multispace** (S futuro) — N adaptive
  HSL/Oklch hue diff alto; paridade P268.2 adaptive N.
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).

### Demais pendências

- **ADR-0055bis variant-aware fonts** (M; refino Text P266 Opção 1).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).

Decisão humana fica em aberto literal pós-P270.2 — cluster Gradient
resolvido a nível user-facing excepto refino Conic CMYK.

---

## §9 — Critério aceitação checklist

- [x] **Fase A diagnóstico** `diagnostico-l3-cmyk-passo-270-2.md`
      criado (§A.1-§A.15; imutável per ADR-0085).
- [x] **Cenário B confirmado** §A.8/§A.11 (Linear+Radial CMYK;
      Conic preserved scope-out).
- [x] **ADR-0091 anotada P270.2** §"Anotação cumulativa P270.2".
- [x] **ADR-0083 anotada P270.2** revogação parcial §DeviceCMYK.
- [x] **ADR-0087/0088/0089/0090 anotadas P270.2** (4 anotações
      curtas variant strategies).
- [x] **ADR-0054 anotada P270.2** cluster L3 status.
- [x] **L0 `entities/gradient.md` anotada P270.2** após anotação
      P270.1; hash propagado.
- [x] **12 tests-primeiro** adicionados antes do código L3.
- [x] **L3 helpers CMYK**: `multispace_sample_stops_linear_cmyk`,
      `_radial_cmyk`, `emit_function_dict_cmyk`, `rgb_to_cmyk`.
- [x] **Dispatcher dual Linear+Radial branches** em
      `emit_gradient_objects`; Conic preserved P270.1 fallback.
- [x] **README ADRs** linha tabela ADR-0091 estendida com anotação
      P270.2; passo 270.2 §"passos-chave"; total 78 preservado.
- [x] **Tests workspace** 2533 → 2545 (+12; **zero regressões**).
- [x] **Lint zero violations** pós `--fix-hashes`.
- [x] **Build cargo** exit 0.
- [x] **Bug #4422 resolvido por construção** verificado via test
      `_shading_devicecmyk` (`/ColorSpace /DeviceCMYK`).

**12 condições §política verificadas — todas satisfeitas**.

---

## §10 — Referências

### Cross-passos

- **P270** — Gradient ColorSpace runtime L1+stdlib (ADR-0091;
  precedente directo).
- **P270.1** — L3 emit 7 spaces RGB-family + perceptual
  (precedente directo refinado por este passo).
- **P262/P264/P267** — Variant L1+stdlib (preservados).
- **P263/P265/P268** — L3 emit templates (preservados; CMYK
  branch aditivo).
- **P268.2** — Adaptive N hybrid Conic (preservado).
- **P269** — Radial focal_* (preservado; campo space aditivo
  cross-variant em ambos branches).
- **P257** — Color 8/8 spaces (ADR-0083; §DeviceCMYK revogação
  parcial P270.2).

### ADRs

- **ADR-0091** — Gradient ColorSpace runtime + CMYK strategy
  (§"Anotação cumulativa P270.2" final L3).
- **ADR-0083** — Color paridade (§"Anotação cumulativa P270.2"
  revogação parcial §DeviceCMYK).
- **ADR-0087/0088/0089/0090** — Variant strategies (anotadas
  cumulativa P270.2).
- **ADR-0054** — Perfil graded (anotada cumulativa P270.2).
- **ADR-0018** — Whitelist crates (preservada; sem ICC).
- **ADR-0085** — Diagnóstico imutável (décimo consumo directo de
  fonte vanilla).

### Documentos cristalinos editados

- `03_infra/src/export.rs` (~138 LOC L3: 4 helpers CMYK novos +
  dispatcher dual Linear+Radial + 12 tests P270.2 novos).
- `00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md` (anotação P270.2).
- `00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md` (anotação P270.2 revogação parcial).
- `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md` (anotação P270.2).
- `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md` (anotação P270.2).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` (anotação P270.2 Cenário B).
- `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md` (anotação P270.2).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` (anotação P270.2).
- `00_nucleo/prompts/entities/gradient.md` (anotação P270.2; hash
  propagado).
- `00_nucleo/adr/README.md` (tabela ADR-0091 estendida + passos-chave
  P270.2).

### Documentos criados

- `00_nucleo/diagnosticos/diagnostico-l3-cmyk-passo-270-2.md`
  (imutável; décimo consumo directo de fonte vanilla).
- `00_nucleo/materialization/typst-passo-270-2-relatorio.md` (este
  relatório).

### Vanilla literal (verificável)

- `lab/typst-original/.../visualize/gradient.rs` — vanilla CMYK
  emit pipeline.
- `lab/typst-original/.../visualize/color.rs:1095-1176` —
  `mix_iter` multi-space (preserved; CMYK arm).

### Fontes empíricas (verificáveis via web)

- typst/typst issue #4422 — CMYK gradient bug vanilla causa raiz
  dictionary errado.
- pdfkit issue #532 análogo confirma causa raiz universal.

### Marco arquitectural

**Cluster Gradient L3 emit Linear+Radial feature-complete em 8/8
spaces**:
- Linear: Oklab + Oklch + sRGB + Luma + LinearRGB + HSL + HSV +
  CMYK directo.
- Radial: idem + focal_* P269 preservados.
- Conic: 7/8 full + CMYK fallback sub-óptimo (P-Gradient-Conic-CMYK
  futuro).

**ADR-0083 §DeviceCMYK revogação parcial** — última pendência
substantiva cluster Color (P257) resolvida (Linear+Radial); Conic
último para fechar definitivamente.

**Sub-padrão "ADR scope-out revogado parcialmente" N=4 cumulativo
limiar formalização clara** — candidato meta-ADR futura paridade
P260 ADR-0084/0085.
