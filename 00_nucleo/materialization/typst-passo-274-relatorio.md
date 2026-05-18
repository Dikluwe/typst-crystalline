# Relatório P274 — P-Gradient-Adaptive-Multispace (adaptive N multispace refino qualitativo)

**Data**: 2026-05-17.
**Magnitude**: M (L3 ~80 LOC + 14 tests; cap hard 250 com folga 67%; cap soft testes 12 estourou 17%).
**Cluster**: Visualize / Gradient (continuação pós feature-complete user-facing P273).
**Tipo**: refino qualitativo aditivo (pipeline RGB-family + perceptual Linear+Radial).
**Spec**: `00_nucleo/materialization/typst-passo-274.md`.

---

## §1 — Sumário executivo

**Adaptive N multispace** materializado em P274 — pré-amostragem
N=16 fixo (P270.1) substituída por `adaptive_n_for_stops` baseado em
ΔE Oklab nativo:

- **N=16** para low contrast (Δ<0.05; light-gray near-identical) —
  preserva paridade P270.1 emit literal.
- **N=32** para moderate contrast (0.05 ≤ Δ < 0.3) — pastels saturados
  ganham mais samples Function Type 3 stitching.
- **N=64** para high contrast (Δ ≥ 0.3) — cap N_max; red→blue
  black/white extremos.

### Marcos arquitecturais P274

**(1) Refino qualitativo multispace Linear+Radial** sem alterar
estratégia (`/ShadingType 2/3` + Function Type 3 stitching preserved).

**(2) Sub-padrão "Aplicação meta-ADR (ADR-0094)" N=3 cumulativo** —
terceira aplicação prática Cap LOC + Industry research pós-formalização
P271.

**(3) Sub-padrão "Aplicação meta-ADR (ADR-0093)" N=2 cumulativo** —
segunda aplicação prática Pattern 2 anotação cumulativa em vez de
ADR nova.

**(4) Desacoplamento `perceptual_distance_in_space`** corrige sintoma
P268.2 removed P272 por construção — helper recebe `ColorSpace` param
futuro-proofing per ADR-0094 Pattern 2 (mesmo que implementation
actual ignora, permite refino futuro com métrica nativa per space).

### Decisões fixadas Fase A (§A.3)

1. **Fórmula adaptive N**: Opção 1B threshold-based (16/32/64
   discreto; fundamento numérico ancorado §A.5 satisfaz todos 3
   critérios).
2. **Variants afectados**: Opção 2A — Linear+Radial RGB-family +
   perceptual apenas. Conic preserved P272 (Coons N=stops*4 já é
   função do número de stops). CMYK preserved P270.2 (sem
   pré-amostragem em primeiro lugar).
3. **Cap N_max + ΔE units**: N_max=64 (4× N_base); ΔE Oklab nativo
   (thresholds 0.05/0.3 em unidades Oklab per Björn Ottosson + W3C
   CSS Color 4).

### Industry research pré-sessão (4 fontes canónicas)

- W3C CSS Color 4 §11 — ΔE Oklab definition + threshold perceptibilidade.
- Björn Ottosson Oklab paper (2020) §"Comparing colors" — Euclidean
  distance em coordenadas Oklab nativas.
- Skia gradient stop densification — referência operacional.
- Cairo pattern_set_extend + mesh gradient docs — referência operacional.

Sub-padrão "Fase A com industry research proactiva" N=5 → **N=6
cumulativo**.

---

## §2 — Diff L3 antes/depois

### §2.1 — Novos helpers L3 (~80 LOC)

```rust
// 03_infra/src/export.rs P274 (após resolve_relative/apply_parent_transform)

/// Distância perceptual entre duas cores num space dado.
/// Métrica: ΔE Oklab universal (Björn Ottosson + W3C CSS Color 4).
/// Param `_space` futuro-proofing (ADR-0094 Pattern 2).
fn perceptual_distance_in_space(
    c0: Color,
    c1: Color,
    _space: ColorSpace,
) -> f32 {
    let (l0, a0, b0, _) = color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, _) = color_to_oklab_with_alpha(c1);
    let dl = l1 - l0;
    let da = a1 - a0;
    let db = b1 - b0;
    (dl * dl + da * da + db * db).sqrt()
}

/// Computa N adaptive para pré-amostragem Linear/Radial.
/// Fórmula Opção 1B threshold (§A.3 Decisão 1):
fn adaptive_n_for_stops(
    stops: &[GradientStop],
    space: ColorSpace,
) -> usize {
    if stops.len() < 2 { return 16; }  // degenerado
    let max_delta_e = stops.windows(2)
        .map(|p| perceptual_distance_in_space(p[0].color, p[1].color, space))
        .fold(0.0_f32, f32::max);
    match max_delta_e {
        x if x < 0.05 => 16,   // low contrast (paridade P270.1)
        x if x < 0.3  => 32,   // moderate
        _             => 64,   // high contrast (cap N_max)
    }
}
```

