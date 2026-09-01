# P1289 — recibo de contrato A/B congelado

**Estado:** `CONTRACT_FROZEN_BEFORE_TESTS_AND_CANDIDATE`  
**Contrato:** `C-P1289-FLOAT-IS-INFINITE-v3`  
**Manifesto:** `00_nucleo/diagnosticos/p1289-manifest.json`  
**SHA-256 do manifesto:** `1bd7c90a45f9cc2a2301eaab89405915d5dfc7d74627508325ca0124c50167ca`  
**Congelado em:** `2026-08-31T11:40:34-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`

## Regime e autoridade

O regime é Tekt A/B: o risco material é vazamento entre solução e testes;
não há contrato de refinamento geral a selar. `/root` atuou como Autor de
Intenção/Contrato e escreveu somente manifesto, recibos de medição/contrato e
L0. Não escreveu implementação candidata, testes protegidos ou veredito.

O Testador A recebe somente o manifesto, os L0s, o recibo de medição e o
baseline binário pinado. O Implementador B só começa depois dos hashes dos
testes/oráculos e do RED; não pode editá-los. O Verificador é terceira
autoridade e só pode escrever o recibo final. O checkout compartilhado impede
atestação de isolamento ambiental forte; a segregação verificável usa ordem,
allowlists e hashes.

## Contrato congelado

- superfície estática `function` com repr `"is-infinite"`;
- `false,false,true,true,false` para finito, finito, `+inf`, `-inf`, `nan`;
- forma ligada chamada equivalente, sem expor o método ligado como valor;
- coerção estática de inteiro;
- diagnósticos fechados para missing, extra, named e tipo errado;
- lookup fechado e fórmula somente no owner `foundations/float`;
- `Unknown` nunca é sucesso.

As cinco mutações obrigatórias são: sempre falso, `nan` infinito, aridade/named
permissivos, repr qualificado e presença sem chamada. O contrato só pode ser
considerado discriminatório com score `1.0` e testemunha para cada mutação.

## Gate causal

O manifesto e todos os L0s listados nele estão congelados antes de Testador A
ou Implementador B. Qualquer mudança de bytes invalida os testes, o selo A/B e
o veredito derivados. Resultado proporcional: contrato segregado por papel,
capacidade, ordem e artefatos, sem isolamento forte do host.

## Refação v2 após pré-condição inválida

O selo v1 (`0bfc2e5d…`) foi invalidado quando o primeiro GREEN parcial revelou
que `float.inf` e `float.nan` eram resíduos independentes, e não pré-condições
disponíveis. O contrato v2 substitui somente esses carriers por
`float("1e999")` e `float("NaN")`, medidos bilateralmente. Resultados,
diagnósticos, política de `Unknown`, mutações e allowlists permanecem iguais.

O candidato v1 já existia no checkout compartilhado quando esta refação foi
necessária. O Testador A v2 continua proibido de ler os consumers e deve
regenerar e congelar seus artefatos apenas a partir deste contrato/L0 v2. Isso
preserva segregação por capacidade e artefatos, mas não cria isolamento forte
retroativo do ambiente.

## Refação v3 por ownership do teste

O verificador v2 encontrou V1 no teste de integração protegido. O selo v2 foi
invalidado e o contrato v3 acrescenta o owner individualizado
`00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md`, sem alterar nenhum
observável ou mutação. Testador A deve adicionar/resselar somente o header do
consumer L4 e repetir RED/oráculos; B deve apenas consumir o novo selo e o
Verificador deve reexecutar V1/V5/V15/V26 e gates focais.
