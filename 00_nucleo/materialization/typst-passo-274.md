# Passo P274 — P-Gradient-Adaptive-Multispace

**Tipo**: refino qualitativo de pipeline gradient existente (RGB-family + perceptual).
**Magnitude estimada**: M (Fase A diagnóstico + L1/stdlib touch zero + L3 helper + alterações emit + tests).
**Pré-requisitos**: P273 fechado (3/3 campos cross-variant: `focal_*`, `space`, `relative`).
**Cluster**: Gradient (continuação pós feature-complete user-facing P273).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; quinta anotação cumulativa), ADR-0093 (Pattern 2 anotação cumulativa), ADR-0094 (Pattern 3 industry research proactiva — JÁ feita pré-sessão).

---

## §0 — Contexto

Cluster Gradient atingiu paridade vanilla user-facing em P273. 8 spaces × 3 variants × cross-variant fields (`focal_*` / `space` / `relative`). Pipeline L1+stdlib+L3 materializado.

Em casos extremos — muitos stops cromáticos ou contraste perceptual alto adjacente — o N=16 fixo da pré-amostragem `multispace_sample_stops_*` (Linear+Radial) produz **banding observable** em 7 spaces RGB-family + perceptual (Oklab/Oklch/sRGB/Luma/LinearRGB/HSL/HSV). CMYK preserved P270.2 directo (sem pré-amostragem, ColorSpace `/DeviceCMYK` nativo).

Conic preserved P272 (Coons Patches Type 6; N=stops*4 RGB + N=stops CMYK — densidade já é função do número de stops por construção).

**Não há regressão observable em casos comuns** (N=16 fixo cobre 2-3 stops com contraste moderado adequadamente). O passo é refino qualitativo aditivo.

### Pesquisa industry pré-sessão (consolidada P273 §"Decisões em aberto")

Achados que sustentam o passo, **sem prescrever fórmula final**:

- Adaptive stop count é solução industry-aligned para banding em gradients amostrados.
- ΔE Oklab é métrica perceptual standard (CIE / W3C CSS Color 4); threshold de perceptibilidade ΔE ≈ 1-2.
- Implementações de referência (Skia, Cairo) usam variantes de stop densification em vez de tessellation arbitrária para gradient quality.

### Precedente removido P272

Helper `oklab_delta_e(c0, c1)` + `compute_adaptive_n_conic(conic)` materializados em P268.2 para Type 4 Gouraud Conic. **Ambos removidos em P272** quando estratégia Conic convergiu para Coons Patches unified. ~310 LOC removed total.

**Decisão arquitectural P274**: NÃO ressuscitar literal. O helper P268.2 acoplava o cálculo de distância perceptual ao espaço Oklab específico — sintoma de acoplamento excessivo (a desaparição em P272 confirma-o empiricamente). P274 produz helper genérico cross-space desacoplado por construção.

---

## §1 — Sub-passo P274.A — Fase A diagnóstico

