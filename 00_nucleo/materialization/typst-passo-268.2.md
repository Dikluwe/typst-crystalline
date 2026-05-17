# typst-passo-268.2 — Refino Conic PDF adaptive N hybrid 1+2 (Type 4 Gouraud qualidade visual)

**Magnitude**: S (cap: ≤ 200 LOC L3 + ≤ 15 testes novos).
**Cluster**: Visualize / Gradient / PDF export (refino qualidade visual).
**Tipo**: refino numerado .2 dentro da série P268. Pattern análogo P268.1 (passo .N dentro da série).
**Origem**: §8 relatório P268.1 pendência reservada; ADR-0090 §"Refino qualidade visual" pendente.
**Sequência**: P268 (Type 4 Gouraud N=32 fixo) → P268.1 (ADR-0090 EM VIGOR formalizando Type 4) → **P268.2 (adaptive N hybrid 1+2)**.
**Estratégia decidida**: utilizador escolheu Critério 4 — hybrid 1+2 (over-engineering deliberado em S; cap acomoda).

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 (anotado P268+P268.1) + ADR (ADR-0090 EM VIGOR estabelece estratégia Type 4). Ordem: diagnóstico empírico factor_delta → anotação ADR-0089 P268.2 → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-adaptive-n-passo-268-2.md` imutável. **Sexto consumo directo de fonte** (P262/P264/P267/P268 vanilla + P268.1 web; P268.2 é primeiro consumo de **literatura técnica perceptual** se factor_delta for justificado por papers Oklab ΔE).

3. **ADR-0090 preservada literal** — Type 4 Gouraud strategy intocada. P268.2 só refina parâmetro N adaptive; estratégia Type 4 permanece.

4. **ADR-0089 anotação cumulativa P268.2** — não criar ADR-0091. Sub-padrão "Anotação cumulativa em vez de ADR nova" **N=5 → N=6 cumulativo**.

5. **ADR-0087/ADR-0088 preservadas** — Linear/Radial intocados.

6. **ADR-0039 preservado** — TextStyle intocado.

7. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P268.2 (qualidade visual Conic PDF refinada).

8. **ADR-0018 preservado** — implementação autónoma cristalino; sem dependências externas adicionais.

9. **Crystalline-lint zero violations** obrigatório.

10. **Reutilização literal helpers Oklab** P262 (`color_to_oklab_with_alpha`, `interpolate_oklab`) para cálculo ΔE Oklab. Sub-padrão "Reutilização literal helpers cross-passos" **N=3 → N=4 cumulativo**.

11. **Justificativa over-engineering deliberado** — Critério 4 hybrid é mais complexo que necessário para o caso médio (2-3 stops contraste moderado N=32 suficiente). Utilizador escolheu hybrid 1+2 explicitamente para cobrir casos extremos (muitos stops ou contraste alto). Cap S acomoda; justificação fica registada em ADR-0089 §anotação P268.2.

12. **Regressão tests P268 proibida** — 6 testes P268 originais devem continuar verdes (`p268_export_pdf_conic_emits_shading_type_4`, dedup, cluster 3 variants, helpers unit).

---

## §1 — Sub-passo P268.2.A — Diagnóstico empírico factor_delta

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-adaptive-n-passo-268-2.md`.

### Objectivo Fase A

Determinar empiricamente:
- `factor_delta` em `N_delta = sum(delta_e_oklab[i,i+1]) * factor_delta`.
- Confirmar `N_base=32`, `N_min=8`, `N_max=128` razoáveis.
- Confirmar `N_stops = (num_stops - 2) * 8` razoável (cada stop adicional adiciona 8 fatias).

### Comandos exactos a executar

