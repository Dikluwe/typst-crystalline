# typst-passo-270.1 — L3 emit multi-space (7 spaces RGB-family + perceptual via Oklab pipeline)

**Magnitude**: M+ (cap composto: L3 hard ≤ 400 LOC + testes hard ≤ 35; testes soft ≤ 50).
**Cluster**: Visualize / Gradient / PDF export (refino L3).
**Tipo**: sub-passo .1 da série P270. Pattern análogo P268.1/P268.2 (sub-passos materiais da série P268).
**Origem**: ADR-0091 §"Decisão L3 futura" P270.1; relatório P270 §8 pendência reservada.
**Sequência**: P270 (L1+stdlib activado; L3 Oklab hardcoded) → **P270.1 (L3 pipeline multi-space; 7 spaces)** → P270.2 (L3 CMYK directo).
**Estratégia decidida**: Op B uniforme (utilizador escolheu pós-P269) — Oklab pipeline para 7 spaces RGB-family + perceptual; CMYK fica P270.2 separado.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0091 anotação cumulativa → prompt L0 → fix-hashes → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-l3-multispace-passo-270-1.md` imutável. **Nono consumo directo de fonte** (P262/P264/P267/P268/P268.1/P268.2/P269/P270 vanilla + **P270.1 vanilla pipeline emit**).

3. **ADR-0091 anotação cumulativa P270.1** — não criar ADR-0092. Sub-padrão "Anotação cumulativa em vez de ADR nova" **N=7 → N=8 cumulativo**. ADR-0091 §"Decisão L3 futura" → §"Decisão L3 materializada P270.1".

4. **ADR-0083 §"DeviceCMYK PDF" preserved scope-out** — revogação adiada para P270.2.

5. **ADR-0087/ADR-0088/ADR-0089/ADR-0090 preservadas literal** — variant strategies intocadas. Anotação cumulativa em cada uma (P270.1 L3 emit multi-space materializado).

6. **ADR-0039 preservado** — TextStyle intocado.

7. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P270.1 (cluster Gradient L3 emit feature-complete 7/8 spaces; CMYK P270.2 último).

8. **ADR-0018 preservado** — implementação autónoma; sem dependências externas adicionais.

9. **Crystalline-lint zero violations** obrigatório.

10. **Reutilização literal `interpolate_in_space` P270** — dispatcher L1 reutilizado em L3 pré-amostragem; **sub-padrão "Reutilização literal helpers cross-passos" N=6 → N=7 cumulativo**. Também reutiliza `oklab_sample_stops_linear/radial/conic` P263/P265/P268.2 como template.

11. **Vanilla read-first autorizado** — `lab/typst-original/crates/typst-pdf/src/gradient.rs` ou equivalente para pipeline multi-space PDF emit.

12. **Regressão tests P262-P270 proibida** — todos os 2500 tests baseline devem continuar verdes. Defaults `space: Oklab` produzem bytes bit-exact idênticos a P263/P265/P268.

13. **Cap LOC hard vs soft explícito** (lição P270 estouro):
    - **Cap hard**: estouro dispara §política condição. Parar antes de continuar.
    - **Cap soft**: estouro regista no relatório mas continua.
    - **Magnitude global**: estouro reformula spec inteira.

14. **CMYK NÃO tocado neste passo** — preserva pipeline P268 (Conic CMYK ignorado por falta de pré-amostragem CMYK em L3). P270.2 fecha.

---

## §1 — Sub-passo P270.1.A — Diagnóstico empírico pipeline L3 actual + vanilla

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-l3-multispace-passo-270-1.md`.

### Comandos exactos a executar

