# Passo 265 — PDF Radial shading complete (fecha promessa P264; replica P263 template)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial **L3 cross-cutting**;
magnitude estimada **S-M** (M cap; ~2-4h, paridade P263).
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- ADR-0027 — CIDFont/Identity-H (precedente arquitectural
  estrutura objectos PDF).
- ADR-0029 (regra geral — L3 emit não obriga diagnóstico
  vanilla per si; estrutura PDF é spec ISO 32000).
- ADR-0033 (paridade observable — gradient Radial real PDF
  render cumpre paridade vanilla user-facing).
- ADR-0054 (perfil graded — Oklab interpolação via
  pré-amostragem preservada).
- ADR-0065 (inventariar primeiro).
- **ADR-0083** (Color paridade — `to_srgb`/`to_rgba_f32` API
  reutilizada P263).
- **ADR-0086** (Paint wrapper; `Paint::Gradient` activo P262).
- **ADR-0087** (Gradient Linear-only IMPLEMENTADO P262 +
  anotação cumulativa P263 PDF shading complete).
- **ADR-0088** (Gradient Radial-only IMPLEMENTADO P264;
  §"Critério revisão" aponta literalmente para este passo).
- Relatórios precedentes:
  - **P263** (PDF Linear shading complete — **template
    literal directo deste passo**).
  - **P264** (Gradient Radial L1+stdlib — abriu a promessa
    que este passo fecha; deixou fallback Solid em 3 sítios
    pattern-match).
  - P73 (image stack — precedente `Arc::as_ptr` dedup;
    aplicado P263 → reaplicado P265 com chave Arc<Radial>).

**Outputs canónicos esperados** ao fim do passo:
- **Anotação cumulativa ADR-0088** com secção "Anotação
  cumulativa P265 — PDF Radial shading complete materializado".
  Status `IMPLEMENTADO` preservado literal (paridade pattern
  P263 anotação cumulativa ADR-0087).
- Prompt L0 actualizado: `00_nucleo/prompts/infra/export.md`
  ganha secção "Anotação P265 — Suporte Gradient Radial via
  Radial Shading Patterns" (paridade estrutura P263 anotação).
- Código L3 actualizado em `03_infra/src/export.rs`:
  - `emit_radial_shading_object(radial, page_bbox) -> ObjectId`
    (helper novo análogo `emit_axial_shading_object`).
  - `compute_radial_coords(center, radius, bbox) -> (P0, P1, r0, r1)`
    (helper novo análogo `compute_axial_coords`).
  - **`pattern_resources` HashMap inalterado** estruturalmente
    — chave `usize` (Arc::as_ptr) já genérica; só os emit
    helpers divergem por variant.
  - 3 sítios pattern-match adaptados em P264 com fallback
    Solid → substituídos por emit real Radial.
  - `build_page_stream_*` 3 paths já têm branch `Paint::Gradient`
    (P263) — Radial entra automaticamente via helper
    `emit_paint_fill` que P263 unificou.
- Hash propagado via `--fix-hashes`.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-265-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

0. **Vanilla read-first explicitamente autorizado** —
   `lab/typst-original/` disponível; Claude Code pode (e
   deve) consultar literal:
   - `lab/typst-original/crates/typst-pdf/src/.../shading.rs`
     (ou similar) — `/ShadingType 3` emit vanilla se quiser
     paridade implementação.
   - **Nota**: este passo é **L3 PDF emit** — spec ISO 32000
     §7.5.7 é fonte canónica; vanilla é referência opcional.
1. **Regra de Ouro CLAUDE.md** — prompt L0 `export.md`
   actualizado **antes** de código alterado. Ordem:
   inventário inline → L0 actualização → fix-hashes →
   testes-primeiro → código L3.
2. **Sem ADR nova** — ADR-0088 já cobre Gradient Radial
   globalmente. P265 é **execução do critério revisão
   implícito** (PDF rendering real). **Anotação cumulativa
   ADR-0088** documenta backend PDF complete; paridade pattern
   P263 anotação cumulativa ADR-0087.
3. **Replica template P263 literal** — pattern `compute_*_coords`
   + `emit_*_shading_object` + `emit_pattern_dict` reutilizado
   directo; só Coords cálculo + ShadingType número mudam.
4. **Ordem testes-primeiro** — para cada função emit nova:
   tests antes de implementação. PDF E2E tests confirmam
   bytes esperados (`/ShadingType 3`, `/Coords` com 6 valores,
   `/Function` reutilizado de P263).
5. **`crystalline-lint .`** zero violations no fim.
6. **Tests workspace** sem regressão (baseline 2386 pós-P264).
   Esperado **+5-10** (paridade P263 +8; P265 escopo
   ligeiramente menor pois infraestrutura genérica já existe).
7. **Materialization é leitura proibida por iniciativa
   própria**.
8. **Política "sem novas reservas"** preservada — Conic continua
   comentário reserva em gradient.rs; `/ShadingType 1`
   (function-based) e tipos 4-7 (mesh/coons/tensor) continuam
   fora do scope.
9. **Paridade observable user-facing P264 promessa cumprida** —
   `#gradient.radial(...)` produzia fallback Solid; pós-P265
   renderiza radial real.

---

## §1 — Sub-passo P265.A: Inventário inline (auto-aplicação ADR-0065 critério #5)

**Objectivo**: inventariar estrutura actual do exporter
pós-P263+P264 + identificar pontos exactos de intervenção.

