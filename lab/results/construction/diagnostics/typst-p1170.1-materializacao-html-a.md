# P1170.1 — materialização de `html.a`

**Data:** 2026-08-25  
**Estado:** GREEN; parado antes de staging  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Árvore:** working tree não commitado P1168–P1170; índice vazio  
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Gate, L0 e RED

O dono aprovou o contrato P1170 em 2026-08-25. O primeiro resselo atualizou
`compiler/stdlib/html.rs` para `333bb929`, sem drift. O teste
`p1170_1_html_module_expoe_a` falhou causalmente com binding ausente:
`0 passed; 1 failed; 5240 filtered out`.

## Implementação

Foi acrescentado exatamente um wrapper estático e uma entrada `a` na tabela
tipada. `A_ATTRS` contém exatamente oito específicos e reutiliza a tabela única
dos 76 globais:

- quatro strings livres: `download`, `href`, `hreflang`, `type`;
- `ping`, lista de strings separada por espaço;
- `referrerpolicy`, none-vazio ou oito tokens;
- `rel`, lista restrita aos 27 tokens medidos;
- `target`, string livre com diagnóstico de união medido.

Body omitido preserva `HtmlBody::None`; body fornecido preserva Content. Não
houve alteração em entidade, exporter, default ou pipeline.

## GREEN e decalque

Os três testes L1 focados passaram (`3 passed; 5240 filtered out`). O teste CLI
focado passou (`1 passed; 61 filtered out`). A fixture `html.a` aninhada em
`html.div` foi byte-idêntica ao vanilla (`cmp` exit 0), incluindo escaping,
ordem, lista `rel` e body com `strong`.

`target: 1` preservou a mensagem esperada que enumera os quatro keywords ou
string. A suíte integral terminou com exit 0; o CLI contém agora 62 testes,
todos verdes, e os dois testes do linter também passaram. Os três doctests de
core permaneceram ignorados.

O fechamento documental alterou o L0; o resselo final atualizou a linhagem
para `2699377c` e terminou com zero drift.

O controle de topo continua produzindo `<body><a ...>` no cristalino, contra
`<body><p><a ...></a></p>` no vanilla. P1170.1 não mascarou nem corrigiu esse
gap de agrupamento phrasing.

## Scope-outs

P1171 continua dono de `br`, tabela void e whitespace. Agrupamento phrasing de
topo exige auditoria e gate próprios. Demais tags, raw, frame, CSS, MathML e
positions continuam abertas. Nenhum ficheiro foi staged ou commitado.
