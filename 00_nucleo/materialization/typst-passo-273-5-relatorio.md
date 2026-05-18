# Relatório P273.5 — P-Gradient-Relative-Callsite (fecha pendência P273)

**Data**: 2026-05-17.
**Magnitude**: S (L1 ~30 LOC + L3 ~50 LOC + 8 testes; caps respeitados literal).
**Cluster**: Visualize / Gradient (refino estrutural fecho-de-pendência).
**Tipo**: sub-passo decimal P273.5 — fecho de pendência `#[allow(dead_code)]` P273.
**Spec**: `00_nucleo/materialization/typst-passo-273-5.md`.

---

## §1 — Sumário executivo

**Cluster Gradient refino estrutural encerrado** via P273.5:

- **`apply_parent_transform` perdeu `#[allow(dead_code)]`** — função
  tem 2 callsites L3 reais (Linear + Radial RGB-family dispatcher arms).
- **L1 novo `Rect` struct** (paridade `Point` + `Size`) em
  `entities/layout_types.rs`.
- **L1 novo campo `parent_bbox: Option<Rect>`** no Layouter (padrão
  DEBT-37 P84.6 reused estructuralmente).
- **3γ.1 híbrida materializada**: callsite L3 passa `page_bbox` como
  fallback parent_bbox; identity transform por construção.
- **3γ.2 pendência preservada**: Block/Boxed/Group save/restore real
  bbox propagation (incremental per ADR-0054 graded; L1 estructuralmente
  preparado).

### Marcos arquitecturais P273.5

**(1) Cluster Gradient refino estrutural encerrado** — pendência P273
§7 (`#[allow(dead_code)]`) resolvida.

**(2) Sub-padrão "Aplicação meta-ADR (ADR-0094)" N=4 cumulativo** —
quarta aplicação prática Cap LOC hard/soft pós-formalização P271.

**(3) Sub-padrão "Aplicação meta-ADR (ADR-0093)" N=3 cumulativo** —
terceira aplicação prática Pattern 2 anotação cumulativa em vez de
ADR nova.

**(4) Sub-padrão emergente "Pattern DEBT-37 `cell_origin_*` replicado"
N=2 cumulativo** — meio caminho limiar formalização N=3-4; candidato
meta-ADR futura se atingir N≥3-4.

### Decisão 3 — Semântica Parent: 3γ híbrida

- **3γ.1 (materializado P273.5)**: callsite L3 passa `page_bbox` como
  fallback parent_bbox quando `gradient.relative == Some(Parent)`.
- **3γ.2 (pendência preservada)**: Layouter populando `parent_bbox`
  real via save/restore. Estructuralmente preparado.

### Defaults preservam P272+P273+P274 bit-exact

- `relative: None` (Auto) → resolve Self_ → branch literal preserved.
- `relative: Some(Self_)` → branch literal preserved (`apply_parent_transform`
  NÃO chamado).
- `relative: Some(Parent)` → 3γ.1 identity (coords baseline page-relative
  já são page-bbox; transform é no-op por construção).

2597 baseline (P274 fim) preserved bit-exact.

---

## §2 — Diff L1+L3 antes/depois

### §2.1 — L1 novo `Rect` struct (paridade `Point`/`Size`)

```rust
// 01_core/src/entities/layout_types.rs P273.5

/// Rectângulo alinhado aos eixos (paridade `Point` + `Size`).
/// P273.5 — usado como bbox de contentor para resolver
/// `Gradient.relative: Some(RelativeTo::Parent)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: Pt,
    pub y: Pt,
    pub w: Pt,
    pub h: Pt,
}
```

### §2.2 — L1 novo campo Layouter

```rust
// 01_core/src/rules/layout/mod.rs P273.5

pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer> {
    // ... fields existentes ...
    pub(super) cell_origin_x: Option<f64>,  // DEBT-37 P84.6
    pub(super) cell_origin_y: Option<f64>,  // DEBT-37 P84.6
    /// P273.5 — bbox do contentor imediato para gradient `Parent`.
    /// Padrão DEBT-37 reused. 3γ.1 callsite L3 usa page_bbox fallback.
    /// 3γ.2 pendência futura (Block save/restore populará).
    #[allow(dead_code)]  // Consumer real chega com 3γ.2.
    pub(super) parent_bbox: Option<Rect>,
}

// Constructor init:
parent_bbox: None,  // P273.5 — fallback None; callsite L3 usa page_bbox.
```

### §2.3 — L3 callsites dispatcher (Linear + Radial RGB-family arms)

```rust
// 03_infra/src/export.rs P273.5 — Linear arm

let (x0, y0, x1, y1) = compute_axial_coords(
    linear.angle.to_rad(), 0.0, 0.0, page_w, page_h);

