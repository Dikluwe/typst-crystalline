# typst-passo-269 — P-Gradient-Focal (activa focal_center + focal_radius Radial; L1+stdlib+PDF)

**Magnitude**: M (cap composto: L1 ≤ 150 LOC + stdlib ≤ 40 LOC + L3 ≤ 60 LOC + 35 testes).
**Cluster**: Visualize / Gradient / Radial (activação de feature).
**Origem**: §11.2 contexto consolidado P-Gradient-Focal pendência candidato #2; ADR-0088 §"variants não materializados" §focal scope-out.
**Tipo**: passo principal P269 (não sub-passo .N). Activação de feature nova: campos `focal_center` + `focal_radius` em Radial passam de scope-out para materializado.
**Sequência**: P264 (Radial L1+stdlib focal=0/center) + P265 (Radial PDF /ShadingType 3) → **P269 (focal_* activado L1+stdlib+PDF)** → próximo passo aberto.
**Estratégia decidida**: utilizador escolheu P269 absorver L1+stdlib+PDF em magnitude M. **Fase A condicional**: se PDF não trivial, §política condição 2 dispara para divisão P269+P270.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L1/stdlib/L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR anotação cumulativa → prompt L0 → fix-hashes → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-gradient-focal-passo-269.md` imutável. **Sétimo consumo directo de fonte** (P262/P264/P267/P268/P268.1/P268.2 + P269 vanilla Radial focal). Pattern análogo P267 diagnóstico Conic vanilla.

3. **ADR-0088 anotação cumulativa P269** — revoga parcialmente §"variants não materializados" §focal_* (sai do scope-out; passa a materializado). **focal_center e focal_radius únicos elementos a revogar**; outras pendências ADR-0088 preservadas literal.

4. **NÃO criar ADR-0091** — anotação cumulativa em ADR-0088 é suficiente; activação não introduz decisão arquitectural nova. Sub-padrão "Anotação cumulativa em vez de ADR nova" **N=6 → N=7 cumulativo**.

5. **ADR-0087/ADR-0089/ADR-0090 preservadas** — Linear/Conic/Type-4-strategy intocados.

6. **ADR-0039 preservado** — TextStyle intocado.

7. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P269 (cluster Gradient extensão Radial focal materializada; ADR-0088 §focal revogado parcialmente).

8. **ADR-0018 preservado** — implementação autónoma; sem dependências externas.

9. **Crystalline-lint zero violations** obrigatório.

10. **Reutilização literal helpers Oklab** P262/P265 (interpolate_oklab, color_to_oklab_with_alpha, oklab_sample_stops_radial) — radial focal reutiliza pipeline existente; **sub-padrão N=4 → N=5 cumulativo**.

11. **Vanilla read-first autorizado** — `lab/typst-original/crates/typst-library/src/visualize/gradient.rs` (RadialGradient focal_center/focal_radius) + `lab/typst-original/crates/typst-pdf/src/paint.rs` (Radial Type 3 com 2-circle shading).

12. **Regressão tests P264/P265 proibida** — todos os tests Radial existentes devem continuar verdes pós-activação focal. Comportamento default `focal=(center, 0)` preservado para call sites sem focal.

---

## §1 — Sub-passo P269.A — Diagnóstico empírico Radial focal

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-gradient-focal-passo-269.md`.

### Comandos exactos a executar

```bash
# 1. Vanilla: RadialGradient shape com focal_center + focal_radius
rg -n "RadialGradient|focal_center|focal_radius" lab/typst-original/crates/typst-library/src/visualize/gradient.rs

# 2. Vanilla: stdlib gradient.radial named args focal_*
rg -n "fn radial|focal" lab/typst-original/crates/typst-library/src/visualize/ | head -20

# 3. Vanilla: PDF Radial emit com 2-circle shading
rg -n "ShadingType.*3|radial|focal" lab/typst-original/crates/typst-pdf/src/paint.rs | head -40
rg -n "/Coords|2-circle|inner_circle|outer_circle" lab/typst-original/crates/typst-pdf/src/ | head -20

# 4. Cristalino L1 actual Radial struct
rg -n "struct Radial|impl Radial|focal" 01_core/src/entities/gradient.rs

# 5. Cristalino stdlib gradient_radial actual
rg -n "native_gradient_radial|gradient.radial|radial" 01_core/src/library/visualize/gradient.rs

# 6. Cristalino L3 emit Radial actual (PDF Type 3; verificar se focal hardcoded)
rg -n "emit.*radial|Radial.*shading|ShadingType.*3|focal" 03_infra/src/export.rs

# 7. Cristalino tests P264/P265 (regressão obrigatória)
rg -n "p264_|p265_" 01_core/src/ 03_infra/src/ | head -20
cargo test -p typst-cristalino-core radial 2>&1 | tail -10
cargo test -p typst-cristalino-infra radial 2>&1 | tail -10

# 8. Cristalino sample/render Radial actual (verificar se focal entra em sample matemática)
rg -n "fn sample.*Radial|Radial::sample|radial.*sample" 01_core/src/entities/gradient.rs
```

