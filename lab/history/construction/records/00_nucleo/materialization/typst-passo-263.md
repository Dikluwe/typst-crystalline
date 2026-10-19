# Passo 263 — PDF shading complete (fecha promessa P262; gradient real PDF render)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial **L3 cross-cutting**;
magnitude estimada **M (M+ cap; ~3-5h)**.
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- ADR-0027 — CIDFont/Identity-H (estrutura objectos PDF;
  precedente arquitectural directo).
- ADR-0029 (regra geral — não obriga diagnóstico vanilla
  aqui pois L3 emit é detalhe de implementação; estrutura
  PDF é spec ISO).
- ADR-0033 (paridade observable — gradient real PDF render
  agora cumpre paridade vanilla user-facing).
- ADR-0054 (perfil graded — ColorSpace Oklab interpolação
  pre-render é divergência aceite).
- ADR-0065 (inventariar primeiro).
- **ADR-0087** (Gradient Linear-only IMPLEMENTADO P262; este
  passo cumpre o §"Critério revisão" implícito sobre PDF
  shading).
- **ADR-0083** (Color paridade — `to_srgb`/`to_rgba_f32` API
  preservada; usada por sample Oklab → sRGB para PDF emit).
- Relatórios precedentes:
  - P73 (image stack — precedente `image_resources` HashMap
    `Arc::as_ptr`; template arquitectural para
    `pattern_resources`).
  - P74 (PNG `/SMask` — precedente cascade resources cross-path
    Helvetica+CIDFont).
  - P262 (Gradient L1+stdlib; **deixou fallback `first_stop_color`
    no PDF**; este passo substitui por shading real).

**Outputs canónicos esperados** ao fim do passo:
- Eventual ADR **anotação cumulativa ADR-0087** com secção
  "Anotação P263 — PDF shading complete materializado"
  documentando rendering real (não nova ADR; ADR-0087 já
  cobre Gradient Linear; este passo materializa o backend
  PDF que ADR-0087 §"Critério revisão" implicitamente
  apontava).
- Código L3 actualizado em `03_infra/src/export.rs`:
  - `pattern_resources: HashMap<usize, PatternResource>`.
  - Função `emit_axial_shading_object(linear, page_bbox) -> ObjectId`.
  - Função `emit_function_dict(stops) -> ObjectId` (Type 2
    ou Type 3 conforme N stops).
  - Função `emit_pattern_dict(shading_id) -> ObjectId`.
  - Helper `compute_axial_coords(angle, bbox)` puro.
  - `build_page_stream_*` 3 caminhos adaptados com branch
    `Paint::Gradient` emit `/Pattern cs; /Pn scn`.
  - Page `/Resources` dict ganha `/Pattern` entry condicional.
- Prompt L0 actualizado: `00_nucleo/prompts/infra/export.md`
  com secção "Anotação P263 — Suporte Gradient via Shading
  Patterns" (estilo cumulativo P258/P259/P261).
- Hash propagado via `--fix-hashes`.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-263-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Regra de Ouro CLAUDE.md** — prompt L0 `export.md`
   actualizado **antes** de código alterado. Ordem:
   diagnóstico inventário → L0 actualização → fix-hashes →
   testes-primeiro → código L3.
2. **Sem ADR nova** — ADR-0087 já cobre Gradient Linear
   materialização global. P263 é **execução do critério
   revisão implícito** (PDF rendering real). **Anotação
   cumulativa ADR-0087** documenta backend PDF complete;
   paridade pattern P258.B/P259.B "histórico cumulativo".
3. **Ordem testes-primeiro** — para cada função emit nova:
   tests antes de implementação. PDF E2E tests confirmam
   bytes esperados (`/ShadingType 2`, `/Coords`, `/Function`).
4. **`crystalline-lint .`** zero violations no fim.
5. **Tests workspace** sem regressão (baseline 2361 pós-P262).
   Esperado **+8-15** (5-8 E2E PDF + 3-7 unit helpers).
6. **Materialization é leitura proibida por iniciativa
   própria**.
7. **Política "sem novas reservas"** preservada — Radial/Conic
   `/ShadingType 3`+/Conic continuam comentários reserva em
   gradient.rs; PDF emit prepara apenas axial Type 2.
8. **Paridade observable user-facing P262 promessa cumprida** —
   `#gradient.linear(...)` produzia fallback Solid no PDF;
   pós-P263 renderiza gradient real.

