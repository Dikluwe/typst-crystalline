# Contexto — correção da página atual em display, 2026-10-05

Corrigida a divergência conhecida da fixture contextual: `#context counter(page).display()` passou de **0 para 1**, coincidindo com vanilla pinado. O problema era uma adaptação incompleta dos leitores do contador: get/at já consultavam a página lógica realizada, display ainda lia somente CounterRegistry. Nenhuma passagem foi removida nem orçamento de estabilização reduzido. A frente Contexto continua parcialmente aberta: total/final é uma unidade diferente, com divergência observada em both:true.

## Medição e origem antes da decisão

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. `before-state.json.gz` registra o estado de `2026-10-05T20:14:17.595978+00:00`, lista exata de arquivos alterados (`git diff HEAD --stat`), conteúdo das fontes/L0 e hashes de binários. Baseline CLI `f8fc2599bc599b39678575df1eea3fd71b807f1ffd257abbb0ce3413b2344915`; vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Fontes e receita do vanilla são as ratificadas pelo projeto; versão textual não é usada como prova.

`01_core/src/compiler/stdlib/counter.rs:378–390` já contém page_counter_value: da Location obtém página física e consulta a página lógica no PageStore. get/at usam essa fórmula em `:463–482`. Antes desta correção, `compiler/stdlib/counter/display.rs:61–77` lia somente o registro causal. `03_infra/src/pipeline/paged.rs:409–433` já injeta posição e PageStore e reexecuta contextos; não faltava simplesmente mais uma passagem.

O commit `9e3a267f554b89107d66e5001400de56d3cc6bb7`, localizado por `git log -S 'fn page_counter_value'`, introduziu a adaptação get/at. O L0 `00_nucleo/prompts/compiler/stdlib/counter.md`, seção P1485.2, especifica esses dois leitores e exclui final do escopo. Isso comprova cobertura incompleta da adaptação, não uma decisão humana de manter display divergente. No vanilla, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:271–284,456–466` display_at e at compartilham CounterAtIntrospection.

## L0, RED e correção

L0s proprietários editados primeiro: `00_nucleo/prompts/compiler/stdlib/counter/display.md` e `00_nucleo/prompts/infra/pipeline/tests.md`. Correção interna de paridade, fluxo contínuo ADR-0127: sem API pública nova, mudança de fase ou novo modo padrão. Consumers em relação 1:1, gate V15/V26 aprovado antes do resselo. Somente esses dois owners foram ressellados nesta etapa; a proposta pública de Coverage::from_ranges continua pendente e não foi implementada.

O teste integrado page_display_uses_realized_logical_page_at_each_context foi RED: duas páginas produziram `["0", "0"]` quando o esperado era `["1", "2"]`. O controle de contador string já passava. `red-state.json.gz`, com timestamp `2026-10-05T20:15:09.470735+00:00`, preserva teste, L0, fonte produtiva anterior e log.

A correção em display usa page_counter_value da fachada quando disponível, depois aplica o histórico/fallback anterior. O estado CounterAt observado é o mesmo entregue ao pattern/callback. Sem posição/PageStore conhecidos não inventa página 1; outras chaves conservam o histórico. Não duplica algoritmo, cria cache ou modifica o pipeline. O texto atualizado, não o byte PDF, é o observável de linguagem verificado.

## CLI pareada

Coleta de `2026-10-05T20:17:17.140145+00:00` a `2026-10-05T20:17:21.305816+00:00`, candidato CLI SHA-256 `8b228c47f5a6294fc057c238e9b87ea712fd9e91c104497188efbf4238baeb01`. `verification.json.gz` contém fontes completas, comandos, stderr, texto integral extraído por pdftotext, páginas com whitespace normalizado e hashes dos PDFs. O baseline é uma cópia do binário anterior comprovado, não rebuild do código atual. Não houve benchmark de desempenho.

| Caso | Baseline | Corrigido | Vanilla pinado |
|---|---|---|---|
| Página atual | 0 | 1 | 1 |
| Duas páginas | 0 / 0 | 1 / 2 | 1 / 2 |
| Início lógico 7 | 7 / 7 | 7 / 8 | 7 / 8 |
| Pattern romano | 0 / 0 | i / ii | i / ii |
| Callback | Page0 | Page1 | Page1 |
| display/get/at | D0 G1 A1 | D1 G1 A1 | D1 G1 A1 |
| Counter string | 3 | 3 | 3 |
| Total, both:true (aberto) | 0/0 | 1/0 | 1/1 |

Sete controles corrigidos/inalterados coincidem com vanilla. O oitavo delimita o escopo e continua aberto: CounterTotal/final requer páginas do documento pertinente, especialmente no bundle. Usar o último slot do PageStore global não resolveria corretamente totais de PDFs anteriores. Não interpretar convergência do número de páginas como prova de que essa leitura total está correta.

## Validação e estado

GREEN dos dois novos testes integrados: seis combinações de início/padrão/callback no primeiro teste, controle de counter string no segundo. Suíte completa core: **6.515 passaram, cinco ignorados**; infra: **1.150 passaram**; total **7.665 aprovados**. Build release passou. Logs integrais e timestamps em `execution.json.gz`.

Lint global sem erros, com avisos herdados: V5=172 (inclui L0 público de Coverage ainda pendente), V16=316, V17=22, V12=19, V18=6; notas V20=858, V19=698, V16=18. Não é zero violations global. Nenhum V5/V6/V15/V26 nos dois consumers desta correção; gate V15/V26 aprovado e git diff --check limpo. Não foram ressellados outros owners para esconder dívida.

Próxima unidade da Frente 7: medir e corrigir total/final por documento, incluindo bundles mistos e updates explícitos, antes de encurtar ciclos que dependam dessas leituras. Esta etapa fecha a divergência da página atual em display; não declara todos os observáveis contextuais corretos.
