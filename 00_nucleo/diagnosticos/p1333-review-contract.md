# P1333 — revisão independente do contrato delimitado

Revisor `/root/p1333_review`, regime A/B sem atestação técnica de isolamento
ou selo de refinamento. Parecer sobre L0 e baseline públicos, sem executar
produto, escrever testes ou modificar os artefatos julgados.

Entradas: manifesto `p1333-manifest.json`, SHA-256 conferido
`75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b`;
baseline público `p1333-baseline-public.json`, SHA-256 conferido
`45a23be20fd81ff9b77c60ff89db3267ce615a77d3fb41df444f3af054bf8800`.
O manifesto declara hash normativo do L0
`517e1544fbd419a5b1c13e524c02570d3c22606a7b97d25562723c278bdd2e83`.
A proveniência da medição está no baseline: HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, árvore não commitada,
diff/stat, inventário, binários e UTC por comando. Este parecer não produz
novas medições funcionais.

O recorte revisto resolve a objeção de escopo anterior. Em
`00_nucleo/prompts/compiler/stdlib/calc.md:1023-1038`, somente falhas do
primeiro posicional passam a preceder named/aridade; primeiro valor válido
e ausência preservam literalmente os guards existentes. Logo, nenhuma
obrigação nova precisa da âncora agregada da chamada. A origem relevante
continua sendo o value-span individual já transportado por Args, inclusive
em With e arguments; ausência/detached continua detached. O impedimento
anterior permanece válido para fechar toda a validação, mas esse objetivo
está explicitamente incompleto e atribuído a P1334 em `:1042-1046`.

A medição sustenta a distinção: content, string, boolean, none, Length misto
e overflow com sobras divergem pela precedência; argumentos válidos com
sobras e chamadas vazias exibem dívidas separadas. O caso eager
`calc.abs(false,bad:panic("later"))` preserva a falha anterior à nativa.
With/arguments medidos confirmam que origem externa pode conservar trace
da chamada final. A igualdade com a chamada unitária em `:1026` deve ser
entendida como diagnóstico nativo sob as mesmas origens, acrescido dos
traces externos previstos, nunca como igualdade de spans de expressões
textualmente distintas.

A fonte ratificada anteriormente auditada confirma que ToAbs calcula
overflow/misto durante o cast, antes de finish. É semântica observável;
não há exigência de reproduzir o cursor Rust. O L0 usa valores de
`Args.items` e metadados apenas para origem; pressupõe a coerência obrigatória
do carrier vigente, sem legitimar Args deliberadamente inconsistente.

Veredito de contrato: apto para congelamento de testes independentes e RED.
É correção local de paridade em fluxo contínuo ADR-0127, sem nova assinatura,
campo público, fase ou modo de produto; não identifico gate humano adicional
para este recorte. A aptidão não atesta materialização, RED/GREEN ou gates
finais. Estes permanecem obrigatórios, com migrações históricas limitadas,
controles de primeiro válido/ausência, dados protegidos íntegros e Unknown
bloqueante. Não autoriza alterar dispatcher nem declarar guards completos.