---

## §1 — Sub-passo P263.A: Inventário inline (auto-aplicação ADR-0065 critério #5)

**Objectivo**: inventariar estrutura actual do exporter +
identificar pontos exactos de intervenção.

**Materialização**: zero código novo. Apenas leitura.

**Forma**: inline no relatório §2 (sem ficheiro imutável
separado — magnitude trivial per ADR-0065 §"Neutras";
precedente P260.A).

### Acções obrigatórias

#### A.1 — Localizar sítios cor actual

```bash
# 4 sítios paint.to_color.to_rgba_f32 (P261)
grep -n "to_color\(\)\.to_rgba_f32\|to_rgba_f32" \
  03_infra/src/export.rs

# Helpers build_page_stream_*
grep -n "fn build_page_stream\|fn emit_\|fn render_" \
  03_infra/src/export.rs

# Resources dict actual
grep -n "/Resources\|/Font\|/XObject" 03_infra/src/export.rs | head -20

# Object IDs allocation pattern
grep -n "next_id\|object_id\|obj_id\b" 03_infra/src/export.rs | head -10
```

**Output esperado**:
- 4 sítios `to_rgba_f32` na chain `Paint::to_color()`.
- 3 funções `build_page_stream_*` (Helvetica, CIDFont,
  multifont).
- `/Resources` dict construction reutilizável.
- Allocation de ObjectId via contador.

#### A.2 — Inventário Paint::Gradient sítios

```bash
# Onde Paint pode ser Gradient (Stroke.paint)
grep -n "Paint::Gradient\|matches!.*Gradient" 03_infra/src/

# FrameItem::Shape (assumido nome — confirmar)
grep -n "FrameItem::Shape\|frameitem.*shape" \
  01_core/src/entities/layout_types.rs
```

**Output esperado**:
- Sítios `FrameItem::Shape { .., paint }` onde Paint pode
  ser Gradient.
- Confirmar nome exacto do variant FrameItem.

#### A.3 — Vanilla PDF emit referência

```bash
# Como vanilla typst emite shading (se disponível em lab/)
grep -rn "ShadingType\|axial_shading\|emit_pattern" \
  lab/typst-original/crates/typst-pdf/ 2>/dev/null | head -20

# Spec PDF ISO 32000 (referência teórica; sem fetch)
# /ShadingType 2 axial — Function-Based Type 2 (2 stops)
#                       ou Type 3 stitching (multi-stop)
```

**Critério**: vanilla typst usa crate `pdf-writer` externa
(não comparable directo); spec ISO 32000 §7.5.7 é referência
canónica. **Decisão arquitectural P263**: emit manual (sem
`pdf-writer` crate; mantém L3 puro manual per ADR-0027
precedente).

#### A.4 — Decisão arquitectural (registar inline §2)

Decisões a tomar:

**Decisão D1 — Function dict type**:
- 2 stops → `/FunctionType 2` (exponential) — simples.
- N>2 stops → `/FunctionType 3` (stitching) — array de N-1
  Type 2 sub-functions.
- **Recomendação**: implementar ambas para cobertura completa.

**Decisão D2 — ColorSpace Oklab → sRGB**:
- ADR-0087 §"Critério revisão" preserved Oklab interpolation
  (P262 sample Oklab).
- **Decisão**: PDF emit pre-samples N intermediate stops (e.g.
  16) em Oklab, emite Type 3 stitching com sub-functions Type 2
  entre cada par adjacente em sRGB. Aproxima Oklab via amostragem
  densa.
- **Alternativa rejeitada**: emit Type 2 directo com endpoint
  colors sRGB — perde interpolação Oklab (vanilla parity falha).

**Decisão D3 — Coords L3**:
- `compute_axial_coords(angle: Angle, bbox: Rect) -> (P0, P1)`
  helper L3 puro.
- bbox em coordenadas locais do shape (não da página).
- Inversão Y do PDF aplica-se após L3 emite Coords locais.

**Decisão D4 — Dedup por `Arc::as_ptr`**:
- Pattern P73 image_resources reaplicado: HashMap chave
  `Arc::as_ptr(linear) as usize`.
- Múltiplos shapes com mesmo `Arc<Linear>` partilham 1
  Pattern object.

**Decisão D5 — Cross-path cobertura**:
- 3 caminhos build_page_stream_* todos ganham branch
  `Paint::Gradient`.