**Magnitude**: S documental (~30-40 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-274A-diagnostico.md`.
**ADR-0094 Pattern 3**: industry research proactiva — já consolidada pré-sessão; secção §A.4 cita fontes canónicas.

### §A.1 — Inventário do estado pós-P273

Listar literal:

- L1 `entities/gradient.rs` — struct fields Linear/Radial/Conic; `space: ColorSpace`, `relative: Option<RelativeTo>` cross-variant; `focal_*` Radial only.
- Helpers L3 actuais em `03_infra/src/export.rs` para pré-amostragem:
  - `multispace_sample_stops_linear` (P270.1; N=16 fixo; 7 spaces RGB-family + perceptual).
  - `multispace_sample_stops_radial` (P270.1; idem).
  - CMYK directo P270.2 (Linear+Radial; sem pré-amostragem).
- Helpers Oklab disponíveis P262 — `color_to_oklab_with_alpha`, `interpolate_oklab` (reusados literal em vários passos; sub-padrão "Reutilização literal helpers cross-passos" cumulativo N=12).
- Helpers genéricos cross-space — listar quais existem em P270.1 que possam ser reutilizados para conversão `Color → (componentes no space)`.

### §A.2 — Análise de risco

Tabela mínima por linha:

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P270.1 originais (N=16 emit literal) | Fórmula adaptive devolver N≠16 para casos comuns | §A.7 critério: N=16 preservado para 2-3 stops contraste moderado |
| Stream PDF explode (N=128 ou superior) | Cap insuficiente | §A.5 cap explícito + tabela exemplos |
| Acoplamento residual ao espaço Oklab | Helper P268.2 estilo (sintoma da remoção P272) | §A.6 helper genérico `perceptual_distance_in_space(c0, c1, space)` |
| Quebra ADR-0091 (centro de aplicação ColorSpace) | Lógica adaptive escolher path errado por space | Adaptive **dentro** do path de cada space (não cross-space) |

### §A.3 — Decisões a fixar na Fase A

Três decisões deixadas explicitamente para a Fase A (não fixadas pré-sessão):

**Decisão 1 — Fórmula adaptive N.** Opções inventariadas:

- Opção 1A: P268.2 hybrid literal — `N_base=16`, `N_stops = max(0, (num_stops - 2) * 8)`, `N_delta = (sum_delta_e * factor_delta) as usize`, `N = clamp(N_base.max(N_stops + N_delta), 8, 128)`, `factor_delta=256.0` per ADR-0091 anotação P268.2 consultada.
- Opção 1B: Threshold-based discreto — `N=16` se `max_pair_delta_e < 0.05`; `N=32` se `0.05..0.2`; `N=64` se `≥ 0.2`.
- Opção 1C: Continuous interpolation — `N = lerp(N_min=16, N_max=64, t = clamp(max_pair_delta_e / 0.2, 0, 1))`.

Critério de decisão: minimizar acoplamento + garantir paridade tests P270.1 originais + cap N que mantenha stream PDF tolerável (referenciar exemplos numéricos §A.5).

**Decisão 2 — Variants afectados.** Opções inventariadas:

- Opção 2A: Linear+Radial apenas (Conic preserved P272 literal; CMYK preserved P270.2 literal).
- Opção 2B: Linear+Radial+Conic (Conic ganha multiplier adaptive sobre N=stops*4 baseline).

Critério de decisão: 2B só justificado se houver evidência empírica de banding em Conic com baseline P272 — pesquisa pré-sessão não evidenciou. Default sugerido: 2A (preserva escopo cluster encerrado P273 e respeita "se não está partido não conserte"). Decisão final na Fase A.

**Decisão 3 — Cap N máximo + threshold ΔE units.** Opções inventariadas:

- N_max ∈ {32, 64, 128}.
- ΔE units: Oklab nativo (sem multiplier) vs normalizado `* 256.0` (P268.2 literal).

Critério: §A.5 tabela com 3-4 casos concretos (2 stops pastel; 3 stops moderados; 5 stops contraste alto; 8 stops contraste extremo) prevendo N adaptive por cada cap+units combination; escolher combinação que dá:

1. N=16 para 2 stops pastel (paridade P270.1 emit literal).
2. N≤64 para casos típicos (stream PDF não explode).
3. Diferenciação observable entre "moderado" e "extremo" (passo serve para algo).

### §A.4 — Fontes canónicas citadas

- W3C CSS Color 4 §11 (ΔE Oklab definition; threshold perceptibilidade).
- Björn Ottosson — Oklab paper (2020) §"Comparing colors" (ΔE = euclidean distance em coordenadas Oklab).
- Skia gradient stop densification approach (referência operacional).
- Cairo pattern_set_extend / mesh gradient docs (referência operacional Type 6 + adaptive).

### §A.5 — Tabela de casos concretos

Para cada combinação (cap N_max, ΔE units, fórmula 1A/1B/1C), calcular N esperado para:

- Caso α: 2 stops pastel — colors `(rgb(255,200,200), rgb(200,255,200))`; max_pair_delta_e ≈ 0.04 Oklab.
- Caso β: 3 stops moderados — colors `(red, white, blue)`; max_pair_delta_e ≈ 0.6 Oklab.
- Caso γ: 5 stops contraste alto — colors saturação alta espalhados em hue.
- Caso δ: 8 stops contraste extremo — black→saturado→white repetido.

Output esperado: matriz 3 fórmulas × 4 casos = 12 valores N. Escolher fórmula que satisfaz §A.3 Decisão 3 critério.

### §A.6 — Helper proposto

```rust
/// Distância perceptual entre duas cores num space dado.
/// Métrica: ΔE Oklab (independente do space de entrada;
/// converte cada cor para Oklab e calcula distância euclidiana).
///
/// Genérico cross-space — recebe `ColorSpace` como parâmetro
/// em vez de hardcode Oklab. Permite reuso por construção em
/// Linear + Radial (+ Conic se Decisão 2 = 2B).
fn perceptual_distance_in_space(c0: &Color, c1: &Color, space: ColorSpace) -> f32 {
    // Converter ambas para Oklab via helpers P262 reusados literal.
    let (l0, a0, b0, _) = color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, _) = color_to_oklab_with_alpha(c1);
    let dl = l1 - l0;
    let da = a1 - a0;
    let db = b1 - b0;
    (dl * dl + da * da + db * db).sqrt()
    // Nota: `space` param actualmente não altera a métrica — Oklab
    // é referência universal per W3C CSS Color 4. Param preservado
    // por design (futuro: spaces com métrica nativa diferente
    // poderiam usar `space` para escolher fórmula; mantém assinatura
    // estável e sinaliza intent cross-space na call site).
}
```

**Justificação do parâmetro `space` aparentemente não-usado**: ADR-0094 Pattern 2 — antecipa reutilização sem custo. Quando call site é `perceptual_distance_in_space(c0, c1, gradient.space)` a leitura no consumer é auto-documentada e o helper não precisa de renomear no futuro. Se Fase A decidir que a antecipação não vale o param, alternativa é assinatura mínima `perceptual_distance(c0, c1) -> f32`.

**Wrapper de mais alto nível** — `compute_adaptive_n_linear` / `compute_adaptive_n_radial` (Decisão 2A) ou inline no `multispace_sample_stops_*` (refactor mínimo) — decidir na Fase A consoante a fórmula escolhida em Decisão 1.

### §A.7 — Critério de paridade tests P270.1

Listar literal os tests P270.1 que exercem N=16 emit count (stream byte size dependente de N) e confirmar que a fórmula escolhida em §A.3 devolve N=16 para os inputs desses tests.

### §A.8 — Critério de aceitação Fase A

- §A.4 cita as 4 fontes canónicas literais.
- §A.5 produz a matriz 3×4 completa com valores numéricos calculados.
- §A.3 Decisões 1/2/3 fixadas com fundamento numérico ancorado em §A.5.
- §A.7 confirma N=16 preservado para inputs de tests P270.1 originais.
- §A.6 assinatura helper genérico fixada (com ou sem param `space` consoante decisão).

---

## §2 — Sub-passo P274.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 cumulativamente (sexta anotação consecutiva — sub-padrão ADR-0093 Pattern 2 cumulativo N=12 → 13).

Template de anotação a preencher pós-Fase A:

```
## Anotação cumulativa P274 — Adaptive N multispace refino qualitativo

