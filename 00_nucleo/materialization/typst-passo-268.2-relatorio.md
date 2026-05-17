# Relatório P268.2 — Refino Conic PDF adaptive N hybrid 1+2 (Type 4 Gouraud qualidade visual industry-grade)

**Data**: 2026-05-15.
**Magnitude**: S (real ~95 LOC L3 + 1 keyword L1 pub + 15 testes; cap 200 LOC L3 / 15 testes — exacto).
**Cluster**: Visualize / Gradient / PDF export (refino qualidade visual).
**Tipo**: refino numerado .2 dentro da série P268.
**Spec**: `00_nucleo/materialization/typst-passo-268.2.md`.

---

## §1 — Sumário executivo

Refino paramétrico do P268 (PDF Conic Type 4 Gouraud) — substitui
N=32 fixo por adaptive N hybrid 1+2 (critério 1 número de stops +
critério 2 contraste Oklab ΔE). **Sem ADR nova** — anotação
cumulativa ADR-0089 §P268.2. **ADR-0090 preservada literal**
(estratégia Type 4 intocada — só parâmetro N refinado).

### Marco arquitectural P268.2

**Cluster Gradient PDF qualidade visual industry-grade** — adaptive
N elimina banding observable em casos extremos (muitos stops ou
contraste alto), mantendo estratégia Type 4 ADR-0090 intocada.
Cristalino Type 4 qualitativamente competitivo com Cairo Type 6/7
sem aumentar magnitude implementação.

**Primeira aplicação do padrão "refino paramétrico preservando ADR
estratégica"** (N=1 inaugural) — P268.2 melhora qualidade visual
sem revogar ADR-0090; precedente para futuros refinos onde ajuste
de constante/parâmetro melhora comportamento sem mudar decisão
arquitectural fundamental.

### Descoberta empírica P268.2.A

Spec original P268.2 §A.5 propôs `factor_delta = 2.0` assumindo ΔE
em escala CIELab (~0-100; ΔE(red,blue) ≈ 70). Verificação empírica
cristalino in-situ (cálculo Python via helpers L1 reproduzidos)
confirma que helpers Oklab L1 implementam **Oklab canónico**
(Björn Ottosson + W3C CSS Color 4): ΔE_OK ∈ [0, ~1.2];
ΔE_OK(red, blue) ≈ 0.537 (não ~70).

**Recalibração `factor_delta = 256.0`** mantém fórmula estrutural
intacta e produz comportamento alvo (N=128 para contraste máximo +
N=32 preservado para pastel). §política condição 2 não accionada —
apenas constante muda, não fórmula nem magnitude.

### Decisão materializada

- **15 tests novos** (8 unit + 4 E2E PDF + 3 snapshot determinístico)
  — todos verdes.
- **6 tests P268 originais preservados literal** — zero regressões.
- **2 helpers L3 novos** (`oklab_delta_e` + `compute_adaptive_n_conic`).
- **1 keyword L1** (`color_to_oklab_with_alpha` promovido a `pub fn`
  cross-crate; function body preservada literal).
- **1 callsite production alterado** (`emit_gradient_objects` Conic
  branch).
- **3 anotações cumulativas** (ADR-0089 + ADR-0054 + L0
  `entities/gradient.md`).
- **1 diagnóstico imutável** (`diagnostico-adaptive-n-passo-268-2.md`).
- **README ADRs** anotado entrada P268.2 + linha tabela ADR-0089
  estendida com "+ anotação cumulativa P268.2".

---

## §2 — Diff `emit_conic_gouraud_stream` antes/depois

### Antes P268.2 (callsite production)

```rust
// 03_infra/src/export.rs:1175 (P268)
let _ = oklab_sample_stops_conic(conic, 16);
let stream = emit_conic_gouraud_stream(conic, 32);  // N=32 fixo
```

### Depois P268.2 (callsite production)

```rust
// 03_infra/src/export.rs:1175 (P268.2)
let _ = oklab_sample_stops_conic(conic, 16);
let n_adaptive = compute_adaptive_n_conic(conic);   // adaptive hybrid 1+2
let stream = emit_conic_gouraud_stream(conic, n_adaptive);
```

### Helpers L3 novos

