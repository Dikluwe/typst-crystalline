# Prompt L0 — `infra/export/builder` — PdfBuilder
Hash do Código: 539a27bd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def

## P1140.20.2 — caixas e camadas PDF

MediaBox cobre canvas incluindo bleed. Bleed não zero cria TrimBox
`[left top left+width top+height]`; zero omite. Stream pinta fill PDF,
background, body, foreground, com origem física. Bytes não são oracle.

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
   - **P941** — fontes cujos glifos usados têm imagem raster (CBDT/CBLC) são
     desenhadas como **imagens XObject**, não embutidas como fonte — ver
     §P941 abaixo. Quando todos os glifos usados de uma fonte são bitmap,
     essa fonte não gera recurso `/Font` nem stream de fonte.
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

### P1360 — exceção explícita para PDF pretty

`PdfBuilder` recebe `PdfSerializationMode` separado de `StreamMode`. Em
`Compact`, preserva integralmente P884. Em `Pretty`, os três caminhos de
build emitem os content streams de página com `<< /Length N >>`, sem
`/FlateDecode`, preservando em claro os bytes já produzidos por
`build_page_stream`. Compressão de fontes, imagens, ICC e metadata permanece
inalterada.

Pretty não troca operadores verbose/compact, não liga/desliga tags, não muda
IDs, xref, trailer, geometria ou texto. A aceitação observa stream de página
legível e PDF parseável; não exige bytes, line count ou serializer krilla
idênticos.

## §P941 — Glifos bitmap (CBDT/CBLC) como imagens XObject, não fonte embutida

**Data:** 2026-07-31

### Contexto e problema

Fontes bitmap por CBDT/CBLC (ex.: Noto Color Emoji) **não são subsettables**
(`subsetter::subset` devolve `UnknownKind`, medido em P940). O caminho actual
embute a fonte inteira (~10.8 MB) como `/CIDFontType2`, o que:

1. Produz PDFs ~135× maiores que o vanilla (~10 MB vs ~80 KB).
2. Renderiza os emojis **monocromáticos** (incorrecto — o vanilla renderiza a
   cores, porque não embute a fonte).
3. Custa tempo de CPU proporcional ao tamanho da fonte (cópia/compressão do
   stream, medido em P940).

O vanilla resolve os três problemas da mesma forma: **não embute a fonte
bitmap** — desenha cada glifo bitmap usado como uma imagem XObject no PDF
(krilla `text/glyph/bitmap.rs`).

### Solução

Para cada glifo usado no documento que tenha imagem raster na fonte
(`ttf_parser::Face::glyph_raster_image`), emitir uma **imagem XObject** em vez
de um glifo de fonte. A fonte CBDT deixa de ser embutida quando todos os seus
glifos usados são bitmap.

#### Fronteira com a coleta raster

`03_infra/src/export/bitmap_glyphs.rs` possui a descoberta dos glyph ids por
face, a extração do maior strike PNG e a normalização pelo pipeline de
imagens. O contrato interno dessa coleta é propriedade de
`infra/export/bitmap_glyphs.md`. Este builder recebe o mapa normalizado e
possui a alocação/deduplicação dos XObjects e sua integração nos recursos PDF.

#### Criação de XObjects (dedup por glifo)

- Cada glifo bitmap **único** (por fonte, por glyph id) gera **um** XObject de
  imagem (mais um SMask se tiver alpha). Glifos repetidos no documento
  referenciam o mesmo XObject — não são re-extraídos nem re-embutidos.
- Os XObjects de glifos bitmap usam o mesmo mecanismo de emissão de imagens já
  existente (`process_png_for_pdf` + dicionário `/XObject`), com IDs alocados
  depois das imagens/gradientes do documento.

#### Emissão no stream de página

Para um `FrameItem::TextShaped`, cada glifo é avaliado:

- **Se o glifo está no mapa de bitmap glyphs**: emite um *image draw* na
  posição do glifo e avança o cursor pelo `x_advance` (o espaço na linha já
  foi calculado pelo shaper a partir do `hmtx` da fonte — o glifo bitmap
  ocupa o mesmo espaço que ocuparia como glifo de fonte).
- **Senão**: emite o glifo no array `TJ` como hoje.

Posicionamento (fórmula derivada de krilla `text/glyph/bitmap.rs:37-52`,
a verificar visualmente na Fase B com `mutool draw`):

```text
scale  = font_size / pixels_per_em          // pontos PDF por pixel do bitmap
w_pt   = width  * scale
h_pt   = height * scale
x_pt   = cursor_x + x_offset_pt + bearing_x * scale
y_top  = base_y   - bearing_y * scale       // ver nota de sinal abaixo
draw:  q  w_pt 0 0 h_pt  x_pt  (y_top - h_pt)  cm  /ImN Do  Q
```

- `x_offset_pt = x_offset * font_size / units_per_em` (mesma conversão usada
  no operador `TJ`).
- `cursor_x` começa em `pos_x` e avança `x_advance * font_size / units_per_em`
  por glifo.
- **Sinal confirmado na Fase B:** `RasterGlyphImage.y` para NotoColorEmoji é
  `-27` (pixels). A fórmula `y_bottom = base_y + bearing_y * scale` posiciona
  os emojis correctamente na linha de base (verificado com `mutool draw` e
  `pdftoppm`, sem avisos de XObject desconhecido).

#### Dicionário de recursos `/XObject`

As entradas dos glifos bitmap (`/ImN id 0 R`) são **fundidas** no bloco
`/XObject << ... >>` das imagens do documento (helper `merge_bitmap_xobjects`).
Sem isto, ficavam fora do dicionário e os leitores PDF não resolviam a
referência (medido: `XObject 'Im3' is unknown` no poppler, render
monocromático por substituição de fonte no mutool).

#### Fonte CBDT não embutida

- Se **todos** os glifos usados de uma fonte estão no mapa de bitmap glyphs,
  a fonte **não é embutida** (sem recurso `/Font` para ela). O texto ao redor
  continua a ser calculado pelas métricas já extraídas pelo shaper (`hmtx`),
  que não dependem da fonte embutida.
- Se a fonte tiver glifos usados **mistas** (bitmap + outline), a fonte é
  embutida (subset normal) para os glifos outline e os bitmap saem como
  imagens. (Não ocorre para NotoColorEmoji, que é 100% bitmap nos glifos usados.)

#### ToUnicode e extracção de texto

Glifos emitidos como imagem não têm entrada ToUnicode (paridade com o vanilla,
que também não gera ToUnicode para imagens). A extracção de texto (`pdftotext`)
perde os emojis — comportamento idêntico ao vanilla.

