# Prompt L0 — `infra/export/font_subset` — Subsetting de fontes no PDF

**Camada**: L3  
**Criado em**: 2026-06-30  
**Arquivos gerados**: `03_infra/src/export/subset.rs` (novo), alterações em `03_infra/src/export/builder.rs`, `03_infra/src/export/fonts.rs`, `03_infra/src/export/stream.rs`  
**ADR referência**: ADR-0027 (revisão de Opção A para Opção B), ADR-0055, ADR-0120  

---

## Contexto

A ADR-0027 escolheu embeber a fonte TrueType completa no PDF (Opção A) para evitar a complexidade do remapeamento de glyph IDs. O Passo 515 (Trilha 5) activa o subsetting para reduzir o tamanho dos PDFs e aproximar o cristalino da paridade de produção com Typst 0.15.0.

O exportador actual já emite CIDFont + Identity-H e usa `FrameItem::TextShaped` (ADR-0120). O subsetting deve operar sobre os glyph IDs reais usados no documento e remapeá-los para um subconjunto compacto antes de gerar o PDF.

## Restrições Estruturais

- Toda a lógica de subsetting fica em L3 (`03_infra/src/export/subset.rs`).
- L1 não conhece subsetting — continua a usar `ShapedGlyph.glyph_id` como índice real na fonte.
- O subset deve preservar o glyph ID 0 como `.notdef`.
- O ToUnicode CMap e o array `/W` de widths devem usar os **novos** glyph IDs do subset, não os originais.
- O operador `TJ` no stream de conteúdo também deve usar os novos glyph IDs.
- A fonte resultante deve ser um ficheiro TrueType/OpenType válido que `ttf-parser::Face::parse` aceite.
- Se o subsetting falhar para uma fonte, o exportador deve fallback para embeber a fonte completa (comportamento actual).

## Instrução

1. Criar `03_infra/src/export/subset.rs` com a função pública:
   ```rust
   pub fn subset_font(font_data: &[u8], used_glyphs: &std::collections::BTreeSet<u16>) -> Option<Vec<u8>>
   ```
   - Parsear a fonte original com `ttf_parser::Face::parse(font_data, 0)`.
   - Criar um mapa de glyph IDs originais → novos glyph IDs sequenciais, começando em 1 (0 reservado a `.notdef`).
   - Incluir `.notdef` (glyph ID 0 original) sempre no subset.
   - Construir uma fonte minimamente válida contendo:
     - `head`, `hhea`, `maxp`, `post`, `loca`, `glyf` (TrueType) ou equivalente CFF (CFF opcional para esta fase).
     - Tabela `cmap` mapeando codepoints usados para os novos glyph IDs.
     - Tabela `hmtx` com advances dos glyphs incluídos.
     - Tabela `name` mínima (preservar nome da família).
   - Para esta fase, **TrueType outlines (`glyf`) são obrigatórios**; CFF é scope-out.
   - Retornar `None` se a fonte for CFF ou se a construção falhar.

2. Criar função auxiliar pública:
   ```rust
   pub fn remap_glyph_id(old_id: u16, mapping: &std::collections::HashMap<u16, u16>) -> u16
   ```
   - Retorna o novo ID se existir no mapa, senão retorna 0 (`.notdef`).

3. Alterar `03_infra/src/export/builder.rs`:
   - Em `build_cidfont` e `build_multifont`, antes de embeber `font_data`, chamar `subset_font`.
   - Se `subset_font` devolver `Some(subset_data)`, usar o subset e o mapa de remapeamento.
   - Passar o mapa de remapeamento para `to_unicode_cmap`, `widths_array` e `text_to_hex_string`/`emit_shaped_pdf`.
   - Se `subset_font` devolver `None`, manter comportamento actual (fonte completa).

4. Alterar `03_infra/src/export/fonts.rs`:
   - Adicionar função `map_chars_to_glyphs_subset` (ou parâmetro opcional de remapeamento) que gere o `mappings: Vec<(char, u16)>` com novos glyph IDs.
   - `widths_array` e `to_unicode_cmap` devem aceitar o mapa de remapeamento opcional.
   - `text_to_hex_string` (ou equivalente usado por `emit_shaped_pdf`) deve mapear `ShapedGlyph.glyph_id` original para o novo ID.

5. Alterar `03_infra/src/export/stream.rs`:
   - No emit de `FrameItem::TextShaped` (caminhos CIDFont e Multifont), aplicar o remapeamento de `glyph_id` antes de serializar no operador TJ.

## Critérios de Verificação

```
Dado uma fonte TrueType fixture e used_glyphs = {65, 66}
Quando subset_font(font_data, used_glyphs) é chamada
Então retorna Some(bytes) e ttf_parser::Face::parse(&bytes, 0) é Ok
E face.number_of_glyphs() == 3 (notdef + A + B)

Dado uma fonte CFF fixture e used_glyphs = {65}
Quando subset_font(font_data, used_glyphs) é chamada
Então retorna None (fallback para fonte completa)

Dado um documento PagedDocument com FrameItem::TextShaped contendo glyph_id 65
Quando export_pdf_with_font é chamado com subsetting activo
Então o PDF gerado é válido e o stream de texto contém o novo glyph ID (tipicamente 1)

Dado subset_font com used_glyphs vazio
Quando chamada
Então retorna Some(bytes) contendo apenas .notdef
```

## Resultado Esperado

- `03_infra/src/export/subset.rs` criado.
- `PdfBuilder` usa subsetting quando possível.
- Testes unitários para subset de TrueType e fallback de CFF.
- Teste de integração: PDF gerado mantém texto seleccionável/copiável (ToUnicode CMap válido).
- `crystalline-lint .` com zero violations.

## Scope-outs

- CFF subsetting (fontes Type1/CFF caem em fallback fonte completa).
- Variation fonts (VF) e fontes com múltiplos eixos.
- Subsetting de tabelas OpenType avançadas (GPOS, GSUB, kern) — o subset resultante pode não conter kerning, mas o posicionamento já foi aplicado pelo rustybuzz no `x_offset`/`x_advance`.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-30 | Criação — activação do subsetting para P515 | `font_subset.md` |
