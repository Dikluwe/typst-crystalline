# typst-passo-270.2 — L3 emit CMYK directo `/DeviceCMYK` (fecha cluster Gradient L3 8/8 spaces)

**Magnitude**: S+ (cap composto: L3 hard ≤ 250 LOC + testes hard ≤ 35; testes soft ≤ 25).
**Cluster**: Visualize / Gradient / PDF export (refino L3 — fecha cluster).
**Tipo**: sub-passo .2 da série P270. Pattern análogo P268.2/P270.1.
**Origem**: ADR-0091 §"Decisão L3 futura" P270.2; relatório P270.1 §8 pendência reservada.
**Sequência**: P270 (L1+stdlib) → P270.1 (L3 7 spaces RGB-family + perceptual) → **P270.2 (L3 CMYK directo `/DeviceCMYK`; fecha cluster 8/8)** → cluster Gradient feature-complete.
**Estratégia decidida**: pesquisa industry preventiva consolidou achados (bug #4422 causa raiz = dictionary errado; pdfkit #532 análogo; ICC profiles scope-out preserved); ADR-0083 §"DeviceCMYK PDF" **revogação final** via anotação cumulativa.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0091 anotação cumulativa + ADR-0083 anotação cumulativa final → prompt L0 → fix-hashes → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-l3-cmyk-passo-270-2.md` imutável. **Décimo consumo directo de fonte** (P262/P264/P267/P268/P268.1/P268.2/P269/P270/P270.1 + **P270.2 vanilla CMYK emit literal + pdf-writer Function 4-component**).

3. **ADR-0091 anotação cumulativa P270.2** — fecha §"Decisão L3 (materializada P270.1+P270.2)"; cluster Gradient L3 emit feature-complete 8/8. Sub-padrão "Anotação cumulativa em vez de ADR nova" **N=8 → N=9 cumulativo**.

4. **ADR-0083 anotação cumulativa P270.2 (revogação final)** — §"DeviceCMYK PDF" scope-out **revogado final**. Última anotação cumulativa da série P270.x. Sub-padrão "ADR scope-out revogado parcialmente" **N=3 → N=4 cumulativo** (P267 Conic + P269 focal_* + P270 ColorSpace + **P270.2 DeviceCMYK**). **Limiar formalização clara N=4 atingido** — candidato meta-ADR formalização futura.

5. **ADR-0087/ADR-0088/ADR-0089/ADR-0090 preservadas literal** — variant strategies intocadas. Anotação cumulativa em cada uma (P270.2 CMYK emit branch adicionado).

6. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P270.2 (cluster Gradient L3 emit feature-complete 8/8; perfil graded DEBT-1 final do cluster).

7. **ADR-0018 preservado** — implementação autónoma; sem dependências externas (ICC profiles scope-out preserved).

8. **ADR-0039 preservado** — TextStyle intocado.

9. **Crystalline-lint zero violations** obrigatório.

10. **Reutilização literal**:
    - `interpolate_in_space` (L1 dispatcher P270) arm CMYK preservado.
    - 3 helpers L3 `multispace_sample_stops_*` P270.1 templates expandidos com CMYK branch.
    - Sub-padrão "Reutilização literal helpers cross-passos" **N=7 → N=8 cumulativo**.

11. **Vanilla read-first autorizado** — `lab/typst-original/crates/typst-pdf/src/` CMYK emit + `lab/krilla-reference/` CMYK ICC suporte (referência apenas; cristalino não usa ICC).

12. **Regressão tests P262-P270.1 proibida** — 2533 baseline preservado. Defaults `space != Cmyk` produzem bytes bit-exact idênticos a P270.1 (CMYK branch novo, não altera outros).

13. **Cap LOC hard vs soft explícito** (P270.1 inaugural; P270.2 segunda aplicação):
    - **Cap hard L3**: 250 LOC. Estouro dispara §política condição 4.
    - **Cap soft L3**: 150 LOC. Estouro regista relatório.
    - **Cap hard testes**: 35.
    - **Cap soft testes**: 25.

14. **ICC profiles scope-out preserved** — cristalino emit `/DeviceCMYK` directo sem ICC. Refino futuro candidato P-Gradient-CMYK-ICC (paridade krilla custom ICC profiles).

15. **Conic CMYK Type 4 Gouraud — pendência condicional** — Fase A confirma se readers universalmente suportam Type 4 com `/ColorSpace /DeviceCMYK` (PDF spec permite mas suporte reader pode variar). Se Fase A revelar problemas, scope-out preserved e Conic CMYK fica candidato futuro; Linear+Radial CMYK materializados P270.2.

---

## §1 — Sub-passo P270.2.A — Diagnóstico empírico CMYK pipeline + vanilla

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-l3-cmyk-passo-270-2.md`.

### Comandos exactos a executar

```bash
# 1. Cristalino L1 Color::to_cmyk — verificar API disponível P257
rg -n "to_cmyk|cmyk_components|Cmyk\(" 01_core/src/entities/color.rs | head -20

# 2. Cristalino L3 actual emit (P270.1 com multispace) — onde CMYK actualmente passa
rg -n "multispace_sample_stops|to_rgba_f32|DeviceRGB" 03_infra/src/export.rs | head -30

# 3. Cristalino Function/Shading dictionary literal — verificar onde /ColorSpace está hardcoded
rg -n "/ColorSpace|DeviceRGB|Decode|Range" 03_infra/src/export.rs | head -30

# 4. Cristalino L1 interpolate_cmyk — verificar arm CMYK do dispatcher P270
rg -n "interpolate_cmyk|interpolate_in_space" 01_core/src/entities/gradient.rs | head -20

# 5. Vanilla typst CMYK emit — ShadingType + ColorSpace dictionary literal
rg -n "DeviceCMYK|cmyk|Cmyk" lab/typst-original/crates/typst-pdf/src/ | head -40

# 6. Vanilla typst Function 4-component output range
rg -n "Range.*\[.*0.*1.*0.*1.*0.*1.*0.*1\]|4.*output|N.*4" lab/typst-original/crates/typst-pdf/src/ | head -20

# 7. Krilla reference CMYK suporte (lab/krilla-reference/ se disponível)
ls lab/krilla-reference/ 2>/dev/null && rg -n "CMYK|cmyk" lab/krilla-reference/src/ | head -20 || echo "krilla-reference não disponível; pular"

# 8. Tests P262-P270.1 baseline (regressão obrigatória)
cargo test -p typst-cristalino-infra gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.13)

