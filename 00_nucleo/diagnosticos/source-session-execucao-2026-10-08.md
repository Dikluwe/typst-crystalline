# Prioridade 6, lote A — sessão sintática implementada

Lote A materializado após o `confirmo` do usuário. Decisão nova: conservar o reuso sintático limitado, pois remove parsing repetido e reduziu CPU de forma repetida no import sintaticamente maior. **Não é memoização de avaliação ou layout, nem fechamento da prioridade 6 inteira.**

## Contrato e proveniência

Ratificação: manifesto `00_nucleo/materialization/incremental-prioridade6-2026-10-08/manifest.json`, SHA-256 `5b8a98ecd797f4db41b7cead7aa7dabc3b3ced8071e9e26b511e21efcd36511d`. `source-session-execucao-2026-10-08/ratification.json` guarda resposta e UTC. A proposta/publicações novas foram aprovadas sob ADR-0127 antes de código.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**, incluindo trabalho anterior preservado. Os recibos estão no diretório irmão `source-session-execucao-2026-10-08/`. `final-state.json.gz` conserva fontes/L0, lista exata de alterações, diff stat e UTC; `scope.json` prova somente cinco consumers, cinco owners e um núcleo novo. Sem mudanças produtivas fora desse conjunto.

- Baseline P5: `897aa1261a069ef1dc601a0e5cb256380e901ac6a0324d1a288b405d5f6d0ff0`, cópia `/tmp/p6-previous-897aa126`.
- Final/medido: `d76926491304bb6d2291be082542f3199619a67757916c3f11485052d18d5371`, `target/release/typst`.
- Vanilla dos controles: pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

O último ajuste marcou os contratos ativos como ratificados, sem alterar regras/código funcional. Rebuild final é byte-idêntico ao release medido, confirmado em execution.json. Não extrapolar medições antigas nem somar ganhos anteriores.

## Implementação

L3 publica `SourceSession::new()` e `SystemWorld::new_with_source_session(...)`. Watch mantém uma sessão sintática e a sessão de fontes independente durante sua chamada; compile transporta os handles por helper privado e usa o novo constructor antes da leitura eager. Constructors antigos e stdin/expressão conservam seus caminhos.

Cada World continua novo: mapas/IDs, bytes/read cache, inputs, plugins, clock, cobertura e dependências são próprios. SourceSlot relê o arquivo em cada World. Só consulta a sessão após leitura UTF-8 bem-sucedida e resolução canônica; com sessão ausente, não acrescenta essa canonicalização. Hit exige caminho, FileId **corrente** e texto integral iguais, sem confiar em mtime/size ou Source::Eq/hash. Nenhum ID é restaurado. A Source/CST é imutável, compartilhada por Arc; diagnósticos de sintaxe voltam a passar pelo eval normal.

A sessão guarda somente Sources markup físicas, sem Module/Func/Sink/Route/Location/Frame/fontes/transientes/negativos. LRU por instância com Mutex: oito entradas, 1 MiB agregado de texto UTF-8, 512 KiB por entrada. Oversize segue parsing normal; orçamento de texto **não é limite de RSS/CST**. Sem unsafe, global, disco persistente ou transferência eval↔layout. Campos e traits de L1 não mudaram.

Owners: `03_infra/src/world.rs`, `world/cache.rs`, `world/source_session.rs`, `04_wiring/src/compile.rs`, `watch.rs`, cada qual com L0 exclusivo. Núcleo `00_nucleo/prompts/_nuclei/watch-source-generation.toml`: raw SHA-256 `1fe31c90e4dace3144214435ff0c2627302c10c8c5e8d60ec0b62c0484023363`, pin efetivo `7ec137b6996e0ea442e3cdda52ce723fd32cb67338ab363b7ff2d733458ef8fa`. Os pins foram ordenados pelo critério byte-sorted do V26; nenhum núcleo anterior foi alterado ou ressellado.

## RED→GREEN e recontagem

A primeira tentativa de montar o stub teve erros de compilação, conservados como inválidos e **não usados como RED**. Corrigida a montagem do teste, o RED compilou e teve seis falhas/três sucessos: ausência de reuso/retenção nos Worlds, identidade, LRU e limites. GREEN: nove testes passaram. Incluem texto diferente com size/mtime restaurados, path/id, mudança de ordem de imports, UTF-8 inválido/remoção/recriação, sintaxe, transientes, concorrência e limites.

GDB separado dos tempos, ciclo inicial e três edições de main, import inalterado:

| Contador acumulado | Baseline P5 | Final |
| --- | --- | --- |
| Source::new | 2 / 4 / 6 / 8 | 2 / 3 / 4 / 5 |
| entrada eval de main | 1 / 2 / 3 / 4 | 1 / 2 / 3 / 4 |
| entrada de import | 1 / 2 / 3 / 4 | 1 / 2 / 3 / 4 |

O import deixa de construir CST, mas a avaliação permanece. Não chamar isso de cache de Module. Recibos count/baseline-count guardam fixtures, script, stdout, horários e binários.

## CPU, latência, memória e startup

Dois watchers próprios coexistem ociosos; só um é editado/compila por vez. Mesma CPU, RAYON_NUM_THREADS=1, PDF sem tags, edição atômica, ordem alternada, aquecimento excluído. Três rodadas de dez pares por fixture/motor em cada ensaio. Monitor contínuo não encontrou build/test/debugger concorrente. Fontes, arquivos produtivos e binários foram revalidados ao fim de cada ensaio. Recibos performance*.json.gz contêm comandos, hashes, fontes, HEAD/árvore, timestamps, amostras e warm startup.

