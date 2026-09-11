# P1349 — autoria segregada do adapter operacional R2

Data: 2026-09-11T15:18:41-03:00

Papel: autor exclusivo do adapter P1349 R2. Regime: `executado sem atestacao de isolamento`.

## Delta causal

O adapter R1 foi preservado integralmente. O novo `p1349-verifier-adapter-r2.py` mantém sem alteração as APIs públicas, os dois mapeamentos meta, as 60 operações focais, os 254 negativos válidos, os 11 controles e a delegação das 48 operações herdadas.

A única mudança de integração é a raiz Oracle R2 explicitamente congelada:

- checker R2: `999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267`;
- caller R2: `4445f1a23a66ad2a0c19a67769cfb043541106bb81ded6e1a1e263d0da974c07`;
- delivery R2 externo: `219f6c23c2ba95592536fb5ab174a0d8f5243c76fab35a4057042f63234a4524`;
- delivery root R2: `5c2b5e15959bfd23f9627d3391e66c0e46c607a962ad0bfc3315770c3c5f4b81`.

O adapter recusa qualquer target cujo path resolvido ou SHA-256 não seja exatamente esse pacote. Também valida os edges checker/caller/receipt no delivery, chama `caller.validate_chain()` somente quando exercido futuramente, confirma o checker carregado e recomputa o delivery root pelo checker R2.

## Rotas preservadas e suporte R2

- `exercise_meta(case, targets)` continua isolando X06/X15 e usa a autoridade `p1349-oracle-r2`.
- `exercise_focal(case, targets, exact_ids)` continua validando os 122 IDs ASCII distintos e seu digest canônico, mantendo X06/X15 fora do domínio focal.
- As 48 operações herdadas continuam delegadas pelo adapter R1 pinado, com seus schemas de classificação/blocker revalidados pelo checker R2.
- X14 mantém `inject_second_exec -> second_exec_external`.
- As 11 operações S01–S11 mantêm exatamente os mesmos nomes e mapeamentos R1 e agora coincidem, uma a uma, com a tabela fechada `OPERATION_REASONS` do checker R2.
- X14 e S01–S11 passam por `checker.supervise("classification", ...)`; um timeout, sinal externo, exceção ou cleanup incompleto continua blocker terminal sem razão.

Cobertura operacional estática R2: meta 2/2; focal 60/60; supervisor 12/12, sendo X14 mais 11 variantes; herdadas 48/48. Ausentes 0, excedentes 0. A decomposição congelada permanece 12 G05 + 22 P1348 + 39 P1347 + 181 P1346 = 254 negativos, além de 11 controles.

## Fail-closed

O adapter não lê expectativas adversariais. Resultados das 12 operações supervisoras vêm diretamente do checker R2. Blockers do supervisor ou do adapter R1 são preservados como blockers; exceções locais de schema, target, import, setup ou tradução produzem `p1349-integration-blocker-r1/INTEGRATION_BLOCKED`, sem `classification` e sem `reason_code`.

Os dois meta resultados continuam fechados e separados da classificação. A tradução meta usa somente observações e pós-condições retornadas pelo checker; não entra no score.

## Entradas públicas consumidas

- adapter/autoria/receipt R1: `db800da93aedf7c34ec3b0c4018661b35d61d2edbe709c3cfb48e8a108caa861`, `51360d9843da7165b02461d5498f310211316cf3dc65c5b3ed3ba6a8b431232e`, `43ec959d289e18382e6cd05d53e053f52be93b80c0371477a78b487e98e9d49d`;
- blocker público P1349 R1: `ebbeedae4cc9c1fff6462abe065fca58aa402c8b67d04d90b3bb48cbaf37b3c1`;
- checker/caller R2: `999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267`, `4445f1a23a66ad2a0c19a67769cfb043541106bb81ded6e1a1e263d0da974c07`;
- autoria/receipt/delivery Oracle R2: `66fbda44ee8b94085c389770f699fc100461f141a5dadbdd3ed0574a982230f9`, `d95113d55015300108e5571fe29d6b83b17a34bc37aa7dfa01be04e934fb6ab8`, `219f6c23c2ba95592536fb5ab174a0d8f5243c76fab35a4057042f63234a4524`.

Nenhum artefato adversarial foi lido nesta revisão.

## Validação permitida

- sintaxe por `compile(source, path, "exec")`: PASS;
- igualdade AST das três tabelas de mapeamento R1/R2: PASS;
- cobertura da tabela R2: 12/12 operações supervisoras suportadas;
- hashes e edges públicos R2: PASS;
- diff-check incluindo os outputs não rastreados: PASS.

Não foram chamados `exercise_meta`, `exercise_focal`, `checker.supervise`, focal, `run_with_adapter`, ptrace, full ou candidato. Nenhum score, meta PASS, classificação ou veredito foi medido.

Proveniência: commit `496c45ac6279d4298079f848b24b5e97ba88edc1`; working tree não commitado e compartilhado; hora `2026-09-11T15:18:41-03:00`. Status: `AUTHORED_NOT_EXECUTED`.