- Helper `emit_paint_fill(paint, ...) -> String` unifica
  emit Color sólida vs Pattern reference.

### Critério de aceitação P263.A

- Inventário §A.1/A.2/A.3 documentado inline no relatório §2.
- Decisões D1-D5 explicitadas com justificação.
- Zero alterações em código (ainda).

---

## §2 — Sub-passo P263.B: Actualizar L0 prompt `infra/export.md`

**Objectivo**: cumprir Regra de Ouro — L0 actualizado antes do
código.

**Materialização**: edição L0 prompt + `--fix-hashes`. Sem
código L3 alterado ainda.

### B.1 — Adicionar secção "Anotação P263"

`00_nucleo/prompts/infra/export.md` ganha secção:

```markdown
## Suporte Gradient via Shading Patterns (Passo 263)

`FrameItem::Shape { paint: Paint::Gradient(g), ... }` renderiza
via PDF shading patterns (ISO 32000 §7.5.7).

### Pattern resources

`pattern_resources: HashMap<usize, PatternResource>` mapa
deduplicado por `Arc::as_ptr(linear) as usize` — paridade
arquitectural `image_resources` P73.

Cada `PatternResource`:
- `shading_id: ObjectId` — Shading dict object.
- `pattern_id: ObjectId` — Pattern dict object referenciando shading.
- `function_id: ObjectId` — Function dict object para interpolação.

### Shading Type 2 (axial) — único materializado P263

`/ShadingType 2` axial:
- `/ColorSpace /DeviceRGB`.
- `/Coords [x0 y0 x1 y1]` — endpoints linha gradient (locais).
- `/Function obj_ref` — Type 2 (2 stops) ou Type 3 stitching (N>2).
- `/Extend [false false]`.

### Function dicts

**Type 2** (exponential interpolation, 2 stops):
```
/FunctionType 2
/Domain [0 1]
/C0 [r0 g0 b0]    % cor inicial em sRGB
/C1 [r1 g1 b1]    % cor final em sRGB
/N 1              % linear
```

**Type 3** (stitching, N>2 stops):
```
/FunctionType 3
/Domain [0 1]
/Functions [/F2obj /F2obj ...]   % N-1 Type 2 sub-funcs
/Bounds [t1 t2 ... t_{N-1}]     % offsets internos
/Encode [0 1 0 1 ...]           % per sub-function
```

### Interpolação Oklab via amostragem densa

Vanilla Gradient interpola em Oklab (ADR-0087 §C.2 cumprida P262).
PDF Type 2/3 nativos não suportam Oklab. **Aproximação**:
pré-amostragem em Oklab → N≈16 stops intermédios em sRGB →
Type 3 stitching linear.

`Linear::sample(t)` é o helper L1 que produz Color em sRGB pós
interpolação Oklab.

### Page Resources dict

Páginas com pelo menos 1 Gradient ganham:
```
/Resources <<
  /Font << ... >>
  /Pattern << /P1 obj_ref /P2 obj_ref ... >>
>>
```

### Page stream emit

Substitui `r g b rg` por:
```
/Pattern cs       % set non-stroke colour space
/P1 scn           % apply pattern P1
```

Para stroke:
```
/Pattern CS
/P1 SCN
```

Cross-path: aplicável a `build_page_stream_type1`,
`build_page_stream_cidfont`, e helper multifont.

### Coords L3

Helper `compute_axial_coords(angle, bbox) -> (P0, P1)`:
- Angle 0° → (left, mid_y) → (right, mid_y).
- Angle 90° → (mid_x, bottom) → (mid_x, top).
- Generalização: projecção do angle no rectangle.

### Limitações P263

- **Linear only** — Radial/Conic continuam comentários reserva
  em `entities/gradient.rs`; PDF emit prepara só Type 2.
- **Anti-alias assume true** (PDF default; vanilla scope-out
  ADR-0087).
- **Relative assume bounding-box** (vanilla default; scope-out
  ADR-0087).

### Helpers internos novos (pub na crate)

| Função | Responsabilidade |
|--------|------------------|
| `compute_axial_coords(angle, bbox)` | (P0, P1) endpoints da linha gradient |
| `oklab_sample_stops(linear, n_samples)` | N stops intermédios em sRGB pós Oklab |
| `emit_function_dict(stops) -> ObjectId` | Type 2 ou Type 3 conforme N stops |
| `emit_axial_shading_object(linear, bbox) -> ObjectId` | Shading dict referenciando function |
| `emit_pattern_dict(shading_id) -> ObjectId` | Pattern dict referenciando shading |
| `emit_paint_fill(paint, ...) -> String` | Helper unificado Color/Pattern fill ops |
```

