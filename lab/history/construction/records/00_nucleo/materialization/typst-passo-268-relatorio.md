# Relatório do passo P268 — PDF Conic shading complete (fecha cluster Gradient L1+stdlib+PDF 3/3)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-268.md`.
**Tipo**: passo composto sequencial L3 cross-cutting;
**subpadrão "dividir granularidade L1+stdlib / L3 dedicado"
N=3 cumulativo completo** (cluster Gradient encerrado).
**Análogo estrutural canónico**: P263 (Linear PDF) + P265
(Radial PDF).
**Magnitude planeada**: S-M (cap 250 LOC L3 + 30 tests).
**Magnitude real**: **S-M (~2h)** — ~190 LOC L3; 6 tests;
helpers Oklab P262/P263/P265 reutilizados literal.

---

## §1 — Sumário executivo

**User pre-flight decisão P268.A** (§política condição 1
disparou): Vanilla usa crate externa `krilla::SweepGradient`
(não autorizada cristalino per ADR-0018). User escolheu
**Type 4 Gouraud manual** em vez de scope-out preserved ou
fallback aproximação.

**Sem ADR nova** — **anotação cumulativa ADR-0089** (paridade
pattern P263 anotação ADR-0087 + P265 anotação ADR-0088).
Status `IMPLEMENTADO` preservado literal.

**Tests delta**: **2407 → 2413** (+6 P268; zero regressões).

**ADRs distribuição preservada**:
- PROPOSTO 11 / EM VIGOR 32 / IMPLEMENTADO 29 / **Total 76**.

**Ficheiros criados**:
- `00_nucleo/diagnosticos/diagnostico-pdf-conic-passo-268.md`
  (imutável per ADR-0085 — **quarto consumo directo vanilla**
  pós-P262/P264/P267).
- `00_nucleo/materialization/typst-passo-268-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/entities/gradient.md` (secção P268
  anotada).
- `01_core/src/entities/gradient.rs` (sem alterações de
  código; apenas hash propagado).
