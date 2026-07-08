# Prompt L0 — `infra/export/builder` — PdfBuilder
Hash do Código: c734340d

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0027 (CIDFont + Identity-H), ADR-0055 (multifont decisão 5)

---

## Contexto

Orquestrador L3 que constrói o ficheiro PDF — alocação de Object
IDs, construção do `/Catalog` + `/Pages` + per-page `/Page` dicts,
serialização final com xref + trailer.

Três caminhos paralelos:
- `build_helvetica` — fallback Type1 (sem font embedding; Latin-1).
- `build_cidfont` — single fonte TrueType/CFF embebida via Type0 + Identity-H + ToUnicode CMap.
- `build_multifont` — N fontes TrueType/CFF, cada `FrameItem::Text` dispatched para `/F{i+1}` baseado em `style.font`.

Centraliza chamadas aos helpers `super::scan_all_images`, `super::scan_all_gradients`, `super::collect_codepoints`, `super::collect_glyph_ids`, etc.

## §P520 — Larguras nominais para delta model do TJ

Em `build_cidfont` e `build_multifont`, construir um mapa
`glyph_to_nominal: HashMap<u16, i32>` que associa cada old `glyph_id`
à sua largura horizontal nominal (`face.glyph_hor_advance`). Este mapa
inclui:

1. Todos os `glyph_id` recolhidos por `collect_glyph_ids(doc)` (glifos
   reais usados por `FrameItem::Glyph` e `FrameItem::TextShaped`,
   incluindo ligatures produzidas pelo shaper).
2. Todos os `glyph_id` mapeados a partir de codepoints (`mappings`).

O mapa é passado para `PageContext::cidfont` / `PageContext::multifont`
através de `FontScenario` e consumido por `emit_shaped_pdf` para calcular
o delta `nominal - x_advance` no operador PDF `TJ`.

## §P568 — Subsetar glyphs do caminho fallback (`FrameItem::Text`)

**Data:** 2026-07-05

Em `build_cidfont` e `build_multifont`, além dos codepoints/glyphs já
recolhidos por `collect_codepoints` / `collect_glyph_ids`, o builder inclui
os codepoints devolvidos por `collect_text_codepoints(doc)`:

1. O conjunto de codepoints passado a `map_chars_to_glyphs` é a união de
   `collect_codepoints` com `collect_text_codepoints`.
2. O conjunto de glyph IDs usados no subset (`all_glyph_ids` / `extended_glyph_ids`)
   estende-se com `face.glyph_index(c)` para cada `c` em `collect_text_codepoints`.

Motivo: quando `try_shape` decide não shapear um texto (ex.: texto que só
contém espaços), o `FrameItem::Text` resultante ainda precisa de ter o seu
glyph presente no subset, caso contrário o leitor de PDF não consegue
selecionar/copiar esse caractere.

## §P560 — Descritor PDF conforme o tipo de fonte (TrueType vs CFF)

`build_cidfont` e `build_multifont` detectam se os bytes a embeber são uma
fonte TrueType (`glyf`) ou uma fonte CFF/OpenType (`CFF`/`CFF2`). O
descritor PDF emitido deve corresponder ao formato interno da fonte, porque
leitores como poppler e mupdf rejeitam uma fonte CFF embutida como
`/FontFile2` com `/Subtype /CIDFontType2`.

Regras:

1. Detecção: após obter `embed_font_data`, fazer `ttf_parser::Face::parse` e
   consultar `face.tables().cff`.
   - Se `cff.is_some()` → fonte CFF/OpenType.
   - Senão → fonte TrueType (`glyf`).
2. Para **TrueType** (comportamento existente):
   - `/Subtype /CIDFontType2` no dicionário `/Font` descendente.
   - `/FontFile2 {stream_id} 0 R` no `/FontDescriptor`.
   - Stream: `<< /Length {len} /Subtype /CIDFontType2 >>`.
   - Bytes do stream: a fonte TrueType completa (SFNT).
