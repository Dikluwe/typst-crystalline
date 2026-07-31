# Prompt L0 — `infra/export/font_subset` — Subsetting de fontes no PDF
Hash do Código: ebe34cd0

**Camada**: L3  
**Criado em**: 2026-06-30  
**Atualizado em**: 2026-07-23 (P874)  
**Arquivos gerados**: `03_infra/src/export/subset.rs` (novo), alterações em `03_infra/src/export/builder.rs`, `03_infra/src/export/fonts.rs`, `03_infra/src/export/stream.rs`  
**ADR referência**: ADR-0027 (revisão de Opção A para Opção B), ADR-0055, ADR-0120  

---

## Contexto

A ADR-0027 escolheu embeber a fonte TrueType completa no PDF (Opção A) para evitar a complexidade do remapeamento de glyph IDs. O Passo 515 (Trilha 5) activa o subsetting para reduzir o tamanho dos PDFs e aproximar o cristalino da paridade de produção com Typst 0.15.0.

O exportador actual já emite CIDFont + Identity-H e usa `FrameItem::TextShaped` (ADR-0120). O subsetting deve operar sobre os glyph IDs reais usados no documento e remapeá-los para um subconjunto compacto antes de gerar o PDF.

**P874 (2026-07-23):** A biblioteca `oxifont-subset` foi substituída por `subsetter`, a mesma usada pelo Typst 0.15.0. O `oxifont-subset` gerava programas CFF Name-keyed inválidos para `/CIDFontType0` + `Identity-H`, o que obrigou P797 a desactivar o subsetting para CFF1. O `subsetter` converte SID-keyed fonts para CID-keyed e garante um mapeamento identidade GID→CID, produzindo subsets válidos tanto para TrueType (`glyf`) como para OpenType/CFF (`CFF `). A `cmap` table é removida pelo subsetter; o mapeamento old_gid → new_gid é obtido directamente do `GlyphRemapper`.

---

## Restrições Estruturais

- Toda a lógica de subsetting fica em L3 (`03_infra/src/export/subset.rs`).
- L1 não conhece subsetting — continua a usar `ShapedGlyph.glyph_id` como índice real na fonte.
- O subset deve preservar o glyph ID 0 como `.notdef`.
- O ToUnicode CMap e o array `/W` de widths devem usar os **novos** glyph IDs do subset, não os originais.
- O operador `TJ` no stream de conteúdo também deve usar os novos glyph IDs.
- A fonte resultante deve ser um ficheiro OpenType válido que `ttf-parser::Face::parse` aceite (mesmo sem `cmap`, que o subsetter remove de propósito).
- Se o subsetting falhar para uma fonte, o exportador deve fallback para embeber a fonte completa (comportamento actual).

---

## Instrução

1. Criar `03_infra/src/export/subset.rs` com a estrutura e função pública:
   ```rust
   pub struct FontSubset {
       pub data: Vec<u8>,
       pub mapping: std::collections::HashMap<u16, u16>, // old_gid → new_gid
   }

   pub fn subset_font_with_mapping(
       font_data: &[u8],
       char_to_old_gid: &std::collections::BTreeMap<char, u16>,
       additional_gids: &std::collections::BTreeSet<u16>,
   ) -> Option<FontSubset>
   ```
   - Recebe os pares `(char, old_glyph_id)` usados no documento (obtidos dos `ShapedGlyph`).
   - Recebe `additional_gids` — glyph IDs adicionais sem codepoint próprio que
     devem ser preservados (ex.: glifos de ligature como "fi", "fl", "ffi").
   - Usa `subsetter::GlyphRemapper` para atribuir novos glyph IDs consecutivos
     a todos os glyphs preservados (incluindo `.notdef` no ID 0).
   - Usa `subsetter::subset` para gerar o subset. O `face_index` passado é 0
     porque o `font_data` já é o byte-slice da face individual (extraída de
     `.ttc` previamente por `FontSlot`).
   - Constrói o mapa `old_gid → new_gid` a partir do `GlyphRemapper` após o
     subset. **Não** reparsear a `cmap` do subset resultante — o subsetter
     remove a tabela `cmap` de propósito.
   - Retorna `None` apenas se o subsetting falhar (fonte inválida ou erro do
     subsetter). Tanto TrueType (`glyf`) como CFF/OpenType (`CFF `) são
     suportados pelo `subsetter`.

