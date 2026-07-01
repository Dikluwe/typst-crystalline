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

1. Criar `03_infra/src/export/subset.rs` com a estrutura e função pública:
   ```rust
   pub struct FontSubset {
       pub data: Vec<u8>,
       pub mapping: std::collections::HashMap<u16, u16>, // old_gid → new_gid
   }

   pub fn subset_font_with_mapping(
       font_data: &[u8],
       char_to_old_gid: &std::collections::BTreeMap<char, u16>,
   ) -> Option<FontSubset>
   ```
   - Recebe os pares `(char, old_glyph_id)` usados no documento (obtidos dos `ShapedGlyph`).
   - Usa `oxifont_subset::subset_with_gid_set` para gerar o subset.
   - Reconstrói o mapa `old_gid → new_gid` parseando a cmap do subset resultante.
   - Inclui `.notdef` (glyph ID 0) sempre no subset.
   - Retorna `None` se a fonte for CFF/OpenType sem `glyf` ou se o subsetting falhar.

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

4. Alterar `03_infra/src/export/fonts.rs`:
   - `widths_array` e `to_unicode_cmap` operam sobre `mappings` já re-mapeados.

5. Alterar `03_infra/src/export/stream.rs`:
   - Adicionar `glyph_mapping` ao `FontScenario::Cidfont` e `per_font_glyph_mapping` ao `FontScenario::Multifont`.
   - No emit de `FrameItem::TextShaped`, aplicar `remap_glyph_id(g.glyph_id, mapping)` antes de serializar no operador TJ. Se o mapping estiver vazio (sem subsetting), manter o `glyph_id` original.

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
- Quando o subsetting é aplicado, o nome base da fonte no PDF
  (`/BaseFont`) recebe prefixo `AAAAAA+` para marcar o subset
  conforme convenção dos produtores PDF (Passo 517).
- `crystalline-lint .` com zero violations.

## Scope-outs

- CFF subsetting (fontes Type1/CFF caem em fallback fonte completa).
- Variation fonts (VF) e fontes com múltiplos eixos.
- Subsetting de tabelas OpenType avançadas (GPOS, GSUB, kern) — o subset resultante pode não conter kerning, mas o posicionamento já foi aplicado pelo rustybuzz no `x_offset`/`x_advance`.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-30 | Criação — activação do subsetting para P515 | `font_subset.md` |
