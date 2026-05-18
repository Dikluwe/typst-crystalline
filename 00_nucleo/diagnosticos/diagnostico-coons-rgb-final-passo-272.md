# Diagnóstico imutável P272.A — Coons RGB Final (converge Conic RGB Type 4 → Type 6; ADR-0090 REVOGADO)

**Data**: 2026-05-17.
**Passo**: typst-passo-272.
**Magnitude**: M (cap composto; net LOC negativo esperado).
**Cluster**: Visualize / Gradient / PDF export.
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Décimo terceiro consumo directo de fonte** (P262-P271 cumulativo;
P272 reutiliza consolidação industry P270.3 ADR-0092 §"Pesquisa
empírica industry" — Cairo + Typst original blog 2023 + ISO 32000-1
§7.5.7.4; sub-padrão "Fase A com industry research proactiva"
**preservado N=4** — não incrementa porque material consolidado
P270.3).

---

## §A.1 — Cristalino L3 dispatcher Conic actual (P270.4)

`03_infra/src/export.rs:1732-1782`:

```rust
GradientObjectKind::Conic(conic) => {
    use typst_core::entities::layout_types::ColorSpace;
    // P270.4 — dispatcher dual: Type 6 Coons CMYK vs Type 4 Gouraud RGB.
    if conic.space == ColorSpace::Cmyk {
        // P270.4 — Type 6 Coons Patch Mesh CMYK (ADR-0092 EM VIGOR).
        let stream = emit_conic_coons_stream_cmyk(conic);
        // /ShadingType 6 + /DeviceCMYK + Decode 6 pares + Function Type 2 N=1 identity.
    } else {
        // P268+P268.2 preserved literal (Type 4 Gouraud RGB-family + perceptual).
        let _ = multispace_sample_stops_conic(conic, 16); // helper validação
        let n_adaptive = compute_adaptive_n_conic(conic);
        let stream = emit_conic_gouraud_stream(conic, n_adaptive);
        // /ShadingType 4 + /DeviceRGB + Decode 5 pares + Function Type 2 N=1 identity.
    }
}
```

**Estado pós-P272 (proposto)**: dispatcher unificado `/ShadingType 6`
para 8/8 spaces; CMYK arm preserved (P270.4); RGB arm activa
`emit_conic_coons_stream_rgb` (P270.3 helper extension).

---

## §A.2 — Helpers Coons P270.3 RGB estado

`03_infra/src/export.rs:816-981`:

- `bezier_control_points_for_arc(center, radius, start_angle, end_angle) -> [(f32, f32); 2]` (827-851).
- `compute_coons_patches_n_stops(conic) -> usize` (856-858) — strategy
  "1 patch per stop" (preserved literal para CMYK P270.4).
- `emit_conic_coons_stream(conic) -> Vec<u8>` (895-981) — RGB version
  com `#[allow(dead_code)]` reservado para P-Gradient-Coons-RGB-Final.
  **37 bytes per patch**: 1 flag + 24 coord + 12 RGB.

**Disponível para P272**: `#[allow(dead_code)]` será removido +
extension N=stops*4 patches estende strategy.

---

## §A.3 — Helpers Type 4 Gouraud P268+P268.2 a remover

`03_infra/src/export.rs`:

| Helper | Linhas | LOC | Callers extra-Conic Type 4 |
|---|---|---|---|
| `oklab_delta_e(c1, c2) -> f32` | 660-674 | ~15 | **APENAS `compute_adaptive_n_conic` + 1 test P268.2** |
| `compute_adaptive_n_conic(conic) -> usize` | 688-728 | ~40 | **APENAS dispatcher Conic RGB arm** |
| `emit_conic_gouraud_stream(conic, n) -> Vec<u8>` | 730-815 | ~85 | **APENAS dispatcher Conic RGB arm + 2 tests P268** |

**Total removível**: ~140 LOC (helpers) + ~30 LOC (comments/imports).

**Análise extra-callers** (§política condição 1 verificada):

