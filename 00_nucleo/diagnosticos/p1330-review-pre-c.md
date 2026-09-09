# P1330 — revisão dos artefatos congelados antes de C

**PASS dos inputs A/B**. Revisor `/root/p1330_review`, regime sem atestação
de isolamento e sem selo de refinamento. Este gate aprova os artefatos
congelados; C continua dependente de integração canônica e RED compilado.

Conferências em `2026-09-09T13:13:54.572Z` e
`2026-09-09T13:15:37.726Z`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
L0 R2/manifesto e baseline são os já pinados na revisão L0 R2.
Proveniência completa da captura: p1330-ab-cli-baseline.json, com horário
inicial `2026-09-09T13:12:10.941866+00:00`, argv/binários/diff-stat integral.
O stat é o R2 revisado: calc.md `249 +++++-`, total
`14 files changed, 3242 insertions(+), 107 deletions(-)`; lista exata dos
arquivos e demais entradas permanece na baseline/revisão preliminar.

| Artefato | SHA-256 conferido |
|---|---|
| p1330-ab-freeze.md | 637d1bf3de8480502a9c6b3be793101d5495dc9e4bbd932e6eb5d2e772be8edf |
| p1330-ab-tests.rs | 6ed8f3af58855a88c9ea15113b38c2dad6029639448b052671b4effed4f5ca59 |
| p1330-ab-p1328-successor.rs | 56edc0c1b8c92d4e25bcfb73d1c3972526ea968f0647ea9462b659c2f517b47f |
| p1330-ab-cli.py | d405464d3d0961f5c23d0d34648e8361281cfdf5399499eb11e30e35951e241c |
| p1330-ab-cli-baseline.json | 8ac2d68d4be81b5e3b9ab6a46e1f8c7aea6469cfdcb421d69c328d5b52229b86 |
| p1330-ab-cli-expected.json | 7eefa5cf55e10b77631cf5ef8d68be2629892fa64309c1274f20d1305112695d |

## Revisão substantiva

Diff direto do sucessor contra p1329-ab-p1328-successor.rs confirma somente
as duas substituições autorizadas MIN→MAX por erro completo: nativo detached
e avaliação pública no argumento. Demais linhas e asserções intactas.
P1329 não recebe sucessão e deve permanecer integral na montagem canônica.

O novo módulo verifica tipo/magnitude sem promover a Float, vizinhos e
precisão acima de 2^53, Float/Decimal além do limite Int, guards com MIN,
origem agregada/ocorrência/value_span distintas, primeiro posicional
detached apesar de decoy resolvível, ausência/vazio, With/nested/Array/Args,
alias, UTF-8/linhas, warning completo e math conservando Content. Os casos
de Args com views sintéticas inconsistentes estão declarados como testes
da seleção da origem, complementados pelas rotas reais.

Auditoria independente em memória de todos os mapas de expectativa/baseline:
332 células únicas, 252 históricas, só overflow/default, overflow/html,
overflow/a11y e overflow/html+a11y mudam. As 248 restantes conservam o
oráculo antigo e coincidem com BASE. As 80 novas seguem VANILLA ou BASE
segundo a dívida declarada. As três rotas pré-vinculadas têm somente o
nome de trace calc.abs construído na expectativa pré-C; o runner compara
saídas candidatas literalmente. Nenhum output é normalizado. A contagem
recebida de 44 diferenças BASE versus sucessor e 220 igualdades integrais
BASE/vanilla foi reproduzida da captura, sem executar novo corpus.
Essas contagens não são GREEN ou prova de paridade geral.

## Integração e limites

Scripts integrate/audit-corpus/close lidos, sem alteração. O audit-corpus
verifica células históricas individuais; close verifica hashes congelados,
inventários exatos dos gates e outputs completos nas três ordens.
O scaffold assemble não representa o snippet final e não deve reconstruí-lo.

Durante a revisão, o integrador detectou falha de postimage: patch com
contexto curto posicionou o módulo novo antes de P1329, em vez de EOF.
Conferência local confirmou runtime ainda idêntico ao baseline e montagem
divergente da ordem canônica. O incidente exige registro e comparação
final exata com baseline + sucessor + snippet, não alteração de oráculos
nem novo freeze se os bytes protegidos permanecerem íntegros. Essa
comparação será auditada junto ao RED antes de C.

Este revisor não escreveu produto/L0/testes/oráculos e não atesta isolamento
técnico. Nenhuma aprovação do candidato ou do fechamento é emitida aqui.
