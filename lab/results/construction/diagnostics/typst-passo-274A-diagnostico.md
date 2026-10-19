# Diagnóstico Fase A P274.A — Adaptive N multispace refino qualitativo

**Data**: 2026-05-17.
**Passo**: typst-passo-274.A.
**Magnitude**: S documental (~30-40 min).
**Cluster**: Visualize / Gradient (continuação pós feature-complete user-facing P273).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 + ADR-0094 Pattern 3.
**Décimo quinto consumo directo de fonte** (cristalino post-P273 + W3C CSS Color 4 §11 + Björn Ottosson Oklab paper 2020 + Skia gradient densification + Cairo mesh gradient docs reutilizados).

---

## §A.1 — Inventário do estado pós-P273

### L1 `entities/gradient.rs`

3 structs cross-variant runtime fields canónica 3/3:

```rust
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,
}
pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,
    pub focal_radius: Ratio,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,
}
pub struct Conic {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle: Angle,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,
}
```

`color_to_oklab_with_alpha` é `pub` (promovido P268.2; preserved P272/P273) — disponível para reuso L3.

### Helpers L3 pré-amostragem actuais

`03_infra/src/export.rs`:

- `multispace_sample_stops(linear, n_samples)` — P270.1; 7 spaces RGB-family + perceptual (Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV); dispatcher arm CMYK preserved P270.2 não usa.
- `multispace_sample_stops_radial(radial, n_samples)` — P270.1; paridade Linear.
- `multispace_sample_stops_conic(conic, n_samples)` — P270.1; preserved como utilitário de teste pós-P272 (Conic L3 emit pós-P272 usa estratégia Coons N=stops*4 directo, não pré-amostragem).
- `multispace_sample_stops_linear_cmyk(linear, n_samples)` — P270.2; CMYK directo.
- `multispace_sample_stops_radial_cmyk(radial, n_samples)` — P270.2.
- `emit_conic_coons_stream_rgb(conic)` — P272; N=stops*4 (estratégia única Coons RGB).
- `emit_conic_coons_stream_cmyk(conic)` — P270.4; N=stops (Coons CMYK preserved literal).

### Callsites dispatcher pré-amostragem

`03_infra/src/export.rs:1577` e `:1617` — Linear/Radial RGB-family + perceptual:
- `multispace_sample_stops(linear, 16)` → **N=16 hardcoded** (target P274 substituir por adaptive).
- `multispace_sample_stops_radial(radial, 16)` → **N=16 hardcoded** (target P274 substituir).

Conic dispatcher P272 não usa pré-amostragem; preserved literal P274.

### Helpers Oklab disponíveis

`01_core/src/entities/gradient.rs`:

- `color_to_oklab_with_alpha(c: Color) -> (f32, f32, f32, f32)` (pub; P262/P268.2/P272/P273 reused).
- `interpolate_oklab(c0, c1, t)` (private; usado em dispatcher `interpolate_in_space`).
- `interpolate_in_space(c0, c1, t, space)` (private; dispatcher P270).

**Reusabilidade P274**: `color_to_oklab_with_alpha` é o único helper L1 directamente reutilizado em L3 (cross-crate; pub). Sub-padrão "Reutilização literal helpers cross-passos" N=12 → **N=13 cumulativo** P274.

---

## §A.2 — Análise de risco

| Risco | Fonte | Mitigação P274 |
|---|---|---|
| Regressão tests P270.1 originais | Fórmula adaptive devolver N≠16 para inputs P270.1 | §A.7: tests P270.1 são determinismo (`pdf1==pdf2`); qualquer fórmula deterministic preserva. Unit tests explicitamente passam N=16 → inalterados. |
| Stream PDF explode (N=128+) | Cap insuficiente | §A.3 Decisão 3: N_max=64 (cap soft 4x N_base=16; tolerável). |
| Acoplamento residual ao Oklab | Helper P268.2 estilo (removido P272) | §A.6 helper genérico `perceptual_distance_in_space(c0, c1, space)` — `space` param futuro-proof; métrica nativa Oklab universal per W3C CSS Color 4. |
| Quebra ADR-0091 (centro ColorSpace) | Lógica adaptive escolher path errado por space | Adaptive **dentro** do path de cada space (não cross-space); ADR-0091 §"Decisão L3" preserved. |
| Quebra paridade Conic P272 | Conic byte snapshots mudam | Decisão 2A: Conic preserved literal (escopo P272 encerrado); §A.3 fundamentado. |
| ADR-0029 pureza física L1 invadida | Helper L3 chama L1 com state mutável | `color_to_oklab_with_alpha` é função pura; sem state mutável. ✓ |

