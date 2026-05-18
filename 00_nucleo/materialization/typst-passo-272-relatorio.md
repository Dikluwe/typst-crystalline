# Relatório P272 — P-Gradient-Coons-RGB-Final (converge Conic RGB Type 4 → Type 6 Coons; ADR-0090 REVOGADO)

**Data**: 2026-05-17.
**Magnitude**: M (real ~80-100 LOC L3 additions + ~140 LOC L3 removals + ~620 LOC tests removals; net -660 a -680 L3).
**Cluster**: Visualize / Gradient / PDF export (convergência arquitectural).
**Tipo**: passo principal P272. Refino estratégico — converge 2 estratégias Conic L3 emit em 1 única.
**Spec**: `00_nucleo/materialization/typst-passo-272.md`.

---

## §1 — Sumário executivo

**Cluster Gradient L3 emit estratégia única Coons** materializada P272:

- **ADR-0090 REVOGADO** (Type 4 Gouraud descontinuado).
- **ADR-0092 expandida cumulativamente** (Cenário A revisado FINAL —
  Coons unified 8/8 spaces).
- **Dispatcher Conic unificado** `/ShadingType 6` para 8/8 spaces:
  - RGB-family + perceptual (7 spaces): N=stops*4 patches angulares.
  - CMYK (preserved P270.4): N=stops patches.

### Marcos arquitecturais P272

**(1) Cluster Gradient L3 emit estratégia única feature-complete
24/24 simplificado** — eliminação 2 estratégias coexistentes Conic
sem perda funcional.

**(2) Primeira ADR cristalina REVOGADA pós-formalização ADR-0093
P271** — sub-padrão "Aplicação meta-ADR (ADR-0093 + ADR-0094)" N=1
inaugural cada. Demonstra empiria das metodologias.

**(3) Net LOC L3 negativo intencional** — ~80-100 additions + ~140
removals helpers + ~620 removals tests = net -660 a -680. Limpeza
arquitectural sem regressão funcional.

**(4) Bug vanilla #4422 preservado resolvido** — Conic CMYK
`/DeviceCMYK` continua correcto via dispatcher P272 unified
(preserved P270.4 logic).

### Strategy N=stops*4 patches (RGB; divergência intencional)

- 2 stops → 8 patches angulares.
- 5 stops → 20 patches.
- 4 stops → 16 patches.
- Corner colors via `Conic::sample(t)` dispatcher P270 (dispatches
  `interpolate_in_space` per `conic.space` automaticamente).
- Justificativa: qualidade visual angular superior; cap LOC
  accommodates (folga ~100% sobre hard 200).

### Defaults preservam parcialmente — behaviour change intencional

- **Linear**: preserved literal (P262/P263/P270.1/P270.2).
- **Radial**: preserved literal (P264/P265/P269/P270.1/P270.2).
- **Conic CMYK**: preserved literal (P270.4 Coons CMYK).
- **Conic RGB-family + perceptual**: **MUDOU** (Type 4 Gouraud →
  Type 6 Coons N=stops*4).
- **Snapshot bytes Conic RGB MUDAM intencionalmente** (Type 4 →
  Type 6); 20 tests P268+P268.2 byte snapshots removed; 5+ tests
  P272 byte snapshots new.

---

## §2 — Diff L3 antes/depois

### §2.1 — Helpers ADDED P272 (~10 LOC)

```rust
// Strategy "N stops * 4" patches angulares.
fn compute_coons_patches_n_stops_extended(conic: &Conic) -> usize {
    conic.stops.len() * 4
}
```

### §2.2 — Helper `emit_conic_coons_stream_rgb` (~90 LOC)

Rename `emit_conic_coons_stream` (P270.3) + extension N=stops*4 +
corner colors via `Conic::sample(t)` dispatcher P270:

```rust
fn emit_conic_coons_stream_rgb(conic: &Conic) -> Vec<u8> {
    let n_patches = compute_coons_patches_n_stops_extended(conic);
    if n_patches == 0 { return Vec::new(); }
    let mut stream = Vec::with_capacity(37 * n_patches);

    let center = (0.5_f32, 0.5_f32);
    let radius = 0.5_f32;
    let angle_offset = conic.angle.to_rad() as f32;
    let n = n_patches as f32;

    for i in 0..n_patches {
        let t_start = (i as f32) / n;
        let t_end = ((i + 1) as f32) / n;
        let color_start = conic.sample(t_start);
        let color_end = conic.sample(t_end);
        // ... 12 control points + 4 corner colors RGB (37 bytes/patch).
    }
    stream
}
```

### §2.3 — Helpers REMOVED P272 (~140 LOC)

- `oklab_delta_e(c1, c2) -> f32` (~15 LOC).
- `compute_adaptive_n_conic(conic) -> usize` (~40 LOC).
- `emit_conic_gouraud_stream(conic, n_slices) -> Vec<u8>` (~85 LOC).