```
§A.1 Cristalino Color::to_cmyk API:
     - Confirmar `Color::to_cmyk_f32() -> (f32, f32, f32, f32)` existe (P257).
     - Se não existe, gap LOC adicionar (~10-20 LOC em color.rs).
§A.2 Cristalino L3 actual CMYK comportamento (pré-P270.2):
     - Stops CMYK passam por `to_rgba_f32()` (P270.1 default).
     - CMYK fica convertido para sRGB no emit — divergência observable de
       vanilla user-intent.
§A.3 Cristalino dispatcher interpolate_in_space arm Cmyk:
     - Confirmar interpolate_cmyk existe (P270).
     - Confirmar interpola em CMYK (4 componentes) em vez de converter Oklab.
§A.4 Vanilla typst CMYK emit literal:
     - /ColorSpace /DeviceCMYK no shading dictionary.
     - /Function FunctionType 2 (Linear/Radial) com N=4 outputs.
     - /Range [0 1 0 1 0 1 0 1] (8 values; 4 pares).
     - C0/C1 com 4 valores [c m y k].
     - Pre-amostragem usa CMYK directo (não pré-convertido para RGB).
§A.5 Vanilla Conic CMYK comportamento:
     - Verificar se vanilla materializa Conic CMYK (krilla pode ou não suportar).
     - PDF spec /ShadingType 4 com /ColorSpace /DeviceCMYK: permitido mas suporte
       reader pode variar.
§A.6 PROPOSTA L3 estrutura CMYK branch:
     - `multispace_sample_stops_<variant>` ganha branch CMYK:
       - Se variant.space == Cmyk: amostra CMYK directo (4 components per amostra).
       - Senão: pipeline P270.1 (RGB output via to_rgba_f32).
     - `emit_gradient_objects` ganha dispatcher dual:
       - Cmyk: emit shading com /ColorSpace /DeviceCMYK + Function 4-component.
       - Não-CMYK: pipeline P270.1 preservado literal.
§A.7 Bug #4422 e pdfkit #532 — causa raiz factual confirmada:
     - Dictionary /ColorSpace errado (DeviceRGB em vez de DeviceCMYK).
     - Cristalino implementação correcta resolve o bug por construção.
§A.8 Decisão Conic CMYK Type 4 Gouraud:
     - Cenário A: materializar (vertex bytes 4 vs 3; stream binary muda;
       /ColorSpace /DeviceCMYK).
     - Cenário B: scope-out preserved (Conic CMYK fica candidato futuro;
       Linear+Radial materializados P270.2).
     - Decisão baseada em §A.5 vanilla suporte + testar visualmente PDF reader.
§A.9 Estimativa cap LOC:
     - Linear CMYK branch: ~25-30 LOC.
     - Radial CMYK branch: ~25-30 LOC.
     - Conic CMYK branch (se Cenário A): ~30-40 LOC.
     - Dispatcher emit: ~10-15 LOC.
     - Helpers extract CMYK if needed: ~10-20 LOC.
     - Tests: ~100-130 LOC.
     - **Total L3 production**: ~80-130 LOC se Cenário A; ~60-90 LOC se Cenário B.
     - Cap hard 250 com folga ~50-70%.
§A.10 Defaults preservam P270.1:
     - space != Cmyk: pipeline P270.1 literal.
     - space == Cmyk: branch novo.
     - 2533 baseline preserved (verificar empíricamente).
§A.11 Cenário detectado:
     - **B1 fecho conceptual** (Linear+Radial+Conic CMYK materializados; cluster 8/8).
     - **B2 sub-passos** (Conic CMYK scope-out preserved; cluster 7+0.66/8).
§A.12 Decisão arquitectural — DeviceCMYK directo sem ICC profiles
     (ADR-0091 §scope-out preserved).
§A.13 ADR-0083 §DeviceCMYK revogação final preview.
```