#### Limitação conhecida (fora de escopo, pré-existente)

O desenho de glifos bitmap como imagem só acontece quando o **shaper escolhe a
fonte bitmap** (Noto Color Emoji) para o glifo. Para alguns codepoints emoji
(🔥 ✨ 📊 🎭 🎬 🏆 🌍), o fallback do shaper escolhe outras fontes
(FreeMono/FreeSans/Noto Sans Symbols2) em vez de Noto Color Emoji — esses
glifos continuam a ser renderizados como texto monocromático nessas fontes.
Medido em P941: a cobertura de NotoColorEmoji inclui esses codepoints, mas o
scoring de `select_fallback` penaliza NotoColorEmoji (`isFixedPitch=1` → flag
`mono=true`, que perde o critério `mono_match` contra o `like` não-monospace),
enquanto FreeMono (`isFixedPitch=0` → `mono=false`) e companhia ganham. O
vanilla escolhe NotoColorEmoji para todos — divergência pré-existente do
fallback (P838/P543), não do caminho de exportação. Candidato a passo próprio.

#### Integração

- `builder.rs` (`build_cidfont`/`build_multifont`): chama
  `collect_bitmap_glyphs` por fonte, cria os XObjects, passa o mapa
  `gid → BitmapGlyphRef` (nome do XObject, dimensões, bearings, ppem) ao
  `FontScenario`/`PageContext`.
- `stream.rs` (`emit_shaped_pdf`): consome o mapa e emite os *image draws*
  intercalados com o `TJ` conforme acima.

#### Critérios de verificação

```
Dado um documento com um único emoji
Quando exportado para PDF
Então contém exactamente um XObject de imagem e zero fontes CBDT embutidas

Dado um documento com o mesmo emoji repetido N vezes
Quando exportado para PDF
Então contém um único XObject de imagem e N referências `Do`

Dado um documento misto (emoji + texto latino/CJK)
Quando exportado para PDF
Então o texto não-emoji usa o caminho normal de fonte (subset) e os emojis
usam imagens XObject

Dado o PDF de utf8-emoji.typ
Quando renderizado com `mutool draw`
Então os emojis aparecem a cores, posicionados na linha de base do texto
```

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
| 2026-07-31 | P941 — glifos bitmap (CBDT/CBLC) desenhados como imagens XObject, não embutidos como fonte; desenho em §P941 | `builder.md`, `builder.rs`, `stream.rs`, `bitmap_glyphs.rs` |
| 2026-08-26 | P1196 — owner da coleta raster individualizado; builder retém integração e emissão de recursos | `builder.md`, `builder.rs` |
| 2026-09-22 | P1475 — trailer `/ID` universal, `pdf:PDFVersion` dinâmico, XMP `pdfaid` + `/OutputIntent` + supressão de `/Info` gated a A2/A3/A4 | `builder.md`, `builder.rs` |
| 2026-09-22 | P1476 — `/AFRelationship`+`/AF` gated a A3/A4f/A4e, extensão de `/Info`/`/OutputIntent`/`pdfaid` a A4f/A4e | `builder.md`, `builder.rs` |

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

## P950 — `BaseFont`/`FontName` com o nome real da fonte (não genérico)

**Achado do dono** (investigação de P949): o PDF exportado traz
`BaseFont`/`FontName` = `CrystallineFont[N]` (+ prefixo `AAAAAA+` de subset,
P517) — genérico, sem referência à família real. O vanilla embute com o nome
PostScript real (`GOKERJ+NewCMMath-Book` — prefixo aleatório de 6 letras +
nome PS). É um defeito de interoperabilidade (substituição de fonte por
leitores externos, triagem de PDFs) e causou confusão real na investigação
("fonte diferente" quando era subsetting normal com nome genérico).

**Decisão**: `base_font_name`/`name` passam a derivar do **nome real da fonte**,
lido da tabela `name` dos bytes da fonte embutida (sub-rotina nova
`real_base_name(font_data, fallback)`): prioridade PostScript (name_id 6) →
full name (4) → família (1) → `fallback` (o genérico actual). Espaços são
removidos (convenção BaseFont). O **prefixo determinístico `AAAAAA+` de P517
mantém-se** (reprodutibilidade de build — decisão registada: não adoptar o
prefixo aleatório do vanilla). O **nome do recurso interno** (`/F1`,
`/CrystallineFontN` no dicionário `/Resources` e no content stream) **não
muda** — é interno ao ficheiro e não é o problema.

**Testes canários existentes** (`03_infra/src/integration_tests.rs`, asserts
`contains("CrystallineFont")` como marcador do caminho CIDFont vs Type1):
actualizados para o novo nome canónico (o marcador passa a ser o prefixo
`AAAAAA+` + nome real, não o literal antigo).


## P956 — recursos do modo verbose: `/ColorSpace` + ICC sempre embutido; `PageContext` ganha `mode`

ADR-0126 (emendada P956): o modo verbose (novo padrão de produção) declara a
cor por bloco via `/c0 cs … scn` (ver `stream.md` §P956), o que exige recursos
novos na página:

- **`/ColorSpace << /c0 [/ICCBased <icc_id> 0 R] >>`** no dicionário
  `/Resources` de cada página (forma array — um colour space ICCBased é, por
  definição PDF, o array `[/ICCBased ref]`, mesma forma já usada pelos
  XObjects de imagem P777 neste ficheiro) (os três pontos de construção: Type1, CIDFont, Multifont), apenas em
  modo verbose. Modo compacto: recursos **byte-inalterados** (sem `/ColorSpace`).
- **Perfil ICC sRGB embutido sempre** em modo verbose (hoje só é embutido
  quando há imagens, P263): o objecto do perfil passa a existir em qualquer
  PDF verbose, e `/c0` aponta para ele.
- **Nuance registada (divergência de recurso, não de operador)**: o vanilla
  declara ICCBased **gray** para preenchimentos acromáticos (`/c0 cs 0 scn` —
  1 componente) e sRGB para cromáticos. A v1 cristalina usa **sRGB único com
  3 componentes sempre** — mesma cor final, número e ordem de operadores
  iguais; o refinamento gray-space fica para passo futuro se o decalque ou
  validadores o exigirem.
- **`PageContext::type1`/`cidfont`/`multifont` ganham `mode: StreamMode`**
  (construído em `builder.rs` a partir do parâmetro recebido do dispatch de
  `mod.rs`). Os três `build_*` de página propagam o mesmo modo a
  `build_page_stream`.
- Nomes de fonte `/F1..N` mantidos (nomes de recurso arbitrários; o vanilla
  usa `/f0` — diferença cosmética, fora do decalque).