```bash
# 1. Cristalino L3 emit Linear actual P263 (pipeline Oklab hardcoded)
rg -n "oklab_sample_stops_linear|emit_linear|ShadingType.*2|/Function" 03_infra/src/export.rs | head -30

# 2. Cristalino L3 emit Radial actual P265
rg -n "oklab_sample_stops_radial|emit_radial|compute_radial_coords" 03_infra/src/export.rs | head -30

# 3. Cristalino L3 emit Conic actual P268+P268.2
rg -n "oklab_sample_stops_conic|emit_conic_gouraud_stream|compute_adaptive_n" 03_infra/src/export.rs | head -30

# 4. Cristalino L3 helpers Oklab — confirmar reutilização viável
rg -n "oklab_delta_e|color_to_oklab_with_alpha|interpolate_oklab" 03_infra/src/export.rs

# 5. Cristalino L1 dispatcher P270 — confirmar disponibilidade L3
rg -n "interpolate_in_space|to_<space>_components" 01_core/src/entities/gradient.rs

# 6. Vanilla L3 pipeline emit multi-space
rg -n "sample.*space|emit.*gradient.*space|stops.*sample|extra_stops" lab/typst-original/crates/typst-pdf/src/ | head -40

# 7. Vanilla constante N (N=16 para Oklab; pode ser diferente para outros)
rg -n "N.*16|sample.*16|stops.*16|num_stops|n_stops" lab/typst-original/crates/typst-pdf/src/ | head -20

# 8. Tests P263/P265/P268 P268.2 (regressão obrigatória)
rg -n "p263_|p265_|p268_export_pdf_conic|p268_2_" 03_infra/src/export.rs | head -20
cargo test -p typst-cristalino-infra gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.13)

```
§A.1 Cristalino L3 actual pipeline (3 sítios sample_stops):
     - oklab_sample_stops_linear (P263).
     - oklab_sample_stops_radial (P265).
     - oklab_sample_stops_conic (P268; usado P268.2 com adaptive N).
     - Todos usam interpolate_oklab + N=16 hardcoded.
§A.2 Cristalino L1 dispatcher P270 — interpolate_in_space disponível
     em L1; pode ser chamado de L3 (cross-camada permitida ADR-0029).
§A.3 Cristalino helpers conversão Color → space components — disponíveis
     L1; pode L3 chamar directamente.
§A.4 PROPOSTA renomear helpers L3:
     - `oklab_sample_stops_linear` → `multispace_sample_stops_linear`
     - idem radial, conic.
     - Assinatura ganha `space: ColorSpace` param.
§A.5 PROPOSTA pipeline multi-space:
     1. Stops como Color valores.
     2. Para cada par adjacente (stop_i, stop_i+1):
        a. Amostra N=16 entre eles via `interpolate_in_space(c_i, c_i+1, t, space)`.
     3. Cada amostra convertida para Color (RGB internamente; já é).
     4. Emit /Function FunctionType 2 em DeviceRGB (preservado P263/P265).
     5. Para Conic Type 4 Gouraud: vertex colors via amostra space; preservado P268.
§A.6 Vanilla pipeline equivalente:
     - Verificar literal vanilla `mix_iter` chamado para criar extra_stops.
     - Default N pode ser diferente; cristalino preserva N=16 paridade actual.
§A.7 Default Oklab preserva bytes P263/P265/P268:
     - Caso especial: arm Oklab em interpolate_in_space chama interpolate_oklab
       literal P262 → bytes idênticos pré-P270.1.
     - Verificar empíricamente: 2500 tests baseline preserved.
§A.8 Hue-wrap em pré-amostragem (HSL/Oklch/HSV):
     - Pré-amostragem N=16 entre 2 stops com hue distantes deve seguir
       caminho shorter (paridade L1 P270).
     - Confirmar que `interpolate_in_space` arm HSL/Oklch/HSV já faz
       hue-wrap shorter (P270).
§A.9 CMYK exclusão:
     - Pipeline P270.1 NÃO trata CMYK. CMYK preserva comportamento
       pré-P270.1 (sem implementação L3 dedicada). P270.2 fecha.
     - Cristalino actual: CMYK em sample stops cai através de qual path?
       Determinar se panic, default Oklab fallback, ou outro.
§A.10 Cap LOC estimativa:
     - Renomear 3 helpers + adicionar space param: ~15 LOC.
     - Substituir interpolate_oklab por interpolate_in_space: ~6 LOC.
     - 3 callsites production: ~12 LOC.
     - Validação CMYK exclusão: ~10 LOC × 3 = ~30 LOC.
     - Edge cases hue-wrap em pré-amostragem: ~30-50 LOC.
     - Tests: ~250-300 LOC.
     - **Total L3 production: ~80-100 LOC** (bem abaixo cap hard 400).
     - **Total L3 tests: ~250-300 LOC** (mas testes são separados do cap LOC L3 production).