```
$ rg -n "oklab_delta_e\b" 03_infra/src/ 01_core/src/ 02_shell/src/ 04_wiring/src/
03_infra/src/export.rs:660       (def)
03_infra/src/export.rs:702       (compute_adaptive_n_conic caller)
03_infra/src/export.rs:4214      (test P268.2 only)
01_core/src/entities/gradient.rs:169  (comment only — no code call)
```

✓ `oklab_delta_e` **safe to remove**: única call site fora do test é
`compute_adaptive_n_conic` (também removed). Comment em L1
gradient.rs:169 a actualizar.

```
$ rg -n "compute_adaptive_n_conic\b" 03_infra/src/
03_infra/src/export.rs:688   (def)
03_infra/src/export.rs:1758  (dispatcher RGB arm)
03_infra/src/export.rs:[tests p268_2_adaptive_n_*]
```

✓ `compute_adaptive_n_conic` **safe to remove**: único call site
production é dispatcher; tests P268.2 removed.

```
$ rg -n "emit_conic_gouraud_stream\b" 03_infra/src/
03_infra/src/export.rs:730  (def)
03_infra/src/export.rs:1759  (dispatcher RGB arm)
03_infra/src/export.rs:[tests p268_emit_conic_gouraud_stream_*]
```

✓ `emit_conic_gouraud_stream` **safe to remove**: único call site
production é dispatcher; tests P268+P268.2 removed.

---

## §A.4 — Tests P268+P268.2 a remover (paridade contagem)

**21 tests P268/P268.2** identificados (vs estimativa spec ~30):

### P268 (6 tests):

1. `p268_multispace_sample_stops_conic_red_blue_endpoints` (3822) —
   testa `multispace_sample_stops_conic` (helper preservado).
   **PRESERVAR** — não específico ao Gouraud emit.
2. `p268_emit_conic_gouraud_stream_n32_size` (3847).
3. `p268_emit_conic_gouraud_stream_min_8_slices` (3868).
4. `p268_export_pdf_conic_emits_shading_type_4` (3890).
5. `p268_export_pdf_conic_dedup_arc_ptr` (3947).
6. `p268_export_pdf_cluster_3_variants_coexistem` (3992).

### P268.2 (15 tests):

7-13. `p268_2_adaptive_n_*` (7 tests, lines 4091-4200).
14. `p268_2_oklab_delta_e_helper_red_blue` (4210).
15-17. `p268_2_export_pdf_conic_adaptive_n_*` (3 tests, lines 4223-4381).
18. `p268_2_export_pdf_regression_p268_cluster_3_variants` (4314).
19. `p268_2_export_pdf_conic_dedup_adaptive_n_preservado` (4381).
20-22. `p268_2_pdf_bytes_reproduziveis_*` (3 tests, lines 4432-4517).