2. Criar função auxiliar pública:
   ```rust
   pub fn remap_glyph_id(old_id: u16, mapping: &std::collections::HashMap<u16, u16>) -> u16
   ```
   - Retorna o novo ID se existir no mapa, senão retorna 0 (`.notdef`).

3. Alterar `03_infra/src/export/builder.rs`:
   - Em `build_cidfont` e `build_multifont`, antes de embeber `font_data`, chamar `subset_font_with_mapping`.
   - Se devolver `Some(FontSubset)`, usar `subset.data` como bytes a embeddar e `subset.mapping` para remapear.
   - Re-mapear `mappings: Vec<(char, u16)>` para novos glyph IDs.
   - Usar a `Face` parseada a partir do subset para calcular `widths_array`.
   - Se devolver `None`, manter comportamento actual (fonte completa) e mapping vazio.
   - Manter a lógica de descritor PDF conforme P560/P797/P882/P883: CFF1 → `/CIDFontType0` + `/FontFile3 /Subtype /CIDFontType0C` (apenas a tabela `CFF ` extraída), CFF2 → `/CIDFontType0` + `/FontFile3 /Subtype /OpenType` (SFNT completo), TrueType → `/CIDFontType2` + `/FontFile2` (SFNT completo). Todos os streams de fonte são comprimidos com FlateDecode (P883).

4. Alterar `03_infra/src/export/fonts.rs`:
   - `widths_array` e `to_unicode_cmap` operam sobre `mappings` já re-mapeados.
   - **P521**: criar `cluster_text(glyphs, text) -> Vec<(old_gid, hex_utf16be)>`.
     As fronteiras de cluster são calculadas a partir do **conjunto ordenado**
     de valores de byte (`g.cluster`), não da posição sequencial no vector de
     glifos. Isto torna o algoritmo correcto tanto para LTR como para RTL
     (onde rustybuzz devolve glifos em ordem visual inversa). Glifos mark que
     partilham o mesmo `cluster` apenas duplicam a substring; apenas a primeira
     ocorrência de cada cluster gera entrada no CMap, as restantes ficam com
     hex vazio.
   - **P521**: `to_unicode_cmap` recebe `&[(new_gid, hex_utf16be)]` e emite
     entradas `beginbfchar` do tipo `<gid> <00660069>` para ligatures.

5. Alterar `03_infra/src/export/stream.rs`:
   - Adicionar `glyph_mapping` ao `FontScenario::Cidfont` e `per_font_glyph_mapping` ao `FontScenario::Multifont`.
   - No emit de `FrameItem::TextShaped`, aplicar `remap_glyph_id(glyph_id, mapping)` antes de serializar no operador TJ. Se o mapping estiver vazio (sem subsetting), manter o `glyph_id` original.

6. Dependências:
   - Remover `oxifont-subset` de `03_infra/Cargo.toml`.
   - Adicionar `subsetter = "0.2.6"` a `03_infra/Cargo.toml` (mesma versão do vanilla Typst 0.15.0).

---

## Critérios de Verificação

```
Dado uma fonte TrueType fixture e used_glyphs = {65, 66}
Quando subset_font(font_data, used_glyphs) é chamada
Então retorna Some(bytes) e ttf_parser::Face::parse(&bytes, 0) é Ok
E face.number_of_glyphs() == 3 (notdef + A + B)

Dado uma fonte CFF fixture e used_glyphs = {65}
Quando subset_font(font_data, used_glyphs) é chamada
Então retorna Some(bytes) e ttf_parser::Face::parse(&bytes, 0) é Ok
E face.tables().cff é Some
E face.number_of_glyphs() >= 2
E o programa CFF resultante é CID-keyed (validado por mutool extract + fontTools, ou por pdftoppm/gs sem erro)

Dado um documento PagedDocument com FrameItem::TextShaped contendo glyph_id 65
Quando export_pdf_with_font é chamado com subsetting activo
Então o PDF gerado é válido e o stream de texto contém o novo glyph ID (tipicamente 1)

Dado subset_font com used_glyphs vazio
Quando chamada
Então retorna Some(bytes) contendo apenas .notdef

Dado o documento 04-math.typ do benchmark P872
Quando compilado cristalino vs vanilla
Então pdffonts mostra sub=yes para NewCMMath-Book (CFF) e o tamanho do PDF aproxima-se do vanilla
```