---

## §A.3 — Decisões fixadas

### Decisão 1 — Fórmula adaptive N: **Opção 1B threshold-based discreto**

**Fórmula final**:

```rust
fn adaptive_n_for_stops(stops: &[GradientStop], _space: ColorSpace) -> usize {
    if stops.len() < 2 {
        return 16;  // degenerado; preserva baseline P270.1
    }
    let max_delta_e = stops.windows(2)
        .map(|pair| perceptual_distance_in_space(
            &pair[0].color, &pair[1].color, _space))
        .fold(0.0_f32, f32::max);

    if max_delta_e < 0.05 {
        16   // low contrast pastel (preserva P270.1 emit literal)
    } else if max_delta_e < 0.3 {
        32   // moderate contrast
    } else {
        64   // high contrast (cap N_max)
    }
}
```

**Critério satisfação**:
- 1B é **discreto** (3 níveis); simples de testar e razoar.
- 1B é **monotonico não-decrescente** em `max_delta_e` (propriedade observable).
- 1B preserva **paridade P270.1 emit literal** para casos low-contrast (ΔE < 0.05; pastel).
- 1B **diferencia "moderado" vs "extremo"** (32 vs 64).
- 1B **não explode stream** (N_max=64; 4× N_base=16).
- 1B **agnóstico ao `space`** (per §A.6 helper genérico; param preservado para futuro-proofing).

**Rejeitadas**:

- **Opção 1A (P268.2 hybrid literal; N_base 16 + stops*8 + delta*256)**: clamp 128 em casos contraste moderado (β: red-white-blue → N=128); stream explode 8× P270.1. Acoplava factor_delta ao Oklab. Confirmação empírica da remoção P272.
- **Opção 1C (continuous interpolation lerp(16, 64, t = max_delta/0.2))**: contínua → não-testável discreto; lerp `t = clamp(max_delta_e / 0.2)` truncado em 1.0 saturado para casos moderados/altos; menor diferenciação.

### Decisão 2 — Variants afectados: **Opção 2A (Linear+Radial apenas)**

**Justificativa**:
- **Conic preserved P272** literal — estratégia Coons N=stops*4 já é função do número de stops por construção (densidade aumenta com stops). Sem evidência empírica banding Conic baseline P272 pré-sessão.
- **CMYK preserved P270.2** literal — sem pré-amostragem em primeiro lugar (`/DeviceCMYK` nativo + Function Type 3 stitching directo).
- **Linear+Radial RGB-family + perceptual** — único path onde N hardcoded 16 ocorre; alvo natural.

**Rejeitada**:
- **Opção 2B (Linear+Radial+Conic com multiplier sobre stops*4)**: expande escopo cluster encerrado P273; risco regressão tests P272 Coons; multiplier semantic não bem definido para "1 patch per stop" baseline.

### Decisão 3 — Cap N_max + ΔE units: **N_max=64; ΔE Oklab nativo**

**N_max=64**: cap soft 4× N_base=16; stream PDF tolerável (2 stops × 32-64 samples = 64-128 stops Function Type 3; bem dentro PDF spec limits).

**ΔE Oklab nativo** (sem multiplier P268.2 `*256.0`): coordenadas Oklab são já normalizadas [0..1.2] approx per Björn Ottosson + W3C CSS Color 4. Thresholds 0.05 / 0.3 são em unidades Oklab nativas (perceptibilidade ΔE ≈ 0.02-0.05 limiar typical).

---

## §A.4 — Fontes canónicas citadas

- **W3C CSS Color 4** §11 (ΔE Oklab definition; threshold perceptibilidade ΔE ≈ 1-2 unidades CIE LCH; Oklab analogo).
- **Björn Ottosson — Oklab paper** (2020) §"Comparing colors" — ΔE = euclidean distance em coordenadas Oklab nativas; range típico [0..1.2].
- **Skia gradient stop densification** — referência operacional aplicada para banding suppression em GPU gradients (pattern de adaptive stops antecipando GPU sampling).
- **Cairo pattern_set_extend / mesh gradient docs** — referência operacional Type 6 + adaptive stops para print-quality rendering.

---

## §A.5 — Matriz de casos concretos (3 fórmulas × 4 casos)

ΔE Oklab nativo computado via `color_to_oklab_with_alpha` literal (mesma fórmula proposta §A.6):

