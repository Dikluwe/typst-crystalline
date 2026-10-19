# Prioridade 2 — cobertura de fontes por intervalos: execução e medição

Registro final UTC: 2026-10-07T23:22:46.387396+00:00. Estado: working tree não commitado no HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. Inventário exato, diffstat e bytes anteriores em `coverage-ranges-2026-10-07/before-code.json.gz`; fontes testadas/sha256 em `tested-final.json.gz`, selos em `seals.json` e preservação em `final-verification.json`. Cada número abaixo vem desse estado, das identidades e dos horários por execução; não representa HEAD limpo nem um futuro rebuild.

O dono ratificou com “confirmo” o pacote cujo manifesto SHA-256 é `f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549`. Os três L0 canônicos foram instalados antes do código; os candidatos/manifesto originais foram conservados. Resselo restrito aos três pares de owners, sem commit ou alteração da árvore produtiva herdada fora deles.

## Implementação e decisão

L1 ganhou `Coverage::from_ranges(impl IntoIterator<Item=Range<u32>>) -> Self`: união exata de intervalos semiabertos, compactação local antes da ordenação, codificação canônica e limites verificados por testes. `from_codepoints` permanece disponível e inalterado. L3 ganhou `03_infra/src/fonts/coverage.rs`, owner `00_nucleo/prompts/infra/fonts/coverage.md`; o facade privado existente em fonts.rs delega ao filho. O parser tipado autoriza formatos 4/12/13; leituras brutas usam slices/aritmética verificadas. Sentinel FFFF, glyph0, segmentos invertidos, buracos, overlap, classificação Unicode e primeira subtable inválida conservam o comportamento anterior.

Falha nativa reverte ao checkpoint da subtable e usa seu callback. Um endpoint exclusivo não representável abandona a tentativa por intervalos da face e conserva o caminho legado completo; não cria promessa para inputs fora do domínio suportado. Formatos 0/6/10 foram exercitados como callback. Nenhuma política de catálogo, metadados, slots, ordem, seleção, cache, shaping, I/O, flag ou fase do produto mudou. Aquisição seletiva de metadados continua fora deste pacote.

## Correção e validação

O constructor teve três falhas semânticas RED antes da implementação (`red-core.log`); o adapter teve quatro falhas RED e dois controles vazios aprovados (`red-infra.log`). GREEN final autoritativo: `workspace.log`, com os três testes L1 e nove testes do adapter aprovados. O log intermediário `green-infra-intermediate.log` conserva a tentativa de contagem u32::MAX que acionou um assert do parser em debug; o fixture final usa contagem truncada representável, sem alterar parser/produto.

Builds debug/release e 7.941 testes aprovados: core 6.637, infra 1.204, shell 90 e CLI query 10; cinco testes core históricos ignorados. Comandos/status/UTC exatos em `validation.json`. Linter: zero erros, 535 warnings históricos e 1638 notes; owners e selos válidos. `verify.py` confirma preservação dos demais arquivos recebidos e identidade entre release medido e release final.

Igualdade de cobertura em 1.140 faces: 1.112 instaladas, 17 embutidas e 11 fixtures, com path/índice/membros e hashes dos assets em `probe-results/{validate,identity}.json`. A comparação usa o callback legado completo e o adapter produtivo, além dos oráculos de conjuntos nos testes. Catálogo completo também conservou FontBook, quantidade de slots e cada path/índice.

## Proveniência dos tempos

Release novo SHA-256 `e44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109`; anterior R1 `0a2410ec5b4a075417943d25a4d24ac860da6ff0adf4845707d4f18dc9cc7add` congelado; vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Oráculo do lab comprovado pelo P1489, não o executável da raiz nem só a versão pública. Fontes de sistema/embutidas/configuração, argv, fontes integrais, HEAD/dirty, fingerprints e UTC nas identidades e estados de `performance/` e `probe-results/`.

CPU 4, Rayon 1, vanilla jobs 1, dois warmups por engine/bloco, três rodadas de 12 trios por fixture, seis permutações. CLI: 864 execuções medidas +144 warmups, 24 blocos válidos; gate de 30s, monitor de concorrência e hashes antes/depois. Wall inclui GNU time/taskset iguais nos três engines; RSS vem do compilador após exec. Sem cold-disk, watch ou incremental. Não comparar absolutos com populações anteriores que usavam wrappers diferentes.