### B.2 — Propagar hash

```bash
cargo run -p crystalline-lint -- --fix-hashes .
```

### B.3 — Verificação

```bash
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
cargo test --workspace --release
# Esperado: 2361 preservado (passo só L0)
```

### Critério de aceitação P263.B

- `export.md` actualizado com secção P263.
- Hash propagado.
- Tests workspace inalterados.
- Zero violations.

---

## §3 — Sub-passo P263.C: Materialização L3

**Ordem obrigatória — testes primeiro per CLAUDE.md**.

### C.1 — Tests primeiro

#### C.1.1 — Unit tests helpers

`03_infra/src/export.rs` (tests submodule):

```rust
#[test]
fn compute_axial_coords_angle_0_horizontal() {
    let bbox = Rect { x0: 0.0, y0: 0.0, x1: 100.0, y1: 50.0 };
    let (p0, p1) = compute_axial_coords(Angle::deg(0.0), bbox);
    assert!((p0.x - 0.0).abs() < 0.01);
    assert!((p0.y - 25.0).abs() < 0.01);
    assert!((p1.x - 100.0).abs() < 0.01);
    assert!((p1.y - 25.0).abs() < 0.01);
}

#[test]
fn compute_axial_coords_angle_90_vertical() { ... }
#[test]
fn compute_axial_coords_angle_45_diagonal() { ... }

#[test]
fn oklab_sample_stops_2_stops_red_blue() {
    let linear = Linear {
        stops: Arc::new([
            GradientStop { color: Color::rgb(255, 0, 0), offset: Some(Ratio(0.0)) },
            GradientStop { color: Color::rgb(0, 0, 255), offset: Some(Ratio(1.0)) },
        ]),
        angle: Angle::deg(0.0),
    };
    let samples = oklab_sample_stops(&linear, 16);
    assert_eq!(samples.len(), 16);
    // Primeiro stop = vermelho puro.
    assert_eq!(samples[0].color.to_srgb(), (255, 0, 0, 255));
    // Último stop = azul puro.
    assert_eq!(samples[15].color.to_srgb(), (0, 0, 255, 255));
}

#[test]
fn emit_function_dict_2_stops_uses_type_2() { ... }
#[test]
fn emit_function_dict_3_stops_uses_type_3_stitching() { ... }
```

#### C.1.2 — E2E PDF tests

```rust
#[test]
fn export_pdf_gradient_linear_2_stops_emits_shading_object() {
    let linear = Gradient::linear(
        vec![
            GradientStop { color: Color::rgb(255, 0, 0), offset: Some(Ratio(0.0)) },
            GradientStop { color: Color::rgb(0, 0, 255), offset: Some(Ratio(1.0)) },
        ],
        Angle::deg(0.0),
    );
    let stroke = Stroke {
        paint: Paint::Gradient(linear),
        thickness: 2.0,
        overhang: false,
    };
    // Construir Frame com Shape { paint: Paint::Gradient(...) }
    // ...
    let pdf = export_pdf(&doc);
    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(pdf_str.contains("/ShadingType 2"));
    assert!(pdf_str.contains("/FunctionType"));  // 2 ou 3
    assert!(pdf_str.contains("/Pattern"));
    assert!(pdf_str.contains("/Coords"));
}

#[test]
fn export_pdf_gradient_in_resources_dict() { ... }
#[test]
fn export_pdf_gradient_deduplication_arc_ptr() {
    // Mesmo Arc<Linear> usado 3 vezes → 1 Shading object.
}
#[test]
fn export_pdf_gradient_angle_90_coords_vertical() { ... }
#[test]
fn export_pdf_gradient_helvetica_path_works() {
    // Caminho Helvetica (sem TrueType) também emite Pattern correctamente.
}
#[test]
fn export_pdf_gradient_cidfont_path_works() {
    // Caminho CIDFont com gradient num shape.
}
```

Executar `cargo test export::gradient` ou similar — verificar
falham.

### C.2 — Implementação L3