## P1140.6 — árvore estrutural mínima de fórmulas

### Medição antes da decisão

O builder atual não aloca objetos de estrutura. P1140.5 já fornece Formula,
placement e alt sem inferência; o PDF continua `Tagged: no`.

### Decisão

Com `PdfTags::Enabled`, fazer uma pré-passagem estrutural por página na mesma
ordem de pintura do stream e atribuir MCIDs zero-based. Para cada envelope
Formula, alocar um `StructElem` com `/S /Formula`, `/P` para um elemento pai
`/Document`, `/Pg` para a página e `/K` igual ao MCID. Emitir `/Alt` somente
para `Some`, como string PDF Unicode UTF-16BE; `Some("")` emite string vazia
e `None` omite a chave, sem fallback visual.

Criar um `StructTreeRoot` referenciado pelo catálogo, um filho `/Document`
contendo os Formula em ordem documental, e `/ParentTree` com `/Nums`: cada
página recebe `/StructParents` único e a entrada correspondente é um array em
que o índice MCID aponta ao `StructElem`. Definir `/MarkInfo << /Marked true
>>` somente neste modo. A alocação de IDs deve ser incorporada à ordem
canônica antes da serialização, sem referências futuras ausentes.

Com `PdfTags::Disabled`, não emitir nenhum desses objetos/chaves. A presença
da estrutura não constitui declaração de PDF/UA; P1140.6 apenas mede
validadores externos e não adiciona opção pública de conformidade.

## P1286 — embedded files e exclusão de artifacts da structure tree

### Medição anterior à decisão

O vanilla coleta `AttachElem` globalmente e cria EmbeddedFile/Filespec
(`typst-pdf/src/attach.rs:13-67`). A medição cobriu nome e bytes extraídos,
`/Desc`, MIME, `/AFRelationship /Supplement` em PDF/A-3b e erro de path
duplicado. A árvore cristalina atual constrói somente StructElem de Formula.

### Decisão condicionada ao gate

O builder consome `PagedDocument.attachments` em ordem, aloca stream
`/Type /EmbeddedFile`, Filespec com nome e `EF`, e name tree
`/EmbeddedFiles` no catálogo. Bytes são preservados; description e MIME são
emitidos somente quando presentes. A pipeline rejeita nomes derivados
duplicados antes do builder. Relationship é preservada no carrier; emissão
dependente de PDF/A-3 permanece `Unknown` enquanto não existir configuração
de standard, e não pode ser alegada como fechada por PDF normal.

Se o documento tiver attachments mas nenhum frame visual, a página branca
de compatibilidade é adicionada sem reconstruir nem descartar os metadados do
`PagedDocument`; attachment-only continua exportável.

Na pré-passagem de tagging, `SemanticKind::Artifact(_)` recursa para manter a
ordem de pintura, mas não aloca MCID, `StructElem` ou ParentTree para si nem
para descendentes enquanto o envelope estiver ativo. Formula fora de artifact
conserva P1140.6.

## P1288 — structure tree acessível de tabela (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`03_infra/src/export/builder.rs:2522-2655` constrói hoje somente
Document→Formula e ParentTree por página. A fonte vanilla pinada mede summary
em `typst-pdf/src/tags/context/table.rs:129-135`, resolução de headers por
level/scope em `:197-239,374-440` e tags/spans em `:289-353`.

### Decisão proposta

Quando tags estão enabled, ampliar a pré-passagem para materializar a árvore:
Table→TR→TD para tabela simples; Table→THead/TBody→TR→TH/TD para header
automático uniforme; e TR direta para linha explicitamente mista, com:

- summary presente somente em `/Table`, distinguindo ausência de string vazia;
- TH com scope Row/Column/Both e IDs determinísticos; `level` resolve relações,
  mas não é emitido como atributo numérico;
- TD/TH com rowspan/colspan quando diferentes de um;
- referências de headers calculadas por ordem, scope e level, sem promover
  Data explícita em header;
- um único TH lógico para cópias visuais de header multipágina, sem duplicar
  THead, IDs ou relações;
- `StructParents` próprio por página, MCIDs reiniciados em zero por página e
  `ParentTree /Nums` associando cada página ao array em ordem de MCID, sem
  referências órfãs;
- `/MarkInfo << /Marked true /Suspects false >>` e uma única
  `/StructTreeRoot` no catálogo tagueado.

Com tags disabled, omitir `/StructTreeRoot`, `/MarkInfo` e toda a estrutura
Table/TR/TH/TD/ParentTree associada, preservando texto, páginas, boxes e
geometria. Não declarar PDF/UA, AT real, reflow ou acessibilidade geral; esses
resultados permanecem Unknown e não são inferidos da estrutura positiva.

## P1405 — nó `/Lbl` para prefixo de outline

Com tags habilitadas, cada `SemanticKind::Label` recebe MCID determinístico e
um nó folha `/Lbl` na ordem estrutural da página. O node não recebe alt,
summary ou atributos de tabela. Com tags desabilitadas não cria nó nem MCID;
seus items visuais continuam presentes. Esta fatia não afirma estrutura
completa de listas ou do outline, reservada a passos próprios.

## P1468 — seleção de páginas: `PageIndexConverter`, links, destinos, outline (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `0b322a926dd2f86b49aabfeb0d2f287ce06aaace`, `build_helvetica`
(`:836-965`), `build_cidfont` (`:969+`) e `build_multifont` (`:1345+`)
assumem sempre `n = doc.pages.len()`, `first_page = 3`, `/Kids`/`/Count`
cobrindo todas as páginas, e `for (i, page) in doc.pages.iter().enumerate()`
sem filtro. `emit_link_annotations` (`:2113-2183`) itera `doc.pages` inteiro
e resolve cada `LinkTarget::Destination(label)` via `/A /GoTo /D {name}`
(destino **nomeado**, não `/Dest` posicional — diferença mecânica face ao
vanilla, permitida por ADR-0107). `emit_named_destinations` (`:2190-2259`)
constrói o dicionário `/Names /Dests` a partir de
`doc.extracted_label_pages`/`extracted_label_positions`, com um fallback
"página inválida → última página do documento" já existente
(`:2219-2227`) para um caso não relacionado (label cujo número de página
nunca existiu). `emit_outlines` (`:2348-2452`) constrói uma árvore
pai/filho/prev/next a partir de `doc.extracted_headings: Vec<(Label,
Option<String>, Content, usize)>`, com o mesmo fallback de página nas
linhas `:2380-2384`.

No vanilla ratificado (medido contra `/usr/local/bin/typst`,
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, em 2026-09-20T21:1x-21:2x, e
confirmado por leitura de `typst-pdf/src/convert.rs:877-908`,
`typst-pdf/src/link.rs`, `typst-pdf/src/outline.rs`):