```bash
# 1. Confirmar helpers Oklab P262 disponíveis (color_to_oklab_with_alpha + interpolate_oklab)
rg -n "color_to_oklab_with_alpha|interpolate_oklab|fn.*oklab" 03_infra/src/export.rs 01_core/src/entities/gradient.rs

# 2. Estado actual emit_conic_gouraud_stream (cap N=32 fixo)
rg -n "emit_conic_gouraud_stream|n_slices|N_SLICES|let n =|max\(8" 03_infra/src/export.rs

# 3. Vanilla typst Conic não usa adaptive N (krilla SweepGradient é Type 1; não tem fatias)
rg -n "n_slices|N_SLICES|adaptive|num_slices|conic" lab/typst-original/crates/typst-pdf/src/ | head -20

# 4. Pesquisa literatura técnica perceptual Oklab ΔE thresholds
# (Claude Code consulta documentação Oklab; thresholds standard:
#  ΔE < 1 imperceptível; ΔE 1-2 perceptível treinado; ΔE > 2 perceptível casual)
# Fonte canónica: Björn Ottosson Oklab paper / W3C CSS Color 4 spec.

# 5. Cristalino tests P268 originais (regressão obrigatória pós-refino)
rg -n "p268_export_pdf_conic|p268_emit_conic|p268_oklab_sample_stops_conic" 03_infra/src/export.rs

# 6. Cluster Gradient tests outros (Linear/Radial não devem regressionar)
cargo test -p typst-cristalino-infra gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.9)

```
§A.1 Helpers Oklab disponíveis — assinaturas + reutilização viável.
§A.2 Estado actual emit_conic_gouraud_stream — N=32 fixo + clamp min 8.
§A.3 Vanilla não tem precedente adaptive N — krilla Type 1 PostScript é
     pixel-perfect por definição; cristalino Type 4 precisa adaptive
     para qualidade visual em casos extremos.
§A.4 PESQUISA LITERATURA Oklab ΔE thresholds — decisão factor_delta:
     - ΔE < 1 imperceptível (CIE/Oklab standard).
     - ΔE 1-2 perceptível treinado.
     - ΔE > 2 perceptível casual.
     - Para gradient banding: N fatias por ΔE total determina suavidade.
§A.5 PROPOSTA factor_delta — literal:
     - factor_delta = 2.0  (cada unidade ΔE total → 2 fatias adicionais).
     - Justificação: ΔE total entre 2 stops em gradiente comum ~50-100
       (red↔blue Oklab ~70); * 2.0 = 140 fatias para contraste máximo;
       clamp N_max=128 trava.
     - ΔE total baixo (~10 stops pastel) → 20 fatias; N_stops domina; N=32 preserved.
§A.6 PROPOSTA fórmula completa:
     N_base = 32
     N_stops = max(0, (num_stops - 2) * 8)
     N_delta = (sum_delta_e * 2.0) as usize
     N = clamp(N_base.max(N_stops + N_delta), 8, 128)
§A.7 Casos teste empíricos esperados:
     - 2 stops red↔blue (ΔE ~70): N_delta = 140 → clamp 128.
     - 2 stops pastel (ΔE ~5): N_delta = 10; N_stops = 0; N = max(32, 10) = 32 preservado P268.
     - 5 stops moderados (ΔE total ~30): N_delta = 60; N_stops = 24; N = max(32, 84) = 84.
     - 1 stop (degenerado): cláusula explícita N=8 (mínimo).
§A.8 Cristalino tests P268 originais — paridade comportamental:
     - p268_export_pdf_conic_emits_shading_type_4 usa 2 stops moderados; N esperado 32 preservado.
     - p268_export_pdf_conic_dedup_arc_ptr idem; N preservado.
     - p268_export_pdf_cluster_3_variants_coexistem idem.
     - Tests stream size (576 bytes para N=32) preservados para esses casos.
§A.9 Decisão arquitectural — hybrid 1+2 com factor_delta=2.0 confirmado.
```

### Critério de aceitação Fase A

- §A.4 cita fonte canónica ΔE thresholds (W3C CSS Color 4 / Oklab paper).
- §A.5 justifica factor_delta=2.0 ou propõe alternativa fundamentada.
- §A.8 confirma que tests P268 originais não regressionam com fórmula proposta.

---

## §2 — Sub-passo P268.2.B — Anotação cumulativa ADR-0089

**Não criar ADR-0091**. Anotar ADR-0089 após anotação P268.1.

### Anotação cumulativa ADR-0089 P268.2

```
## Anotação cumulativa P268.2 — Refino adaptive N hybrid 1+2

