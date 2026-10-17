# Catálogo: lifetime, capacidade e memória residente — 2026-10-10

## Escopo e proveniência

Item 1 da lista de candidatos: investigar a retenção após o reaproveitamento de payloads cmap, preservando a melhoria aceita. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. `catalog-heap-retention-2026-10-10/before.json.gz` guarda UTC, status completo, diffstat, hashes dos 966 consumers produtivos e texto integral dos owners envolvidos. Os recibos de fases/CLI guardam estado exato, horário, fontes, argv, scripts, bibliotecas e binários. `final.json.gz` guarda o estado final e os logs.

Baseline aceito com cmap: SHA-256 `584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba`, congelado em `/tmp/tekt-catalog-heap-benchmark/old/typst`. Oráculo upstream pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência ratificada em P1489. Não usar a versão pública como prova de identidade.

## Medição antes da decisão

`03_infra/src/world/fonts.rs:202` compõe books text/system/math-code/project. No baseline, os três books auxiliares sobreviviam às suas cópias até o retorno. O diagnóstico copia o corpo produtivo e usa bibliotecas release para discovery/metadados; não acrescenta observadores ao produto. `heap_probe.c.txt` contém o interposer diagnóstico glibc (compilar com `-x c`); foi arquivado como texto após o parser C do linter rejeitar `_Thread_local`. Os recibos históricos preservam o nome original `heap_probe.c` e seu hash, sem mudar a medição.

A autotestagem do interposer (`heap-selftest.json`) verificou malloc/calloc/realloc/free. O registro usa mmap separado; os checkpoints registram sua residência. Não houve perda de entradas ou frees desconhecidos nos ensaios. RSS desses executáveis inclui código de diagnóstico e não deve ser confundido com RSS absoluto da CLI. Pedidos vivos incluem capacidades, enquanto `malloc_usable_size` inclui arredondamento do alocador. `mallinfo2` inclui metadata/tcache e não equivale ao volume lógico vivo. `malloc_trim` foi usado somente como controle diagnóstico, sem mudança de política no produto.

Cinco processos por combinação, alternância old/new, com/sem trim. `phase-catalog.json.gz` mede baseline; `phase-catalog-early-drop.json.gz` mede protótipo; `phase-catalog-materialized.json.gz` mede código materializado. Os modos old/new referem-se ao adapter cmap anterior/atual dentro do mesmo executável, não ao vanilla. Comparação de lifetime usa apenas cmap atual e processos sem trim.

| Checkpoint | Antes, bytes solicitados vivos | Depois, bytes solicitados vivos |
|---|---:|---:|
| Após embedded | 4.812 | 4.812 |
| Discovery vivo | 518.682 | 518.682 |
| Book system + database vivos | 3.215.647 | 3.215.647 |
| Após descartar database | 2.703.485 | 2.703.485 |
| Antes de metadados project, após math | 4.445.967 | 2.020.724 |
| Catálogo final vivo | 2.020.385 | 2.020.385 |
| Após descartar catálogo | 4.102 | 4.102 |

No catálogo final, arena glibc caiu de 5.132.288 para 4.866.048 bytes (−260 KiB). Blocos livres na arena caíram de 2.899.840 para 2.633.648 bytes. A liberação antecipada elimina uma sobreposição observada de 2.425.243 bytes antes do processamento project; não elimina a duplicação necessária durante a própria cópia do book system. O hash canônico final foi igual (`17172753192570882911`) nos adapters e lifetimes.

Separação de capacidade/payload no ensaio materializado: 1.130 slots, capacidade 2.244, slot 96 bytes → 106.944 bytes de capacidade excedente do vetor de slots. Famílias somam 18.740 bytes e a mesma capacidade; FontInfo tem 56 bytes, portanto 63.280 bytes lógicos nas entradas. Cobertura canônica tem 398.282 palavras u32 (1.593.128 bytes lógicos). Não foi exposta API para ler capacidades privadas de Coverage/FontBook; esses valores lógicos não pretendem fechar todos os bytes vivos por campo. O coletor cmap reduziu a soma de capacidades temporárias de ranges de 1.991.512 para 997.276 entradas, e a máxima de 65.536 para 32.768; a soma é acumulada entre chamadas, não memória simultaneamente residente.

Trim liberou páginas mantendo exatamente o mesmo volume solicitado vivo: no modo cmap atual materializado, após audit 2.022.090 bytes vivos, RSS descontado do registro 11.747.328 → 10.063.872 bytes; após descartar catálogo, 4.099 bytes vivos, RSS 9.674.752 → 7.421.952 bytes (medianas dos cinco processos com trim). Isso demonstra páginas livres conservadas pelo alocador no ensaio. Não demonstra que todo o acréscimo anterior de aproximadamente 0,4 MiB da CLI tenha uma única causa; o ensaio de fases não reproduziu essa diferença integral.

## Correção e classificação

Correção interna de lifetime em L3, fluxo contínuo ADR-0127: L0 atualizado antes do código, testes e resselo restrito ao owner `infra/system-world/fonts.md`. Três `drop` explícitos liberam system/math/project books imediatamente depois de copiar as entradas. Ordem, pareamento, metadados, seleção e aquisição preguiçosa permanecem sob os contratos existentes. Não muda API, fase do pipeline nem política do alocador. O adapter cmap aceito permanece byte a byte igual ao baseline deste item.

