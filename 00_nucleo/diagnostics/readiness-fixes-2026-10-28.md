# Readiness — índice preservado e regressão JPEG corrigida

A correção do índice de equações e a recuperação de desempenho JPEG estão
qualificadas no build padrão atual. Os doze gates técnicos e os 21 estágios da
qualificação completa terminaram. As quatro aprovações humanas permanecem
intactas; nenhuma tolerância, fixture ou baseline foi relaxada.

## Fonte e proveniência

Árvore não commitada, HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`. Build padrão concluído em
`2026-10-30T01:42:06.056852+00:00`, executável `physical24-release`, SHA-256
`2d5f9f565178554d2f40f9a8918854ef86d73076430565ecfe05c3b5ac292beb`. O build remove explicitamente overrides de rustflags do
ambiente e usa a configuração real do repositório. Código, L0 e configuração
permaneceram congelados durante toda a qualificação.
O [recibo consolidado](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-final-qualification24.json) vincula fontes integrais,
SHA-256, `git diff HEAD --stat`, instantes de medição e cada coletor.

Oráculo: commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, tree
`483bd6f1d8ac8810db47b65336d402647955fd17`, executável SHA-256
`22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`.
Execuções com MemoryMax 12 GiB, High 8 GiB, swap zero e um job de Cargo, sem
compilação concorrente nas medições de desempenho.

## Por que uma correção do índice afetou JPEG

O reparo do índice mudou a disposição do executável. O decoder JPEG continuava
com o mesmo código e a mesma quantidade de trabalho. O laço escalar de 16 bytes
na rotina nativa de cópia passou do deslocamento 24 para 56 módulo 64; na segunda
posição ele cruza a linha de cache de instruções. Os controles isolaram a
posição da mesma rotina: o caso cinza maior ficou cerca de 17% mais lento.
A [medição controlada](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-copy-placement-measure20/receipt.json) conserva pares,
fontes e binários; isso sustenta causalidade de posicionamento nesta máquina,
sem presumir contagem de cache misses ou garantia para qualquer processador.

A correção usa alinhamento de funções de 8 bytes em **toda a recompilação das
dependências**, no alvo x86_64-unknown-linux-gnu. Mantém release s, ThinLTO,
uma unidade de codegen e zune-jpeg opt3. No controle correto, a mesma rotina de
913 bytes volta a pôr o laço no deslocamento 24 módulo 64. Configuração de 32
bytes, alinhamento de todos os blocos e overrides do consumidor foram rejeitados
por tempo/memória. O [controle inválido da opção tardia](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-late-alignment-refutation23.json)
foi explicitamente invalidado: mudar só a etapa final não desfazia o alinhamento
já compilado nas dependências. Todos esses resultados foram preservados.

A [diferença total de fontes](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-net-source-delta24.json) comprova que,
desde o build19 reprovado em tempo, só mudaram `.cargo/config.toml`, seu L0 e o
hash de linhagem de `main.rs`. Os corpos produtivos do índice e do JPEG ficaram
idênticos. O ajuste é mecânica interna de compilação, fluxo contínuo da ADR-0127.

O problema original do índice estava no L0 e no código: Equation dentro de Math
não cria nova ocorrência consultável/contador/label; o snapshot original de
`show` não pode emitir Metadata fantasma. O Nucleus de equation-membership e os
três percursos corrigidos permanecem preservados, com a distinção de corpos Flow.

## Qualificação completa atual

- 12 gates técnicos aprovados; 10247 testes Rust e
  42 Python aprovados, zero falhas e sete ignorados preexistentes.
  Linter estrito sem erros/warnings; linhagem reversa: 1006 consumidores, zero erros.
- Índice: 43 verificações bilaterais aprovadas.
- 622 casos PDF/SVG: zero regressões em
  11442 critérios visuais anteriores e
  17638 verificações de compilação anteriores,
  mantendo 0,001 pt e paint independente.
- Preservação histórica: 1720 critérios sem regressões;
  17776 arquivos históricos intactos.
- JPEG: 936 amostras pareadas e 1916 diretas;
  todos os gates originais de tempo, IC, Luma e RSS aprovados.
- Compatibilidade JPEG: 17922 casos executados novamente
  com a biblioteca de testes **release atual**, incluindo erros legados exatos.
- Vinte controles JPEG preservados contra o produto anterior em status, erro,
  páginas, texto, geometria, raster e payloads. Observações contra o vanilla,
  inclusive negativos e diferenças preexistentes, continuam no recibo integral.
  Na coleta original, o vanilla excedeu o prazo de 45 s no caso `oversized` e
  atingiu cerca de 9 GB; a pressão de memória bloqueou também o watchdog
  interno. A
  [intervenção externa registrada](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-native-watchdog24.json) encerrou
  somente esse subprocesso. A
  [reprodução com supervisão independente](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/jpeg-oversized-independent25/report.md)
  eliminou a incerteza: com os mesmos limites, o vanilla atingiu pico de
  8.864.845.824 bytes, acumulou 11.870 eventos `memory.high` e foi encerrado
  pelo deadline independente em 45 s; o Crystalline rejeitou o mesmo input em
  0,31 s com “Memory limit exceeded”. O caso está confirmado como bug de
  robustez/amplificação de recursos do vanilla pinado, não requisito de paridade.
- R6: tamanho normalizado 0,867819 (teto 0,95) e média geométrica
  de RSS 0,956552 (teto 0,97); limites por caso, repetibilidade
  e tempo de math contra z+shared também aprovados.

Razões de mediana atuais; todos os nove casos satisfazem os limites originais:

| Caso | Tempo/controle rápido | Tempo/recebido na tarefa | RSS/controle rápido |
|---|---:|---:|---:|
| gray-noise | 0,983461 | 1,003606 | 1,006838 |
| gray-gradient | 0,904561 | 1,021067 | 1,005108 |
| gray-large | 1,004373 | 1,005362 | 1,005914 |
| rgb-large | 0,998918 | 0,995546 | 1,002017 |
| gray-horizontal | 0,988675 | 1,011517 | 1,003724 |
| gray-vertical | 0,996615 | 1,006795 | 1,009383 |
| shared-rgb-16 | 1,011808 | 1,008699 | 1,004640 |
| shared-rgb-pages | 1,013658 | 1,006106 | 1,004808 |
| distinct-rgb-4 | 0,991598 | 0,986785 | 1,007059 |

O ruído também satisfaz mediana<=1,02, IC95% superior<=1,03 e recuperação>=2%
contra o controle recebido. A campanha pareada conserva seus gates próprios.

## Aprovações, negativos históricos e alcance

Preservadas: protocolo físico de headings/sumário/numeração;
`PageRunningBody::with_numbering_input`; os dois métodos de overhang de
`FontMetrics`; e `Option<TableAlignment>` com set declarativo.
Os textos ratificados e seus recibos mantêm os mesmos hashes.

A falha histórica R4 que exigia o antigo `α` continua nos logs originais. A
conversão `𝛼` já correspondia integralmente ao oráculo antes do reparo do índice.
A auditoria atual de linguagem e os 28 controles PDF/SVG contra o build19
passaram, sem alteração de observador/tolerância. Todos os controles R5 foram
executados, inclusive após a falha mecânica histórica; não há estágios omitidos.

A qualificação aprova estas correções e a preservação do que já era aceito.
Dívidas de paridade de matemática, HTML e outros recursos permanecem abertas;
isto não certifica paridade integral do produto. Desempenho refere-se à máquina
medida. Sem commit ou push. O [diagnóstico anterior](../../lab/results/construction/diagnostics/readiness-fixes-2026-10-28/diagnostic-before-jpeg-recovery24.md)
conserva o estado reprovado anterior e todos os controles/recibos originais.