**Data**: 2026-05-XX.
**Motivo**: P270.1 estabeleceu pré-amostragem N=16 fixo para 7 spaces
RGB-family + perceptual em Linear+Radial. Casos extremos (muitos stops
ou contraste cromático alto adjacente) podem apresentar banding visível.
P274 refina N adaptive por gradient sem alterar estratégia
(/ShadingType 2/3 + Function Type 3 stitching).

**Fórmula adaptive N escolhida** (literal §A.5 diagnóstico):
- [preencher fórmula 1A / 1B / 1C escolhida].
- [preencher cap N_max escolhido].
- [preencher units ΔE escolhidas].

**Variants afectados**: [preencher 2A ou 2B].
- Linear+Radial RGB-family + perceptual: adaptive N por gradient.
- CMYK preserved P270.2 directo (sem pré-amostragem; inalterado).
- Conic [preserved P272 literal / OU adaptive multiplier sobre stops*4 baseline].

**Helper genérico `perceptual_distance_in_space`** introduzido em
`03_infra/src/export.rs` (privado). Por construção desacoplado do
space; aceita `ColorSpace` como parâmetro. Sub-padrão "Reutilização
literal helpers cross-passos" N=12 → 13 (helpers P262 Oklab
reusados literal pela 4ª vez no cluster Gradient).