### Critério de aceitação Fase A

- §A.1 confirma Color::to_cmyk_f32 disponível ou identifica gap.
- §A.3 confirma interpolate_cmyk arm dispatcher P270 funcional.
- §A.7 confirma causa raiz bug #4422 (dictionary errado) — cristalino resolve por construção.
- §A.8 decide Cenário A vs B Conic CMYK; §A.11 confirma B1 ou B2.
- §A.10 confirma defaults preservam P270.1 bit-exact.

---

## §2 — Sub-passo P270.2.B — Anotações cumulativas (fecho série P270)

### B.1 — ADR-0091 anotação cumulativa P270.2 (fecho L3)

Renomear secção existente: §"Decisão L3 (materializada P270.1)" → §"Decisão L3 (materializada P270.1+P270.2)".

Adicionar após anotação P270.1:

```
## Anotação cumulativa P270.2 — L3 emit CMYK directo (fecha cluster L3 8/8)

**Data**: 2026-05-17.
**Motivo**: ADR-0091 §"Decisão L3 futura" P270.2 — CMYK directo
materializado via /ColorSpace /DeviceCMYK + Function 4-component
output. Cluster Gradient L3 emit feature-complete 8/8 spaces.

**Estratégia materializada**:
- `multispace_sample_stops_<variant>` ganha CMYK branch:
  - space == Cmyk: amostra 4 componentes CMYK directo via
    interpolate_in_space arm Cmyk (P270 dispatcher).
  - Senão: pipeline P270.1 RGB output literal.
- `emit_gradient_objects` dispatcher dual:
  - Cmyk: shading dictionary com /ColorSpace /DeviceCMYK,
    Function /Range [0 1 0 1 0 1 0 1], 4-component C0/C1.
  - Não-CMYK: pipeline P270.1 preservado literal.

**Conic CMYK status**: [B1 materializado | B2 scope-out preserved]
conforme §A.8/§A.11 diagnóstico.

**Bug #4422 resolvido por construção**: cristalino emit /ColorSpace
/DeviceCMYK correcto (vs vanilla bug dictionary errado). pdfkit #532
análogo confirma causa raiz universal.

**ICC profiles scope-out preserved**: cristalino emit DeviceCMYK
directo sem ICC. Vanilla via krilla suporta CMYK custom ICC
profiles (referência); cristalino refino futuro candidato
P-Gradient-CMYK-ICC.

**Helpers reutilizados literal**:
- `interpolate_in_space` arm Cmyk (P270 dispatcher).
- 3 helpers L3 `multispace_sample_stops_*` (P270.1 templates).
- Sub-padrão "Reutilização literal helpers cross-passos" N=7 → N=8.

**Defaults preservam P270.1**: space != Cmyk não-altered; 2533
baseline preserved verificado §política condição 4.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=8 → N=9
cumulativo.

**Sub-padrão "ADR scope-out revogado parcialmente"** atinge **N=4
cumulativo limiar formalização clara** (P267 + P269 + P270 + P270.2).
Candidato meta-ADR futura paridade P260 ADR-0084/0085.

**Cluster Gradient L3 emit feature-complete 8/8 spaces** — paridade
vanilla user-facing total. ADR-0083 §"DeviceCMYK PDF" revogado final.
```

