# Prompt L0 — `infra/font_variant` — Helpers para Variation Fonts
Hash do Código: ad6c824b

**Camada**: L3  
**Criado em**: 2026-07-01  
**Arquivos gerados**: `03_infra/src/font_variant.rs` (novo), `03_infra/src/font_variant_instancer.py` (novo), alterações em `03_infra/src/shaper.rs`, `03_infra/src/pipeline.rs`, `03_infra/src/export/builder.rs`, `03_infra/src/export/stream.rs`, `03_infra/src/export/mod.rs`  
**ADR referência**: ADR-0107, ADR-0108, ADR-0109, ADR-0120

---

## Contexto

O shaper (P525) passou a aplicar coordenadas de eixo OpenType (`wght`, `ital`) via `rustybuzz::Face::set_variations`, calculando avanços correctos para cada peso/estilo. No entanto, o export PDF embutia sempre a instância default da fonte, pelo que `text(weight: 700)` numa fonte VF não era visualmente bold (regressão confirmada em P527/P528).

A P529 validou que a instanciação estática é viável **se feita depois do subsetting**: `oxifont-subset` reduz a fonte para poucos glifos, e `fontTools.varLib.instancer` opera sobre essa fonte pequena em ~0.3s por combinação.

Este módulo centraliza os helpers necessários para:

1. Converter `TextStyle` → `FontVariant`.
2. Mapear `FontVariant` → coordenadas de eixo OpenType (`wght`, `ital`).
3. Detectar se uma fonte é variável (tabela `fvar`).
4. Instanciar estaticamente uma fonte VF subsetada para uma combinação de eixos.

## Restrições Estruturais

- Toda a lógica de instanciação fica em L3 (`03_infra/src/font_variant.rs`).
- L1 continua a expor `FontVariant`, `FontWeight`, `FontStyle`, `FontStretch` como entidades puras.
- O shaper (L3) e a pipeline/export (L3) partilham o mesmo mapeamento `TextStyle` → `FontVariant` → eixos.
- A instanciação só ocorre quando:
  - a fonte for VF (`fvar` presente);
  - a `FontVariant` pedir um peso/estilo diferente do default;
  - a fonte tiver os eixos correspondentes (filtrar eixos inexistentes).
- Eixos não explicitamente pedidos mas presentes na fonte devem ser fixados ao seu valor default, de modo a produzir uma fonte totalmente estática (remover `fvar`/`gvar`).
- O instancer remove `GPOS`/`GSUB`/`GDEF` antes da instanciação, porque o shaper já aplicou kerning/ligatures e o fontTools pode falhar ao processar tabelas OTL subsetadas.
- A dependência de Python + fontTools em runtime deve ser documentada; se não estiver disponível, o export faz fallback para a instância default com aviso.

## Instrução

1. Criar `03_infra/src/font_variant.rs` com as funções públicas:
   ```rust
   pub fn text_style_to_font_variant(style: &TextStyle) -> FontVariant;
   pub fn axis_variations_for_font_variant(
       variant: &FontVariant,
       custom_axes: &[(EcoString, f64)],
   ) -> Vec<rustybuzz::Variation>;
   pub fn is_variable_font(data: &[u8]) -> bool;
   pub fn instantiate_variable_font(
       data: &[u8],
       variations: &[(ttf_parser::Tag, f32)],
   ) -> Option<Vec<u8>>;
   ```
   
   `custom_axes` (P660) contém eixos OpenType explícitos vindos de
   `text.font_axes` (ex.: `("wdth", 62.5)`). São convertidos para
   `ttf_parser::Tag` e adicionados às variações derivadas de `FontVariant`;
   em caso de tag duplicada, o valor explícito vence.

2. Criar `03_infra/src/font_variant_instancer.py` embebido via `include_str!`.
   - Lê a fonte subsetada de stdin (binário).
   - Remove `GPOS`/`GSUB`/`GDEF`.
   - Preenche eixos não pedidos com valores default.
   - Aplica `fontTools.varLib.instancer.instantiateVariableFont`.
   - Escreve a fonte estática em stdout (binário).

3. Mover as funções `text_style_to_font_variant` e `axis_variations_for_font_variant` de `shaper.rs` para `font_variant.rs`; `shaper.rs` importa-as.

4. Na pipeline (`pipeline.rs`):
   - `collect_fonts_from_doc` devolve `Vec<(FontList, FontVariant)>`.
   - `resolve_font` recebe `&FontVariant` e usa `font_book.select_pattern(name, variant)`.
   - `resolve_fonts` devolve `Vec<((FontList, FontVariant), Vec<u8>)>`.

5. No export (`export/mod.rs`, `export/builder.rs`, `export/stream.rs`):
   - `export_pdf_multifont` e `build_multifont` recebem `&[((FontList, FontVariant), Vec<u8>)]`.
   - Após subsetar cada fonte, se houver eixos a aplicar, chamar `instantiate_variable_font`.
   - `FontScenario::Multifont` e `emit_text_pdf`/`emit_shaped_pdf` seleccionam a fonte por `(FontList, FontVariant)`.

## Testes

- `p525_axis_variations_weight_italic` (em `shaper.rs`): mapeamento `FontVariant` → eixos.
- Teste de pipeline: documento com `weight: 700` e `weight: 100` numa VF gera múltiplas fontes embutidas com `/W` distintos.
- Teste E2E: compilação do corpus CFF (P523) continua a passar.

## Notas

- `stretch` e `Oblique(angle)` não são suportados pelo `TextStyle` actual; os ramos correspondentes ficam preparados para futuro.
- A dependência de Python em runtime é uma limitação conhecida; alternativas Rust-native (Fontations/skrifa) devem ser pesquisadas para versões futuras.