- `03_infra/src/export.rs` (~190 LOC novas: helpers
  oklab_sample_stops_conic + emit_conic_gouraud_stream +
  GradientObjectKind::Conic variant + emit_gradient_objects
  branching Conic + 3 sítios pattern-match substituídos + 6
  tests).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`
  (secção "Anotação cumulativa P268" adicionada).
- `00_nucleo/adr/README.md` (cross-reference ADR-0089 +
  entrada P268 ~50 linhas).

**~5 ficheiros tocados; ~190 LoC L3 + 6 tests + L0 + ADR
anotação**.

---

## §2 — Sub-passo P268.A — Diagnóstico PDF Conic vanilla

### Estratégia decidida

Vanilla usa crate externa `krilla::SweepGradient` (paint.rs:255)
não autorizada cristalino. Vanilla também emite warning
"conic gradients are not supported in this PDF standard"
(convert.rs:514).

User escolheu **Type 4 Free-Form Gouraud Triangle Mesh**
manual (PDF Spec ISO 32000 §7.5.7).

### Decisões locais

- N=32 fatias do disco.
- BitsPerCoordinate/Component/Flag = 8.
- Cor central = primeiro stop (fallback simples).
- Stream binary: 576 bytes para N=32.
- Flag=0 todos os triângulos (sem continuation optimization).

---

## §3 — Sub-passo P268.B — Anotação cumulativa ADR-0089 + L0

### B.1 — L0 prompt `entities/gradient.md`

Secção "Anotação P268 — PDF Conic shading (`/ShadingType 4`
Gouraud)" adicionada documentando estratégia Type 4 + cluster
3/3 completo.

### B.2 — ADR-0089 anotação cumulativa P268

Secção "Anotação cumulativa P268 — PDF Conic shading complete"
adicionada após anotação P265. Status `IMPLEMENTADO` preservado
literal.

### B.3 — Hashes propagados

`crystalline-lint --fix-hashes` propagou hash em
`entities/gradient.md`.

---

## §4 — Sub-passo P268.C — Materialização L3

### C.1 — Tipo `GradientObjectKind` expandido

```rust
enum GradientObjectKind {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),
    Conic(Arc<Conic>),     // P268 — novo variant
}
```

### C.2 — Helpers novos

**`oklab_sample_stops_conic`**: paridade literal
`oklab_sample_stops_radial` P265.

**`emit_conic_gouraud_stream(conic, n_slices)`**:
- Triangulação N=32 fatias (cap min 8).
- Cada triangle = (center, edge[i], edge[i+1]).
- Vertex bytes: flag (1) + x (1) + y (1) + R (1) + G (1) +
  B (1) = 6 bytes.
- Total stream: 18N bytes (576 para N=32; 144 para N=8).
- Cor central = primeiro stop; cores edges via
  `Conic::sample(i/N)` (Oklab L1).

### C.3 — `emit_gradient_objects` branching Conic

```rust
GradientObjectKind::Conic(conic) => {
    let stream = emit_conic_gouraud_stream(conic, 32);
    let header = format!(
        "<< /ShadingType 4 /ColorSpace /DeviceRGB \
           /BitsPerCoordinate 8 /BitsPerComponent 8 \
           /BitsPerFlag 8 \
           /Decode [0 1 0 1 0 1 0 1 0 1] \
           /Length {} >>\nstream\n", len);
    let mut shading_bytes = header.into_bytes();
    shading_bytes.extend_from_slice(&stream);
    shading_bytes.extend_from_slice(b"\nendstream");
    // Function placeholder (não usado Type 4).
    self.add(function_id, "<< /FunctionType 2 ... >>");
    self.add_bytes(shading_id, shading_bytes);
}
```

### C.4 — 3 sítios pattern-match substituídos

- `scan_all_gradients`: Conic registado via
  `GradientObjectKind::Conic(Arc::clone(c))`.
- `pattern_resources_for_page`: Conic emit resource entry.
- `emit_stroke_paint`: Conic emit `/Pattern CS /Pn SCN`
  (substituí fallback Solid P267).

### C.5 — 6 Tests P268

**Unit helpers** (3):
- `p268_oklab_sample_stops_conic_red_blue_endpoints`.
- `p268_emit_conic_gouraud_stream_n32_size` (576 bytes).
- `p268_emit_conic_gouraud_stream_min_8_slices` (clamp 144 bytes).

**E2E PDF** (3):
- `p268_export_pdf_conic_emits_shading_type_4` — confirma
  `/ShadingType 4` + `/BitsPerCoordinate/Component/Flag 8` +
  `/Decode` + `/Pattern <<` + `SCN`.
- `p268_export_pdf_conic_dedup_arc_ptr` — 3 shapes mesmo
  Arc<Conic> → 1 Shading dedup.
- `p268_export_pdf_cluster_3_variants_coexistem` — **marco
  P268**: Linear + Radial + Conic todos coexistem no mesmo
  doc.

### C.6 — Verificação intermediária

- `cargo build --workspace` → verde.
- `cargo test --workspace --release` → **2407 → 2413** (+6;
  zero regressões).
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P268.D — README + relatório

### D.1 — README ADRs

- ADR-0089 ganha cross-reference cumulativa P268 (anotação
  documental).
- Entrada P268 ~50 linhas nos passos-chave (paridade entrada
  P265).
- Distribuição preservada: PROPOSTO 11, EM VIGOR 32,
  IMPLEMENTADO 29, total 76.

### D.2 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Subpadrão "Dividir granularidade L1+stdlib / L3" N=2 → N=3 cumulativo completo

Cumulativo:
- N=1 P262/P263 (Linear).
- N=2 P264/P265 (Radial).
- **N=3 P267/P268** (Conic; **cluster Gradient encerrado**
  quanto a 3 variants base).

### Subpadrão "Anotação cumulativa em vez de ADR nova" N=4 → N=5

Cumulativo:
- N=1 P258.B (style_chain anotação Model).
- N=2 P259.B (geometry anotação Visualize).
- N=3 P263 (ADR-0087 anotação PDF Linear).
- N=4 P265 (ADR-0088 anotação PDF Radial).
- **N=5 P268** (ADR-0089 anotação PDF Conic).

### Subpadrão "Reutilização literal de helpers cross-passos" N=2 → N=3

Cumulativo:
- N=1 P265 (PDF Linear helpers reutilizados).
- N=2 P267 (Conic L1 helpers Oklab P262).
- **N=3 P268** (PDF Conic helpers cross-passos N=4 internos
  reutilizados: oklab_sample_stops_radial template + Conic::sample
  L1 P267 + interpolate_oklab P262 + ...).

### Subpadrão "Diagnóstico imutável precedente à acção" N=8 → N=9 cumulativo

Cumulativo:
- N=1-4 P255/P257/P258/P259 (audit Fase A).
- N=5 P262 (Linear vanilla).
- N=6 P264 (Radial vanilla).
- N=7 P266 (Text audit Fase A formal).
- N=8 P267 (Conic L1 vanilla).
- **N=9 P268** (Conic PDF vanilla — quarto consumo directo
  vanilla pós-P262/P264/P267).

---

## §7 — Cobertura

**Cluster Gradient pós-P268**:

| Variant | L1 | Stdlib | PDF |
|---------|----|----|-----|
| Linear | P262 ✓ | P262 ✓ | P263 ✓ `/ShadingType 2` |
| Radial | P264 ✓ | P264 ✓ | P265 ✓ `/ShadingType 3` |
| **Conic** | **P267 ✓** | **P267 ✓** | **P268 ✓ `/ShadingType 4`** |

**Cluster 3/3 completo em L1+stdlib+PDF**.

**Cobertura Visualize agregada**:
- Pre-P268: ~75% (P267 Conic L1+stdlib +2pp).
- **Pós-P268: ~78%** (+3pp via PDF Conic real Type 4 Gouraud).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P268 (não-bloqueantes)

**Cluster Gradient extensões**:
1. **P-Gradient-Focal** (M; activa `focal_center` +
   `focal_radius` Radial; revoga ADR-0088 §focal scope-out).
2. **P-Gradient-Space-Custom** (S+; activa `space: ColorSpace`
   cross-variant).
3. **P-Gradient-Relative-Custom** (M; activa `relative:
   RelativeTo`).

**Refinos qualitativos P268**:
4. **Anti-aliasing PDF** (default true; refino se controlo
   necessário).
5. **Spread mode repeat** (Type 4 não suporta; Pad implícito).
6. **N=32 fatias optimização** — fidelidade vs LOC tradeoff;
   refino futuro pode usar N adaptive baseado em radius/stops.

**Cluster outros Visualize**:
7. DEBT-33 Bézier bbox.
8. Stroke<Length>.
9. Curve variant + Polygon estrutural.
10. SVG image format.
11. Tiling pattern.

**Cluster Text refinos (P266)**:
12. C.5 Variant-aware font selection.
13. C.6 Font subsetting.

---

## §9 — Critério de aceitação global P268 — Checklist final

- [x] `crystalline-lint .` retorna `✓ No violations found`.
- [x] `cargo test --workspace --release` retorna **2413 verdes**
  (+6 vs baseline 2407; sem regressão).
- [x] `diagnostico-pdf-conic-passo-268.md` existe com §A.1-§A.12
  preenchidos.
- [x] ADR-0089 anotação cumulativa P268 adicionada.
- [x] `entities/gradient.md` L0 secção P268 anotada.
- [x] `03_infra/src/export.rs` ganha **2 helpers novos**
  (`oklab_sample_stops_conic` + `emit_conic_gouraud_stream`).
- [x] `GradientObjectKind` enum ganha variant `Conic(Arc<Conic>)`.
- [x] `emit_gradient_objects` ganha branching Conic → emit
  `/ShadingType 4`.
- [x] 3 sítios pattern-match P267 substituídos com emit real.
- [x] E2E tests confirmam bytes PDF (`/ShadingType 4` + Type 4
  Gouraud structure + dedup `Arc::as_ptr<Conic>` + cluster 3
  variants coexistem).
- [x] Tests unit confirmam helpers (oklab_sample_stops_conic
  + emit_conic_gouraud_stream N=32 / min 8).
- [x] Distribuição ADRs preservada total 76 (sem ADR nova).
- [x] **Paridade observable cumprida pós-P268**: PDFs com
  `#gradient.conic(...)` renderizam Type 4 Gouraud real (não
  fallback Solid).
