# typst-passo-272 — P-Gradient-Coons-RGB-Final (converge Conic RGB Type 4 Gouraud → Type 6 Coons N=stops*4; ADR-0090 REVOGADO)

**Magnitude**: M (cap composto: L3 additions hard ≤ 200 / soft ≤ 120 LOC + testes additions hard ≤ 30 / soft ≤ 22; **net LOC negativo esperado -200 a -150** via remoção helpers P268+P268.2).
**Cluster**: Visualize / Gradient / PDF export (convergência arquitectural).
**Tipo**: passo principal P272. Refino estratégico — converge 2 estratégias Conic L3 emit (Type 4 + Type 6) para 1 estratégia única (Type 6 Coons cobre 8/8 spaces).
**Origem**: relatório P270.4 §7 + §8 "P-Gradient-Coons-RGB-Final candidato futuro M"; relatório P271 §"sequência pós-P271".
**Sequência**: P270.3 (Coons RGB infra `#[allow(dead_code)]`) + P270.4 (Coons CMYK activado) → **P272 (Coons RGB activado; Type 4 Gouraud removido; ADR-0090 REVOGADO)** → cluster Gradient L3 emit estratégia única feature-complete.
**Decisões prévias**: utilizador escolheu (1) Remover literal P268+P268.2 helpers; (2) Strategy N = stops * 4 patches; (3) ADR-0090 REVOGADO + ADR-0092 expandida.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0090 transição REVOGADO + ADR-0092 anotação cumulativa P272 + ADR-0093 aplicado scope-out → prompt L0 → fix-hashes → testes-primeiro → código → ADR-0092 promoção final.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-coons-rgb-final-passo-272.md` imutável. **Décimo terceiro consumo directo de fonte** (P262-P271 + **P272 vanilla Typst blog 2023 1-patch-per-stop + Cairo Type 6 + ISO 32000-1 §7.5.7.4 N stops*4 patches**).

3. **ADR-0090 transição EM VIGOR → REVOGADO** — paridade ADR-0093 §Pattern 1 §"Quando NÃO aplicar" (revogação invalida decisão de fundo). Pattern "ADR REVOGADO + substituta":
    - ADR-0090 §status: `EM VIGOR` → `REVOGADO P272`.
    - ADR-0090 §"Revogação P272" secção nova: documenta motivo (convergência industry-aligned para mesh-based Type 6 Coons; eliminação 2 estratégias coexistentes).
    - ADR-0092 expandida cumulativamente P272 cobrindo agora estratégia Conic única para 8/8 spaces.

4. **Sub-padrão "ADR REVOGADO + substituta" N=? inaugural ou cumulativo** — Fase A §A.X clarifica histórico cristalino (contexto §3 lista REVOGADO 2 mas sem passos identificados).

5. **ADR-0092 anotação cumulativa P272** — estratégia Conic única (Coons) para 8/8 spaces materializada. Estende §"Decisão Cenário A revisado" → §"Decisão Cenário A revisado FINAL (Coons única estratégia)".

6. **ADR-0091 anotação cumulativa P272** — cluster Gradient L3 emit agora estratégia única Coons (8/8 spaces); preserved feature-complete 24/24.

7. **ADR-0093 aplicado** — Pattern 1 §"Quando NÃO aplicar" — primeira aplicação prática post-formalização P271. Sub-padrão "Aplicação meta-ADR" **N=1 inaugural** (candidato observar reincidência).

8. **ADR-0087/ADR-0088/ADR-0089 preservadas literal** — Linear/Radial/Conic identidade preservada. ADR-0089 anotação cumulativa P272 (Conic agora estratégia única).

9. **ADR-0054 anotação cumulativa P272** — cluster Gradient strategy unificada simplifica perfil graded DEBT-1.

10. **ADR-0083 preservada** — Color paridade vanilla; cluster CMYK ainda 24/24 P270.4 preservado.

11. **ADR-0039 preservado** — TextStyle intocado.

12. **ADR-0018 preservado** — implementação autónoma; sem dependências externas.

13. **Crystalline-lint zero violations** obrigatório.

14. **Reutilização literal helpers cross-passos** **N=10 → N=11 cumulativo** (consolidação clara persistente):
    - `interpolate_in_space` L1 P270 dispatcher arm RGB para corner colors interpolation entre stops.
    - 3 helpers Coons P270.3: `bezier_control_points_for_arc`, `compute_coons_patches_n_stops` (extendido), `emit_conic_coons_stream` (activado).
    - Helpers Color P257 cross-space.

15. **Regressão tests P262-P271 preservada parcialmente** — **2572 baseline NÃO preservado literal** porque tests P268+P268.2 são removidos. Net result esperado: 2572 - 30 (P268+P268.2 removed) + 20-25 (Coons RGB new) = **~2562-2567**.

16. **Cap LOC hard vs soft explícito** (5ª aplicação consolida sub-padrão N=5):
    - **Cap hard L3 additions**: 200 LOC. Estouro dispara §política condição 4.
    - **Cap soft L3 additions**: 120 LOC. Estouro regista relatório.
    - **L3 removals**: sem cap (limpeza esperada ~310 LOC).
    - **Cap hard tests additions**: 30.
    - **Cap soft tests additions**: 22.

17. **Strategy patches N = stops * 4** — divergência intencional Typst original blog 2023 ("1 patch per stop"). Justificativa: qualidade visual angular superior; reader compatibility preserved (Cairo/Inkscape suportam patches sub-stop arbitrários).

18. **Fase A com industry research proactiva NÃO aplicável** — consolidação industry P270.3 ADR-0092 reutilizada literal. Coons + 7 spaces RGB-family + perceptual = caso testado há 20+ anos Cairo. Sub-padrão NÃO incrementa (N=4 preserved).

19. **Vanilla read-first preservado** — Typst original blog 2023 (referência "1 patch per stop"; cristalino diverge N=stops*4 fundamentado).

---

## §1 — Sub-passo P272.A — Diagnóstico empírico estado pré-P272 + Coons RGB extension

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-coons-rgb-final-passo-272.md`.

