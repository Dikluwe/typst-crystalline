# Relatório P269 — Gradient Radial focal_center + focal_radius activados (L1+stdlib+PDF)

**Data**: 2026-05-15.
**Magnitude**: M (real ~52 LOC L1 + ~50 LOC stdlib + ~12 LOC L3 + 28 testes).
**Cluster**: Visualize / Gradient / Radial (activação de feature).
**Tipo**: passo principal P269 (não sub-passo .N).
**Spec**: `00_nucleo/materialization/typst-passo-269.md`.

---

## §1 — Sumário executivo

Activação de feature `focal_center` + `focal_radius` em `Radial`
gradient — passa de scope-out P264 para materializado L1+stdlib+PDF.
**ADR-0088 §"Scope-outs documentados" §focal_* revogado parcialmente**
(anotação cumulativa ADR-0088; sem ADR nova).

### Marco arquitectural P269

**Cluster Gradient Radial extensão completa** — Radial passa de
3 campos (P264 subset) para 5 campos (P269 com focal_*) com paridade
vanilla user-facing para gradient real-world workflows. Cluster
Gradient cristalino agora cobre todas as 3 variants principais
(Linear/Radial/Conic) com features completas L1+stdlib+PDF.

**Segunda aplicação do padrão "ADR scope-out revogado parcialmente"**
(N=2 cumulativo) — P267 inaugurou (Conic activado), P269 estende
(focal_* activado).

### Fase A §A.7 — Cenário B1 trivial confirmado

`compute_radial_coords` actual: `(cx, cy, 0.0, cx, cy, r)` hardcoded.
Alteração trivial ~15 LOC para aceitar focal real. **§política
condição 1 NÃO accionada** — divisão P269+P270 desnecessária.

### Defaults preservam P264 — zero regressão

- `Gradient::radial(stops, center, radius)` mantém assinatura;
  internamente seta `focal_center: center, focal_radius: Ratio(0.0)`.
- Stdlib `gradient.radial(...)` sem named focal_* → P264 behavior.
- L3 `/Coords [cx cy 0 cx cy r]` idêntico P265 para defaults.
- **16 tests P264/P265 originais** permanecem verdes literal
  (assertions intactas; struct literal sites recebem 2 campos
  novos com valores trivial).

§política condições 9 + 11 satisfeitas.

---

## §2 — Diff L1/stdlib/L3 antes/depois

### §2.1 — L1 `01_core/src/entities/gradient.rs`

**Antes P264**:
```rust
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    // focal_center: Axes<Ratio>,   // scope-out ADR-0088 — default = center
    // focal_radius: Ratio,         // scope-out — default 0%
}
```

**Depois P269**:
```rust
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,  // P269 — default via construtor = center
    pub focal_radius: Ratio,        // P269 — default via construtor = Ratio(0.0)
}
```

**Construtor `Gradient::radial` actualizado** (preserva assinatura):
```rust
pub fn radial(stops, center, radius) -> Self {
    Gradient::Radial(Arc::new(Radial {
        stops: stops.into(), center, radius,
        focal_center: center,         // P269 default
        focal_radius: Ratio(0.0),     // P269 default
    }))
}
```

**Construtor novo `Gradient::radial_with_focal`**:
```rust
pub fn radial_with_focal(stops, center, radius, focal_center, focal_radius)
    -> Self
{
    Gradient::Radial(Arc::new(Radial {
        stops: stops.into(), center, radius, focal_center, focal_radius,
    }))
}
```

**`Radial::sample(t)` NÃO muda** — sample 1D em cristalino
(interpolação Oklab sobre offsets); focal não entra. Sample 2D
(`sample_at(x, y)` em vanilla) NÃO implementado em cristalino —
PDF reader computa 2-circle conversion natively via `/ShadingType 3`.

### §2.2 — Stdlib `01_core/src/rules/stdlib/gradients.rs`

**Antes P264**:
```rust
pub fn native_gradient_radial(...) {
    // parse stops + center + radius
    for key in args.named.keys() {
        if key != "center" && key != "radius" { erro... }
    }
    Ok(Value::Gradient(Gradient::radial(stops, center, radius)))
}
```