### Estrutura do diagnóstico (§A.1 a §A.11)

```
§A.1 Vanilla RadialGradient shape — campos exactos:
     - stops, center, radius, focal_center, focal_radius, space, relative, anti_alias.
§A.2 Vanilla stdlib gradient.radial named args:
     - center: (50%, 50%); radius: 50%; focal_center: Auto (default = center);
       focal_radius: 0% (default).
§A.3 Vanilla PDF Radial Type 3 emit — 2-circle shading dictionary literal:
     - /Coords [x0 y0 r0 x1 y1 r1] onde (x0,y0,r0) = focal_circle e
       (x1,y1,r1) = outer_circle.
§A.4 Cristalino L1 actual Radial struct — campos materializados P264:
     - Determinar se focal_center / focal_radius já existem como `Option<>` ou estão ausentes.
§A.5 Cristalino L1 actual Radial::sample — matemática:
     - Verificar se sample assume `focal=(center,0)` hardcoded ou se já aceita focal arbitrário.
§A.6 Cristalino stdlib gradient_radial actual — named args:
     - Determinar se focal_center/focal_radius já parseados (provavelmente não; P264 scope-out).
§A.7 Cristalino L3 emit Radial actual — /Coords hardcoded?
     - Determinar se /Coords [cx cy 0 cx cy r] (focal trivial) ou estrutura mais complexa.
     - **DECISÃO CENTRAL P269**: se hardcoded, alteração L3 trivial ~10-20 LOC; se estrutural, divisão P269+P270 necessária.
§A.8 Helpers Oklab reutilizáveis cristalinos — assinatura.
§A.9 Gap a fechar — lista literal de itens a materializar L1+stdlib+L3.
§A.10 Cenário detectado:
     - **B1 fecho conceptual** (PDF trivial; absorve P269) — esperado se P265 hardcoded focal.
     - **B2 sub-passos** (PDF estrutural maior) — força divisão P269+P270.
§A.11 Decisão arquitectural — focal materializado conforme vanilla; preservar default `focal=center, focal_radius=0` para compatibilidade P264.
```

### Critério de aceitação Fase A

- §A.7 decisão binária trivial vs estrutural com evidência literal export.rs.
- §A.10 cenário B1 vs B2 declarado; B2 dispara §política condição 2.
- §A.11 confirma estratégia P264 preserved (defaults focal trivial).

---

## §2 — Sub-passo P269.B — Anotação cumulativa ADR-0088 + L0

### B.1 — ADR-0088 anotação cumulativa P269

Adicionar após §"variants não materializados" actualizada:

```
## Anotação cumulativa P269 — Gradient Radial focal_* activado

**Data**: 2026-05-15.
**Motivo**: ADR-0088 §"variants não materializados" listou focal_center +
focal_radius como scope-out P264. P269 revoga parcialmente esse scope-out
— focal_* passa a materializado L1+stdlib+PDF. Demais pendências
ADR-0088 preservadas literal.

**Estratégia materializada**:
- L1: `Radial.focal_center: Axes<Ratio>` (default = center) +
  `Radial.focal_radius: Ratio` (default = 0%).
- Stdlib: `gradient.radial(...)` ganha named args `focal_center` +
  `focal_radius` paridade vanilla.
- L3: PDF `/Coords [focal_x focal_y focal_r center_x center_y center_r]`
  per /ShadingType 3 2-circle shading nativo.

**Defaults preservam comportamento P264**:
- focal_center default = center → cristalino renderiza idêntico a P264
  para call sites sem focal_* arg.
- focal_radius default = 0% → idem.

**Regressão tests P264/P265 zero obrigatória** — defaults preservam
comportamento. Verificado §política condição 9.

**Reutilização literal helpers Oklab** P262/P265 — sub-padrão N=4 → N=5
cumulativo.

**Vanilla validation**: `lab/typst-original/.../visualize/gradient.rs`
RadialGradient §A.1 6+2 campos. Cristalino activa 2 (focal_center +
focal_radius); demais (space custom, relative custom, anti_alias)
preservam scope-out P264.
```