1. **Page tree**: `PageIndexConverter` mapeia índice físico antigo → novo
   índice sequencial (só para páginas incluídas); páginas excluídas não
   entram no mapa. `/Count`/`/Kids` refletem só as páginas mantidas.
2. **Links** (`typst-pdf/src/link.rs:191-209`, `pos_to_xyz`): se a página de
   destino de um link foi excluída, a *annotation inteira é omitida*
   (`return Ok(())`) — não há erro, não há redirecionamento para outra
   página. Confirmado experimentalmente: um link cujo alvo (elemento comum,
   não heading) cai numa página excluída produz zero `/Annots` na página de
   origem.
3. **Named destinations** (`typst-pdf/src/convert.rs:858-872`): só existe
   destino nomeado para um heading com label se a página desse heading
   **está incluída**; caso contrário a entrada não é criada e qualquer link
   que dependa dela cai no comportamento 2 (omitido).
4. **Bookmark tree nativa (`/Outlines`) — bug identificado, não reproduzido.**
   No vanilla medido, a árvore fica **sempre vazia** quando `pdf.pages` está
   definido, **mesmo quando a seleção cobre todas as páginas**
   (`--pages 1-4` num documento de 4 páginas → `/Outlines` ausente do
   catálogo). A causa, lida em `typst-pdf/src/outline.rs:25-31`, é uma
   inversão de sinal: `visible = pages.v.is_none_or(|ranges|
   !ranges.includes_page(...))` faz headings em páginas **incluídas**
   contarem como `visible=false` e headings em páginas **excluídas**
   contarem como `visible=true` — o oposto do que o nome da variável e o
   uso em `include = bookmarked && visible` evidentemente pretendem. Não há
   nenhuma razão de produto para "selecionar páginas apaga todos os
   bookmarks mesmo quando nada foi removido"; isto não é uma escolha de
   design (como o warning de tagging, que tem uma justificação de
   acessibilidade), é um erro claro de sinal. **Decisão do dono
   (2026-09-20): não copiar este erro.** O cristalino diverge
   deliberadamente do observável medido e implementa o comportamento que a
   estrutura do vanilla evidentemente pretendia: um heading cuja página está
   **incluída** aparece na árvore; um heading cuja página foi **excluída** é
   omitido, e os seus descendentes (se houver, pela hierarquia de `level`)
   são reparentados ao ancestral incluído mais próximo — mesmo princípio de
   poda de `OutlineNode::build_tree`
   (`typst-library/src/model/outline.rs:338-395`), sem a inversão de sinal.
   Isto é registado como divergência intencional, não como lacuna de
   paridade.
5. **PageLabel**: o vanilla preserva o número físico original via
   `/PageLabel` quando páginas são omitidas
   (`typst-pdf/src/convert.rs:130-141`). **Scope-out explícito**: o
   cristalino hoje não emite `/PageLabel` para nenhum caso (confirmado:
   ausente do builder atual); adicionar essa estrutura é trabalho novo e
   ortogonal à seleção de páginas em si. P1468 não implementa `/PageLabel`;
   a numeração visível dentro do conteúdo da página (produzida pelo layout)
   já reflete o número físico original independentemente da seleção, porque
   a seleção só afeta o exportador, nunca o layout.

### Contrato

Introduzir, neste ficheiro ou num submódulo próprio do exportador
(`03_infra/src/export/page_selection.rs`, decisão de organização livre,
mecânica), o helper puro:

```rust
pub(crate) struct PageIndexConverter {
    // índice físico antigo (0-indexed) -> novo índice (0-indexed), só para
    // páginas incluídas.
}

impl PageIndexConverter {
    pub fn new(total_pages: usize, ranges: Option<&PageRanges>) -> Self;
    pub fn exported_count(&self) -> usize;
    /// `None` se a página (0-indexed) foi excluída da seleção.
    pub fn new_index(&self, old_index: usize) -> Option<usize>;
}
```

Sem `ranges` (`None`), `new_index` é a identidade e `exported_count ==
total_pages` (comportamento atual, byte-idêntico). Com `ranges` presente,
usar `PageRanges::includes_page` (1-indexed, `old_index + 1`) para decidir
inclusão, na mesma ordem física 1..=N — a "deduplicação"/"ordenação" da
seleção emerge exclusivamente desta iteração, nunca de normalizar a lista de
`PageRange` guardada (ver `entities/document_format_options.md` P1468).

Nas três variantes `build_*`: construir um `PageIndexConverter` a partir de
`doc.pages.len()` e do `pages: Option<PageRanges>` recebido (novo parâmetro,
ver `infra/export/mod.md` P1468); usar `pic.exported_count()` no lugar de
`doc.pages.len()` para `n`/`/Count`; iterar só os índices com `new_index`
`Some`, usando o **novo** índice para `page_id = first_page + new_i`
(mantendo a ordem física relativa). Todos os cálculos de offset de objeto
que hoje dependem de `n` (streams, fontes, imagens, gradients) usam a nova
contagem filtrada.

Em `emit_link_annotations`: iterar só páginas incluídas (via `pic`); para
cada `LinkTarget::Destination(label)`, resolver `doc.extracted_label_pages
.get(label)` (número físico) através de `pic.new_index(page - 1)` — se
`None` (página excluída), **omitir a annotation** (não emitir o objeto nem
referenciá-la em `/Annots`), replicando o comportamento 2 medido acima. O
mecanismo de destino nomeado (`/A /GoTo /D {name}`) não muda; só a decisão
de "emitir ou não a annotation" muda.

Em `emit_named_destinations`: para cada label, resolver a página via
`pic.new_index(page - 1)`; se `None`, **não** adicionar a entrada ao
dicionário `/Dests` (comportamento 3). Substituir `FIRST_PAGE_ID + page_idx`
por `FIRST_PAGE_ID + new_index` no `page_ref` emitido. O fallback existente
"página inválida → última página" (`:2219-2227`, para número de página que
nunca existiu no documento) continua a existir **apenas** para esse caso
distinto — não se aplica a página validamente excluída pela seleção.