#### C.2.1 — Tipo `PatternResource`

```rust
// 03_infra/src/export.rs

struct PatternResource {
    shading_id:  ObjectId,
    pattern_id:  ObjectId,
    function_id: ObjectId,
}
```

#### C.2.2 — Helper `compute_axial_coords`

```rust
fn compute_axial_coords(angle: Angle, bbox: Rect) -> ((f64, f64), (f64, f64)) {
    let theta = angle.to_rad();
    let cx = (bbox.x0 + bbox.x1) / 2.0;
    let cy = (bbox.y0 + bbox.y1) / 2.0;
    let half_w = (bbox.x1 - bbox.x0) / 2.0;
    let half_h = (bbox.y1 - bbox.y0) / 2.0;
    // Projecção: para angle 0° linha horizontal através do centro
    let dx = theta.cos();
    let dy = theta.sin();
    // Encontrar intersecção da linha (cx, cy) + t*(dx, dy) com bbox
    // (algoritmo standard — recomendação simples: usar bbox edges)
    // ...
    ((cx - half_w * dx, cy - half_h * dy),
     (cx + half_w * dx, cy + half_h * dy))
}
```

**Decisão**: implementação simplificada baseado em bbox semi-axes.
Algoritmo paridade vanilla **pode divergir** se vanilla usar
"longest projection" — verificar empíricamente em E2E test
quando `angle = 30°` vs vanilla output.

#### C.2.3 — Helper `oklab_sample_stops`

```rust
fn oklab_sample_stops(linear: &Linear, n_samples: usize) -> Vec<GradientStop> {
    (0..n_samples)
        .map(|i| {
            let t = i as f32 / (n_samples - 1).max(1) as f32;
            let color = linear.sample(t);  // L1 helper P262
            GradientStop { color, offset: Some(Ratio(t as f64)) }
        })
        .collect()
}
```

**N=16 default** — equilíbrio entre fidelidade Oklab e tamanho
PDF.

#### C.2.4 — Função emit Function dict

```rust
fn emit_function_dict(stops: &[GradientStop], next_id: &mut u32) -> ObjectId {
    if stops.len() == 2 {
        // Type 2 exponential, linear (N=1)
        let id = *next_id; *next_id += 1;
        // ... emit /FunctionType 2 /Domain [0 1] /C0 [...] /C1 [...] /N 1
        id
    } else {
        // Type 3 stitching: N-1 Type 2 sub-functions
        // Emit cada Type 2 primeiro; depois Type 3 a referenciar
        // ...
    }
}
```

#### C.2.5 — Função emit Shading object

```rust
fn emit_axial_shading_object(
    linear: &Linear,
    bbox: Rect,
    next_id: &mut u32,
) -> (ObjectId, ObjectId) {  // (shading_id, function_id)
    let samples = oklab_sample_stops(linear, 16);
    let function_id = emit_function_dict(&samples, next_id);
    let ((x0, y0), (x1, y1)) = compute_axial_coords(linear.angle, bbox);
    let shading_id = *next_id; *next_id += 1;
    // emit /ShadingType 2 /ColorSpace /DeviceRGB /Coords [...] /Function function_id
    (shading_id, function_id)
}
```

#### C.2.6 — Função emit Pattern dict

```rust
fn emit_pattern_dict(shading_id: ObjectId, next_id: &mut u32) -> ObjectId {
    let pattern_id = *next_id; *next_id += 1;
    // emit /PatternType 2 /Shading shading_id /Matrix [1 0 0 1 0 0]
    pattern_id
}
```

#### C.2.7 — `build_page_stream_*` adaptados

Três pontos de modificação (Helvetica + CIDFont + multifont):

```rust
// Pseudocódigo unificado
fn emit_paint_fill(paint: &Paint, pattern_resources: &HashMap<usize, PatternResource>) -> String {
    match paint {
        Paint::Solid(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{r} {g} {b} rg")
        }
        Paint::Gradient(g) => {
            let arc_ptr = match g {
                Gradient::Linear(l) => Arc::as_ptr(l) as usize,
            };
            let resource = pattern_resources.get(&arc_ptr)
                .expect("pattern not registered before emit");
            format!("/Pattern cs\n/P{} scn", resource.pattern_id_index)
        }
    }
}
```

Cada `build_page_stream_*` precisa de **pre-pass** que regista
todos os gradients no `pattern_resources` antes de emit do
stream.

