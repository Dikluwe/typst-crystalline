# P1349 — autoria segregada do adapter operacional R1

Data: 2026-09-11T14:51:58-03:00

Papel: autor exclusivo do adapter P1349. Regime: `executado sem atestacao de isolamento`.

## Resultado

Foi escrito `p1349-verifier-adapter-r1.py` com as APIs públicas separadas `exercise_meta(case, targets)` e `exercise_focal(case, targets, exact_ids)`. X06 e X15 entram somente em `exercise_meta`, usam os executores descartáveis de `checker.exercise_meta` e retornam exclusivamente `p1349-meta-result-r1` quando a observação planejada e todas as pós-condições são confirmadas. Nenhum campo de classificação ou razão é emitido pelo domínio meta.

X14 e as onze variantes P1349 são roteados exclusivamente por `checker.supervise("classification", ...)`. Um resultado normal é transportado apenas se o checker devolver o schema de classificação fechado. Timeout, sinal externo, exceção, framing incompleto, processo órfão ou limpeza incompleta permanecem `p1349-integration-blocker-r1`, sem `reason_code` e sem crédito semântico.

Os 22 focais P1348 restantes, 39 regressões P1347, 181 replays P1346 e 11 controles delegam suas operações congeladas ao adapter P1348 integralmente pinado e transportam somente a classificação/razão efetivamente observada. X06/X15 não existem na tabela focal.

Status: `AUTHORED_WITH_FROZEN_API_INCOMPATIBILITY_NOT_EXECUTED`. Isto não é veredito, pré-selo ou alegação de discriminação.

## Incompatibilidade congelada encontrada

A superfície `_execute` do checker P1349 R1 reconhece somente as operações classificáveis `canonical`, `missing_traceexit`, `second_exec_external` e `raise_exception`. Portanto:

- X14 possui mapeamento fiel `inject_second_exec -> second_exec_external` e alcança o supervisor/checker congelado;
- nenhuma das onze operações S01–S11 possui implementação correspondente no checker congelado;
- o adapter envia cada nome S01–S11 explicitamente ao supervisor, que produzirá blocker terminal pela operação ausente;
- o adapter não substitui essa ausência por `Violated`, não copia `expected_reason_code` e não fabrica evento ptrace, deadline, PID/pidfd, DAG ou cleanup.

Essa incompatibilidade impede alegar que as onze variantes podem produzir as classificações esperadas pelas APIs disponíveis. Corrigi-la exigiria nova autoridade sobre o oráculo ou contrato; este autor não a assumiu.

Há ainda uma divergência nominal congelada: a suíte passa a `exercise_focal` os 122 IDs canônicos de linguagem em `exact_ids`, enquanto o texto contratual descreve o `case_id` focal como membro de `exact_ids`. O adapter preserva o dado realmente fornecido pela suíte: exige exatamente 122 strings ASCII distintas com digest `e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907` e mantém a identidade adversarial separada. Não acrescenta IDs focais à lista recebida.

## Cobertura estática

| domínio/fatia | casos | operações distintas cobertas |
|---|---:|---:|
| meta X06/X15 | 2 | 2 |
| G05 X14 + variantes P1349 | 12 | 12 |
| G06 outros focais P1348 | 22 | herdadas na tabela P1348 |
| G07 regressões P1347 | 39 | herdadas na tabela P1348 |
| G08 replays P1346 | 181 | `replay_p1346_negative` |
| controles P1347 | 11 | herdadas na tabela P1348 |

O conjunto focal congelado contém 60 nomes operacionais distintos e o adapter cobre 60/60, com ausentes 0 e excedentes 0. Somados aos dois nomes meta disjuntos, são 62 nomes. O denominador focal congelado permanece 254 negativos válidos; os 11 controles e dois meta-testes ficam fora do score.

## Fronteiras de segurança