### B.2 — ADR-0083 anotação cumulativa P270.2 (revogação final)

Adicionar após §"Anotação cumulativa P270" existente:

```
## Anotação cumulativa P270.2 — DeviceCMYK PDF revogação final

**Data**: 2026-05-17.
**Motivo**: P270.2 materializa L3 emit `/ColorSpace /DeviceCMYK`
directo. §"DeviceCMYK PDF" scope-out revogado final.

**Scope-outs ADR-0083 status pós-P270.2**:
- ~~ColorSpace runtime~~: revogado P270 (anotação cumulativa).
- ~~DeviceCMYK PDF~~: revogado P270.2 (esta anotação).
- Operadores cor: preserved scope-out.
- Constantes nomeadas extras: preserved scope-out.

ADR-0083 perfil graded DEBT-1 §"Color paridade vanilla 8/8 spaces"
agora cobre L1 + L3 PDF emit em 8/8 spaces (P257 L1 + P270.1+P270.2 L3).

**Sub-padrão "ADR scope-out revogado parcialmente" N=4 cumulativo
limiar formalização clara**. Ver ADR-0091 §"Anotação cumulativa
P270.2".
```

### B.3 — ADR-0087/0088/0089/0090 anotações cumulativas P270.2

Cada uma recebe anotação curta:

```
## Anotação cumulativa P270.2 — CMYK emit branch

Variant L3 emit ganha CMYK branch via dispatcher dual em
`emit_gradient_objects`. space == Cmyk: shading
/ColorSpace /DeviceCMYK + Function 4-component. Não-CMYK:
P270.1 pipeline preserved. Cluster L3 emit feature-complete 8/8.
Ver ADR-0091 §"Anotação cumulativa P270.2".
```

ADR-0089 (Conic) recebe anotação adicional sobre cenário Conic CMYK:

```
## Anotação cumulativa P270.2 — Conic CMYK status

Conic Type 4 Gouraud CMYK status conforme §A.8/§A.11 diagnóstico
P270.2:
- Cenário A: materializado (stream binary 4 bytes per vertex;
  /ColorSpace /DeviceCMYK; preserved P268.2 adaptive N).
- Cenário B: scope-out preserved (Conic CMYK candidato futuro
  P-Gradient-Conic-CMYK; Linear+Radial materializados P270.2).
```

### B.4 — ADR-0054 anotação cumulativa P270.2

```
P270.2 — cluster Gradient L3 emit feature-complete 8/8 spaces;
ADR-0083 §DeviceCMYK revogado final; perfil graded DEBT-1
preservado.
```

### B.5 — L0 `entities/gradient.md` anotação P270.2

Adicionar após anotação P270.1:

```
**Anotação P270.2**: CMYK emit branch directo /DeviceCMYK
materializado em L3; pipeline dual em emit_gradient_objects
(space == Cmyk: 4-component DeviceCMYK; senão P270.1 literal).
Conic CMYK status [§A.8 diagnóstico]. ICC profiles scope-out
preserved. ADR-0083 §DeviceCMYK revogado final. Cluster L3 emit
feature-complete 8/8 spaces.
```

### B.6 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0. Zero violations.

---

## §3 — Sub-passo P270.2.C — Materialização L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 + ADR-0083 + ADR-0087/0088/0089/0090 + ADR-0054 anotações §2.
3. L0 anotação §2.B.5.
4. `crystalline-lint --fix-hashes`.
5. **Testes-primeiro** — adicionar ~15-25 testes ANTES de qualquer LOC L3.
6. L3 código — `multispace_sample_stops_*` CMYK branches + dispatcher dual + Function 4-component.
7. Verificação final.

### Cap LOC

- **L3 hard**: 250 LOC. Estouro dispara §política condição 4.
- **L3 soft**: 150 LOC. Estouro regista relatório.
- **Testes hard**: 35.
- **Testes soft**: 25.

### Alteração L3 esperada (Cenário A — Conic CMYK materializado)

```rust
// 03_infra/src/export.rs

// 1. Helpers samplers CMYK branch
fn multispace_sample_stops_linear_cmyk(
    linear: &Linear,
    n: usize,
) -> Vec<(f32, f32, f32, f32)> {
    let n = n.max(2);
    (0..n).map(|i| {
        let t = i as f32 / (n - 1) as f32;
        let c = linear.sample(t);  // P270 dispatcher arm Cmyk
        let (cy, m, y, k) = c.to_cmyk_f32();
        (cy.clamp(0.0, 1.0), m.clamp(0.0, 1.0), y.clamp(0.0, 1.0), k.clamp(0.0, 1.0))
    }).collect()
}

// Análogo multispace_sample_stops_radial_cmyk + multispace_sample_stops_conic_cmyk

// 2. Dispatcher dual em emit_gradient_objects
fn emit_linear_branch(linear: &Linear, ...) {
    if linear.space == ColorSpace::Cmyk {
        emit_linear_cmyk(linear, ...);  // novo branch P270.2
    } else {
        emit_linear_rgb(linear, ...);  // P270.1 pipeline preserved literal
    }
}

// 3. emit_linear_cmyk — shading dictionary CMYK
fn emit_linear_cmyk(linear: &Linear, ...) {
    let stops_cmyk = multispace_sample_stops_linear_cmyk(linear, 16);
    
    // Function FunctionType 2 ou 3 (stitching) — 4 outputs
    // /Range [0 1 0 1 0 1 0 1]
    // C0 [c m y k] vs P270.1 [r g b]
    
    let function = format!(
        "<< /FunctionType 3 /Domain [0 1] /Bounds [...] \
           /Encode [...] /Functions [...] >>"
    );
    
    let shading = format!(
        "<< /ShadingType 2 \
           /ColorSpace /DeviceCMYK \
           /Coords [{x0} {y0} {x1} {y1}] \
           /Function {func_id} 0 R >>"
    );
    
    // ...
}

// Análogo emit_radial_cmyk + emit_conic_cmyk (se Cenário A).
```

### Alteração Conic CMYK (Cenário A; condicional §A.8)

```rust
fn emit_conic_cmyk(conic: &Conic, ...) {
    let n = compute_adaptive_n_conic(conic);  // P268.2 preserved
    let samples_cmyk = multispace_sample_stops_conic_cmyk(conic, n);
    
    // Stream binary: 4 bytes per vertex em vez de 3
    let mut stream = Vec::new();
    for vertex in vertices {
        stream.push(flag);
        stream.push(x_coord);
        stream.push(y_coord);
        stream.push(c_byte);
        stream.push(m_byte);
        stream.push(y_byte);
        stream.push(k_byte);
        // Total 7 bytes per vertex (vs 6 P268 RGB)
    }
    
    let shading = format!(
        "<< /ShadingType 4 \
           /ColorSpace /DeviceCMYK \
           /BitsPerCoordinate 8 \
           /BitsPerComponent 8 \
           /BitsPerFlag 8 \
           /Decode [0 1 0 1 0 1 0 1 0 1] >>"  // 5 pares (x, y, c, m, y, k)
    );
}
```

