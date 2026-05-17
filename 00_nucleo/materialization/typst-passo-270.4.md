# typst-passo-270.4 — Coons CMYK activação opt-in flag ON (fecha cluster Gradient L3 24/24 absoluto)

**Magnitude**: S (cap composto: L3 hard ≤ 200 LOC + testes hard ≤ 18; cap soft L3 120 / testes 12).
**Cluster**: Visualize / Gradient / PDF export (refino L3 activação CMYK).
**Tipo**: sub-passo .4 da série P270 (fecho cluster). Pattern análogo P270.2/P270.3.
**Origem**: ADR-0092 §"Decisão Cenário A revisado" P270.4 activação; relatório P270.3 §8 pendência reservada.
**Sequência**: P270.3 (Coons RGB infra-estrutura; helpers `#[allow(dead_code)]`) → **P270.4 (Coons CMYK activado; opt-in flag ON; cluster 24/24)** → cluster Gradient L3 feature-complete absoluto.
**Estratégia decidida**: utilizador escolheu activação directa após P270.3. Decisão sub-decisão anterior "adaptive N CMYK recalibrar factor_delta" **não se aplica a Coons** — Coons usa 1 patch per stop (N = stops.len()), não Gouraud adaptive N. Sub-decisão preservada como reserva para passos futuros refino qualidade visual (P-Gradient-Adaptive-Multispace candidato).

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR-0092 anotação cumulativa + ADR-0091 revogação final → prompt L0 → fix-hashes → testes-primeiro → código.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-conic-coons-cmyk-passo-270-4.md` imutável. **Décimo segundo consumo directo de fonte** (P262-P270.3 + **P270.4 verificação ADR-0092 estrutura + vanilla CMYK 4-component emit + ISO 32000-1 §7.5.7.4 Coons CMYK extension**).

3. **ADR-0092 anotação cumulativa P270.4 (fecho preparação)** — opt-in flag activado para `space == Cmyk`; cluster L3 24/24 absoluto. Sub-padrão "Anotação cumulativa em vez de ADR nova" **N=9 → N=10 cumulativo**.

4. **ADR-0091 anotação cumulativa P270.4 (revogação final §"Conic CMYK scope-out preserved")** — §scope-out P270.2 Cenário B revogado **definitivamente** por activação Coons CMYK. Cluster Gradient L3 emit feature-complete 24/24.

5. **ADR-0083 anotação cumulativa P270.4 (revogação final §DeviceCMYK)** — anotação P270.2 revogou parcialmente Linear+Radial; P270.4 estende para Conic; §DeviceCMYK PDF revogado **definitivamente**. Sub-padrão "ADR scope-out revogado parcialmente" **N=5 → N=6 cumulativo** (P267 + P269 + P270 + P270.2 + P270.3 + **P270.4**) — limiar formalização clara **ainda mais ultrapassado**; candidato meta-ADR URGENTE.

6. **ADR-0090 preservada literal** — Type 4 Gouraud RGB intocado.

7. **ADR-0089 anotação cumulativa P270.4** — Conic 2 emit paths agora ambos activos (Type 4 RGB + Type 6 CMYK).

8. **ADR-0087/ADR-0088 preservadas literal** — Linear/Radial intocados.

9. **ADR-0054 anotação cumulativa P270.4 (fecho cluster)** — cluster Gradient L3 emit feature-complete 24/24 absoluto. Perfil graded DEBT-1 final do cluster.

10. **ADR-0018 preservado** — implementação autónoma; sem dependências externas (ICC profiles scope-out preserved candidato P-Gradient-CMYK-ICC).

11. **ADR-0039 preservado** — TextStyle intocado.

12. **Crystalline-lint zero violations** obrigatório.

13. **Reutilização literal helpers cross-passos** **N=9 → N=10 cumulativo**:
    - 3 helpers Coons P270.3 (`bezier_control_points_for_arc`, `compute_coons_patches_n_stops`, `emit_conic_coons_stream` template).
    - Helpers CMYK P270.2 (Function 4-component dictionary; `multispace_sample_stops_*_cmyk`).
    - `interpolate_in_space` arm Cmyk (L1 dispatcher P270).
    - `to_cmyk_f32` (Color P257).

14. **Regressão tests P262-P270.3 proibida** — 2560 baseline preservado. Default `space != Cmyk` continua Type 4 Gouraud literal P268+P268.2. Apenas `space == Cmyk` activa Type 6 Coons CMYK.

15. **Cap LOC hard vs soft explícito** (quarta aplicação consolida sub-padrão N=4):
    - **Cap hard L3**: 200 LOC. Estouro dispara §política condição 4.
    - **Cap soft L3**: 120 LOC. Estouro regista relatório.
    - **Cap hard testes**: 18.
    - **Cap soft testes**: 12.

16. **Adaptive N CMYK NÃO se aplica a Coons** — sub-decisão prévia "recalibrar factor_delta CMYK" foi tomada para Conic Gouraud P268.2 (N adaptive). Coons usa 1 patch per stop (N = stops.len()); não há adaptive N a recalibrar. Sub-decisão preservada como reserva para P-Gradient-Adaptive-Multispace candidato futuro.

17. **Vanilla read-first autorizado** — `lab/typst-original/` Coons CMYK emit (se vanilla materializa; krilla opaco) + ISO 32000-1 §7.5.7.4 4-component vertex bytes.

---

## §1 — Sub-passo P270.4.A — Diagnóstico empírico Coons CMYK + verificação ADR-0092

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-conic-coons-cmyk-passo-270-4.md`.