### §2.2 — Dispatcher Linear/Radial RGB-family arms actualizados

```rust
// 03_infra/src/export.rs:1641 (Linear RGB-family arm):
- let stops = multispace_sample_stops(linear, 16);
+ let n = adaptive_n_for_stops(&linear.stops, linear.space);
+ let stops = multispace_sample_stops(linear, n);

// 03_infra/src/export.rs:1684 (Radial RGB-family arm):
- let stops = multispace_sample_stops_radial(radial, 16);
+ let n = adaptive_n_for_stops(&radial.stops, radial.space);
+ let stops = multispace_sample_stops_radial(radial, n);
```

**Não tocados**:
- CMYK arms (P270.2) — sem pré-amostragem em primeiro lugar.
- Conic dispatcher (P272 Coons N=stops*4) — preserved literal.
- Helpers Oklab L1 — `color_to_oklab_with_alpha` reutilizado literal
  (4ª vez no cluster Gradient).

---

## §3 — Matriz §A.5 revisada (verificada via tests)

| Caso | Stops | ΔE Oklab (real) | N escolhido (1B) | Comportamento |
|---|---|---|---|---|
| α (light-gray near-identical) | rgb(250,250,250), rgb(245,245,245) | ≈ 0.017 | **16** | Paridade P270.1 emit literal preservada |
| α' (pastel saturado) | rgb(255,200,200), rgb(200,255,200) | ≈ 0.155 | **32** | Moderate — refino N samples Function Type 3 |
| β (3 moderados) | red, white, blue | ≈ 0.65 (max pair) | **64** | High contrast — cap N_max |
| γ (5 alta sat) | red, yellow, green, blue, magenta | ≈ 0.7 | **64** | Cap N_max |
| δ (8 black-white extremo) | alternating black/white | ≈ 1.0 | **64** | Cap N_max |

**Nota empírica**: matriz §A.5 original subestimava pastels saturados
(canais a/b cromáticos contribuem ~0.15 Oklab, não 0.04). Diagnóstico
actualizado §A.5 reflete medições empíricas via tests.

---

## §4 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P274 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=12 → N=13 cumulativo consolidação clara persistente** | + P274 ADR-0091 anotada |
| Reutilização literal helpers cross-passos | **N=12 → N=13 cumulativo consolidação clara persistente** | `color_to_oklab_with_alpha` 4ª vez no cluster Gradient |
| Cap LOC hard vs soft explícito | **N=6 → N=7 cumulativo consolidação total** | L3 hard 250 / real ~80; tests soft 12 estourou ~17% |
| Fase A com industry research proactiva | **N=5 → N=6 cumulativo** | 4 fontes W3C/Ottosson/Skia/Cairo consolidadas |
| **Aplicação meta-ADR (ADR-0094)** | **N=2 → N=3 cumulativo** | Pattern 3 industry research + Pattern 1 cap LOC; terceira aplicação prática |
| **Aplicação meta-ADR (ADR-0093)** | **N=1 → N=2 cumulativo** | Pattern 2 anotação cumulativa em vez de ADR nova; segunda aplicação prática |
| Diagnóstico imutável (décimo quinto consumo) | **N=19 → N=20 cumulativo** | + P274 |
| Auditoria condicional (ADR-0084) | **N=18 → N=19 cumulativo** | + P274 |
| Auto-aplicação ADR-0065 inline | **N=18 → N=19 cumulativo** | + P274 |

---

## §5 — Métricas finais

| Métrica | Pré-P274 | Pós-P274 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2583 | **2597** | +14 |
| Tests P274 novos | — | 14 | 10 unit + 4 E2E |
| Tests P262-P273 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`5801d27c`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR) |
| LOC L3 (additions) | — | ~80 | cap hard 250 (folga 67%); cap soft 180 (folga 55%) |
| LOC L1+stdlib (additions) | — | 0 | sem touch L1/stdlib |

### §política condições verificadas

- ✓ Cap LOC L3 hard 250 — real ~80; folga 67%.
- ✓ Cap LOC L3 soft 180 — real ~80; folga 55%.
- ✓ Cap testes hard 18 — real 14; folga 22%.
- ⚠ Cap testes soft 12 — real 14; estouro ~17% registado per ADR-0094
  Pattern 1.
- ✓ Defaults adaptive deterministic preservam reproducibility tests.
- ✓ ADR-0029 pureza física L1 preserved (helpers L3 only;
  `color_to_oklab_with_alpha` é função pura).
