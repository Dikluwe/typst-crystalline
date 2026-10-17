# Reutilização do prefixo post — 2026-10-10

## Decisão

**Experimento rejeitado; consumer e L0 restaurados exatamente.** A versão com prefixo na pilha diminuiu leituras e alocações, mas não demonstrou ganho geral relevante de tempo/RSS. O controle `default-scopes` apresentou RSS positivo em todas as três rodadas das duas janelas: +0,411% inicialmente e +0,220% na repetição. Os ganhos iniciais de tempo em `local-set-1600` e `no-fill-1600` diminuíram na repetição. Não incorporamos a complexidade adicional por uma economia interna pequena sem benefício geral comprovado. A causa das diferenças de RSS não foi isolada; efeitos de alocador/arranjo do heap continuam hipóteses.

## Proveniência

- HEAD: `06848cdcb5450f2c555626bf116bc7acd82bd82b`; **working tree não commitada**. Lista completa de arquivos alterados/não rastreados e `git diff HEAD --stat` nos recibos `before.json.gz`, `final-validation.json.gz` e em cada medição. Snapshot de todos os 966 consumers Rust antes, no candidato e após restauração.
- Janela inicial final: 2026-10-10T05:27:08.759148+00:00 → 2026-10-10T05:28:10.074890+00:00 UTC.
- Repetição final: 2026-10-10T05:28:46.916249+00:00 → 2026-10-10T05:30:18.763674+00:00 UTC.
- Referência congelada e binário final restaurado: SHA-256 `90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335`.
- Candidato final medido: SHA-256 `c8eeb1df0333d34d2136652b3ec1421093897cdcc69728aeeee0f2d08ffe3373`. Fonte e L0 completos no `final-validation.json.gz`; delta em `owner-diff.patch` e selos em `seal.json` são históricos do candidato, **não** do owner restaurado.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Árvore/receita ratificadas em P1489.
- Hashes de binários, fixtures, fontes, scripts, argv, ambiente, CPU, UTC e todas as amostras nos recibos. Binaries antes/depois com o mesmo basename `typst` e caminhos de igual comprimento. RAYON=1, stack64MiB, CPU fixada, warmups excluídos. Tempo/RSS sem interposer; contadores de alocação medidos separadamente com interposer, sem usar seu tempo para decidir desempenho.

## Evidência antes da implementação

A fonte congelada, `03_infra/src/fontdb/selective.rs:155–178`, lê até34 bytes de `post` para versão/count, descarta o prefixo e relê o trecho retido. A alteração tentou aproveitar a primeira leitura, com validação integral dos limites declarados, metadados/cobertura, identidade do arquivo e fallback preservados. Cada refinamento foi especificado no L0 antes do código, com resselo somente do owner afetado. Mudança privada interna, fluxo contínuo ADR0127.

Três alternativas:

1. **Prefixo Vec + suffix Vec temporário:** menos leituras, mas **+534 alocações** por compilação em cada um dos seis casos. Fonte, L0 e recibos em `temporary-suffix/`.
2. **Prefixo Vec crescente + leitura direta do suffix:** **−289 alocações**, −9.248 bytes solicitados acumulados por compilação. Tempo de variantes +0,843% e +1,162% em duas janelas; todas as rodadas positivas. Rejeitado. Fonte/L0/recibos em `growing-prefix/`.
3. **Prefixo [u8;34] na pilha + reserva única do payload:** **−1.112 alocações**, **−37.230 bytes** solicitados acumulados por compilação em todos os seis casos, com contagens idênticas nas três repetições. Esses bytes são volume acumulado de pedidos ao alocador, não RSS nem memória simultaneamente viva.

## Correção e controles

- RED de execução no baseline: dois testes falharam, registrando a releitura do prefixo e a ausência da nova leitura de suffix onde os faults foram injetados. Primeiro houve erro de compilação por import de teste; corrigido antes do RED válido. Log/source reconstruída verificável no recibo inicial de `temporary-suffix/`.
- Candidato final: **14 testes seletivos passaram**; suíte infra **1.282 passaram, 2 ignorados**. Limites, rollback de leitura parcial, seek/read failures, TTC, truncamentos, versões post, cobertura e identidade foram verificados.
- Corpus: **1.140 faces**, **1.139 aquisições seletivas aceitas e uma recusa preservada**. Bytes projetados, metadados e cobertura iguais ao baseline; fallback equivalente ao parser completo. Requested bytes **51.334.449 → 51.296.297** (−38.152). Operações instrumentadas de Read/Seek **11.460 → 11.156** cada (−304). Wrapper Cursor reproduz os corpos produtivos; isso mede pedidos do leitor, não uma contagem independente de syscalls do kernel. Hashes de fontes conferidos antes/depois.
- **33 documentos**: mesmos textos/páginas/geometria e raster36dpi antes/depois. **Seis consultas**, nos três engines, preservaram `query(table).len() = 1`. Controles adicionais do oracle registrados; isso não fecha divergências anteriores com vanilla, especialmente no caso variants, cujo trabalho não é equivalente.
- Builds dev/release e lint concluídos com exit0; lint mantém avisos preexistentes em outros arquivos arquivados. Nenhum V5/V15/V26 no owner afetado. Não afirmar zero avisos no projeto.

## Tempo e RSS

Primeira janela: três rodadas × quatro pares por caso, seis casos com oracle e quatro controles curtos. Repetição dirigida pelos riscos: três rodadas × oito pares, oito casos. A tabela mostra mediana das razões pareadas candidato/baseline; não é a razão entre medianas agregadas.

| Caso, repetição final | Tempo | Pico RSS |
|---|---:|---:|
| plain | -0.220% | +0.467% |
| unused-font | -0.589% | +0.333% |
| unused-style | -0.017% | +0.079% |
| default-scopes | -0.151% | +0.220% |
| local-set-1600 | -0.264% | +0.117% |
| no-fill-1600 | -0.048% | +0.005% |
| variants-1600 | +0.455% | +0.050% |
| inline-set-1600 | -0.138% | +0.172% |

São diferenças pequenas; não certificamos aceleração geral nem redução de RSS. Não repetimos medições indefinidamente para escolher uma janela favorável. `summary.json` registra razões por rodada, sinais e hashes dos recibos; `summarize.py` reproduz os cálculos.

## Estado entregue e próximo candidato

Consumer `03_infra/src/fontdb/selective.rs` e owner `00_nucleo/prompts/infra/fontdb/selective.md` voltaram aos bytes recebidos. O binário release voltou ao SHA da referência. `restoration.json.gz` confirma todos os 966 hashes produtivos e os logs de build/lint/teste após restauração. Nenhum commit.

Próxima medição sugerida: quantificar subtabelas Unicode que apontam para o mesmo payload `cmap` e o custo da coleta/união de seus intervalos. `03_infra/src/fonts/coverage.rs:76–103` percorre todas as subtabelas Unicode; `Coverage::from_ranges` compacta, ordena e codifica em `01_core/src/entities/font_book.rs:213–245`. Os L0 vigentes foram consultados: união/codificação pertencem ao font_book; validação/terminação das subtabelas pertencem ao extrator L3. Não propomos pular subtabelas ou remover ordenação sem medir duplicações, CPU e membership primeiro.