Total: ~140 LOC L3 removed.

### §2.4 — Dispatcher unificado em `emit_gradient_objects`

```rust
GradientObjectKind::Conic(conic) => {
    use typst_core::entities::layout_types::ColorSpace;
    let (stream, colorspace, decode_array, c0, c1) =
        if conic.space == ColorSpace::Cmyk {
            (emit_conic_coons_stream_cmyk(conic),
             "/DeviceCMYK",
             "[0 1 0 1 0 1 0 1 0 1 0 1]",
             "[0 0 0 0]", "[1 1 1 1]")
        } else {
            (emit_conic_coons_stream_rgb(conic),
             "/DeviceRGB",
             "[0 1 0 1 0 1 0 1 0 1]",
             "[0 0 0]", "[1 1 1]")
        };
    let len = stream.len();
    let header = format!(
        "<< /ShadingType 6 /ColorSpace {} \
           /BitsPerCoordinate 8 /BitsPerComponent 8 \
           /BitsPerFlag 8 \
           /Decode {} \
           /Length {} >>\nstream\n",
        colorspace, decode_array, len,
    );
    // ... emit shading + function dict
}
```

### §2.5 — `multispace_sample_stops_conic` preserved `#[allow(dead_code)]`

Helper preserved como utilitário de teste — usado por tests P270.1+
multispace que validam o L1 dispatcher; sem callers em production
pós-P272.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P272 | Nota |
|---|---|---|
| **ADR REVOGADO + substituta** | **N=2 → N=3 cumulativo** | + P272 ADR-0090 → ADR-0092 expandida |
| **Aplicação meta-ADR (ADR-0093)** | **N=1 inaugural** | Primeira aplicação prática Pattern 1 §"Quando NÃO aplicar" pós-formalização P271 |
| **Aplicação meta-ADR (ADR-0094)** | **N=1 inaugural** | Cap LOC hard/soft Pattern 1 aplicado spec P272 |
| Anotação cumulativa em vez de ADR nova | **N=10 → N=11 cumulativo** | + P272 ADR-0092 expandida + 4 anotações paralelas |
| Reutilização literal helpers cross-passos | **N=10 → N=11 cumulativo** | + P272 (helpers Coons P270.3 + Conic::sample P270) |
| Cap LOC hard vs soft explícito | **N=4 → N=5 cumulativo consolidação total** | + P272 (real ~80-100 additions; folga grande) |
| Anotação cumulativa cross-ADR | **N=5 → N=6 cumulativo** | + P272 (5 ADRs anotadas paralelas) |
| Diagnóstico imutável (décimo terceiro consumo) | **N=17 → N=18 cumulativo** | + P272 (consolidação P270.3 reutilizada literal) |
| Fase A com industry research proactiva | N=4 preserved | P272 reutiliza P270.3, não nova |
| Auditoria condicional (ADR-0084) | **N=16 → N=17 cumulativo** | + P272 |
| Auto-aplicação ADR-0065 inline | **N=16 → N=17 cumulativo** | + P272 |

**3 sub-padrões inaugurados/expandidos com significado metodológico**:
- **"ADR REVOGADO + substituta"** N=3 (não inaugural — N=2 prévio
  ADR-0007/ADR-0018 + ADR-0028/ADR-0029).
- **"Aplicação meta-ADR (ADR-0093)"** N=1 inaugural — primeira
  empiria pós-formalização.
- **"Aplicação meta-ADR (ADR-0094)"** N=1 inaugural — primeira
  aplicação cap LOC hard/soft formalizado.

---

## §4 — Métricas finais

