# Prompt L0 — `infra/export/builder` — PdfBuilder
Hash do Código: 0df1c4d6

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

### §P772u — `glyph_to_nominal` tem de usar a MESMA instância que `/W`

**Data:** 2026-07-17

O modelo de delta do TJ (acima) só é correcto se `nominal` (usado para
calcular o delta) e `w0` (a largura declarada no array `/W`, lida pelo
leitor de PDF em tempo de render) vierem da **mesma** face/instância —
a fórmula `deslocamento = w0 - delta = w0 - (nominal - x_advance)`
só se reduz a `x_advance` quando `w0 == nominal`.

Em `build_multifont`, `/W` é construído a partir de `face_for_widths`
(`subset_face.as_ref().unwrap_or(face)`, onde `subset_face` vem de
`embed_data` **pós-instanciação** — ver §P530/`instantiate_variable_font`).
`glyph_to_nominal`, por ser calculado em espaço de GID **original**
(pré-subset — o mesmo espaço que `ShapedGlyph::glyph_id`, produzido pelo
shaper a partir da fonte variável crua), **não pode** usar `face_for_widths`
directamente (GIDs pós-subset não correspondem). Em vez disso, aplicar a
mesma variação de eixo (`axis_variations_for_font_variant`) a um **clone**
de `face` (a face original, em espaço de GID pré-subset) antes de ler
`glyph_hor_advance` — **só quando `instantiate_variable_font` teve
sucesso** (`instancing_applied`). Se a instanciação falhar (ex.: Python/
fontTools ausente — P667), `embed_data`/`/W` ficam na instância por
omissão (fallback já existente, com aviso `eprintln!`); `glyph_to_nominal`
tem de acompanhar esse fallback e **não** aplicar a variação nesse caso,
para os dois lados do delta continuarem a vir da mesma instância (mesmo
que "errada" visualmente — problema pré-existente e já avisado,
independente desta correcção):

```rust
let mut nominal_face = face.clone();
if instancing_applied {
    for v in &axis_vars {
        nominal_face.set_variation(v.tag, v.value);
    }
}
// usar nominal_face.glyph_hor_advance(...) para construir glyph_to_nominal
```

**Sintoma da violação** (achado por instrumentação, P772u): quando
`glyph_to_nominal` usa a face **sem** variação enquanto `/W` usa a face
**com** variação, o delta do TJ mede a diferença de peso como se fosse
kerning, e o deslocamento real no leitor de PDF passa a ser
`2×x_advance_correcto − nominal_sem_variação` — um avanço adicional
igual à própria variação de peso, por glifo. Em texto contínuo (várias
letras seguidas), este excesso acumula-se e pode exceder a largura do
espaço entre palavras seguintes, colapsando-as visualmente (confirmado
com Cantarell-VF.otf, CFF2/HVAR, wght=800 — `#image`/`#text` não
afectados, é específico do caminho de texto variável). `build_cidfont`
(fonte única, sem instanciação nesse caminho) não sofre disto porque
`glyph_to_nominal` e `/W` usam consistentemente a mesma face sem
variação — mas por isso também não suporta peso variável real (fonte
embutida fica sempre na instância por omissão).

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

## §P560/§P772u — Descritor PDF conforme o tipo de fonte (TrueType vs CFF vs CFF2)

`build_cidfont` e `build_multifont` detectam se os bytes a embeber são uma
fonte TrueType (`glyf`) ou uma fonte CFF/CFF2/OpenType. O descritor PDF
emitido deve corresponder ao formato interno da fonte, porque leitores como
poppler e mupdf rejeitam uma fonte CFF/CFF2 embutida como `/FontFile2` com
`/Subtype /CIDFontType2`.