Em `emit_outlines`: para cada `(label, _number, body, level)` de
`doc.extracted_headings`, resolver a página física via
`extracted_label_pages` e checar `pic.new_index(page - 1)`. Se `Some`
(página incluída), processar exatamente como hoje — empilhar/desempilhar por
`level`, criar o nó, encadear `/Parent`/`/Prev`/`/Next`/`/First`/`/Last` — só
que o `page_ref` usa o **novo** índice (`FIRST_PAGE_ID + new_index`), não o
físico. Se `None` (página excluída), **não criar nó para este heading e não
o empilhar** — mas continuar a percorrer a lista normalmente. Como o topo da
pilha (`stack.last()`) só é atualizado por headings efetivamente empilhados,
um heading de nível mais profundo que segue um heading excluído é
automaticamente reparentado ao último ancestral **incluído** ainda na pilha
(ou promovido a top-level, se nenhum ancestral incluído sobrar) — sem
necessidade de um "nível mais alto pulado" explícito como no vanilla, porque
a estrutura cristalina já é uma pilha por nível, não uma árvore genérica.
Sem `pages`, comportamento idêntico ao atual (nenhum heading é filtrado).

### Fronteiras

Este owner não decide a interação `pages`+`tagged` (já resolvida em
`infra/pipeline.md` antes de o `PdfTags` chegar aqui); não implementa
`/PageLabel` (scope-out explícito); não normaliza a lista de `PageRange`
recebida.

### Aceitação e gate

Testes cobrem: `pages: None` (regressão byte-idêntica nas três variantes de
build e nos três `emit_*`); seleção de página única, range fechado, limites
abertos; página física removida cujo destino é referenciado por um link
comum (annotation omitida) e por um heading com label (named destination
ausente); outline nativa com seleção cobrindo todas as páginas (todos os
headings sobrevivem, divergência deliberada do vanilla — ver medição 4);
outline com página intermediária removida cujo heading tem filhos de nível
mais profundo (filhos reparentados ao ancestral incluído, heading excluído
ausente da árvore); `/Count`/`/Kids` refletindo só as páginas exportadas;
neutralidade de PNG/SVG/HTML. Inspecionar estrutura parseada (contagem de
`/Type /Page`, presença/ausência/parentesco de `/Annots`/`/Outlines`/entradas
de `/Dests`), nunca igualdade total de bytes. Este é um dos L0s do gate
ADR-0127 de P1468 (mudança de comportamento por defeito do exportador).

A confirmação humana explícita posterior aos commits causais somente-L0
`9bb20b8e8`/`70d09eae2` foi recebida em 2026-09-20/21. Agente A implementou
o `PageIndexConverter` **dentro de `builder.rs`** (não num submódulo
`page_selection.rs` separado como o contrato admitia como opção mecânica
livre) — criar um segundo ficheiro exigiria um segundo Prompt L0 e
violaria a cardinalidade 1:1 do ADR-0129 (V15), fora do mandato de
implementação pura do Agente A; a decisão de organização "neste ficheiro"
já estava prevista no contrato. Reindexou as três variantes `build_*` e os
três `emit_*` (incluindo `emit_xmp_metadata`'s `NPages`, não citado
explicitamente no contrato original mas exigido pela mesma reindexação) sem
ver os testes. Agente B escreveu 12 testes de integração de binário
(`04_wiring/tests/p1468_format_pdf_pages.rs`) a partir do mesmo baseline sem
ver a implementação, cobrindo explicitamente o comportamento retificado do
outline (headings de páginas incluídas sobrevivem; reparenting quando o pai
é excluído) — não o bug do vanilla. O teste independente passou de RED a
GREEN (12/12) após a integração, sem regressão na suíte de PDF (32/32 nos
irmãos p1462/p1463/p1464/p1465/p1467).

## P1469 — header `%PDF-{version}` dinâmico (ESCOPO REDUZIDO — contrato de linguagem) (MATERIALIZADO; gate ADR-0127 satisfeito)

Ver `entities/document_format_options.md` P1469 para a nota de escopo
reduzido (decisão do dono, 2026-09-21) — **este owner só ajusta o header
declarado, nunca a estrutura interna do PDF.**

### Medição anterior à decisão

No baseline `85b7fc5f1eda67e89494a478c2458de36d4ee20d`, `serialize`
(`:3300-3302`) escreve `b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n"` — literal fixo,
único ponto de escrita do header em todo o exportador (`grep` confirma
zero outras ocorrências de `%PDF-` fora de testes/comentários). Vários
testes existentes (`export/tests.rs`, ~8 ocorrências) fixam
`pdf.starts_with(b"%PDF-1.7")` como asserção — o comportamento *default*
(sem `standard` explícito, de nenhuma fonte) tem de continuar a produzir
exatamente `%PDF-1.7`.

### Contrato

`PdfBuilder` (ou o tipo que hoje carrega `stream_mode`/`pdf_tags`/
`pdf_serialization`, mesmo padrão) ganha um campo `standard: PdfStandards`,
propagado desde a nova entry point de `infra/export/mod.md` P1469. Em
`serialize`, substituir o literal `b"%PDF-1.7"` por
`format!("%PDF-{}", self.standard.version.as_str()).as_bytes()` (ou
equivalente), onde `PdfVersion::as_str()` produz exatamente `"1.4"`, `"1.5"`,
`"1.6"`, `"1.7"`, `"2.0"` (strings já fechadas em
`entities/document_format_options.md` P1469 — mesmos nomes usados no cast
documental/CLI). O comentário binário (`%\xe2\xe3\xcf\xd3\n`) e o resto da
estrutura do PDF (xref, tipos de objeto, `/Catalog`, tudo o resto que
`build_helvetica`/`build_cidfont`/`build_multifont`/`emit_*` já produzem)
**não mudam com a versão declarada** — isto é uma declaração de versão no
header, não uma adaptação estrutural real a cada versão.

### Scope-out explícito (repete a nota de `entities/document_format_options.md` P1469)

Este owner não implementa: estruturas específicas de PDF/A (identificação
XMP com nível de conformidade, restrições de cor/fontes/transparência),
estruturas específicas de PDF/UA além do que P1140.5-A/P1405 já cobrem
(tagging básico, não os requisitos completos de acessibilidade do perfil),
nem qualquer recurso específico de PDF 2.0 (ex.: xref stream obrigatório —
o cristalino continua a emitir a mesma estrutura de xref independentemente
da versão declarada). Declarar `%PDF-2.0` sem adotar exclusivamente
recursos de 2.0 é aceitável porque PDF 2.0 é retrocompatível com estruturas
válidas em versões anteriores (xref clássica continua válida); não é
alegação de conformidade completa com o perfil.

### Aceitação e gate

Testes cobrem: default sem `standard` nenhum continua `%PDF-1.7` (regressão
zero nos ~8 testes existentes que fixam isso); `standard` documental/CLI
para cada uma das cinco versões (`1.4`/`1.5`/`1.6`/`1.7`/`2.0`) produz o
header correspondente; um perfil PDF/A ou PDF/UA sem versão explícita
produz o header do **máximo** do seu range (`a-2b` sozinho → `%PDF-1.7`;
`a-1b` sozinho → `%PDF-1.4`; `a-4` sozinho → `%PDF-2.0`); resto da estrutura
do PDF (contagem de objetos, xref, tipos) idêntico entre versões, provando
que só o header muda. Este é um dos L0s do gate ADR-0127 de P1469.

