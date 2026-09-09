# P1327 — refutação do owner set pelo transcript CLI

Regime A/B sem atestação de isolamento. Veredito: **Violated**, closure
proibida no recorte congelado. Nenhum oracle/expectativa/produto alterado
pelo revisor. A revisão estática anterior permanece registro de um delta
local correto; não constituiu aceitação do transcript completo.

## Medição

Corpus candidato `p1327-ab-cli-candidate.json`, SHA-256
`6ffa5fc5ce6925eb0c85e40749adff5d1688ce006fe2400cdad3a4edeb47fafe`,
binário `5f00502b5055fedf8ca1dc2c879112ac1300e12621e11fdb2c731364450a62f9`.
Wrapper `p1327-cli-candidate.json`, SHA-256
`3940e04d86b519b248f57e3711f815726a9648b005df532146b9496c8df3a317`,
preserva HEAD/diff/stat/inventário before/after do working tree.
O corpus fixa argv, cwd e timestamp por execução; exemplos default/normal
começam em 2026-09-09T10:57:17.216926 e 10:57:17.307200 UTC.

São 312 Preserved e 24 Violated. Todas as falhas pertencem a
`later-error` e `alias-later-error`, nos quatro perfis e três ordens.
Recalculei todas as 24: exit 1 e stdout vazio coincidem; inverter os dois
blocos completos do stderr candidato produz exatamente o stderr esperado.
Não há divergência de conteúdo, range, hint, severidade ou cardinalidade.

Candidato imprime warning antes do erro; vanilla imprime erro antes do
warning. As expressões são `{ import std; global }` e
`{ let renamed = std; import renamed; global }`.

## Causa e redecisão

`04_wiring/src/main.rs:500-507` drena warnings antes de inspecionar result;
`:508-520` só então drena os errors. Fonte SHA-256
`0ee273c4411d9752f14900982f02f481c2bc50f125944296111fbe760d5fae9a`.
L0 vigente `00_nucleo/prompts/wiring.md:120-123` atribui a run_eval a
drenagem warnings/errors pelo formatter L2; SHA-256 bruto
`83d7a1364b136a7d198d3fd84c601336ef2fff687b34987c904f637e8173870d`.

Vanilla ratificado, `typst-cli/src/eval.rs:64-81`, coleta primeiro os errors
e passa errors/warnings para `print_diagnostics`; `compile.rs:727` usa
`errors.iter().chain(warnings)`. A divergência é ordem pública da mensagem,
não mecânica interna irrelevante sob ADR-0107.

A premissa de suficiência de apenas modules.rs foi refutada pelo próprio
critério L0 P1327: outro consumer produtivo é necessário. O GREEN interno
observa erro e sink separadamente, portanto não discrimina a ordem final
de apresentação. Os testes estavam corretamente congelados e o oracle
não deve ser enfraquecido para aceitar esta saída.

Próximo recorte técnico necessário: owner `04_wiring/src/main.rs` e seu L0
`00_nucleo/prompts/wiring.md`, restrito à ordem de diagnostics no comando
eval quando result é Err. Reabrir scope/manifesto e legitimar L0 antes de
editar esse owner; preservar os recibos e congelamentos anteriores como
cadeia refutada. Compile/query, serialização, I/O, target HTML e outras
ordens permanecem fora; uma correção genérica do drain alargaria o efeito
sem necessidade demonstrada. Avaliar sentinelas com outros warnings eval,
sucessos e falhas sem warnings para verificar a composição sem alterar L1.

Esta revisão identifica a causa e o owner adicional, não aprova implementação
ou certificação sucessora. Unknown continua bloqueante e R0 não vira PASS.