### Comandos exactos a executar

```bash
# 1. Cristalino helpers Coons P270.3 (verificar #[allow(dead_code)] marcações)
rg -n "emit_conic_coons_stream|bezier_control_points_for_arc|compute_coons_patches_n_stops" 03_infra/src/export.rs | head -20

# 2. Cristalino dispatcher Conic actual (preserved P268+P268.2 literal P270.3)
rg -n "GradientObjectKind::Conic|emit_conic_gouraud_stream" 03_infra/src/export.rs | head -20

# 3. Cristalino CMYK pipeline P270.2 (template estrutural reutilizável)
rg -n "multispace_sample_stops_linear_cmyk|multispace_sample_stops_radial_cmyk|emit_function_dict_cmyk" 03_infra/src/export.rs | head -20

# 4. Cristalino Color::to_cmyk_f32 + interpolate_cmyk (P257 + P270 dispatcher)
rg -n "to_cmyk_f32|interpolate_cmyk|Cmyk" 01_core/src/entities/color.rs 01_core/src/entities/gradient.rs | head -20

# 5. PDF spec Type 6 Coons CMYK structure (vanilla precedente se disponível)
rg -n "Coons.*CMYK|Coons.*DeviceCMYK|patch.*cmyk" lab/typst-original/ 2>/dev/null | head -10

# 6. Tests P262-P270.3 baseline (regressão obrigatória)
cargo test -p typst-cristalino-infra gradient 2>&1 | tail -10
```

### Estrutura do diagnóstico (§A.1 a §A.13)

```
§A.1 ADR-0092 verificação estrutura disponível:
     - 3 helpers Coons P270.3 com #[allow(dead_code)]: removível
       quando P270.4 conecta dispatcher.
     - emit_conic_coons_stream template P270.3 retorna Vec<u8> bytes
       RGB (3 bytes per corner color).

§A.2 PROPOSTA emit_conic_coons_stream_cmyk (variant CMYK):
     - Paridade estrutural emit_conic_coons_stream P270.3.
     - Corner colors via `stop.color.to_cmyk_f32()` (4 bytes per corner).
     - Total bytes per patch: 1 flag + 24 control points + 16 corner CMYK
       = 41 bytes (vs 37 RGB).
     - N stops → 41N bytes total.

§A.3 PROPOSTA dispatcher Conic em emit_gradient_objects:
     - Branch novo P270.4:
       if conic.space == ColorSpace::Cmyk {
           // P270.4 — Type 6 Coons CMYK
           let stream = emit_conic_coons_stream_cmyk(conic);
           // /ShadingType 6 /ColorSpace /DeviceCMYK
           // /BitsPerCoordinate 8 /BitsPerComponent 8 /BitsPerFlag 8
           // /Decode [0 1 0 1 0 1 0 1 0 1 0 1]  (6 pares: x, y, c, m, y, k)
       } else {
           // P268+P268.2 preserved (Type 4 Gouraud RGB-family + perceptual).
       }
     - Decisão arquitectural: 2 emit paths Conic AGORA AMBOS ACTIVOS
       (Cenário A revisado fechado).

§A.4 PROPOSTA shading dictionary CMYK:
     - /ShadingType 6 (Coons patch mesh).
     - /ColorSpace /DeviceCMYK (vs /DeviceRGB em P270.3 RGB).
     - /BitsPerCoordinate 8 (preservado).
     - /BitsPerComponent 8 (preservado).
     - /BitsPerFlag 8 (preservado).
     - /Decode array: 6 pares [0 1 0 1 0 1 0 1 0 1 0 1] (x, y, c, m, y, k).
       (vs 5 pares em RGB: x, y, r, g, b.)

§A.5 Tests P270.3 baseline preserved:
     - 15 tests P270.3 (8 unit + 4 E2E + 3 snapshot) verdes literal.
     - Helpers Coons existentes não modificados estruturalmente.
     - #[allow(dead_code)] removido quando dispatcher conecta.

§A.6 Adaptive N CMYK NÃO se aplica a Coons:
     - Coons strategy 1 patch per stop (N = conic.stops.len()).
     - Não há N adaptive a recalibrar (apenas em Gouraud P268.2).
     - Sub-decisão "factor_delta CMYK recalibrar" preservada reserva.

§A.7 Defaults preservam P262-P270.3:
     - space != Cmyk → branch "else" → emit_conic_gouraud_stream literal.
     - 2560 baseline bit-exact preserved.
     - space == Cmyk → branch novo P270.4 → Coons CMYK.

§A.8 Bug #4422 resolvido para Conic CMYK:
     - Cristalino emit /ColorSpace /DeviceCMYK correcto (paridade P270.2
       Linear+Radial).
     - pdfkit #532 análogo confirma causa raiz universal.

§A.9 Reader compatibility:
     - Type 6 Coons CMYK em readers principais (Cairo/Inkscape/Adobe
       Reader/Apple Preview): suporte universal esperado (industry
       precedent 20+ anos).
     - PDF/A compliance: DeviceCMYK directo sem ICC; refino futuro
       candidato P-Gradient-CMYK-ICC.

§A.10 Estimativa cap LOC:
     - emit_conic_coons_stream_cmyk: ~40-50 LOC (variant CMYK).
     - Dispatcher branching novo: ~15 LOC.
     - Shading dictionary CMYK: ~15-20 LOC.
     - Remoção #[allow(dead_code)] P270.3: -3 LOC.
     - Tests: ~80-120 LOC.
     - Total L3 production: ~70-85 LOC.
     - Cap hard 200 folga ~58-65%; cap soft 120 folga ~30-42%.

§A.11 Cenário detectado:
     - **B1 fecho conceptual** (activação opt-in trivial; helpers
       P270.3 preparados).
     - **B2 sub-passos** improvável (P270.3 já preparou infra-estrutura
       toda).

§A.12 Decisão arquitectural — cluster L3 24/24 absoluto via Coons CMYK
     activação.

§A.13 Cluster Gradient L1+stdlib+L3 emit feature-complete absoluto
     pós-P270.4 — marco arquitectural máximo do cluster.
```

