# JPEG grayscale com ruído — leitor direto e validação (2026-10-28)

A regressão anteriormente medida em **+5,48%** foi reduzida: na janela final, o ruído ficou **-6,51% contra o executável recebido** e **-0,88% contra o anterior rápido**. O gradiente ficou -11,88% contra o recebido. Passaram os nove limites de tempo/RSS, os gates anteriores e a validação completa. A receita de compilação e as dependências permanecem idênticas às recebidas.

A temporização com GDB localizou o acréscimo dentro da validação: aproximadamente 24,009 ms no anterior rápido e 26,350 ms no recebido para o ruído. Com `std`, o `ZCursor` de zune passa pelo adaptador genérico de `BufRead`/`Seek`, incluindo um laço de leitura exata e reposicionamento. O novo leitor privado trabalha diretamente no slice completo: uma leitura limitada e uma cópia para o destino já fornecido, mantendo posição, prefixo parcial e rollback em EOF. Só a reconstrução nativa grayscale usa esse transporte; o caminho convertido/RGB, o guard SSE2, SIMD, limites, oito linhas reutilizadas, reconstrução completa de Huffman/IDCT, condição Complete e fallbacks permanecem iguais. Não há novo modo, contrato público ou fase.

| JPEG | Recebido (ms) | Atual (ms) | Tempo / recebido | Tempo / anterior rápido | RSS / recebido |
|---|---:|---:|---:|---:|---:|
| gray-noise | 47,56 | 44,46 | -6,51% | -0,88% | +0,33% |
| gray-gradient | 97,97 | 86,33 | -11,88% | -10,47% | +0,30% |
| gray-large | 30,95 | 31,26 | +0,99% | +1,72% | +0,77% |
| rgb-large | 45,96 | 46,54 | +1,26% | +0,93% | +0,45% |
| gray-horizontal | 34,93 | 34,76 | -0,46% | -1,08% | +0,93% |
| gray-vertical | 33,58 | 33,67 | +0,28% | +0,10% | +1,52% |
| shared-rgb-16 | 149,08 | 150,17 | +0,73% | +0,78% | +0,50% |
| shared-rgb-pages | 149,50 | 149,93 | +0,29% | +0,31% | +0,38% |
| distinct-rgb-4 | 165,23 | 166,33 | +0,67% | +0,54% | +0,44% |

Ruído: IC95% pareado atual/recebido **[-6,63%, -5,98%]**; atual/anterior rápido **[-1,24%, -0,64%]**. As medianas de RSS foram 23,39 MiB recebido e 23,47 MiB atual. Os outros casos também permanecem dentro do teto fixo de 103%; a tabela mantém visíveis as pequenas diferenças positivas de tempo, sem afirmar aceleração universal.

A campanha final usa três executáveis na mesma janela, com seis permutações balanceadas de ordem, dois warmups, **101 pares por papel no ruído/gradiente, 61 no uniforme/RGB e 31 nos demais**: **1.437 processos medidos**, além dos warmups. GNU time registra maxRSS; bootstrap pareado tem 10.000 reamostragens e seed 20261028. Um job, Rayon 1, MemoryMax 12 GiB, High 8 GiB, zero swap; sem build ou benchmark concorrente. Máquina medida: AMD Ryzen 7 5800H, x86_64, Rust 1.92.0. Os números não certificam outras máquinas ou imagens.

O controle rápido é o Tekt `5c6e11a8f386a4d6e6d087188f72721fe9e729d74b07661f7861b7c920907226`, usado apenas como referência de desempenho. O recebido desta tarefa é `4ec7ab5e12336c0b00ac8660776ddce0d60c7d5b8784ea4aef68a2e9d11058c2`. A comparação de linguagem usa o vanilla ratificado, e não esses controles. A observação histórica de +5,48%, o RED novo e a janela final têm seus próprios estados e horários; não são fundidos em uma única medição.

O **RED de desempenho** foi confirmado antes da implementação: recebido/anterior rápido 1,05374182, IC95% [1,05062819; 1,05896200], contra limites previamente fixados de mediana ≤1,02 e limite superior do IC ≤1,03. O mesmo predicado passa no produto final; também se exige recuperação de pelo menos 2% contra o recebido, com IC superior abaixo de 1,0. A primeira coleta com dois papéis tinha uma falha de ordenação e foi excluída do aceite; o RED formal usa a coleta balanceada de três papéis, preservada em `resource-red2.json`.