| Caso | Stops | max_pair_delta_e | 1A literal P268.2 (clamp 128) | 1B threshold | 1C continuous lerp(16,64) |
|---|---|---|---|---|---|
| α (light-gray near-identical) | rgb(250,250,250), rgb(245,245,245) | ≈ 0.017 (verificado teste) | 16 + 0 + 4 = 20 | **16** ✓ paridade | 16 + 0.085*(48) = 20 |
| α' (pastel saturado revisado) | rgb(255,200,200), rgb(200,255,200) | ≈ 0.155 (verificado teste) | 16 + 0 + 39 = 55 | **32** moderate | 16 + 0.775*(48) = 53 |
| β (3 moderados) | red, white, blue | ≈ 0.65 (max white-red ou white-blue) | 16 + 8 + (0.65*256≈166) = **128** ⚠️ explode | **64** | 16 + 1.0*(48) = 64 (clamp) |
| γ (5 alta sat) | red, yellow, green, blue, magenta | ≈ 0.7 (max pair) | 16 + 24 + 179 = **128** ⚠️ explode | **64** | 64 (clamp) |
| δ (8 extremo) | black, white, black, white, ... | ≈ 1.0 (max pair) | 16 + 48 + 256 = **128** ⚠️ explode | **64** | 64 (clamp) |

**Nota empírica revisada**: pastels saturados como (255,200,200) vs
(200,255,200) caem em N=32 (moderate; ΔE Oklab ≈ 0.15). Só
light-gray quase idênticas (Δ<0.05) preservam N=16 baseline literal.
Estimativa §A.5 original "≈ 0.04" subestimava a contribuição dos
canais cromáticos a/b — pastels saturados têm ΔE Oklab realmente
moderado (a/b coordinates relevantes). **Comportamento P274
correcto**: N=32 para pastels saturados é refino qualitativo
appropriate (mais samples no Function Type 3 stitching).

**Análise comparativa**:
- **1A clamp 128 em β/γ/δ**: stream PDF 8× baseline; comportamento "tudo ou nada" não-graduado. Rejeitado.
- **1B threshold discreto**: 16/32/64 níveis claros; diferenciação observable; cap em 64 (tolerável); satisfaz **todos 3 critérios** §A.3.
- **1C continuous**: idêntico a 1B em β/γ/δ (saturado em 64); apenas α difere ligeiramente (26 vs 16). Menos diferenciação P270.1 emit literal em α. 1B preferido.

**Conclusão**: **Opção 1B threshold-based é a escolha óptima** — fundamento numérico ancorado §A.5 satisfazem todos critérios §A.3.

---

## §A.6 — Helper proposto

### `perceptual_distance_in_space` (helper genérico cross-space)

```rust
/// P274 — Distância perceptual entre duas cores num space dado.
///
/// Métrica: ΔE Oklab (independente do space de entrada; converte cada
/// cor para Oklab e calcula distância euclidiana per Björn Ottosson
/// 2020 + W3C CSS Color 4 §11).
///
/// **Parâmetro `_space` reservado** — não altera a métrica actual.
/// ADR-0094 Pattern 2 (antecipa reutilização sem custo): call site
/// `perceptual_distance_in_space(c0, c1, gradient.space)` é
/// auto-documentado e helper não precisa renomear no futuro se um
/// space específico justificar métrica nativa diferente.
///
/// Distinção vs P268.2 `oklab_delta_e` (removido P272): assinatura
/// recebe `space` param; semantically cross-space-aware (mesmo que
/// implementation actual ignora). Desacoplamento por construção.
fn perceptual_distance_in_space(
    c0: typst_core::entities::layout_types::Color,
    c1: typst_core::entities::layout_types::Color,
    _space: typst_core::entities::layout_types::ColorSpace,
) -> f32 {
    let (l0, a0, b0, _) = typst_core::entities::gradient::color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, _) = typst_core::entities::gradient::color_to_oklab_with_alpha(c1);
    let dl = l1 - l0;
    let da = a1 - a0;
    let db = b1 - b0;
    (dl * dl + da * da + db * db).sqrt()
}
```

### `adaptive_n_for_stops` (wrapper N adaptive Linear+Radial)

```rust
/// P274 — Computa N adaptive para pré-amostragem Linear/Radial.
///
/// Fórmula §A.3 Decisão 1 Opção 1B threshold-based:
/// - N=16 se max_pair_delta_e < 0.05 (low contrast; preserva
///   paridade P270.1 emit literal para pastel).
/// - N=32 se 0.05 ≤ max_pair_delta_e < 0.3 (moderate).
/// - N=64 se max_pair_delta_e ≥ 0.3 (high contrast; cap N_max).
fn adaptive_n_for_stops(
    stops: &[typst_core::entities::gradient::GradientStop],
    space: typst_core::entities::layout_types::ColorSpace,
) -> usize {
    if stops.len() < 2 {
        return 16;
    }
    let max_delta_e = stops.windows(2)
        .map(|pair| perceptual_distance_in_space(pair[0].color, pair[1].color, space))
        .fold(0.0_f32, f32::max);
    if max_delta_e < 0.05 {
        16
    } else if max_delta_e < 0.3 {
        32
    } else {
        64
    }
}
```