A confirmação humana explícita posterior aos commits causais somente-L0
`65e68ad45`/`e14002d55` foi recebida em 2026-09-21. Agente A acrescentou
`PdfBuilder.standard: PdfStandards` e o header dinâmico em `serialize()` sem
ver os testes. Agente B escreveu 10 testes de integração de binário
(`04_wiring/tests/p1469_format_pdf_standard.rs`) a partir do mesmo baseline
sem ver a implementação. O teste independente passou de RED a GREEN (10/10)
após a integração, sem regressão na suíte de PDF (44/44 nos irmãos
p1462/p1463/p1464/p1465/p1467/p1468).

## P1475 — Núcleo PDF/A-2/3/4: trailer `/ID`, XMP `pdfaid`, `/OutputIntent`, supressão de `/Info` em A4

Ver `typst-passo-1475.md` para a medição completa (diagnóstico prévio
`typst-p1469-revisita-verapdf-2026-09-21.md` Achado 4, mais medições
confirmatórias próprias desta sessão via veraPDF 1.30.2 e inspeção `qpdf
--qdf` de PDFs vanilla reais). Único owner dos quatro itens; nenhuma
mudança em `entities/document_format_options.rs` nem em
`infra/export/images.rs` — só consumo do que já existe.

### Medição anterior à decisão

`emit_xmp_metadata` (`builder.rs:2703-2810`) já calcula
`(instance_id, document_id)` via `xmp_instance_and_document_id` (`:409-428`,
base64 de 16 bytes) mas não persiste esses valores; `serialize` (`:3316-
3358`) constrói o trailer só com `/Size`/`/Root`/`/Info` opcional, sem
`/ID`. Medido no vanilla (documento sem `standard` nenhum): o trailer tem
sempre `/ID [ (X) (Y) ]` com `X == Y == xmpMM:InstanceID/DocumentID` desse
mesmo PDF — comportamento universal, não gated a PDF/A. `emit_info`
(`:2664-2698`) sempre aloca `/Info`; medido no vanilla `a-4`: trailer sem
`/Info` (ISO 19005-4 6.1.3). `emit_xmp_metadata` nunca emite o namespace/
elementos `pdfaid`; medido: A2/A3 usam `pdfaid:part`+`pdfaid:conformance`
(letra do sufixo do nome), A4 usa `pdfaid:part`+`pdfaid:rev` (`"2020"`,
**sem** `conformance` — achado novo desta sessão, não estava no Achado 4.2
original). Nenhum `/OutputIntent`/`/OutputIntents` é emitido hoje; medido
em `a2b`/`a4` vanilla: objeto idêntico em forma
(`/Type /OutputIntent /S /GTS_PDFA1 /OutputConditionIdentifier (Custom)
/OutputCondition (sRGB) /RegistryName () /Info (sRGB v4.2) /DestOutputProfile
{ref}`) — `/S` fixo em `GTS_PDFA1` independentemente do nível A1-A4.
`build_icc_profile_stream(srgb_icc_profile_bytes())` (já importados,
`builder.rs:130-141`, uso existente em `:1043/1423/1924` para P777) produz
exatamente a forma do objeto `/DestOutputProfile` medido
(`<< /N 3 /Range [0 1 0 1 0 1] /Filter /FlateDecode >>`).

### Contrato

1. **Trailer `/ID` (universal)**: `PdfBuilder` ganha campo
   `xmp_ids: Option<(String, String)>`; `emit_xmp_metadata` grava
   `self.xmp_ids = Some((instance_id.clone(), document_id.clone()))` ao
   computar os dois valores. `serialize()` insere
   `format!(" /ID [ ({inst}) ({doc}) ]")` no trailer quando `Some`, nada
   quando `None` (defensivo — não acontece nos três `build_*` atuais, que
   sempre chamam `emit_xmp_metadata` antes de `serialize`). Aplica-se a
   **todo** export, com ou sem `standard`.
2. **`pdf:PDFVersion` dinâmico (fluxo contínuo)**: literal `"1.7"`
   (`:2779`) substituído por `self.standard.version.as_str()`.
3. **XMP `pdfaid` (gated)**: quando `self.standard.archival` ∈
   `{A2a,A2b,A2u,A3a,A3b,A3u}`, emitir `<pdfaid:part>{2|3}</pdfaid:part>
   <pdfaid:conformance>{A|B|U}</pdfaid:conformance>` (letra via helper
   `pdfaid_letter(PdfArchival) -> &'static str`); quando `A4`, emitir
   `<pdfaid:part>4</pdfaid:part><pdfaid:rev>2020</pdfaid:rev>` (sem
   `conformance`); caso contrário, nada. O namespace `xmlns:pdfaid` só é
   declarado quando o elemento não é vazio (único namespace condicional do
   bloco XMP — os demais são incondicionais).
4. **`/OutputIntent` (gated ao mesmo conjunto + A4)**: novo método privado
   `emit_pdfa_output_intent(&mut self)`, chamado nas três variantes
   `build_*` logo após `emit_info(doc)` e antes de `emit_xmp_metadata(doc)`
   (mesma posição relativa nas três, preserva alocação de IDs idêntica
   entre caminhos). Aloca objeto ICC próprio (não reusa o de P777, mesmo
   quando ambos existem no documento — decisão mecânica, ADR-0107, sem
   efeito observável para veraPDF) via
   `build_icc_profile_stream(srgb_icc_profile_bytes())`, e o objeto
   `/OutputIntent` referenciando-o; edita o catálogo (objeto 1) com
   `/OutputIntents [{id} 0 R]` via `insert_pdf_dict_entry` (helper já
   existente, `:3464`, usado por `emit_tag_structure`). `/S` é sempre
   `/GTS_PDFA1`, nunca varia por nível.
5. **Supressão de `/Info` (só `A4`)**: `emit_info` retorna cedo (sem
   alocar nada, `self.info_id` continua `None`) quando
   `self.standard.archival == Some(PdfArchival::A4)`. Não se aplica a
   `A4f`/`A4e` (fora do `match`). `serialize()` já omite `/Info` do
   trailer quando `info_id` é `None` — nenhuma mudança adicional
   necessária aí.

### Scope-out explícito