Primeiro ensaio UTC `2026-10-08T13:47:21.516043+00:00` a `13:48:10.696560+00:00`, intervalo fixo 160 ms. A latência pareada ficou sensível ao polling; foi conservada, não descartada. Ensaio adicional UTC `13:50:31.281904+00:00` a `13:51:29.176074+00:00`, intervalos variáveis 130–310 ms, seed 61008, mesmos binários.

Percentuais são **medianas das razões pareadas**, não razões das medianas absolutas:

| Fixture, import inalterado | CPU anterior → final, ms (fixo) | Variação CPU fixo | CPU anterior → final, ms (variável) | Variação CPU variável |
| --- | ---: | ---: | ---: | ---: |
| 4.000 declarações | 60,36 → 57,67 | −5,31% | 58,38 → 55,77 | −3,94% |
| 30.000 iterações | 68,56 → 69,01 | +0,47% | 66,34 → 66,22 | +0,10% |
| import mínimo | 53,34 → 53,53 | +1,13% | 50,81 → 50,95 | +0,51% |

Caso sintático: seis medianas de rodada de CPU menores; 27/30 pares melhores no fixo e 26/30 no variável. Há benefício repetido nesse recorte. Controles pequenos/de cálculo não melhoraram; seu custo pode crescer ligeiramente. A avaliação desses módulos não é evitada, e a maior parte do ciclo permanece. Não certificar ganho geral ou estatística universal com três clusters.

Latência sintática no fixo: medianas absolutas 89,36 → 89,08 ms, razão pareada 1,203. No variável: 124,81 → 111,46 ms, razão pareada 0,967; razões de rodada 1,008/0,897/1,074. Os resultados não sustentam ganho repetido de latência. Import mínimo variável ficou 99,96 → 126,58 ms; essa piora observada também está conservada. Polling e fase de chegada participam dessas medidas; não interpretar CPU menor como publicação necessariamente mais rápida.

RSS sintático mediano: 46.800 → 46.308 KiB no fixo e 46.850 → 46.224 KiB no variável. Em outros controles houve variações positivas e negativas; não atribuir causalmente RSS à sessão. Retenção de CST continua sendo um custo de memória potencial, apesar dessa observação. Startup tem apenas três pares por fixture/ensaio: no sintático variável CPU 132,08 → 131,62 ms e wall 135,31 → 134,36 ms; não certifica ganho ou ausência de regressão de startup. Todos os valores/controles completos estão nos summaries.

Decisão: conservar o lote pela redução de trabalho/CPU repetida no import maior e preservação de linguagem, registrando latência não certificada e overhead pequeno nos controles. Lotes B/C e validação de recursos continuam frentes distintas.

## Linguagem, render e checks

Os testes reais com previous/final/vanilla passaram em nove mutações por motor: main/import/read, erro, arquivo criado após negativo e recuperação. PNG e substituição/adição/remoção de fontes reconhecidas no próximo ciclo passaram sob o contrato vigente. `language-checks.json` compara o texto completo das mutações bem-sucedidas entre os Tekt e testemunhos de PNG. Erros conservam o último artefato; não usar esse fato como igualdade geral de diagnósticos com upstream.

`geometry.json` comprova onze pares de PDFs previous/final: nove saídas finais dos três workloads/rodadas e duas de recursos/recuperação. Texto/caixas de palavras, páginas/dimensões, fontes e rasters a 36 dpi iguais. Não é certificação global de paridade ou de todas as superfícies.

- Infraestrutura: **1.245 passaram, zero falhas, um ignorado**, incluindo nove novos.
- Watch CLI: **cinco passaram**. Dois HTTP falharam inicialmente por bind bloqueado no sandbox; repetição autorizada em localhost passou 2/2. Não houve rejeição da revisão automática nessa repetição.
- Build dev/release passaram; último release é byte-idêntico ao medido.
- Lint: **zero erros**, 535 avisos e 1.640 notas herdadas. Não é zero violations global.
- Scope: exatamente onze arquivos produtivos/L0/núcleo; diff check limpo. Não foi reexecutada toda a suíte de workspace; dívida HTML anterior permanece fora deste lote.

Uma colisão de nome de recibo no harness de recursos sobrescreveu `before.json.gz`. Foram preservados os recibos de behavior e recuperadas as fontes anteriores do diagnóstico incremental já existente, conferidas contra todos os hashes-base aprovados. O digest histórico original do contador foi conservado como histórico; o estado final, release e snapshots de cada benchmark têm identidade própria verificada. A recuperação está explicitada em execution.json, sem inventar o timestamp perdido.

## Reprodução e continuação

Comandos: `cargo test --offline -p typst-infra --lib -j 1 -- --test-threads=1`; build dev/release; filtro CLI `watch` e repetição `p1485_watch_` em ambiente que permita localhost; `crystalline-lint .`. Scripts behavior/count/performance/performance-jitter/geometry e logs estão no diretório do pacote. Contadores e tempos devem rodar separados, sem edição de fontes durante o benchmark.

Lote A concluído. **B: memoização da avaliação de imports com dependências, efeitos, warnings e identidades rastreados. C: reuso de layout validado por observação causal de conteúdo, estilo, recursos, contexto e páginas.** Esses contratos não são autorizados pelo gate deste lote.