- [x] Hashes propagados.
- [x] Relatório criado.

**Estado pós-P268**:
- Tests workspace: **2413 verdes** (+6 vs baseline 2407).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição preservada: PROPOSTO 11; EM VIGOR 32;
  IMPLEMENTADO 29; **total 76 preservado**.
- Prompts L0 editados: 1 (`entities/gradient.md`).
- Diagnóstico imutável criado: 1 (**quarto consumo directo
  vanilla** pós-P262/P264/P267).
- ADRs anotadas cumulativamente: 1 (ADR-0089).

**Marco P268**: **PDF Conic shading complete materializado**;
**promessa P267 fechada**; **cluster Gradient L1+stdlib+PDF
3/3 completo** (Linear + Radial + Conic via 3 ShadingTypes
distintos /ShadingType 2/3/4).

**Recomendação subjectiva pós-P268**:

- **P-Gradient-Focal** (M; activa focal_* Radial; revoga
  ADR-0088 §focal scope-out).
- **OU P266 Opção 1 Variant-aware fonts** (M; refino Text).
- **OU outras Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length>.
  - Curve variant + Polygon estrutural separada.
- **OU P-Footnote-N** refino M (P258 pendência).
- **OU Tiling activação** (Paint::Tiling).

**Decisão humana fica em aberto literal** pós-P268.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0089** §"Anotação cumulativa P268" — PDF Conic shading
  complete materializado.
