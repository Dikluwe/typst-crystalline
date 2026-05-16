# Prompt L0 — `infra/export` — Exportador Físico de Documentos
Hash do Código: cbb1181e

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export.rs`
**Criado em**: 2026-03-29 (Passo 21)
**Atualizado em**: 2026-04-19 (Passo 73 — FrameItem::Image, XObject JPEG, deduplicação por Arc::as_ptr)
**ADRs relevantes**: ADR-0027 (CIDFont + Identity-H para Unicode completo)

---

## Contexto e Objetivo

O L1 produz um `PagedDocument` — uma árvore de instruções de desenho com
coordenadas puras (`FrameItem::Text`, `FrameItem::Line`, `FrameItem::Glyph`,
`FrameItem::Image`).
**Este módulo** (L3) converte essa geometria pura em bytes estruturados de
PDF-1.7, sem `crates` externas de PDF — geração manual de objectos, xref e
trailer.

### Suporte a Imagens (Passo 73)

`FrameItem::Image` é emitido pelo layouter quando `Content::Image` é processado.
O exportador suporta apenas **JPEG** neste passo (magic bytes `0xFF 0xD8 0xFF`),
usando `/Filter /DCTDecode` — os bytes crus são embutidos sem recodificação.

**Deduplicação:** a mesma imagem pode aparecer várias vezes no documento.
O mapa `image_resources: HashMap<usize, ImageResource>` usa `Arc::as_ptr(data) as usize`
como chave — seguro porque `PagedDocument` mantém todos os Arcs vivos durante
`export_pdf`, impedindo reutilização de endereços.

**DEBT-27:** PNG requer descodificação completa (canal alpha → /SMask).
Ignorado silenciosamente neste passo — o espaço está reservado no layout.

**DEBT-29:** assume `DeviceRGB` para todos os JPEGs. JPEG Grayscale/CMYK
detectados pelo Passo 74 via marcadores SOF0/SOF2.

### Fronteira de Arquitectura

- **Zero Lógica de Layout**: não decide onde colocar caracteres, não calcula
  quebras de linha — apenas executa ordens emitidas pelo `PagedDocument`.
- Único módulo da codebase autorizado a usar `ttf_parser::Face` directamente
  (para mapear char → glyph_id em `map_chars_to_glyphs` e calcular widths).
- Geometria de coordenadas: **inversão do eixo Y** — PDF usa y=0 no canto
  inferior esquerdo; o Cristalino usa y=0 no topo.
  `y_pdf = page_height - y_cristalino`

---

## Dois Caminhos de Exportação

### Caminho A — Helvetica (fallback, sem embedding)

Usado por `export_pdf(doc)`. Sem fonte TrueType → usa Helvetica Type1
(WinAnsiEncoding, Latin-1). Caracteres não-ASCII (> 127) ficam substituídos
por `?`. Adequado para documentos de texto simples em testes de integração.

```
Object layout (Helvetica path):
1 — Catalog
2 — Pages (kids)
3..3+n — Page dicts (n = número de páginas)
3+n..  — Content streams (BT/ET por FrameItem)
font_id   — /Helvetica Type1 (/F1)
font_id+1 — /Helvetica-Bold Type1 (/F2)
font_id+2 — /Helvetica-Oblique Type1 (/F3)
xref + trailer + %%EOF
```

`FrameItem::Glyph` é ignorado silenciosamente no caminho Helvetica
(sem fonte TrueType, não existe glyph ID válido).

### Caminho B — CIDFont + Identity-H (ADR-0027, Unicode completo)

Usado por `export_pdf_with_font(doc, font_data)`. Com TrueType embebida:
- Encoding: `/Identity-H` (glyph IDs em big-endian 2 bytes na stream)
- Mapeamento: `collect_codepoints` → `map_chars_to_glyphs` → `char_to_gid: HashMap<char, u16>`
- Passo 45 (DEBT-9): `FrameItem::Glyph` (delimitadores matemáticos) também
  incluídos no `ToUnicode` via `build_math_glyph_reverse_map` (L3 → `font_metrics.rs`)

```
Object layout (CIDFont path):
1 — Catalog
2 — Pages
3..3+n — Page dicts
3+n..  — Content streams
font_id   — Type0 font (/F1, /Identity-H, descendents: cidfont_id)
font_id+1 — CIDFont (/CIDFontType2, widths array W)
font_id+2 — FontDescriptor (flags, bbox, /FontFile2)
font_id+3 — Font data stream (bytes brutos .ttf — sem subsetting, ADR-0027 Opção A)
font_id+4 — ToUnicode CMap stream (em blocos de ≤ 100 bfchar)
xref + trailer + %%EOF
```

---

## Interface Pública

```rust
/// Fallback Helvetica — Latin-1, sem embedding.
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8>