### Callsites alterados

```rust
// 03_infra/src/export.rs ~line 1577 (Linear RGB-family arm):
let n = adaptive_n_for_stops(&linear.stops, linear.space);
let stops = multispace_sample_stops(linear, n);

// ~line 1617 (Radial RGB-family arm):
let n = adaptive_n_for_stops(&radial.stops, radial.space);
let stops = multispace_sample_stops_radial(radial, n);
```

**Não tocados**: CMYK arms (P270.2); Conic dispatcher (P272 Coons).

---

## §A.7 — Critério paridade tests P270.1

### Tests P270.1 que exercem stream byte count

**Tests byte snapshot** (verificam `pdf1 == pdf2` — determinismo, NÃO byte-equality-to-baseline):

- `p270_1_pdf_bytes_oklab_default_reproduziveis` (linha 4753) — input: red→blue Linear Oklab.
- `p270_1_pdf_bytes_hsl_reproduziveis` (linha 4789) — input: red→blue Linear HSL.
- `p270_1_pdf_bytes_oklch_hue_wrap_reproduziveis` (linha 4827) — input: red→blue Linear Oklch hue-wrap.

**Análise**: tests verificam `pdf1 == pdf2` (mesmo input duas vezes). Fórmula determinística produz N=64 consistentemente para red→blue (max_delta_e ≈ 1.0 > 0.3). Tests passam.

**Tests unit explicit N=16** (linhas 4284+ — `multispace_sample_stops(&l, 16)` chamada directa):
- Inalterados; passam N=16 literal — não usam `adaptive_n_for_stops`.

**Tests E2E PDF emit** (verificam presence/absence de strings PDF):
- `p270_1_export_pdf_cluster_3_variants_multispace_coexistem` etc — verificam `/ShadingType 2/3/6` presence. Inalterados — adaptive N só muda byte count, não strings PDF.

**Conclusão**: **Zero regressão tests P270.1** com fórmula 1B. Paridade observable preserved para pastel (N=16); behavioral change para high-contrast (N=32/64) é o objectivo do passo e é interno ao stream (não-observable via test assertions actuais).

### Tests byte snapshot que poderiam falhar (não-existentes)

Não existem tests em cristalino que comparam bytes PDF contra valor literal byte sequence em P270.1. Tests usam `pdf1 == pdf2` patterns ou string `contains` checks.

---

## §A.8 — Critério aceitação Fase A

- ✓ §A.4 cita 4 fontes canónicas literais.
- ✓ §A.5 produz matriz 3×4 completa com valores numéricos calculados (clamp/lerp/threshold).
- ✓ §A.3 Decisões 1/2/3 fixadas com fundamento numérico ancorado em §A.5:
  - **Decisão 1**: Opção 1B threshold-based.
  - **Decisão 2**: Opção 2A (Linear+Radial apenas).
  - **Decisão 3**: N_max=64; ΔE Oklab nativo.
- ✓ §A.7 confirma N=16 preservado para tests P270.1 que dependem (none — todos reproducibility ou explicit N=16 args).
- ✓ §A.6 assinatura helper genérico fixada com param `space` (futuro-proofing per ADR-0094 Pattern 2).

**Fase A completa**. Pronta para P274.B (anotação cumulativa ADR-0091) e P274.C (materialização L3).

---

## §A.9 — Sub-padrões aplicados P274.A

- **"Fase A com industry research proactiva"** N=5 → **N=6 cumulativo** (4 fontes consolidadas pré-sessão).
- **"Diagnóstico imutável"** N=19 → **N=20 cumulativo** (décimo quinto consumo directo de fonte).
- **"Auto-aplicação ADR-0065 inline"** N=18 → **N=19 cumulativo** (Fase A inline em diagnóstico).
- **"Aplicação meta-ADR (ADR-0094)"** N=2 → **N=3 cumulativo** (Pattern 3 industry research + Pattern 1 cap LOC aplicados).

---

*Diagnóstico imutável produzido em 2026-05-17 P274.A. Linhagem
empírica preservada como evidência ADR-0085 + auto-aplicação
ADR-0065. Décimo quinto consumo directo de fonte (cristalino +
vanilla + Oklab paper + W3C CSS Color 4 + Skia + Cairo industry
consolidação).*
