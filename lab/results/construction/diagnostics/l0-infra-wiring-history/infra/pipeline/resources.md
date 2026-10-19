# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/resources.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `d355ced9e16586426cf850f65fa89769b126dd5e45314795bf577c95ec367da8`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — resources
Hash do Código: 6799e651

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/resources.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Possui coleta ordenada e resolução das fontes do documento, fallback por família, variações explícitas e instanciação de fontes variáveis. Recursa pelos containers e pelas camadas marginais existentes, preservando a chave completa e os diagnósticos vigentes.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## Helpers privados de dispatch (Passos 140B + 141 + 146)

```rust
fn collect_fonts_from_doc(doc: &PagedDocument) -> Vec<FontList>;
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
fn resolve_fonts(
    font_lists: &[FontList],
    font_book:  &FontBook,
    world:      &dyn World,
) -> Vec<(FontList, Vec<u8>)>;

// Helper preservado do 140B (test-only no pipeline.rs):
#[allow(dead_code)]
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList>;
```

- `collect_fonts_from_doc` (Passo 146): itera `doc.pages →
  items` recursivamente (atravessa `FrameItem::Group`) e
  devolve **todas** as `FontList` distintas em ordem de
  primeira ocorrência. Dedup estrutural via `Vec::contains`.
- `resolve_font` (Passo 141): itera `font_list.as_slice()` em
  ordem. Para cada família, consulta
  `font_book.select(name, &FontVariant::default())`; se devolve
  `Some(index)`, chama `world.font(index)`; se devolve
  `Some(font)`, devolve os bytes. **Primeira família a completar
  ambos os passos vence**. Cenário patológico (índice stale)
  não curto-circuita.
- `resolve_fonts` (Passo 146): map-filter de `resolve_font`
  sobre `&[FontList]`. Devolve `(FontList, bytes)` por entrada
  resolvida (silent drop para entries que não resolvem;
  consistente com 140B).
- `first_font_from_doc` (Passo 140B, **preservado em
  `#[allow(dead_code)]`**): historicamente o entry-point do MVP
  single-font (devolvia primeira `FontList`). Substituído pelo
  dispatch multi-font no Passo 146; permanece como referência
  e para os testes unitários do 140B continuarem activos.
- Funções privadas ao módulo e cobertas por testes unitários
  em `#[cfg(test)] mod tests`.

Multi-font materializada no Passo 146 (ADR-0055 decisão 5
anotada). Variant-aware (ADR-0055bis, candidata) e subsetting
(ADR-0056, candidata) permanecem fora do escopo actual.

## P906 — `collect_fonts_in_items` cego a `FrameItem::Glyph`

**Contexto**: mecanismo de esticamento horizontal (`compiler/layout.md`
§P906) — achado mais fundo, na SELECÇÃO de quais fontes embutir no PDF
(distinto do embedding/subsetting, `export/builder.md` §P906).

**Achado, confirmado por instrumentação directa** (não inferido):
`collect_fonts_in_items` (usada por `collect_fonts_from_doc`, que decide
Cidfont-vs-Multifont e QUAIS fontes resolver) tinha o braço
`FrameItem::Glyph { .. } => {}` — ignorava por completo qualquer glifo de
esticamento matemático. Como a decisão Cidfont/Multifont é feita
EXCLUSIVAMENTE a partir de `style.font` visto em `Text`/`TextShaped`, uma
equação cujo ÚNICO conteúdo a precisar da fonte companion MATH fosse um
glifo de esticamento (`underbracket(a+b+c)` sem mais texto itálico
matemático à volta — caso isolado por outro achado incidental, P906
também, em `apply_math_default` não recursar `MathUnderover`) escolhia só a
fonte de corpo como candidata Cidfont única. O `glyph_id` do esticamento,
correcto na fonte MATH (confirmado via `FallbackFontMetrics`, que já
resolve correctamente para efeitos de LAYOUT), caía fora do subset embutido
— glifo `.notdef` no PDF real, apesar do mecanismo de layout estar
inteiramente correcto e testado.

**Correcção**: `collect_fonts_from_doc`/`collect_fonts_in_items` ganham
`world: &dyn World`, constroem um `FallbackFontMetrics` local, e o braço
`FrameItem::Glyph { style, base_char, .. }` chama o novo método
`FallbackFontMetrics::resolve_font_combo(base_char, style)` (`infra/
font_metrics.md` §P906 — mesmo mecanismo `covering`/`resolve_primary_with_
math_fallback` já usado internamente para LAYOUT, agora reaproveitado para
devolver a IDENTIDADE da fonte, não dados de glifo) — o resultado entra na
mesma lista `seen`/dedup, como mais um span de "texto" para efeitos de
selecção.

## P836 — chave de fonte com variações explícitas

`collect_fonts_from_doc`/`resolve_fonts` passam a chavear por
`(FontList, FontVariant, FontVariations)` (o terceiro componente vem de
`TextStyle::variations`, `unwrap_or_default`): dois runs com o mesmo
`FontVariant` mas `variations:` distintas embutem fontes instanciadas
distintas — paridade vanilla (`FontInstance` chaveado por variações
completas).

O gate `needs_variable_font_instancer` e o desvio single-font→multi-font
passam a usar `axis_variations_for_text_style` (fusão derivados +
explícitos), de modo que um documento cuja única variação é explícita
(ex. `wght: 250` com weight regular) também dispara a instanciação.

---

## P1140.5-A — preservação de grupos semânticos

### Medição antes da decisão

Os walkers da pipeline (`pipeline.rs:1106,1161`) descem apenas por Group/Link;
não há ramo semântico. O artefato PDF cristalino medido é `Tagged: no`.

### Decisão

Todas as passagens de fonte/shaping/normalização atravessam
`FrameItem::Semantic.items` como container transparente e preservam
kind/placement/alt byte a byte. Nenhuma passagem converte `alt` em texto nem
remove o wrapper. HTML continua eixo separado até medir seu contrato próprio.