- O adapter não lê `expected_classification`, `expected_reason_code`, `expected_meta_status` ou `expected_meta_evidence`.
- `exercise_focal` rejeita X06/X15 antes de qualquer operação e valida o pin dos 122 IDs canônicos.
- Exceções de schema, import, setup, delegação ou tradução retornam blocker fechado `INTEGRATION_BLOCKED`, nunca classificação.
- Blocker devolvido pelo supervisor P1349 é propagado integralmente. Blocker do adapter P1348 vira blocker P1349, não razão de mutante.
- Os roots temporários operacionais pertencem aos checkers congelados e permanecem sob `/dev/shm`.

## Entradas pinadas

- passos P1349/P1348/P1347: `071ee50547bab6dc86b84d630b6c501867967121234cce79ab8de14075973a81`, `4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7`, `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c`;
- contrato P1349 spec/binding/receipt: `9b3363fe2c60d634a7547357217299a2fc610e04f3c0116b667ce23caf555335`, `3db25f33a5af2716b432788cd06e8ec527567584b803def215bd45cad06e29bd`, `6baf1c42baaea917035e0d54729dba9430c5799c5cd61212f5f31dbce33b6ef6`;
- adversário P1349 suite/authorship/receipt: `9f7320db878b16f7be0436b364b34a2f3fdc7ea6a8b34cfb50da0a38930a1703`, `ea364c7194fa2c3f9b15da2b53cac261cb4576e925713a2329e3d7b7484818b9`, `ec08d9406e3fbd993d750236e454b6d3f7210c11a1eddc31b1770f558af7b6a2`;
- oráculo P1349 manifest/corpus/checker/caller: `1d2b30fd83d78d1fada381f82062309cbe5eaba2a4d32665fa59872dd9c3c907`, `045af76f9540107bb633c8d2c904e726f96624ecf8e9a91d706b99f530c56803`, `f8e0311992e0e4973825a9b0ddbfaaea27e959f34bbcc4406ba5434ac1974843`, `747da52dc06b9ccf81242f059fc7eb599133de7bd25ab7800eaac89e86dc2579`;
- oráculo P1349 authorship/receipt/delivery: `c2a6de2bb138b3fca422a859c6f9dd9d8cadf4ffd19be914e70f5640fa8f9ba0`, `d72fc7bf73c7432ef2b64ebf6dbca6b22f5f452dfdf98da7018d2c86ff776feb`, `81fa483c5a8c2ccdb0a05a720378765438a77ab0bd18f3d88b8227ce0cb71c0b`;
- blocker/adapter/autoria/receipt P1348: `e5b9e9423e2e3ad20064cbc20f2594e25396f7e1bffe20123a4f8392cb8501de`, `bcec1c05eed907ec3ea8f45c55940ee5244fe9498c731e66458b9817d946edfa`, `da35f9cf12b42c37885a5545c96e93bb6e9143f1c30a79fd853e9e4c31759c8d`, `782cbf5afef9250292ddbe47b808446ec412618b6e37ababf6c5c596478f8fee`;
- blocker/adapter P1347: `3b649ced677059a459262710ce3466cb2efbd69edbfb6e60c98df07cffa64522`, `cc71d74b85b14e6624aae53630f25a3c5fe34724cab7d62222f84377d6845a7a`.

## Validação permitida e proveniência

Somente `compile(source, path, "exec")`, extração estática das tabelas de operação, hashes de ficheiros e diff-check foram usados. Não foram chamados meta, focal, `run_with_adapter`, `checker.supervise`, ptrace, full ou candidato.

Sintaxe: PASS. Cobertura: PASS para os nomes congelados. Pins: PASS. Diff-check: PASS. Compatibilidade operacional S01–S11: BLOCKED pela ausência descrita acima.

Commit observado: `496c45ac6279d4298079f848b24b5e97ba88edc1`. Working tree não commitado e compartilhado. Antes deste relatório, o único output P1349 deste papel era `00_nucleo/diagnosticos/p1349-verifier-adapter-r1.py`. Hora: `2026-09-11T14:51:58-03:00`.