### Comandos exactos a executar

```bash
# 1. Cristalino L3 actual Conic emit (dispatcher dual P270.4)
rg -n "GradientObjectKind::Conic|emit_conic_gouraud_stream|emit_conic_coons_stream_cmyk|emit_conic_coons_stream " 03_infra/src/export.rs | head -30

# 2. Cristalino helpers Coons P270.3 (verificar #[allow(dead_code)] estado)
rg -n "emit_conic_coons_stream|bezier_control_points_for_arc|compute_coons_patches_n_stops|#\[allow\(dead_code\)\]" 03_infra/src/export.rs | head -20

# 3. Cristalino helpers Type 4 Gouraud P268+P268.2 a remover
rg -n "emit_conic_gouraud_stream|compute_adaptive_n_conic|oklab_delta_e" 03_infra/src/export.rs | head -20

# 4. Cristalino tests P268+P268.2 a remover (paridade contagem)
rg -n "fn p268_|fn p268_2_" 03_infra/src/export.rs 01_core/src/ | head -40

# 5. Cristalino L1 interpolate_in_space arm RGB-family + perceptual
rg -n "interpolate_in_space|interpolate_oklab|interpolate_hsl|interpolate_oklch" 01_core/src/entities/gradient.rs | head -20

# 6. Cristalino ADRs REVOGADO histórico (sub-padrão N inaugural ou cumulativo?)
grep -l "REVOGADO" 00_nucleo/adr/*.md | head -10
rg -n "^Status.*REVOGADO" 00_nucleo/adr/*.md | head -10

# 7. Vanilla Typst blog 2023 strategy literal "1 patch per stop" reference
# (consolidação P270.3 ADR-0092 §"Pesquisa empírica industry"; reutilização)

# 8. Tests baseline pre-P272
cargo test -p typst-cristalino-infra 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.15)

```
§A.1 Cristalino L3 dispatcher Conic actual (P270.4):
     if conic.space == ColorSpace::Cmyk {
         emit_conic_coons_stream_cmyk(conic)  // P270.4 active
     } else {
         emit_conic_gouraud_stream(conic, compute_adaptive_n_conic(conic))  // P268+P268.2
     }

§A.2 Helpers Coons P270.3 RGB estado:
     emit_conic_coons_stream com #[allow(dead_code)] — reserved P-Gradient-Coons-RGB-Final.
     Strategy: 1 patch per stop (N = stops.len()).

§A.3 Helpers Type 4 Gouraud P268+P268.2 a remover:
     - emit_conic_gouraud_stream(conic, n) — ~190 LOC.
     - compute_adaptive_n_conic(conic) — ~50 LOC.
     - oklab_delta_e(c0, c1) — ~25 LOC.
     - Helper auxiliares Oklab N=16 — preservados (usados Linear/Radial).
     Total ~265 LOC + ~30 tests removidos.

§A.4 Tests P268+P268.2 a remover:
     - p268_export_pdf_conic_emits_shading_type_4 + dedup + cluster.
     - p268_emit_conic_gouraud_stream_n32_size + min_8_slices.
     - p268_oklab_sample_stops_conic_red_blue_endpoints.
     - p268_2_compute_adaptive_n_* (8 tests).
     - p268_2_oklab_delta_e_* (4 tests).
     - p268_2_export_pdf_conic_adaptive_n_* (4 tests).
     - p268_2_pdf_bytes_* (3 snapshot tests).
     Total ~30 tests.

