# Relatório do passo P263 — PDF shading complete (fecha promessa P262)

**Data**: 2026-05-15.
**Spec**: `00_nucleo/materialization/typst-passo-263.md`.
**Tipo**: passo composto sequencial L3 cross-cutting; **subpadrão
emergente "P262/P263 dividir granularidade" N=1 inaugurado**.
**Análogo estrutural canónico**: P262 (L1+stdlib) → **P263
(L3 PDF dedicado)** — preserva ADR-0061 §"granularidade 1-2
features/passo".
**Magnitude planeada**: M (M+ cap; ~3-5h).
**Magnitude real**: **M (~2-3h)** — ~300 LoC L3 novas; 3 paths
adaptados; 8 tests P263; PDF emit branching unificado via helper.

---

## §1 — Sumário executivo

**Fase A confirmada**: 4 sítios `s.paint.to_color().to_rgba_f32()`
identificados; 3 paths `build_page_stream_*` (Helvetica + CIDFont
+ multifont); ImageRef/scan_all_images template reutilizado para
PatternRef/scan_all_gradients; `FrameItem::Shape.fill` é
`Option<Color>` (não Paint) — só stroke pode ter Gradient.

**Sem ADR nova** — ADR-0087 já cobre Gradient Linear; P263
materializa o backend PDF que ADR-0087 implicitamente apontava
via **anotação cumulativa** (paridade pattern P258.B/P259.B
"histórico cumulativo" ADR-0080 §"refactor aditivo").

**Tests delta**: **2361 → 2369** (+8 P263; zero regressões).

**ADRs distribuição**:
- PROPOSTO 11 (preservado).
- EM VIGOR 32 (preservado).
- IMPLEMENTADO 27 (preservado — anotação cumulativa não muda
  status).
- **Total 74** preservado.

**Ficheiros criados**:
- `00_nucleo/materialization/typst-passo-263-relatorio.md`
  (este ficheiro).

**Ficheiros editados**:
- `00_nucleo/prompts/infra/export.md` (secção P263 anotada;
  hash `6c3343df`).
- `03_infra/src/export.rs` (~300 LoC novas; tipos +
  scan_all_gradients + 4 helpers + emit_gradient_objects
  método PdfBuilder + emit_stroke_paint branching + 3 paths
  adaptados + 3 sítios stroke emit + 8 tests).