### Estrutura testes esperada

**Unit pré-amostragem CMYK** (6 tests; 3 variants × 2 cenários):
- `p270_2_linear_sample_cmyk_2_stops`: amostragem 4-component.
- `p270_2_radial_sample_cmyk_2_stops`.
- `p270_2_conic_sample_cmyk_adaptive_n` (Cenário A) OU `p270_2_conic_cmyk_scope_out` (Cenário B).
- `p270_2_linear_cmyk_preserva_p270_1_default_oklab`: defaults bit-exact.
- `p270_2_radial_cmyk_preserva_p270_1_default_oklab`.
- `p270_2_conic_cmyk_preserva_p270_1_default_oklab`.

**E2E PDF dispatcher dual** (5 tests):
- `p270_2_export_pdf_linear_cmyk_shading_devicecmyk`: confirma `/ColorSpace /DeviceCMYK`.
- `p270_2_export_pdf_radial_cmyk_shading_devicecmyk`.
- `p270_2_export_pdf_linear_oklab_preserva_devicergb`: regressão P270.1.
- `p270_2_export_pdf_cluster_3_variants_cmyk_coexistem`: Cenário A.
- `p270_2_export_pdf_function_4_output_range`: `/Range [0 1 0 1 0 1 0 1]`.

**Snapshot determinístico** (3 tests):
- `p270_2_pdf_bytes_linear_cmyk_reproduziveis`.
- `p270_2_pdf_bytes_radial_cmyk_reproduziveis`.
- `p270_2_pdf_bytes_conic_cmyk_reproduziveis` (Cenário A) OU skip (Cenário B).

**Bug #4422 resolução** (1 test):
- `p270_2_export_pdf_cmyk_resolve_bug_4422_dictionary`: confirma `/ColorSpace /DeviceCMYK` (não DeviceRGB).

**Regressão P262-P270.1** (não novos; verificar verdes):
- 2533 baseline preserved literal.

Total esperado:
- Cenário A: 6 + 5 + 3 + 1 = **15 testes**.
- Cenário B: 5 + 4 + 2 + 1 = **12 testes**.

Cap soft 25; cap hard 35. Folga grande em ambos cenários.

---

## §4 — Sub-passo P270.2.D — Fecho cluster + README + relatório

1. **ADR-0091** anotação cumulativa P270.2 (cluster L3 8/8 fechado).
2. **ADR-0083** anotação cumulativa P270.2 (§DeviceCMYK revogação final; perfil graded final).
3. **ADR-0087/0088/0089/0090** anotações cumulativas P270.2 adicionadas.
4. **ADR-0054** anotação cumulativa P270.2 adicionada.
5. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (+~1-2pp via CMYK L3 materializado; cluster 8/8 fechado).
   - Entrada P270.2 ~50-70 linhas (fecho da série P270; 7 anotações cumulativas).
   - Cross-reference ADR-0091 §"Anotação cumulativa P270.2".
   - **Cluster Gradient L1+stdlib+L3 emit feature-complete 8/8 spaces** — marco arquitectural máximo.
   - ADR-0083 §DeviceCMYK revogado final marcado.
6. **Distribuição ADRs preservada** — total 78 mantido (sem ADR nova; só anotações cumulativas).
7. **Relatório** `00_nucleo/materialization/typst-passo-270-2-relatorio.md`:
   - Métricas finais (esperado 2533 + 12-15 = ~2545-2548).
   - Fase A §A.8 decisão Conic CMYK Cenário A/B documentada.
   - Bug #4422 resolução por construção marcada.
   - Diff helpers L3 antes/depois.
   - Sub-padrões + N cumulativo (4 cumulativos atingem limiar formalização clara).
   - Regressão zero 2533 baseline preservado.
   - **Cluster Gradient L3 emit feature-complete 8/8 spaces** — marco arquitectural máximo.
   - **ADR-0083 §DeviceCMYK revogação final** — última pendência cluster Color resolvida.