```rust
fn oklab_delta_e(c1: Color, c2: Color) -> f32 {
    let (l1, a1, b1, _) = color_to_oklab_with_alpha(c1);  // L1 pub P268.2
    let (l2, a2, b2, _) = color_to_oklab_with_alpha(c2);
    let dl = l1 - l2;
    let da = a1 - a2;
    let db = b1 - b2;
    (dl*dl + da*da + db*db).sqrt()
}

fn compute_adaptive_n_conic(conic: &Conic) -> usize {
    const N_BASE: usize = 32;
    const N_MIN: usize = 8;
    const N_MAX: usize = 128;
    const FACTOR_DELTA: f32 = 256.0;  // Oklab canónico — ver §A.5 diagnóstico

    let num_stops = conic.stops.len();
    if num_stops < 2 { return N_MIN; }

    let n_stops = num_stops.saturating_sub(2) * 8;
    let sum_delta_e: f32 = conic.stops.windows(2)
        .map(|p| oklab_delta_e(p[0].color, p[1].color))
        .sum();
    let n_delta = (sum_delta_e * FACTOR_DELTA) as usize;

    let n_adaptive = N_BASE.max(n_stops + n_delta);
    n_adaptive.clamp(N_MIN, N_MAX)
}
```

### Função `emit_conic_gouraud_stream` — body intocado

Assinatura `fn emit_conic_gouraud_stream(conic, n_slices) -> Vec<u8>`
preservada literal; corpo da função intocado. Adaptive N entra
apenas no callsite production. **Decisão arquitectural §A.2
diagnóstico**: preservar assinatura 2-arg para garantir zero
regressão dos 6 tests P268 originais (alguns passam `n_slices=32`
ou `n_slices=4` literal).

### L1 — 1 keyword modificada

```rust
// 01_core/src/entities/gradient.rs:163 (antes P268.2)
fn color_to_oklab_with_alpha(c: Color) -> (f32, f32, f32, f32) {

// depois P268.2
pub fn color_to_oklab_with_alpha(c: Color) -> (f32, f32, f32, f32) {
```

Body preservado literal. Doc comment ampliado para registar
promoção P268.2.

---

## §3 — factor_delta=256.0 confirmado empiricamente

### Verificação cristalino in-situ (cálculo Python via helpers L1 reproduzidos)

| Caso | num_stops | sum ΔE_OK | n_stops | n_delta | N final |
|---|---|---|---|---|---|
| 2 stops red↔blue | 2 | 0.537 | 0 | 137 → clamp 128 | **128** |
| 2 stops black↔white | 2 | 1.000 | 0 | 256 → clamp 128 | **128** |
| 2 stops pastel pink↔azure | 2 | 0.073 | 0 | 19 | **32** (max(32, 19)) |
| 2 stops red↔orange | 2 | 0.155 | 0 | 40 | **40** |
| 5 stops moderado (R→O→Y→G→B) | 5 | 1.324 | 24 | 339 | **128** (clamp) |
| 8 stops pastel | 8 | 0.306 | 48 | 78 | **126** |
| 1 stop (degenerado) | 1 | — | — | — | **8** (N_MIN) |
| 2 stops iguais ΔE=0 | 2 | 0.0 | 0 | 0 | **32** (N_BASE) |

### Conferência empírica vs tests P268.2

Todos os 15 tests P268.2 passaram com factor_delta=256.0:
- `p268_2_adaptive_n_2_stops_pastel_preserva_32` → 32 ✓
- `p268_2_adaptive_n_2_stops_red_blue_clamp_128` → 128 ✓
- `p268_2_adaptive_n_5_stops_moderados` → 128 ✓
- `p268_2_adaptive_n_8_stops_pastel` → 126 ✓ (janela [60, 128])
- `p268_2_adaptive_n_1_stop_degenerado_n_min` → 8 ✓
- `p268_2_adaptive_n_stops_identicos_delta_zero` → 32 ✓
- `p268_2_adaptive_n_clamp_n_max_128` → 128 ✓
- `p268_2_oklab_delta_e_helper_red_blue` → 0.537 ✓ (janela [0.5, 0.6])

### Justificação factor_delta=256

- **black↔white = N_MAX**: ΔE_OK máximo possível ≈ 1.0; factor × 1.0
  = 256 → clamp 128. ✓