| Métrica | Pré-P272 | Pós-P272 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2572 | **2557** | -15 net |
| Tests P272 (novos/renomeados) | — | 15 | renomeados de P270.3 (~8) + novos (~5-7) |
| Tests P268+P268.2 (removidos) | 20 | 0 | -20 |
| Tests P270.3 remanescentes | 15 | 7 | -8 (renomeados para P272) |
| Tests P262-P270.4 não-Conic (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados L0 | — | 0 (gradient.md já actualizado pré-build) | 0 |
| ADRs totais | 81 | **81** | 0 (P272 revoga ADR-0090; ADR-0092 expandida; sem nova ADR) |
| ADRs EM VIGOR | 35 | **34** | -1 (ADR-0090 → REVOGADO) |
| ADRs REVOGADO | 2 | **3** | +1 (ADR-0090) |
| ADRs IMPLEMENTADO | 31 | 31 | 0 |
| LOC L3 (additions) | — | ~90-100 | cap hard 200 folga 100%; cap soft 120 folga 20-33% |
| LOC L3 (removals helpers) | — | ~140 | sem cap |
| LOC tests (removals) | — | ~620 | 20 tests × ~31 LOC |
| Net LOC L3 | — | **-660 a -680** | negativo intencional; limpeza |

### §política condições verificadas

- 1 (Fase A §A.3 verificou `oklab_delta_e` não usado fora do Conic
  Type 4 pipeline — safe to remove). ✓
- 2 (Fase A §A.4 verificou 20 tests P268+P268.2 — 1 preserved
  (multispace genérico)). ✓
- 3 (Cap L3 additions hard 200 — real ~90-100; folga 100%). ✓
- 4 (Cap testes additions hard 30 — real 5 novos + 8-10 renames;
  folga grande). ✓
- 5 (Defaults Linear/Radial/Conic CMYK preservam literal). ✓
- 6 (Snapshot bytes Conic RGB MUDAM intencionalmente — Type 4 →
  Type 6 behaviour change; tests P268+P268.2 removed; tests P272
  byte snapshots new). ✓
- 7 (Crystalline-lint zero violations pós anotações + REVOGADO). ✓
- 8 (Regressão tests P262-P270.4 não-Conic preservada literal —
  Linear/Radial/Conic CMYK byte snapshots bit-exact). ✓
- 9 (Fase A §A.9 sub-padrão "ADR REVOGADO + substituta" N=2 prévio
  → N=3 P272; pattern emergente já estabelecido historicamente). ✓
- 10 (Cluster Gradient marco `p272_export_pdf_cluster_3_variants_unified_strategy`
  passa). ✓
- 11 (Strategy N=stops*4 N=8 patches para 2 stops; corner colors
  via Conic::sample dispatcher P270). ✓
- 12 (Anotações cross-ADR 5 ADRs coerentes — cada refere ADR-0092
  §"Anotação cumulativa P272"). ✓

**12 condições §política verificadas — todas satisfeitas**.

---

## §5 — Cluster Gradient L3 emit estratégia única feature-complete 24/24

| Variant × ColorSpace | Pipeline L3 emit pós-P272 |
|---|---|
| Linear × 8 spaces | `/ShadingType 2` axial + Function Type 3 stitching (preserved P262-P270.2) |
| Radial × 8 spaces | `/ShadingType 3` radial + Function Type 3 stitching (preserved P264-P270.2) |
| **Conic × 7 RGB-family/perceptual** | **`/ShadingType 6` Coons + /DeviceRGB + N=stops*4 patches (P272 new)** |
| Conic × CMYK | `/ShadingType 6` Coons + /DeviceCMYK + N=stops patches (preserved P270.4) |

**Cluster 24/24 absoluto** preserved + simplificado arquiteturalmente
(estratégia única Coons; 2 estratégias coexistentes P270.4 convergidas).

---

## §6 — ADR-0090 REVOGADO + 4 anotações cumulativas P272 + L0

### §6.1 — ADR-0090 transição EM VIGOR → REVOGADO

`00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`:

- Status: `EM VIGOR` → `REVOGADO P272`.
- §"Revogação P272" nova adicionada documentando:
  - Motivo industry-aligned (Cairo/Inkscape/Typst blog 2023 mesh-based).
  - Helpers removed (`emit_conic_gouraud_stream` + `compute_adaptive_n_conic`
    + `oklab_delta_e`).
  - 20 tests P268+P268.2 removed.
  - Substituição ADR-0092 expandida cumulativamente.
  - Pattern ADR-0093 §Pattern 1 §"Quando NÃO aplicar" — revogação
    invalida decisão de fundo.
  - Sub-padrões "ADR REVOGADO + substituta" N=3 + "Aplicação meta-ADR
    (ADR-0093)" N=1 inaugural + "Aplicação meta-ADR (ADR-0094)" N=1
    inaugural.

### §6.2 — ADR-0092 anotação cumulativa P272 (expansão final)

`00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md` §"Anotação
cumulativa P272 — Decisão Cenário A revisado FINAL":

- Estratégia única Coons materializada (RGB + CMYK).
- Strategy N=stops*4 (RGB) vs N=stops (CMYK preserved).
- Helpers reutilizados literal + helpers removed.
- Tests delta -15.
- Cluster Gradient L3 emit estratégia unificada 24/24.
- Sub-padrões aplicados (8 cumulativos).

### §6.3 — ADR-0091 anotação cumulativa P272

§"Decisão L3" estendida; estratégia única Coons 8/8 spaces; cluster
24/24 simplificado.

### §6.4 — ADR-0089 anotação cumulativa P272

Conic 2 emit paths coexistentes P270.4 convergem em Type 6 Coons
único; ADR-0090 REVOGADO; ADR-0092 expandida.

### §6.5 — ADR-0054 anotação cumulativa P272

Cluster Gradient strategy unificada Coons; perfil graded DEBT-1
simplificado por eliminação 2 estratégias Conic coexistentes.
Sub-padrão "Aplicação meta-ADR" N=1 inaugural cada para ADR-0093 +
ADR-0094.

### §6.6 — L0 `entities/gradient.md` anotação P272

Anotação P272 adicionada (após P270.4):
- Estratégia unificada Coons.
- Strategy N=stops*4 (RGB) + N=stops (CMYK).
- Helpers removed + helpers reutilizados.
- Sub-padrões cumulativos.

Hash propagado automatic via `crystalline-lint --fix-hashes` (1
propagation `01_core/src/entities/gradient.rs:794c2e61` pré-build —
comment P272 sobre `color_to_oklab_with_alpha` ainda usada).

---

## §7 — Pendências preservadas pós-P272

Sem mudança vs P270.4 + P271:

- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **P-Gradient-CMYK-ICC** (S-M; PDF/A compliance).
- **P-Gradient-Adaptive-Multispace** (S; HSL/Oklch banding refino).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 / Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**Decisão humana fica em aberto literal** pós-P272 — cluster
Gradient L3 emit estratégia única materializada; arquitectura
simplificada; abre próximo cluster/refino.

---

## §8 — Verificação regressão zero P262-P270.4 não-Conic

**Linear/Radial/Conic CMYK tests preserved literal** (byte snapshots
bit-exact):

- typst-core: 2162 preserved.
- typst-shell: 24 preserved.
- typst-infra: 363 → 348 (-15 net: -20 P268+P268.2 + 5 novos P272).
- typst-wiring + bins: 23 preserved.

**Total: 2572 → 2557 (-15 net)**.

Mecânica: arm Conic dispatcher branchs `conic.space == Cmyk` →
`emit_conic_coons_stream_cmyk` (P270.4 preserved literal); `else` →
`emit_conic_coons_stream_rgb` (P272 new). Linear/Radial pipelines
intocados literal.

§política condições 8 absoluta satisfeita (regressão funcional zero
em não-Conic; behaviour change Conic RGB intencional per spec P272).

---

## §9 — Marco final P272

**Cluster Gradient L3 emit estratégia única Coons feature-complete
24/24 simplificado materializado**:

- 3 variants (Linear/Radial/Conic) × 8 spaces × emit completo.
- 1 estratégia Conic (Coons) para 8 spaces (vs 2 estratégias P270.4).
- Bug vanilla #4422 preservado resolvido.
- Industry-aligned mesh-based (Cairo/Inkscape/Typst blog 2023).
- Primeira aplicação prática meta-ADRs ADR-0093 + ADR-0094 P271.

Cristalino oferece pipeline gradient unificado, simplificado, e
industry-aligned. Série P-Gradient (P262-P272) consolidada em 11
passos cumulativos:

- P262: Linear L1.
- P263: Linear L3 PDF emit.
- P264: Radial L1.
- P265: Radial L3 PDF emit.
- P267: Conic L1.
- P268: Conic L3 Type 4 Gouraud (DESCONTINUADO P272).
- P268.1: ADR-0090 metodologia + correcção.
- P268.2: Adaptive N hybrid (DESCONTINUADO P272).
- P269: Radial focal_*.
- P270 série: ColorSpace runtime + L3 multi-space.
- P270.3: Coons RGB infra.
- P270.4: Coons CMYK ativação.
- **P272: Conic Coons unified 8/8 spaces** (P-Gradient-Coons-RGB-Final).

---

## §10 — Referências

- ADR-0090 — Type 4 Gouraud (REVOGADO P272).
- ADR-0092 — Conic Coons (expandida cumulativamente P272).
- ADR-0091 — ColorSpace runtime (anotada cumulativa P272).
- ADR-0089 — Gradient Conic-only (anotada cumulativa P272).
- ADR-0054 — Perfil graded (anotada cumulativa P272).
- ADR-0093 — Meta-metodologia evolução ADRs (primeira aplicação
  prática Pattern 1 §"Quando NÃO aplicar" P272).
- ADR-0094 — Meta-operacional specs (primeira aplicação prática
  Cap LOC hard/soft P272).
- ADR-0085 — Diagnóstico imutável (décimo terceiro consumo).
- `00_nucleo/diagnosticos/diagnostico-coons-rgb-final-passo-272.md`
  — diagnóstico imutável.
- ADR-0007/ADR-0028 — precedentes "ADR REVOGADO + substituta" N=2
  prévio.
- Spec P272 — `00_nucleo/materialization/typst-passo-272.md`.

---

*Relatório imutável produzido em 2026-05-17. Linhagem completa
preservada — cluster Gradient L3 emit estratégia única materializada;
arquitectura simplificada; primeira aplicação prática meta-ADRs
P271.*
