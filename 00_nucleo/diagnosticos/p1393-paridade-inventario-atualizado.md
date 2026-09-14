# P1393 — inventário atualizado de paridade

## Veredito do inventário

O repositório ainda não possui um denominador semântico integral executado no
HEAD atual. Há três camadas de evidência que não podem ser somadas como se
fossem a mesma métrica:

| Camada | Denominador | Estado útil | Limite |
|---|---:|---|---|
| Inventário user-facing antigo | 141 agregados | última conta P415: 75 implementados + 27 parciais | obsoleto; não mede P416–P1392 |
| Inventário público P1357 | 8.724 células profile/path | 8.012 both-present, 544 vanila-only, 168 cristalino-only | presença e kind, não semântica; binário cristalino anterior ao HEAD |
| Matriz P1393 | 20 casos, 54 células ativas | 45 MATCH, 3 DIFFERENCE, 6 UNKNOWN | atual e bilateral, mas amostra pequena |

Portanto, não existe uma percentagem honesta de “toda a linguagem” neste
passo. Na matriz atual, 48/54 células são decididas e 45/48 das decididas são
MATCH (93,75%). Isso mede somente os 20 casos declarados.

## Denominador nominal preservado

O denominador nominal mais amplo continua sendo o crosswalk P1357, executado
contra o mesmo pin vanila hoje ratificado nos quatro perfis. A relação
path-by-path está preservada em
`p1357-parity-backlog.json` (SHA-256
`d097b8c0ed5d81932075733a0569f0358a0c3fb7469bcef8346fd295c12523b9`)
e seu envelope em `p1357-inventory-summary.json` (SHA-256
`f7b4233b3bb7339bbe63289191fcaea3f86794d7a0cb4031cdec05410c37a14f`).

Cada uma das linhas desses anexos recebe em P1393 o estado
`UNKNOWN_CURRENT_HEAD` até reexecução do enumerador com o binário
`9088dfe1...`. A evidência histórica continua válida para o binário P1357 e
não foi apagada. Em particular:

- 145 paths classificados como gap retido somavam 492 células não-MATCH;
- 127 paths de target drift somavam 500 células depois do complemento P1357;
- os 272 nomes são rotas, ancestors e membros, não 272 features independentes;
- 42 extensões cristalinas por perfil não recebem crédito nem são defeito de
  paridade por si sós.

Essa referência satisfaz a enumeração linha-a-linha sem copiar ou renomear os
272 paths. P1393 não promove `BOTH_PRESENT` a comportamento equivalente.

## Reconciliação P1358–P1392

As correções posteriores reduzem o backlog antigo, mas somente no recorte de
seus witnesses:

| Cluster | Estado atual | Evidência |
|---|---|---|
| Int/Str, Bool/None/Auto e Array: field ausente | MATCH escopado | P1336–P1338, anteriores ao inventário P1357 e já absorvidos nele |
| Contexto, bindings e selectors | MATCH escopado | P1339–P1340, já absorvidos por P1357 |
| `luma` e limites alpha | MATCH escopado | P1358 |
| `--pdf-tagged`, `--pretty` e status de `watch` | MATCH escopado, não CLI geral | P1359–P1367 |
| Precedência de setting PDF e formatter de duração | MATCH escopado | P1370–P1371 |
| `block` relativo/fracionário, columns e prefixos inline | MATCH escopado | P1373–P1392 |

Nenhum desses fechamentos autoriza remover por semelhança um sibling nominal
do anexo P1357. O próximo inventário executável deve aplicar deltas por ID.

## Estado por eixo

| Eixo | Evidência atual | O que permanece aberto |
|---|---|---|
| S — sintaxe/diagnósticos | 4 casos atuais MATCH | corpus amplo, spans de várias famílias e diagnósticos de loaders continuam sem replay atual integral |
| E — avaliação | `sys.version` atual MATCH | precedência de callee local em math é DIFFERENCE confirmada para `abs` e `sqrt`; casts/closures/repr têm gaps herdados |
| B — stdlib | 3 casos atuais MATCH; `luma` fechado em P1358 | 145 rotas retidas incluem famílias math, state, selectors, outline e aliases; sem reexecução atual por rota |
| I — introspecção | query/counter smoke atual MATCH | members `state_*`, `counter_*`, `outline.entry.*` e selectors herdados permanecem candidatos não-MATCH |
| L — layout | smoke e geometria atuais MATCH; P1373–P1392 fecham recortes | bidi, footnotes completas, flow complexo e formas fora dos certificados seguem parciais/UNKNOWN |
| X — export | SVG/HTML/capability atuais MATCH; PDF observável tem uma diferença | PNG tem 2 UNKNOWN por defeito de baseline; `format.*` é a maior família nominal nova ainda sem owner único |
| C — CLI | help/packages atuais MATCH; fragmentos recentes fechados | `--ppi` não existe no cristalino; query/features e CLI não coberta integralmente continuam abertas |

## Cobertura declarada, medida e paridade medida

- **Cobertura declarada atual:** `UNKNOWN/denominador`, porque a tabela de 141
  agregados não foi recontada e a presença P1357 antecede o HEAD.
- **Cobertura medida atual:** 48 decididas / 54 células ativas da matriz P1393;
  seis permanecem UNKNOWN.
- **Paridade medida atual:** 45 MATCH / 48 decididas = 93,75%, exclusivamente
  no corpus de 20 casos.
- **Cobertura nominal histórica no pin atual:** 8.012 BOTH_PRESENT / 8.724
  células profile/path = 91,84%; não é paridade semântica.

## Conclusão

O cristalino tem um núcleo grande de comportamento compatível, mas ainda
faltam tanto produto quanto prova. O menor defeito de linguagem reproduzido no
HEAD é a precedência lexical de funções locais em modo math. A maior família
nominal é `format.*`; o bloqueio imediato do harness é PNG sem PPI explícito.
Os três são problemas diferentes e não devem ser fundidos numa correção.