**Total removidos**: **20 tests** (P268 sem #1 multispace + P268.2 todos)
= 20.

**Test `p268_multispace_sample_stops_conic_red_blue_endpoints`
PRESERVADO** — testa helper genérico `multispace_sample_stops_conic`
ainda usado por outros tests (lines 5222-5263).

---

## §A.5 — PROPOSTA emit_conic_coons_stream_rgb extension N=stops*4

**Strategy P272**: N patches angulares = `conic.stops.len() * 4`.

```rust
// Renomear ou adicionar helper extension:
fn compute_coons_patches_n_stops_extended(conic: &Conic) -> usize {
    conic.stops.len() * 4
}

// Activar P270.3 helper (remover #[allow(dead_code)]):
fn emit_conic_coons_stream_rgb(conic: &Conic) -> Vec<u8> {
    let n_stops = conic.stops.len();
    let n_patches = n_stops * 4;
    let mut stream = Vec::with_capacity(37 * n_patches);
    let center = (0.5, 0.5);
    let radius = 0.5;
    let angle_offset = conic.angle.to_rad() as f32;

    for i in 0..n_patches {
        let t_start = i as f32 / n_patches as f32;
        let t_end = (i + 1) as f32 / n_patches as f32;
        let color_start = conic.sample(t_start);
        let color_end = conic.sample(t_end);

        let angle_start = angle_offset + t_start * TAU;
        let angle_end = angle_offset + t_end * TAU;

        // ... emit 1 flag + 12 control points + 4 corner RGB
        // corners: [color_start, color_start, color_end, color_end]
    }
    stream
}
```

**Corner colors via `Conic::sample(t)`** (public API L1) — dispatches
via `interpolate_in_space` automaticamente no `conic.space`. Default
Oklab preserves perceptual quality; HSL/Oklch hue-wrap shorter; sRGB
linear; etc.

---

## §A.6 — PROPOSTA dispatcher Conic em emit_gradient_objects (P272)

```rust
GradientObjectKind::Conic(conic) => {
    use typst_core::entities::layout_types::ColorSpace;
    let (stream, colorspace, decode_array) = if conic.space == ColorSpace::Cmyk {
        (emit_conic_coons_stream_cmyk(conic),
         "/DeviceCMYK",
         "[0 1 0 1 0 1 0 1 0 1 0 1]")
    } else {
        (emit_conic_coons_stream_rgb(conic),
         "/DeviceRGB",
         "[0 1 0 1 0 1 0 1 0 1]")
    };
    let len = stream.len();
    let header = format!(
        "<< /ShadingType 6 /ColorSpace {} \
           /BitsPerCoordinate 8 /BitsPerComponent 8 \
           /BitsPerFlag 8 \
           /Decode {} \
           /Length {} >>\nstream\n",
        colorspace, decode_array, len,
    );
    let function_c0 = if conic.space == ColorSpace::Cmyk { "[0 0 0 0]" } else { "[0 0 0]" };
    let function_c1 = if conic.space == ColorSpace::Cmyk { "[1 1 1 1]" } else { "[1 1 1]" };
    self.add(function_id, format!(
        "<< /FunctionType 2 /Domain [0 1] /C0 {} /C1 {} /N 1 >>",
        function_c0, function_c1
    ));
    let mut shading_bytes = header.into_bytes();
    shading_bytes.extend_from_slice(&stream);
    shading_bytes.extend_from_slice(b"\nendstream");
    self.add_bytes(shading_id, shading_bytes);
}
```

---

## §A.7 — ADR-0090 transição REVOGADO P272

- Status: `EM VIGOR` → `REVOGADO P272`.
- §"Revogação P272" nova: documenta motivo industry-aligned
  (Cairo/Inkscape/Typst original blog 2023 mesh-based).
- Cross-reference ADR-0092 expandida.

---

## §A.8 — ADR-0092 anotação cumulativa P272 expansão

- §"Decisão Cenário A revisado" → §"Decisão Cenário A revisado FINAL".
- Estratégia unificada Coons para 8/8 spaces.
- Type 4 Gouraud descontinuado.

---

## §A.9 — Sub-padrão "ADR REVOGADO + substituta" N empírico cristalino

Pesquisa `^**Status**.*REVOGADO`:

| ADR | Status | Substituída por |
|---|---|---|
| ADR-0007 | `REVOGADO` | ADR-0018 (rustc_hash reintroduzido) |
| ADR-0028 | `REVOGADO` | ADR-0029 (pureza física L1) |
| **ADR-0090 P272** | `EM VIGOR` → `REVOGADO` | **ADR-0092 expandida (Coons unified)** |

**Sub-padrão "ADR REVOGADO + substituta"** **N=2 prévio → N=3
cumulativo** com P272. Pattern emergente (não inaugural; cristalino
historicamente já fez isto em ADR-0007/ADR-0018 e ADR-0028/ADR-0029).

**Distinção P272**: primeira aplicação **pós-formalização ADR-0093
P271** §Pattern 1 §"Quando NÃO aplicar". Sub-padrão **"Aplicação
meta-ADR (ADR-0093)" N=1 inaugural** — P272 demonstra que
formalização P271 é aplicável na prática.

---

## §A.10 — Estimativa cap LOC

- **Additions** (Coons RGB extension N=stops*4 + dispatcher unificado):
  ~80-100 LOC (rename + extension + dispatcher refactor).
- **Removals** (Type 4 Gouraud helpers + tests):
  - Helpers: ~140 LOC L3.
  - Tests: ~620 LOC tests (20 tests * ~31 LOC/test).
  - Total: ~760 LOC.
- **Net change L3**: -660 a -680 LOC (negativo intencional; limpeza).
- **Cap hard additions 200**: folga 100-120%. ✓
- **Cap soft additions 120**: folga 20-40%. ✓

---

## §A.11 — Tests delta esperado

- Removals: **20 tests** P268+P268.2 (vs estimativa spec ~30).
- Additions: **~18 tests** Coons RGB P272.
- **Net: -2 tests**.
- Baseline 2572 → **~2570**.

---

## §A.12 — Defaults preservam P262-P267 + P270.4 CMYK

- Linear: preserved literal (P262/P263/P270.1/P270.2).
- Radial: preserved literal (P264/P265/P269/P270.1/P270.2).
- Conic CMYK: preserved literal (P270.4 Coons CMYK).
- **Conic RGB-family + perceptual: MUDAR** (Type 4 Gouraud → Type 6
  Coons N=stops*4).
- **Conic byte snapshots P268+P268.2 MUDAM intencionalmente**
  (behaviour change; tests removed).

§política condição 6 (snapshot bytes Conic RGB NÃO preservados
literal) — confirmada absoluta.

---

## §A.13 — Cenário detectado

**B1 fecho conceptual** — helpers Coons preparados P270.3 + dispatcher
estrutura clara P270.4; refactor cirúrgico contido em ~80-100 LOC
additions + ~760 LOC removals. Cap LOC accommodates com folga.

---

## §A.14 — Strategy N = stops * 4 justificativa empírica

- **Cairo precedente**: patches sub-stop arbitrários supported.
- **Typst original blog 2023**: "1 patch per stop" simplicity (vanilla
  uses krilla SweepGradient opaco; ISO 32000-1 §7.5.7.4 mesh genérico).
- **Cristalino divergência P272**: N=stops*4 trade-off qualidade
  visual angular superior vs simplicidade.
- **Cap LOC accommodates** (~80-100 LOC additions; folga 100% sobre
  hard).
- **Corner colors interpolated**: cada patch tem corners
  conic.sample(t_start) + conic.sample(t_end); transições suaves
  Bezier control points P270.3 helper reused.

---

## §A.15 — Decisão arquitectural

**Cenário B1 confirmado** — estratégia Conic única Coons
materializada P272:

- **ADR-0090 REVOGADO** (Type 4 Gouraud descontinuado).
- **ADR-0092 expandida cumulativamente** (Cenário A revisado FINAL).
- **Dispatcher unificado `/ShadingType 6`** para 8/8 spaces.
- **Cluster Gradient L3 emit estratégia única feature-complete
  24/24 simplificado**.

**Sub-padrão "Aplicação meta-ADR (ADR-0093)" N=1 inaugural** —
primeira aplicação prática pós-formalização P271; demonstra empiria
da metodologia.

**Sub-padrão "Aplicação meta-ADR (ADR-0094)" N=1 inaugural** — Cap
LOC hard/soft Pattern 1 aplicado em P272 spec.

**Sub-padrão "ADR REVOGADO + substituta" N=3 cumulativo** —
cristalino N=2 prévio (ADR-0007/ADR-0018 e ADR-0028/ADR-0029) + P272
ADR-0090/ADR-0092 expandida.

**Sub-padrão "Reutilização literal helpers cross-passos" N=10 →
N=11 cumulativo** — helpers Coons P270.3 (3 funções) + Conic::sample
dispatcher P270.

---

*Diagnóstico imutável produzido em 2026-05-17 P272.A. Linhagem
empírica preservada como evidência ADR-0085 + auto-aplicação
ADR-0065. Décimo terceiro consumo directo de fonte (cristalino +
Cairo + Typst blog 2023 + ISO 32000-1 §7.5.7.4 reutilizados de
P270.3 ADR-0092 consolidação).*
