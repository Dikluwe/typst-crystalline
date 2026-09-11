# P1339 — definições finais do lote 1, auditoria anterior ao manifesto

Verificador `/root/p1311_review`, 2026-09-10 04:44:14 UTC. Executado sem
atestação de isolamento. Somente leitura/auditoria estática e este recibo;
nenhuma execução semântica, mock, matriz, candidato, RED ou selo. HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado;
proveniência do estado de preparação e diff está no recibo
`p1339-verifier-batch1-successor-audit-r1.json`. As contagens abaixo são
extraídas dos artefatos pinados, não resultados de execução do produto.

## Entradas canônicas

Todos os nomes têm prefixo `00_nucleo/diagnosticos/`.

- `p1339-ab-batch1-retention-lifecycle-typed-r3.json`:
  `6d8c257997631133fb1a7a55816daaa50e1633c129fafd17f6d1ddeb378ca541`.
- `p1339-ab-batch1-coverage-map-r2.json`:
  `ad95dd42ce102810a75986511c67a996286ec78a486c11e87faf50cd54aa51ea`.
- `p1339-ab-batch1-lifecycle-predicate-v3.py`:
  `715adff3a42c6be12aaedf582c4c7846d30c226c7be45afbfaa432889dfbf60f`.
- `p1339-positive-oracles.json`:
  `482babb923e306357f1aa70490e4fef8a5a16f772a494819d50feaee6f2c6972`.
- `p1339-opaque-oracles.json`:
  `4f33c33353caee89f509fe81ed76a2f631510684645f1c9f6b82341587472ed0`.
- `p1339-ab-batch1-public-predicate-v2.py`:
  `5a0802c79948420744ebcbb972679d6ab9d2a039f06bb8b0fd83d5560e9abb3e`.
- Seu predecessor `p1339-ab-batch1-public-predicate.py`:
  `9fa26cecc4d6bf266f72016a619b7f5b283cec801f5a61bea6028d943bdb5294`.
- `p1339-mutant-closed-state-final-runner-r2.py`:
  `7b78be73f5280ee5eb519f97aa8fc9fda28de178c0df5fef4eae586fe7a3f645`.
- `p1339-mutant-closed-state-batch1-integration.md` ancestral:
  `2d7aa4f21432f810808ce23ab605d5e31f6ce16f9dc4115a6de66d3a7b3090e1`.

Conferi 330 pares distintos de referência path/hash alcançados dos dois
wrappers, typed-r3 e coverage-map-r2, sem divergência. Essa conferência não
concede atestação de isolamento ou aceite semântico às referências.

## Pendência das 50 proposições: definição fechada

O mapa lifecycle agora coincide entre typed-r3 e coverage-map-r2 e liga as
50 proposições a 117 referências exatas de assertions, todas conferidas
contra hash e linha, mais testemunhas específicas. Li o checker-v3 e os
predicados predecessores conjuntivos. Capturas e parent generations distintos,
cadeias de entrada distintas, ordem invalidação→novo corpo, leituras estáveis,
bijeção da publicação e igualdade do diagnóstico final completo são exigidos
explicitamente. Isso fecha as lacunas anteriores de definição.

As 14 SW-LC especificam def-use e vínculos por caso/perfil/ordem: captura
zero/replacement, tamanhos das cadeias, completude das origens de invocação,
linhagem dos diagnósticos finais, produtor ordinário aninhado, retenção de
callback, ordem Step/Func/Step, NaN na relação privada versus chave pública,
erro provisório/final e seleção antes da falha de label. São receitas
refutáveis de prova sobre o candidato, não afirmações genéricas de cobertura.
Todas continuam obrigatórias em F com eventos e caminhos reais. Inventário,
metadata final e IDs autodeclarados não substituem esses vínculos.

## Wrappers e comparação pública

Os wrappers particionam exatamente os 724 IDs do plano: 719 definições
públicas positivas/erros esperados, um controle opaco assimétrico e quatro
Angle condicionais sem crédito positivo. Internos: 117 não opacos e dois
opacos, todos devidos apenas em F. Os três direct-show controls conservam
baseline; text mantém OtherDiagnosticBeforeCallback no candidato e callback
executado no vanilla. Não retornam ao alvo vanilla indevido da revisão antiga.

O predicado público exige matriz exata, origem/binário/source hash, modo,
cwd e ambiente de diagnóstico; verifica referências antes de selecioná-las,
compara dimensões integrais e conjuga efeito causal de show. A composição
empty-versus-bare continua PENDING_MEASUREMENT: receita e controle foram
fixados antes do raw, com substituição única e fail-closed. Raw e expected
derivado válidos ainda são necessários antes do selo. Não existe licença
para ajustar a receita após resultado inesperado.

O opaco mantém timeout Unknown somente no vanilla e erro conhecido exato
no baseline/candidato, com zero crédito positivo. As quatro Angle mantêm
raw separado de aplicabilidade condicional. Um Unknown obrigatório comum
recebe mandatory_unknown, não sucesso. O programa de comparação imprime
classificações; exit0 sozinho NÃO significa aprovação. O verificador deve
julgar todas as linhas, diferenças e mandatory_unknown, incluindo os controles
negativos designados, sem depender do status do processo comparador.

## Driver final e ligação entre crates

O runner-r2 substitui efetivamente typed-r2/checker-v2 por typed-r3/checker-v3
e pina os ancestrais. Confirmei igualdade integral dos objetos `cases` e
`dto_schema` entre typed-r2 e r3; por isso o collector antigo continua sendo
tradução exata dos inputs, enquanto o checker final é o sucessor. Não há
troca silenciosa de assertions. Seis conjuntos exatos de linhas e metadados
do binário real são exigidos; raw, sinal/timeout e streams são preservados.
O runner chama os predicados Python independentes, não somente exit dos
coletores. Seu PASS_CLOSED_TESTS_ONLY não fecha provas estruturais nem CLI.

A proposta de custom cfg exclusivamente de testes aplicado também à
dependência é admissível como concretização mecânica de candidate_test_binding,
não como quarta API produtiva. Registrar configuração/nome/build isolado
antes do selo; manter ausente no build normal e sem feature/flag/default de
produto. Telemetria só passiva, owner-local, sem global/registry/I/O em L1,
sem algoritmo paralelo e sem acesso a expected. R3 permite late binding de
nomes/layout, mas exige auditoria dos corpos, grafo e configurações ordinary
versus instrumented. A menção cfg(test) não permite instrumentação produtiva
permanente. O suplemento concreto e seus pins ainda serão julgados; nenhuma
implementação desse binding foi autorizada antecipadamente por este recibo.

Conclusão: definições e driver sucessor aceitos no recorte estático. Prontos
para inclusão no manifesto agregado, ainda NÃO autorização dos 384 processos
focais. A skill mantém parada até a revisão do manifesto: contrato R3 intacto,
budget explícito do mesmo lote, full counter 0/2, sem selo ou candidato antes
de discriminação real e demais gates.