Os seis testes do leitor foram escritos antes de sua implementação e inicialmente falharam por tipo ausente — um RED de materialização, separado da regressão real de desempenho. Eles verificam EOF/prefixo/rollback, leituras parciais e peeks, sufixo restante, seeks, extremos sem overflow e 1.500 operações mistas contra o adaptador recebido. Um sétimo teste cobre JPEG ruidoso de dimensões ímpares, reconstrução completa e cortes de entropia com EOI comparados ao decoder legado. O módulo ficou **28/28**; os testes de alocação e compartilhamento anteriores permanecem ativos.

Passaram os **12 gates completos: 10.030 testes Rust, 7 ignorados preexistentes, 42 testes Python**, builds, Clippy, fmt, Rustdoc, linhagem direta/reversa e lint integral sem avisos/erros. V15/V26 precederam as resselagens. As 2170 entradas produtivas/L0 ficaram congeladas durante a validação. **17.922 casos de compatibilidade JPEG** passaram no harness recompilado do estado final; as observações JPEG e payloads mantêm os critérios anteriores.

A campanha anterior de **936 processos** também passou, sem alterar os limites: gradiente atual/antes-SSE -13,85%, IC95% de razão [0.8604084361046443, 0.8669999198174946], atual/antes-native-Luma -21,36%, nove RSS ≤1,03. R6: tamanho normalizado/R5 **0.86944144**, RSS geomean/R5 **0.95448794**, com limites individuais, repetibilidade e tempo de matemática aprovados. O release final tem 51481224 bytes e SHA-256 `d726aef1e418e0aa848562c5bbd1d348bc368faae56477d9a2ad6f3b99a8e4be`.

Foram preservados os alvos já corrigidos: matemática de bloco 35/35 fontes completas SVG/posições dentro do corpus de 36, quatro fronteiras físicas e o refutador de colunas explícito; largura 40/40 PDF/SVG e 8/8 consultas; Stack 32/32 PDF, 77/77 SVG, 6/6 matemática, efeitos/Here, 28/28 imagem/h, 10/10 fronteiras e 12/12 rasters RGB. As oito matrizes têm zero violações/desconhecidos e ordem idêntica. Os 40 rasters de largura preservam as referências qualificadas, e a imagem comparativa foi aberta para conferir a continuação vertical mantida.

Continuam os limites preexistentes: tracking R10 e demais refutadores registrados, matemática inline seguida de linha, atribuição de coluna, `repeat-math-control`, binding `phantom-child` rejeitado pelo oracle, topo de prosa após imagem/quebra no mesmo filho, baseline de rodapé de 2,595993 pt e quatro diferenças de pintura JPEG (CMYK, gradiente, horizontal e vertical grayscale). O caso oversized do vanilla termina por timeout 124 e permanece não qualificado por recurso. A correção deste relatório trata o tempo do JPEG com ruído; não fecha esses itens.

Proveniência: HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**, recebida em 2026-10-28T12:42:22.873143+00:00 e congelada em 2026-10-28T14:22:18.986622+00:00. Os 181 arquivos sujos/não rastreados recebidos foram preservados, exceto a edição autorizada do scan e seu owner e a entrada aditiva no índice. Entraram apenas o leitor privado e seu L0, com propriedade 1:1: quatro entradas produtivas/L0 alteradas ou novas, dois consumidores. Cargo, lockfile, flags de target e wiring permanecem byte a byte iguais. Lista exata, fontes, comandos, binários e horários estão nos recibos; a auditoria preservou 278782 arquivos dos manifestos anteriores, incluindo a tarefa da continuação vertical.

Oracle: commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`, SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Nenhum resync, commit ou push foi feito.

[Gate final e ICs](../../lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/final-noise-gate.json) · [Medições e amostras](../../lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/final-direct/receipt.json) · [Patch sobre o recebido](../../lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/patch-against-received.diff) · [Estado congelado](../../lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/final-state.json) · [Relatório integral](../../lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/relatorio.md).