§A.11 Cenário detectado:
     - **B1 fecho conceptual** esperado (pipeline pequeno; refactor cirúrgico).
     - **B2 sub-passos** se vanilla revelar matemática hue-wrap em pré-amostragem
       que cristalino L1 não cobre.
§A.12 Decisão arquitectural — Op B uniforme materializada conforme ADR-0091.
§A.13 CMYK strategy P270.2 preview — fallback temporário P270.1.
```

### Critério de aceitação Fase A

- §A.4 confirma renomeação viável literal.
- §A.7 confirma defaults Oklab preservam bytes via interpolate_oklab literal.
- §A.9 estabelece comportamento CMYK temporário.
- §A.10 confirma estimativa cap hard 400 com folga (~75% folga).
- §A.11 confirma cenário B1.

---

## §2 — Sub-passo P270.1.B — Anotação cumulativa ADR-0091 + L0

### B.1 — ADR-0091 anotação cumulativa P270.1

Adicionar após §"Decisão L3 futura" → renomear para §"Decisão L3 (materializada P270.1)":

```
## Anotação cumulativa P270.1 — L3 emit multi-space materializado (7 spaces)

**Data**: 2026-05-17.
**Motivo**: ADR-0091 §"Decisão L3 futura" P270.1 — Op B uniforme
materializada. Pipeline cristalino L3 ganha consciência de
`Radial.space` / `Linear.space` / `Conic.space` em pré-amostragem N=16.

**Mudança estrutural**:
- Helpers L3 renomeados: `oklab_sample_stops_<variant>` →
  `multispace_sample_stops_<variant>` com param `space: ColorSpace`.
- Pre-amostragem N=16 chama `interpolate_in_space` (L1 dispatcher P270)
  em vez de `interpolate_oklab` hardcoded.
- 3 callsites production passam `radial.space` / `linear.space` /
  `conic.space` para helper.

**7 spaces materializados L3 emit**:
- Oklab (preservado bit-exact via arm Oklab dispatcher).
- Oklch, sRGB, LinearRGB, Luma, HSL, HSV (novos; pipeline uniforme
  via interpolate_in_space).

**CMYK preservado scope-out P270.1**:
- Pipeline P270.1 NÃO trata CMYK.
- Fallback temporário (§A.9 diagnóstico) escolhe estratégia mínima
  (sample arm Oklab em CMYK ou panic explícito).
- P270.2 materializa CMYK directo `/DeviceCMYK`; revoga ADR-0083
  §DeviceCMYK.

**Helpers reutilizados literal**:
- `interpolate_in_space` (L1 P270 dispatcher).
- Conversões Color → space components (L1 P270 helpers).
- 3 helpers L3 P263/P265/P268 templates renomeados, estrutura preserved.

**Defaults Oklab preservam bytes P263/P265/P268**:
- Arm Oklab em `interpolate_in_space` chama `interpolate_oklab` literal
  P262.
- 2500 tests baseline P262-P270 zero regressão verificado §política
  condição 9.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=7 → N=8
cumulativo.

**Sub-padrão "Reutilização literal helpers cross-passos"** N=6 → N=7
cumulativo (dispatcher L1 P270 + 3 helpers L3 templates).

**Próximo passo**: P270.2 materializa CMYK directo /DeviceCMYK;
revoga ADR-0083 §DeviceCMYK final.
```

### B.2 — Anotação cumulativa ADR-0087/0088/0089/0090 P270.1

Cada uma recebe anotação curta:

```
## Anotação cumulativa P270.1 — L3 emit multi-space