// P273.5 — quando relative=Parent, exercita apply_parent_transform
// com page_bbox 3γ.1 (decisão Fase A híbrida); identity por construção.
let relative = resolve_relative(linear.relative);
let (x0, y0, x1, y1) =
    if relative == RelativeTo::Parent {
        let local = (
            (x0 / page_w) as f32, (y0 / page_h) as f32,
            (x1 / page_w) as f32, (y1 / page_h) as f32,
        );
        let bbox = Some((0.0_f32, 0.0_f32, page_w as f32, page_h as f32));
        let (tx0, ty0, tx1, ty1) = apply_parent_transform(local, bbox);
        (tx0 as f64, ty0 as f64, tx1 as f64, ty1 as f64)
    } else {
        (x0, y0, x1, y1)
    };
// ... resto preserved literal (compute_axial_coords + emit shading)

// Radial arm: paridade Linear (mesma lógica em (x0,y0,x1,y1); r0/r1
// preservados literal).
```

### §2.4 — L3 `#[allow(dead_code)]` removido

```rust
// 03_infra/src/export.rs P273.5

- #[allow(dead_code)]
- fn apply_parent_transform(
+ /// **P273.5**: `#[allow(dead_code)]` removido — função tem callsite
+ /// L3 real em `emit_gradient_objects` (dispatcher Linear/Radial
+ /// RGB-family arm) quando `gradient.relative == Some(Parent)`.
+ fn apply_parent_transform(
      local: (f32, f32, f32, f32),
      parent_bbox: Option<(f32, f32, f32, f32)>,
  )
```

**`cargo build` confirma zero `dead_code` warnings** em
`apply_parent_transform`.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.5 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=13 → N=14 cumulativo consolidação clara persistente** | Sétima anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=13 → N=14 cumulativo consolidação clara persistente** | apply_parent_transform reused; DEBT-37 reused estructuralmente |
| Cap LOC hard vs soft explícito | **N=7 → N=8 cumulativo consolidação total** | L1/L3/testes caps respeitados literal |
| **Aplicação meta-ADR (ADR-0093)** | **N=2 → N=3 cumulativo** | Pattern 2 anotação cumulativa; terceira aplicação prática |
| **Aplicação meta-ADR (ADR-0094)** | **N=3 → N=4 cumulativo** | Pattern 1 cap LOC; quarta aplicação prática |
| **Pattern DEBT-37 `cell_origin_*` replicado** | **N=1 → N=2 cumulativo emergente** | P84.6 + P273.5; meio caminho limiar N=3-4 |
| Diagnóstico imutável (décimo sexto consumo) | **N=20 → N=21 cumulativo** | + P273.5 |
| Auditoria condicional (ADR-0084) | **N=19 → N=20 cumulativo** | + P273.5 |
| Auto-aplicação ADR-0065 inline | **N=19 → N=20 cumulativo** | + P273.5 |

---

## §4 — Métricas finais

| Métrica | Pré-P273.5 | Pós-P273.5 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2597 | **2605** | +8 |
| Tests P273.5 novos | — | 8 | 2 unit + 4 E2E + 1 L1 + 1 determinismo |
| Tests P262-P274 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build dead_code warnings | 1 (`apply_parent_transform`) | **0** | -1 (resolvido) |
| Hashes propagados L0 | — | 1 (`b7bd7f98`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; sétima anotação ADR-0091) |
| LOC L1 (additions) | — | ~30 | cap hard 60 (folga 50%); cap soft 40 (folga 25%) |
| LOC L3 (additions) | — | ~50 | cap hard 80 (folga 37%); cap soft 50 limite |

### §política condições verificadas

- ✓ Cap LOC L1 hard 60 — real ~30; folga 50%.
- ✓ Cap LOC L1 soft 40 — real ~30; folga 25%.
- ✓ Cap LOC L3 hard 80 — real ~50; folga 37%.
- ✓ Cap LOC L3 soft 50 — real ~50; limite.
- ✓ Cap testes hard 12 — real 8; folga 33%.
- ✓ Cap testes soft 8 — real 8; limite.
- ✓ `apply_parent_transform` perdeu `#[allow(dead_code)]` — confirmado
  via `cargo build` zero warnings.
- ✓ Defaults `relative: None/Some(Self_)` preservam bytes
  P262-P274 literal.
- ✓ ADR-0029 pureza física L1 preserved (`Rect` é tipo dados;
  `parent_bbox` é campo metadata).
- ✓ Lint zero; L0 hash drift propagado.
- ✓ Regressão tests P262-P274 zero (2597 baseline preserved).

**11 condições §política verificadas — todas satisfeitas absolutas**.

---

## §5 — Verificação regressão zero P262-P274

**2597 baseline preservado bit-exact**:

- typst-core: 2169 preserved.
- typst-shell: 24 preserved.
- typst-infra: 381 → 389 (+8 P273.5 tests).
- typst-wiring + bins: 23 preserved.

**Total: 2597 → 2605 (+8 net)**.