### Critério de aceitação Fase A

- §A.2 confirma estrutura emit_conic_coons_stream_cmyk paridade P270.3.
- §A.3 confirma dispatcher branching analógico P270.2 Linear+Radial.
- §A.5 confirma 2560 baseline preserved bit-exact.
- §A.6 confirma adaptive N não se aplica (clarifica sub-decisão prévia).
- §A.7 confirma defaults preserved.
- §A.10 confirma estimativa cap hard 200 com folga ~58-65%.

---

## §2 — Sub-passo P270.4.B — Anotações cumulativas (fecho série P270)

### B.1 — ADR-0092 anotação cumulativa P270.4

Adicionar após §"Decisão Cenário A revisado":

```
## Anotação cumulativa P270.4 — Coons CMYK activação opt-in flag ON

**Data**: 2026-05-17.
**Motivo**: ADR-0092 §"Decisão Cenário A revisado" Conic CMYK via Coons
materializado. Cluster Gradient L3 emit feature-complete 24/24 absoluto.

**Materialização opt-in flag ON**:
- emit_conic_coons_stream_cmyk variant criado paridade estrutural
  emit_conic_coons_stream RGB (P270.3).
- Dispatcher Conic em emit_gradient_objects:
  - space == Cmyk → /ShadingType 6 /ColorSpace /DeviceCMYK + Coons
    stream 41 bytes/patch.
  - senão → /ShadingType 4 /ColorSpace /DeviceRGB + Gouraud P268+P268.2
    preserved literal.
- 3 helpers Coons P270.3 perdem #[allow(dead_code)] — agora em uso.

**Conic 2 emit paths AGORA AMBOS ACTIVOS**:
- Type 4 Gouraud: 7 spaces RGB-family + perceptual (P268+P268.2+P270.1
  preserved).
- Type 6 Coons: CMYK (P270.4 activação).

**Bug #4422 resolvido para Conic CMYK por construção** — cristalino
emit /ColorSpace /DeviceCMYK correcto via Coons. Paridade P270.2
Linear+Radial.

**Reader compatibility Type 6 + DeviceCMYK**:
- Cairo/Inkscape precedent 20+ anos: universal suporte Type 6 + CMYK.
- Adobe Reader/Apple Preview/pdf.js: suporte esperado universal.
- PDF/A compliance: DeviceCMYK directo sem ICC; refino futuro
  candidato P-Gradient-CMYK-ICC.

**Adaptive N NÃO se aplica a Coons** — strategy 1 patch per stop
(N = conic.stops.len()). Sub-decisão prévia "recalibrar factor_delta
CMYK" preservada reserva para P-Gradient-Adaptive-Multispace.

**Helpers reutilizados literal**:
- bezier_control_points_for_arc (P270.3) — coord patches preserved.
- compute_coons_patches_n_stops (P270.3) — preserved.
- emit_conic_coons_stream (P270.3 RGB) — template estrutural CMYK.
- to_cmyk_f32 (Color P257) — corner colors.
- interpolate_in_space arm Cmyk (P270 dispatcher) — interpolação.
- Sub-padrão "Reutilização literal helpers cross-passos" N=9 → N=10.

**Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto**
— marco arquitectural máximo. 3 variants × 8 spaces = 24 combinações
user-facing.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=9 → N=10
cumulativo.

**Sub-padrão "ADR scope-out revogado parcialmente"** atinge **N=6
cumulativo limiar formalização clara ainda mais ultrapassado** —
meta-ADR URGENTE.

**Marco final série P270** — P270 + P270.1 + P270.2 + P270.3 + P270.4
fecham cluster Gradient feature-complete a nível user-facing.
```

