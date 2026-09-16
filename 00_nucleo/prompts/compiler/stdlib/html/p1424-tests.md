# Prompt L0 — testes A/B L1 de `html.frame` P1424

Hash do Código: bb84f682

**Camada:** L1 (testes)  
**Ficheiro alvo exclusivo:** `01_core/src/compiler/stdlib/html/p1424_tests.rs`  
**Owner produtivo observado:** `00_nucleo/prompts/compiler/stdlib/html.md`

## Obrigação

Este consumer exclusivamente de teste congela a superfície de linguagem P1424
sem legitimar código produtivo. Deve provar, a partir do módulo público e sem
usar um constructor privado candidato:

- binding `html.frame` separado da tabela de tags e `repr` da função `frame`;
- assinatura de exatamente um `Content` posicional obrigatório, com os três
  diagnósticos públicos para ausência, named e segundo positional;
- carrier `Content::HtmlFrame(HtmlFrameElem { body })`, distinto de
  `HtmlElem`, body nu, vazio ou bytes SVG;
- `plain_text`, `map_text` e `map_content` atravessam o body e preservam a
  variante;
- layout paginado do frame simples e aninhado é observacionalmente idêntico ao
  body nu em páginas, geometria, itens e posições suportados.

O teste não conhece a implementação candidata, não escolhe a entry point
intrínseca e não atesta export HTML/SVG. Foi derivado do manifesto e contrato
P1424 congelados no baseline `79c7a6d60`.

## Proveniência e limite

Manifesto SHA-256 `e4164634bd958e5889778484d42ac2b37c11f7e8eaccefff89e0ab9271b2dfab`;
contrato SHA-256 `d92d2ccf4efd592c92cfa59a30dcfc8beb5707976aa167f7ac6f90e87f6b8f31`.
Execução: **executado sem atestação de isolamento**. Alterar as expectativas
exige invalidar e ressellar a cadeia P1424.
