# Origem da lacuna de memoização em eval e layout

A infraestrutura rastreável foi entregue antes das fronteiras que efetivamente guardariam e reutilizariam resultados. Em maio, os registros passaram a atribuir memoização automática a `track`, e os gates aceitaram contagem de chamadas/eviction como cumprimento de uma exigência de hits/misses. Em outubro, o problema foi investigado novamente e dividido expressamente: retenção de CST foi implementada; memoização de imports e reuso amplo de layout ficaram para contratos posteriores. Há evidência de adiamento e de uma promessa não demonstrada pelos critérios de encerramento. Não há prova de esquecimento, remoção de um cache funcional ou exclusão definitiva da memoização.

Investigação de 4 de novembro de 2026, sem mudança produtiva/L0 ou benchmark novo. Branch `Tekt`, HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **uncommitted working tree**. Captura inicial UTC `2026-11-04T15:10:52.586372+00:00`; lista exata do diffstat, status e hashes em [state-before.json](../../lab/results/construction/diagnostics/memoization-gap-2026-11-04/state-before.json). Este diagnóstico não é L0 e não autoriza implementação.

## Cronologia causal

- **Março e abril:** ADR-0001 autorizou comemo em L1, mantendo isolamento em L3 como direção futura. O adaptador B3 `TrackedWorld` entrou em `1475b27b5` e foi removido em `0f67240e9`. Os snapshots examinados não mostram função memoizada produtiva em L1 nesse período. P28 e P99 adiaram expressamente a integração incremental. ADR-0031 e `c991a791a` introduziram early hashing de Source, preparando uma capacidade sem ativar cache de eval.
- **Maio, M7 → M8:** ADR-0072:63–82 deixou a convergência por hash como solução intermediária e anunciou memoização entre iterações para M8. ADR-0073 entregou `track`, handles no Layouter, Position e eviction. P204A:214–230 excluiu re-walks parciais de M8, mas afirmou que comemo memoizaria consultas. ADR-0073:335–336 repetiu a promessa. **`track` não guarda resultados sozinho:** falta a computação memoizada que consome essas dependências.
- **Maio, encerramento:** P204G rejeitou a API per-call `last_was_hit` e criou contadores de eviction/chamadas. P204H marcou cumprida a condição de logging hits/misses; a divergência aparece na própria ADR-0073:386–387,493–507 e no inventário P204H:23. F3/ADR-0074 encerrou selamento de Positions, aceitando migração de consumidores como cumprida por inexistirem consumidores produtivos então. Isso prova infraestrutura, não cache hits.
- **Julho:** P699 (`a0ef796f7`) e P819 (`c8b46fef8`) introduziram a memoização produtiva encontrada em plugins. A alegação de P699 de que memoize já era usado noutras partes não foi corroborada pelos snapshots L1 examinados.
- **Outubro:** [Prioridade 6](../../lab/results/construction/diagnostics/incremental-prioridade6-2026-10-08.md) separou A = Source/CST, B = imports e C = layout. [A execução do lote A](../../lab/results/construction/diagnostics/source-session-execucao-2026-10-08.md):3,19–23 declara explicitamente que não encerra eval/layout. A investigação de watch também priorizou custos de fontes antes de B/C. Isso refuta a interpretação de que ninguém voltou ao assunto.

## Estado atual e correções ao estudo original