### B.2 — ADR-0091 anotação cumulativa P270.4 (revogação final §"Conic CMYK scope-out preserved")

Adicionar após anotação cumulativa P270.2:

```
## Anotação cumulativa P270.4 — Conic CMYK scope-out revogação final

**Data**: 2026-05-17.
**Motivo**: §"Conic CMYK scope-out preserved" P270.2 Cenário B revogado
**definitivamente** por activação Coons CMYK P270.4 (ADR-0092 EM VIGOR).

**Cluster Gradient L3 emit pós-P270.4**:
- Linear: 8/8 spaces (P270.1 + P270.2 CMYK directo).
- Radial: 8/8 spaces (P270.1 + P270.2 CMYK directo; focal_* P269 preserved).
- **Conic: 8/8 spaces** (P270.1 7 spaces Gouraud + **P270.4 CMYK Coons**).

**Cluster L3 emit 24/24 absoluto** — paridade vanilla user-facing total.

Estrutura emit Conic divide entre 2 estratégias coexistentes:
- Type 4 Gouraud (P268+P268.2) para 7 spaces RGB-family + perceptual.
- Type 6 Coons (P270.4) para CMYK.

Ver ADR-0092 §"Anotação cumulativa P270.4" para detalhes.

**Cluster ColorSpace runtime completo L1+stdlib+L3** — paridade vanilla
total a nível user-facing. ADR-0091 §"Decisão L3 (materializada
P270.1+P270.2+P270.4)" — sequência completa.
```

### B.3 — ADR-0083 anotação cumulativa P270.4 (revogação final §DeviceCMYK)

Adicionar após anotação P270.2:

```
## Anotação cumulativa P270.4 — DeviceCMYK PDF revogação final absoluta

**Data**: 2026-05-17.
**Motivo**: P270.2 revogou parcialmente §"DeviceCMYK PDF" para Linear+
Radial; P270.4 estende para Conic via Type 6 Coons.

**§"DeviceCMYK PDF" revogado DEFINITIVO**:
- Linear: P270.2 directo /ShadingType 2.
- Radial: P270.2 directo /ShadingType 3 (focal_* preserved).
- Conic: P270.4 via /ShadingType 6 Coons patches.

**Scope-outs ADR-0083 status final pós-P270.4**:
- ~~ColorSpace runtime~~: revogado P270 (anotação cumulativa).
- ~~DeviceCMYK PDF~~: revogado P270.2 + P270.4 (esta anotação final).
- Operadores cor: preserved scope-out.
- Constantes nomeadas extras: preserved scope-out.

ADR-0083 perfil graded DEBT-1 §"Color paridade vanilla 8/8 spaces"
agora cobre L1 + L3 PDF emit em 8/8 spaces para 3/3 variants =
24/24 absoluto.

**Sub-padrão "ADR scope-out revogado parcialmente" N=6 cumulativo
limiar muito ultrapassado** — meta-ADR URGENTE. Ver ADR-0092
§"Anotação cumulativa P270.4".
```

### B.4 — ADR-0089 anotação cumulativa P270.4

```
## Anotação cumulativa P270.4 — Conic 2 emit paths ambos activos

Conic L3 emit dispatcher dual ACTIVO:
- **Type 4 Gouraud** (P268+P268.2 preserved): 7 spaces RGB-family
  + perceptual via /ShadingType 4 /ColorSpace /DeviceRGB.
- **Type 6 Coons** (P270.3 infra + P270.4 activação): CMYK via
  /ShadingType 6 /ColorSpace /DeviceCMYK.

Estratégia dispatcher por `conic.space`:
- space != Cmyk → Type 4 Gouraud.
- space == Cmyk → Type 6 Coons.

Ver ADR-0092 §"Anotação cumulativa P270.4" para detalhes técnicos.
```