- **red↔blue = N_MAX**: ΔE_OK ≈ 0.54; factor × 0.54 = 137 → clamp
  128. ✓ (cobre alvo spec "contraste máximo → 128 fatias").
- **Pastel preserva N_BASE=32**: ΔE_OK ~0.05-0.1; factor × 0.1 = 25
  ≤ N_BASE. ✓
- **256 ≈ 1 / threshold perceptual Oklab (0.004)**: cada unidade
  ΔE_OK gera ~256 fatias máximo.

---

## §4 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P268.2 | Nota |
|---|---|---|
| **Refino paramétrico preservando ADR estratégica** | **N=1 inaugural** | P268.2 inaugura (ajuste de constante FACTOR_DELTA + N adaptive vs fixo; ADR-0090 Type 4 strategy intocada) |
| Anotação cumulativa em vez de ADR nova | **N=5 → N=6** | + P268.2 (P258.B/P259.B/P263/P265/P268/**P268.2**) |
| Reutilização literal helpers cross-passos | **N=3 → N=4** | + P268.2 (`color_to_oklab_with_alpha` P262 reutilizado literal por L3 — pub mudança 1 keyword; body intocado) |
| Auditoria condicional (ADR-0084) | **N=9 → N=10** | + P268.2 (Fase A diagnóstico empírico criou `diagnostico-adaptive-n-passo-268-2.md` imutável; sub-passo .A nucleação) |
| Diagnóstico imutável (ADR-0085; fonte estendida) | **N=10 → N=11** | + P268.2 (sexto consumo directo de fonte: P262/P264/P267/P268 vanilla + P268.1 web + **P268.2 literatura técnica perceptual**) |
| Auto-aplicação ADR-0065 inline | **N=8 → N=9** | + P268.2 (cap real respeitado; tests-primeiro disciplinado) |
| Diagnóstico empírico web em vez de filesystem | **N=1 → N=2** | + P268.2 (consulta canónica Oklab/CSS Color 4 + cálculo cristalino in-situ; estende P268.1) |
| Descoberta empírica que recalibra spec autor | **N=1 inaugural** | P268.2 (spec §A.5 propôs factor=2.0 com escala CIELab; cristalino usa Oklab canónico → factor=256.0; fórmula intacta; magnitude intacta) |

---

## §5 — Métricas finais

| Métrica | Pré-P268.2 | Pós-P268.2 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2413 | **2428** | +15 |
| Tests P268.2 novos | — | 15 | +15 (8 unit + 4 E2E + 3 snapshot) |
| Tests P268 originais (verdes) | 6 | 6 | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 (L0 gradient.md) | +1 |
| ADRs totais | 77 | 77 | **0** (sem ADR nova) |
| LOC L1 alterado | — | 1 keyword (`pub`) | minimal |
| LOC L3 alterado | — | ~95 (helpers + callsite) | ≤ 200 cap |
| Ficheiros documentais editados | — | 5 | ADR-0089, ADR-0054, L0 gradient.md, README ADRs, gradient.rs header |
| Ficheiros documentais criados | — | 2 | diagnóstico + relatório |

**Cap LOC L3 respeitado**: ~95 LOC < 200 (§política condição 4 não
accionada).

**Cap testes respeitado**: 15 exacto (§política condição 5 não
accionada).

**Zero regressão P268 originais**: 6 tests passam literal (§política
condição 9 não accionada).

**N=32 preservado para 2 stops pastel**: confirmado test
`p268_2_adaptive_n_2_stops_pastel_preserva_32` (§política condição
10 não accionada).

**Cluster 3 variants test**: `p268_2_export_pdf_regression_p268_cluster_3_variants`
passa (§política condição 11 não accionada).

**Snapshot bytes reproduzíveis**: 3 tests determinísticos passam
(§política condição 6 não accionada).

---

## §6 — Anotações cumulativas materializadas

### §6.1 — ADR-0089 anotação cumulativa P268.2

Adicionada após §"Anotação cumulativa P268.1" existente.

**Conteúdo essencial**: fórmula adaptive N hybrid 1+2 com
factor_delta=256.0 recalibrado; ADR-0090 preservada literal;
helpers Oklab P262 reutilizados literal (pub mudança 1 keyword);
6 tests P268 originais preservados literal; cluster Gradient PDF
industry-grade; subpadrão "Refino paramétrico preservando ADR
estratégica" N=1 inaugural; subpadrão "Anotação cumulativa em vez
de ADR nova" N=5 → N=6.

Status `IMPLEMENTADO` preservado literal.

### §6.2 — ADR-0054 anotação cumulativa P268.2

Adicionada após §"Anotação cumulativa P268.1" existente.

**Conteúdo essencial**: refino qualidade visual sem mudar estratégia
ADR-0090; cluster Gradient PDF cristalino industry-grade; perfil
graded DEBT-1 preservado (refino é optimização local, não
simplificação).

Status `EM VIGOR` preservado literal.

### §6.3 — L0 `entities/gradient.md` anotação P268.2

Adicionada após anotação P268.1 existente.

**Conteúdo essencial**: callsite production usa N adaptive hybrid
1+2; fórmula §A.6 `diagnostico-adaptive-n-passo-268-2.md`;
factor_delta=256.0 calibrado Oklab canónico; ADR-0090 preservada
literal; helper privado `color_to_oklab_with_alpha` promovido a
`pub fn`; assinatura `emit_conic_gouraud_stream(conic, n_slices)`
preservada literal — zero regressão P268.

Hash propagado via `crystalline-lint --fix-hashes` (1 ficheiro
afectado: `01_core/src/entities/gradient.rs` header).

---

## §7 — README ADRs distribuição

### Linha tabela ADR-0089

Estendida com "+ anotação cumulativa P268.2 ..." (paridade pattern
P263/P265/P268 sobre ADR-0087/0088/0089).

### Total ADRs

**77 preservado** (P268.2 sem ADR nova).

### Distribuição

| Status | Pré-P268.2 | Pós-P268.2 | Delta |
|---|---|---|---|
| `PROPOSTO` | 11 | 11 | 0 |
| `IDEIA` | 2 | 2 | 0 |
| `EM VIGOR` | 33 | 33 | 0 |
| `IMPLEMENTADO` | 29 | 29 | 0 |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| **Total** | **77** | **77** | **0** |

### Passos-chave

Nova entrada `- **Passo 268.2**` adicionada após Passo 268.1.
~85 linhas (paridade entrada P268 estendida via subpadrões
cumulativos novos).

### Cobertura Visualize agregada

~75% (P267/P268) → **~76% pós-P268.2** (+1pp via qualidade visual
industry-grade; F.3 Gradient Conic agora bandas eliminadas em
casos extremos).

---

## §8 — Critério aceitação checklist

- [x] **Fase A diagnóstico** `diagnostico-adaptive-n-passo-268-2.md`
      criado (§A.1-§A.9; imutável per ADR-0085).
- [x] **factor_delta empiricamente justificado** §A.5 (recalibrado
      256.0 vs spec original 2.0; estrutura fórmula preservada;
      §política condição 2 não accionada).
- [x] **ADR-0089 anotada P268.2** após anotação P268.1.
- [x] **ADR-0054 anotada P268.2** após anotação P268.1.
- [x] **L0 `entities/gradient.md` anotado P268.2** após anotação
      P268.1; hash propagado.
- [x] **ADR-0090 preservada literal** (status EM VIGOR; estratégia
      Type 4 intocada).
- [x] **15 tests-primeiro** (8 unit + 4 E2E + 3 snapshot) adicionados
      antes do código L3.
- [x] **2 helpers L3 novos** (`oklab_delta_e` + `compute_adaptive_n_conic`)
      ~30 LOC.
- [x] **1 keyword L1** (`pub fn color_to_oklab_with_alpha`).
- [x] **1 callsite production** ajustado para adaptive N.
- [x] **Assinatura `emit_conic_gouraud_stream`** preservada literal.
- [x] **README ADRs** linha tabela ADR-0089 estendida; passo 268.2
      §"passos-chave"; total 77 preservado.
- [x] **Cap LOC L3 respeitado** (~95 < 200).
- [x] **Cap testes respeitado** (15 exacto).
- [x] **Tests workspace** 2413 → 2428 (+15; zero regressões P268).
- [x] **Lint zero violations** pós `--fix-hashes`.
- [x] **Snapshot bytes reproduzíveis** (3 tests determinísticos).
- [x] **Build cargo** exit 0.

**§política condições NÃO accionadas**:
- 1 (Oklab ΔE thresholds confirmados Björn Ottosson + W3C CSS Color 4).
- 2 (factor recalibrado sem mudar fórmula nem estoutar magnitude).
- 3 (gap helpers Oklab P262 ≤ 30 LOC — 1 keyword `pub` + ~8 LOC wrapper).
- 4 (cap L3 200 LOC respeitado — ~95 LOC totais).
- 5 (cap testes 15 exacto).
- 6 (determinismo float — Oklab f32 reproduzível confirmado por 3
  tests snapshot).
- 7 (lint zero pós `--fix-hashes`).
- 8 (ADR-0090 intocada — só parâmetro N refinado).
- 9 (zero regressão P268 — 6 tests originais permanecem verdes
  literal).
- 10 (N=32 preservado para 2 stops pastel — caso comum).
- 11 (cluster 3 variants test preservado verde).

---

## §9 — Pendências preservadas pós-P268.2

- **P-Gradient-Focal** (futuro M) — activa `focal_*` Radial; revoga
  ADR-0088 §focal scope-out.
- **ADR-0055bis variant-aware fonts** (M) — refino Text.
- **P-Footnote-N** (M) — Model pendência.
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling** (Paint::Tiling activação).
- **space/relative custom** gradient (futuros; preservados).
- **anti-aliasing** PDF default (preservado).

Decisão humana fica em aberto literal pós-P268.2.

---

## §10 — Referências

### Cross-passos

- **P268** — PDF Conic Type 4 Gouraud N=32 fixo (precedente directo
  refinado).
- **P268.1** — ADR-0090 EM VIGOR formalizando Type 4 (preservada
  literal por este passo).
- **P267** — Gradient Conic L1+stdlib (ADR-0089).
- **P262** — Gradient Linear L1+stdlib (helpers Oklab origem).
- **P263** — PDF Linear `/ShadingType 2` (helpers Oklab origem).
- **P265** — PDF Radial `/ShadingType 3` (helpers Oklab N=16
  reutilizados — N=1 inaugural reutilização).

### ADRs

- **ADR-0089** — Gradient Conic-only (anotação cumulativa P268.2
  com fórmula completa + factor_delta calibrado).
- **ADR-0090** — Type 4 Gouraud strategy (preservada literal por
  P268.2).
- **ADR-0054** — Perfil graded DEBT-1 (anotação cumulativa P268.2).
- **ADR-0018** — Whitelist crates (preservada; krilla não autorizada).
- **ADR-0085** — Diagnóstico imutável (sexto consumo directo de
  fonte; primeiro consumo de literatura técnica perceptual).
- **ADR-0084** — Auditoria condicional (Fase A diagnóstico P268.2.A).

### Documentos cristalinos editados

- `03_infra/src/export.rs` (~95 LOC L3 adicionado: helpers +
  callsite ajustado + 15 testes).
- `01_core/src/entities/gradient.rs` (1 keyword `pub` +
  doc comment ampliado; header hash propagado).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` (anotação
  cumulativa P268.2).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` (anotação
  cumulativa P268.2).
- `00_nucleo/prompts/entities/gradient.md` (anotação P268.2; hash
  propagado).
- `00_nucleo/adr/README.md` (tabela ADR-0089 estendida + passos-chave
  P268.2).
- `00_nucleo/diagnosticos/diagnostico-adaptive-n-passo-268-2.md`
  (criado imutável).
- `00_nucleo/materialization/typst-passo-268-2-relatorio.md` (este
  relatório).

### Literatura técnica perceptual (P268.2 primeiro consumo)

- **Björn Ottosson — "A perceptual color space for image processing"**
  (2020) — fonte canónica Oklab; ΔE_OK definição.
- **W3C CSS Color Module Level 4 §10 "OKLab"** — Candidate
  Recommendation; ΔE_OK como sqrt euclidiana directa.

### Vanilla literal (verificável)

- `lab/typst-original/crates/typst-pdf/src/paint.rs:242-267` —
  `SweepGradient` krilla; sem adaptive N (Type 1 pixel-perfect por
  construção).
- `lab/typst-original/crates/typst-pdf/src/convert.rs:514` — warning
  "conic gradients are not supported in this PDF standard".