- `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`
  (secção "Anotação cumulativa P263 — PDF shading complete
  materializado" adicionada).
- `00_nucleo/adr/README.md` (entrada P263 ~50 linhas nos
  passos-chave; distribuição preservada).

**~4 ficheiros tocados; ~300 LoC L3 + 8 tests + L0 + ADR
anotação**.

---

## §2 — Sub-passo P263.A — Inventário inline (auto-aplicação ADR-0065)

### A.1 — Sítios cor actual

4 sítios `s.paint.to_color().to_rgba_f32()` no exporter:
- `build_page_stream_type1:1117` (Helvetica path principal).
- `draw_item_local:1453` (shapes em groups com `cm`).
- `build_page_stream_cidfont:1701`.
- `build_page_stream_multifont:1885`.

3 funções `build_page_stream_*` + 1 `draw_item_local` recursiva.

### A.2 — Paint::Gradient sítios

Zero hits estruturais pré-P263 (esperado — P262 deixou
`Paint::to_color()` fallback automatic). `FrameItem::Shape.fill:
Option<Color>` — **literal Color, não Paint**; só `stroke.paint:
Paint` pode ser Gradient.

### A.3 — Vanilla typst PDF emit

Vanilla typst usa crate `pdf-writer` externa (não comparable
directo). Spec ISO 32000 §7.5.7 é referência canónica.
**Decisão arquitectural P263**: emit manual sem `pdf-writer`
crate (mantém L3 puro manual per ADR-0027 precedente).

### A.4 — Decisões D1-D5 explicitadas

**D1 Function dict type**: ambos Type 2 (2 stops, exponential
linear `/N 1`) + Type 3 (stitching N>2) implementados para
cobertura completa.

**D2 ColorSpace Oklab → sRGB**: pré-amostragem N=16 stops em
Oklab via `Linear::sample(t)` L1 (P262); emit Type 3 stitching
linear sRGB. Aproxima Oklab via amostragem densa per ADR-0087
P262 user decision Q2.

**D3 Coords L3**: helper `compute_axial_coords(angle_rad, x0,
y0, w, h)` puro L3; semi-axes projection.

**D4 Dedup `Arc::as_ptr`**: HashMap chave
`Arc::as_ptr(linear) as usize` (paridade pattern P73
N=2 do template arquitectural — `image_resources` HashMap).

**D5 Cross-path cobertura**: 3 paths `build_page_stream_*`
(Helvetica + CIDFont + multifont) todos ganham branch
`Paint::Gradient` via helper `emit_stroke_paint` unificado.

---

## §3 — Sub-passo P263.B — L0 export.md actualizado

Secção nova "Suporte Gradient via Shading Patterns (Passo 263)"
adicionada ao fim de `00_nucleo/prompts/infra/export.md`:

- Pattern resources arquitectura.
- Shading Type 2 (axial) — único materializado P263.
- Function dicts (Type 2 + Type 3 stitching).
- Interpolação Oklab via amostragem densa.
- Page Resources dict (/Pattern condicional).
- Page stream emit Stroke branching.
- Coords L3 helper.
- 4 helpers internos novos enumerados.
- Object IDs allocation (3 por gradient único).
- Limitações P263 (Linear only; apenas Stroke; anti-alias
  true; relative bbox-local).

Hash propagado via `--fix-hashes`: `export.md` → `6c3343df`.

---

## §4 — Sub-passo P263.C — Materialização L3

### C.1 — Tipos novos

```rust
struct PatternRef {
    pattern_obj_id: usize,
    name:           String,
}

struct GradientObject {
    linear:      Arc<Linear>,
    function_id: usize,
    shading_id:  usize,
    pattern_id:  usize,
}
```

Paridade arquitectural com `ImageRef` (P73 N=1 do template).

### C.2 — `scan_all_gradients` pre-pass

```rust
fn scan_all_gradients(doc: &PagedDocument, first_id: usize)
    -> (Vec<PatternRef>, HashMap<usize, usize>, Vec<GradientObject>)
```

- Percorre `doc.pages.items` procurando `FrameItem::Shape {
  stroke: Some(Stroke { paint: Paint::Gradient(g), .. }), .. }`.
- Dedup via `Arc::as_ptr(linear) as usize`.
- Aloca 3 ObjectIDs por gradient único: Function + Shading +
  Pattern.
- Output paridade `scan_all_images` (P73 template).

### C.3 — Helpers L3

**`compute_axial_coords(angle_rad, x0, y0, w, h) -> (f64, f64, f64, f64)`**:
- Centro `(cx, cy) = (x0 + w/2, y0 + h/2)`.
- Direcção `(dx, dy) = (cos(angle), sin(angle))`.
- Endpoints: `(cx - (w/2)*dx, cy - (h/2)*dy)` →
  `(cx + (w/2)*dx, cy + (h/2)*dy)`.
- Algoritmo simplificado (semi-axes projection); paridade
  vanilla pode diferir (TODO refino futuro se divergência
  empírica).

**`oklab_sample_stops(linear, n=16) -> Vec<(f32, f32, f32)>`**:
- N pontos uniformemente em t ∈ [0, 1].
- Cada ponto: `linear.sample(t)` L1 (P262) → `Color` (Oklab
  interp); `Color::to_rgba_f32` → (r, g, b) sRGB.
- Clamp [0, 1].

**`emit_function_dict(stops, function_id, sub_first_id)`**:
- 2 stops → `/FunctionType 2 /Domain [0 1] /C0 [...] /C1 [...]
  /N 1` (linear exponential).
- N>2 stops → `/FunctionType 3` stitching com N-1 sub-Type-2.
- Retorna `(function_dict_string, sub_function_dicts)`.

### C.4 — Método PdfBuilder `emit_gradient_objects`

```rust
fn emit_gradient_objects(
    &mut self,
    grad_objs: Vec<GradientObject>,
    page_dimensions: &[(f64, f64)],
    next_sub_id: &mut usize,
)
```

Para cada gradient:
1. Amostrar 16 stops em Oklab.
2. Emit sub-Functions (se Type 3).
3. Emit Function dict principal.
4. Emit Shading dict (`/ShadingType 2 /ColorSpace /DeviceRGB
   /Coords [...] /Function id 0 R /Extend [false false]`).
5. Emit Pattern dict (`/PatternType 2 /Shading id 0 R /Matrix
   [1 0 0 1 0 0]`).

### C.5 — `emit_stroke_paint` helper branching unificado

```rust
fn emit_stroke_paint(
    ops: &mut String,
    paint: &Paint,
    thickness: f64,
    pat_ptr_to_idx: &HashMap<usize, usize>,
    pat_refs: &[PatternRef],
)
```

- `Paint::Solid(c)` → `r g b RG {w} w` literal P261 preservado.
- `Paint::Gradient(g)` → `/Pattern CS /P{name} SCN {w} w`.
- Fallback paranóide para Gradient não registado: `to_color()`
  + RG emit.

### C.6 — `pattern_resources_for_page` helper

```rust
fn pattern_resources_for_page(page, ptr_to_idx, refs) -> String
```

Retorna `/Pattern << /P1 X 0 R /P2 Y 0 R ... >>` ou string
vazia.

### C.7 — 3 paths build_helvetica/cidfont/multifont adaptados

Em cada path:
1. **Pre-pass `scan_all_gradients`** após `scan_all_images`.
2. **Allocar IDs gradient** após image IDs (`first_grad_id =
   first_img_id + img_xobjects.len() * 2 + 100`).
3. **Sub-function IDs** após gradient object IDs
   (`next_sub_id = first_grad_id + n_grads * 3`).
4. **Resources string** ganha `pat_res = pattern_resources_for_page(...)`.
5. **`build_page_stream_*` signature** ganha `pat_ptr_to_idx`
   + `pat_refs`.
6. **`emit_gradient_objects`** chamado depois das XObjects.

### C.8 — 3 sítios stroke emit principal adaptados

Sítios pré-P263 com `let (r, g, b, _) = s.paint.to_color()
.to_rgba_f32(); ops.push_str(&format!("... RG ..."))`
substituídos por `emit_stroke_paint(&mut ops, &s.paint,
s.thickness, pat_ptr_to_idx, pat_refs)`.

3 sítios cobertos:
- `build_page_stream_type1:~1117`.
- `build_page_stream_cidfont:~1701`.
- `build_page_stream_multifont:~1885`.

### C.9 — Scope-out adicional `draw_item_local`

`draw_item_local` (shape recursivo dentro de groups com `cm`
transformations) preserva fallback `first_stop_color` per
scope-out adicional — função interna sem acesso a
`pat_ptr_to_idx` no escopo; refactor para passar pattern
resources adiado (refino futuro se consumer real exigir).

### C.10 — 8 Tests P263 cumulativos

**Unit helpers** (5):
- `p263_compute_axial_coords_angle_0_horizontal`.
- `p263_compute_axial_coords_angle_90_vertical`.
- `p263_oklab_sample_stops_red_blue_endpoints`.
- `p263_emit_function_dict_2_stops_uses_type_2`.
- `p263_emit_function_dict_4_stops_uses_type_3_stitching`.

**E2E PDF** (3):
- `p263_export_pdf_gradient_in_stroke_emits_shading` — confirma
  `/ShadingType 2`, `/PatternType 2`, `/FunctionType`, `/Coords`,
  `/Pattern <<`, `SCN`.
- `p263_export_pdf_gradient_solid_preserva_rg_emit` — paridade
  P261 Solid path.
- `p263_export_pdf_gradient_dedup_arc_ptr` — 3 shapes com mesmo
  Arc<Linear> → 1 Shading object dedup.

### C.11 — Verificação intermediária

- `cargo build --workspace` → verde.
- `cargo test --workspace --release` → **2361 → 2369** (+8
  P263; zero regressões).
- `crystalline-lint --fix-hashes .` → `export.md` hash
  propagado para `6c3343df`.
- `crystalline-lint .` → `✓ No violations found`.

---

## §5 — Sub-passo P263.D — Anotação ADR-0087 + relatório

### D.1 — Anotação cumulativa ADR-0087

Secção nova "Anotação cumulativa P263 — PDF shading complete
materializado" adicionada após "Próximos passos":

- Componentes materializados em `03_infra/src/export.rs`
  (tipos + scan_all_gradients + 5 helpers + método
  emit_gradient_objects + emit_stroke_paint + 3 paths
  adaptados + 3 sítios stroke).
- Paridade observable cumprida pós-P263.
- Scope-out adicional pós-P263.C (draw_item_local + Fill).
- Decisões D1-D5 documentadas.
- 8 tests adicionais P263.
- Cobertura Visualize agregada (~58% → ~63%).
- Subpadrões cumulativos.

**Status `IMPLEMENTADO` preservado** — anotação cumulativa
não muda status (paridade pattern P258.B/P259.B "histórico
cumulativo" ADR-0080 §"refactor aditivo").

### D.2 — README ADRs

- Sem alteração de contagens (anotação cumulativa).
- Entrada P263 ~50 linhas nos passos-chave (paridade entrada
  P261/P262).
- Distribuição preservada: PROPOSTO 11, EM VIGOR 32,
  IMPLEMENTADO 27, total 74.

### D.3 — Relatório (este ficheiro)

Estrutura canónica §1-§10.

---

## §6 — Padrões metodológicos

### Subpadrão "Refactor cross-cutting entity primitivo" N=3 → N=4

Cumulativo:
- N=1 P252 (Stroke `overhang`).
- N=2 P257 (Color expansão).
- N=3 P261 (Paint wrapper).
- **N=4 P263** (PDF exporter cross-path Gradient emit; toca
  3 paths simultaneamente — Helvetica + CIDFont + multifont).

**Patamar N=4 reforça formalização**. Próxima aplicação
candidata: Stroke<Length> ou Tiling activação.

### Subpadrão emergente "P262/P263 dividir granularidade" N=1 inaugurado

Pattern emergente novo:
- **P262** = L1+stdlib materialização.
- **P263** = L3 PDF rendering dedicado.

Divisão preserva ADR-0061 §"granularidade 1-2 features/passo".
Candidato N=2 se Radial/Conic seguir mesmo padrão (P-Gradient-
Radial L1 + P-Gradient-Radial-PDF L3). Promoção formalização
adiada per política N=3-4.

### Anotação cumulativa vs ADR nova

P263 não cria ADR nova. ADR-0087 já cobre Gradient Linear
globalmente; P263 materializa o backend PDF que ADR-0087
implicitamente apontava. **Anotação cumulativa** documenta;
status `IMPLEMENTADO` preservado literal.

Paridade pattern P258.B/P259.B "histórico cumulativo" (ADR-0080
§"refactor aditivo").

### Subpadrão "Auto-aplicação ADR-0065 critério #5 inline" cumulativo

P263.A inventário inline no relatório §2 (sem ficheiro
separado). Paridade pattern P260.A — quando inventário é
trivial (mag XS), inline aceitável per ADR-0065 §"Neutras".

Cumulativo aplicações pattern P156K-style:
- N=1 P156K (ADR-0064 + ADR-0065).
- N=2 P160A (ADR-0066).
- N=3 P260 (ADR-0084 + ADR-0085).
- N=4 P263 (inventário inline pattern preservado).

---

## §7 — Cobertura

**Visualize agregado**:
- Pre-P262: ~52% (P259 audit).
- Pre-P263: ~58% (P262 Gradient L1+stdlib +5pp).
- **Pós-P263: ~63%** (+5pp via PDF render real Gradient
  Linear; F.1 Gradient Linear promovido `implementado+stdlib`
  → `implementado+stdlib+render`).

**Cobertura agregada user-facing total**: ~75-76% preservado.
**Layout Fase 5**: ~98-99% preservado.
**Math**: DEBT-8 ENCERRADO P255 preservado.
**Model**: ~73% pós-P258 preservado.
**Color** subsistema: 100% estrutural preservado.
**Visualize agregado**: **~63% pós-P263** (+10pp pós-P261;
+5pp pós-P262).

---

## §8 — Limitações e trabalho futuro

### Pendências residuais P263 (não-bloqueantes; candidatos passos dedicados)

**Cluster Gradient extensões**:
1. **P-Gradient-Radial** (M; replica P262+P263 pattern —
   L1+stdlib + L3 PDF dedicado; `/ShadingType 3`).
2. **P-Gradient-Conic** (M; replica template; baixa prioridade).

**Refinos qualitativos P263**:
3. **`draw_item_local` Gradient support** — refactor para
   passar `pat_ptr_to_idx` + `pat_refs` no escopo recursivo;
   actualmente fallback `first_stop_color` para shapes em
   groups com `cm` transformations.
4. **`FrameItem::Shape.fill: Option<Paint>`** — refino futuro
   se Fill Gradient for prioritário (actualmente Color
   literal).
5. **Coords algoritmo** — `compute_axial_coords` simplificado
   semi-axes projection; paridade vanilla pode diferir; refino
   se divergência empírica detectada.

**Cluster Stroke refinos (Opção 3 P259)**:
6. **DEBT-33 Bézier bbox exacto** (S+; ~+5 tests).
7. **Stroke<Length>** (M; ~+10-15 tests).
8. **Dash patterns**.
9. **LineCap/LineJoin/MiterLimit**.

**Cluster Shapes**:
10. **Polygon variant estrutural separada**.
11. **Curve variant**.

**Cluster Image (Opção 5)**:
12. **SVG image format** (L+; requer ADR `usvg`/`resvg`).
13. **Image metadata** `alt`/`fit` (S).

**Transform**:
14. **`origin` pivot** (scope-out ADR-0061 preservado).

**Tiling**:
15. **Tiling pattern** — pré-requisito Paint::Tiling activar.

### Sem ADR nova aberta

Política P158 "sem novas reservas" preservada. P263 usa
anotação cumulativa ADR-0087.

### Sem DEBT novo aberto

Saldo DEBTs preservado.

---

## §9 — Critério de aceitação global P263 — Checklist final

- [x] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [x] `cargo test --workspace --release` retorna **2369 verdes**
  (+8 vs baseline 2361; sem regressão).
- [x] Inventário P263.A inline no relatório §2 com decisões
  D1-D5 explicitadas.
- [x] `00_nucleo/prompts/infra/export.md` actualizado com
  secção P263; hash propagado (`6c3343df`).
- [x] `03_infra/src/export.rs` ganha **5 helpers + 1 método
  PdfBuilder** (`scan_all_gradients`,
  `pattern_resources_for_page`, `compute_axial_coords`,
  `oklab_sample_stops`, `emit_function_dict`,
  `emit_gradient_objects`, `emit_stroke_paint` +
  `emit_stroke_paint_type1` alias).
- [x] `pattern_resources: HashMap<usize, PatternRef>`
  funcional (dedup `Arc::as_ptr`).
- [x] 3 `build_page_stream_*` paths adaptados (Helvetica +
  CIDFont + multifont).
- [x] `/Resources /Pattern` dict condicional adicionado.
- [x] E2E tests confirmam bytes PDF (`/ShadingType 2`,
  `/PatternType 2`, `/Pattern <<`, `/Coords`, `SCN`, dedup
  `Arc::as_ptr`).
- [x] Tests unit confirmam helpers (compute_axial_coords
  angle 0/90; oklab_sample_stops red/blue; emit_function_dict
  Type 2 / Type 3 stitching).
- [x] ADR-0087 ganha secção "Anotação cumulativa P263".
- [x] **Paridade observable cumprida pós-P263**: PDF com
  `#gradient.linear(...)` renderiza gradient real via
  `/ShadingType 2` (não fallback).
- [x] Cross-path testado: 3 paths build_page_stream_*
  funcionais com gradient (zero regressões).
- [ ] **Stroke + Fill** — só Stroke ganha Paint::Gradient
  branching; **Fill continua Color literal** (scope-out
  adicional; refino futuro se Fill Paint estender).

**Estado pós-P263**:
- Tests workspace: **2369 verdes** (+8 vs baseline 2361).
- Hash drift: zero.
- Lint: zero violations.
- DEBTs saldo: **10 preservado**.
- ADRs distribuição preservada: PROPOSTO 11; IDEIA 2; EM VIGOR
  32; **IMPLEMENTADO 27**; REVOGADO 2; ADIADO 1; **total 74
  preservado**.
- Prompts L0 editados: 1 (`infra/export.md`).
- Diagnóstico imutável criado: 0 (P263 não é audit; ADR-0085
  scope é Fase A audit).
- ADRs anotadas cumulativamente: 1 (ADR-0087).
- **45 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.

**Marco P263**: **PDF shading complete materializado**;
**promessa P262 cumprida** (gradient real PDF render via
`/ShadingType 2 axial` + Function Type 3 stitching 16 stops
Oklab); granularidade ADR-0061 preservada via divisão
P262/P263. **Cross-path coverage cumprida** (Helvetica +
CIDFont + multifont todos com Gradient stroke funcional).

**Recomendação subjectiva pós-P263**:

- **P-Gradient-Radial** (M; replica template P262+P263 —
  L1+stdlib + L3 PDF `/ShadingType 3` dedicado; activa
  `Gradient::Radial` variant; cobertura Visualize +5pp).
- **OU outras Opções P259 alternativas**:
  - DEBT-33 Bézier bbox + Stroke<Length> Opção 3.
  - Curve variant + Polygon estrutural separada Opção 2.
- **OU Text audit** (segundo audit pós-formalização P260 —
  consumo directo ADR-0084 + 0085).
- **OU P-Footnote-N** refino M (P258 pendência residual).
- **OU Tiling activação** (Paint::Tiling — análogo P262/P263
  estrutural).

**Decisão humana fica em aberto literal** pós-P263.

---

## §10 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- **ADR-0087** §"Anotação cumulativa P263" — PDF shading
  complete materializado (criada por este passo).
- **ADR-0086** — Paint wrapper Solid only (P261; Paint chain
  consumer pre-pronta).
- **ADR-0084, ADR-0085** — Auditoria condicional + diagnóstico
  imutável (P260; consumido metodologicamente via inventário
  inline P263.A).
- ADR-0027 — CIDFont/Identity-H (precedente arquitectural
  estrutura objectos PDF; template directo `build_cidfont`
  paridade `build_helvetica`).
- ADR-0029 — Pureza física L1 (regra geral; P263 é L3 — não
  obriga diagnóstico vanilla).
- ADR-0033 — Paridade observable vanilla (gradient real
  cumpre paridade user-facing).
- ADR-0034 — Diagnóstico canónico.
- ADR-0039 — TextStyle SR (preservado literal).
- ADR-0054 — Perfil graded (ColorSpace Oklab interpolação
  pre-render é divergência aceite).
- ADR-0061 — Granularidade 1-2 features/passo (cumprido via
  divisão P262/P263).
- ADR-0065 — Inventariar primeiro (cumprido inline P263.A).
- DEBT-1 — Fechado P142 (preservado).
- ISO 32000-1 §7.5.7 — Shading patterns (referência canónica
  spec PDF).
- **P73** — Image stack dedup `Arc::as_ptr` (template
  arquitectural `pattern_resources` N=1 do pattern; **P263
  N=2**).
- P74 — PNG `/SMask` (precedente cross-path resource cascade
  Helvetica+CIDFont).
- P257 — Color paridade vanilla 8/8 (ADR-0083; `Color::oklab`
  + `to_rgba_f32` consumidos).
- P261 — Paint wrapper Solid only (consumer chain Stroke.paint
  pre-pronta).
- **P262** — Gradient L1+stdlib (precedente directo; este
  passo é continuação directa; **divide granularidade** N=1
  inaugurada).
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico imutável precedente (consumido P263).
- `00_nucleo/prompts/infra/export.md` — L0 prompt actualizado
  por este passo (secção P263 anotada; hash `6c3343df`).
- `03_infra/src/export.rs` — código L3 alterado por este
  passo (~300 LoC novas; tipos + scan + 5 helpers + método
  + branching emit + 3 paths + 8 tests).