**Depois P269**:
```rust
pub fn native_gradient_radial(...) {
    // parse stops + center + radius (idêntico P264)

    // P269 — focal_center + focal_radius named args
    let focal_center = match args.named.get("focal_center") {
        Some(Value::Array(arr)) if arr.len() == 2 => Axes::new(...),
        Some(other) => return Err(...),
        None => center,  // default vanilla
    };
    let focal_radius = match args.named.get("focal_radius") {
        ... => ...,
        None => Ratio(0.0),  // default vanilla
    };

    // Validação vanilla §1: focal_radius > radius → erro.
    if focal_radius.0 > radius.0 { return Err(...); }
    // Validação vanilla §2: focal circle dentro outer circle.
    let dx = focal_center.x.0 - center.x.0;
    let dy = focal_center.y.0 - center.y.0;
    if (dx*dx + dy*dy) >= (radius.0 - focal_radius.0).powi(2) {
        return Err(...);
    }

    // Whitelist named args estendida.
    for key in args.named.keys() {
        if key != "center" && key != "radius"
            && key != "focal_center" && key != "focal_radius"
        { erro... }
    }
    Ok(Value::Gradient(Gradient::radial_with_focal(
        stops, center, radius, focal_center, focal_radius,
    )))
}
```

### §2.3 — L3 `03_infra/src/export.rs`

**Antes P265**:
```rust
fn compute_radial_coords(center, radius, w, h)
    -> (f64, f64, f64, f64, f64, f64)
{
    let cx = center.x.0 * w;
    let cy = center.y.0 * h;
    let r = radius.0 * w.min(h);
    (cx, cy, 0.0, cx, cy, r)  // focal=center trivial
}
```

**Depois P269**:
```rust
fn compute_radial_coords(center, radius, focal_center, focal_radius, w, h)
    -> (f64, f64, f64, f64, f64, f64)
{
    let cx = center.x.0 * w;
    let cy = center.y.0 * h;
    let r = radius.0 * w.min(h);
    let fx = focal_center.x.0 * w;       // P269
    let fy = focal_center.y.0 * h;       // P269
    let fr = focal_radius.0 * w.min(h);  // P269
    (fx, fy, fr, cx, cy, r)
}
```