### B.5 — ADR-0054 anotação cumulativa P270.4 (fecho cluster)

```
P270.4 — cluster Gradient L1+stdlib+L3 emit feature-complete absoluto
24/24 (3 variants × 8 spaces). Marco arquitectural máximo do cluster.
Perfil graded DEBT-1 §"Color paridade vanilla" agora cobre 24/24
combinações user-facing L1+L3. Série P270 completa (P270+P270.1+
P270.2+P270.3+P270.4).
```

### B.6 — L0 `entities/gradient.md` anotação P270.4

Adicionar após anotação P270.3:

```
**Anotação P270.4**: opt-in flag Coons CMYK activado.
emit_conic_coons_stream_cmyk variant materializado (corner colors
4-component CMYK; 41 bytes/patch). Dispatcher Conic em
emit_gradient_objects:
- space == Cmyk → /ShadingType 6 /ColorSpace /DeviceCMYK Coons.
- senão → /ShadingType 4 /ColorSpace /DeviceRGB Gouraud P268+P268.2.

Helpers Coons P270.3 perdem #[allow(dead_code)]. Adaptive N não se
aplica a Coons (strategy 1 patch per stop). Bug #4422 resolvido por
construção. Cluster Gradient L3 emit 24/24 absoluto. Ver ADR-0092
§"Anotação cumulativa P270.4".
```

### B.7 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hashes em L0. Zero violations.

---

## §3 — Sub-passo P270.4.C — Materialização L3 (testes primeiro)

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0092 + ADR-0091 + ADR-0083 + ADR-0089 + ADR-0054 anotações §2.
3. L0 anotação §2.B.6.
4. `crystalline-lint --fix-hashes`.
5. **Testes-primeiro** — adicionar ~10-12 testes ANTES de qualquer LOC L3.
6. L3 código — emit_conic_coons_stream_cmyk variant + dispatcher branching + shading dict CMYK + remover `#[allow(dead_code)]` P270.3.
7. Verificação final.

### Cap LOC

- **L3 hard**: 200 LOC. Estouro dispara §política condição 4.
- **L3 soft**: 120 LOC. Estouro regista relatório.
- **Testes hard**: 18.
- **Testes soft**: 12.

### Alteração L3 esperada

```rust
// 03_invra/src/export.rs P270.4

// 1. Remover #[allow(dead_code)] dos 3 helpers Coons P270.3:
//    - bezier_control_points_for_arc
//    - compute_coons_patches_n_stops
//    - emit_conic_coons_stream

// 2. Helper novo: variant CMYK paridade estrutural
fn emit_conic_coons_stream_cmyk(conic: &Conic) -> Vec<u8> {
    let n = compute_coons_patches_n_stops(conic);
    let mut stream = Vec::with_capacity(41 * n);
    
    let center = (0.5, 0.5);
    let radius = 0.5;
    
    for i in 0..n {
        let stop_curr = &conic.stops[i];
        let stop_next = &conic.stops[(i + 1) % n];
        
        let angle_start = (i as f32) / (n as f32) * std::f32::consts::TAU;
        let angle_end = ((i + 1) as f32) / (n as f32) * std::f32::consts::TAU;
        
        let flag: u8 = 0;
        stream.push(flag);
        
        // 12 control points × 2 coord bytes (paridade P270.3 RGB literal).
        let cp_arc = bezier_control_points_for_arc(center, radius, angle_start, angle_end);
        // ... emit 24 bytes coordinates ...
        
        // 4 corner colors × 4 bytes CMYK (vs 3 RGB em P270.3).
        let (c0, m0, y0, k0) = stop_curr.color.to_cmyk_f32();
        let (c1, m1, y1, k1) = stop_next.color.to_cmyk_f32();
        
        // corner 0 (centro topo): stop_curr CMYK
        stream.extend_from_slice(&[
            (c0 * 255.0) as u8,
            (m0 * 255.0) as u8,
            (y0 * 255.0) as u8,
            (k0 * 255.0) as u8,
        ]);
        // corner 1 (edge_start): stop_curr CMYK
        stream.extend_from_slice(&[
            (c0 * 255.0) as u8,
            (m0 * 255.0) as u8,
            (y0 * 255.0) as u8,
            (k0 * 255.0) as u8,
        ]);
        // corner 2 (edge_end): stop_next CMYK
        stream.extend_from_slice(&[
            (c1 * 255.0) as u8,
            (m1 * 255.0) as u8,
            (y1 * 255.0) as u8,
            (k1 * 255.0) as u8,
        ]);
        // corner 3 (centro baixo singularidade): stop_next CMYK
        stream.extend_from_slice(&[
            (c1 * 255.0) as u8,
            (m1 * 255.0) as u8,
            (y1 * 255.0) as u8,
            (k1 * 255.0) as u8,
        ]);
    }
    
    stream
}

// 3. Dispatcher branching em emit_gradient_objects:
GradientObjectKind::Conic(conic) => {
    if conic.space == ColorSpace::Cmyk {
        // P270.4 — Type 6 Coons CMYK
        let stream = emit_conic_coons_stream_cmyk(conic);
        let shading = format!(
            "<< /ShadingType 6 \
               /ColorSpace /DeviceCMYK \
               /BitsPerCoordinate 8 \
               /BitsPerComponent 8 \
               /BitsPerFlag 8 \
               /Decode [0 1 0 1 0 1 0 1 0 1 0 1] \
               /Length {} >>",
            stream.len()
        );
        // ... emit shading + stream
    } else {
        // P268+P268.2 preserved literal (Type 4 Gouraud RGB-family + perceptual).
        let n = compute_adaptive_n_conic(conic);
        let stream = emit_conic_gouraud_stream(conic, n);
        // ...
    }
}
```

