# Prompt L0 — `infra/export/bitmap_glyphs` — coleta de glifos bitmap

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def

Hash do Código: 4665b640

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/bitmap_glyphs.rs`
**Criado em**: 2026-08-26 (P1196; individualização do §P941 de `builder.md`)
**ADRs**: ADR-0107, ADR-0108, ADR-0129

---

## Medição antes da decisão

Em `bitmap_glyphs.rs`, `collect_bitmap_glyphs_for_ids` recebe uma face e um
conjunto de glyph ids, consulta o maior strike raster e normaliza somente PNG
por `process_png_for_pdf`. `used_glyph_ids_for_face` constrói o conjunto usado
pela união dos glifos shaped e dos codepoints mapeáveis pela mesma face.

O módulo não aloca object IDs, não escreve dicionários PDF e não decide se uma
fonte inteira será embutida. Essas responsabilidades pertencem ao builder e ao
stream. A antiga especificação conjunta em `builder.md` criava dois consumers
produtivos para um Prompt L0; P1196 individualiza o owner sem mudar código.

## Responsabilidade

Este módulo é o adaptador puro de coleta raster do export PDF:

1. calcula os glyph ids usados por uma face;
2. extrai o maior strike raster disponível para cada id;
3. aceita somente `RasterImageFormat::PNG`;
4. converte o PNG em `PdfImagePayload` pelo pipeline de imagens existente;
5. devolve um mapa por glyph id com payload e métricas de posicionamento.

## Estruturas

`BitmapGlyph` contém o payload PDF processado, `bearing_x`, `bearing_y` e
`pixels_per_em`. Largura e altura permanecem no próprio payload.

`BitmapGlyphRef` representa a referência já emitida que o stream consome:
nome do XObject, dimensões, bearings e pixels-per-em. A alocação do nome e do
object ID não ocorre neste módulo.

## Coleta raster

`collect_bitmap_glyphs_for_ids(glyph_ids, face)`:

- consulta `face.glyph_raster_image(GlyphId(gid), u16::MAX)`;
- ignora ids sem imagem e formatos diferentes de PNG;
- ignora PNG que `process_png_for_pdf` não consiga processar;
- insere no máximo uma entrada por glyph id no mapa da face;
- preserva `img.x`, `img.y` e `img.pixels_per_em` sem reinterpretar sinais.

Ausência no mapa é fallback explícito para o caminho normal de fonte; não é
erro nem autorização para sintetizar raster.

## Descoberta dos ids usados

`used_glyph_ids_for_face(doc, face)` devolve `BTreeSet<u16>` determinístico:

- começa pelos ids de `collect_glyph_ids(doc)` menores que
  `face.number_of_glyphs()`;
- une os codepoints de `collect_text_codepoints(doc)` e
  `collect_codepoints(doc)` que `face.glyph_index` resolve;
- rejeita qualquer id fora do range declarado pela face.

O conjunto ordenado evita duplicação antes da extração e não confunde glyph
ids iguais entre faces diferentes: a chamada e o mapa pertencem a uma face.

## Limites de ownership

- O builder possui deduplicação e alocação dos XObjects por fonte e glyph id.
- O stream possui a fórmula de posicionamento e a emissão de `/ImN Do`.
- O pipeline de imagens possui a decodificação e compressão do PNG.
- Este módulo não decide seleção/fallback de família tipográfica.

## Critérios de verificação

- Glifo PNG suportado gera uma entrada com payload e métricas preservadas.
- Glifo repetido no conjunto continua representado por uma única chave.
- Raster não-PNG, raster ausente ou PNG inválido não entra no mapa.
- Id fora do range da face não entra em `used_glyph_ids_for_face`.
- Codepoints de texto e glifos shaped contribuem para o mesmo conjunto usado.
- A alteração de linhagem do P1196 não modifica corpos Rust.

## Histórico de revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-08-26 | P1196 — owner 1:1 extraído de `builder.md`; invariantes compartilhadas movidas para Núcleo Tekt | `bitmap_glyphs.md`, `bitmap_glyphs.rs` |