**Data**: 2026-05-15.
**Motivo**: P268.1 ADR-0090 formalizou Type 4 Gouraud com N=32 fixo;
casos extremos (muitos stops ou contraste cromático alto) podem
apresentar banding visível. P268.2 refina N adaptive sem mudar
estratégia Type 4.

**Fórmula adaptive N hybrid 1+2** (literal §A.6 diagnóstico):
- N_base = 32 (preserva P268 caso comum).
- N_stops = max(0, (num_stops - 2) * 8) — critério 1.
- N_delta = (sum_delta_e_oklab * 2.0) as usize — critério 2.
- N = clamp(N_base.max(N_stops + N_delta), 8, 128).

**factor_delta = 2.0** empiricamente justificado em §A.5 diagnóstico:
2 fatias por unidade ΔE Oklab total — cobre threshold perceptibilidade
ΔE 1-2 (CIE/Oklab standard).

**N_max = 128** evita stream PDF explodir (2304 bytes vs 576 actual
P268; tolerável).

**ADR-0090 preservada literal** — estratégia Type 4 intocada;
P268.2 só refina parâmetro N.

**Helpers Oklab P262 reutilizados literal** (`color_to_oklab_with_alpha`
+ interpolate_oklab para cálculo ΔE entre stops adjacentes); sub-padrão
"Reutilização literal helpers cross-passos" N=3 → N=4 cumulativo.

**Regressão tests P268 originais proibida** — fórmula hybrid preserva
N=32 para casos comuns (2-3 stops contraste moderado); §A.8 confirma.

**Cluster Gradient PDF qualidade visual industry-grade** — adaptive N
elimina banding observable em casos extremos; cristalino Type 4
qualitativamente competitivo com Cairo Type 6/7 sem aumentar magnitude
implementação.

**Justificação over-engineering deliberado**: Critério 4 hybrid 1+2 é
mais complexo que crit 1 isolado (~5 LOC) ou crit 2 isolado
(~30-40 LOC). Utilizador escolheu hybrid explicitamente para cobrir
ambos casos extremos; cap S (200 LOC) acomoda; refino vale a
complexidade.
```

### Anotação ADR-0054 P268.2

```
P268.2 — refino adaptive N hybrid 1+2 materializa qualidade visual
Type 4 Gouraud sem mudar estratégia ADR-0090; cluster Gradient PDF
qualitativamente industry-grade. Perfil graded DEBT-1 preservado
(refino é optimização local, não simplificação).
```

### L0 anotação `entities/gradient.md` P268.2

```
**Anotação P268.2**: emit_conic_gouraud_stream usa N adaptive hybrid
1+2 (número de stops + contraste Oklab ΔE) em vez de N=32 fixo;
fórmula §A.6 diagnóstico-adaptive-n-passo-268-2.md. ADR-0090
preservada literal.
```

Hashes propagados via `crystalline-lint --fix-hashes`. Zero violations.

---

## §3 — Sub-passo P268.2.C — Materialização L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. L0 + ADR-0089 + ADR-0054 anotações §2.
3. `crystalline-lint --fix-hashes`.
4. **Testes-primeiro** — adicionar ~10-15 testes ANTES de qualquer LOC L3.
5. L3 código — alterar `emit_conic_gouraud_stream` + helper novo `compute_adaptive_n_conic`.
6. Verificação final.

### Cap LOC

- L3: ≤ 200 LOC em `03_infra/src/export.rs` (helper novo + alteração `emit_conic_gouraud_stream`).
- Testes: ≤ 15 novos.

### Helper novo proposto

```rust
/// Computes adaptive N for Conic Type 4 Gouraud triangulation.
/// Hybrid critério 1 (num_stops) + critério 2 (Oklab ΔE).
///
/// Formula:
///   N_base = 32
///   N_stops = max(0, (num_stops - 2) * 8)
///   N_delta = (sum_delta_e_oklab * 2.0) as usize
///   N = clamp(N_base.max(N_stops + N_delta), 8, 128)
fn compute_adaptive_n_conic(conic: &Conic) -> usize {
    const N_BASE: usize = 32;
    const N_MIN: usize = 8;
    const N_MAX: usize = 128;
    const FACTOR_DELTA: f32 = 2.0;

    let num_stops = conic.stops.len();
    if num_stops < 2 {
        return N_MIN;  // degenerado; preserva clamp P268
    }

    let n_stops = (num_stops.saturating_sub(2)) * 8;

    let sum_delta_e: f32 = conic.stops.windows(2)
        .map(|pair| oklab_delta_e(&pair[0].color, &pair[1].color))
        .sum();
    let n_delta = (sum_delta_e * FACTOR_DELTA) as usize;

    let n_adaptive = N_BASE.max(n_stops + n_delta);
    n_adaptive.clamp(N_MIN, N_MAX)
}