/// CIDFont + Identity-H — Unicode completo, fonte embebida.
/// font_data: bytes brutos de .ttf/.otf
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8>
```

---

## Helpers Internos (pub na crate apenas)

| Função | Responsabilidade |
|--------|-----------------|
| `collect_codepoints(doc)` | `BTreeSet<char>` de todos os chars em `FrameItem::Text` |
| `collect_glyph_ids(doc)` | `BTreeSet<u16>` de todos os glyph_id em `FrameItem::Glyph` |
| `map_chars_to_glyphs(face, chars)` | `Vec<(char, u16)>` com glyph_index da fonte |
| `widths_array(face, mappings)` | String PDF `"gid [width]..."` em unidades 1/1000 text space |
| `to_unicode_cmap(mappings)` | CMap stream para ToUnicode (blocos de ≤ 100) |
| `text_to_hex_string(text, map)` | `<XXXX>` em Identity-H para cada char do texto |
| `escape_pdf_string(text)` | Escaping `(` `→` `\(`, `)` `→` `\)`, `\` `→` `\\`; não-ASCII `→` `?` |
| `build_page_stream_type1(page)` | Stream BT/ET para Helvetica |
| `build_page_stream_cidfont(page, map)` | Stream BT/ET para CIDFont (hex strings + Glyph directo) |

---

## Critérios de Verificação

```
// Estrutura PDF
export_pdf(doc).starts_with(b"%PDF-1.7")                          = true
export_pdf(doc).endswith("%%EOF")                                  = true
conteúdo contém: "xref", "trailer", "startxref", "/Catalog",
                  "/Pages", "Helvetica"

// Inversão Y
page_height=842, item_y=84 → y_pdf=758.0

// MediaBox A4
conteúdo contém "595" e "842"

// Texto ASCII visível no PDF (Helvetica)
export_pdf(layout(Content::text("Hello"))) → contém "Hello"

// Escaping
escape_pdf_string("(a)") = "\\(a\\)"
escape_pdf_string("a\\b") = "a\\\\b"
// % não precisa de escape em PDF strings:
escape_pdf_string("100%") contém '%' mas NÃO contém "\\%"

// Documento vazio → PDF válido
export_pdf(PagedDocument::new(vec![])).starts_with(b"%PDF-1.7")

// Helpers CIDFont
collect_codepoints(vazio) = []
collect_codepoints(texto_com_duplicados) → sem duplicados (BTreeSet)
collect_glyph_ids(doc_com_glyph=[42, 42, 99]) = {42, 99}  // deduplied

// ToUnicode CMap
to_unicode_cmap([('A', 36)]) → contém "begincmap", "endcmap", "beginbfchar"
                             → contém "<0024> <0041>"  // glyph 36=0x0024, A=U+0041
// Blocos de 100:
101 mappings → 2 blocos "beginbfchar"

// text_to_hex_string (Identity-H)
text_to_hex_string("Hi", {'H':0x48, 'i':0x69}) = "<00480069>"
text_to_hex_string("X", {}) = "<0000>"    // notdef para char sem mapeamento

