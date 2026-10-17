# Prioridade 6 — diagnóstico incremental e lote A proposto

A prioridade 6 foi investigada; **a implementação produtiva está pendente de ratificação de API**. O pacote concreto fica em `00_nucleo/materialization/incremental-prioridade6-2026-10-08/`. O lote A propõe reuso de Sources sintáticas físicas entre Worlds novos de watch. Não encerra a incrementalidade de avaliação/layout.

## Medição antes da escolha

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. `00_nucleo/diagnosticos/incremental-prioridade6-2026-10-08/before.json.gz` e `control-state.json.gz` conservam fontes/L0 completos, lista exata de alterações, diff stat, UTC, fontes e binários. Todas as mudanças produtivas anteriores permanecem. Os recibos measurement/control-measurement/count/parse-probe guardam comandos, fixtures, horários e identidade do estado.

- Tekt P5, SHA-256 `897aa1261a069ef1dc601a0e5cb256380e901ac6a0324d1a288b405d5f6d0ff0`.
- Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Três rodadas por fixture/motor, cinco edições medidas após aquecimento por rodada, processo watch real, mesma CPU/RAYON_NUM_THREADS=1 e PDF sem tags. Edição atômica muda o texto visível do main, conservando o import. Motores medidos sequencialmente; sem build/testes/debugger simultâneos aos tempos. Fontes/produto foram conferidos e cada PDF contém a revisão esperada. CPU pertence às threads do watcher; latência inclui polling/publicação e não é custo de computação.

| Import inalterado | CPU Tekt, ms/edição | CPU vanilla, ms/edição | Latência Tekt, ms | Latência vanilla, ms |
| --- | ---: | ---: | ---: | ---: |
| mínimo, um inteiro | 53,03 | 1,55 | 81,98 | 102,48 |
| cálculo, 30.000 iterações | 67,58 | 1,42 | 96,79 | 101,99 |
| sintaxe, 4.000 declarações | 60,33 | 1,54 | 89,52 | 102,02 |

Essas são medianas descritivas destas fixtures, sem intervalo de confiança. Não interpretar a razão dos dois motores como ganho que uma correção terá. O import mínimo já custa 53 ms no Tekt: recursos demandados durante a avaliação participam de eval_ms, que foi 52,08 ms nesse controle. Os outros eval_ms foram 66,77 e 58,39 ms; layout ficou aproximadamente 0,27 ms em cada fixture. Não subtrair medianas de fases/casos para atribuir causalmente um custo, nem prometer eliminar toda a diferença por memoização de imports.

## Repetição comprovada e contratos vigentes

GDB no release Tekt contou ciclo inicial e três edições. Acumulados Source::new: 2/4/6/8; eval_module_import: 1/2/3/4; eval root: 1/2/3/4. O main muda, mas o import não muda. A contagem mostra criação sintática repetida e entrada repetida de avaliação, não duração nem contagem de todos os passos internos. O vanilla ratificado é stripped, portanto não há contagem GDB equivalente; não inferir número de evals upstream dos tempos.

`03_infra/src/world.rs:141-196` cria World/slots/IDs novos; `world/cache.rs:44-56` cria Source após leitura; `01_core/src/entities/source.rs:56-68` faz parse/numberize. `compiler/eval/modules.rs:44-115` executa imports com contexto/Engine próprios. `compiler/eval/mod.rs:732` recebe World sem tracking integrado. ADR-0005 exige World puro e separação B3; ADR-0030 situa comemo em L3. O annotation memoize de `lab/typst-original/crates/typst-eval/src/lib.rs:38-47` recebe World tracked. Acrescentar somente memoize à fachada não prova invalidação das leituras delegadas.

O L0 Source atual exclui reparse incremental. O L0 SystemWorld preserva caches por World, exceto sessão de fontes já ratificada. L0s eval/paged conservam isolamento de ocorrências, warnings, identidade, contexto e tentativas. Não propor reutilizar Module/Frame por Hash/PartialEq visual de Content, hash u64 de Source ou igualdade mecânica de índices.

## Experimento sintático isolado

`parse-probe.rs.txt` é diagnóstico fora das camadas produtivas, compilado no diretório temporário. Trinta amostras por modo, três execuções na mesma CPU. Medianas Source::new: 3,014/3,030/3,046 ms. Comparação integral de texto mais Source clone: 0,002435/0,002475/0,002490 ms. Inclui cópia da String; não inclui filesystem, World, avaliação, layout, exportação ou destruição completa de CST. Não é benchmark de produto com cache implementado.

Há um custo evitável sintático, mas pequeno comparado ao ciclo inteiro dessas fixtures. Hipótese: conservar CST do import pode melhorar a fixture sintaticamente maior, sem mexer em eval/efeitos. Será refutada se validação/retenção custar o ganho, se não houver redução repetida da CLI/CPU ou se algum observável mudar. Não alegar que o lote A resolve o custo dominante ou torna o Tekt incremental como upstream.

## Pacote para ratificação

Proposta: `SourceSession` opaca L3 com `new()`, e `SystemWorld::new_with_source_session(...)`. A sessão conserva somente Sources markup físicas, comparando caminho canônico, FileId corrente e texto integral após releitura. Não restaura IDs, não compartilha bytes binários/negativos/Module/Func/frames/efeitos/contexto/fontes e não evita avaliação/layout. Transientes continuam isolados. World continua novo; dependências e resolução continuam atuais. LRU: oito entradas, 1 MiB total de texto e 512 KiB por entrada, sem promessa de limite de RSS. Sem unsafe/global/disco persistente/relógio em L1.

Cinco owners: world, cache, novo source_session, compile e watch. Um núcleo TOML compartilhado. Candidatos completos estão no pacote; metadata de núcleo usa um único bloco no topo. Manifesto SHA-256 `5b8a98ecd797f4db41b7cead7aa7dabc3b3ced8071e9e26b511e21efcd36511d`; candidatos e bases possuem hashes completos. Os L0 vigentes **não foram substituídos**, nem foi escrito código produtivo.

A ADR-0127 §2 exige parada para contrato público. A confirmação deve autorizar essas novas entradas e o lote concreto. Depois: instalar L0s/pins, RED→GREEN, implementar, selar somente owners, medir redução Source::new para import inalterado e invalidação por texto/id/path, testar erros/carets/Unicode/mtime restaurado/ordem de imports/efeitos/evicção/concorrência/fontes/assets/render, builds/suítes/lint e comparação pareada contra P5. Ausência de benefício repetido exige retirada da conexão produtiva, com evidência conservada.

A continuação B é memoização de avaliação de imports com dependências/efeitos/identidades rastreados; C é layout com observação causal de estilo/recursos/contexto/páginas. Esses contratos não estão ratificados por este pacote. Retenção de fontes, validação e reconstrução de stdlib permanecem custos a medir separadamente; não incluir reparos desses eixos sob o rótulo de incrementalidade.

## Estado final desta preparação

`verification.json` confirma nenhum arquivo produtivo/L0 vigente alterado, binário P5 preservado e hashes do pacote. Linter: zero erros, 535 avisos e 1.640 notas herdadas. Não houve mudança produtiva para justificar reexecução de suítes; a sonda compilou e as medições/counter concluíram. Nenhum ganho produtivo novo foi certificado nesta etapa.