**Materialização**: zero código novo. Apenas leitura.

**Forma**: inline no relatório §2 (sem ficheiro imutável
separado — magnitude trivial per ADR-0085 §"Neutras";
precedente P260.A + P263.A).

### Acções obrigatórias

#### A.1 — Localizar helpers P263 + sítios pattern-match P264

```bash
# Helpers P263 existentes
grep -n "fn compute_axial_coords\|fn oklab_sample_stops\|fn emit_function_dict\|fn emit_axial_shading_object\|fn emit_pattern_dict\|fn emit_paint_fill" \
  03_infra/src/export.rs

# Sítios pattern-match P264 adaptados (fallback Solid para Radial)
grep -n "Gradient::Radial\|Gradient::Linear" 03_infra/src/export.rs

# pattern_resources HashMap actual
grep -n "pattern_resources\|PatternResource" 03_infra/src/export.rs

# 3 funções build_page_stream
grep -n "fn build_page_stream\|fn scan_all_gradients\|fn pattern_resources_for_page" \
  03_infra/src/export.rs
```

**Output esperado**:
- 5 helpers P263 todos presentes.
- 3 sítios `Gradient::Radial(_) => /* fallback Solid: first_stop_color */` —
  preparados P264 para substituição P265.
- `pattern_resources` HashMap genérico `<usize, PatternResource>`.
- 3 paths `build_page_stream_*` com branch `Paint::Gradient`
  já unificado via `emit_paint_fill`.

#### A.2 — Verificar `Linear::sample` vs `Radial::sample` paridade

```bash
# Confirmar Radial::sample existe e é paridade Linear::sample
grep -A 20 "impl Radial " 01_core/src/entities/gradient.rs
grep -n "fn sample\b" 01_core/src/entities/gradient.rs
```

**Output esperado** (per relatório P264):
- `Radial::sample(t)` materializada P264; reutiliza helpers
  Oklab privados (`linear_rgb_to_oklab`, `interpolate_oklab`,
  etc.).
- **`oklab_sample_stops` helper P263** já genérico sobre `Linear` —
  precisa de **2ª variante para Radial** OU **generalização
  genérica**.

#### A.3 — Decisão arquitectural inline (registar §2)

**Decisão D1 — Generalização `oklab_sample_stops`**:

- **Opção α**: criar `oklab_sample_stops_radial(radial: &Radial, n) -> Vec<GradientStop>`
  análogo a `oklab_sample_stops` (que recebe Linear). Duplicação
  pequena; explícito.
- **Opção β**: refactor `oklab_sample_stops` para trait/genérico
  `fn oklab_sample_stops<G: Sampleable>(gradient: &G, n)` onde
  `Sampleable` é trait privado com `fn sample(&self, t: f32) -> Color`.
  Mais limpo mas overhead arquitectural.
- **Recomendação preliminar**: **Opção α (duplicação explícita)**
  — paridade pattern minimal P263 sem trait machinery.

**Decisão D2 — `compute_radial_coords` algoritmo**:

PDF `/ShadingType 3` Coords:
```
/Coords [x0 y0 r0 x1 y1 r1]
```

onde:
- `(x0, y0, r0)` = **focal point** (inner; gradient começa).
- `(x1, y1, r1)` = **target point** (outer; gradient termina).

Para Radial subset materializado P264 (sem focal_*):
- `(x0, y0, r0) = (center.x, center.y, 0.0)` — focal point
  fixo no center; raio 0 (foco pontual).
- `(x1, y1, r1) = (center.x, center.y, radius)` — concêntrico.

**Forma simplificada** — círculos concêntricos. Resultado:
gradient radial **classic** (center → borda).

**Quando focal_* materializar futuro** (P-Gradient-Focal):
- `(x0, y0)` = focal_center.
- `r0` = focal_radius.

Helper `compute_radial_coords` aceita `center: Axes<Ratio>`
e `radius: Ratio` (subset actual), retorna 6 valores no
sistema de coordenadas locais do shape (bbox).

**Conversão `Ratio` → pontos absolutos no bbox**:
- `cx = bbox.x0 + center.x.get() * (bbox.x1 - bbox.x0)`.
- `cy = bbox.y0 + center.y.get() * (bbox.y1 - bbox.y0)`.
- `r = radius.get() * min(bbox.width, bbox.height)` — paridade
  vanilla (radial radius proporcional à menor dimensão).

**Inversão Y PDF** aplica-se após L3 emite Coords locais
(igual P263).

**Decisão D3 — Pattern_resources chave**:

- P263 usa `Arc::as_ptr(linear) as usize`.
- P265 reutiliza mesmo HashMap; chave `Arc::as_ptr(radial) as usize`.
- **Conflito potencial**: dois Arcs distintos podem em
  princípio ter mesmo `as_ptr`? Não — Rust garante endereços
  distintos para Arcs vivos distintos. Documento mantém vivos
  → seguro per P73 documentation.

**Decisão D4 — Cross-path cobertura**:

P263 unificou via `emit_paint_fill(paint, ...)`. Pós-P264 o
helper aceita Paint::Gradient com fallback Solid para Radial.
P265:
- Modificar `emit_paint_fill` para distinguir
  `Gradient::Linear(_)` vs `Gradient::Radial(_)` no lookup
  pattern.
- Branch pattern emit já é unificado — não muda código
  build_page_stream_*.

**Decisão D5 — Function reutilizada**:

P263 emit_function_dict aceita `stops: &[GradientStop]`. P265
reutiliza idêntico — Oklab pré-amostragem produz 16 stops
GradientStop em sRGB; mesmo dict Type 2/Type 3 stitching
funciona para Radial.

### Critério de aceitação P265.A

- Inventário §A.1/A.2 documentado inline no relatório §2.
- Decisões D1-D5 explicitadas com justificação.
- Zero alterações em código (ainda).

---

## §2 — Sub-passo P265.B: Actualizar L0 prompt `infra/export.md`

**Objectivo**: cumprir Regra de Ouro — L0 actualizado antes do
código.

**Materialização**: edição L0 prompt + `--fix-hashes`. Sem
código L3 alterado ainda.

### B.1 — Adicionar secção "Anotação P265"

`00_nucleo/prompts/infra/export.md` ganha secção (paridade
secção P263):

```markdown
## Suporte Gradient Radial via Shading Patterns (Passo 265)

`FrameItem::Shape { paint: Paint::Gradient(Gradient::Radial(_)), ... }`
renderiza via PDF `/ShadingType 3` radial (ISO 32000 §7.5.7).

### Pattern resources — reutilizado

`pattern_resources: HashMap<usize, PatternResource>` mantém
forma genérica P263. Chave `Arc::as_ptr(radial) as usize` em
vez de Linear. PatternResource struct inalterada (shading_id,
pattern_id, function_id).

### Shading Type 3 (radial) — materializado P265

`/ShadingType 3` radial:
- `/ColorSpace /DeviceRGB`.
- `/Coords [x0 y0 r0 x1 y1 r1]` — **6 valores** (vs 4 axial):
  - `(x0, y0, r0)`: focal point (gradient origin).
  - `(x1, y1, r1)`: target point (gradient outer).
- `/Function obj_ref` — Type 2 (2 stops) ou Type 3 stitching
  (N>2). **Idêntico ao axial P263**.
- `/Extend [true true]` — estender cor fora dos círculos
  (paridade vanilla default).

### Radial subset materializado P264 — círculos concêntricos

Sem focal_* (scope-out ADR-0088). Coords subset:
- `(x0, y0, r0) = (cx, cy, 0.0)` — foco pontual no center.
- `(x1, y1, r1) = (cx, cy, r)` — concêntrico.

Resulta em gradient classic center → borda. Quando focal_*
materializar futuro (P-Gradient-Focal), Coords expandem.

### Conversão Ratio → pontos absolutos

```
cx = bbox.x0 + center.x.get() * (bbox.x1 - bbox.x0)
cy = bbox.y0 + center.y.get() * (bbox.y1 - bbox.y0)
r  = radius.get() * min(bbox.width, bbox.height)
```

Inversão Y PDF aplica-se após L3 (igual P263 pattern).

### Function dicts — reutilizado P263

Type 2 (2 stops linear) ou Type 3 stitching (N>2 stops).
**Idêntica estrutura** entre Linear e Radial. Pré-amostragem
Oklab N=16 stops produz mesmo formato Type 3.

### Interpolação Oklab via amostragem densa — reutilizado

`Radial::sample(t)` paridade `Linear::sample(t)` (P264 §C.3).
Pré-amostragem N=16 produz GradientStop em sRGB para emit
PDF.

### Page stream emit — reutilizado

`emit_paint_fill` (P263) ganha branch `Gradient::Radial(_)`:
- Look-up `Arc::as_ptr(radial)` em pattern_resources.
- Emit `/Pattern cs; /P{n} scn` (idêntico P263).

Cross-path: helper unificado P263 cobre 3 paths automaticamente.

### Helpers internos novos P265

| Função | Responsabilidade |
|--------|------------------|
| `compute_radial_coords(center, radius, bbox)` | `(P0, P1, r0, r1)` — 6 valores axes locais |
| `oklab_sample_stops_radial(radial, n)` | N stops intermédios em sRGB pós Oklab (paridade `oklab_sample_stops`) |
| `emit_radial_shading_object(radial, bbox, next_id)` | Shading dict `/ShadingType 3` referenciando function |

### Helpers reutilizados P263 (sem alteração)

| Função | Reutilização |
|--------|--------------|
| `emit_function_dict(stops)` | Idêntico — Type 2 ou Type 3 stitching |
| `emit_pattern_dict(shading_id)` | Idêntico — Pattern dict /PatternType 2 |
| `emit_paint_fill(paint, ...)` | Branch novo `Gradient::Radial(_)` adicionado |

### Limitações P265

- **Radial subset** — focal_* preservado scope-out ADR-0088;
  círculos concêntricos only.
- **Conic continua comentário reserva** em
  `entities/gradient.rs`.
- **`/ShadingType 1, 4-7`** continuam fora de scope.
- **Anti-alias true assumed** (paridade P263).
- **Relative bounding-box assumed** (paridade ADR-0087/0088).
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
# Esperado: 2386 preservado (passo só L0)
```

### Critério de aceitação P265.B

- `export.md` actualizado com secção P265.
- Hash propagado.
- Tests workspace inalterados.
- Zero violations.

---

## §3 — Sub-passo P265.C: Materialização L3

**Ordem obrigatória — testes primeiro per CLAUDE.md**.

### C.1 — Testes primeiro

#### C.1.1 — Unit tests helpers

`03_infra/src/export.rs` (tests submodule):