`a-1a`/`a-1b`: sempre PDF 1.4 (`min_version == max_version == V1_4`,
`document_format_options.rs:174-182`); o perfil ICC v4.2 reusado por este
passo falha a regra 6.2.3.2 em PDF 1.4 (medido, Achado 4.4) — A1 exige um
segundo perfil ICC v2 (736 bytes) mais `/CIDSet` por fonte subsetada
(Achado 4.5, toca `fonts.rs`) — passo próprio. `a-4f`/`a-4e`: não medidos
(exigem ficheiro anexado). `ua-1`: eixo estrutural à parte (`/Lang`,
tagging, `/MarkInfo /DisplayDocTitle`), maior, já classificado em P1469
como "candidato a passo(s) separado(s)". Combinação `a-2*`/`a-3*` +
versão explícita `"1.4"` (fora do default `"1.7"`) pode reproduzir a
mesma falha de A1 — não medida, registada como limitação aceite, não
bloqueia este passo.

### Aceitação e gate

Testes cobrem: `/ID` presente e determinístico em qualquer export;
regressão zero de `pdf:PDFVersion` sem `standard`; `pdfaid` correto para
os sete archivals do escopo e ausente para os demais (incluindo os
scope-out, negativamente); `/OutputIntent`/`/OutputIntents` presente só
nesse mesmo conjunto, com `/S /GTS_PDFA1` fixo; `/Info` ausente só em
`A4`. Medição final com veraPDF (binário já instalado) nos seis perfis
A2/A3: zero falhas nas 3 regras nucleares; A4: zero falhas nas 5 regras do
Achado 1. Este é um dos L0s do gate ADR-0127 de P1475 — a mudança do
trailer `/ID` universal (item 1) altera o comportamento por defeito de
**todo** export PDF, não só do caminho opt-in `standard:`; a confirmação
de P1467-P1469 não cobre esta mudança.

### Execução segregada e integração (2026-09-22)

Confirmação humana explícita recebida em 2026-09-22, na mesma sessão,
posterior ao commit causal somente-L0 `7a20ec72d`. Agente A
(implementação, commit `eeeee4538`) e Agente B (testes RED, commit
`e090fc29f`) partiram do mesmo baseline em worktrees isoladas, ambos
reproduzindo e corrigindo por conta própria o bug de infra já conhecido
(worktree a arrancar no baseline vanilla em vez de `Tekt`). Nenhum dos
dois viu o trabalho do outro. Os dois patches aplicaram-se sem conflito
de merge (papel C, mesma sessão) mesmo tocando o mesmo ficheiro
`builder.rs` extensivamente.

**Achado de integração 1 (correção de RED, não de implementação):** o
teste `p1475_trailer_id_is_present_universal_and_matches_xmp_ids`
(Agente B) assumia `X == Y` no `/ID` do trailer "no caso comum", citando
literalmente o texto do passo (§5). Agente A, de forma independente,
já tinha medido e reportado que isso é falso para o cristalino: `xmp_
instance_and_document_id` (pré-existente, P615/P617, fora do escopo
deste passo) gera `InstanceID`/`DocumentID` **sempre distintos** — duas
constantes fixas diferentes em modo de teste, duas chamadas
independentes a `random_xmp_id_bytes()` em produção. O vanilla mede
X==Y (Achado 4.1) porque o krilla usa o mesmo valor para as duas
posições na primeira gravação — o cristalino nunca replicou esse detalhe
mecânico, e P1475 não pediu para o fazer. A asserção `X==Y` foi corrigida
(removida, substituída por comentário explicando o achado); as duas
asserções que realmente provam o contrato do item 1 (`/ID[0] ==
xmpMM:InstanceID`, `/ID[1] == xmpMM:DocumentID`, mesmo PDF) continuam e
passam. O texto do §5 acima **não foi silenciosamente apagado** — esta
nota é o registo da correção, ADR-0108.

**Achado de integração 2 (V16, qualidade, não contrato):** o `match`
original do Agente A para `pdfaid_elem` (§3.3) usava `_ => String::new()`
para o caso "fora do conjunto A2/A3/A4". `crystalline-lint` sinalizou V16
("wildcard descarta informação com default neutro em enum de domínio").
Corrigido para `match` exaustivo sobre `Option<PdfArchival>`, listando
`None` e os quatro scope-outs (`A1a`/`A1b`/`A4f`/`A4e`) explicitamente —
mesmo padrão já usado por `pdfaid_letter` (Agente A, `debug_assert!` nos
braços que nunca deveriam disparar). Sem mudança de comportamento, só de
forma do código.

