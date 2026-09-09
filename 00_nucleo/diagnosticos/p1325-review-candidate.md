# P1325 — revisão do candidato

Veredito `PASS_CANDIDATE_SCOPE`, aguardando gates finais. Regime A/B sem
atestação técnica de isolamento e sem selo. Revisor somente leu produto,
testes e contratos julgados.

Recibo `p1325-review-candidate-audit.json`, SHA-256
`823c4f997374ddffee7abdb32fcded29f1505426c5491eae17cdc5f7a4c258a5`,
em `2026-09-09T01:02:47.754Z`, registra HEAD/diffstat, hashes, protegidos e
linhagem. Fonte candidata SHA-256
`2d32b6e5ab691b19dde2213fd8df32cc9e6cfc1adae4477fe96d7cec976cda6a`.

Verificação independente removeu apenas a linha @prompt-hash e o sufixo
literal do snippet R1. O restante difere de baseline.original_owner apenas
pela seleção local de span: Module, Dict, Content e Float usam field.span;
o guard nativo existente permanece. O corpo de lookup, warnings, gates,
pré-despacho, callee, outros variants e assinaturas são idênticos. Não há
alteração em LocatedContent, nem em lookup de `[x].text`.

Teste R1 continua literal e todas as entradas produtivas alheias ao owner e
seu L0 permanecem idênticas. O hash normativo L0 permanece congelado. Hash
canônico independente do source, excluindo somente a linha de header:
`48f648ca2ed2c4ea08fc930a45be25ad600f60cb2028dbc8fef666962d7db749`.
Assim, Hash do Código esperado é `48f648ca`; ainda não estava aplicado no
instante da auditoria. Hash efetivo `6f7aad92` e pin conferem.

## Retificação do metadado temporal do freeze

O recibo de revisão prépatch capturou o freeze provisório de SHA-256
`b5a2e3b5697243780734caf58c38ad78e5f18b5b8fb1fa42c1342c3ecc8f0853`,
com `at=2026-09-09T01:01:56+00:00`. O autor explicou que digitou esse horário
incorretamente e o corrigiu para a saída observada de `date`,
`2026-09-09T01:01:40+00:00`. O freeze final anunciado é
`5efd24f3a37df5a42f6746e954922b04dacbec958b2877a025e812790bc5af01`.

A comparação direta dos objetos confirma que apenas `at` mudou: norma,
expectativas, casos, testes, comparador, binários, protegidos e orçamento são
os mesmos. Esta retificação preserva o erro temporal como limitação da
proveniência; não representa revisão do contrato nem relaxamento posterior
ao candidato. O root esclareceu posteriormente que leu o envelope provisório
antes de C, e recebeu o anúncio do hash final durante ou depois do tool de
aplicação de C/início GREEN. Logo não há prova de leitura do envelope final
exato pelo implementador antes de C; a afirmação inicial nesse sentido foi
retificada. A evidência prépatch sustenta congelamento das entradas
normativas e oráculos idênticos antes de C, não a ordem exata do envelope
temporal retificado. Não se reivindica selo/atestação que essa ordem exigiria.

Permanecem necessários GREEN de todos os testes do owner, corpus A/B final,
validação integral de linhagem e gates arquiteturais. Esta revisão do diff
não antecipa seus resultados.