3. Para **CFF/OpenType**:
   - `/Subtype /CIDFontType0` no dicionário `/Font` descendente.
   - `/FontFile3 {stream_id} 0 R` no `/FontDescriptor`.
   - Stream: `<< /Length {len} /Subtype /CIDFontType0C >>`.
   - Bytes do stream: **apenas a tabela `CFF`** extraída do contêiner
     OpenType/SFNT via `ttf_parser::Face::table_data(Tag::from_bytes(b"CFF "))`.
     Leitores de PDF esperam o programa CFF puro, não o contêiner SFNT
     completo, quando o descritor diz `/CIDFontType0C`.
4. O ToUnicode CMap, o array `/W` e o operador `TJ` permanecem inalterados —
   apenas a envolvência do descritor de fonte muda.

## Restrições estruturais

- `PdfBuilder` é `pub(super)` — apenas `mod.rs` instancia.
- Métodos `build_helvetica/cidfont/multifont` são `pub(super)`; chamados por dispatch em mod.rs ou directamente em `build_multifont`.
- Helpers privados (`add`, `add_bytes`, `serialize`, `emit_*_xobjects`) ficam no impl block sem `pub`.
- Não emite ASCII directo — usa `emit_text_pdf` + `emit_glyph_pdf` de `super::stream`.
- Não conhece detalhes de gradient math — delega para `super::gradients::*`.

## Interface

```rust
pub(super) struct PdfBuilder { objects: Vec<(usize, Vec<u8>)> }
impl PdfBuilder {
    pub(super) fn new() -> Self;
    pub(super) fn build(self, doc: &PagedDocument, font_data: Option<&[u8]>) -> Vec<u8>;
    pub(super) fn build_multifont(self, doc: &PagedDocument, fonts: &[(FontList, Vec<u8>)], faces: &[Face<'_>]) -> Vec<u8>;
}
```

## Allocação de Object IDs (invariante)

Ordem canónica para todos os caminhos:
1. `/Catalog` = obj 1
2. `/Pages` = obj 2
3. Per-page `/Page` = obj 3..3+N
4. Per-page `/Contents` stream = obj 3+N..3+2N
5. Font objects (3 para Helvetica, 5 para CIDFont, 5*N para multifont)
6. Image XObjects (1 ou 2 cada — RGB + opcional SMask)
7. Gradient objects (3 cada — Function + Shading + Pattern) + sub-Function IDs
8. Link annotations (`/Type /Annot /Subtype /Link`) — alocados dinamicamente
   após todos os recursos, referenciados pelo `/Annots` de cada página.

Alocação dependente: gradients vêm após imagens; sub-Functions vêm após gradients;
annotations vêm após gradients; named destinations vêm após annotations;
bookmarks (`/Outlines`) vêm após named destinations.

## §P535 — Bookmarks PDF (`/Outlines`)

O `PdfBuilder` constrói uma árvore `/Outlines` a partir de
`PagedDocument::extracted_headings`:

1. Cada heading gera um objecto outline item com `/Title`, `/Parent`, `/Prev`,
   `/Next`, `/First`, `/Last`, `/Count` (quando tem filhos) e `/Dest`.
2. `/Dest` aponta para a página e posição `(x, y-up)` extraídas de
   `extracted_label_pages` / `extracted_label_positions` via auto-label do heading.
3. O dicionário raiz `/Outlines` é referenciado pelo `/Catalog` (objecto 1).
4. `/Count` num item com filhos é o número de filhos directos, com sinal
   negativo (entrada fechada por defeito), seguindo o vanilla 0.15.0.
5. `/Count` na raiz `/Outlines` é o número de bookmarks de topo (itens sem
   pai), com sinal positivo (abertos por defeito).

## §P536 — Metadados do documento (`/Info`)

