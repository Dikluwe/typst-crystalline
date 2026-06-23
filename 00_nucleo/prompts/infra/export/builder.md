# Prompt L0 — `infra/export/builder` — PdfBuilder
Hash do Código: 9d76cf4b

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
- `build_cidfont` — single TTF embebida via Type0 + Identity-H + ToUnicode CMap.
- `build_multifont` — N TTFs, cada `FrameItem::Text` dispatched para `/F{i+1}` baseado em `style.font`.

Centraliza chamadas aos helpers `super::scan_all_images`, `super::scan_all_gradients`, `super::collect_codepoints`, `super::collect_glyph_ids`, etc.

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
annotations vêm após gradients.

## Critérios de verificação

- `PdfBuilder::new().build(doc, None)` produz PDF Helvetica para doc qualquer.
- `PdfBuilder::new().build(doc, Some(data))` produz PDF CIDFont se `data` parser TTF/OTF; senão fallback Helvetica.
- Tests `pdf_header_correcto`, `pdf_termina_com_eof`, `pdf_tem_estrutura_valida` em `tests.rs` validam invariantes estruturais.

## Link annotations (P424)

Para cada `FrameItem::Link` encontrado nas páginas do documento:
1. Coletar `url`, `pos` e `size` (coordenadas globais de página).
2. Converter coordenadas de Y-down (layout) para Y-up (PDF):
   `pdf_y = page_height - pos.y - size.height`.
3. Emitir um objeto dictionary:
   ```
   << /Type /Annot /Subtype /Link
      /Rect [x pdf_y (x+width) (pdf_y+height)]
      /Border [0 0 0]
      /A << /Type /Action /S /URI /URI (escaped_url) >> >>
   ```
4. Referenciar todos os annotations da página no `/Annots` do respetivo
   dicionário `/Page`.

Escopo: apenas URI externo; links internos (`#link("<label>")`) e
`QuadPoints` multi-linha são scope-out.

## Determinismo

Não usa `SystemTime`, RNG, ou parallel iteration sobre ordem emergente.
HashMaps internos só são consultados (não iterados para output).
`flate2` é determinístico em toolchain fixa.
