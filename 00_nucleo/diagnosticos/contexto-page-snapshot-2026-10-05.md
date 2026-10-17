# Contador Page: snapshot por ocorrência e documento — 2026-10-05

## Resultado e proveniência

Materializado o conjunto L0 confirmado pelo humano, manifesto SHA-256 `1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. `before-state.json.gz` preserva o estado inicial; `measurement.json.gz` registra horário UTC, lista exata de mudanças (`git diff HEAD --stat`), fontes, receita de CLI, textos e hashes de binários. `execution.json.gz` conserva o estado final e logs. Os esclarecimentos internos e correções de pins/ordenação dos blocos L0 seguiram fluxo contínuo ADR-0127. O pin do Núcleo é o digest canônico com seu separador de dependências, distinto do SHA dos bytes no manifesto.

Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Tekt SHA-256 `5bc45f4ec0972c6c46450c0cab3479e2ae2ef41c286c424201c1b7fc07823eb1`. Não usar a versão pública como prova de proveniência.

## Correção da herança

Antes, o último update causal era usado como início de todas as páginas; total/final também liam estado global sem avanço físico nem escopo documental. Agora L1 produz conjuntamente current/final por Location a partir de ações canônicas, ordem causal, posições e documentos realizados. Readers get/at/display/final/both e replay consultam essas vistas. O PageStore recebe os números lógicos da mesma geração; L3 compõe posições e margens. Callbacks Page aguardam posições conhecidas, conservando o estado completo entre ações; as leituras projetam um nível mesmo com delta zero, conforme o oráculo. Nenhum arquivo inteiro é reavaliado, nenhuma fórmula de update foi duplicada em L3 e nenhum unsafe foi acrescentado.

O walk de bundle registra privadamente o pertencimento das Locations de cada body. Checkpoints podem repetir depois da transformação e não delimitam o escopo. HTML não consome páginas físicas e retém updates manuais para a expansão; markers já consumidos são removidos antes da exportação. Os ciclos e seu orçamento foram conservados. A tabela numerada sem contextos continua usando duas passagens; a independente mantém uma.

## Verificação

RED inicial: duas sentinelas falharam (total zero e marker CounterUpdate chegando à exportação HTML). RED adicional: estado vazio/multinível divergente no get. Logs preservados em `execution.json.gz`. GREEN inclui callbacks com estado paginado, updates iniciais/intermediários/tardios, páginas sem leitura final, estados vazios/multiníveis, HTML, bundles mistos, margens e recuperação após callback inválido no mesmo World.

As **17 fixtures CLI** coincidiram com o vanilla em texto por página/PDF e texto do body HTML. Isso prova os casos medidos, não paridade universal ou igualdade de bytes PDF/HTML. Exemplo: duas páginas passaram de total/final zero para 2; update 7 na segunda página preserva 1/7 na primeira e 7/7 na segunda; o primeiro PDF do bundle conserva 1/1 enquanto o seguinte termina em 8.

Suítes completas: **6515 testes L1**, **1154 L3**, cinco testes L1 ignorados. Build release passou. `crystalline-lint .` terminou sem erros; V15/V26 e linhagem desta etapa passam. Permanecem **534 warnings e 1574 notes** anteriores à etapa, sem findings novos por comparação ao SARIF anterior. Portanto não declarar zero violations do repositório. A proposta anterior de Coverage::from_ranges permanece pendente e não foi materializada nem ressellada.

## Retenção e limite da medição

Sonda de retenção, log `retention.log`: tabela de 4 células/2 páginas e tabela de 320 células/5 páginas retiveram quatro entradas somadas nas duas vistas, seis slots de capacidade e 32 bytes de payload Vec. O limite inferior conhecido de armazenamento reservado dessas duas vistas foi **416 bytes** em ambas: structs HashMap + slots vezes sizeof((Location, Vec<usize>)) + capacidades dos Vec. Esse número não inclui metadados/alinhamento do alocador, scopes privados do registry, clones transitórios, históricos de gerações, fontes ou RSS. O teste foi executado antes de acrescentar a sentinela de recovery; o código produtivo era o mesmo, e os horários dos logs estão preservados.

Não há comparação temporal A/B desta mudança. Não afirmar ganho de velocidade ou memória total. A frente de contexto está corrigida para os controles medidos; memoização, invalidação abrangente e redução dos ciclos continuam frentes próprias. A correção remove o bloqueio semântico identificado antes de se encurtarem esses ciclos.
