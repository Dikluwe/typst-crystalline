# Query compilada — três resíduos corrigidos

O humano confirmou o pacote query-compilada-2026-10-06.md em 2026-10-06,
resposta confirmo. A aprovação conserva os hashes base/candidato e o manifesto
original em materialization/query-compilada-aprovacao-2026-10-06.json. Os cinco
L0s completos foram aplicados antes dos testes e código. A fachada pública e o
caminho de query compilada estão ratificados; APIs legadas continuam existentes.

## Medição e decisão

query-residuos-2026-10-06/before.json.gz mede callback contextual silenciado,
metadata gerada ausente, .where válido rejeitado e precedência de erro invertida.
O upstream query.rs:34–66 compila antes de retrieve; :86–105 avalia selector em
Scope default separado do documento. Eval --in já obtinha resultado correto
para callback e .where(level), com a compilação paginada existente.

A correção usa esse pipeline: compile_query_context em pipeline.rs:156 recebe o
introspector final e warnings, descarta o documento sem exportar. Não há segunda
avaliação global ou novo fixpoint. query_compiled_elements em query_helpers.rs:495
avalia a expressão code do selector diretamente, sem interpolar query(...), em
scope stdlib. Reutiliza native_query e os seletores/locatability do L1. Compilation
preserva SourceDiagnostics; Retrieval conserva mensagem textual própria.

Wiring drena errors → warnings → depreciação somente na falha de compilação.
Falha de seleção/cardinalidade/campo retorna erro sem depreciação; sucesso
escreve stdout antes dos warnings. Stdin e MainNotFound mantêm os reparos
anteriores. None entries recuperam label por Location do registro final; Some
entries preservam seus campos congelados, sem reconstrução ou sobreposição.

## RED, refinamentos e limites

red-infra.log é RED da presença da API; red-cli.log contém os quatro controles
comportamentais falhando antes da implementação. Green-infra e green-query-helper
passaram nas fachadas novas. Green-cli-focused revelou dois controles adicionais:
contexto aninhado sem leituras perdia o filho e smartquote era serializado como
texto. O segundo já ocorria também no caminho existente de eval --in.

Discover em context_stabilization.rs:257–269 realizava filhos somente em HTML.
O L0 próprio foi atualizado antes da correção para descobrir/realizar filhos
produzidos também em Paged, usando realize existente sem reexecutar root válido.
Nested-green.log passa em erro aninhado e metadata. Seleção permanece por leitura
observada, com identidade/captura/styles/Source e orçamento existentes.

Green-cli-refined revelou label ausente da figura encontrada. O L0 de transporte
foi refinado antes de código para a projeção de entries None, conservando Some.
Green-cli-complete passou nos quatro testes novos, incluindo comparação integral
do corpo entre query e eval equivalente. Não houve alteração de L1 ou de sua
representação de aspas neste pacote.

O RED inicial exigindo smartquote não foi chamado de paridade fechada. A coleta
refined-debug.json.gz e a final comparam query-where-body **e**
eval-where-body-reference: vanilla mantém smartquote(double:false), Tekt já
resolve a aspa em Text("’") em markup_stream.rs:125. O critério do teste de
seletor foi separado desse débito preexistente de construção/serialização:
cardinalidade, corpo completo, textos adjacentes e equivalência com eval. O
negativo morfológico permanece nos recibos e no plano; não se certifica
morfologia de aspas nem se adivinham nodes a partir do glyph no shell.

Uma tentativa inicial de medida debug foi interrompida pelo rebuild removendo
o executável, sem coleta completa: initial-materialization.log não serve como
prova de paridade. Os snapshots de source correspondentes foram preservados.
As coletas seguintes copiam binários para caminhos imutáveis em /tmp antes das
execuções; o registro SHA-256 identifica exatamente os bytes medidos.

## Proveniência e comparação entregue

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitada.
Coleta UTC 2026-10-06T18:20:19.296441+00:00 até 2026-10-06T18:20:25.264562+00:00.
Delivered.json.gz conserva git diff HEAD --stat/status, hashes dos arquivos
alterados, comandos, fontes, cwd/ambiente e stdout/stderr/status. Sealed-state e
delivered-state conservam o conteúdo e hashes produtivos das rodadas; as fontes
não mudaram após o resselo. Binários medidos:

- Tekt release: 58dda0f1f04aaccbb7d235e389a9b6f7398382b4b569c89da72d28437942ecd2.
- Vanilla: 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea,
  pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489.

**23/23 controles originais** de warnings-cli coincidem integralmente em
status/stdout/stderr, incluindo os três resíduos pedidos: callback stdin,
selector com apóstrofo/Unicode diante de documento inválido e arquivo ausente.
A matriz ampliada coincide em 34/36: os dois negativos restantes são o mesmo
débito de morfologia de aspas, observado por query e eval. Summary.json registra
cada diferença e comparação do texto PDF; sem percentual global ou bytes PDF
como critério de linguagem. Nenhum benchmark de desempenho foi realizado.

## Escopo e checks

Seals.json registra somente seis pares:

- infra/pipeline → 03_infra/src/pipeline.rs.
- infra/query-helpers → 03_infra/src/query_helpers.rs.
- infra/pipeline/tests → 03_infra/src/pipeline/tests.rs.
- infra/pipeline/context_stabilization → 03_infra/src/pipeline/context_stabilization.rs.
- wiring/query → 04_wiring/src/query.rs.
- wiring/tests/cli → 04_wiring/tests/cli.rs.

Todos os owners acima têm caminhos canônicos sob 00_nucleo/prompts/.
Preseal valida V15/V26 antes do resselo; não houve --fix-hashes global.
Preservation.json verifica fontes/L0s anteriores fora do escopo. Nenhum unsafe,
contrato de watch, novo target/flag ou proposta PDF/bundle foi materializado.
Infra-full.log: 1190 testes passaram; build-debug/release e diff-check passaram.
Cli-full.log: 126 testes passaram, zero falhas (200.53 s registrados pelo
harness; não é benchmark do compilador). A rodada completa usa o estado selado.
Linter final.sarif: zero erros, 528 warnings e 1582 notes, sem V5/V6/V15/V26 nos
owners alterados. Avisos herdados permanecem fora do escopo; não é zero
violations global. D02/D03 e os três resíduos deste recorte estão fechados;
as outras frentes do plano e a morfologia de aspas continuam abertas.