### B.2 — ADR-0054 anotação cumulativa P269

```
P269 — cluster Gradient Radial focal_* materializado L1+stdlib+PDF;
ADR-0088 §focal_* scope-out revogado parcialmente. Perfil graded
DEBT-1 preservado (activação per ADR explícita; defaults preservam P264).
```

### B.3 — L0 prompt `entities/gradient.md` anotação P269

Adicionar à secção Radial:

```
**Anotação P269 — focal_center + focal_radius activados**:
- `Radial.focal_center: Axes<Ratio>` (default = center).
- `Radial.focal_radius: Ratio` (default = 0%).
- Stdlib named args paridade vanilla.
- PDF /Coords 2-circle nativo Type 3.
- ADR-0088 §focal_* scope-out revogado parcialmente.
- Defaults preservam comportamento P264 zero regressão.
```

### B.4 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0 + headers L1/L3 afectados. Zero violations.

---

## §3 — Sub-passo P269.C — Materialização (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. L0 + ADR-0088 + ADR-0054 anotações §2.
3. `crystalline-lint --fix-hashes`.
4. **Testes-primeiro** — adicionar ~30-35 testes ANTES de qualquer LOC L1/stdlib/L3.
5. L1 código — campos focal_* + Radial::sample matemática.
6. Stdlib código — named args parsing.
7. L3 código — /Coords focal real (depende decisão §A.7).
8. Verificação final.

### Cap LOC

- L1: ≤ 150 LOC em `01_core/src/entities/gradient.rs` (struct field + sample matemática focal-aware + construtor + helpers internos se necessário).
- Stdlib: ≤ 40 LOC em `01_core/src/rules/stdlib/gradients.rs` (named args parsing).
- L3: ≤ 60 LOC em `03_infra/src/export.rs` (/Coords alteração) — **condicional Fase A §A.7**.
- Testes: ≤ 35 novos.

### Alteração L1 esperada

```rust
pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,  // P269 — novo campo; default = center
    pub focal_radius: Ratio,        // P269 — novo campo; default = 0%
}

impl Radial {
    pub fn radial(stops, center, radius) -> Self {
        Self {
            stops, center, radius,
            focal_center: center,   // P269 default
            focal_radius: Ratio::zero(),  // P269 default
        }
    }

    pub fn radial_with_focal(stops, center, radius, focal_center, focal_radius) -> Self {
        Self { stops, center, radius, focal_center, focal_radius }
    }

    pub fn sample(&self, t: f32) -> Color {
        // Existente P264; verificar Fase A §A.5 se precisa focal-awareness.
        // Vanilla matemática 2-circle:
        // t = distance(point, focal_center) / distance(outer_circle_intersect, focal_center)
        // ... preserva P264 quando focal=(center, 0).
    }
}
```

### Alteração stdlib esperada

```rust
pub fn native_gradient_radial(args) -> SourceResult<Value> {
    // Existente P264 + 2 named args novos
    let focal_center: Axes<Ratio> = args.named("focal_center")?.unwrap_or(center);
    let focal_radius: Ratio = args.named("focal_radius")?.unwrap_or(Ratio::zero());
    // ... construtor radial_with_focal.
}
```

### Alteração L3 esperada (depende §A.7)

**Cenário trivial** (esperado se §A.7 = hardcoded `[cx cy 0 cx cy r]`):

```rust
// 03_infra/src/export.rs emit_radial — alteração ~10-20 LOC
let coords = format!(
    "[{} {} {} {} {} {}]",
    radial.focal_center.x.to_f32() * w,  // novo
    radial.focal_center.y.to_f32() * h,  // novo
    radial.focal_radius.to_f32() * r,    // novo
    radial.center.x.to_f32() * w,
    radial.center.y.to_f32() * h,
    radial.radius.to_f32() * r,
);
// Resto preservado P265.
```

**Cenário estrutural** (§política condição 2 dispara): divisão P269+P270.

### Estrutura testes esperada

**Unit L1** (10 testes):
- `p269_radial_construcao_default_focal_preserva_p264`: focal=center, focal_radius=0.
- `p269_radial_construcao_focal_explicito`: focal arbitrário.
- `p269_radial_sample_focal_default_paridade_p264`: matemática preservada para defaults.
- `p269_radial_sample_focal_offset`: focal != center; matemática 2-circle.
- `p269_radial_sample_focal_radius_positivo`: focal_radius > 0.
- `p269_radial_sample_t_clamp`: t fora [0,1] preserva clamp.
- `p269_radial_clone_arc_o1`: Arc preservado.
- `p269_radial_partial_eq_focal`: PartialEq compara focal fields.
- `p269_radial_focal_dentro_outer_circle`: validação (vanilla constraint).
- `p269_radial_focal_validacao_erro`: focal_radius ≥ radius → erro.