Variant L3 emit ganha consciência de `space` field via
`multispace_sample_stops_<variant>(stops, space, N=16)`. Pipeline
preservado P263/P265/P268; só interpolação em pré-amostragem muda
(interpolate_in_space P270 em vez de interpolate_oklab hardcoded).
Default Oklab preserva bytes pré-P270.1. CMYK preserva scope-out
P270.1; P270.2 fecha. Ver ADR-0091 §"Decisão L3 (materializada
P270.1)".
```

### B.3 — Anotação cumulativa ADR-0054 P270.1

```
P270.1 — cluster Gradient L3 emit feature-complete 7/8 spaces
(CMYK último); perfil graded DEBT-1 preservado (refino L3 sem
mudar estratégia ADR-0087/0088/0089/0090).
```

### B.4 — L0 prompt `entities/gradient.md` anotação P270.1

Adicionar à secção P270 existente:

```
**Anotação P270.1**: pipeline L3 emit pré-amostragem N=16 ganha
consciência de space — helpers renomeados
`multispace_sample_stops_<variant>(stops, space)` invocam
`interpolate_in_space` (P270 dispatcher) em vez de `interpolate_oklab`
hardcoded. 7 spaces materializados; CMYK adiado P270.2. Defaults
Oklab preservam bytes pré-P270.1 bit-exact.
```

### B.5 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0. Zero violations.

---

## §3 — Sub-passo P270.1.C — Materialização L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 + ADR-0087/0088/0089/0090 + ADR-0054 anotações §2.
3. L0 anotação §2.B.4.
4. `crystalline-lint --fix-hashes`.
5. **Testes-primeiro** — adicionar ~25-35 testes ANTES de qualquer LOC L3.
6. L3 código — renomear helpers + space-aware pré-amostragem + callsites.
7. Verificação final.

### Cap LOC

- **L3 hard**: 400 LOC em `03_infra/src/export.rs`. Estouro dispara §política condição 4.
- **L3 soft**: 250 LOC (estimativa empírica §A.10 ~80-100; folga grande).
- **Testes hard**: 50 (cap absoluto).
- **Testes soft**: 35 (alvo).

### Alteração L3 esperada

```rust
// 03_infra/src/export.rs

// Renomeação + space param
fn multispace_sample_stops_linear(
    linear: &Linear,
    n: usize,
) -> Vec<Color> {
    sample_stops_in_space(&linear.stops, linear.space, n)
}

fn multispace_sample_stops_radial(
    radial: &Radial,
    n: usize,
) -> Vec<Color> {
    sample_stops_in_space(&radial.stops, radial.space, n)
}

fn multispace_sample_stops_conic(
    conic: &Conic,
    n: usize,
) -> Vec<Color> {
    sample_stops_in_space(&conic.stops, conic.space, n)
}

// Helper central novo
fn sample_stops_in_space(
    stops: &[GradientStop],
    space: ColorSpace,
    n: usize,
) -> Vec<Color> {
    // CMYK fallback temporário P270.1
    if space == ColorSpace::Cmyk {
        // Estratégia decidida §A.9 — pode ser fallback Oklab
        // ou panic explícito. Decisão Fase A.
        return sample_stops_oklab_fallback(stops, n);
    }
    
    let mut samples = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        // Encontra par adjacente para t
        let (c0, c1, local_t) = find_adjacent_stops(stops, t);
        // Reutiliza L1 dispatcher P270 literal
        samples.push(interpolate_in_space(c0, c1, local_t, space));
    }
    samples
}

// Callsites production (3 sítios actualizados)
// Antes P263:
let samples = oklab_sample_stops_linear(linear, 16);