A busca em atributos reais encontrou **dois** `#[comemo::memoize]` produtivos, ambos em `01_core/src/entities/plugin_func.rs:49,63`, nenhum em eval/layout; o vanilla pinado contém **59**. A contagem estática não mede cache hits ou ganho. Pin atual confirmado: `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`; SHA-256 do oracle `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Os blobs dos arquivos vanilla com memoize coincidem com o pin. [scan.json](../../lab/results/construction/diagnostics/memoization-gap-2026-11-04/scan.json) guarda as verificações.

“Nada é reaproveitado” é incorreto. Já existem:

- caches de métricas compartilhados por Arc entre clones, inclusive nas tentativas de convergência (`font_metrics.rs:49–90`; `layout/convergence.rs:105–106`);
- compartilhamento explícito e reuso condicionado no pipeline paginado (`pipeline/paged.rs:908–915,921,976,1222–1237`);
- índice de realização retido pelo Module, com invalidação nos setters (`entities/module.rs:49,122–149`);
- reuso local de produtos de medição de células (`layout/grid/measurement_cache.rs:385–491`);
- SourceSession e FontCatalogSession persistentes no watch (`04_wiring/src/watch.rs:65–66`), com validação e limites próprios; fontes já têm prova rápida de geração elegível (`world/font_session.rs:32–39,61–64`).

O ponto fixo reentra em layout com novo Layouter, até cinco tentativas, compartilhando métricas clonadas (`layout/convergence.rs:88–139`). Contar `new` no fonte não determina instâncias por compilação. Source já tem Hash O(1) (`entities/source.rs:30–40`); refazer Content não é pré-requisito universal para memoizar imports. O hash público de Content não é automaticamente chave adequada de layout, pois pode projetar diferenças de origem relevantes para renderização.

O reuso dentro de uma compilação, entre ciclos do watch e entre processos exige contratos distintos. Não foi encontrado cache persistente de resultados eval/layout entre processos. FallbackFontMetrics empresta World; transportar seus caches entre Worlds requer identidade e invalidação das fontes, além de limites de memória.

## Tensão arquitetural e segurança

ADR-0030:45 situa expressamente memoização com comemo em L3; ADR-0029 proíbe globals mutáveis. Em paralelo, ADR-0001 permite comemo em L1, `crystalline.toml:88` permite memoize e o L0 de plugin_func manda usar cache no processo. As autorizações específicas posteriores explicam plugins, mas não revogam expressamente a regra geral nem autorizam automaticamente memoizar eval/layout. Há inconsistência documental a conciliar no escopo concreto; uma ADR genérica dizendo que “cache é RAM” não demonstra segurança.

O L0 de plugin_func:39–48 registra colisão possível entre hosts que reutilizem IDs/nome/args, porque host fica fora de Hash/Eq. Um host por World não assegura unicidade entre Worlds. Para imports, `Engine.world` é atualmente `&dyn World`, não World tracked (`entities/engine.rs:45`); route e sink são tracked. A chave/validação deve considerar dependências transitivas, inputs, features/target, contexto, rotas, diagnósticos e efeitos (`eval/modules.rs:44–116`).

## Ordem proposta de investigação e correção

Primeiro medir o estado atual por fase e tentativa, em CLI e watch, separando fontes/recursos do trabalho de eval. Depois contar reuso real dos caches existentes, custo de chave/validação e memória. Os benchmarks históricos de outubro usavam outro pin e estado; não certificam ganhos atuais.

Corrigir o critério de conclusão antes de declarar incrementalidade: chamada/eviction não pode valer como hit. Se imports forem o custo residual relevante, especificar e testar uma fronteira mínima com reuso demonstrado e invalidação por dependências/efeitos. Layout exige experimento separado com estilos, regiões, fontes, identidades, contexto, callbacks e rollback. Ganhos continuam hipóteses até medição pareada.

Qualquer implementação exige L0 proprietário atualizado antes de código, conciliação de ADR-0030/V13, grafo V15/V26 válido e gates RED→GREEN/revalidação. ADR-0127 determina parada para mudanças públicas/default/fase/compatibilidade; mudanças internas compatíveis seguem fluxo contínuo após classificação. Builds/testes futuros devem usar alvo exclusivo em `/dev/shm`, um job, teto de 12 GiB e zero swap, sem competir com trabalhos concorrentes. Paridade continua sendo de linguagem, conforme ADR-0107; copiar a mecânica vanilla não é obrigação.

O [relatório detalhado](../../lab/results/construction/diagnostics/memoization-gap-2026-11-04/relatorio.md) contém a linha do tempo com commits, referências file:line, hipóteses de refutação e limites das buscas. [git-history.json](../../lab/results/construction/diagnostics/memoization-gap-2026-11-04/git-history.json) e [sources.json.gz](../../lab/results/construction/diagnostics/memoization-gap-2026-11-04/sources.json.gz) preservam a evidência; [verification.json](../../lab/results/construction/diagnostics/memoization-gap-2026-11-04/verification.json) registra a preservação final dos arquivos recebidos.