---

## §política de paragem

1. **§A.1 Color::to_cmyk_f32 não existe ou gap > 30 LOC** — adicionar em color.rs L1 pode invadir escopo P270.2. Confirmar antes de continuar.

2. **§A.3 interpolate_cmyk arm dispatcher P270 não funcional** — confirmar antes; pode requerer adicionar arm (gap ~10-15 LOC L1).

3. **§A.5 vanilla Conic CMYK suporte ambíguo** — vanilla pode não materializar; cristalino decisão local sobre Cenário A vs B.

4. **Cap LOC L3 hard (250) ameaça ser ultrapassado** — refactor maior que estimativa §A.9 (~80-130 LOC). Confirmar antes.

5. **Cap testes hard (35) ameaça ser ultrapassado** — > 35 testes indica scope creep.

6. **§A.7 verificação falha**: cristalino emit `/ColorSpace /DeviceRGB` em vez de `/DeviceCMYK` — indica pipeline pré-converte CMYK para RGB. §política absoluta.

7. **Snapshot bytes PDF não reproduzíveis** — float não-determinismo CMYK conversões.

8. **Crystalline-lint reporta violations** após anotações.

9. **Regressão tests P262-P270.1** — qualquer test anterior falha. §política absoluta.

10. **Conic CMYK Type 4 Gouraud não renderiza em readers principais** — Fase A testa empíricamente com pdftoppm/mupdf. Se falha consistente, Cenário B (scope-out preserved).

11. **PDF reader compatibility issues** — bug #4422 vanilla pode ter sintomas residuais em readers específicos (não relacionados ao dictionary). Documentar mas não bloquear.

