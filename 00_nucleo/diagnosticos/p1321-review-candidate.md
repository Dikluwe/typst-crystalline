# P1321 — candidato conforme ao consumer, A/B bloqueado por origem agregada

Veredito: NÃO fechar P1321 contra o freeze A/B atual. O candidato corrige
as precedências e obedece literalmente ao uso de args.span do L0, mas a
inferência de que loading recebe dados suficientes foi refutada em missing
sem named source. Não alterar expectativas para converter esse achado em êxito.

## Proveniência reproduzível

Executável `/tmp/p1321-target.669PuL/release/typst`, SHA-256
`deb6aed85d133665b00d416fabee92aeda04196ff76b6a734ef0451bbdeaea3c`.
Fonte `8024109818bfa83a8cf62f01dd1962b1e3dc01100e22d09eb6333b623292faa6`,
L0 `878571e48bb3dd48eb573e3e0af7eba35ce83bde3f827a8338cfbd0d1a33bf1a`.
HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093, working tree não commitado;
build/RED/A/B conservam horários, fontes e diff/stat. Build passou entre
2026-09-08T20:31:02.881862+00:00 e 2026-09-08T20:32:21.203385+00:00.

Checker independente `p1321-review-audit.cjs`, executado com argumento
`00_nucleo/diagnosticos/p1321-build.json`, produz exit 1 e recibo integral
`p1321-review-candidate-audit.json`. A/B bruto auditado SHA-256
`d1699b5ae42327c7ba7254f5f1f2830453d0cd225fce8f79807cf329aa2a1a25`.

## Resultado e causa

São 32 falhas por ordem normal/repeat/reverse, total 96 em 1320 comparações;
408 por ordem preservam exatamente os observáveis congelados. Todas as
falhas são as oito expressões missing sem named source em quatro perfis:
missing_none, missing_delimiter, missing_unknown, alias_missing, args_missing,
sink_missing, with_missing e map_missing. Mensagem é correta; âncora é menor.
Em `csv()`, candidato aponta somente `()` na coluna 3; vanilla aponta `csv()`
na coluna 0. O mesmo fenômeno conserva o tamanho lexical do callee nas outras
rotas, e não é ruído de ordem ou feature.

Fonte da causa: `compiler/eval/call_dispatch.rs:398` inicializa Args.span com
args_node.span(), a lista de argumentos. `transport_native_call_span` em
linhas 460–478 transmite o span da chamada integral somente a encoders
JSON/TOML/YAML e panic; CSV não recebe esse transporte. A chamada normal em
1630 passa call.span ao transportador, portanto a informação existe naquele
outro owner e já não está disponível na entrada do consumer CSV. O L0 P1321
proíbe alterar dispatch/outros owners; há uma fronteira real a reabrir.

Não há razão para mexer no parser ou deduzir ranges por texto/nome da função.
O próximo trabalho requer decisão explícita do escopo e L0 proprietário
afetado antes de novo código, preservando estes recibos. Não houve duas
revisões locais sem ganho: a primeira execução completa encontrou uma causa
nova e sua origem precisa, então a ação apropriada é reabrir essa fronteira.

## O que a revisão aprovou

O diff desde RED modifica só missing em arg_csv_source, helper privado
csv_finish_args e ordem de native_csv. O helper escolhe primeiro remanescente
causal mesmo antes da primeira fonte, depois dos casts; opções e parser
permanecem textualmente intactos. Não introduz I/O/API/entidade/trait.
Todo módulo de testes é byte-idêntico ao RED e nenhuma entrada congelada
do A/B ou inputs históricos pinados P1319 foi alterada.

Recalculei independentemente, pela fórmula das fontes do linter:

- B reverso: c0f17d3b47e3d98cd8255cd8c36dd35279bbdf98247a1077a50dc81ab1922210;
- A efetivo: 446dd8d6d5b5aa691cbdbd363bec3b5ab1d68a19312d9695d243b0c8810a3704;
- Núcleo: 5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24.

Todos conferem com os headers/pin. Esse sucesso de linhagem não suplanta
a falha funcional integral. Regime permanece A/B sem atestação de isolamento,
sem selo e sem alegação de paridade geral.