**Correcção P772u:** a versão original desta regra (P560) só verificava
`face.tables().cff` (CFF1) — `ttf_parser` expõe `cff` e `cff2` como campos
**distintos** em `Face::tables()`; uma fonte CFF2 (ex.: fontes variáveis
OpenType como Cantarell-VF.otf) sem tabela `CFF ` caía silenciosamente no
ramo TrueType (`/CIDFontType2`/`/FontFile2`), mesmo não tendo `glyf`. A regra
abaixo já inclui a detecção de CFF2.

Regras:

1. Detecção: após obter `embed_font_data`, fazer `ttf_parser::Face::parse` e
   consultar, por ordem, `face.tables().cff` e depois `face.tables().cff2`.
   - Se `cff.is_some()` → fonte CFF1/OpenType.
   - Senão, se `cff2.is_some()` → fonte CFF2/OpenType (variável).
   - Senão → fonte TrueType (`glyf`).
2. Para **TrueType** (comportamento existente):
   - `/Subtype /CIDFontType2` no dicionário `/Font` descendente.
   - `/FontFile2 {stream_id} 0 R` no `/FontDescriptor`.
   - Stream: `<< /Length {len} /Filter /FlateDecode /Subtype /CIDFontType2 >>`
     quando comprimido; `<< /Length {len} /Subtype /CIDFontType2 >>` quando não.
   - Bytes do stream: a fonte TrueType completa (SFNT), **comprimida com FlateDecode** (P883)
     — **excepto** quando o stream excede `MAX_COMPRESS_FONT_STREAM` (256 KB, P940).
   - **P883** — o vanilla 0.15.0 comprime os streams de fonte; o cristalino
     passou a fazer o mesmo, reduzindo o tamanho do PDF sem alterar o conteúdo
     da fonte. Em caso de falha do compressor, emite o stream sem compressão.
   - **P940** — quando o subset falha (ex.: fontes CBDT/emoji, `subsetter`
     devolve `UnknownKind`), o export embute a fonte inteira (~10 MB) e a
     compressão Flate domina o `render_ms` (~300 ms medidos). Acima de
     `MAX_COMPRESS_FONT_STREAM`, o stream é emitido sem compressão: o custo de
     CPU cai para uma cópia de memória, ao preço de um PDF maior. O caminho
     normal (subset bem-sucedido) continua comprimido.
3. Para **CFF1/OpenType**:
   - `/Subtype /CIDFontType0` no dicionário `/Font` descendente.
   - `/FontFile3 {stream_id} 0 R` no `/FontDescriptor`.
   - Stream: `<< /Length {len} /Filter /FlateDecode /Subtype /CIDFontType0C >>`.
   - Bytes do stream: **apenas a tabela `CFF`** extraída do contêiner
     OpenType/SFNT via `ttf_parser::Face::table_data(Tag::from_bytes(b"CFF "))`,
     **comprimida com FlateDecode** (P883).
     Leitores de PDF esperam o programa CFF puro, não o contêiner SFNT
     completo, quando o descritor diz `/CIDFontType0C`.
   - **P882** — a implementação P560/P797 embutia o SFNT completo
     (`/Subtype /OpenType`) também para CFF1; isto aumentava o PDF em ~1.5 KB
     por ocorrência de fonte em comparação com o vanilla 0.15.0, que embute
     só o programa CFF puro. Corrigido para CFF1 usar `/CIDFontType0C` + CFF
     bare; CFF2 continua com `/OpenType` porque o spec PDF não define um
     subtipo para programa CFF2 puro.
4. Para **CFF2/OpenType** (P772u): **não existe** subtype PDF para "programa
   CFF2 puro" (ISO 32000-2 §9.9.4 só define `Type1C`, `CIDFontType0C` — bare
   CFF1 — e `OpenType` — contêiner completo). Por isso:
   - `/Subtype /CIDFontType0` no dicionário `/Font` descendente (mesmo
     subtype CID que CFF1 — a distinção fica só no `FontFile`/stream).
   - `/FontFile3 {stream_id} 0 R` no `/FontDescriptor`.
   - Stream: `<< /Length {len} /Filter /FlateDecode /Subtype /OpenType >>`.
   - Bytes do stream: `font_data` **completo** (contêiner SFNT/OpenType, não
     uma tabela extraída — não há equivalente de "CFF2C"), **comprimido com
     FlateDecode** (P883).