// FrameItem::Glyph (Passo 45)
collect_glyph_ids retorna IDs únicos
CMap inclui entrada para glyph variante: "<00A2> <0028>" para glyph_id=0xA2 → '('
```

---

## Dívidas Técnicas Documentadas

| ID | Descrição | Passo |
|----|-----------|-------|
| DEBT-5 | Unicode não-ASCII em Helvetica → `?` (CIDFont resolve) | Passo 24 |
| DEBT-9 | `FrameItem::Glyph` no ToUnicode via `build_math_glyph_reverse_map` | Passo 45 |
| ADR-0027 Opção A | Fonte completa embebida sem subsetting — subset reduz tamanho | Futuro |
| Bold/Italic CIDFont | Identity-H usa apenas /F1; bold/italic requerem fontes adicionais | Futuro |

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-03-29 | Criação — Passo 21: serialição Helvetica manual, inversão Y, escaping | `export.rs` |
| 2026-04-12 | Restauro — expandido: CIDFont/Identity-H (ADR-0027), FrameItem::Glyph (Passo 45), ToUnicode, helpers internos | `export.md` |

## Rounded-rect clip path — Passo 242 (M9d/M7+5; ADR-0081 IMPLEMENTADO parcial 3/5)

Extensão DEBT-30 (clip-mask, fechado P79) para `ShapeKind::RoundedRect
{ radii: Corners<Length> }`. Helper local:

```rust
fn emit_rounded_rect_ops(
    ops: &mut String,
    x: f64, y: f64, w: f64, h: f64,
    radii: &Corners<Length>,
)
```

**Algoritmo**: Bezier 4 corners path via PDF operadores
`m` (move) + `l` (line) + `c` (cubic bezier) + `h` (close).
Coordenadas em sistema PDF (Y crescente para cima); caller
externo aplica inversão Y se necessário.

**Sequência** sentido horário começando após canto top-left:
1. `m` posição inicial top-edge (`x + tl_r, y_top`).
2. `l` line para `x_right - tr_r, y_top`.
3. `c` cubic top-right corner (skip se `tr == 0`).
4. `l` line para `x_right, y_bottom + br_r`.
5. `c` cubic bottom-right corner (skip se `br == 0`).
6. `l` line para `x_left + bl_r, y_bottom`.
7. `c` cubic bottom-left corner (skip se `bl == 0`).
8. `l` line para `x_left, y_top - tl_r`.
9. `c` cubic top-left corner (skip se `tl == 0`).
10. `h` close path.

**Bezier kappa**: `0.552_284_749_831` (paridade `ShapeKind::Ellipse`
mesmo ficheiro; minimiza erro quarto de círculo).

**Clamp radii**: `tl/tr/br/bl` clamped a `min(w, h) / 2.0` para
evitar overflow geométrico (paridade vanilla).

**Consumers**:
- `draw_item_global` (page-relative) — usa `pos.x.val()`, `pdf_y`.
- `emit_shape_path_local` (Group local space) — usa `(0, -height)`
  origem.
- `draw_item_local` (após `cm` transformação) — usa `pos.x.0`,
  `local_y`.
- 2× draw_item duplicados em paths Shape (similar arms).

Total **5 sítios** com arm `ShapeKind::RoundedRect` consistente.

**Layout integration P242**: `FrameItem::Group { clip_mask:
Some(RoundedRect { radii }), ... }` emitido quando
`Content::Block.clip == true` && `radius != Corners::uniform(ZERO)`.
PDF exporter desenha path + emite `W n` (paridade DEBT-30 fechado
P79 mas para shape rounded).

---

## Suporte Gradient via Shading Patterns (Passo 263)

`FrameItem::Shape { stroke: Some(Stroke { paint: Paint::Gradient(g), ... }), ... }`
renderiza via PDF shading patterns (ISO 32000 §7.5.7).

**Escopo P263**: apenas `Stroke.paint::Gradient` activa shading
real. `FrameItem::Shape.fill: Option<Color>` é literal sRGB
(refino futuro pode estender para Paint enum se prioritário).

### Pattern resources

`scan_all_gradients(doc, first_id)` paralelo a `scan_all_images`
(P73). Retorna `(refs, ptr_to_idx, gradient_objs)`:
- `refs: Vec<PatternRef>` — name (e.g. `P1`, `P2`) + obj_id por
  gradient.
- `ptr_to_idx: HashMap<usize, usize>` — `Arc::as_ptr(linear) as
  usize → idx em refs`.
- `gradient_objs: Vec<GradientObject>` — dados para emit
  Function/Shading/Pattern dicts.

### Shading Type 2 (axial) — único materializado P263

`/ShadingType 2` axial:
- `/ColorSpace /DeviceRGB`.
- `/Coords [x0 y0 x1 y1]` — endpoints linha gradient (locais).
- `/Function obj_ref` — Type 2 (2 stops) ou Type 3 stitching (N>2).
- `/Extend [false false]`.

### Function dicts

**Type 2** (exponential interpolation, 2 stops):
```
/FunctionType 2 /Domain [0 1] /C0 [r0 g0 b0] /C1 [r1 g1 b1] /N 1
```

**Type 3** (stitching, N>2 stops):
```
/FunctionType 3 /Domain [0 1]
/Functions [F2 F2 ...]   % N-1 Type 2 sub-funcs
/Bounds [t1 t2 ... t_{N-1}]
/Encode [0 1 0 1 ...]
```

### Interpolação Oklab via amostragem densa

Vanilla Gradient interpola em Oklab (ADR-0087 P262). PDF Type
2/3 nativos não suportam Oklab.

**Aproximação P263**: pré-amostragem em Oklab via
`Linear::sample(t)` L1 (P262) → N=16 stops intermédios em sRGB
→ Type 3 stitching linear.

### Page Resources dict

Páginas com pelo menos 1 Gradient ganham:
```
/Resources <<
  /Font << ... >>
  /XObject << ... >>
  /Pattern << /P1 obj_ref /P2 obj_ref ... >>