// Depois P270.1:
let samples = multispace_sample_stops_linear(linear, 16);
// linear.space contém info; helper internamente despacha.
```

### Estrutura testes esperada

**Unit pré-amostragem multi-space** (21 tests; 7 spaces × 3 variants):
- `p270_1_linear_sample_oklab_preserva_p263`: arm Oklab idêntico.
- `p270_1_linear_sample_srgb`: 16 amostras red↔blue em sRGB.
- `p270_1_linear_sample_oklch_hue_wrap`: hue shorter em pré-amostragem.
- (4 análogos Linear para outros spaces.)
- (7 análogos Radial.)
- (7 análogos Conic.)

**Unit dispatcher integração** (4 tests):
- `p270_1_sample_stops_oklab_idempotente_p263`: bytes idênticos.
- `p270_1_sample_stops_n16_paridade`: N=16 paridade actual.
- `p270_1_sample_stops_n_adaptive_conic_preserva_p268_2`: paridade.
- `p270_1_cmyk_fallback_estrategia_a9`: cenário definido §A.9.

**E2E PDF regressão** (5 tests):
- `p270_1_export_pdf_linear_oklab_bytes_paridade_p263`.
- `p270_1_export_pdf_radial_oklab_bytes_paridade_p265`.
- `p270_1_export_pdf_conic_oklab_adaptive_n_paridade_p268_2`.
- `p270_1_export_pdf_linear_hsl_bytes_diferentes_p263`.
- `p270_1_export_pdf_cluster_3_variants_multispace_coexistem`.

**Snapshot** (3 tests):
- `p270_1_pdf_bytes_oklab_default_reproduziveis`.
- `p270_1_pdf_bytes_hsl_reproduziveis`.
- `p270_1_pdf_bytes_oklch_hue_wrap_reproduziveis`.

Total esperado: 21 + 4 + 5 + 3 = **33 testes** (cap soft 35; cap hard 50; folga ~17).

---

## §4 — Sub-passo P270.1.D — Promoção + README + relatório

1. **ADR-0091** anotação cumulativa P270.1 fechada (Decisão L3 materializada).
2. **ADR-0087/0088/0089/0090** anotações cumulativas P270.1 adicionadas.
3. **ADR-0054** anotação cumulativa P270.1 adicionada.
4. **ADR-0083** preservada (CMYK ainda scope-out; P270.2).
5. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (+~2-3pp via 7 spaces L3 materializado).
   - Entrada P270.1 ~50-70 linhas (refino L3 da série P270).
   - Cross-reference ADR-0091 §anotação P270.1.
   - Cluster Gradient L3 emit: agora 3 variants × 7 spaces (CMYK último P270.2).
6. **Distribuição ADRs preservada** — total 78 mantido (sem ADR nova; só anotações cumulativas).
7. **Relatório** `00_nucleo/materialization/typst-passo-270-1-relatorio.md`:
   - Métricas finais (esperado 2500 + 33 = ~2533).
   - Fase A §A.9 CMYK fallback estratégia documentada.
   - Diff helpers L3 antes/depois.
   - Sub-padrões + N cumulativo.
   - Regressão zero P262-P270 baseline (2500 preservados).
   - **Cluster Gradient L3 emit 7/8 spaces materializado** marcado.
   - **CMYK P270.2 reservado** registado literal.

---

## §política de paragem

1. **Fase A §A.2 revela que L1 dispatcher P270 não é chamável de L3** — ADR-0029 (pureza física L1 sem I/O) pode restringir. Confirmar; pode requerer extracção dispatcher para crate compartilhada.

2. **Cap LOC L3 hard (400) ameaça ser ultrapassado** — refactor maior que estimativa §A.10 (~80-100 LOC). Confirmar antes de continuar.

3. **Cap testes hard (50) ameaça ser ultrapassado** — > 50 testes indica scope creep.

4. **§A.7 verificação falha**: defaults Oklab produzem bytes diferentes de P263/P265/P268. §política absoluta. Indica que arm Oklab em `interpolate_in_space` não é literal `interpolate_oklab` ou que `sample_stops_in_space` reordena operações.

5. **Snapshot bytes PDF não reproduzíveis** — float não-determinismo em pré-amostragem multi-space; pode ocorrer se `interpolate_hsl/oklch` usa f64 internamente.

6. **Crystalline-lint reporta violations** após anotações.

7. **Regressão tests P262-P270** — qualquer test anterior falha. §política absoluta.

8. **CMYK fallback estratégia §A.9 ambígua** — Fase A não consegue decidir entre Oklab fallback ou panic. Confirmar.

9. **Cluster Gradient marco quebra** — `p270_1_export_pdf_cluster_3_variants_multispace_coexistem` falha.

10. **Helpers L3 renomeação revela call sites externos** — outros sítios export.rs ou outros crates chamam `oklab_sample_stops_*` directamente. Refactor expande.

11. **Hue-wrap pré-amostragem N=16 produz banding visual** — pré-amostragem em HSL/Oklch com hue diff alto pode amostrar mal; N pode precisar ser adaptive (paridade P268.2 adaptive N). Indica decisão arquitectural não trivial; parar.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P270.1 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=7 → N=8** | + P270.1 anotada ADR-0091 (P258.B/P259.B/P263/P265/P268/P268.2/P270/**P270.1**) |
| Reutilização literal helpers cross-passos | **N=6 → N=7** | + P270.1 (dispatcher L1 P270 + helpers L3 templates P263/P265/P268) |
| Diagnóstico imutável (nono consumo) | **N=13 → N=14** | + P270.1 |
| Auditoria condicional (ADR-0084) | **N=12 → N=13** | + P270.1 |
| Auto-aplicação ADR-0065 inline | **N=11 → N=12** | + P270.1 |
| Cap LOC hard vs soft explícito | **N=1 inaugural** | P270.1 inaugura distinção (lição P270 estouro) |
| Anotação cumulativa cross-ADR | **N=1 → N=2** | P270 inaugurou 6 ADRs; **P270.1 estende com 5 ADRs** (ADR-0091/0087/0088/0089/0090 + ADR-0054) |

### Marco arquitectural P270.1

**Cluster Gradient L3 emit 7/8 spaces materializado** — paridade vanilla L3 user-facing excepto CMYK. P270.2 último para fechar cluster 8/8.

**Primeira aplicação "Cap LOC hard vs soft explícito"** — lição operacional P270 (cap "ou" L1/stdlib não disparou §política) aplicada. P270.1 distingue literal hard (gate) vs soft (informativo).

### Sequência pós-P270.1

- **P270.2** — L3 CMYK directo `/DeviceCMYK` (S+; revoga ADR-0083 §DeviceCMYK final). Resolve bug vanilla #4422 com implementação cristalina autónoma.
- **P-Gradient-Relative-Custom** (M; activa relative: RelativeTo).
- **ADR-0055bis variant-aware fonts** (M; refino Text).
- **P-Footnote-N** (M).
- **DEBT-33** + outros Visualize.

---

## §referências cross-passos

- **P270** — Gradient ColorSpace runtime L1+stdlib (ADR-0091; precedente directo materializado em L3 por este passo).
- **P263** — PDF Linear pipeline Oklab (template estrutural; renomeado P270.1).
- **P265** — PDF Radial pipeline Oklab (template; renomeado).
- **P268** — PDF Conic Type 4 Gouraud (template; renomeado).
- **P268.2** — Adaptive N hybrid (preservado; arm Oklab continua adaptive).
- **P269** — Radial focal_* (preservado; campo space aditivo cross-variant).
- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa P270.1).
- ADR-0087/0088/0089/0090 — Variant strategies (anotadas cumulativa P270.1).
- ADR-0054 — Perfil graded (anotada cumulativa P270.1).
- ADR-0083 — Color 8/8 spaces (§DeviceCMYK preservada P270.1; P270.2 fecha).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.2 é crítica** — confirmar `interpolate_in_space` é chamável de L3. ADR-0029 pode restringir; se sim, dispatcher precisa estar em crate compartilhada ou L3 implementa version própria reutilizando helpers L1.
- **Defaults Oklab preservam bytes P263/P265/P268** literal — §política condição 4 absoluta. Verificar antes de finalizar com `cargo test p263_ p265_ p268_`.
- **Regressão tests P262-P270 zero** (2500 baseline) — §política condição 7 absoluta.
- **CMYK fallback decisão §A.9** — Fase A define estratégia mínima; pode ser:
  - Arm Oklab no `interpolate_in_space` para CMYK (cristalino converte CMYK→Oklab para interpolar).
  - Panic explícito (CMYK não suportado L3 P270.1; P270.2 fecha).
  - Recomendação: arm Oklab fallback (não-fatal; user ainda consegue PDF mas qualidade idêntica P268 actual).
- **Hue-wrap pré-amostragem** — verificar §política condição 11. Se HSL/Oklch banding visível com N=16, pode precisar adaptive N (paridade P268.2). Decidir conforme empírica Fase A; se requer adaptive, magnitude muda para M (vs M+).
- **Renomeação helpers L3** — `oklab_sample_stops_*` → `multispace_sample_stops_*`. Verificar §política condição 10 (call sites externos).
- **Cap hard L3 400 + testes hard 50** — gate absoluto; estouro dispara §política.
- **Cap soft L3 250 + testes soft 35** — informativo; estouro regista mas continua.
- **Anotações cross-ADR 5 ADRs** — verificar coerência (cada anotação refere ADR-0091 §"Decisão L3 materializada P270.1").
- **Relatório final esperado**: 2500 + 33 = ~2533 testes verdes; hash drift L0; lint zero; ADRs 78 preservado (sem ADR nova).
- **Marco "Cluster Gradient L3 emit 7/8 spaces"** documentado em relatório §1 + ADR-0091 §anotação P270.1.