#### C.2.8 — `/Resources` dict adaptado

Páginas com pelo menos 1 Gradient ganham:

```
/Resources <<
  /Font << ... >>
  /Pattern << /P1 N M R /P2 N M R ... >>
  /ColorSpace << ... >>
>>
```

### C.3 — Verificação intermediária

```bash
cargo build --workspace
# Esperado: verde
RUST_MIN_STACK=33554432 cargo test --workspace --release
# Esperado: 2361 → 2369-2376 (+8-15 P263)
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
```

### Critério de aceitação P263.C

- 4 helpers L3 novos implementados (`compute_axial_coords`,
  `oklab_sample_stops`, `emit_function_dict`,
  `emit_axial_shading_object`, `emit_pattern_dict`).
- 3 `build_page_stream_*` adaptados com branch Gradient.
- `pattern_resources` HashMap dedup `Arc::as_ptr` (paridade
  P73).
- `/Resources /Pattern` dict adicionado condicional.
- Tests workspace **2361 → 2369-2376** (+8-15).
- Zero violations.
- E2E tests confirmam bytes PDF esperados.

---

## §4 — Sub-passo P263.D: Anotação ADR-0087 cumulativa + relatório

### D.1 — Anotação cumulativa ADR-0087

`00_nucleo/adr/typst-adr-0087-gradient-linear-only.md` ganha
secção:

```markdown
## Anotação cumulativa P263 — PDF shading complete materializado

(Data 2026-05-15)

PDF rendering real Gradient Linear materializado. Substitui
fallback `first_stop_color` introduzido em P262.

### Componentes materializados

- `pattern_resources` HashMap dedup `Arc::as_ptr` (paridade P73).
- `compute_axial_coords` (L3 puro; bbox locais).
- `oklab_sample_stops` (N=16 pre-sample em Oklab via
  `Linear::sample`; emit sRGB).
- `emit_function_dict` (Type 2 para 2 stops; Type 3 stitching
  para N>2).
- `emit_axial_shading_object` (`/ShadingType 2`).
- `emit_pattern_dict` (`/PatternType 2`).
- `emit_paint_fill` helper unificado (Solid `rg` vs Gradient
  `/P{n} scn`).
- 3 `build_page_stream_*` paths actualizados (Helvetica +
  CIDFont + multifont).
- `/Resources /Pattern` dict condicional.

### Paridade observable cumprida pós-P263

`#gradient.linear(red, blue, angle: 90deg)` em Stroke ou Fill
agora renderiza gradient real no PDF (não fallback Solid).

### Scope-outs preservados

- Radial/Conic continuam comentários reserva em
  `entities/gradient.rs` (passos dedicados futuros).
- ColorSpace Oklab via pré-amostragem N=16 (não nativo PDF).
- Anti-alias true assumed (default PDF).

Cross-references:
- L3 emit: `03_infra/src/export.rs` (~200-300 LoC novas).
- Tests E2E: confirmam `/ShadingType 2`, `/Pattern`, `/Coords`,
  dedup `Arc::as_ptr`.
- ADR-0027 (precedente estrutura objectos PDF).
- P262 (precedente Gradient L1+stdlib).
```

**Status `IMPLEMENTADO` preservado** — anotação cumulativa não
muda status; refina aplicação.

### D.2 — README ADRs

Sem alteração de contagens. Linha ADR-0087 ganha referência
cumulativa P263 análoga ADR-0083 pós-P259.

### D.3 — Subpadrões cumulativos

**"Refactor cross-cutting entity primitivo" N=3 → N=4**:
- N=1 P252 Stroke `overhang`.
- N=2 P257 Color expansão.
- N=3 P261 Paint wrapper.
- **N=4 P263** PDF exporter expand cross-path (Helvetica +
  CIDFont + multifont todos adaptados).

Patamar N=4 reforça formalização — pattern emergente sólido.

**Subpadrão emergente "P262/P263 dividir granularidade"** —
P262 fez L1+stdlib; P263 fez L3. Paridade pattern P156K
estilo "passo administrativo divide para preservar
granularidade ADR-0061". Precedente N=1 (poderá repetir-se
em Radial L1+stdlib + Radial L3 dedicado).

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-263-relatorio.md`
estrutura canónica:

- **§1 Sumário executivo** — magnitude real; tests delta;
  ADR anotação; ficheiros tocados.