```rust
#[test]
fn p265_compute_radial_coords_center_default() {
    let bbox = Rect { x0: 0.0, y0: 0.0, x1: 100.0, y1: 100.0 };
    let center = Axes::new(Ratio(0.5), Ratio(0.5));
    let radius = Ratio(0.5);
    let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(center, radius, bbox);
    assert!((x0 - 50.0).abs() < 0.01);
    assert!((y0 - 50.0).abs() < 0.01);
    assert!((r0 - 0.0).abs() < 0.01);
    assert!((x1 - 50.0).abs() < 0.01);
    assert!((y1 - 50.0).abs() < 0.01);
    assert!((r1 - 50.0).abs() < 0.01);
}

#[test]
fn p265_compute_radial_coords_center_offset() {
    let bbox = Rect { x0: 0.0, y0: 0.0, x1: 200.0, y1: 100.0 };
    let center = Axes::new(Ratio(0.25), Ratio(0.75));
    let radius = Ratio(0.4);
    let (x0, y0, _, x1, y1, r1) = compute_radial_coords(center, radius, bbox);
    assert!((x0 - 50.0).abs() < 0.01);   // 0.25 * 200
    assert!((y0 - 75.0).abs() < 0.01);   // 0.75 * 100
    assert_eq!(x0, x1);                   // concêntrico
    assert_eq!(y0, y1);
    assert!((r1 - 40.0).abs() < 0.01);   // 0.4 * min(200, 100)
}

#[test]
fn p265_compute_radial_coords_non_square_bbox_uses_min_dim() {
    let bbox = Rect { x0: 0.0, y0: 0.0, x1: 300.0, y1: 50.0 };
    let (_, _, _, _, _, r1) = compute_radial_coords(
        Axes::new(Ratio(0.5), Ratio(0.5)),
        Ratio(1.0),
        bbox,
    );
    assert!((r1 - 50.0).abs() < 0.01);   // min(300, 50)
}

#[test]
fn p265_oklab_sample_stops_radial_red_to_blue() {
    let radial = Radial {
        stops: Arc::new([
            GradientStop { color: Color::rgb(255, 0, 0), offset: Some(Ratio(0.0)) },
            GradientStop { color: Color::rgb(0, 0, 255), offset: Some(Ratio(1.0)) },
        ]),
        center: Axes::new(Ratio(0.5), Ratio(0.5)),
        radius: Ratio(0.5),
    };
    let samples = oklab_sample_stops_radial(&radial, 16);
    assert_eq!(samples.len(), 16);
    assert_eq!(samples[0].color.to_srgb(), (255, 0, 0, 255));
    assert_eq!(samples[15].color.to_srgb(), (0, 0, 255, 255));
}
```

#### C.1.2 — E2E PDF tests

```rust
#[test]
fn p265_export_pdf_radial_2_stops_emits_shading_type_3() {
    let radial = Gradient::radial(
        vec![
            GradientStop { color: Color::rgb(255, 0, 0), offset: Some(Ratio(0.0)) },
            GradientStop { color: Color::rgb(0, 0, 255), offset: Some(Ratio(1.0)) },
        ],
        Axes::new(Ratio(0.5), Ratio(0.5)),
        Ratio(0.5),
    );
    let stroke = Stroke {
        paint: Paint::Gradient(radial),
        thickness: 2.0,
        overhang: false,
    };
    // Construir Frame com Shape { paint: ... }
    // ...
    let pdf = export_pdf(&doc);
    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(pdf_str.contains("/ShadingType 3"));  // crítico
    assert!(pdf_str.contains("/Coords"));
    assert!(pdf_str.contains("/Pattern"));
    // Coords deve ter 6 valores (vs 4 axial)
    // ... regex ou parse mais sofisticado
}

#[test]
fn p265_export_pdf_radial_concentric_coords_count() {
    // Verifica que Coords [x0 y0 r0 x1 y1 r1] tem 6 elementos.
    // x0 == x1, y0 == y1 (concêntrico), r0 == 0, r1 == radius*min.
    // ...
}

#[test]
fn p265_export_pdf_radial_dedup_arc_ptr() {
    // Mesmo Arc<Radial> usado 3 vezes → 1 Shading object.
    // ...
}

#[test]
fn p265_export_pdf_radial_helvetica_path_works() { ... }

#[test]
fn p265_export_pdf_radial_cidfont_path_works() { ... }

#[test]
fn p265_export_pdf_linear_e_radial_coexistem() {
    // Documento com 1 Linear + 1 Radial → 2 Shading objects distintos.
    // ...
}
```

Executar `cargo test export::p265` ou similar — verificar
falham.

### C.2 — Implementação L3

#### C.2.1 — Helper `compute_radial_coords`

```rust
// 03_infra/src/export.rs

fn compute_radial_coords(
    center: Axes<Ratio>,
    radius: Ratio,
    bbox: Rect,
) -> (f64, f64, f64, f64, f64, f64) {
    let cx = bbox.x0 + center.x.get() * (bbox.x1 - bbox.x0);
    let cy = bbox.y0 + center.y.get() * (bbox.y1 - bbox.y0);
    let r = radius.get() * (bbox.x1 - bbox.x0).min(bbox.y1 - bbox.y0);
    // Focal point: pontual no center (r0=0). Target: concêntrico
    // com radius. Subset materializado P264 (focal_* scope-out).
    (cx, cy, 0.0, cx, cy, r)
}
```