## Resultado Esperado

- `03_infra/src/export/subset.rs` usa `subsetter`.
- `PdfBuilder` usa subsetting quando possível, incluindo CFF1.
- Testes unitários para subset de TrueType e CFF.
- Teste de integração: PDF gerado mantém texto seleccionável/copiável (ToUnicode CMap válido).
- Quando o subsetting é aplicado, o nome base da fonte no PDF
  (`/BaseFont`) recebe prefixo `AAAAAA+` para marcar o subset
  conforme convenção dos produtores PDF (Passo 517).
- `crystalline-lint .` com zero violations.

## Scope-outs

- Variation fonts (VF) e fontes com múltiplos eixos.
- Subsetting de tabelas OpenType avançadas (GPOS, GSUB, kern) — o subset resultante pode não conter kerning, mas o posicionamento já foi aplicado pelo rustybuzz no `x_offset`/`x_advance`.
- **P940** — fontes bitmap por CBDT/CBLC (ex.: Noto Color Emoji) **não são suportadas** pelo `subsetter` (`UnknownKind`). Nesse caso, o export embute a fonte inteira; o custo de compressão é evitado pelo limiar de P940 no builder (ver `infra/export/builder.md`).
- **P941** — a renderização destes glifos passa a ser feita **como imagens XObject**, não como fonte embutida (ver §P941 em `infra/export/builder.md`). A falha de subsetting CBDT deixa de ser relevante para fontes 100% bitmap, porque essas fontes já não são embutidas.

## Notas P523/P560/P797/P874

- P797 desactivou o subsetting CFF1 porque `oxifont-subset` produzia CFF Name-keyed inválido para `/CIDFontType0` + `Identity-H`.
- P874 substitui `oxifont-subset` por `subsetter`, que converte SID-keyed para CID-keyed e restabelece o subsetting CFF1.
- O trabalho de polimento de descritor PDF (`CID TrueType` vs `CID Type 0C`) foi feito em P560 no `PdfBuilder`.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-30 | Criação — activação do subsetting para P515 | `font_subset.md` |
| 2026-06-30 | P520 — mapping de `additional_gids` via codepoints PUA para ligatures | `font_subset.md`, `subset.rs` |
| 2026-07-01 | P521 — ToUnicode completo para ligatures via `cluster_text` (LTR/RTL) | `font_subset.md`, `fonts.rs`, `builder.rs` |
| 2026-07-04 | P560 — CFF/OpenType não retorna None; descritor PDF tratado no builder | `font_subset.md`, `builder.rs` |
| 2026-07-23 | P874 — substitui `oxifont-subset` por `subsetter` para CFF CID-keyed válido | `font_subset.md`, `03_infra/Cargo.toml`, `subset.rs` |
| 2026-07-23 | P882 — descritor PDF de CFF1 passa a embutir programa CFF puro (`/CIDFontType0C`) em vez de SFNT completo; CFF2 mantém `/OpenType` | `font_subset.md`, `builder.rs` |
| 2026-07-23 | P883 — streams de fonte comprimidos com FlateDecode; teste de regressão para formato CFF1 bare/CFF2 OpenType | `font_subset.md`, `builder.rs`, `tests.rs` |
| 2026-07-31 | P940 — registo de limitação: `subsetter` não suporta CBDT (`UnknownKind`); teste `p940_measure_noto_color_emoji_subset` documenta o fallback de fonte inteira | `font_subset.md`, `subset.rs` |
| 2026-07-31 | P941 — glifos bitmap passam a imagens XObject; a falha de subsetting CBDT deixa de ser relevante para fontes 100% bitmap (não embutidas); desenho em `builder.md` §P941 | `font_subset.md`, `builder.md` |