- **§2 P263.A** — inventário inline + decisões D1-D5.
- **§3 P263.B** — L0 export.md actualizado.
- **§4 P263.C** — código L3 materializado.
- **§5 P263.D** — anotação ADR-0087.
- **§6 Padrões metodológicos** — "Refactor cross-cutting"
  N=3 → 4; "P262/P263 dividir granularidade" N=1.
- **§7 Cobertura** — Visualize ~58% → **~63%** (+5pp via
  PDF render real Gradient).
- **§8 Limitações e trabalho futuro** — Radial/Conic dedicados;
  refinos Coords vs vanilla (se divergência empírica detectada).
- **§9 Critério de aceitação global P263 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P263.D

- ADR-0087 anotada (sem alteração de status).
- README ADRs cross-reference cumulativa.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P263

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [ ] `cargo test --workspace --release` retorna ≥ 2361 + 8-15
  = 2369-2376 (sem regressão).
- [ ] Inventário P263.A inline no relatório §2 com decisões
  D1-D5 explicitadas.
- [ ] `00_nucleo/prompts/infra/export.md` actualizado com
  secção P263; hash propagado.
- [ ] `03_infra/src/export.rs` ganha 5 helpers novos
  (`compute_axial_coords`, `oklab_sample_stops`,
  `emit_function_dict`, `emit_axial_shading_object`,
  `emit_pattern_dict`).
- [ ] `pattern_resources: HashMap<usize, PatternResource>`
  funcional.
- [ ] 3 `build_page_stream_*` paths adaptados.
- [ ] `/Resources /Pattern` dict condicional adicionado.
- [ ] E2E tests confirmam bytes PDF (`/ShadingType 2`,
  `/Pattern`, `/Coords`, dedup `Arc::as_ptr`).
- [ ] Tests unit confirmam helpers (compute_axial_coords,
  oklab_sample_stops, emit_function_dict).
- [ ] ADR-0087 ganha secção "Anotação cumulativa P263".
- [ ] **Paridade observable cumprida pós-P263**: PDFs com
  `#gradient.linear(...)` renderizam gradient real (não
  fallback).
- [ ] Cross-path testado: Helvetica + CIDFont + multifont
  todos com gradient funcional.
- [ ] **Stroke + Fill** ambos com Paint::Gradient funcionais
  (se Fill estender consumer fora ADR-0039 — verificar; pode
  ser apenas Stroke se ADR-0039 preservada).

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, ADR-0027 (precedente arquitectural),
   ADR-0083/0086/0087, relatórios P73 (image dedup template),
   P261 (Paint chain), P262 (Gradient L1).
2. **Reportar** estado inicial: tests 2361 + lint baseline +
   ADRs 74.
3. **P263.A** — Executar grep inventário; decisões D1-D5
   inline no relatório draft.
4. **P263.B** — Editar `export.md` com secção P263; propagar
   hash; verificar lint limpo + tests preservados.
5. **P263.C** — Tests primeiro (unit helpers + E2E PDF);
   implementação 5 helpers + 3 paths adaptados +
   `pattern_resources` HashMap + `/Resources /Pattern`;
   verificar tests verdes + lint limpo.
6. **P263.D** — Anotar ADR-0087 cumulativamente; actualizar
   README ADRs cross-reference; criar relatório.
7. **Verificação final** — checklist §5 satisfeito.
8. **Reportar** ao utilizador: ADR anotada, tests delta,
   ficheiros criados/editados, recomendação P264+.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P263.A revela que **vanilla typst usa abordagem
  fundamentalmente diferente** (e.g. evita `/ShadingType 2`
  e usa Image XObjects pre-rasterizados; isto invalidaria
  decisão minimalista).
- P263.C revela que **`Linear::sample(t)` L1** não cobre todos
  os casos necessários (e.g. stops fora de [0,1] permitidos;
  edge case vanilla específico).
- P263.C revela que **`/Resources /Pattern` placement** entra
  em conflito com resources existentes (e.g. shared resources
  vs page-specific decision pendente).
- P263.C revela que **bbox de FrameItem::Shape** não está
  disponível em build_page_stream_* (e.g. precisa de
  pre-computation pass) — refactor exporter intermediate.
- P263.C revela que **algoritmo Coords vanilla** diverge
  significativamente da implementação simplificada (D3 decisão
  preliminar) — pode exigir investigação adicional.