**Decisão local**: `bbox.y1 - bbox.y0` é altura em coordenadas
locais — inversão Y aplicada quando emitida no PDF (paridade
P263).

#### C.2.2 — Helper `oklab_sample_stops_radial`

```rust
fn oklab_sample_stops_radial(radial: &Radial, n_samples: usize) -> Vec<GradientStop> {
    (0..n_samples)
        .map(|i| {
            let t = i as f32 / (n_samples - 1).max(1) as f32;
            let color = radial.sample(t);  // L1 helper P264
            GradientStop { color, offset: Some(Ratio(t as f64)) }
        })
        .collect()
}
```

**Paridade literal `oklab_sample_stops` (P263)**, apenas
diferindo no tipo `Radial` vs `Linear`. Duplicação aceite
per decisão D1 §A.3.

#### C.2.3 — Helper `emit_radial_shading_object`

```rust
fn emit_radial_shading_object(
    radial: &Radial,
    bbox: Rect,
    next_id: &mut u32,
) -> (ObjectId, ObjectId) {  // (shading_id, function_id)
    let samples = oklab_sample_stops_radial(radial, 16);
    let function_id = emit_function_dict(&samples, next_id);
    let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
        radial.center,
        radial.radius,
        bbox,
    );
    let shading_id = *next_id; *next_id += 1;
    // emit /ShadingType 3
    //      /ColorSpace /DeviceRGB
    //      /Coords [x0 y0 r0 x1 y1 r1]   ← 6 valores (vs 4 axial)
    //      /Function function_id
    //      /Extend [true true]
    (shading_id, function_id)
}
```

**Estrutura idêntica a `emit_axial_shading_object` (P263)**;
diferenças isoladas:
- `oklab_sample_stops_radial` em vez de `oklab_sample_stops`.
- `compute_radial_coords` 6 valores em vez de
  `compute_axial_coords` 4 valores.
- `/ShadingType 3` em vez de `/ShadingType 2`.
- `/Extend [true true]` em vez de `[false false]` — paridade
  vanilla default radial.

#### C.2.4 — Adaptar `emit_paint_fill` para Radial

P263 versão (esquemática):

```rust
fn emit_paint_fill(paint: &Paint, pattern_resources: &HashMap<usize, PatternResource>) -> String {
    match paint {
        Paint::Solid(c) => { ... }
        Paint::Gradient(g) => {
            let arc_ptr = match g {
                Gradient::Linear(l) => Arc::as_ptr(l) as usize,
                // Gradient::Radial(_) => fallback first_stop_color (P264 stub)
            };
            // ...
        }
    }
}
```

P265 versão:

```rust
fn emit_paint_fill(paint: &Paint, pattern_resources: &HashMap<usize, PatternResource>) -> String {
    match paint {
        Paint::Solid(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{r} {g} {b} rg")
        }
        Paint::Gradient(g) => {
            let arc_ptr = match g {
                Gradient::Linear(l) => Arc::as_ptr(l) as usize,
                Gradient::Radial(r) => Arc::as_ptr(r) as usize,  // P265 — substitui fallback
            };
            let resource = pattern_resources.get(&arc_ptr)
                .expect("pattern not registered before emit");
            format!("/Pattern cs\n/P{} scn", resource.pattern_id_index)
        }
    }
}
```

**Branch `Gradient::Radial(_)` substitui fallback Solid P264**.

#### C.2.5 — Adaptar `scan_all_gradients`, `pattern_resources_for_page`

Per relatório P264:
- 3 sítios pattern-match adaptados com fallback Solid.

Cada um ganha branch Radial **real** em vez de fallback:

```rust
// Pseudocódigo — adaptar conforme estrutura real export.rs
fn scan_all_gradients(doc: &PagedDocument) -> Vec<(usize, GradientRef)> {
    let mut gradients = Vec::new();
    for page in &doc.pages {
        for item in &page.items {
            match item {
                FrameItem::Shape { paint: Paint::Gradient(g), .. } => {
                    let arc_ptr = match g {
                        Gradient::Linear(l) => (Arc::as_ptr(l) as usize, GradientRef::Linear(l.clone())),
                        Gradient::Radial(r) => (Arc::as_ptr(r) as usize, GradientRef::Radial(r.clone())),
                    };
                    gradients.push(arc_ptr);
                }
                // ... outros FrameItem variants
            }
        }
    }
    gradients
}
```

`GradientRef` enum local pode ser introduzido para distinguir
emit handler:

```rust
enum GradientRef {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),
}
```

Ou, alternativamente: continuar a tratar via match no momento
de emit, sem GradientRef intermédio. **Decisão local** —
recomendação preliminar `GradientRef` separado para clareza.

#### C.2.6 — Adaptar `pattern_resources_for_page` (registo pre-pass)

Pre-pass que regista todos os gradients antes de emit do
stream — adapta para acomodar Radial:

```rust
fn pattern_resources_for_page(...) -> HashMap<usize, PatternResource> {
    let mut resources = HashMap::new();
    for (arc_ptr, gradient_ref) in scan_all_gradients(...) {
        if resources.contains_key(&arc_ptr) { continue; }
        let (shading_id, function_id) = match gradient_ref {
            GradientRef::Linear(l) => emit_axial_shading_object(&l, page_bbox, &mut next_id),
            GradientRef::Radial(r) => emit_radial_shading_object(&r, page_bbox, &mut next_id),
        };
        let pattern_id = emit_pattern_dict(shading_id, &mut next_id);
        resources.insert(arc_ptr, PatternResource { shading_id, pattern_id, function_id });
    }
    resources
}
```