5. O ToUnicode CMap, o array `/W` e o operador `TJ` permanecem inalterados —
   apenas a envolvência do descritor de fonte muda. **Excepção:**
   `glyph_to_nominal` (usado para o delta do TJ, §P520) tem de continuar a
   usar a MESMA variação de eixo que gerou o `/W` — ver §P772u em §P520
   acima; esta é uma correcção independente da detecção de subtype.

## §P884 — Content streams de página comprimidos com FlateDecode

**Data:** 2026-07-24

Os content streams de página (`/Contents`) são comprimidos com `/Filter
/FlateDecode` quando a compressão reduz o tamanho em relação ao stream em
claro. Quando a compressão não é rentável (streams muito pequenos) ou falha,
emite-se o stream sem `/Filter`.

### Implementação

- Adicionar helper `build_content_stream(stream_data: &[u8]) -> Vec<u8>` em
  `builder.rs`.
- Usar `compress_zlib` (já usado para imagens e fontes) nos bytes devolvidos
  por `build_page_stream`.
- Aplicar o helper em `build_helvetica`, `build_cidfont` e `build_multifont`,
  nos três pontos onde o content stream de página é adicionado via
  `self.add_bytes(stream_id, ...)`.
- O dicionário do stream comprimido é `<< /Length {len} /Filter /FlateDecode
  >>`; o não-comprimido mantém `<< /Length {len} >>`.

### Critérios de verificação

- Teste `p884_content_streams_comprimidos_com_flate_decode`: documento com
  texto repetido — o marcador não aparece em claro no PDF bruto, mas é
  recuperável ao descomprimir os content streams.
- Testes existentes que inspeccionam operadores PDF dentro dos content streams
  devem usar `extract_page_content_streams_text(&pdf)` (helper em `tests.rs`)
  para descomprimir automaticamente antes de verificar strings.
- Validação em poppler (`pdftoppm`, `pdftotext`) e ghostscript para documentos
  de benchmark após a mudança.

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
6. Opcional: objecto `/ICCBased` sRGB partilhado (apenas se houver JPEGs RGB no documento)
7. Image XObjects (1 ou 2 cada — RGB + opcional SMask)
8. Gradient objects (3 cada — Function + Shading + Pattern) + sub-Function IDs
9. Link annotations (`/Type /Annot /Subtype /Link`) — alocados dinamicamente
   após todos os recursos, referenciados pelo `/Annots` de cada página.

Alocação dependente: gradients vêm após imagens; sub-Functions vêm após gradients;
annotations vêm após gradients; named destinations vêm após annotations;
bookmarks (`/Outlines`) vêm após named destinations.

## §P777 — Perfil ICC sRGB para JPEGs RGB

**Data:** 2026-07-16

Para replicar o vanilla (krilla), JPEGs RGB usam `/ColorSpace [/ICCBased <id> 0 R]`
em vez de `/DeviceRGB` directo. O perfil ICC é um perfil sRGB compacto (480 bytes,
compatível com lcms2) embutido como constante em L3.

Regras:

1. O builder detecta se o documento contém pelo menos um JPEG RGB antes de
   alocar object IDs para imagens (`has_rgb_jpeg`).
2. Se houver JPEGs RGB, reserva um object ID para o perfil ICC partilhado
   imediatamente após os objectos de fonte e antes do primeiro XObject de imagem.
3. O ID do perfil ICC é passado a `scan_all_images`, que o associa a cada
   `ImageXObject::Jpeg` cujo SOF marker indique 3 componentes de cor.
4. `emit_image_xobjects` emite o stream `/ICCBased` antes dos JPEGs que o
   referenciam.