**Distinção vs precedente P268.2 removido P272**: helper P268.2
`oklab_delta_e` + `compute_adaptive_n_conic` acoplavam métrica ao
Oklab específico e fórmula ao Type 4 Gouraud. P274 corrige por
construção — helper recebe `ColorSpace`, fórmula desacoplada de
estratégia.

**Regressão tests P270.1 originais proibida** — fórmula escolhida
preserva N=16 para inputs de tests P270.1 (§A.7 confirma).

**Helpers Oklab P262 reutilizados literal** (`color_to_oklab_with_alpha`)
— sub-padrão cumulativo cluster Gradient.

**ADR-0091 preservada literal** — centro de aplicação ColorSpace
intocado; P274 só refina N por path RGB-family + perceptual.
```

---

## §3 — Sub-passo P274.C — Materialização L3 (testes primeiro)

**Magnitude**: M (~150-250 LOC consoante Decisão 2).

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 anotação §2 escrita pós-fixação de Decisões 1/2/3.
3. `crystalline-lint --fix-hashes` (refactor preserva hashes L0 — gradient.md, export.md inalterados estruturalmente).
4. **Testes-primeiro** — adicionar tests antes de qualquer LOC L3 produtivo.
5. L3 código — helper novo `perceptual_distance_in_space` + alterações `multispace_sample_stops_linear` / `_radial` (+ Conic se 2B).
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1)

- **L3 hard cap**: ≤ 250 LOC em `03_infra/src/export.rs` (helper novo + alterações dos dois ou três helpers existentes consoante Decisão 2).
- **L3 soft cap**: ≤ 180 LOC (alvo; sinaliza se atingir).
- **Tests hard cap**: ≤ 18 novos.
- **Tests soft cap**: ≤ 12.

### Tests propostos (lista mínima — completar pós-Fase A)

Cobertura literal:

1. `p274_perceptual_distance_oklab_zero_for_identical_colors` — `perceptual_distance_in_space(c, c, space) == 0.0` para qualquer space.
2. `p274_perceptual_distance_symmetric` — `distance(a, b) == distance(b, a)`.
3. `p274_perceptual_distance_extreme_high` — black vs white em Oklab ≈ 1.0; sanity check.
4. `p274_linear_adaptive_n_low_contrast_preserves_n16` — Linear gradient 2 stops pastel emite stream com N=16 (paridade P270.1).
5. `p274_linear_adaptive_n_high_contrast_increases_n` — Linear gradient com contraste forte emite stream com N > 16 e ≤ N_max.
6. `p274_linear_adaptive_n_caps_at_n_max` — Linear gradient com 8 stops contraste extremo emite stream com N == N_max.
7. `p274_radial_adaptive_n_mirrors_linear` — paridade comportamental Radial (mesma fórmula; focal_* preserved).
8. `p274_radial_adaptive_n_focal_preserved` — Radial com focal_center custom + adaptive N alto preserva semântica focal (regressão P269).
9. `p274_cmyk_unchanged` — Linear+Radial CMYK preserved P270.2 (sem adaptive; output byte-identical P270.2).
10. `p274_conic_unchanged_if_2A` (Decisão 2A) — Conic stream byte-identical P272. *(Substituir por `p274_conic_adaptive_multiplier` se 2B.)*
11. `p274_relative_preserved_with_adaptive` — `relative: Some(...)` Linear+Radial + adaptive N alto preserva semântica P273.
12. `p274_space_dispatch_unchanged` — todos 7 spaces RGB-family + perceptual continuam a despachar para o path correcto (regressão ADR-0091).
13. (Tests adicionais para a fórmula específica escolhida — preencher pós-Fase A.)

### Alterações esperadas no código L3

```rust
// helper novo (privado)
fn perceptual_distance_in_space(c0: &Color, c1: &Color, space: ColorSpace) -> f32 { ... }