>>
```

### Page stream emit — Stroke branching

Quando `Stroke.paint::Solid(c)`: emit `r g b RG` literal P261
preservado.

Quando `Stroke.paint::Gradient(g)`:
```
/Pattern CS
/P1 SCN
{:.2} w
```

`CS`/`SCN` (uppercase) para stroke; `cs`/`scn` (lowercase) para
fill (não usado P263 — Fill é Color literal).

Cross-path: `build_page_stream_type1`, `build_page_stream_cidfont`,
`build_page_stream_multifont` todos adaptados.

### Coords L3

Helper `compute_axial_coords(angle, x0, y0, w, h) -> (f64, f64, f64, f64)`:
- Bbox local do shape (não da página).
- Centro `(cx, cy) = (x0 + w/2, y0 + h/2)`.
- Direção `(dx, dy) = (cos(angle), sin(angle))`.
- Endpoints: `(cx - (w/2)*dx, cy - (h/2)*dy)` → `(cx + (w/2)*dx,
  cy + (h/2)*dy)`.

Coords em **espaço PDF** (Y já invertido pelo `build_page_stream`).

### Helpers internos novos

| Função | Responsabilidade |
|--------|------------------|
| `scan_all_gradients(doc, first_id)` | Pre-pass dedup `Arc::as_ptr`; aloca obj IDs |
| `compute_axial_coords(angle, x0, y0, w, h)` | (x0, y0, x1, y1) endpoints |
| `oklab_sample_stops(linear, n_samples)` | N stops intermédios em sRGB pós Oklab L1 |
| `pattern_resources_for_page(page, ptr_to_idx, refs)` | `/Pattern << /P1 X 0 R ... >>` page-level |

### Object IDs allocation

Cada gradient único aloca 3 IDs:
- `function_id`: Function dict (Type 2 ou Type 3).
- `shading_id`: Shading dict referenciando function.
- `pattern_id`: Pattern dict referenciando shading.

Total objects PDF = `+3 * count(gradients únicos)`.

### Limitações P263

- **Linear only** — Radial/Conic continuam comentários reserva
  em `entities/gradient.rs`; PDF emit prepara só Type 2.
- **Apenas Stroke** — Fill continua Color literal (refino futuro
  se Fill Paint estender).
- **Anti-alias assume true** (PDF default; vanilla scope-out
  ADR-0087).
- **Relative assume bounding-box** (vanilla default; scope-out
  ADR-0087).

Cross-references:
- ADR-0087 — Gradient Linear-only (IMPLEMENTADO P262;
  anotação cumulativa P263 documenta backend PDF).
- ADR-0027 — CIDFont/Identity-H (precedente arquitectural
  resources dict).
- P73 — Image stack dedup `Arc::as_ptr` (template arquitectural).
- P262 — Gradient L1+stdlib (precedente directo).
