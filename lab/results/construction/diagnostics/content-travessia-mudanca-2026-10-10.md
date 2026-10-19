# Travessia de conteúdo com reconstrução após mudança — 2026-10-10

## Estado e proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Os recibos em `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/` conservam status completo, `git diff HEAD --stat`, UTC, comandos, CPU, fontes, fixtures e hashes de drivers/binários. `snapshot-change-aware-before.json.gz` conserva o conteúdo e SHA-256 dos seis arquivos de código/L0 anteriores à experiência. Não houve commit, staging, reset ou resselo global.

O binário anterior é SHA-256 `29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`, preservado em `/tmp/tekt-before-change-aware-2026-10-10`. O candidato final é `target/release/typst`, SHA-256 `1765fa094270c83be515a8ca738b06a92e8ba4a7f5594b3d76cb084cfe82c9cc`. O vanilla é o pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; versão textual não substitui essa identidade.

## Medição antes da decisão

O censo anterior `table-map-passes-before.json.gz` e a fonte de `entities/content/traversal.rs`, `entities/elements/table.rs` e `entities/elements/table_cell.rs` mostram reconstrução dos pais mesmo quando callbacks retornam `None`. Os atalhos já aceitos em `finish_occurrences` e `wrap_page_columns` retiraram duas passagens, mas não davam essa propriedade às demais chamadas de `map_content`.

`change-aware-red-gate.json.gz`, UTC **00:05:16–00:05:18**, fixou antes da implementação no máximo **470.000** pedidos glibc em `local-set-1600`; o release anterior fez **483.823**. O controle `no-fill-1600` fez **378.152**. O RED de seis testes de módulo confirmou três falhas: a Table inteira, o carrier Realized e a Cell irmã não alterada eram recriados. Ordem, interrupção por erro e substituição explicitamente igual já passavam.

## Decisão e implementação

Os três L0 proprietários foram atualizados antes do código: `00_nucleo/prompts/entities/content/traversal.md`, `00_nucleo/prompts/entities/elements/table.md` e `00_nucleo/prompts/entities/elements/table_cell.md`. Trata-se de mudança interna sob ADR-0127; assinaturas públicas, métodos de trait e fase permanecem. A paridade de linguagem é separada da mecânica de vtable/igualdade Rust conforme ADR-0107/0026/0109.

`map_content` mantém seu match estático exaustivo. Internamente, `None` agora significa que nenhum filho ou callback solicitou reconstrução; `Some(Content)` transporta a substituição. O callback roda uma vez, depois dos filhos, e qualquer `Some` conta como mudança mesmo se o valor for igual. Não há teste de igualdade/hash/endereço para deduzir mudança. Sequências alocam o resultado apenas no primeiro filho alterado e conservam os irmãos não alterados por clone de referências.

Source conserva normalização de span e transparência; Located suprime o callback do body imediato; Realized transforma somente visual; ShowRecipe conserva seu snapshot. Table e Cell decidem a reconstrução nos próprios arquivos, mantendo a descida raw/resolved/standalone, os campos lexicais e os planos. Variantes ainda não migradas delegam ao percurso anterior e retornam conservadoramente um resultado materializado. Isso limita o ganho, mas não pula seu conteúdo. `map_text` e `map_content_pre` não foram migrados nesta experiência.

## Alocação, tempo e memória

`change-aware-green-gate.json.gz`, UTC **00:11:28–00:11:30**, mediu **418.107** pedidos em `local-set-1600` (**−13,6%**) e **342.836** em `no-fill-1600` (**−9,3%**), passando o gate. Esses pedidos incluem malloc/calloc/realloc/memalign; não são a contagem exata de alocações vivas nem de bytes retidos.

`measurements-change-aware.json.gz`, UTC **00:11:36–00:12:11**, fez três rodadas com 12 pares por variante, ordem intercalada, uma CPU afixada, uma thread, fontes e opções iguais. Nenhuma execução usou o interpositor de alocação. Os valores abaixo são medianas das razões pareadas; RSS é o pico do processo, não memória retida de watch.

| Fixture | Tempo candidato/anterior | Pico RSS anterior → candidato, KiB | Tempo anterior/vanilla | Tempo candidato/vanilla |
|---|---:|---:|---:|---:|
| `local-set-400` | −3,06% | 32.996 → 31.292 | −5,77% | −9,42% |
| `local-set-1600` | −11,16% | 49.928 → 42.358 | +35,20% | +19,93% |
| `no-fill-1600` | −5,31% | 39.938 → 38.202 | +19,51% | +13,50% |
| `inline-set-1600` | −10,45% | 54.378 → 47.834 | +43,77% | +28,99% |

As três medianas de rodada foram favoráveis em cada fixture. No caso local de 1.600 células, ficaram em **−9,75%, −11,27%, −11,26%**. Isso sustenta manter a peça neste corpus; não prova ganho universal, ausência de regressão em todas as famílias ou equivalência integral de performance com vanilla.

## Qualificação e validação

`qualification-change-aware.json.gz`, UTC **00:12:21–00:12:39**, confirmou **25/25** fixtures com páginas, texto, palavras, caixas e raster idênticos ao release anterior. `qualification-change-aware-extended.json.gz`, UTC **00:12:21–00:12:24**, confirmou mais **6/6** documentos cobrindo contexto/query, show/alias, matemática/estilos/bloco, colunas, substituição textual/espaços e include de fonte. O material incluído também tem hash no recibo. As diferenças já existentes contra vanilla não são fechadas por essa comparação de preservação.

`qualification-change-aware-query.json.gz`, UTC **00:12:20–00:12:24**, confirmou `query(table).len() == 1` em seis fixtures para ambos os releases Tekt e o vanilla pinado. Os oito testes focais cobrem reuso, irmãos alterados/inalterados, substituição igual explícita, erros, fronteiras causais, vista de Table preparada, prioridade/metadata de Cell standalone/resolved e normalização de Source. A suíte completa final do núcleo passou: **6.669 testes, zero falhas, cinco ignorados**. `cargo build`, `cargo build --release` e `crystalline-lint .` retornaram zero; o lint mantém avisos preexistentes, sem drift nos três owners afetados. Logs e estado final estão em `validation-change-aware.json.gz`.

`seal-change-aware.json` registra código/prompt e SHA-256 dos três pares exclusivos: traversal `97fb6130`/`905d8da6`, Table `c59e3951`/`0e2187ac`, Cell `5cd7a7e8`/`b9923892`.

## Limite e próximo candidato

A peça pode ser usada pelos consumers existentes de `map_content`: não exige outro callback público nem um segundo percurso para detectar mudanças. Ela já reduz trabalho na observação, na passagem principal e na normalização quando os ramos não recebem substituição. Contêineres ainda conservadores, como Block, Boxed e a família de matemática, continuam candidatos para propagar a mesma prova através de seus owners.

`normalize_insertions` continua retornando substituição para cada Sequence, mesmo quando não precisa achatar nem ajustar espaços. O próximo recorte deve permitir que esse callback reporte ausência de mudança quando houver prova, preservando newline, vizinhos e normalização de zero/um filho. As divergências restantes contra vanilla devem ser medidas novamente, com este release como referência aceita.

## Continuação — normalização de inserções

O candidato 1 foi implementado e medido em [content-normalizacao-reuso-2026-10-10.md](content-normalizacao-reuso-2026-10-10.md). O ganho é localizado em reuso/memória; os tempos dos controles permanecem mistos.