/// ΔE Oklab entre 2 cores; usa helpers P262 literal.
fn oklab_delta_e(c1: &Color, c2: &Color) -> f32 {
    let ok1 = color_to_oklab_with_alpha(c1);  // P262
    let ok2 = color_to_oklab_with_alpha(c2);  // P262
    let dl = ok1.l - ok2.l;
    let da = ok1.a - ok2.a;
    let db = ok1.b - ok2.b;
    (dl*dl + da*da + db*db).sqrt()
}
```

### Alteração `emit_conic_gouraud_stream`

```rust
fn emit_conic_gouraud_stream(conic: &Conic) -> Vec<u8> {
    let n = compute_adaptive_n_conic(conic);  // P268.2 — substitui N=32 fixo
    // ... resto preservado P268 literal ...
}
```

Call site `emit_gradient_objects` mantém-se igual (não passa n_slices; helper decide).

### Estrutura testes esperada

**Unit `compute_adaptive_n_conic`** (8 testes):
- `p268_2_adaptive_n_2_stops_pastel_preserva_32`: 2 stops ΔE baixo → N=32 (regressão P268 preservada).
- `p268_2_adaptive_n_2_stops_red_blue_clamp_128`: red↔blue ΔE alto → N=128 clamp.
- `p268_2_adaptive_n_5_stops_moderados`: 5 stops ΔE total moderado → N intermediário (calculado fórmula).
- `p268_2_adaptive_n_8_stops_pastel`: 8 stops contraste baixo → N_stops domina (48); N_delta baixo.
- `p268_2_adaptive_n_1_stop_degenerado_n_min`: 1 stop → N=8 (N_MIN).
- `p268_2_adaptive_n_stops_identicos_delta_zero`: 2 stops iguais ΔE=0 → N=32 (N_BASE).
- `p268_2_adaptive_n_clamp_n_max_128`: caso extremo > 128 → clamp 128.
- `p268_2_oklab_delta_e_helper_red_blue`: ΔE red↔blue ~70 confirmado.

**E2E PDF** (4 testes):
- `p268_2_export_pdf_conic_adaptive_n_red_blue_stream_size`: ~2304 bytes para N=128.
- `p268_2_export_pdf_conic_adaptive_n_pastel_preserva_576`: N=32 preservado (regressão P268).
- `p268_2_export_pdf_regression_p268_cluster_3_variants`: cluster 3 preservado.
- `p268_2_export_pdf_conic_dedup_adaptive_n_preservado`: 3 shapes mesmo Arc<Conic> → mesmo N adaptive.

**Snapshot** (3 testes):
- `p268_2_pdf_bytes_reproduziveis_pastel`: hash bytes determinístico N=32.
- `p268_2_pdf_bytes_reproduziveis_red_blue`: hash bytes determinístico N=128.
- `p268_2_pdf_bytes_reproduziveis_moderado`: hash bytes determinístico N intermediário.

Total esperado: 8 + 4 + 3 = 15 testes (cap exacto).

### Tests P268 originais (regressão obrigatória)

- `p268_export_pdf_conic_emits_shading_type_4` (2 stops moderados): N=32 preservado.
- `p268_export_pdf_conic_dedup_arc_ptr`: N=32 preservado.
- `p268_export_pdf_cluster_3_variants_coexistem`: N=32 preservado.
- `p268_emit_conic_gouraud_stream_n32_size` (576 bytes): N=32 preservado para input do teste.
- `p268_emit_conic_gouraud_stream_min_8_slices` (144 bytes): N=8 preservado para input degenerado.
- `p268_oklab_sample_stops_conic_red_blue_endpoints`: helper P268 preservado.

**Se qualquer destes 6 regressionar, §política condição 9 dispara**.

---

## §4 — Sub-passo P268.2.D — Promoção + README + relatório

1. **ADR-0089** anotação cumulativa P268.2 fechada.
2. **ADR-0054** anotação cumulativa P268.2 adicionada.
3. **ADR-0090** **preservada literal** — nenhuma anotação P268.2 (estratégia Type 4 intocada).
4. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (+~1-2pp esperados via qualidade visual).
   - Entrada P268.2 ~30-40 linhas (paridade entrada P268 mas menor; refino).
   - Cross-reference ADR-0089 §anotação P268.2.
   - Cluster Gradient agora "industry-grade qualidade visual".
5. **Distribuição ADRs preservada** — total 77 mantido (sem ADR nova).
6. **Relatório** `00_nucleo/materialization/typst-passo-268-2-relatorio.md`:
   - Métricas finais (esperado 2413 + 15 = 2428 testes verdes).
   - factor_delta=2.0 confirmado empiricamente Fase A.
   - Diff `emit_conic_gouraud_stream` antes/depois (N=32 fixo → adaptive).
   - Sub-padrões + N cumulativo.
   - Regressão zero P268 tests originais.
   - Cluster Gradient industry-grade qualidade visual marcado.

---

## §política de paragem

Claude Code para e pergunta se qualquer das seguintes condições ocorrer:

1. **Fase A §A.4 revela ΔE thresholds não-standard** — Oklab ΔE thresholds amplamente aceites (CIE/W3C) não confirmáveis em literatura. factor_delta=2.0 fica sem base empírica.

2. **Fase A §A.5 propõe factor_delta significativamente diferente** de 2.0 — caso a justificativa empírica leve a factor 0.5 ou 10.0, fórmula muda; magnitude pode estourar.

3. **Helpers Oklab P262 (`color_to_oklab_with_alpha`, `interpolate_oklab`) não reutilizáveis literal** — gap helpers > 30 LOC.

4. **Cap LOC L3 (200) ameaça ser ultrapassado** — refino hybrid devia caber; estouro indica fórmula mais complexa que prevista.

5. **Cap testes (15) ameaça ser ultrapassado** — > 15 testes indica casos extra não previstos (edge cases muito específicos).

6. **Snapshot bytes PDF não reproduzíveis** — float não-determinismo no cálculo ΔE; pode ocorrer se Oklab usa f64 internamente.

7. **Crystalline-lint reporta violations** após anotações.

8. **ADR-0090 ameaçada** — alteração proposta toca estratégia Type 4 (não só parâmetro N).

9. **Regressão tests P268 originais** — qualquer dos 6 tests P268 falha pós-refino.

10. **N adaptive não converge para casos comuns** — fórmula produz N != 32 para 2 stops pastel (caso comum); revisão fórmula.

11. **Cluster Gradient marco quebra** — `p268_export_pdf_cluster_3_variants_coexistem` falha.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N após P268.2 | Nota |
|---|---|---|
| Auditoria condicional (ADR-0084) | N=10 | + P268.2 |
| Diagnóstico imutável (ADR-0085) | N=11 (sexto consumo de fonte) | + P268.2 (P262/P264/P267/P268/P268.1/**P268.2**) |
| Anotação cumulativa em vez de ADR nova | **N=6** | + P268.2 anotada ADR-0089 |
| Reutilização literal helpers cross-passos | **N=4** | + P268.2 (helpers Oklab P262) |
| Auto-aplicação ADR-0065 inline | N=9 | + P268.2 |
| ADR-0090 preservada por refino paramétrico | **N=1 inaugural** | P268.2 inaugura padrão "refino sem revogar ADR estratégica" |

### Marco arquitectural P268.2

**Cluster Gradient PDF industry-grade qualidade visual** — adaptive N hybrid elimina banding observable em casos extremos (muitos stops ou contraste alto), mantendo estratégia Type 4 ADR-0090 intocada.

**Primeira aplicação do padrão "refino paramétrico preservando ADR estratégica"** — P268.2 melhora qualidade visual sem revogar ADR-0090; precedente para futuros refinos onde ajuste de constante/parâmetro melhora comportamento sem mudar decisão arquitectural fundamental.

### Sequência pós-P268.2

- **P-Gradient-Focal** (M; activa focal_* Radial; revoga ADR-0088 §focal scope-out).
- **ADR-0055bis variant-aware fonts** (M; refino Text).
- **P-Footnote-N** (M; Model pendência).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling** (Paint::Tiling activação).

---

## §referências cross-passos

- **P268** — PDF Conic Type 4 Gouraud N=32 fixo (precedente directo refinado).
- **P268.1** — ADR-0090 EM VIGOR formalizando Type 4 (preservada literal por este passo).
- **P267** — Gradient Conic L1+stdlib (ADR-0089).
- **P263** — PDF Linear /ShadingType 2 (helpers Oklab origem).
- **P265** — PDF Radial /ShadingType 3 (helpers Oklab N=16 reutilizados).
- **P262** — Gradient Linear L1+stdlib (helpers Oklab origem).
- **P261** — Paint wrapper (ADR-0086).
- ADR-0090 — Type 4 Gouraud strategy (preservada literal).
- ADR-0089 — Gradient Conic-only (anotada cumulativa P268.2).
- ADR-0054 — Perfil graded (anotada cumulativa P268.2).
- ADR-0018 — Whitelist crates (preservada).
- ADR-0085 — Diagnóstico imutável (sexto consumo de fonte).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.4 é central** — citar fonte canónica ΔE thresholds (W3C CSS Color 4 / Björn Ottosson Oklab paper). Se não-confirmável, §política condição 1 dispara.
- **factor_delta=2.0 é proposta** — Fase A pode justificar valor diferente baseado em §A.4; se mudar, §política condição 2 dispara para confirmar antes de continuar.
- **Helpers Oklab P262** (`color_to_oklab_with_alpha`, `interpolate_oklab`) reutilizados literal; `oklab_delta_e` é wrapper trivial. Se P262 não tem essas assinaturas exactas, gap > 30 LOC dispara §política 3.
- **Tests P268 originais devem permanecer verdes** — §política condição 9 absoluta. Verificar com `cargo test p268_` antes de finalizar.
- **Snapshot bytes determinísticos** — Oklab f32 vs f64; verificar consistência. §política 6.
- **N para 2 stops pastel = 32** preservado obrigatoriamente (regressão P268 caso comum). §política 10.
- **ADR-0090 não tocada** — qualquer alteração à ADR estratégica fora do âmbito P268.2 dispara §política 8.
- **Relatório final esperado**: 2413 + 15 = 2428 testes verdes; hash drift 1 ficheiro; lint zero; ADRs 77 preservado (sem ADR nova); zero LOC vanilla typst lab tocado.
- **Marco "Cluster Gradient industry-grade qualidade visual"** documentado em relatório §1 + ADR-0089 §anotação P268.2.