O probe usa o módulo produtivo diretamente; controle legado reproduz o catálogo anterior com o mesmo metadata parser. Timer de extração inclui Face::parse e exclui leituras dos bytes; catálogo inclui descoberta, leitura, metadados, coverage e descarte. Os 36 pares alternados de cada modo conservaram book/slots/coverage. Runner efetivamente medido preservado em `probe-run-initial.py`, cujo hash é verificado contra o estado inicial; runner atual registra somente overrides relevantes no logger. A tentativa inicial de gate que identificou o próprio shell coordenador como concorrente não chegou a medir: foi repetida com exec direto.

## Resultados

Extração + parsing: mediana anterior 64,36ms → atual 15,90ms; redução pareada 75,45%, todos os 36 pares melhores. Catálogo integral: 135,49ms →93,69ms; redução pareada 31,06%, todos os 36 pares melhores. A variação de medianas entre rodadas foi 6,35%/7,29% na extração e 6,37%/5,03% no catálogo (anterior/atual); portanto os valores absolutos e percentuais são indicativos, não uma garantia precisa. O ganho do catálogo manteve a razão pareada com faixa de 1,47%.

Na tabela, delta é a mediana das razões pareadas contra o anterior; RSS é mediana do pico do processo. A coluna de repetibilidade exige média/mediana e faixa das medianas de cada engine abaixo de 3%.

| Fixture | Anterior ms | Atual ms | Vanilla ms | Delta pareado | RSS anterior→atual MiB | Repete <3% anterior/atual | Saída geométrica equivalente ao vanilla |
|---|---:|---:|---:|---:|---:|---|---|
| context | 157.71 | 115.13 | 128.92 | -25.48% | 44.28→44.17 | não | não |
| image | 8.00 | 8.01 | 9.61 | +1.45% | 18.47→18.42 | não | sim |
| multiscript | 274.57 | 236.27 | 165.20 | -14.13% | 143.48→143.30 | sim | não |
| pipeline-single-font | 152.83 | 112.44 | 126.53 | -26.53% | 42.90→42.77 | sim | sim |
| shapes | 10.46 | 10.78 | 10.11 | +1.90% | 19.78→19.91 | não | sim |
| table-alias | 163.14 | 122.16 | 131.17 | -24.83% | 45.42→45.29 | não | sim |
| table-local | 175.43 | 139.42 | 134.14 | -23.24% | 45.35→45.46 | não | sim |
| text-dense | 174.13 | 134.27 | 130.37 | -22.92% | 44.38→44.30 | não | não |

O controle Hello (`pipeline-single-font`) passou repetibilidade nos três engines e qualificação contra vanilla: 152,83→112,44ms, redução pareada 26,53% (IC bootstrap95 da razão: [0.7327403255830212, 0.7391030861072696]). Multiescrita passou repetibilidade, com redução de14,13% contra saída anterior equivalente, mas conserva diferenças de render perante vanilla. Contexto/texto denso/tabelas mostram ganhos pareados de22,92–25,48%; sua maior dispersão impede tratar esses percentuais como promessa.

JPEG e formas sem texto não acionam esta cobertura: ficaram praticamente iguais. Shapes teve delta de+1,90% (cerca de0,3ms nas medianas), com intervalo pareado acima de1; o critério de repetibilidade falhou e não há perfil que atribua esse delta à função alterada. Preservar essa negativa; não alegar aceleração de todos os documentos. RSS da CLI permaneceu praticamente igual; não há redução relevante de RAM demonstrada. O processo isolado do probe mantém bytes de todas as faces em memória, portanto seu RSS não representa memória exclusiva da coverage nem da CLI.

## Saídas e limites

Todos os oito fixtures e suas três rodadas conservaram texto, páginas/dimensões, famílias/tamanhos/cores e posições perante o binário anterior; stdout/stderr idênticos em288 pares. Imagem, shapes e multiescrita também tiveram raster idêntico ao anterior a144dpi, controles em `visual/` e `controls.json`. Pixels de shapes coincidem com vanilla; imagem e multiescrita conservam diferenças perante vanilla. Contexto e texto denso também não qualificam geometricamente perante vanilla. A geometria/fontes iguais em imagem sem palavras não prova equivalência de pixels — o controle raster explicita essa diferença.

A implementação está validada e o ganho de compilação com fontes está demonstrado no controle repetível Hello. O subitem D13/cobertura por intervalos está materializado; aquisição seletiva de metadados continua pendente. Este pacote não fecha Styled/Table/render universalmente nem corrige os resíduos anteriormente registrados em C2-R1.

Arquivos de prova: `final-verification.json`, `controls.json`, `performance/summary.json`, `performance/index.json`, `probe-results/{extract,catalog,validate,execution,identity}.json`, receipts/estados e logs. Sem commit.