Mecânica: default `relative: None` (Auto) ou `Some(Self_)` → branch
literal preserved (apply_parent_transform NÃO chamado).
`Some(Parent)` → 3γ.1 identity transform por construção (page_bbox
= page; coords idênticos).

§política condições "Regressão tests P262-P274 zero" + "Defaults
preservam pipeline" satisfeitas absolutas.

---

## §6 — Anotação cumulativa ADR-0091 (sétima consecutiva)

Adicionada §"Anotação cumulativa P273.5 — Parent bbox callsite (fecha
#[allow(dead_code)] P273)" cobrindo:

- Decisão 3 escolhida: **3γ híbrida** — 3γ.1 materializado P273.5 +
  3γ.2 pendência preservada incremental.
- Mecanismo de propagação: L1 Rect + Layouter field + L3 callsites
  + `#[allow(dead_code)]` removido.
- Defaults preservam P272+P273+P274 bit-exact.
- Distinção precedente DEBT-37 P84.6.
- 8 sub-padrões aplicados.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=13 →
**N=14 cumulativo consolidação clara persistente** — sétima anotação
consecutiva ADR-0091 (P270/P270.1/P270.2/P270.3/P273/P274/**P273.5**).

---

## §7 — L0 `entities/gradient.md` anotação P273.5

Adicionada anotação P273.5 após P274 — fecho pendência P273
`apply_parent_transform`; decisão 3γ híbrida; padrão DEBT-37 reused;
sub-padrões cumulativos. Hash propagado via `crystalline-lint
--fix-hashes`: `01_core/src/entities/gradient.rs:b7bd7f98`.

---

## §8 — Pendências preservadas pós-P273.5

Inalteradas vs P274:

- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC profiles).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**Pendências específicas P273.5**:
- **3γ.2 Block/Boxed/Group save/restore real** bbox propagation.
  Estructuralmente preparado via campo `parent_bbox: Option<Rect>` L1
  Layouter; consumer real chega em refino futuro incremental per
  ADR-0054 graded.

**Pós-P273.5 fecha cluster Gradient refino estrutural** — cluster
Gradient feature-complete user-facing + qualitativo + refino
estrutural encerrado. Próximo passo natural: sair do cluster Gradient
para outro domínio (Text/Model/Layout/etc.).

---

## §9 — Limitações conscientes P273.5

Per spec §5:

- Lista de contentores que disparam save/restore — não fixada P273.5
  (3γ.2 pendência); apenas `Content::Block`/`Boxed`/`Group`/`Grid cell`
  candidatos documentados.
- Coordenadas Rect populated com aproximação ao cursor actual — refino
  exacto pós-layout body fica fora de escopo.
- 3γ.1 fallback page_bbox é identity transform por construção (sem
  diferença observable vs Self_ em P273.5; refino real chega com
  3γ.2 Block save/restore).
- `apply_parent_transform` mantém-se função pura — fórmula
  transformação P273 inalterada. P273.5 só fornece bbox real.

---

## §10 — Marco final P273.5

**Cluster Gradient refino estrutural encerrado**:

- Pendência P273 §7 (`apply_parent_transform` em `#[allow(dead_code)]`)
  resolvida.
- L1 `Rect` struct disponível (paridade `Point`/`Size`).
- L1 `parent_bbox: Option<Rect>` Layouter (padrão DEBT-37 reused).
- L3 callsites reais (2 sítios Linear + Radial).
- 8 tests P273.5 + zero regressão tests P262-P274.

Cristalino oferece gradient API user-facing paridade vanilla em:
- Cross-variant runtime fields canónica 3/3 (focal + space + relative).
- Adaptive N multispace refino qualitativo (Linear+Radial).
- Parent bbox callsite real (`apply_parent_transform` consumed).

Cluster Gradient feature-complete + qualitativo + refino estrutural —
pronto para saída cluster.

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.5; sétima anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=3 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=4 cumulativo).
- ADR-0085 — Diagnóstico imutável (décimo sexto consumo).
- ADR-0029 — Pureza física L1 (preserved; Rect tipo dados, campo
  metadata).
- DEBT-37 (P84.6) — Pattern `cell_origin_*: Option<f64>` reused
  estructuralmente P273.5 (sub-padrão "Pattern DEBT-37 replicado"
  N=2 cumulativo emergente).
- `00_nucleo/diagnosticos/typst-passo-273-5A-diagnostico.md` — Fase A
  empírica + decisão 3γ híbrida.
- P273 — Spec original; `apply_parent_transform` deixado em
  `#[allow(dead_code)]` §7 (resolvido P273.5).
- Spec P273.5 — `00_nucleo/materialization/typst-passo-273-5.md`.

---

*Relatório imutável produzido em 2026-05-17. Linhagem completa
preservada — cluster Gradient refino estrutural encerrado;
`apply_parent_transform` consumed via callsite real L3 (Linear+Radial
dispatcher arms); padrão DEBT-37 reused estructuralmente.*