5. JPEGs grayscale (`/DeviceGray`) e CMYK (`/DeviceCMYK`) não referenciam o
   perfil ICC; continuam a usar ColorSpace directo.

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

## §P611 — Stream de metadados XMP

**Data:** 2026-07-08

Além do dicionário `/Info` (P536/P601), o `PdfBuilder` emite sempre um
stream `/Type /Metadata /Subtype /XML` com um pacote XMP mínimo,
referenciado a partir de `/Metadata` no catálogo (objeto 1).

Motivo: paridade com o vanilla 0.15.0, que escreve XMP mesmo quando o
documento não tem metadados de utilizador.

### Conteúdo do pacote XMP

O XML segue a estrutura exacta gerada pelo vanilla (krilla + xmp-writer),
com os mesmos namespaces e ordem de campos:

```xml
<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="xmp-writer">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
      xmlns:dc="http://purl.org/dc/elements/1.1/"
      xmlns:xmp="http://ns.adobe.com/xap/1.0/"
      xmlns:xmpMM="http://ns.adobe.com/xap/1.0/mm/"
      xmlns:xmpTPg="http://ns.adobe.com/xap/1.0/t/pg/"
      xmlns:pdf="http://ns.adobe.com/pdf/1.3/">
      <dc:title><rdf:Alt><rdf:li xml:lang="x-default">...</rdf:li></rdf:Alt></dc:title>
      <pdf:Keywords>...</pdf:Keywords>
      <dc:creator><rdf:Seq><rdf:li>...</rdf:li></rdf:Seq></dc:creator>
      <xmp:CreatorTool>...</xmp:CreatorTool>
      <dc:language><rdf:Bag><rdf:li>en</rdf:li></rdf:Bag></dc:language>
      <xmp:ModifyDate>...</xmp:ModifyDate>
      <xmp:CreateDate>...</xmp:CreateDate>
      <xmpTPg:NPages>...</xmpTPg:NPages>
      <dc:format>application/pdf</dc:format>
      <xmpMM:InstanceID>...</xmpMM:InstanceID>
      <xmpMM:DocumentID>...</xmpMM:DocumentID>
      <xmpMM:RenditionClass>proof</xmpMM:RenditionClass>
      <pdf:PDFVersion>1.7</pdf:PDFVersion>
    </rdf:Description>
  </rdf:DF>
</x:xmpmeta>
<?xpacket end="r"?>
```

Regras de geração:

1. **Sempre emitido**, mesmo que `PagedDocument::document_info` esteja vazio.
2. Campos condicionais a metadados do utilizador (`dc:title`, `pdf:Keywords`,
   `dc:creator`) só aparecem quando `document_info` os tem preenchidos.
3. `xmp:CreatorTool` usa `typst-crystalline` (divergência de branding
   permitida; o vanilla usa `Typst 0.15.0`).
4. `dc:language` usa `en` como valor fixo até o cristalino propagar a
   linguagem do documento para `PagedDocument`.
5. `xmp:CreateDate` e `xmp:ModifyDate` usam o mesmo instante de `/Info`
   (P601), no formato ISO 8601 com offset (`YYYY-MM-DDTHH:MM:SS±HH:MM`).
   O vanilla usou o offset local (`-03:00`); o cristalino segue o mesmo
   instante, convertido para offset local via `time::OffsetDateTime`.
6. `xmpTPg:NPages` é o número de páginas de `PagedDocument`.
7. `dc:format` é sempre `application/pdf`.
8. `xmpMM:InstanceID` e `xmpMM:DocumentID` são base64 de 16 bytes.
   Por defeito (P615), ambos são aleatórios, seguindo o vanilla 0.15.0.
   - Em produção, ambos são gerados aleatoriamente e independentemente.
   - Durante testes (`CRYSTALLINE_PDF_FIXED_EPOCH` definida), ambos usam
     valores fixos (`typst-crystalline/xmp-instance` e
     `typst-crystalline/xmp-document`), garantindo snapshots deterministas.
   - **P617** — se um `DocumentID` externo de 16 bytes for fornecido ao
     `PdfBuilder` (via `--document-id` / `CRYSTALLINE_DOCUMENT_ID`), esse
     valor é usado para `xmpMM:DocumentID` em vez do aleatório. O
     `xmpMM:InstanceID` continua sempre aleatório (ou fixo em testes),
     porque cada compilação é uma instância diferente.