**Regressão de fixtures binários (esperada, §2.1 já sinalizava):** os 9
snapshots dourados de `p307b_snapshot_tests` (`03_infra/fixtures/p307b/
reference/*.pdf`) regrediram exatamente +62 bytes cada — o tamanho exato
do novo `/ID [ (X) (Y) ]` universal. Confirmado byte-a-byte (`cmp -l`,
inspeção do trailer) que **nada mais** mudou em nenhum dos 9 ficheiros.
Regenerados via `UPDATE_P307B_SNAPSHOTS=1` (mecanismo já existente desde
P307b, "permitir regenerar referências quando o output PDF muda
intencionalmente").

**Achado não corrigido, fora de escopo, registado:** `cargo test -p
typst-wiring` tem uma falha pré-existente e não relacionada
(`p1464_svg_png_global_pdf_html_and_homonymous_closure_controls`, oracle
desatualizado do P1471 — espera `pdf` como `"module"`, já é `"function"`
desde a migração `Value::Module -> Value::Func` materializada em
`52d3d337c`, muito antes do baseline deste passo). Confirmado
independentemente por Agente A (worktree isolada em `7a20ec72d`, sem
qualquer alteração de P1475) e por papel C (leitura direta do teste). Não
tocado — fora do owner deste L0.

### Gates finais medidos (2026-09-22)

```text
cargo build --workspace                                     -> sucesso
cargo test -p typst-core --lib                                -> 6129/6129
cargo test -p typst-shell --lib                                -> 83/83
cargo test -p typst-infra --lib                                -> 1006/1006 (RED->GREEN
                                                                   dos 11 novos + 9 fixtures
                                                                   regenerados, duas rondas
                                                                   completas)
cargo test -p typst-wiring                                     -> 1 falha pré-existente
                                                                   não relacionada (acima),
                                                                   confirmada no baseline
cargo fmt (builder.rs/tests.rs, únicos ficheiros tocados)      -> limpo (drift pré-existente
                                                                   em figure_caption.rs/
                                                                   footnote_entry.rs, não
                                                                   deste passo, não tocado)
cargo clippy -p typst-infra --all-targets --message-format=json
    comparado por contagem (ficheiro, mensagem) contra o
    baseline `7a20ec72d` em worktree isolada                  -> zero avisos novos
crystalline-lint .                                              -> 0 erros/fatais (V16 real
                                                                   corrigido; um V16
                                                                   pré-existente e não
                                                                   relacionado permanece,
                                                                   fora do diff)
crystalline-lint --checks v15,v26 .                             -> 0 violações
git diff --check                                                -> sucesso
```

Medição de conformidade veraPDF 1.30.2 (binário cristalino recompilado
neste commit, documento de teste com heading/negrito/itálico/figura com
caption): **os sete archivals do escopo (A2a/A2b/A2u/A3a/A3b/A3u/A4) são
100% conformes** — `isCompliant="true"`, `failedRules="0"` em todos,
excedendo a meta original do passo (que só exigia zero falhas nas regras
nucleares medidas em `typst-p1469-revisita-verapdf-2026-09-21.md` Achado
1, não conformidade total).

Veredito final: os cinco itens do contrato (§3) fecham integralmente,
sem escopo reduzido adicional além do já declarado em §2.5. Dois achados
de integração corrigidos (asserção de teste incorreta sobre X==Y; V16 de
qualidade) confirmam outra vez o valor da execução segregada A/B/C —
nenhum dos dois exigiu reabrir o L0 nem o gate ADR-0127, ambos são
correções de fluxo contínuo dentro do que o contrato já previa.

## P1476 — Anexos PDF/A: `/AFRelationship`, `/AF`, extensão de `/Info`/`/OutputIntent`/`pdfaid` a A4f/A4e

Ver `typst-passo-1476.md` para a medição completa. Continuação direta de
dois scope-outs já registados: `typst-passo-1475.md` §2.5 (A4f/A4e não
medidos) e o docstring de `emit_attachments` abaixo (P1286: "a relação
associada ao ficheiro permanece no domínio para futuros perfis PDF/A...
não escreve `/AFRelationship` nem `/AF`"). Owner único; nenhuma mudança
em `01_core/src/entities/elements/pdf_attach.rs` (o campo
`relationship: Option<AttachedFileRelationship>` já existe, já chega a
`PagedDocument.attachments` — só não era consumido na emissão).

### Medição anterior à decisão

Vanilla A4f/A4e + anexo: 100% conforme veraPDF (`isCompliant=true`,
`failedRules=0`). Único byte observável entre A4f e A4e: XMP
`<pdfaid:conformance>F</pdfaid:conformance>` vs `...E...` — achado que
corrige P1475 §2.2 (A4 "puro" não tem `conformance`, mas A4f/A4e têm as
**três** chaves `part`+`rev`+`conformance`, não só `part`+`rev`). Filespec
vanilla ganha `/AFRelationship {nome}` (mapeamento 1:1 confirmado com o
enum já existente `AttachedFileRelationship::{Source,Data,Alternative,
Supplement}`; `None` do domínio → `/Unspecified`); catálogo ganha
`/AF [{filespec_id} 0 R, ...]`. Confirmado que `A3a/A3b/A3u` + anexo têm
exatamente o mesmo padrão (`/AFRelationship`+`/AF`, `/OutputIntent` já
coberto desde P1475, `/Info` nunca suprimido — só a família A4 suprime).
Sem `standard` nenhum: nem `/AFRelationship` nem `/AF` — gated, não
universal. Medição negativa com o build cristalino atual (pré-P1476)
sob `a-4f` + anexo + figura RGB confirma exatamente três causas de
falha: `/Info` presente (6.1.3), `/AFRelationship` ausente (6.9),
`/OutputIntent` ausente (6.2.4.3), `pdfaid` ausente (6.7.3) — nenhuma
falha relativa a `/AF` (replicado por completude/paridade, custo
marginal zero, não comprovadamente exigido por esta suíte de regras).

### Contrato

1. **`/AFRelationship` na Filespec + `/AF` no catálogo** — gated a
   `archival_supports_embedded_files` (helper novo:
   `A3a|A3b|A3u|A4f|A4e`, únicas famílias que suportam anexos conformes
   nos respetivos ISO 19005). `emit_attachments` (`builder.rs:2420`)
   mapeia `attachment.relationship` via `af_relationship_name` (helper
   puro, `None -> "Unspecified"`, variantes 1:1) e inclui o campo
   condicionalmente na Filespec; acumula os `filespec_id` processados e,
   se o gate estiver ativo e houver pelo menos um, edita o catálogo com
   `/AF [...]` via `insert_pdf_dict_entry`.
2. **`/Info` suprimido** — `emit_info` (`builder.rs:2680`) estende a
   comparação de P1475 (`Some(PdfArchival::A4)`) para
   `matches!(_, Some(PdfArchival::A4 | PdfArchival::A4f | PdfArchival::A4e))`.
3. **`/OutputIntent`** — `emit_pdfa_output_intent` (`builder.rs:2728`)
   acrescenta `A4f | A4e` aos braços já existentes do `matches!`.
4. **XMP `pdfaid`** — `pdfaid_elem` (`builder.rs:2830`) ganha dois
   braços novos: A4f → `part=4 rev=2020 conformance=F`; A4e →
   `part=4 rev=2020 conformance=E`; ambos removidos do braço de
   scope-out (que passa a ser só `None | Some(A1a | A1b)`). `match`
   continua exaustivo.

### Scope-out explícito

`/Params` (`/Size`/`/ModDate`) no `EmbeddedFile` — vanilla emite,
veraPDF não exigiu na medição (§ acima); formato de data do vanilla
(`D:...±HH'mm'`) diverge do já usado por `/Info` (`D:...` sem offset) —
precisaria de medição própria, não implementado. `/UF` sempre UTF-16BE
hex (mesmo para nomes ASCII, onde o vanilla usa string literal) —
divergência de forma (ADR-0107), não de conformidade, não tocada. A1a/
A1b continuam scope-out de P1475 (ICC v2 + `/CIDSet`). PDF/UA-1 não
tocado.

### Aceitação e gate

Testes cobrem os quatro itens, positivo e negativo (ausência fora do
gate, incluindo A2\*/A4-sem-f/e onde aplicável), mais regressão dos 11
testes de P1475. Medição final veraPDF em A3b/A3u/A3a/A4f/A4e + anexo
(com conteúdo RGB para exercitar OutputIntent): zero falhas esperado.
Nenhum item deste contrato é universal (todos gated a `standard:`
explícito) — mais próximo de fluxo contínuo que o item 1 de P1475, mas
segue o mesmo protocolo de parar para confirmação humana explícita
(P1286 já tinha declarado que reabrir isto exigia decisão própria).