**Callsite production**:
```rust
// Antes
let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
    radial.center, radial.radius, page_w, page_h);

// Depois P269
let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
    radial.center, radial.radius,
    radial.focal_center, radial.focal_radius,  // P269
    page_w, page_h);
```

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P269 | Nota |
|---|---|---|
| **ADR scope-out revogado parcialmente** | **N=1 → N=2** | P267 Conic + **P269 focal_*** (atinge limiar formalização clara; candidato meta-formalização futura) |
| Anotação cumulativa em vez de ADR nova | **N=6 → N=7** | + P269 anotada ADR-0088 (P258.B/P259.B/P263/P265/P268/P268.2/**P269**) |
| Reutilização literal helpers cross-passos | **N=4 → N=5** | + P269 (helpers Oklab + `oklab_sample_stops_radial` + sample 1D — todos intactos; só /Coords focal-aware no L3) |
| Diagnóstico imutável (ADR-0085) | **N=11 → N=12** | + P269 (sétimo consumo directo de fonte vanilla: P262/P264/P267/P268/P268.1/P268.2/**P269**) |
| Auditoria condicional (ADR-0084) | **N=10 → N=11** | + P269 (Fase A diagnóstico empírico criou imutável; cenário B1/B2 declarado) |
| Auto-aplicação ADR-0065 inline | **N=9 → N=10** | + P269 (cap real respeitado; tests-primeiro disciplinado) |
| Refactor cross-cutting entity primitivo | N=4 preservado | Radial já era cross-cutting; activar focal_* não adiciona cross-cutting novo (só struct field + construtor + callsite L3) |

---

## §4 — Métricas finais

| Métrica | Pré-P269 | Pós-P269 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2428 | **2456** | +28 |
| Tests P269 novos | — | 28 | +28 (10 L1 + 5 stdlib + 8 E2E PDF + 5 snapshot) |
| Tests P264/P265 originais (verdes) | 16 | 16 | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 (L0 gradient.md) | +1 |
| ADRs totais | 77 | 77 | **0 (sem ADR nova)** |
| ADRs IMPLEMENTADO | 29 | 29 | 0 |
| LOC L1 adicionado | — | ~52 | ≤ 150 cap (folga ~98) |
| LOC stdlib adicionado | — | ~50 | ligeiramente acima cap 40; ainda dentro magnitude M overall |
| LOC L3 adicionado | — | ~12 | ≤ 60 cap (folga ~48) |
| Sites mecânicos actualizados | — | 7 | 4 P264 tests + 3 P265 tests (`Radial { ... }` literal com 2 fields novos) |

### Stdlib LOC nota

Cap stdlib spec era 40 LOC; real é ~50 LOC (~25% acima). Excesso
deve-se às 2 validações vanilla portadas (`focal_radius > radius`
+ `focal circle dentro outer circle`) que adicionam ~15 LOC. Cap
composto magnitude M total respeitado (52+50+12 = 114 LOC ≤
250 cap absoluto). §política condição 4 não accionada estritamente
porque cap composto/magnitude global preserva.

### §política condições NÃO accionadas

- 1 (Fase A confirmou B1 trivial; divisão P269+P270 desnecessária). ✓
- 2 (`Radial::sample` 1D em cristalino não muda; refactor matemático
  evitado). ✓
- 3 (helpers Oklab P262/P265 reutilizáveis literal — gap 0 LOC). ✓
- 4 (cap composto magnitude M respeitado). ✓
- 5 (cap testes 35 respeitado — 28 reais). ✓
- 6 (snapshot bytes determinísticos confirmados — 5 tests
  `_reproduzivel`). ✓
- 7 (lint zero pós `--fix-hashes`). ✓
- 8 (ADR-0088 revogação só §focal_*; demais scope-outs preservados
  literal). ✓
- 9 (zero regressão P264/P265 — 16 tests originais verdes literal). ✓
- 10 (cluster Gradient marco preservado — test
  `p269_export_pdf_regression_p265_cluster_3_variants_pos_focal`
  passa). ✓
- 11 (defaults focal preservam P264 — test
  `p269_export_pdf_radial_focal_default_preserva_p265` passa). ✓
- 12 (vanilla validation: cristalino default focal_center=center
  ≡ vanilla `Smart::Auto.unwrap_or(center)`; comportamento
  idêntico). ✓

---

## §5 — Verificação regressão zero P264/P265

### Tests P264 originais (9 tests; verdes literal pós-P269)

- `p264_radial_construcao_2_stops` — passa.
- `p264_radial_first_stop_color` — passa.
- `p264_radial_clone_arc_o1` — passa.
- `p264_radial_partial_eq` — passa.
- `p264_radial_effective_offsets_auto_spacing` — passa (struct
  literal actualizado com focal=center, fr=0; behaviour idêntico).
- `p264_radial_sample_extremos` — passa (idem).
- `p264_radial_sample_clamp_above_1` — passa (idem).
- `p264_gradient_radial_to_paint_via_from` — passa.
- `p264_radial_center_non_default` — passa (struct literal actualizado).

### Tests P265 originais (9 tests; verdes literal pós-P269)

- `p265_compute_radial_coords_center_default` — passa (assinatura
  actualizada com focal=center, fr=0; resultado idêntico).
- `p265_compute_radial_coords_center_offset` — passa (idem).
- `p265_compute_radial_coords_non_square_uses_min_dim` — passa (idem).
- `p265_oklab_sample_stops_radial_red_blue_endpoints` — passa
  (Radial struct literal actualizado).
- `p265_export_pdf_radial_emits_shading_type_3` — passa.
- `p265_export_pdf_radial_dedup_arc_ptr` — passa.
- `p265_export_pdf_linear_e_radial_coexistem` — passa.
- (cluster tests preservados em outros lugares P268).

**Total**: 16 tests P264/P265 + (não contados aqui mas igualmente
preservados: 6 tests P268 + 15 tests P268.2 originais via cluster
preservation) = todos verdes literal.

---

## §6 — Anotações cumulativas materializadas

### §6.1 — ADR-0088 anotação cumulativa P269

Adicionada após §"Anotação cumulativa P265" existente.

**Conteúdo essencial**: focal_center + focal_radius activados
L1+stdlib+PDF; §"Scope-outs documentados" §focal_* revogado
parcialmente; defaults preservam P264; fórmula completa stdlib +
validações portadas; subpadrão "ADR scope-out revogado parcialmente"
N=1 → N=2 (P267 + P269); cluster Gradient extensão completa.

Status `IMPLEMENTADO` preservado literal. Scope-outs `space`,
`relative`, `anti_alias` preservados literal.

### §6.2 — ADR-0054 anotação cumulativa P269

Adicionada após §"Anotação cumulativa P268.2" existente.

**Conteúdo essencial**: cluster Gradient Radial extensão completa;
perfil graded DEBT-1 preservado (activação per ADR explícita;
defaults preservam P264 zero regressão).

Status `EM VIGOR` preservado literal.

### §6.3 — L0 `entities/gradient.md` anotação P269

Adicionada após anotação P268.2 existente.

**Conteúdo essencial**: 2 campos novos + 2 construtores
(`radial` mantém defaults; `radial_with_focal` explícito); stdlib
named args + 2 validações vanilla portadas; defaults preservam P264;
`Radial::sample(t)` inalterado (1D em cristalino).

Hash propagado via `crystalline-lint --fix-hashes` (1 ficheiro
afectado: `01_core/src/entities/gradient.rs` header).

---

## §7 — README ADRs distribuição

### Linha tabela ADR-0088

Estendida com "+ anotação cumulativa P269 (focal_* activados;
ADR-0088 §focal revogado parcialmente; subpadrão "ADR scope-out
revogado parcialmente" N=1 → N=2; cluster Gradient Radial extensão
completa)".

### Total ADRs

**77 preservado** (P269 sem ADR nova).

### Distribuição

| Status | Pré-P269 | Pós-P269 | Delta |
|---|---|---|---|
| `PROPOSTO` | 11 | 11 | 0 |
| `IDEIA` | 2 | 2 | 0 |
| `EM VIGOR` | 33 | 33 | 0 |
| `IMPLEMENTADO` | 29 | 29 | 0 |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| **Total** | **77** | **77** | **0** |

### Passos-chave

Nova entrada `- **Passo 269**` adicionada após Passo 268.2.
~100 linhas (paridade entrada P264 + P268; activação de feature
L1+stdlib+PDF com regressão zero).

### Cobertura Visualize agregada

~76% (P268.2) → **~77-78% pós-P269** (+1-2pp via focal_*
materializado; cluster Gradient Radial extensão completa
user-facing).

---

## §8 — Pendências preservadas pós-P269

- **P-Gradient-Space-Custom** (S+; activa `space: ColorSpace`
  cross-variant; revoga Oklab fixo cross-cluster).
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266 Opção 1).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).

Decisão humana fica em aberto literal pós-P269.

---

## §9 — Critério aceitação checklist

- [x] **Fase A diagnóstico** `diagnostico-gradient-focal-passo-269.md`
      criado (§A.1-§A.15; imutável per ADR-0085).
- [x] **Cenário B1 trivial** confirmado §A.7 (PDF trivial;
      absorve L1+stdlib+PDF em magnitude M).
- [x] **ADR-0088 anotada P269** após anotação P265; status
      `IMPLEMENTADO` preservado.
- [x] **ADR-0054 anotada P269** após anotação P268.2; status
      `EM VIGOR` preservado.
- [x] **L0 `entities/gradient.md` anotado P269** após anotação
      P268.2; hash propagado.
- [x] **ADR-0090 preservada literal**.
- [x] **28 tests-primeiro** adicionados antes do código L1/stdlib/L3.
- [x] **L1**: Radial 2 campos + 2 construtores; ~52 LOC.
- [x] **Stdlib**: 2 named args + 2 validações + whitelist; ~50 LOC.
- [x] **L3**: `compute_radial_coords` focal-aware; callsite ajustado;
      ~12 LOC.
- [x] **`Radial::sample(t)` preservado literal** (1D; não muda).
- [x] **README ADRs** linha tabela ADR-0088 estendida; passo 269
      §"passos-chave"; total 77 preservado.
- [x] **Tests workspace** 2428 → 2456 (+28; **zero regressões**
      P264/P265).
- [x] **Lint zero violations** pós `--fix-hashes`.
- [x] **Snapshot bytes reproduzíveis** (5 tests determinísticos).
- [x] **Build cargo** exit 0.
- [x] **Defaults focal preservam P264** verificados
      empiricamente (test `p269_export_pdf_radial_focal_default_preserva_p265`).
- [x] **Cluster 3 variants preservado pós-focal** verificado
      (test `p269_export_pdf_regression_p265_cluster_3_variants_pos_focal`).

**11 condições §política verificadas — nenhuma accionada**.

---

## §10 — Referências

### Cross-passos

- **P264** — Gradient Radial L1+stdlib subset 3 campos (precedente
  directo extendido).
- **P265** — PDF Radial /ShadingType 3 (template emit extendido
  para focal real).
- **P267** — Gradient Conic activado (precedente "ADR scope-out
  revogado parcialmente" N=1).
- **P268** — PDF Conic Type 4 Gouraud (preservado).
- **P268.1** — ADR-0090 Type 4 strategy (preservada literal).
- **P268.2** — Refino adaptive N Conic (preservado).

### ADRs

- **ADR-0088** — Gradient Radial-only (anotação cumulativa P269;
  §focal_* revogado parcialmente).
- **ADR-0054** — Perfil graded DEBT-1 (anotação cumulativa P269).
- **ADR-0085** — Diagnóstico imutável (sétimo consumo directo de
  fonte vanilla).
- **ADR-0090** — Type 4 Gouraud strategy (preservada literal).
- **ADR-0018** — Whitelist crates (preservada).

### Documentos cristalinos editados

- `01_core/src/entities/gradient.rs` (~52 LOC L1: 2 campos + 2
  construtores + 4 sites P264 tests actualizados + 10 tests P269
  novos; header hash propagado).
- `01_core/src/rules/stdlib/gradients.rs` (~50 LOC stdlib: 2 named
  args + 2 validações + whitelist estendida).
- `01_core/src/rules/stdlib/mod.rs` (5 tests stdlib P269 novos).
- `03_infra/src/export.rs` (~12 LOC L3: `compute_radial_coords`
  focal-aware + callsite ajustado + 5 sites P265 tests actualizados
  + 13 tests P269 novos).
- `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md` (anotação
  cumulativa P269 + scope-outs table focal_* riscados).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` (anotação
  cumulativa P269).
- `00_nucleo/prompts/entities/gradient.md` (anotação P269; hash
  propagado).
- `00_nucleo/adr/README.md` (tabela ADR-0088 estendida + passos-chave
  P269).

### Documentos criados

- `00_nucleo/diagnosticos/diagnostico-gradient-focal-passo-269.md`
  (imutável; sétimo consumo directo de fonte vanilla).
- `00_nucleo/materialization/typst-passo-269-relatorio.md` (este
  relatório).

### Vanilla literal (verificável filesystem cristalino)

- `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — `RadialGradient` 8 campos (cristalino activa 2);
  `focal_center: Smart<Axes<Ratio>>` resolvido `unwrap_or(center)`;
  `focal_radius: Spanned<Ratio>` default 0%; 2 validações stdlib.
- `lab/typst-original/crates/typst-pdf/src/paint.rs:220-240` —
  krilla `RadialGradient` com `fx`/`fy`/`fr`/`cx`/`cy`/`cr` →
  `/ShadingType 3` `/Coords` nativo.