§A.5 PROPOSTA emit_conic_coons_stream_rgb extension:
     - Activar P270.3 helper (remover #[allow(dead_code)]).
     - Estender strategy: N = stops.len() * 4 patches angulares (cada 360°/(N*4) graus).
     - Para cada par stops adjacentes (stop_i, stop_i+1):
       - 4 patches inter-stop.
       - Corner colors interpolados via interpolate_in_space:
         - patch 1: corners stop_i + interp(0.25).
         - patch 2: corners interp(0.25) + interp(0.5).
         - patch 3: corners interp(0.5) + interp(0.75).
         - patch 4: corners interp(0.75) + stop_i+1.

§A.6 PROPOSTA dispatcher Conic em emit_gradient_objects (P272):
     // Substituir P270.4 dispatcher dual por estratégia única:
     GradientObjectKind::Conic(conic) => {
         let stream = if conic.space == ColorSpace::Cmyk {
             emit_conic_coons_stream_cmyk(conic)  // P270.4 preserved
         } else {
             emit_conic_coons_stream_rgb(conic)  // P272 new
         };
         let colorspace = if conic.space == ColorSpace::Cmyk {
             "/DeviceCMYK"
         } else {
             "/DeviceRGB"
         };
         // ... emit /ShadingType 6 + colorspace + stream
     }

§A.7 ADR-0090 transição REVOGADO P272:
     - Status: EM VIGOR → REVOGADO.
     - §"Revogação P272" nova: documenta motivo industry-aligned (Cairo/Inkscape/Typst blog 2023).
     - Cross-reference ADR-0092 expandida.

§A.8 ADR-0092 anotação cumulativa P272 expansão:
     - §"Decisão Cenário A revisado" → §"Decisão Cenário A revisado FINAL".
     - Estratégia unificada Coons para 8/8 spaces.
     - Type 4 Gouraud descontinuado.

§A.9 Sub-padrão "ADR REVOGADO + substituta" N empírico cristalino:
     - Pesquisar `^Status.*REVOGADO` em 00_nucleo/adr/.
     - Se N=0 prévio (REVOGADO 2 em contexto §3 sem passos identificados — pode ser ADRs nascidas REVOGADO ou substituídas):
       - P272 é primeiro caso "ADR REVOGADO + substituta" formal.
       - Sub-padrão N=1 inaugural; observar reincidência.

§A.10 Estimativa cap LOC:
     - Additions (Coons RGB extension N=stops*4): ~80-120 LOC.
     - Removals (Type 4 Gouraud helpers + tests): ~310 LOC.
     - Net change: -200 a -150 LOC (negativo; limpeza).
     - Cap hard additions 200; folga 40-60%.
     - Cap soft additions 120; folga 0-33%.

§A.11 Tests delta esperado:
     - Removals: ~30 tests P268+P268.2.
     - Additions: ~20-25 tests Coons RGB N=stops*4.
     - Net: -5 a -10 tests.
     - Baseline 2572 → ~2562-2567.

§A.12 Defaults preservam P262-P267 + P270.4 CMYK:
     - Linear: preserved (P262/P263/P270.1/P270.2).
     - Radial: preserved (P264/P265/P269/P270.1/P270.2).
     - Conic CMYK: preserved (P270.4 Coons CMYK).
     - Conic RGB-family + perceptual: MUDAR (Type 4 → Type 6).
     - Conic byte snapshots MUDAM intencionalmente (behaviour change).

§A.13 Cenário detectado:
     - **B1 fecho conceptual** (helpers preparados P270.3; refactor cirúrgico).
     - **B2 sub-passos improvável** (P270.3 + P270.4 cobriram complexidade).

§A.14 Strategy N = stops * 4 justificativa empírica:
     - Cairo precedente: patches sub-stop arbitrários supported.
     - Typst original blog 2023: "1 patch per stop" simplicity.
     - Cristalino divergência: N=stops*4 trade-off qualidade visual vs LOC.
     - Cap LOC accommodates (~80-120 additions).

§A.15 Decisão arquitectural — estratégia Conic única Coons; ADR-0090
     REVOGADO; ADR-0092 expandida; cluster Gradient L3 emit
     simplificado.
```

### Critério de aceitação Fase A

- §A.1 confirma dispatcher P270.4 actual.
- §A.2 confirma helpers Coons P270.3 disponíveis com `#[allow(dead_code)]`.
- §A.3 + §A.4 identificam removals literais (~310 LOC + ~30 tests).
- §A.5 confirma extension viável N=stops*4 + corner colors interpolation.
- §A.7 + §A.8 confirmam ADR transições.
- §A.9 clarifica sub-padrão "ADR REVOGADO + substituta" N empírico.
- §A.10 confirma cap hard 200 com folga 40-60%.
- §A.12 confirma defaults preserved partial (Conic RGB byte snapshots mudam intencionalmente).

---

## §2 — Sub-passo P272.B — ADR transições + anotações cumulativas

### B.1 — ADR-0090 transição EM VIGOR → REVOGADO

Estrutura update `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`:

```
**Status**: EM VIGOR → **REVOGADO P272**.
**Data revogação**: 2026-05-17.

## Revogação P272

**Motivo**: convergência industry-aligned para mesh-based Type 6
Coons (Cairo/Inkscape/Typst original blog 2023 precedente literal).

Pesquisa industry P270.3 + experiência P270.4 (Coons CMYK)
materializaram Type 6 com sucesso. Eliminar 2 estratégias coexistentes
(Type 4 Gouraud + Type 6 Coons) reduz complexidade arquitectural sem
perda funcional.

**Substituição**: ADR-0092 estendida cumulativamente P272 cobrindo
estratégia Conic unificada Coons para 8/8 spaces.

**Implicações materializadas P272**:
- emit_conic_gouraud_stream P268 removido.
- compute_adaptive_n_conic P268.2 removido.
- oklab_delta_e P268.2 removido.
- ~30 tests P268+P268.2 removed.
- emit_conic_coons_stream_rgb P272 active (extension P270.3 N=stops*4).
- Dispatcher Conic em emit_gradient_objects unificado (Coons para 8/8 spaces).

**Decisão de fundo invalidada**: Type 4 Free-Form Gouraud já não é
estratégia Conic cristalina; substituída por Type 6 Coons Patch Mesh
(Cairo/Inkscape industry-aligned).

**Pattern aplicado**: ADR-0093 §Pattern 1 §"Quando NÃO aplicar" —
revogação invalida decisão de fundo; use status REVOGADO + ADR
substituta. Primeira aplicação prática post-formalização ADR-0093
P271.

**Sub-padrão "ADR REVOGADO + substituta"** N=[1 inaugural | N
cumulativo conforme Fase A §A.9].

**Sub-padrão "Aplicação meta-ADR" N=1 inaugural** — primeira aplicação
prática ADR-0093 §Pattern 1 §"Quando NÃO aplicar" pós-formalização P271.

**Trabalho prévio preservado historicamente**:
- ADR-0090 conteúdo original preserved como registo arquitectural.
- Industry research P268.1-correção preserved.
- Sub-padrão "Correcção ADR pré-commit" anti-pattern preservado.

Cross-reference: ADR-0092 §"Anotação cumulativa P272 — Decisão
Cenário A revisado FINAL".
```

### B.2 — ADR-0092 anotação cumulativa P272 (expansão final)

Adicionar após §"Anotação cumulativa P270.4":

```
## Anotação cumulativa P272 — Decisão Cenário A revisado FINAL (estratégia única Coons 8/8 spaces)

**Data**: 2026-05-17.
**Motivo**: ADR-0090 REVOGADO P272 (Type 4 Gouraud descontinuado);
ADR-0092 estendida cobrindo estratégia Conic unificada para 8/8
spaces.

**Estratégia única Coons materializada**:
- RGB-family + perceptual (7 spaces: Oklab/Oklch/sRGB/Luma/LinearRGB/
  HSL/HSV): `/ShadingType 6` Coons + `/ColorSpace /DeviceRGB` +
  corner colors RGB 3 bytes.
- CMYK (P270.4 preserved): `/ShadingType 6` Coons + `/ColorSpace /DeviceCMYK`
  + corner colors CMYK 4 bytes.

**Strategy N = stops * 4 patches** (divergência intencional Typst
original blog 2023 "1 patch per stop"):
- Para cada par stops adjacentes, 4 patches angulares inter-stop.
- Corner colors interpolados via `interpolate_in_space` L1 P270 em
  t = 0.25, 0.5, 0.75.
- Justificativa: qualidade visual angular superior; cap LOC
  accommodates.

**Helpers reutilizados literal P272**:
- emit_conic_coons_stream_rgb (P270.3 + extension N=stops*4).
- bezier_control_points_for_arc (P270.3 preserved).
- compute_coons_patches_n_stops_extended (P270.3 + multiplier 4).
- interpolate_in_space arm RGB-family (L1 P270 dispatcher).
- Helpers Color P257 cross-space.

**Helpers REMOVED P272**:
- emit_conic_gouraud_stream (P268).
- compute_adaptive_n_conic (P268.2).
- oklab_delta_e (P268.2; CMYK pipeline path; mover para legacy se
  outras callers — Fase A §A.3 verifica).

**Tests REMOVED P272**: ~30 tests P268 + P268.2.

**Tests ADDED P272**: ~20-25 tests Coons RGB N=stops*4 (paridade
estrutural P270.4 Coons CMYK; corner colors interpolated).

**Cluster Gradient L3 emit estratégia unificada feature-complete
24/24** — mantém marco P270.4 mas com simplicidade arquitectural.

**ADR-0090 REVOGADO**: Type 4 Gouraud descontinuado; cross-reference
ADR-0090 §"Revogação P272".

**Sub-padrão "ADR REVOGADO + substituta"** consolidado N=[1
inaugural | N cumulativo conforme Fase A].

**Sub-padrão "Aplicação meta-ADR" N=1 inaugural** — primeira
aplicação ADR-0093 Pattern 1 pós-formalização P271.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=10 → N=11
cumulativo.

**Sub-padrão "Reutilização literal helpers cross-passos"** N=10 →
N=11 cumulativo.
```

### B.3 — ADR-0091 anotação cumulativa P272

Adicionar após §"Anotação cumulativa P271":

```
## Anotação cumulativa P272 — Estratégia Conic unificada Coons

§"Decisão L3 (materializada P270.1+P270.2+P270.4)" estendida P272 —
Conic agora estratégia única Coons para 8/8 spaces (ADR-0090
REVOGADO; ADR-0092 expandida cumulativamente).

**Estratégia L3 emit pós-P272**:
- Linear: `/ShadingType 2` axial (preserved).
- Radial: `/ShadingType 3` radial (preserved).
- Conic: **`/ShadingType 6` Coons unified** (RGB + CMYK).

Cluster Gradient L3 emit estratégia única materializada feature-complete
24/24. Ver ADR-0092 §"Anotação cumulativa P272 — Decisão Cenário A
revisado FINAL".
```

### B.4 — ADR-0089 anotação cumulativa P272

Adicionar após §"Anotação cumulativa P271":

```
## Anotação cumulativa P272 — Conic estratégia única Coons (2 emit paths convergem)

Conic L3 emit estratégia unificada materializada — 2 emit paths
coexistentes P270.4 (Type 4 Gouraud RGB + Type 6 Coons CMYK)
convergem em **Type 6 Coons único** para 8/8 spaces:
- ADR-0090 (Type 4) REVOGADO P272.
- ADR-0092 (Coons) expandida P272.

Cluster Conic L3 emit simplificado; 8/8 spaces materializados.
Ver ADR-0092 §"Anotação cumulativa P272".
```

### B.5 — ADR-0054 anotação cumulativa P272

Adicionar:

```
P272 — cluster Gradient strategy unificada Coons; perfil graded
DEBT-1 simplificado por eliminação 2 estratégias Conic coexistentes
(P270.3+P270.4 → P272 single strategy). ADR-0090 REVOGADO; ADR-0092
expandida.
```

### B.6 — L0 `entities/gradient.md` anotação P272

Adicionar após anotação P270.4:

```
**Anotação P272**: Conic L3 emit estratégia unificada Coons —
emit_conic_coons_stream_rgb (P270.3 activado + N=stops*4 extension)
+ emit_conic_coons_stream_cmyk (P270.4 preserved). Dispatcher Conic
único `/ShadingType 6` para 8/8 spaces. Corner colors interpolated
via interpolate_in_space L1 P270 (N=stops*4 patches inter-stop;
t=0.25/0.5/0.75 corner colors). emit_conic_gouraud_stream +
compute_adaptive_n_conic + oklab_delta_e removidos. ADR-0090 REVOGADO.
Ver ADR-0092 §"Anotação cumulativa P272".
```

### B.7 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0. Zero violations.

---

## §3 — Sub-passo P272.C — Materialização L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0090 transição REVOGADO §2.B.1.
3. ADR-0092 + ADR-0091 + ADR-0089 + ADR-0054 anotações §2.B.2-§2.B.5.
4. L0 anotação §2.B.6.
5. `crystalline-lint --fix-hashes`.
6. **Testes-primeiro** — adicionar ~20-25 testes Coons RGB N=stops*4 ANTES de qualquer LOC L3.
7. L3 código additions — emit_conic_coons_stream_rgb extension N=stops*4 + corner colors interp + dispatcher unificado.
8. L3 código removals — emit_conic_gouraud_stream + compute_adaptive_n_conic + oklab_delta_e + ~30 tests P268+P268.2.
9. Verificação final.

### Cap LOC

- **L3 additions hard**: 200 LOC. Estouro dispara §política condição 4.
- **L3 additions soft**: 120 LOC. Estouro regista relatório.
- **L3 removals**: sem cap (~310 LOC esperado).
- **Tests additions hard**: 30.
- **Tests additions soft**: 22.

### Alteração L3 esperada

```rust
// 03_infra/src/export.rs P272

// 1. Activar P270.3 helper (remover #[allow(dead_code)])
fn emit_conic_coons_stream(conic: &Conic) -> Vec<u8> {
    // P270.3 RGB version preserved + extension P272 N=stops*4
}

// 2. Helper renomeado para clarity P272
fn compute_coons_patches_n_stops_extended(conic: &Conic) -> usize {
    conic.stops.len() * 4  // P272 — divergência Typst blog 2023 (1 per stop)
}

// 3. Corner colors interpolation P272
fn emit_conic_coons_stream_rgb(conic: &Conic) -> Vec<u8> {
    let n_per_stop_pair = 4;  // P272 multiplier
    let n_stop_pairs = conic.stops.len();
    let n_patches = n_stop_pairs * n_per_stop_pair;
    let mut stream = Vec::with_capacity(37 * n_patches);
    
    let center = (0.5, 0.5);
    let radius = 0.5;
    
    for stop_idx in 0..n_stop_pairs {
        let stop_curr = &conic.stops[stop_idx];
        let stop_next = &conic.stops[(stop_idx + 1) % n_stop_pairs];
        
        // 4 patches inter-stop
        for sub_idx in 0..4 {
            let t_start = sub_idx as f32 / 4.0;
            let t_end = (sub_idx + 1) as f32 / 4.0;
            
            // Corner colors interpolated via L1 dispatcher P270
            let color_start = interpolate_in_space(
                stop_curr.color, stop_next.color, t_start, conic.space
            );
            let color_end = interpolate_in_space(
                stop_curr.color, stop_next.color, t_end, conic.space
            );
            
            // Angle range per patch
            let angle_per_patch = std::f32::consts::TAU / n_patches as f32;
            let angle_start = (stop_idx * 4 + sub_idx) as f32 * angle_per_patch;
            let angle_end = angle_start + angle_per_patch;
            
            // Emit patch: 1 flag + 12 control points × 2 coord + 4 corners × 3 RGB
            // (paridade estrutural P270.3 RGB)
            stream.push(0); // flag
            // ... emit 12 control points (24 bytes)
            // ... emit 4 corner colors RGB (12 bytes)
            // Total 37 bytes/patch.
        }
    }
    
    stream
}

// 4. Dispatcher unificado em emit_gradient_objects
GradientObjectKind::Conic(conic) => {
    let stream = if conic.space == ColorSpace::Cmyk {
        emit_conic_coons_stream_cmyk(conic)  // P270.4 preserved
    } else {
        emit_conic_coons_stream_rgb(conic)  // P272 new
    };
    
    let colorspace = if conic.space == ColorSpace::Cmyk {
        "/DeviceCMYK"
    } else {
        "/DeviceRGB"
    };
    
    let decode_array = if conic.space == ColorSpace::Cmyk {
        "[0 1 0 1 0 1 0 1 0 1 0 1]"  // 6 pares
    } else {
        "[0 1 0 1 0 1 0 1 0 1]"  // 5 pares
    };
    
    let n_patches = if conic.space == ColorSpace::Cmyk {
        compute_coons_patches_n_stops(conic)  // CMYK: 1 per stop (P270.4)
    } else {
        compute_coons_patches_n_stops_extended(conic)  // RGB: stops*4 (P272)
    };
    
    let shading = format!(
        "<< /ShadingType 6 \
           /ColorSpace {} \
           /BitsPerCoordinate 8 \
           /BitsPerComponent 8 \
           /BitsPerFlag 8 \
           /Decode {} \
           /Length {} >>",
        colorspace, decode_array, stream.len()
    );
    // ... emit shading object
}

// 5. REMOVE: emit_conic_gouraud_stream + compute_adaptive_n_conic + oklab_delta_e
// (verificar Fase A §A.3 — se outros call sites usam oklab_delta_e, preservar
//  ou mover para helper genérico).
```

### Estrutura testes esperada

**Unit Coons RGB N=stops*4** (8 tests):
- `p272_emit_conic_coons_rgb_2_stops_8_patches`: 2 stops → 8 patches angulares.
- `p272_emit_conic_coons_rgb_5_stops_20_patches`: 5 stops → 20 patches.
- `p272_emit_conic_coons_rgb_stream_size_37n_bytes`: 37 × N patches.
- `p272_emit_conic_coons_rgb_corner_colors_interpolated`: corner colors via interpolate_in_space.
- `p272_emit_conic_coons_rgb_t_quarter_half_three_quarters`: t=0.25/0.5/0.75 corners.
- `p272_emit_conic_coons_rgb_paridade_p270_3_structural`: paridade estrutural com P270.3 (mesmo flag + control points layout).
- `p272_emit_conic_coons_rgb_default_oklab_preserva_p267_l1_sample`: defaults Oklab L1 sample preserved (L3 emit byte snapshot muda).
- `p272_emit_conic_coons_rgb_hue_wrap_oklch_hsl`: hue-wrap shorter em corner colors.

**E2E PDF dispatcher unificado** (6 tests):
- `p272_export_pdf_conic_rgb_shading_type_6_unified`: cluster Conic agora `/ShadingType 6` para todos.
- `p272_export_pdf_conic_oklab_devicergb`: 7 spaces RGB-family.
- `p272_export_pdf_conic_cmyk_devicecmyk_preserva_p270_4`: CMYK preserved.
- `p272_export_pdf_conic_no_more_shading_type_4`: confirma `/ShadingType 4` removido.
- `p272_export_pdf_cluster_3_variants_unified_strategy`: cluster preserved 24/24.
- `p272_export_pdf_conic_oklch_hsl_hue_wrap_correct`.

**Snapshot determinístico** (4 tests):
- `p272_pdf_bytes_conic_rgb_reproduziveis_n_stops_4_patches`.
- `p272_pdf_bytes_conic_cmyk_preserva_p270_4_byte_exact`.
- `p272_pdf_bytes_dispatcher_unified_reproduziveis`.
- `p272_pdf_bytes_no_gouraud_stream_present`.

**Regressão tests P262-P270.4** (verificar verdes):
- Linear/Radial tests preserved literal.
- Conic CMYK tests P270.4 preserved literal.
- Conic RGB tests P268+P268.2 REMOVED (~30 tests).

**Total tests added**: 8 + 6 + 4 = **18 tests**.
**Total tests removed**: ~30 tests.
**Net tests delta**: ~-12.

Cap testes additions hard 30 / soft 22. Folga grande (18 < 22).

---

## §4 — Sub-passo P272.D — README + relatório

1. **ADR-0090** transição REVOGADO P272 fechada.
2. **ADR-0092** anotação cumulativa P272 expansão fechada.
3. **ADR-0091** anotação cumulativa P272 (estratégia unificada).
4. **ADR-0089** anotação cumulativa P272 (Conic 2 paths convergem).
5. **ADR-0054** anotação cumulativa P272 (perfil graded simplificado).
6. **ADR-0087/0088/0083** preservadas literal.
7. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (~85-87% preserved; estratégia única simplifica arquitectura sem mudar cobertura).
   - Entrada P272 ~70-90 linhas (refino estratégico; 5 anotações cumulativas + 1 transição REVOGADO).
   - Cross-reference ADR-0090 REVOGADO + ADR-0092 expandida.
   - **Cluster Gradient L3 emit estratégia única Coons feature-complete 24/24 simplificado**.
8. **Distribuição ADRs**: total 81 preservado; **EM VIGOR 35 → 34** (-1 ADR-0090); **REVOGADO 2 → 3** (+1 ADR-0090). IMPLEMENTADO 31 preservado.
9. **Relatório** `00_nucleo/materialization/typst-passo-272-relatorio.md`:
   - Métricas finais (esperado 2572 - 30 + 18 = ~2560; net negativo intencional).
   - Fase A §A.5 + §A.9 + §A.14 decisões documentadas.
   - Diff helpers L3 antes/depois (3 helpers removed; 1 helper renamed + extended).
   - Sub-padrões + N cumulativo (incluindo "ADR REVOGADO + substituta" inaugural ou cumulativo).
   - Regressão tests P262-P270.4 não-Conic preservada literal.
   - **Cluster Gradient L3 emit estratégia única simplificada** — marco arquitectural.
   - **ADR-0090 REVOGADO** — primeira aplicação prática ADR-0093 Pattern 1 §"Quando NÃO aplicar".

---

## §política de paragem

1. **Fase A §A.3 revela oklab_delta_e usado por outros call sites** — helper não removível literal; mover para módulo genérico ou preservar. Confirmar antes de remover.

2. **Fase A §A.4 revela testes P268+P268.2 referenciados por outros sítios** — refactor expande. Confirmar.

3. **Cap LOC L3 additions hard (200) ameaça ser ultrapassado** — refactor maior que estimativa §A.10 (~80-120 LOC). Confirmar antes.

4. **Cap testes additions hard (30) ameaça ser ultrapassado**.

5. **Defaults Conic RGB L1 sample preservam P267** literal — sample em L1 não muda; só emit L3 muda (byte snapshots intencionalmente).

6. **Snapshot bytes PDF Conic RGB NÃO preservados** literal — behaviour change intencional (Type 4 → Type 6); tests P268+P268.2 byte snapshots removed; tests P272 byte snapshots new.

7. **Crystalline-lint reporta violations** após anotações + REVOGADO transition.

8. **Regressão tests P262-P270.4 não-Conic** — Linear/Radial/Conic CMYK tests devem permanecer verdes literal. §política absoluta.

9. **Fase A §A.9 sub-padrão "ADR REVOGADO + substituta" ambíguo** — contexto §3 indica REVOGADO 2 mas Fase A pode revelar passos identificados (N cumulativo) ou ADRs nascidas REVOGADO (N=1 inaugural P272).

10. **Cluster Gradient marco quebra** — `p272_export_pdf_cluster_3_variants_unified_strategy` falha.

11. **Strategy N=stops*4 produz banding visual** — Fase A teste empírico com pdftoppm/mupdf. Se banding visível, revisitar (N=stops*8 ou Type 7 Tensor).

12. **Anotações cross-ADR 5 ADRs coerência** — cada anotação P272 refere ADR-0092 §"Anotação cumulativa P272".

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P272 | Nota |
|---|---|---|
| **ADR REVOGADO + substituta** | **N=1 inaugural OU N cumulativo conforme Fase A §A.9** | P272 primeira aplicação prática ADR-0093 Pattern 1 §"Quando NÃO aplicar" pós-formalização |
| **Aplicação meta-ADR (ADR-0093)** | **N=1 inaugural** | P272 primeira aplicação prática meta-ADR ADR-0093 (Pattern 1 §"Quando NÃO aplicar") pós-formalização P271 |
| **Aplicação meta-ADR (ADR-0094)** | **N=1 inaugural** | P272 primeira aplicação prática meta-ADR ADR-0094 (Cap LOC hard/soft Pattern 1) pós-formalização P271 |
| Anotação cumulativa em vez de ADR nova | **N=10 → N=11 cumulativo** | + P272 ADR-0092 expansão |
| Reutilização literal helpers cross-passos | **N=10 → N=11 cumulativo** | + P272 (interpolate_in_space + helpers Coons P270.3 + Color P257) |
| Cap LOC hard vs soft explícito | **N=4 → N=5 cumulativo (consolidação total)** | + P272 |
| Diagnóstico imutável (décimo terceiro consumo) | **N=17 → N=18 cumulativo** | + P272 |
| Auditoria condicional (ADR-0084) | **N=16 → N=17 cumulativo** | + P272 |
| Auto-aplicação ADR-0065 inline | **N=16 → N=17 cumulativo** | + P272 |
| Anotação cumulativa cross-ADR | **N=5 → N=6 cumulativo** | + P272 (5 ADRs anotadas) |
| Fase A com industry research proactiva | N=4 preserved | P272 reutiliza consolidação P270.3 (não nova) |

### Marco arquitectural máximo P272

**Cluster Gradient L3 emit estratégia única Coons feature-complete
24/24 simplificado** — eliminação 2 estratégias coexistentes Conic
sem perda funcional. Cristalino agora oferece pipeline gradient
unificado:
- Linear: `/ShadingType 2` axial.
- Radial: `/ShadingType 3` radial.
- Conic: `/ShadingType 6` Coons (RGB N=stops*4 + CMYK N=stops).

**Primeira aplicação prática ADR-0093 Pattern 1 §"Quando NÃO aplicar"**
— pós-formalização P271 (passo metodológico). Sub-padrão "Aplicação
meta-ADR" inaugurado N=1.

**Sub-padrão "ADR REVOGADO + substituta"** N inaugural ou cumulativo
conforme Fase A §A.9. Primeira ADR cristalina REVOGADA com substituta
identificada literal (vs ADRs prévias REVOGADAS sem substituta clara).

### Sequência pós-P272

- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **P-Gradient-CMYK-ICC** (S-M; PDF/A compliance).
- **P-Gradient-Adaptive-Multispace** (S; HSL/Oklch banding refino).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 / Stroke<Length> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

---

## §referências cross-passos

- **P262/P264/P267** — Variant L1+stdlib (preservados).
- **P263/P265** — Linear/Radial L3 emit (preserved).
- **P268** — Conic Type 4 Gouraud (DESCONTINUADO P272; ADR-0090 REVOGADO).
- **P268.1-correção** — ADR-0090 correcção factual (preserved historicamente).
- **P268.2** — Adaptive N hybrid (DESCONTINUADO P272; helpers removed).
- **P269** — Radial focal_* (preservado).
- **P270 série completa (P270-P270.4)** — ColorSpace runtime + L3 emit 24/24 (preservado, Conic unificado P272).
- **P270.3** — Coons RGB infra (ACTIVADO P272; emit_conic_coons_stream_rgb extension).
- **P270.4** — Coons CMYK (preserved literal; estratégia consistente P272).
- **P271** — Meta-formalização ADR-0093 + ADR-0094 (precedente directo aplicado P272).
- ADR-0090 — Type 4 strategy (REVOGADO P272).
- ADR-0092 — Conic Coons (expandida cumulativamente P272).
- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa P272).
- ADR-0089 — Gradient Conic-only (anotada cumulativa P272).
- ADR-0054 — Perfil graded (anotada cumulativa P272 simplificada).
- ADR-0093 — Meta-metodologia evolução ADRs (primeira aplicação prática Pattern 1 §"Quando NÃO aplicar").
- ADR-0094 — Meta-operacional specs (primeira aplicação prática Cap LOC hard/soft).
- ADR-0085 — Diagnóstico imutável (décimo terceiro consumo).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.3 + §A.4 críticas** — verificar `oklab_delta_e` callers e tests P268+P268.2 dependencies antes de remover. §política condições 1 + 2.
- **Fase A §A.9 clarifica sub-padrão "ADR REVOGADO + substituta"** N empírico (inaugural ou cumulativo).
- **Snapshot bytes Conic RGB MUDAM intencionalmente** — Type 4 → Type 6 behaviour change; remover snapshots P268 + adicionar snapshots P272. §política condição 6.
- **Regressão tests P262-P270.4 não-Conic zero** — Linear/Radial/Conic CMYK preserved literal. §política condição 8 absoluta.
- **Strategy N=stops*4 patches Coons RGB** — divergência intencional Typst original blog 2023; justificativa qualidade visual.
- **Corner colors interpolados via L1 dispatcher P270** — `interpolate_in_space(c0, c1, t, space)` em t = 0.25/0.5/0.75.
- **Helpers removed P272**: emit_conic_gouraud_stream + compute_adaptive_n_conic + oklab_delta_e (verificar callers Fase A).
- **Tests removed P272**: ~30 tests P268+P268.2.
- **Tests added P272**: ~18-20 tests Coons RGB N=stops*4.
- **Net tests change**: ~-10 a -12 (negativo intencional).
- **Net LOC change**: ~-200 a -150 (negativo intencional; limpeza).
- **Cap hard L3 additions 200 + testes additions hard 30** — gate absoluto.
- **Cap soft L3 additions 120 + testes additions soft 22** — informativo.
- **Anotações cross-ADR 5 ADRs** — verificar coerência cada referência ADR-0092 §"Anotação cumulativa P272".
- **Cluster Gradient L3 emit estratégia única Coons feature-complete 24/24** — marco final documentado em relatório §1 + ADR-0092 §"Anotação cumulativa P272".
- **Relatório final esperado**: 2572 - 30 + 18 ≈ **2560 testes verdes** (net negativo); hash drift L0; lint zero; ADRs 81 preservado; EM VIGOR 35 → 34; REVOGADO 2 → 3.
