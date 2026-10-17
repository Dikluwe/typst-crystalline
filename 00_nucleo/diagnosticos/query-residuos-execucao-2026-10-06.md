# Três resíduos — execução e gate pendente

O pedido autoriza corrigir os três resíduos da coleta anterior. Uma correção
foi materializada; as outras duas exigem decisão concreta de pipeline e API.

## Entrada inexistente corrigida

L0s query/tests foram atualizados antes de código; missing-red.log registra
exit 2 inesperado. O matcher tipado de SystemWorldError::MainNotFound agora
emite input file not found (searched at INPUT) e exit 1, sem depreciação.
Outros comandos/classes de I/O mantêm o comportamento. Missing-green.log passa
nas formas relativa, ./ e absoluta; warnings-regression.log passa nos três
controles de ordenação/hint/Source. Não houve API pública ou fase nova.
Ownership/núcleos passaram antes de resselo; seal.py/seals.json modificam só
wiring/query e wiring/tests/cli. Build-debug.log/build-release.log e diff-check
passaram. Linter final.sarif: zero erros, 528 warnings,
1579 notes; avisos herdados fora do escopo permanecem.

## Medição e proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitada.
Coleta UTC 2026-10-06T18:06:44.856394+00:00 até 2026-10-06T18:06:51.953116+00:00.
After-missing.json.gz guarda comandos/fontes/cwd/ambiente, diff HEAD --stat,
status, hashes das fontes e stdout/stderr/status. Binário Tekt
d6b82174a611dddd171e17f5e28ac8c586469a34bc203b94487953f2d40b5f76; vanilla
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489.
Os três controles de arquivo inexistente coincidem integralmente; summary.json
registra os demais negativos, sem nova certificação global de paridade.
Baseline-state/delivered-state conservam as árvores; preservation.json verifica
trabalho anterior fora do escopo. As rodadas usam os estados não commitados
registrados; os dois hashes de código diferem conforme o reparo documentado.

## Callback e selector — proposta concreta

Query CLI usa query_elements, que parseia antes de avaliar e consulta Module
sem execução contextual completa. Before.json.gz mede callback e contexto
aninhado silenciados, metadata contextual ausente, .where válido rejeitado,
erro de selector precedendo erro documental e depreciação indevida em retrieval.
Eval --in já obtém resultado correto para callback/where-level através do
pipeline paginado existente. Upstream query.rs:34–66 compila antes de retrieve;
:86–105 avalia selector em scope stdlib independente e converte LocatableSelector.

O pacote 00_nucleo/materialization/query-compilada-2026-10-06.md contém cinco
L0s completos, owners 1:1 e hashes base/candidato. Propõe compile_query_context,
query_compiled_elements e CompiledQueryError, usando compilação paginada atual
sem exportação ou segunda execução global. Mantém APIs legadas para consumers
atuais; evita parsing ad hoc, classificação por string/span e novo fixpoint.

A ADR-0127 §2 exige parar para assinatura pública e mudança arquitetural de
fase/ordem. O pacote de warnings anteriormente confirmado não apresentava
essas APIs ou mudança de caminho. Não se presume ratificação a partir do pedido
geral de correção. Nenhum código destes dois resíduos foi escrito; os L0s
propostos ficam em materialization, com manifesto validado, até confirmação.
Após gate: aplicar L0, RED, materializar, ressellar owners, build/lint e repetir
os controles originais/positivos/de precedência. O plano permanece em curso.

## Continuação ratificada

O gate query-compilada foi confirmado e materializado; os três resíduos
originais agora coincidem nos controles. Relatório atual:
query-compilada-execucao-2026-10-06.md. As menções a gate pendente acima
são registro histórico da etapa anterior.