9. `xmpMM:RenditionClass` é sempre `proof`.
10. `pdf:PDFVersion` é sempre `1.7`.
11. Caracteres especiais no título/autor/palavras-chave são escapados para
    entidades XML (`&`, `<`, `>`, `"`, `'`).

### Alocação de Object IDs

O stream `/Metadata` é alocado **após** todos os outros objectos (incluindo
`/Info`), logo antes de `serialize`. O seu ID é guardado em `PdfBuilder` e
referenciado no `/Catalog` (objeto 1) via `/Metadata {id} 0 R`.

## §P811 — Documento sem páginas emite 1 página em branco

**Data:** 2026-07-21

Quando `PagedDocument.pages` está vazio (documento sem conteúdo visível —
ex.: ficheiro vazio, `$ $`, ` `, `$frak()$` antes da validação de P811), os
caminhos de build usavam `doc.pages.len().max(1)` para o `/Kids` mas o loop
de emissão iterava as páginas reais (0) — o `/Pages` declarava
`/Kids [3 0 R] /Count 1` sem o objecto 3 existir: **PDF inválido** (`Kid
object (page 1) is wrong type (null)`, medido com `pdfinfo`). O vanilla
emite **1 página em branco** (A4 default) nestes casos (medido para ficheiro
vazio, `$ $` e ` `).

Regra: `PdfBuilder::build()` (ponto único por onde passam todos os exports
públicos) sintetiza uma `Page` em branco A4 (`595.28 × 841.89`, sem items)
quando `doc.pages.is_empty()`, antes de despachar para qualquer caminho
(`build_helvetica`/`build_cidfont`/`build_multifont`).

## §P675 — Evitar walks duplicados do documento em `build_multifont`

**Data:** 2026-07-10

Em `build_multifont`, a colecção de textos shaped (`collect_shaped_cluster_texts`) era feita dentro do loop por fonte. Como esta função percorre todo o `PagedDocument`, o custo escalava linearmente com o número de fontes resolvidas. O P675 identificou que, no `macro-10x`, esta colecção consumia ~91 % do tempo da fase `faces` do export.

Regra:
1. `collect_shaped_cluster_texts(doc)` deve ser chamada **uma única vez** antes do loop por fonte.
2. O `Vec<(u16, String)>` resultante é partilhado entre todas as fontes.
3. Cada fonte continua a fazer o seu próprio `remap_glyph_id` e a manter o seu `seen_to_unicode_gids`, preservando a semântica e os mapeamentos ToUnicode por fonte.

## §P805a — ToUnicode de ligaduras também no embed integral (fallback P797)

**Data:** 2026-07-21

Em `build_cidfont`, os cluster texts shaped (`collect_shaped_cluster_texts`)
só eram adicionados ao ToUnicode quando `glyph_mapping` não era vazio (subset
TrueType). No fallback de embed integral de P797 (CFF, mapping vazio), as
ligaduras ("fi" → gid `f_i`) ficavam sem entrada ToUnicode — **renderizavam
correctamente** (a fonte integral contém o glifo), mas a extracção
(`pdftotext`, poppler) perdia os caracteres ("fieri" → "eri").