### Estrutura testes esperada

**Unit emit_conic_coons_stream_cmyk** (4 tests):
- `p270_4_emit_conic_coons_cmyk_stream_size_41n_bytes`: stream size = 41 × stops.len().
- `p270_4_emit_conic_coons_cmyk_corner_colors_4_bytes`: 4 bytes per corner.
- `p270_4_emit_conic_coons_cmyk_paridade_p270_3_rgb_structure`: structure paridade (1 flag + 24 coords + 16 CMYK = 41 bytes vs 37 RGB).
- `p270_4_emit_conic_coons_cmyk_preserva_p270_3_helpers`: helpers shared sem alteração.

**E2E PDF dispatcher Conic CMYK** (4 tests):
- `p270_4_export_pdf_conic_cmyk_shading_devicecmyk`: confirma `/ShadingType 6` + `/ColorSpace /DeviceCMYK`.
- `p270_4_export_pdf_conic_oklab_preserva_p268_gouraud`: defaults Type 4 Gouraud preserved.
- `p270_4_export_pdf_conic_cmyk_decode_array_6_pares`: `/Decode [0 1 0 1 0 1 0 1 0 1 0 1]`.
- `p270_4_export_pdf_cluster_3_variants_cmyk_coexistem`: cluster L3 24/24 absoluto.

**Snapshot determinístico** (3 tests):
- `p270_4_pdf_bytes_conic_cmyk_reproduziveis`.
- `p270_4_pdf_bytes_default_oklab_preserved_p268`.
- `p270_4_pdf_bytes_cluster_24_24_absoluto_reproduziveis`.

**Bug #4422 resolução final** (1 test):
- `p270_4_export_pdf_conic_cmyk_resolve_bug_4422_dictionary`: confirma cristalino emit correcto.

**Regressão P262-P270.3** (não novos; verificar verdes):
- 2560 baseline preserved literal (defaults preservam Type 4 Gouraud).

Total esperado: 4 + 4 + 3 + 1 = **12 testes**. Cap soft 12 / hard 18; folga.

---

## §4 — Sub-passo P270.4.D — Fecho cluster + README + relatório

1. **ADR-0092** anotação cumulativa P270.4 fechada (cluster L3 24/24 absoluto).
2. **ADR-0091** anotação cumulativa P270.4 (revogação final §"Conic CMYK scope-out preserved").
3. **ADR-0083** anotação cumulativa P270.4 (revogação final §DeviceCMYK).
4. **ADR-0089** anotação cumulativa P270.4 (Conic 2 emit paths ambos activos).
5. **ADR-0054** anotação cumulativa P270.4 (fecho cluster).
6. **ADR-0087/0088/0090** preservadas literal.
7. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (+~1-2pp via Conic CMYK; cluster L3 24/24 absoluto).
   - Entrada P270.4 ~80-100 linhas (fecho série P270 + 5 anotações cumulativas).
   - Cross-reference ADR-0092 §"Anotação cumulativa P270.4".
   - **Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto** — marco máximo do cluster.
   - ADR-0091 + ADR-0083 §scope-outs CMYK revogados finais marcados.