## Verificação

Dois testes inline foram escritos antes da implementação e já passavam no baseline: ordem/metadados/cobertura/pareamento e composição vazia. O RED de recursos foi executado separadamente em `lifetime_gate.py`: baseline falhou nas cinco repetições (4.445.967 > limite 2.036.769); materializado passou nas cinco (2.020.724 ≤ limite). O limite pertence à fixture auditada, não define paridade de linguagem nem é threshold de tempo em unit test.

Suite infra: 1.285 testes passaram, dois ignorados. Builds dev/release passaram. Lint final exit 0, sem V5/V15/V26 do owner afetado; warnings históricos continuam. A primeira execução do lint falhou no parser do artefato C diagnóstico; o fonte foi preservado como texto, e a execução final passou. Não houve resselo global.

34 documentos (25 tabelas/controles + nove escopos, incluindo multiscript) conservaram texto, páginas, dimensões, caixas e raster a 36 dpi entre baseline/candidato. Seis consultas de tabelas deram 1 nos três engines. A comparação com vanilla é controle de regressão; diferenças preexistentes de trabalho/linguagem não são convertidas em alegação de ganho de performance contra o oráculo.

## CLI nativa: antes/depois

`measurements-repeat.json.gz`: três rodadas, oito pares medidos por caso, aquecimento excluído, CPU fixa, um thread, sem interposer/trim. Delta de tempo = mediana dos ratios pareados; RSS abaixo = medianas absolutas em KiB, não diferença de médias.

| Caso | Delta tempo | RSS antes | RSS candidato |
|---|---:|---:|---:|
| plain | +0.070% | 27394 | 27330 |
| unused-font | +0.172% | 29088 | 28968 |
| unused-style | +0.438% | 27956 | 27904 |
| default-scopes | +0.502% | 53642 | 53956 |
| local-set-400 | +0.426% | 31144 | 31174 |
| local-set-1600 | -0.043% | 42300 | 42520 |
| no-fill-1600 | -0.391% | 38068 | 38174 |
| inline-set-1600 | -0.984% | 47768 | 47792 |
| families-1600 | -0.085% | 38736 | 38748 |
| variants-1600 | +0.258% | 292972 | 293128 |

A redução do catálogo isolado não se traduziu em queda geral de RSS: casos maiores subiram levemente. Tempo em unused-style, default-scopes e local-set-400 subiu nas três rodadas. Por esse risco concreto, foi repetida uma janela independente com plain e esses três controles, mesmo protocolo, outra ordem de execução.

### Repetição dos riscos

`measurements-risk-repeat.json.gz`, três rodadas × oito pares por caso:

| Caso | Delta tempo pareado | Rodadas de tempo | RSS antes → candidato (KiB) |
|---|---:|---|---:|
| plain | +0.054% | +0.006%, -0.194%, +0.415% | 27338 → 27310 |
| unused-style | +0.207% | +0.296%, +0.230%, -0.244% | 28062 → 27944 |
| default-scopes | +0.040% | -0.087%, +0.454%, -0.039% | 53652 → 53894 |
| local-set-400 | -0.028% | -0.641%, +0.433%, +0.045% | 31196 → 31132 |

## Decisão: hipótese medida, tentativa rejeitada

A melhoria de lifetime é real no ensaio, mas RSS nativo não melhora de forma geral. Plain economiza somente 64 KiB na primeira janela e 28 KiB na repetição, enquanto default-scopes cresce 314 KiB e 242 KiB. Os pequenos deltas de tempo não sustentam uma melhoria: a repetição variou de sinal entre rodadas. Não concluir uma regressão universal de CPU nem considerar fechado o acréscimo anterior de aproximadamente 0,4 MiB.

Rejeitada a incorporação dos três drops. Restaurados byte a byte somente `03_infra/src/world/fonts.rs` e seu Prompt L0 a partir do snapshot deste item, incluindo retirada dos testes experimentais. O código/L0 exatos do candidato e logs ficam em `candidate.json.gz`; o RED/GREEN de recursos e as qualificações permanecem auditáveis. O adapter cmap aceito não foi tocado. Todos os 966 consumers produtivos devem coincidir com `before.json.gz` após restauração, e o release deve recuperar o SHA-256 baseline. `final.json.gz` registra essa verificação.

Resultado útil: o catálogo mantém cerca de 2 MB vivos; Coverage é a maior parcela lógica; há capacidade excedente nos slots, duplicação temporária de metadados e páginas livres ainda residentes. Menos alocações acumuladas do cmap não equivale a menos RSS. A origem integral dos 0,4 MiB extras da CLI continua aberta. Próximo experimento orientado por estes dados: localizar retenção no processo completo por fase com a mesma entrada, distinguindo catálogo, layout/export e páginas de código, antes de escolher outra alteração.

Restauração verificada: nenhum dos 966 consumers produtivos difere do início deste item; os quatro arquivos de código/L0 guardados no snapshot estão idênticos. Builds dev/release e lint após restauração passaram (exit 0); release SHA-256 recuperado exatamente `584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba`. Warnings históricos permanecem.