Regra: os cluster texts shaped são **sempre** incluídos no ToUnicode; o
`remap_glyph_id` só se aplica quando há subset (mapping não vazio) — no embed
integral o gid final é o próprio `old_gid`. Descoberto em P805 (a validação
de `#lorem(30)` divergia só em palavras com "fi").

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-19 | Criação em P307c | `builder.md` |
| 2026-07-04 | P560 — descritor PDF distinto para fontes TrueType e CFF/OpenType | `builder.md`, `builder.rs` |
| 2026-07-08 | P611 — stream de metadados XMP | `builder.md`, `builder.rs` |
| 2026-07-10 | P675 — evitar walks duplicados do documento em `build_multifont` | `builder.md`, `builder.rs` |
| 2026-07-17 | P772u — detecção de CFF2 em `font_embedding_data` (fontes CFF2 caíam no ramo TrueType); `glyph_to_nominal` em `build_multifont` passa a usar a mesma variação de eixo que `/W`, corrigindo colapso de espaço entre palavras em fontes variáveis a pesos altos (Cantarell-VF, CFF2/HVAR) | `builder.md`, `builder.rs` |
| 2026-07-23 | P882 — CFF1/OpenType embute programa CFF puro (`/CIDFontType0C`) em vez de SFNT completo (`/OpenType`); CFF2 mantém `/OpenType` | `builder.md`, `builder.rs` |
| 2026-07-23 | P883 — streams de fonte comprimidos com FlateDecode (paridade com vanilla 0.15.0); teste de regressão para CFF1 bare vs CFF2 OpenType | `builder.md`, `builder.rs`, `tests.rs` |
| 2026-07-24 | P884 — content streams de página comprimidos com FlateDecode; testes ajustados para descomprimir via `extract_page_content_streams_text` | `builder.md`, `builder.rs`, `tests.rs` |
| 2026-07-31 | P940 — streams de fonte acima de 256 KB (fallback integral quando subset falha, ex.: CBDT/emoji) emitidos sem FlateDecode; elimina ~300 ms de `render_ms` de compressão | `builder.md`, `builder.rs`, `subset.rs` |

## Critérios de verificação

- `PdfBuilder::new().build(doc, None)` produz PDF Helvetica para doc qualquer.
- `PdfBuilder::new().build(doc, Some(data))` produz PDF CIDFont se `data` parser TTF/OTF; senão fallback Helvetica.
- Fontes TrueType geram `/CIDFontType2` + `/FontFile2`.
- Fontes CFF1/OpenType geram `/CIDFontType0` + `/FontFile3 /Subtype /CIDFontType0C` (apenas a tabela `CFF ` extraída), com stream comprimido por FlateDecode; fontes CFF2/OpenType (variáveis) geram `/CIDFontType0` + `/FontFile3 /Subtype /OpenType` (contêiner SFNT completo), também comprimido por FlateDecode.
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

## P836 — instanciação com eixos explícitos

Em `build_multifont`, a lista de eixos passada a
`instantiate_variable_font` deixa de ser só
`axis_variations_for_font_variant(font_variant)` e passa a ser a fusão
derivados + explícitos (`axis_variations_for_text_style` aplicada à
chave `(FontList, FontVariant, FontVariations)`), de modo que a fonte
embutida é instanciada nas coordenadas pedidas por
`#text(variations:)` — os contornos dos glifos no PDF refletem o eixo,
não só os avanços.

## P906 — `per_font_glyph_reverse` partilhado com `emit_glyph_pdf`

Ver `export/stream.md` §P906 (achado completo — `/F1` hardcoded +
remap de subsetting em falta em `emit_glyph_pdf`).

`build_multifont` já computava `glyph_reverse = build_math_glyph_reverse_map
(face)` por fonte (P45/DEBT-9, dentro do `for face in faces`) — só para
registar glifos de esticamento no subset. Passa também a acumular em
`per_font_glyph_reverse: Vec<HashMap<u16, char>>` (um `.push(glyph_reverse)`
a mais, sem recomputar nada) e a passar esse vector a `PageContext::
multifont` (novo 9º parâmetro), que o expõe em
`FontScenario::Multifont::per_font_glyph_reverse` para `emit_glyph_pdf`
resolver `/Fn` por `glyph_id` sem precisar de `style`.
