# P1172.1 — materialização do agrupamento phrasing HTML

**Data:** 2026-08-25  
**Estado:** GREEN; parado antes de staging  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Árvore:** working tree não commitado P1168–P1172; índice vazio  
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Classificação, resselo e RED

P1172 provou que Content/repr coincide e a causa é exclusivamente o
`block_sequence` L3. A mudança foi classificada como correção interna de
paridade em fluxo contínuo ADR-0127. O primeiro resselo atualizou L3 para
`19531b70`, sem drift. O RED falhou porque `span/div/a` não recebia os dois
wrappers de parágrafo (`0 passed; 1 failed; 850 filtered out`).

## Implementação

L3 ganhou uma tabela estática das tags phrasing visíveis medidas em
`tag.rs:290-350`, excluindo as display-none de `property.rs:62-76`.
`block_sequence` alimenta o buffer de parágrafo para HtmlElem groupable e
fecha o buffer para boundaries. HtmlElem isolado groupable também recebe p.
Custom/desconhecida permanece boundary. Bodies de HtmlElem continuam inline;
nenhuma entidade, eval, API ou fase mudou.

## GREEN

- teste L3 P1172.1 GREEN;
- dois testes P1171.1 de void/whitespace GREEN;
- teste CLI P1172.1 GREEN (`63 filtered out`);
- sete fixtures diferenciais byte-idênticas (`cmp` exit 0): `a` tipado,
  `html.elem("a")`, `span`, `span/div/a`, parbreak, phrasing em div e `a`
  transparente com body block;
- suíte integral exit 0: core `5244`, infra `851`, shell `55`, wiring unitário
  `2`, CLI `64`, linter `2`; três doctests core ignorados.

O fechamento documental alterou o L0; o resselo final atualizou L3 para
`afe9f695` e terminou novamente com zero drift.

## Scope-outs

Não foram adicionados constructors. Raw, CSS, MathML, frame, pretty e tags
ainda ausentes continuam fora. A tabela aceita somente tags conhecidas; custom
não é inferida como phrasing. Nenhum staging ou commit foi realizado.
