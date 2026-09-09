# P1322 — veredito independente D

**Auditoria preservada com limites declarados. Recomendação única: `html-experimental-warning-hints`, prioridade 2, um owner, um path causal.** Não é paridade global nem autorização para implementar P1323. O recibo `p1322-review-final.json` fixa UTC, hashes de entradas e do relatório final do operador.

D revisou inventário, execução bilateral, classificação, seleção e gates sem escrever produto/L0 nem os dados julgados. A skill `tekt-materializacao-segregada` determinou plano de ataques anterior ao checker, papéis separados, registros de calibração e bloqueio da seleção quando a revisão de substância refutou a prioridade proposta. Ferramentas/filesystem são compartilhados: **executado sem atestação técnica de isolamento**. Esta é segregação de autoria e revisão documentada, não certificação técnica de capacidades.

## Resultado e evidência

O catálogo tem 4.718 probes: os 2.182 históricos preservados literalmente e 2.536 modificadores Symbol novos. Cada uma das três ordens mede 18.872 células principais, com 18.220 iguais (96,5451%), zero EXECUTION_UNKNOWN e nenhuma regressão real de MATCH histórico. A igualdade ajustada é 18.220/18.716 (97,3499%), excluindo somente 39 extensões documentadas. São proporções do inventário lookup/kind/repr/diagnóstico, não de toda a linguagem. Presença não fecha funcionalidade.

O suplemento tem 816 casos/3.186 células por ordem e fica fora desse denominador. As três ordens são estáveis; os fechamentos recentes e seus limites foram reconciliados com fontes atuais. Ausência bilateral de csv.encode/read.encode/xml.encode não é ausência unilateral de encoder. Unknown de owners/custo futuros não foi convertido em baixo risco.

Recibos canônicos D: `p1322-review-runtime-final.json` (1.979.138 checks), `p1322-review-supplement.json` (230.102), `p1322-review-semantic-final.json` (114.198, universo anterior à adição dos canais), `p1322-review-transversal-r2.json` (9.702), `p1322-review-gates.json` (7.860), `p1322-review-preservation-final.json` (8.096), mais `p1322-review-channels.json` e `p1322-review-selection-r2.json`. Contagens de checks são operações do auditor, não testes de produto ou features.

Os MATCH transversais correspondem somente à projeção declarada. A conferência adicional reconstrói literalmente 222 linhas executadas, incluindo 51 MATCH projetados com stdout/stderr diferente — 17 por ordem, zero instabilidade. Hints de query/HTML continuam divergentes apesar de JSON/DOM igual. Help é fechado apenas estruturalmente, mantendo metadados de descoberta não resolvidos. A política atual de path externo é documentada, sem crédito de igualdade do diagnóstico bruto; hash próprio de build é proveniência mecânica. PDF/raster simples não certificam layout/export global; D não realizou inspeção visual nova.

A rejeição íntegra de query `--features` é capacidade CLI ausente, não execução/igualdade do JSON cristalino. O predicado focal exige o comando e transcript completo observado. Os controles rejeitam stderr ausente/truncado, erro genérico, crash e comando diferente como Unknown; exit 2 não foi globalmente convertido em sucesso ou diferença conhecida.

## Ataques e controles

**13/13 ataques únicos válidos rejeitados, zero survivors.** São os D01–D13 do plano congelado, em cópias de dados: dois de catálogo, cinco de runtime e seis semânticos. Catálogo reordenado passa; Unknown honesto permanece Unknown. D06 usa opacidade simulada e D09 constrói uma regressão em cópia de uma célula histórica genuína: não relatam timeout/regressão observados no produto. As transformações, controles e hashes constam nos recibos `p1322-review-attacks-{catalog,runtime,semantic-r1}.json`.

D12 foi repetido no universo causal sucessor: HTML é o controle válido; selecionar novamente namespace Some é o mutante de prioridade e foi rejeitado. Essa repetição e controles extras dos canais não aumentam 13/13. Nenhum mutante produtivo foi executado; as 37 famílias históricas de certificação não foram quitadas.

## Decisão causal nova

O primeiro anexo C classificava os hints HTML como prioridade 3 e mantinha seis funções com namespace como vencedoras. D rejeitou essa conclusão: `wiring.md:172–173` promete o warning experimental **medido no vanilla**, cujo objeto em `lab/typst-original/crates/typst/src/lib.rs:249–256` contém três hints. O consumidor `04_wiring/src/main.rs:388–392` emite somente a headline. Não é necessário que L0 transcreva todas as strings para que essa promessa específica seja vinculante. É contradição L0 em rota canônica, prioridade 2.

O anexo e a seleção R2 preservam os originais e adotam rank `[2,1,-1,1,"html-experimental-warning-hints"]`, antes de `[3,1,-6,1,"namespace-function-missing-field"]`. D leu integralmente wiring.md, shell/diagnostic.md e núcleo pertinente, verificou as fontes bilaterais e conferiu as 128 coortes/17 elegíveis. A suficiência de um owner limita-se a completar a emissão fixa já atribuída a L4, sem algoritmo de formatter, dados novos, tipo/API pública, alteração de target/default/feature ou pipeline. Duas grafias e repetições não multiplicam o path causal.

P1323 deverá explicitar o envelope no L0 antes de código, fazer RED→GREEN e revalidar presença/ordem dos três hints, headline única, ausência quando desligado e preservação dos artefatos/modos/outros formatos. Necessidade de formatter/API/owner adicional refuta esse recorte e reabre o gate. O hint dinâmico de query não foi fundido com HTML para fabricar uma coorte maior.

## Preservação e limitações do processo

HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093 com working tree não commitado foi preservado: 3.912 arquivos de produto/L0/lab, 126 evidências iniciais, seis inputs históricos, passo autorizado, HEAD/branch/diff/staged. Build locked em target exclusivo e identidades SHA completas não dependem de strings de versão. Workspace: 6.666 passaram, zero falharam, três ignorados; lint: zero erros, 240 warnings e 1.138 infos explícitos; fmt, linhagem V5/V15/V26 e diff-check passaram.

Incidentes preservados/retificados: expansão artificial inicial do inventário (invalidada antes de consumo); perfis R1 transversais sem flags reais (306 violações, R2 executado); hipótese de escape refutada; calibrações do leitor D (serialização de testemunha e TSV/display/QUOTE_NONE); primeira emissão C que misturou 49 suplementares na contagem histórica e foi substituída antes do freeze, sem arquivo integral recuperável. Esta última falha limita a trilha do primeiro rascunho: não é ocultada por aprovação atual. Os nove inputs finais C foram congelados e checks/ataques focais repetidos depois; seus hashes e os demais inputs canônicos continuam íntegros.

A descoberta tardia dos canais e a objeção que efetivamente mudou o vencedor permanecem em `p1322-review-channels-objection.md`, nos anexos C R1/R2 e na seleção sucessora. A aprovação final alcança somente o relatório com essas retificações e limites. Nenhum stage/commit/push, limpeza de temporários ou alteração de produto/L0 foi realizado por D.
