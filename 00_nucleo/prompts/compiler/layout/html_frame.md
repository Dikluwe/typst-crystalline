# Prompt L0 — layout da ilha `html.frame`

**Estado:** APROVADO NO GATE ADR-0127 P1424 EM 2026-09-16  
**Camada:** L1  
**Ficheiro alvo exclusivo:** `01_core/src/compiler/layout/html_frame.rs`  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0128, ADR-0129

## Medição anterior à decisão

`typst-html/src/rules.rs:88-91` torna `FrameElem` transparente no target
paginado. `typst-html/src/convert.rs:140-154` relayouta seu body sob target
`Paged`, região ilimitada, e entrega um frame intrínseco ao encoder HTML.
Sondas P1424 mediram `rect(width: 10pt, height: 5pt)` com `viewBox="0 0 10 5"`
e nesting de frame byte-idêntico ao frame simples.

## Obrigação

Esta unidade contém somente a lógica de layout da variante
`Content::HtmlFrame`, em forma B:

- no dispatcher paginado, `layout(layouter, elem)` delega uma vez ao body e
  não emite wrapper, margem ou item próprio;
- a entry point intrínseca cria Layouter isolado com chain, métricas e image
  sizer injetados, executa o body numa região de largura/altura ilimitadas e
  devolve `Page` transparente recortada às dimensões reais;
- nesting colapsa naturalmente porque o braço paginado de um frame interno é
  transparente;
- posições são normalizadas ao referencial local sem cortar tinta suportada;
- callbacks/erros usam os canais existentes e nunca executam Func em layout.

O dispatcher central conserva somente o braço magro e o entry point público de
fachada. Não importar L3, não criar renderer SVG, não paginar em A4 e não usar
`Content::HtmlElem` ou tag sentinela.

## Aceitação

Texto simples produz dimensões intrínsecas positivas; rect 10pt×5pt produz
page 10×5 transparente; frame aninhado é geometricamente idêntico; no layout
paginado ordinário o body ocupa exatamente a posição que teria sem wrapper.
IDs/bytes SVG não pertencem a esta unidade.
