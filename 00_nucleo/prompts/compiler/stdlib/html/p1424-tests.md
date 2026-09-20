# Prompt L0 — testes A/B L1 de `html.frame` P1424
Hash do Código: be8ae620

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
intrínseca e não atesta export HTML/SVG. Ele deriva somente do comportamento
público descrito acima.

## Proveniência e limite

As expectativas pertencem a este L0. Alterações devem ser justificadas por
comportamento público e cobertas por testes RED→GREEN; autoria, isolamento,
selos, hashes de implementação e ordem de agentes não fazem parte da aceitação.