**Unit stdlib** (5 testes):
- `p269_stdlib_radial_focal_center_named`: gradient.radial(red, blue, focal_center: (30%, 40%)).
- `p269_stdlib_radial_focal_radius_named`: focal_radius: 10%.
- `p269_stdlib_radial_focal_ambos_named`: ambos args.
- `p269_stdlib_radial_focal_defaults_preserva_p264`: sem args focal_* → P264 behavior.
- `p269_stdlib_radial_focal_invalido_erro`: focal_radius > radius erro.

**E2E PDF** (8 testes):
- `p269_export_pdf_radial_focal_coords_real`: /Coords contém focal real.
- `p269_export_pdf_radial_focal_default_preserva_p265`: defaults produzem mesmos bytes P265.
- `p269_export_pdf_radial_focal_dedup_arc_ptr`: dedup Arc preservado.
- `p269_export_pdf_radial_focal_offset_render`: focal offset renderiza correctamente.
- `p269_export_pdf_radial_focal_radius_render`: focal_radius > 0 renderiza.
- `p269_export_pdf_regression_p265_cluster_3_variants`: cluster preservado.
- `p269_export_pdf_radial_focal_oklab_interp`: interpolação Oklab preservada.
- `p269_export_pdf_radial_focal_negative_clamp`: edge case focal fora outer.

**Snapshot determinístico** (5 testes):
- `p269_pdf_bytes_radial_focal_default_reproduzivel`.
- `p269_pdf_bytes_radial_focal_offset_reproduzivel`.
- `p269_pdf_bytes_radial_focal_radius_reproduzivel`.
- `p269_pdf_bytes_cluster_3_variants_pos_focal_reproduzivel`.
- `p269_pdf_bytes_dedup_focal_reproduzivel`.

**Regressão P264/P265** (não novos; verificar verdes):
- Todos os tests p264_* e p265_* devem permanecer verdes literal.

Total esperado: 10 + 5 + 8 + 5 = 28-35 testes (cap 35 respeitado).

---

## §4 — Sub-passo P269.D — Promoção + README + relatório

1. **ADR-0088** anotação cumulativa P269 fechada (§focal revogado parcialmente).
2. **ADR-0054** anotação cumulativa P269 adicionada.
3. **ADR-0090** preservada literal.
4. **README.md** actualizar:
   - Tabela cobertura Visualize (+~2-3pp via focal_* activado).
   - Entrada P269 ~50-60 linhas (paridade entrada P264 — activação de feature L1+stdlib+PDF).
   - Cross-reference ADR-0088 §anotação P269.
   - Cluster Gradient: agora Radial inclui focal_* materializado.
5. **Distribuição ADRs preservada** — total 77 mantido (sem ADR nova).
6. **Relatório** `00_nucleo/materialization/typst-passo-269-relatorio.md`:
   - Métricas finais (esperado 2428 + 28-35 = ~2456-2463).
   - Fase A §A.7 decisão B1/B2 documentada.
   - Diff L1/stdlib/L3 antes/depois.
   - Sub-padrões + N cumulativo.
   - Regressão zero P264/P265.
   - **ADR-0088 §focal revogação parcial** marcada.

---

## §política de paragem

1. **Fase A §A.7 revela cenário B2 estrutural** — PDF L3 alteração maior que 60 LOC ou requer refactor `GradientObjectKind::Radial` para focal-aware. Magnitude P269 estoura; dividir em P269 (L1+stdlib) + P270 (PDF). Parar antes de qualquer LOC L3 e confirmar com utilizador.

2. **Fase A §A.5 revela que `Radial::sample` precisa refactor matemático maior** — sample actual P264 assume `focal=(center, 0)` profundamente; alteração L1 estoura cap 150 LOC. Confirmar antes de continuar.

3. **Helpers Oklab P262/P265 não reutilizáveis literal** — gap > 20 LOC.

4. **Cap LOC L1 (150), stdlib (40) ou L3 (60) ameaça ser ultrapassado**.

5. **Cap testes (35) ameaça ser ultrapassado**.

