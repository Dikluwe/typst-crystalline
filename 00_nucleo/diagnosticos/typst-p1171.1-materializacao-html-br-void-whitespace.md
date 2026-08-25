# P1171.1 — materialização de `html.br`, void e whitespace

**Data:** 2026-08-25  
**Estado:** GREEN; parado antes de staging  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Árvore:** working tree não commitado P1168–P1171; índice vazio  
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Gate, resselo e RED

O dono aprovou P1171 em 2026-08-25. O primeiro resselo atualizou L1 para
`6d1e95bd` e L3 para `a41110f7`, com zero drift. O RED L1 falhou porque
`html.br` estava ausente; os dois REDs L3 falharam porque void recebia end tag
e espaços de borda eram preservados (`0 passed; 2 failed; 848 filtered out`).

## Implementação

L1 ganhou exatamente `html.br`, sem específicos e sem body. O helper void
reutiliza os 76 globais, rejeita qualquer posicional e produz
`HtmlBody::Unset`. Nenhuma entidade pública mudou.

L3 ganhou uma tabela única das 13 tags void. Void sem body emite somente start
tag; body content retorna `HTML void elements must not have children`. A
normalização do body remove `Content::Space` apenas nas bordas e junto a void,
preservando espaço entre siblings inline. Não houve `trim()` global, mudança
de eval ou pipeline.

## GREEN

- L1 focado: 1 teste GREEN;
- L3 focado: 2 testes GREEN (`848 filtered out`);
- CLI focado: 1 teste GREEN (`62 filtered out`);
- fixtures formatadas `A/br/B` e dois spans: byte-idênticas ao vanilla
  (`cmp` exit 0 em ambas);
- filho de `html.elem("br")`: exit 1 com diagnóstico medido;
- suíte integral: exit 0; core `5244`, infra `850`, shell `55`, wiring unitário
  `2`, CLI `63`, linter `2`; três doctests core ignorados.

A primeira suíte integral encontrou somente a sentinela P1168 que ainda
declarava `br` fora do lote; a expectativa histórica foi atualizada e a
repetição integral passou.

O fechamento documental alterou os L0s; o resselo final atualizou L1 para
`6c283969` e L3 para `5bd73e7d`, novamente com zero drift.

## Scope-outs

Agrupamento phrasing de topo permanece aberto. As outras 12 tags void têm
serialização correta via `html.elem`, mas não receberam constructors tipados.
Demais tags, raw, frame, CSS, MathML e positions continuam fora. Nenhum staging
ou commit foi realizado.