O `PdfBuilder` **sempre** emite um dicionário `/Info`, independentemente de
`PagedDocument::document_info` estar vazio:

1. Campos presentes (`/Title`, `/Author`, `/Keywords`) são escritos como
   strings UTF-16BE com marca de ordem de bytes (`\xFE\xFF`) codificadas em
   hexadecimal (`<FEFF...>`). Isto evita dupla codificação de texto UTF-8
   proveniente do documento Typst e suporta qualquer carácter Unicode.
2. `/CreationDate` e `/ModDate` são escritos no formato
   `D:YYYYMMDDHHMMSS` (UTC do momento da compilação).
3. `/Creator` é preenchida com `typst-crystalline`.
4. O objecto `/Info` é alocado após todos os outros objectos e referenciado
   pelo `/Trailer` (`/Info X 0 R`).

## Critérios de verificação

- `PdfBuilder::new().build(doc, None)` produz PDF Helvetica para doc qualquer.
- `PdfBuilder::new().build(doc, Some(data))` produz PDF CIDFont se `data` parser TTF/OTF; senão fallback Helvetica.
- Fontes TrueType geram `/CIDFontType2` + `/FontFile2`.
- Fontes CFF/OpenType geram `/CIDFontType0` + `/FontFile3 /Subtype /OpenType`.
- PDF com texto árabe/hebraico (fallback CFF) renderiza correctamente em poppler
  (`pdftoppm`) e mupdf (`mutool draw`).
- Tests `pdf_header_correcto`, `pdf_termina_com_eof`, `pdf_tem_estrutura_valida` em `tests.rs` validam invariantes estruturais.

## Link annotations (P424/P463)

Para cada `FrameItem::Link` encontrado nas páginas do documento:
1. Coletar `target`, `pos` e `size` (coordenadas globais de página).
2. Converter coordenadas de Y-down (layout) para Y-up (PDF):
   `pdf_y = page_height - pos.y - size.height`.
3. Emitir um objeto dictionary conforme o `LinkTarget`:
   - URL externo (`LinkTarget::Url`):
     ```
     << /Type /Annot /Subtype /Link
        /Rect [x pdf_y (x+width) (pdf_y+height)]
        /Border [0 0 0]
        /A << /Type /Action /S /URI /URI (escaped_url) >> >>
     ```
   - Destino interno (`LinkTarget::Destination(Label)`):
     ```
     << /Type /Annot /Subtype /Link
        /Rect [x pdf_y (x+width) (pdf_y+height)]
        /Border [0 0 0]
        /A << /Type /Action /S /GoTo /D /name >> >>
     ```
     O nome `/name` referencia a entrada em `/Names /Dests` (P460).
4. Referenciar todos os annotations da página no `/Annots` do respetivo
   dicionário `/Page`.

Escopo: `QuadPoints` multi-linha e escaping avançado de nomes são scope-out.

## Named destinations (P460)

Para cada label registado em `PagedDocument.extracted_label_pages` e
`extracted_label_positions`:
1. Alocar IDs para `/Names` e `/Dests` após annotations.
2. Construir dicionário `/Dests` com entradas `/name [page_ref /XYZ x y null]`.
3. Coordenadas Y convertidas de Y-down (layout) para Y-up (PDF):
   `pdf_y = page_height - pos.y`.
4. Editar o `/Catalog` (objeto 1) para incluir `/Names {names_id} 0 R`.

Nomes vazios ou com caracteres fora do conjunto de nomes PDF são escapados
para notação hexadecimal `<hex>`.

## Determinismo

Não usa `SystemTime`, RNG, ou parallel iteration sobre ordem emergente.
HashMaps internos só são consultados (não iterados para output).
`flate2` é determinístico em toolchain fixa.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-19 | Criação em P307c | `builder.md` |
| 2026-07-04 | P560 — descritor PDF distinto para fontes TrueType e CFF/OpenType | `builder.md`, `builder.rs` |