### C.3 — Verificação intermediária

```bash
cargo build --workspace
# Esperado: verde
RUST_MIN_STACK=33554432 cargo test --workspace --release
# Esperado: 2386 → 2391-2396 (+5-10 P265)
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
```

### Critério de aceitação P265.C

- 3 helpers L3 novos implementados (`compute_radial_coords`,
  `oklab_sample_stops_radial`, `emit_radial_shading_object`).
- `emit_paint_fill` ganha branch Radial.
- `scan_all_gradients` + `pattern_resources_for_page`
  adaptados (eventual `GradientRef` enum local).
- `pattern_resources` HashMap inalterado estruturalmente.
- 3 sítios pattern-match fallback Solid substituídos por
  emit real.
- Tests workspace **2386 → 2391-2396** (+5-10).
- Zero violations.
- E2E tests confirmam bytes PDF (`/ShadingType 3`, Coords 6
  valores, dedup Arc::as_ptr<Radial>, coexistência Linear +
  Radial no mesmo doc).

---

## §4 — Sub-passo P265.D: Anotação ADR-0088 cumulativa + relatório

### D.1 — Anotação cumulativa ADR-0088

`00_nucleo/adr/typst-adr-0088-gradient-radial-only.md` ganha
secção (paridade pattern P263 anotação ADR-0087):

```markdown
## Anotação cumulativa P265 — PDF Radial shading complete materializado

(Data 2026-05-15)

PDF rendering real Gradient Radial materializado. Substitui
fallback `first_stop_color` introduzido em P264.

### Componentes materializados

- `compute_radial_coords(center, radius, bbox) -> (x0, y0, r0, x1, y1, r1)` —
  6 valores Coords axes locais; círculos concêntricos
  (focal_* scope-out preservado).
- `oklab_sample_stops_radial(radial, n=16)` — paridade literal
  `oklab_sample_stops` P263.
- `emit_radial_shading_object(radial, bbox, next_id)` —
  `/ShadingType 3` + `/Coords [6 valores]` + `/Extend [true true]`.
- `emit_paint_fill` branch `Gradient::Radial(_)` substitui
  fallback Solid P264.
- `scan_all_gradients` + `pattern_resources_for_page`
  adaptados para Radial (eventual `GradientRef` enum local).
- `pattern_resources` HashMap genérico inalterado.
- Reutilização literal:
  - `emit_function_dict` (Type 2 / Type 3 stitching).
  - `emit_pattern_dict` (/PatternType 2).
  - `Radial::sample(t)` Oklab L1 P264.
  - 3 paths `build_page_stream_*` via `emit_paint_fill`
    unificado P263.

### Paridade observable cumprida pós-P265

`#gradient.radial(red, blue, center: (50%, 50%), radius: 60%)`
em Stroke renderiza radial real no PDF (círculos concêntricos
center → borda). Fallback Solid pré-P265 eliminado.

### Scope-outs preservados

- `focal_center`/`focal_radius` → P-Gradient-Focal futuro.
- Conic continua comentário reserva em gradient.rs.
- `/ShadingType 1, 4-7` fora de scope.

### Subpadrões aplicados

- **"P262/P263 dividir granularidade" cresce N=2 → N=3** (P262/
  P263 Linear divisão; P264/P265 Radial divisão; **atinge
  limiar formalização clara**).
- **"Anotação cumulativa em vez de ADR nova" reaplicada** —
  paridade P263 anotação ADR-0087.
- **"Refactor cross-cutting entity primitivo" N=4 → N=5** se
  cascade `emit_paint_fill` cross-path contar.
- **"Reutilização literal de helpers cross-passos"** N=1 (P265
  reutiliza 4 helpers P263 inalterados; padrão novo
  inaugurado — N=1 fresco; promoção formalização adiada per
  política N=3-4).

Cross-references:
- L3 emit: `03_infra/src/export.rs` (~100-150 LoC novas).
- Tests E2E: confirmam `/ShadingType 3`, Coords 6 valores,
  dedup, coexistência Linear+Radial.
- ADR-0087 anotação P263 (precedente directo template).
- P263 (template literal — paridade quase 1-para-1).
- P264 (origem da promessa fechada por este passo).
```

**Status `IMPLEMENTADO` preservado** — anotação cumulativa
não muda status; refina aplicação.

### D.2 — README ADRs

Sem alteração de contagens. Linha ADR-0088 ganha referência
cumulativa P265 análoga ADR-0083 pós-P259 ou ADR-0087 pós-P263.

### D.3 — Subpadrões cumulativos

**"P262/P263 dividir granularidade" N=2 → N=3**:
- N=1 P262 (Linear L1+stdlib) → P263 (Linear PDF).
- N=2 P264 (Radial L1+stdlib) → P265 (Radial PDF).
- **N=3 atingido cumulativamente** (cluster Gradient completa
  duas divisões).

**Patamar N=3 atinge limiar formalização clara**. Candidato a
ADR meta futuro — **improvável** (auto-documentado por cada
aplicação).

**"Reutilização literal de helpers cross-passos" N=1 — pattern
novo inaugurado**:
- P265 reutiliza `emit_function_dict`, `emit_pattern_dict`,
  `emit_paint_fill` (com extensão), `Radial::sample` (P264)
  inalterados.
- 70% do código L3 do passo é wiring + helpers novos
  específicos (compute_radial_coords, oklab_sample_stops_radial,
  emit_radial_shading_object).
- Subpadrão emergente: passos de extensão herdam infraestrutura
  estabelecida em passos anteriores. Candidato à formalização
  N=3-4 cumulativo (se P-Gradient-Conic seguir mesmo padrão).

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-265-relatorio.md`
estrutura canónica:

- **§1 Sumário executivo** — magnitude real; tests delta;
  ADR-0088 anotada; ficheiros tocados.
- **§2 P265.A** — inventário inline + decisões D1-D5.
- **§3 P265.B** — L0 export.md actualizado.
- **§4 P265.C** — código L3 materializado.
- **§5 P265.D** — anotação ADR-0088.
- **§6 Padrões metodológicos** — "dividir granularidade"
  N=2 → 3; "reutilização literal helpers cross-passos" N=1
  novo; "anotação cumulativa em vez de ADR nova" reaplicada.
- **§7 Cobertura** — Visualize ~68% → **~73%** (+5pp via PDF
  Radial real; F.2 promovido implementado L1+stdlib →
  implementado completo).
- **§8 Limitações e trabalho futuro** — Conic dedicado;
  focal_* dedicado; refinos Coords vs vanilla.
- **§9 Critério de aceitação global P265 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P265.D

- ADR-0088 anotada (sem alteração de status).
- README ADRs cross-reference cumulativa.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P265

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [ ] `cargo test --workspace --release` retorna ≥ 2386 + 5-10
  = 2391-2396.
- [ ] Inventário P265.A inline no relatório §2 com decisões
  D1-D5 explicitadas.
- [ ] `00_nucleo/prompts/infra/export.md` actualizado com
  secção P265; hash propagado.
- [ ] `03_infra/src/export.rs` ganha 3 helpers novos
  (`compute_radial_coords`, `oklab_sample_stops_radial`,
  `emit_radial_shading_object`).
- [ ] `emit_paint_fill` branch Radial substitui fallback Solid
  P264.
- [ ] `scan_all_gradients` + `pattern_resources_for_page`
  adaptados (eventual `GradientRef` enum local).
- [ ] `pattern_resources` HashMap inalterado estruturalmente
  (reutilizado).
- [ ] 3 sítios pattern-match fallback P264 substituídos.
- [ ] E2E tests confirmam bytes PDF (`/ShadingType 3`, Coords
  6 valores, dedup `Arc::as_ptr<Radial>`, coexistência
  Linear + Radial).
- [ ] Tests unit confirmam helpers (`compute_radial_coords`
  default/offset/non-square; `oklab_sample_stops_radial`).
- [ ] ADR-0088 ganha secção "Anotação cumulativa P265".
- [ ] **Paridade observable cumprida pós-P265**: PDFs com
  `#gradient.radial(...)` renderizam radial real (não
  fallback).
- [ ] Cross-path testado: Helvetica + CIDFont + multifont
  todos com Radial funcional.
- [ ] **Stroke + Fill** ambos com Paint::Gradient (Radial)
  funcionais (paridade P263).

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, ADR-0027 (precedente PDF), ADR-0083/
   0086/0087/0088 (cluster Gradient), relatórios P263
   (template literal directo) e P264 (origem promessa).
2. **Reportar** estado inicial: tests 2386 + lint baseline +
   ADRs 75.
3. **P265.A** — Executar grep inventário; decisões D1-D5
   inline no relatório draft.
4. **P265.B** — Editar `export.md` com secção P265; propagar
   hash; verificar lint limpo + tests preservados.
5. **P265.C** — Tests primeiro (unit helpers + E2E PDF);
   implementação 3 helpers novos + `emit_paint_fill` branch
   Radial + adaptar scan_all_gradients/pattern_resources_for_page;
   verificar tests verdes + lint limpo.
6. **P265.D** — Anotar ADR-0088 cumulativamente; actualizar
   README ADRs cross-reference; criar relatório.
7. **Verificação final** — checklist §5 satisfeito.
8. **Reportar** ao utilizador: ADR anotada, tests delta,
   ficheiros criados/editados, recomendação P266+.

---

## §7 — Política de paragem

**Nota preliminar**: a spec contém palpites sobre estrutura
exacta dos helpers P263 (`compute_axial_coords`,
`oklab_sample_stops`, etc.). **Discrepância palpite-vs-código
real não é gatilho de paragem por si** — Inventário §A.1
regista o estado real e adapta estrutura P265 conforme.

Claude Code **deve parar e perguntar ao utilizador** se:

- P265.A revela que **`pattern_resources` HashMap não é
  estruturalmente genérico** como o relatório P264 sugere
  (e.g. usa um sub-struct `LinearPatternResource` específico) —
  exigiria refactor inicial.
- P265.A revela que **`emit_paint_fill` foi inlined em P263**
  em vez de helper unificado — exigiria criação de helper
  ou inline expand em 3 paths.
- P265.A revela que **`Radial::sample(t)` semântica radial 1D
  diverge significativamente** do `Linear::sample(t)` (e.g.
  amostragem 2D necessária) — re-pensar `oklab_sample_stops_radial`.
- P265.A revela que **vanilla emit Radial usa Coords diferentes**
  (e.g. círculos não-concêntricos por defeito sem focal_*) —
  decisão local sobre paridade observable.
