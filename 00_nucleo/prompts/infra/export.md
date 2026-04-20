# Prompt L0 — `infra/export` — Exportador Físico de Documentos
Hash do Código: 99ae3678

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