6. **Snapshot bytes PDF não reproduzíveis** — float não-determinismo em /Coords.

7. **Crystalline-lint reporta violations** após anotações.

8. **ADR-0088 revogação total ameaçada** — alteração toca outros pontos §"variants não materializados" além de focal_*.

9. **Regressão tests P264/P265** — qualquer test antigo falha pós-activação focal_*. §política absoluta.

10. **Cluster Gradient marco quebra** — `p265_export_pdf_cluster_3_variants_coexistem` ou equivalente falha.

11. **Defaults focal não preservam P264 behavior** — call sites P264 sem focal_* produzem bytes diferentes pós-P269. Indica fórmula sample não-paridade.

12. **Vanilla validation falha** — focal_* defaults cristalino divergem de vanilla (cristalino default focal_center=center; vanilla default focal_center=Auto). Confirmar antes de continuar.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P269 | Nota |
|---|---|---|
| Auditoria condicional (ADR-0084) | N=11 | + P269 |
| Diagnóstico imutável (ADR-0085) | N=12 (sétimo consumo de fonte vanilla) | + P269 (P262/P264/P267/P268/P268.1/P268.2 + **P269**) |
| Anotação cumulativa em vez de ADR nova | **N=7** | + P269 anotada ADR-0088 |
| Reutilização literal helpers cross-passos | **N=5** | + P269 (helpers Oklab P262/P265) |
| Auto-aplicação ADR-0065 inline | N=10 | + P269 |
| Refactor cross-cutting entity primitivo | N=4 preservado | Radial já era cross-cutting; activar focal_* não adiciona cross-cutting novo |
| ADR scope-out revogado parcialmente | **N=2 cumulativo** | P267 revogou ADR-0088 §Conic (parcial); **P269 revoga §focal_*** |

### Marco arquitectural P269

**Cluster Gradient Radial extensão completa** — focal_* materializado paridade vanilla. ADR-0088 §"variants não materializados" §focal_* revogado parcialmente; demais pendências (Conic já tratada P267; Linear sem extensões); cluster Gradient agora cobre todas as 3 variants com features principais L1+stdlib+PDF.

**Segunda aplicação do padrão "ADR scope-out revogado parcialmente"** — P267 inaugurou (Conic sai do scope-out §"variants não materializados"); P269 estende (focal_* sai do mesmo scope-out).

### Sequência pós-P269

- **P-Gradient-Space-Custom** (S+; activa `space: ColorSpace` cross-variant; revoga Oklab fixo).
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266 Opção 1).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).

---

## §referências cross-passos

- **P264** — Gradient Radial L1+stdlib (ADR-0088; precedente directo extendido).
- **P265** — PDF Radial /ShadingType 3 (helpers Oklab N=16; template emit).
- **P267** — Gradient Conic L1+stdlib (precedente "ADR scope-out revogado parcialmente").
- **P268** — PDF Conic Type 4 Gouraud (precedente template emit).
- **P268.2** — Refino adaptive N Conic (precedente reutilização helpers Oklab N=4 → N=5).
- ADR-0088 — Gradient Radial-only (anotada cumulativa P269; §focal revogado parcialmente).
- ADR-0054 — Perfil graded (anotada cumulativa P269).
- ADR-0018 — Whitelist crates (preservada).
- ADR-0085 — Diagnóstico imutável (sétimo consumo de fonte vanilla).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.7 é decisão central** — cenário B1 vs B2 determina se L3 está dentro do cap 60 LOC ou se passo divide. Se ambíguo, §política condição 1 dispara antes de qualquer LOC L3.
- **Defaults focal preservam P264** — `focal_center=center, focal_radius=0` obrigatório; verificar §política condição 11.
- **Regressão tests P264/P265 zero** — verificar com `cargo test p264_ p265_` antes de finalizar. §política condição 9.
- **Vanilla validation §A.1** — `RadialGradient` cristalino activa 2 dos 6+2 campos vanilla; demais preservam scope-out P264.
- **Snapshot bytes determinísticos** — focal coordinates podem introduzir float não-determinismo; verificar com snapshot tests.
- **ADR-0088 §"variants não materializados"** edição literal: focal_center + focal_radius riscados (struck-through); demais (Conic já tratada; outras) preservados.
- **Relatório final esperado**: 2428 + 28-35 = ~2456-2463 testes verdes; hash drift L0 + headers L1/L3; lint zero; ADRs 77 preservado.
- **Marco "Cluster Gradient Radial extensão focal_* completa"** documentado em relatório §1 + ADR-0088 §anotação P269.