- P265.C revela que **`/ShadingType 3` `/Extend` default** difere
  do esperado `[true true]` (e.g. vanilla emite `[false false]`).
- P265.C revela que **`GradientRef` enum local** entra em
  conflito com estrutura existente (e.g. P263 já tem enum
  similar) — re-pensar.
- P265.C revela que **cascade `emit_paint_fill`** estoira em
  3 paths simultaneamente (vs branch isolado em 1 helper) —
  magnitude real M+ → considerar adiar.
- **Paridade observable falha** em E2E test específico
  (Radial renderiza visualmente errado vs vanilla).
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.

Em qualquer paragem, registar contexto no relatório parcial e
aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P264 (Radial L1+stdlib)

P264 deixou pendência clara: PDF mostrava fallback
`first_stop_color` para Radial. P265 fecha promessa — radial
real PDF render. **Granularidade ADR-0061 preservada** via
divisão P264/P265 análoga P262/P263.

### Subpadrão "P262/P263 dividir granularidade" N=2 → N=3

Cumulativo:
- N=1 P262 (Linear L1+stdlib) → P263 (Linear PDF).
- N=2 P264 (Radial L1+stdlib) → P265 (Radial PDF).
- **N=3 atingido cumulativamente** com este passo.

**Patamar N=3 atinge limiar formalização clara**. Candidato a
ADR meta — **improvável** (auto-documentado).

### Subpadrão "Reutilização literal de helpers cross-passos" N=1 inaugurado

P265 reutiliza inalterados:
- `emit_function_dict` (P263) — interpolação stops.
- `emit_pattern_dict` (P263) — Pattern dict wrapper.
- `emit_paint_fill` (P263) — branching central; **só ganha
  branch novo**.
- `Radial::sample` (P264) — Oklab sampling L1.
- Helpers Oklab privados de `gradient.rs` (P262).

**70% do código L3 P265 é wiring + helpers específicos**.
Magnitude S-M reflecte herança arquitectural.

Pattern emergente sólido inaugurado **N=1**. Candidato a
formalização N=3-4 cumulativo se P-Gradient-Conic + outros
passos seguirem.

### Anotação cumulativa em vez de ADR nova

Paridade pattern P263 anotação ADR-0087. ADR-0088 já cobre
Gradient Radial globalmente; P265 materializa backend PDF que
ADR-0088 implicitamente apontava. **Anotação cumulativa**
documenta; status `IMPLEMENTADO` preservado literal.

### Política "sem novas reservas"

Preservada. Conic continua comentário reserva em gradient.rs;
focal_* continua scope-out ADR-0088.

### Pós-P265 — sequência lógica recomendada

1. **P-Gradient-Conic L1+stdlib + PDF** (replica P262/P263 +
   P264/P265 patterns; activa último Gradient variant; M+S-M
   cumulativo).
2. **OU outras Opções P259 alternativas**:
   - DEBT-33 Bézier bbox + Stroke<Length>.
   - Curve variant + Polygon estrutural.
3. **OU Text audit** (segundo consumo directo ADR-0084 +
   0085).
4. **OU P-Footnote-N** refino (Model pendência).
5. **OU P-Gradient-Focal** (activar `focal_center` +
   `focal_radius` Radial).
6. **OU Tiling** (Paint::Tiling activação).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0027 — CIDFont/Identity-H (precedente estrutura PDF).
- ADR-0029, ADR-0033, ADR-0054, ADR-0065 — metodologia.
- ADR-0039 — TextStyle SR (preservado literal).
- **ADR-0083** — Color paridade (`to_srgb`/`to_rgba_f32`
  reutilizados).
- **ADR-0086** — Paint wrapper.
- **ADR-0087** — Gradient Linear-only (anotação cumulativa
  P263 — precedente directo template).
- **ADR-0088** — Gradient Radial-only (anotação cumulativa
  por este passo).
- DEBT-1 — Fechado P142 (preservado).
- ISO 32000-1 §7.5.7 — Shading patterns (`/ShadingType 3`
  radial referência canónica).
- P73 — Image stack dedup `Arc::as_ptr` (template arquitectural).
- P74 — PNG `/SMask` (precedente cross-path resource cascade).
- P257 — Color paridade 8/8 (ADR-0083; helpers Color
  reutilizados).
- P261 — Paint wrapper (Paint::Gradient activa absorve
  Radial sem cascade refactor).
- P262 — Gradient L1+stdlib (precedente N=1 dividir
  granularidade).
- **P263** — Gradient Linear PDF (**template literal
  P265**; anotação cumulativa ADR-0087 — paridade exacta).
- **P264** — Gradient Radial L1+stdlib (origem promessa fechada
  por este passo; 3 sítios pattern-match preparados).
- `00_nucleo/diagnosticos/diagnostico-gradient-radial-vanilla-passo-264.md`
  — diagnóstico imutável precedente.
- `00_nucleo/diagnosticos/diagnostico-gradient-vanilla-passo-262.md`
  — diagnóstico Linear precedente.
- `00_nucleo/prompts/infra/export.md` — L0 prompt actualizado
  por este passo.
- `03_infra/src/export.rs` — código L3 alterado por este passo.
- Vanilla
  `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`
  — fonte canónica (Radial §1063-1080 lido P264).
- Vanilla
  `lab/typst-original/crates/typst-pdf/src/.../shading.rs`
  (opcional — referência implementação se decisão local
  requerer paridade exacta).