- ✓ Lint zero; hashes L0 propagados.
- ✓ Regressão tests P262-P273 zero (2583 baseline preserved).

**8 condições §política verificadas — 7 satisfeitas absolutas + 1
estouro soft registado** per Pattern 1.

---

## §6 — Verificação regressão zero P262-P273

**2583 baseline preservado bit-exact**:

- typst-core: 2169 preserved.
- typst-shell: 24 preserved.
- typst-infra: 367 → 381 (+14 P274 tests).
- typst-wiring + bins: 23 preserved.

**Total: 2583 → 2597 (+14 net)**.

Mecânica: adaptive N deterministic — mesmo input → mesmo N → mesmo
PDF bytes. Tests reproducibility (`pdf1 == pdf2`) passam para
qualquer formula deterministic. Tests unit com N=16 explicit args
inalterados (não usam `adaptive_n_for_stops`).

§política condição "Regressão tests P262-P273 zero" satisfeita
absoluta.

---

## §7 — Anotação cumulativa ADR-0091 (centro de aplicação)

Adicionada §"Anotação cumulativa P274 — Adaptive N multispace refino
qualitativo" cobrindo:

- Fórmula adaptive N escolhida (Opção 1B threshold).
- Variants afectados (2A: Linear+Radial RGB-family + perceptual).
- N_max + ΔE units (64; Oklab nativo).
- Helper genérico `perceptual_distance_in_space` cross-space.
- Distinção vs precedente P268.2 removido P272.
- Regressão tests P270.1 originais zero.
- 7 sub-padrões aplicados.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=12 →
**N=13 cumulativo consolidação clara persistente** — sexta anotação
consecutiva da ADR-0091 (P270/P270.1/P270.2/P270.3/P273/**P274**).

---

## §8 — L0 `entities/gradient.md` anotação P274

Adicionada anotação P274 após P273 — adaptive N multispace refino
qualitativo Linear+Radial; helper `perceptual_distance_in_space`
genérico cross-space + `adaptive_n_for_stops` threshold-based. Hash
propagado via `crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:5801d27c`.

---

## §9 — Pendências preservadas pós-P274

Inalteradas vs P273:

- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC profiles).
- **P-Gradient-Relative-Callsite** (S; supply parent_bbox real;
  activa apply_parent_transform; pendência P273 §7).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**Pendências futuras específicas P274**:
- Métrica nativa per space (param `_space` aproveitado real) — se
  HSL/Oklch mostrar discrepância empírica significativa, alternativa
  à métrica Oklab universal.
- Geometria-aware adaptive — gradients muito largos com poucos stops
  podem mostrar banding fora do que adaptive N detecta. Refino fica
  fora de escopo P274.

**Decisão humana fica em aberto literal** pós-P274 — cluster Gradient
feature-complete user-facing + refino qualitativo multispace adaptive
N materializado; abre próximo cluster/refino.

---

## §10 — Limitações conscientes P274

Per spec §5:

- Métrica perceptual usa Oklab como referência universal independentemente
  do space declarado. Decisão por simplicidade + paridade W3C CSS Color 4.
- Adaptive N só aplicado em Linear+Radial RGB-family + perceptual.
  CMYK preserved P270.2; Conic preserved P272.
- Cap N_max=64 introduz tecto observable — gradients com contraste
  extremamente alto e ≥N_max stops continuam a poder mostrar banding
  residual. Aceitável.
- Fórmula adaptive é função apenas dos stops + space. Não depende
  da geometria — gradients muito largos com poucos stops podem
  continuar a mostrar banding. Refino geometria-aware fora de escopo.

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P274 centro de aplicação; 6ª anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 anotação
  cumulativa em vez de ADR nova; **2ª aplicação prática P274**).
- ADR-0094 — Meta-operacional specs (Cap LOC hard/soft Pattern 1 +
  Industry research Pattern 3; **3ª aplicação prática P274**).
- ADR-0085 — Diagnóstico imutável (décimo quinto consumo).
- ADR-0029 — Pureza física L1 (verificação preserved — `color_to_oklab_with_alpha`
  função pura).
- `00_nucleo/diagnosticos/typst-passo-274A-diagnostico.md` — Fase A
  empírica + matriz §A.5 + decisões fixadas.
- W3C CSS Color 4 §11 — ΔE Oklab definition.
- Björn Ottosson Oklab paper (2020) §"Comparing colors".
- Skia gradient stop densification.
- Cairo pattern_set_extend + mesh gradient docs.
- Spec P274 — `00_nucleo/materialization/typst-passo-274.md`.

---

*Relatório imutável produzido em 2026-05-17. Linhagem completa
preservada — adaptive N multispace refino qualitativo materializado;
helper genérico cross-space desacoplado por construção corrige
sintoma P268.2 removed P272.*