8. **Distribuição ADRs preservada** — total 79 mantido (sem ADR nova; 5 anotações cumulativas).
9. **Relatório** `00_nucleo/materialization/typst-passo-270-4-relatorio.md`:
   - Métricas finais (esperado 2560 + 12 = ~2572).
   - Fase A §A.6 clarificação adaptive N não se aplica documentada.
   - Diff helpers L3 antes/depois (+1 helper novo + 3 sem `#[allow(dead_code)]`).
   - Sub-padrões + N cumulativo (4 atingem limiar formalização clara).
   - Regressão zero 2560 baseline preserved.
   - **Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto** — marco máximo.
   - **Bug #4422 resolvido por construção final** (Linear + Radial + Conic CMYK).

---

## §política de paragem

1. **Fase A §A.1 helpers P270.3 não disponíveis** — improvável (P270.3 fechado limpo); confirma. Se gap, indica regressão arquitectural.

2. **§A.6 adaptive N CMYK ambiguidade** — se Fase A revelar que Coons precisa adaptive N (improvável dado strategy 1 patch per stop), revisar sub-decisão prévia.

3. **Cap LOC L3 hard (200) ameaça ser ultrapassado** — refactor maior que estimativa §A.10 (~70-85 LOC). Confirmar antes.

4. **Cap testes hard (18) ameaça ser ultrapassado**.

5. **§A.7 verificação falha**: defaults Type 4 Gouraud não preservam bytes P268+P268.2 literal. §política absoluta.

6. **Snapshot bytes PDF não reproduzíveis** — float non-determinism em CMYK conversões.

7. **Crystalline-lint reporta violations** após anotações.

8. **Regressão tests P262-P270.3** — qualquer test anterior falha. §política absoluta.

9. **Conic CMYK Type 6 reader compatibility issues** — Fase A testa empíricamente com pdftoppm/mupdf. Improvável dado Cairo precedent 20+ anos, mas confirmar.

10. **PDF reader emit issues bug #4422 residual** — sintomas não relacionados ao dictionary podem persistir; documentar mas não bloquear.

11. **Cluster Gradient marco quebra** — `p270_4_export_pdf_cluster_3_variants_cmyk_coexistem` falha.

12. **Anotações cross-ADR 5 ADRs coerência** — verificar cada anotação refere ADR-0092 §"Anotação cumulativa P270.4".

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-P270.4 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=9 → N=10 cumulativo** | + P270.4 ADR-0092 anotada |
| **ADR scope-out revogado parcialmente** | **N=5 → N=6 cumulativo (limiar muito ultrapassado)** | + P270.4 (ADR-0091 + ADR-0083) — **candidato meta-ADR URGENTE FINAL** |
| Reutilização literal helpers cross-passos | **N=9 → N=10 cumulativo** | + P270.4 (3 helpers P270.3 + helpers P270.2 + P257 + P270) |
| Diagnóstico imutável (décimo segundo consumo) | **N=16 → N=17 cumulativo** | + P270.4 |
| Auditoria condicional (ADR-0084) | **N=15 → N=16 cumulativo** | + P270.4 |
| Auto-aplicação ADR-0065 inline | **N=14 → N=15 cumulativo** | + P270.4 |
| **Cap LOC hard vs soft explícito** | **N=3 → N=4 cumulativo (consolidação total)** | + P270.4 — pattern estabelecido sólido |
| **Anotação cumulativa cross-ADR** | **N=4 → N=5 cumulativo** | + P270.4 (5 ADRs anotadas) |
| Fase A com industry research proactiva | N=3 preserved | P270.4 reutiliza pesquisa P270.3 (não nova) |

### Marco arquitectural máximo P270.4 — fecho cluster Gradient

**Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto** — paridade vanilla user-facing total para `gradient.linear/radial/conic(...)` em 8 spaces. Cluster cristalino agora cobre:
- 3 variants (Linear, Radial, Conic).
- 8 spaces (Oklab, Oklch, sRGB, LinearRGB, Luma, HSL, HSV, CMYK).
- **24 combinações user-facing total — todas materializadas L1+stdlib+L3 emit**.

**Estratégia L3 emit final**:
- Linear: /ShadingType 2 axial; 7 RGB-family /DeviceRGB + 1 CMYK /DeviceCMYK.
- Radial: /ShadingType 3 radial; 7 RGB-family /DeviceRGB + 1 CMYK /DeviceCMYK (focal_* P269 preserved).
- Conic: 2 estratégias coexistentes:
  - 7 RGB-family + perceptual: /ShadingType 4 Gouraud (P268+P268.2 preserved).
  - CMYK: /ShadingType 6 Coons (P270.3 + P270.4).

**ADR-0083 §"Color paridade vanilla 8/8 spaces" cobertura final** — L1 + L3 PDF emit em 24/24 combinações. Cluster Color resolvido a nível user-facing.

**ADR-0091 + ADR-0083 §scope-outs CMYK revogados finais** — cluster Gradient pendência CMYK fechada.

**Sub-padrão "ADR scope-out revogado parcialmente" N=6 cumulativo** — meta-ADR formalização URGENTE.