- P263.C revela que **stops com offset `None`** (auto-spacing
  P262) não estão totalmente cobertos por `effective_offsets()`
  L1 — precisa de helper L3 adicional.
- **Cascade L3** ultrapassa ~400 LoC (vs ~200-300 estimado) —
  magnitude real M+ → L; considerar dividir P263 em P263.C1
  (Helvetica path) + P263.C2 (CIDFont/multifont paths).
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.
- **Paridade observable falha** em E2E test específico
  (gradient renderiza visualmente errado vs vanilla).

Em qualquer paragem, registar contexto no relatório parcial
e aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P262 (Gradient L1+stdlib)

P262 deixou pendência clara: PDF mostrava fallback
`first_stop_color`. P263 fecha promessa — gradient real PDF
render. **Granularidade ADR-0061 preservada** via divisão
P262/P263.

### Subpadrão "Refactor cross-cutting entity primitivo" N=3 → N=4

Cumulativo:
- N=1 P252 (Stroke `overhang`).
- N=2 P257 (Color expansão).
- N=3 P261 (Paint wrapper).
- **N=4 P263** (PDF exporter cross-path Gradient emit; toca
  3 paths simultaneamente).

**Patamar N=4 reforça formalização**. Próxima aplicação
candidata: Stroke<Length> ou Tiling activação.

### Subpadrão emergente "P262/P263 dividir granularidade" N=1

Pattern emergente novo:
- P262 = L1+stdlib materialização.
- P263 = L3 PDF rendering dedicado.

Divisão preserva ADR-0061 §"granularidade 1-2 features/passo".
Candidato N=2 se Radial/Conic seguir mesmo padrão (P-Gradient-Radial
L1 + P-Gradient-Radial-PDF L3). Promoção formalização adiada per
política N=3-4.

### Anotação cumulativa vs ADR nova

P263 não cria ADR nova. ADR-0087 já cobre Gradient Linear
globalmente; P263 materializa o backend PDF que ADR-0087
implicitamente apontava. **Anotação cumulativa** documenta;
status `IMPLEMENTADO` preservado literal.

Paridade pattern P258.B/P259.B "histórico cumulativo" (ADR-0080
§"refactor aditivo").

### Política "sem novas reservas"

Preservada. Radial/Conic comentários em gradient.rs continuam
reservas visuais; PDF emit P263 prepara só Type 2 axial.

### Pós-P263 — sequência lógica recomendada

1. **P-Gradient-Radial** (M; replica P262/P263 pattern com
   L1+stdlib+PDF; ShadingType 3).
2. **OU outras Opções P259 alternativas**:
   - DEBT-33 Bézier bbox + Stroke<Length>.
   - Curve variant + Polygon estrutural.
3. **OU Text audit** (segundo consumo directo ADR-0084 +
   0085 — primeiro audit pós-formalização).
4. **OU P-Footnote-N** refino (Model pendência P258).
5. **OU Tiling** (Paint::Tiling activação).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0027 — CIDFont/Identity-H (precedente directo estrutura
  objectos PDF; template arquitectural).
- ADR-0029, ADR-0033, ADR-0054, ADR-0065 — metodologia.
- ADR-0039 (TextStyle SR; preservado literal).
- **ADR-0083** (Color paridade vanilla; `to_srgb`/`to_rgba_f32`
  reutilizados).
- **ADR-0086** (Paint wrapper; consumer chain).
- **ADR-0087** (Gradient Linear-only; anotação cumulativa
  por este passo).
- DEBT-1 (fechado P142; preservado).
- ISO 32000-1 §7.5.7 — Shading patterns (referência canónica
  spec PDF).
- P73 — Image stack dedup `Arc::as_ptr` (template arquitectural
  `pattern_resources`).
- P74 — PNG `/SMask` (precedente cross-path resource
  cascade Helvetica+CIDFont).
- P257 — Color paridade vanilla 8/8 (ADR-0083; pre-flight Q2
  Oklab decision P262 derivada).
- P261 — Paint wrapper (consumer chain Stroke.paint pre-pronta).
- P262 — Gradient L1+stdlib (este passo é continuação directa).
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico imutável precedente.
- `00_nucleo/prompts/infra/export.md` — L0 prompt actualizado
  por este passo.
- `03_infra/src/export.rs` — código L3 alterado por este passo.