// helper wrapper para N adaptive (decidir nome + scope na Fase A)
fn adaptive_n_for_stops(stops: &[GradientStop], space: ColorSpace) -> usize { ... }

// alteração emit Linear
fn multispace_sample_stops_linear(gradient: &Linear) -> Vec<...> {
    let n = adaptive_n_for_stops(&gradient.stops, gradient.space);  // era: const N: usize = 16;
    // resto do corpo P270.1 inalterado
}

// alteração emit Radial análoga
```

### Verificação final

- Cap LOC respeitado (hard + soft).
- `cargo test --workspace` verde — workspace ~2583 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 preservados (refactor L3-only; L1 intocado).
- Tests P270.1 originais inalterados.
- DEBT saldo preserved 10.

---

## §4 — Sub-padrões cumulativos pós-P274

Listar literal o que P274 contribui:

| Sub-padrão | N pré-P274 | N pós-P274 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 12 | 13 |
| Reutilização literal helpers cross-passos | 12 | 13 |
| Diagnóstico imutável (consumo directo fonte) | 19 (14º consumo) | 20 (15º consumo) |
| Auto-aplicação ADR-0065 inline | 18 | 19 |
| Auditoria condicional | 18 | 19 |
| Anotação cumulativa cross-ADR | 7 | 7 (não cross-ADR) |
| Cap LOC hard vs soft explícito | 6 | 7 |
| Fase A com industry research proactiva | 5 | 6 |
| Aplicação meta-ADR (ADR-0093) | 1 | 2 |
| Aplicação meta-ADR (ADR-0094) | 2 | 3 |

---

## §5 — Limitações conscientes P274

- Métrica perceptual usa Oklab como referência universal independentemente do space declarado. Decisão por simplicidade + paridade W3C CSS Color 4; futuros refinos podem usar métricas nativas se algum space mostrar discrepância empírica significativa.
- Adaptive N só é aplicado em Linear+Radial RGB-family + perceptual. CMYK preserved P270.2 directo (sem pré-amostragem em primeiro lugar; adaptive seria fora de escopo arquitectural). Conic preserved P272 ou ganha multiplier consoante Decisão 2.
- Cap N_max introduz tecto observável — gradients com contraste extremamente alto e ≥ N_max stops continuam a poder mostrar banding residual. Aceitável; alternativa (sem cap) explode stream PDF.
- Fórmula adaptive é função apenas dos stops + space. Não depende da geometria (largura/altura do gradient) — gradients muito largos com poucos stops podem continuar a mostrar banding fora do que o adaptive N detecta. Refino geometria-aware fica fora de escopo P274.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code (separadamente). Output: `00_nucleo/diagnosticos/typst-passo-274A-diagnostico.md` com Decisões 1/2/3 fixadas + matriz §A.5 calculada.
3. Utilizador upload do diagnóstico.
4. Claude web revê diagnóstico + valida critério §A.8.
5. Utilizador executa P274.B + P274.C em Claude Code.
6. Utilizador upload do relatório `typst-passo-274-relatorio.md`.
7. Claude web analisa relatório, propõe próximo passo do cluster Gradient ou outro cluster.

---

## §7 — Pendências preservadas (não-bloqueantes)

Inalteradas vs P273:

- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC profiles; PDF/A compliance).
- **P-Gradient-Relative-Callsite** (S; supply parent_bbox real; activa apply_parent_transform; pendência P273 §7 `#[allow(dead_code)]`).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266 Opção 1).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient** (Text/Model/Layout/etc.).

---

## §8 — Critério de fecho do passo

P274 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada com decisões finais.
- L3 alterado dentro do cap LOC.
- Tests workspace verdes (cap tests respeitado).
- Lint zero.
- Hashes L0 preservados.
- Tests P270.1 originais inalterados.
- DEBT saldo 10 preserved.
- Sub-padrões §4 atualizados em ADRs cumulativos consolidados.