**Série P270 completa** — P270 (L1+stdlib) + P270.1 (L3 7 spaces) + P270.2 (L3 CMYK Linear+Radial) + P270.3 (Coons RGB infra) + P270.4 (Coons CMYK activação) = cluster Gradient feature-complete.

### Sequência pós-P270.4

Cluster Gradient resolvido a nível user-facing. Pendências:

- **Meta-ADR formalização sub-padrões N=6 + N=3 + N=4 + N=10** — passo administrativo XS candidato URGENTE paridade P260 ADR-0084/0085.
- **P-Gradient-Coons-RGB-Final** (M futuro; converge Conic RGB Type 4 → Type 6; elimina 2 estratégias coexistentes).
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **P-Gradient-CMYK-ICC** (S-M futuro; krilla paridade ICC profiles; PDF/A compliance).
- **P-Gradient-Adaptive-Multispace** (S futuro; N adaptive HSL/Oklch hue diff alto).
- **ADR-0055bis variant-aware fonts** (M; refino Text P266).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** + outros Visualize.
- **Tiling activação** (Paint::Tiling).

---

## §referências cross-passos

- **P262/P264/P267** — Variant L1+stdlib (preservados).
- **P263/P265/P268** — L3 emit templates (preservados; CMYK Coons branch aditivo P270.4).
- **P268.2** — Adaptive N hybrid Gouraud (preserved; aplicado a Type 4 RGB).
- **P269** — Radial focal_* (preserved; campo space cross-variant em 2 branches).
- **P270/P270.1/P270.2/P270.3** — ColorSpace runtime L1+stdlib + L3 emit RGB + L3 emit CMYK Linear+Radial + Coons RGB infra (precedentes directos).
- **P257** — Color 8/8 spaces (ADR-0083; §DeviceCMYK revogado final P270.4).
- ADR-0092 — Conic Coons Patches (anotada cumulativa P270.4 activação).
- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa P270.4 revogação final).
- ADR-0083 — Color paridade (§DeviceCMYK revogação final P270.4).
- ADR-0089 — Gradient Conic-only (anotada cumulativa P270.4).
- ADR-0054 — Perfil graded (anotada cumulativa P270.4 fecho cluster).
- ADR-0087/0088 — Linear/Radial strategies (preservadas).
- ADR-0090 — Type 4 Gouraud strategy (preservada).
- ADR-0018 — Whitelist crates (preservada; ICC scope-out).
- ADR-0085 — Diagnóstico imutável (décimo segundo consumo).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A §A.1 + §A.2** — verificar helpers P270.3 disponíveis e estrutura. Se gap, indica regressão arquitectural improvável.
- **§A.6 clarificação literal**: adaptive N NÃO se aplica a Coons (strategy 1 patch per stop). Sub-decisão prévia recalibrar factor_delta CMYK preservada reserva.
- **Defaults Type 4 Gouraud preservam bytes P268+P268.2** literal — §política condição 5 absoluta. Verificar com `cargo test p268_`.
- **Regressão tests P262-P270.3 zero** (2560 baseline) — §política condição 8 absoluta.
- **Helpers Coons P270.3 perdem `#[allow(dead_code)]`** — 3 marcações removidas; net -3 LOC.
- **emit_conic_coons_stream_cmyk variant** — paridade estrutural P270.3 RGB; corner colors 4-component CMYK; 41 bytes/patch (vs 37 RGB).
- **Dispatcher Conic branching** — paridade P270.2 Linear+Radial: if space == Cmyk then Coons else Gouraud.
- **Shading dictionary CMYK** — /ShadingType 6 /ColorSpace /DeviceCMYK /Decode 6 pares (x, y, c, m, y, k).
- **Bug #4422 resolvido por construção** — verificar via test `p270_4_export_pdf_conic_cmyk_resolve_bug_4422_dictionary`.
- **Cap hard L3 200 + testes hard 18** — gate absoluto; estouro dispara §política.
- **Cap soft L3 120 + testes soft 12** — informativo; estouro regista mas continua.
- **Anotações cross-ADR 5 ADRs** — verificar coerência cada anotação refere ADR-0092 §"Anotação cumulativa P270.4".
- **Relatório final esperado**: 2560 + 12 = ~2572 testes verdes; hash drift L0; lint zero; ADRs 79 preservado (sem ADR nova; 5 anotações cumulativas).
- **Marco máximo "Cluster Gradient L1+stdlib+L3 emit feature-complete 24/24 absoluto"** documentado em relatório §1 + ADR-0092 §"Anotação cumulativa P270.4" + ADR-0091 §"Anotação cumulativa P270.4" + ADR-0083 §"Anotação cumulativa P270.4".