- **ADR-0088** — Radial (precedente N=2 do pattern dividir
  granularidade; cluster Gradient cumulativo).
- **ADR-0087** §"Anotação cumulativa P263" — precedente
  directo template anotação cumulativa.
- ADR-0086 — Paint wrapper.
- ADR-0085 — Diagnóstico imutável (quarto consumo directo).
- ADR-0084 — Auditoria condicional.
- ADR-0083 — Color paridade.
- ADR-0027 — PDF objects estrutura.
- ADR-0018 — Whitelist crates externas (`krilla` não
  autorizada).
- ADR-0039 — TextStyle SR (preservado).
- ADR-0054 — Perfil graded.
- ADR-0061 — Granularidade 1-2 features/passo.
- ADR-0065 — Inventariar primeiro.
- ADR-0080 — L0 minimal.
- DEBT-1 (fechado P142; preservado).
- ISO 32000-1 §7.5.7 — Shading Patterns Type 4 (Free-Form
  Gouraud).
- P73 — Image stack dedup `Arc::as_ptr` (template arquitectural).
- P252 — Stroke `overhang`.
- P257 — Color paridade 8/8.
- P261 — Paint wrapper.
- P262 — Gradient Linear L1+stdlib.
- **P263** — Gradient Linear PDF (template).
- P264 — Gradient Radial L1+stdlib.
- **P265** — Gradient Radial PDF (template; helpers Oklab N=16).
- P266 — Text audit Fase A.
- **P267** — Gradient Conic L1+stdlib (precedente directo;
  promessa fechada por este passo).
- `00_nucleo/diagnosticos/diagnostico-pdf-conic-passo-268.md`
  — diagnóstico imutável P268.A.
- `00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md`
  — diagnóstico Conic L1 precedente.
- Vanilla `lab/typst-original/crates/typst-pdf/src/paint.rs:242-267`
  (Conic via krilla::SweepGradient não autorizada).
