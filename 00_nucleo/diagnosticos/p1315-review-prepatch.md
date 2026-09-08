# P1315 — GO_PREPATCH

Revisor `/root/p1315_review`, em `2026-09-08T13:45:19Z`. Regime A/B
executado sem atestação de isolamento. Esta decisão libera a materialização
do recorte congelado; não é veredito funcional final ou selo de refinamento.

## Evidência verificada

HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado.
O diff inspecionado contém somente L0 e testes locais; nenhuma alteração
produtiva candidata. `git diff HEAD --stat` no instante da revisão:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 55 ++++++++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 65 ++++++++++++++++++++++++++++
 2 files changed, 119 insertions(+), 1 deletion(-)
```

Entradas auditadas:

- L0 raw `d84772e07c0cce407fcffe341afc4f18813cbde54f0b63fd14616a626bd24b4d`;
  normativo `9a67fcb0cd258ba661221b844e59d149061d4b673a52618bc62092054080b40a`,
  exclusão explícita da única linha canônica Hash do Código.
- Freeze `p1315-ab-freeze.json`, SHA-256
  `c2cd0c90e4ea09082980b7dba5ed960dc8c9415bc4930fbeb6c68c2566ddac4f`.
- RED `p1315-unit-red.json`, SHA-256
  `ba7177838dcae34e4e759c0c3eb562553f920c50efa30a375f3712bd0580b505`.

O RED executou `cargo test -p typst-core --release p1315 --lib`, exit 101,
com duas falhas reais de mensagem line 3 contra line 2 e um controle de
valores aprovado. Antes/depois conservam os hashes do L0 e do código com
testes, conforme recibo. Não é falha de compilação aceita como RED.

A auditoria somente leitura do freeze, executada no namespace host às
`2026-09-08T13:45:15.037Z`, verificou todos os hashes em inputs, os dois
executáveis, o L0 raw, a cardinalidade e as saídas contra os recibos originais.
Foram conferidas 1160 expectativas de 290 casos: 936 históricas, 152 de
ordinal, 48 de valores e 24 controles; 112 expectativas distinguem o
baseline. A execução baseline final contém 1384 processos. Nenhuma
discrepância foi encontrada nessa auditoria.

Para cada expectativa ordinal, foi conferida uma única ocorrência do
fragmento em baseline, vanilla e esperado; X/Y são iguais; o esperado tem o
fragmento vanilla e o exit/stdout/stderr integral do baseline alterando
somente N. Para os demais casos, o esperado é literal do baseline. Os
históricos ainda coincidem integralmente com a execução normal P1314,
incluindo expressão e cwd. Não houve normalização de sufixos, spans ou
traces para declarar igualdade geral.

O runner foi lido integralmente. A coleta exige identidade dos binários
antes/depois; freeze rejeita observações duplicadas, identidade ambígua,
fragmentos ausentes e discrepância entre ordinal manual e vanilla. Compare
exige cobertura exata de normal/repeat/reverse, entradas protegidas e L0
normativo intactos, preservando Unknown como bloqueio. O veredito final
também conferirá argv completo/perfil e identidade do candidato nos recibos.

## Calibração e decisão

A única revisão pré-candidato dos quatro inputs UTF-8 troca construção
bytes+bytes sem suporte por tuplas dos mesmos octetos. O freeze documenta
predecessor e focal por hashes. O focal de 32 processos tem exit 1 e erro
UTF-8 em todos os casos; seu hash coincide com o freeze. O corpus final usa
as expressões corrigidas. É reparo da capacidade do teste atingir a obrigação
de preservação, sem mudar intenção normativa nem adaptar-se a um candidato.

GO_PREPATCH para corrigir exclusivamente N de UnequalLengths conforme L0.
Classificação mantém fluxo contínuo ADR-0127. Conservar freeze, casos,
runner e oráculos; qualquer mudança normativa exige reabrir a primeira
fase afetada. Encerramento depende de GREEN local, A/B completo com replays,
linhagem, build e lint. Não há aprovação de paridade geral CSV, formato
completo de diagnósticos, UTF-8 ou isolamento técnico.