12. **ICC profile requirement** — se PDF/A compliance for verificado em E2E test, DeviceCMYK directo pode falhar PDF/A; documentar como pendência futura P-Gradient-CMYK-ICC.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P270.2 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=9** | + P270.2 anotada ADR-0091 (P258.B/P259.B/P263/P265/P268/P268.2/P270/P270.1/**P270.2**) |
| **ADR scope-out revogado parcialmente** | **N=4 cumulativo (limiar formalização atingido)** | + P270.2 (P267 + P269 + P270 + **P270.2**) — candidato meta-ADR futura |
| Reutilização literal helpers cross-passos | **N=8** | + P270.2 (dispatcher P270 arm Cmyk + helpers L3 P270.1 templates) |
| Diagnóstico imutável (décimo consumo) | **N=15** | + P270.2 (vanilla CMYK emit literal + pdf-writer Function 4-component) |
| Auditoria condicional (ADR-0084) | **N=14** | + P270.2 |
| Auto-aplicação ADR-0065 inline | **N=13** | + P270.2 |
| Cap LOC hard vs soft explícito | **N=2 cumulativo** | P270.1 inaugurou; **P270.2 estende** (segunda aplicação consolida pattern) |
| Anotação cumulativa cross-ADR | **N=3 cumulativo** | P270 (6 ADRs) + P270.1 (6 ADRs) + **P270.2 (6 ADRs)** — terceira aplicação consolida |
| Fase A com industry research proactiva | **N=2 cumulativo** | P270 inaugurou; **P270.2 segunda aplicação** (pesquisa CMYK pré-spec; bug #4422 causa raiz identificada) |

### Marco arquitectural máximo P270.2

**Cluster Gradient L1+stdlib+L3 emit feature-complete 8/8 spaces** — paridade vanilla user-facing total para `gradient.linear/radial/conic(red, blue, space: <8 spaces>)`. Cluster cristalino agora cobre:
- 3 variants (Linear, Radial, Conic).
- 8 spaces (Oklab, Oklch, sRGB, LinearRGB, Luma, HSL, HSV, CMYK).
- 24 combinações user-facing total.
- L1 sample + stdlib named args + L3 PDF emit.

**ADR-0083 §"DeviceCMYK PDF" revogação final** — última pendência cluster Color (P257) resolvida. Perfil graded DEBT-1 §"Color paridade vanilla" agora cobre 8/8 spaces em L1+L3.

**Sub-padrão "ADR scope-out revogado parcialmente" N=4 cumulativo limiar formalização clara** — candidato meta-ADR futura paridade P260 ADR-0084/0085 que formalizaram outros sub-padrões em N=5-6.

**Sub-padrão "Fase A com industry research proactiva" N=2 cumulativo** — segunda aplicação preventiva pré-spec (P270 inaugural; P270.2 estende). Atinge limiar formalização N=2-3 — candidato meta-ADR junto com "ADR scope-out revogado parcialmente".

### Sequência pós-P270.2

Cluster Gradient resolvido a nível user-facing. Próximos candidatos:

- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo` cross-variant).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266 Opção 1).
- **P-Footnote-N** (M; Model pendência P258).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).
- **P-Gradient-CMYK-ICC** (candidato refino futuro; krilla paridade custom ICC profiles).
- **P-Gradient-Adaptive-Multispace** (candidato refino futuro; N adaptive HSL/Oklch hue diff alto).
- **P-Gradient-Conic-CMYK** (candidato condicional se P270.2 Cenário B; activa Conic CMYK adiada).

---

## §referências cross-passos

- **P270** — Gradient ColorSpace runtime L1+stdlib (ADR-0091; precedente directo).
- **P270.1** — L3 emit 7 spaces RGB-family + perceptual (precedente directo refinado por este passo).
- **P262/P264/P267** — Variant L1+stdlib (preservados).
- **P263/P265/P268** — L3 emit templates (preservados; CMYK branch aditivo).
- **P268.2** — Adaptive N hybrid Conic (preservado; CMYK Cenário A reutiliza adaptive N).
- **P269** — Radial focal_* (preservado; campo space aditivo cross-variant).
- **P257** — Color 8/8 spaces (ADR-0083; §DeviceCMYK revogado final P270.2).
- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa P270.2 final).
- ADR-0083 — Color paridade (§DeviceCMYK revogação final P270.2).
- ADR-0087/0088/0089/0090 — Variant strategies (anotadas cumulativa P270.2).
- ADR-0054 — Perfil graded (anotada cumulativa P270.2 final).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.1 + §A.3 críticas** — confirmar Color::to_cmyk_f32 e interpolate_cmyk arm disponíveis antes de continuar. Se gap > 30 LOC L1, §política condições 1+2 disparam.
- **§A.7 causa raiz bug #4422** — verificar literal vs cristalino pipeline; cristalino implementação correcta por construção (`/ColorSpace /DeviceCMYK`).
- **§A.8 decisão Conic CMYK** — Cenário A (materializado) ou Cenário B (scope-out preserved); Fase A teste empírico com pdftoppm/mupdf decide.
- **Defaults space != Cmyk preservam P270.1 bytes** literal — §política condição 4 absoluta. Verificar com `cargo test p270_1_`.
- **Regressão tests P262-P270.1 zero** (2533 baseline) — §política condição 9 absoluta.
- **Cap hard L3 250 + testes hard 35** — gate absoluto; estouro dispara §política.
- **Cap soft L3 150 + testes soft 25** — informativo; estouro regista mas continua.
- **Anotações cross-ADR 6 ADRs** — verificar coerência cada anotação refere ADR-0091 §"Anotação cumulativa P270.2".
- **ICC profiles scope-out preserved** — cristalino DeviceCMYK directo sem ICC; refino futuro candidato P-Gradient-CMYK-ICC.
- **Relatório final esperado**: 2533 + 12-15 = ~2545-2548 testes verdes; hash drift L0; lint zero; ADRs 78 preservado (sem ADR nova; 6 anotações cumulativas).
- **Marco máximo "Cluster Gradient L1+stdlib+L3 emit feature-complete 8/8 spaces"** documentado em relatório §1 + ADR-0091 §"Anotação cumulativa P270.2" + ADR-0083 §"Anotação cumulativa P270.2 revogação final".
