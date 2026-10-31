# Comparação com o vanilla fixado após R11–R13 — 2026-10-25

> Atualização posterior: a [correção da quebra terminal em notas](terminal-footnote-break-2026-10-26.md) qualifica mais um dos 32 refutadores; restam 16. Este relatório conserva os números da rodada anterior.

**R11–R13 foram reconfirmados em uma nova rodada, em debug e release, sem
alterar código produtivo ou L0.** As duas omissões de texto de R11, as
configurações de entrada de R12 e os três separadores percentuais de R13
continuam corrigidos nos seus reprodutores originais.

Dos 32 refutadores adicionados após R8–R10, **15 qualificam e 17 não qualificam**
em geometria PDF/SVG a 0,001 pt, com resultados iguais nos dois builds.
São 17 fontes de teste, não 17 causas independentes nem o total global de
divergências do projeto. As três fontes adicionais de R12 com
`line(length:50%)`, externas a esses 32 casos, agora qualificam.
Nove novas fontes isolam argumentos de `super`, sem esconder as rejeições dos
casos combinados. Pintura, geometria, aceitação e desempenho têm gates distintos.

## Estado e proveniência

- HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, working tree não commitado, recebido após R13.
  [baseline.json](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/baseline.json) registra UTC, status, o stat exato de
  `git diff HEAD --stat`, documentos recebidos e os 2165 hashes de entrada;
  [received-source.json.gz](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/received-source.json.gz) preserva seus bytes.
  Os inputs coincidem com o encerramento de R13 e ficaram iguais nesta rodada.
- Vanilla fixado: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
  `cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256
  `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
  A referência é a receita ratificada de 2026-10-17; não houve resync.
- Houve nova validação completa, novo build release e novas execuções dos
  comparadores. Resultados anteriores são referências identificadas, não execuções
  renomeadas como atuais. Os estágios e horários estão em
  [pipeline.json](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/pipeline.json) e [post.json](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/post.json).
- SHA-256 final debug: `6a80387ccdd7b10c722a3580ae9c06ef7d2ad5b697307bcc364c846287ce2d77`;
  release: `5700cc81dfe2fa556a55e181d7b43e9c39ad010d770d60e2f222f475e8542aa4`. O build arquiva os fontes
  completos em [final-build/source.json.gz](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/revalidation/final-build/source.json.gz).
  O [encerramento](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/completion.json) sela todos os arquivos desta rodada.

## R11–R13 e controles anteriores

| Escopo | PDF por build | SVG por build | Resultado no corpus original |
|---|---:|---:|---|
| R11 — notas literais e aninhadas | 68 | 68 | Qualificado; texto completo por página |
| R12 — entradas | 54 | 54 | Qualificado |
| R12 — fronteiras de fluxo | 4 | 4 | Qualificado |
| R13 — comprimentos relativos/mistos/em | 25 | 25 | Qualificado, incluindo os três fontes originais de R12 |
| R8 | 28 | 22 | Preservado |
| R9 | 75 | 75 | Preservado |
| R10 | 69 | 69 | Preservado, com controles bitmap independentes |

O [audit.json](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/audit.json) confere igualdade dos hashes de cada fonte
com o corpus selado de R13. A [auditoria detalhada](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/closing-audit.json)
verifica dimensões físicas, palavras completas por página, bounding boxes e
origens/primitivas independentes do SVG. Não há aceitação por igualdade de bytes.
Os oito controles de erros de linha foram repetidos nos dois builds; suas
mensagens e colunas não são declaradas universalmente idênticas ao oracle.
Os warnings conhecidos de R11 foram retidos com a mesma proveniência.

As falhas assertionais RED→GREEN pertencem às implementações R11–R13 e permanecem
em seus recibos históricos. Nesta comparação, os dez testes R13 e os demais
testes foram executados novamente sobre os mesmos inputs; não se inventou um
novo RED para uma tarefa exclusivamente diagnóstica.

## Os 17 casos adicionais ainda abertos

| Grupo | Fontes não qualificadas | Observação |
|---|---:|---|
| Nota com linha inicial vazia ou quebra terminal | 2 | Posição ou distribuição por páginas |
| Argumentos públicos combinados de `super` | 6 | Três fontes × duas formas rejeitadas |
| Composição de scripts | 9 | Aninhamento, Box e quebra dentro de link |

Os cinco casos que passaram a qualificar desde o snapshot após R8–R10 são:
`note-entry-indent`, `note-entry-gap`, `note-entry-no-separator`, `note-long-literal-breaks`, `note-long-nested`.
As correções de R11/R12 permanecem fechadas; o limite percentual separado foi
fechado por R13. Os recibos completos
[debug](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-debug/receipt.json) e
[release](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/receipt.json) repetem os 32 fontes originais
sem mudanças de entrada ou tolerância.

| Reprodutor remanescente | Delta PDF em pt ou classificação | SVG qualificado |
|---|---|---|
| [DejaVu Sans-forced-synthesis](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/DejaVu%20Sans-forced-synthesis.typ) | rejeição de compilação | não |
| [DejaVu Sans-explicit-baseline](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/DejaVu%20Sans-explicit-baseline.typ) | rejeição de compilação | não |
| [DejaVu Sans-nested-super](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/DejaVu%20Sans-nested-super.typ) | 0.173812 | não |
| [DejaVu Sans-nested-sub](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/DejaVu%20Sans-nested-sub.typ) | 3.097485 | não |
| [DejaVu Sans-boxed-break](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/DejaVu%20Sans-boxed-break.typ) | 2.147092 | não |
| [Nimbus Sans-forced-synthesis](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Nimbus%20Sans-forced-synthesis.typ) | rejeição de compilação | não |
| [Nimbus Sans-explicit-baseline](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Nimbus%20Sans-explicit-baseline.typ) | rejeição de compilação | não |
| [Nimbus Sans-nested-super](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Nimbus%20Sans-nested-super.typ) | 0.152900 | não |
| [Nimbus Sans-nested-sub](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Nimbus%20Sans-nested-sub.typ) | 2.655940 | não |
| [Nimbus Sans-linked-break](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Nimbus%20Sans-linked-break.typ) | distribuição/segmentação | não |
| [Nimbus Sans-boxed-break](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Nimbus%20Sans-boxed-break.typ) | distribuição/segmentação | não |
| [Liberation Serif-forced-synthesis](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Liberation%20Serif-forced-synthesis.typ) | rejeição de compilação | não |
| [Liberation Serif-explicit-baseline](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Liberation%20Serif-explicit-baseline.typ) | rejeição de compilação | não |
| [Liberation Serif-nested-sub](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Liberation%20Serif-nested-sub.typ) | distribuição/segmentação | não |
| [Liberation Serif-boxed-break](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/Liberation%20Serif-boxed-break.typ) | 2.503193 | não |
| [note-long-leading-blank](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/note-long-leading-blank.typ) | 1.403865 | não |
| [note-long-trailing-break](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuters-release/note-long-trailing-break.typ) | distribuição/segmentação | não |

Na nota com linha inicial vazia, o texto e as quatro páginas são preservados,
mas resta deslocamento geométrico. A quebra terminal mantém quatro páginas no
produto contra três no vanilla. Isso não reabre as omissões de texto literal e
aninhado já corrigidas. Scripts com segmentação diferente não recebem um delta
numérico fictício nem são chamados de omissão sem observar os caracteres.

### Isolamento dos argumentos públicos

Os casos antigos combinavam `typographic`, `baseline` e `size`; uma rejeição
antecipada não isolava o comportamento de cada argumento. As nove fontes novas
foram executadas nos dois builds, com `typographic` e `baseline` separados e
`size` como controle. Os hashes, comandos e resultados estão em
[additional-debug](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/additional-debug/receipt.json) e
[additional-release](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/additional-release/receipt.json).

| Fonte | Saídas produto PDF/SVG | Saídas vanilla PDF/SVG | Geometria qualificada |
|---|---|---|---|
| DejaVu Sans-typographic-only | [1, 1] | [0, 0] | não |
| DejaVu Sans-baseline-only | [1, 1] | [0, 0] | não |
| DejaVu Sans-size-control | [0, 0] | [0, 0] | sim |
| Nimbus Sans-typographic-only | [1, 1] | [0, 0] | não |
| Nimbus Sans-baseline-only | [1, 1] | [0, 0] | não |
| Nimbus Sans-size-control | [0, 0] | [0, 0] | sim |
| Liberation Serif-typographic-only | [1, 1] | [0, 0] | não |
| Liberation Serif-baseline-only | [1, 1] | [0, 0] | não |
| Liberation Serif-size-control | [0, 0] | [0, 0] | sim |

Os códigos 0 e 1 indicam sucesso e erro de compilação, respectivamente. Essas
sondas isolam a aceitação dos argumentos; o comportamento de renderização de
`baseline` após corrigida a rejeição ainda requer medição própria.

O [levantamento de fontes](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/source-evidence.json) conserva o L0 e o
construtor atual, além do contrato da referência fixada. Ele distingue a superfície
observada de hipóteses de causa interna. Essas fontes suplementares não são
somadas como nove defeitos independentes aos 17 reprodutores antigos.

## Pintura medida separadamente

O [recibo dos 32 refutadores](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/refuter-paint/receipt.json) retém a pintura
de todos os casos que compilam em ambos os executáveis, em PDF/SVG a 72 dpi;
quatro controles preselecionados também a 600 dpi. Inclui contagem de páginas,
palavras e multiconjuntos de caracteres globais/por página. Debug e release
têm as mesmas observações. Os casos rejeitados não recebem uma falsa
qualificação de pintura.

Resíduos de pintura entre os casos geometricamente qualificados:

- `Liberation Serif-nested-super`, SVG 72 dpi: página 1, 2 pixels, delta de canal 1.
- `note-entry-indent`, SVG 72 dpi: página 1, 1 pixels, delta de canal 1.
- `note-entry-gap`, SVG 72 dpi: página 1, 2 pixels, delta de canal 1.
- `note-entry-clearance`, SVG 72 dpi: página 1, 3 pixels, delta de canal 1.
- `note-entry-no-separator`, SVG 72 dpi: página 1, 3 pixels, delta de canal 1.
- `note-long-literal-breaks`, PDF 600 dpi: página 1, 0 pixels, delta de canal 0, página 2, 2 pixels, delta de canal 11, página 3, 2 pixels, delta de canal 11.

O [recibo de pintura R13](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/line-paint/receipt.json) mede oito fontes por
build a 72/600 dpi. Os separadores percentuais preservam os rasters das
equivalentes linhas absolutas de R12; o resíduo de dois pixels SVG em DejaVu Sans
a 72 dpi continua explicitamente registrado. Linha zero com cap butt tem sua
ausência de pintura verificada independentemente da representação SVG.

## Demais divergências reconferidas

Os 21 replays e cinco suplementos anteriores passaram e foram auditados nesta
rodada. Os limites externos aos 32 casos continuam separados:

- R2: quatro variantes justificadas com sobreposição óptica e três contextos
  recusados, além das fronteiras com tracking não zero. Os controles qualificados
  e `overhang:false` mantêm seus critérios.
- R3: `control-markup-base`, `control-markup-vs` e
  `control-math-explicit-break` continuam limites de geometria/fallback.
- R4: os 210 controles de campos/conversão continuam dentro dos critérios;
  `accent`, `class` e `equation` continuam limites geométricos.
- R5: diferenças de pintura em `different`, `fill`, `gutter`, gradientes,
  tiling e PNG nativo permanecem quantificadas nos recibos frescos. Não se
  converte byte/estrutura de PDF em requisito de paridade da linguagem.
- R7: 198 símbolos conferidos em cada build, mantendo 162 MATCH e 36 EXTENSION.
  Não há falha remanescente no gate declarado de R7.

Os detalhes estão em [auditoria A–H/R1–R7](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/revalidation/audit.json),
[pintura R5](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/revalidation/after-release/receipt.json) e
[pintura preparada](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/revalidation/prepared-paint-audit.json).
| Pintura R5 | PDF 72 dpi | SVG 72 dpi | PDF 600 dpi | SVG 600 dpi | Maior delta de canal |
|---|---:|---:|---:|---:|---:|
| different | 26 | 172 | 1704 | 2911 | 255 |
| fill | 0 | 0 | 0 | 12 | 1 |
| gutter | 562 | 1006 | 33664 | 37241 | 255 |
| gradient-line | 556 | 473 | 32490 | 28178 | 148 |
| gradient-table | 1482 | 1968 | 102675 | 107172 | 255 |
| tiling-line | 0 | 0 | 0 | 105 | 1 |
| tiling-local | 0 | 0 | 0 | 38 | 1 |

As contagens de pintura acima vêm dos renders desta rodada e mantêm a separação
entre pintura e geometria. O PNG nativo e seus deltas estão no mesmo recibo.

Regiões infinitas/auto e generalização da reflexão de linhas não recebem
certificação universal a partir deste corpus finito.

## Recursos e latência atuais

Os sete workloads repetem a receita R6 e os gates originais: tamanho normalizado
≤95% de R5 e ≤oracle; média geométrica RSS ≤97% de R5, cada caso ≤103%,
variação entre medianas de rodadas ≤3%. Todos passaram, assim como os três
stresses. Razão de tamanho normalizado: **0.855764449**;
razão geométrica RSS: **0.949260301**.
O controle histórico é o executável R5 SHA-256
`c3d2d2e416e506f9e49bff28d55a05138030440a6288d88bc77631348dae5549`;
o HEAD desta medição não é apresentado como o HEAD de construção desse controle.
Não se atribui ganho causal desta rodada a R11, R12 ou R13.

| PDF | Atual KiB | Vanilla KiB | Atual / vanilla |
|---|---:|---:|---:|
| empty | 26816 | 27328 | 0.981265 |
| hello | 30560 | 31164 | 0.980619 |
| prose | 37284 | 34880 | 1.068922 |
| math | 35484 | 34164 | 1.038637 |
| table | 31556 | 33716 | 0.935935 |
| multipage | 30764 | 36572 | 0.841190 |
| context | 31488 | 35076 | 0.897708 |

RSS acima do vanilla nesta medição: `prose`, `math`.
Tamanho original: 50746088 bytes no produto e
52271128 no vanilla; normalizado por strip em cópias:
42815648 e 52271120 bytes.
[Recursos](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/revalidation/final-resources/receipt.json) e
[stresses](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/revalidation/legacy-resources/receipt.json) conservam amostras,
referências, geometrias e repetibilidade.

O [benchmark](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/benchmark/receipt.json) usa 20 pares de processos novos,
duas execuções de aquecimento por papel, ordem balanceada/randomizada e IC95%
por bootstrap pareado. A razão é tempo vanilla / atual; vantagem só é creditada
se todo o IC exceder 1,05 e a saída qualificar. Os dois perfis têm gates
independentes de saída. Não é medida de cache de processo/watch.

| Carga | Atual ms, padrão | Vanilla ms, padrão | Razão padrão [IC95%] | Razão 1 CPU [IC95%] | Saída |
|---|---:|---:|---|---|---|
| empty | 90.602 | 114.959 | 1.265 [1.257; 1.285] | 1.279 [1.276; 1.288] | sim |
| hello | 92.970 | 116.914 | 1.253 [1.246; 1.269] | 1.268 [1.262; 1.279] | sim |
| prose | 117.921 | 123.534 | 1.049 [1.032; 1.056] | 1.057 [1.049; 1.063] | sim |
| math | 153.756 | 120.911 | 0.783 [0.778; 0.789] | 0.788 [0.780; 0.790] | sim |
| table | 100.492 | 121.824 | 1.214 [1.201; 1.228] | 1.233 [1.225; 1.244] | sim |
| multipage | 102.702 | 126.507 | 1.229 [1.224; 1.241] | 1.238 [1.233; 1.247] | sim |
| context | 96.889 | 119.240 | 1.223 [1.217; 1.246] | 1.239 [1.227; 1.243] | sim |
| eval-startup | 3.981 | 5.213 | 1.310 [1.284; 1.380] | 1.303 [1.282; 1.351] | sim |
| eval-fold | 10.700 | 17.765 | 1.660 [1.629; 1.699] | 1.622 [1.582; 1.674] | sim |
| eval-map | 4.718 | 6.481 | 1.368 [1.354; 1.416] | 1.359 [1.332; 1.388] | sim |

Vantagem creditada: 8/10 cargas no
perfil padrão e 8/10 com uma CPU.
Lentidão estável: padrão ['math']; uma CPU ['math'].
Ausência de vantagem creditada não é prova de igualdade de desempenho.

## Validação e preservação

Os 12 gates foram executados novamente e passaram: **9946 testes Rust aprovados,
sete ignorados e 42 testes Python**; workspace build, todos os targets, Clippy,
fmt, rustdoc, linhagem estrita/reversa e verificações estruturais. O lint completo
terminou sem violações. Os comandos e o estado congelado estão em
[validation-complete](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/validation-complete/validation-commands.json).

O [recibo de preservação](../../lab/results/tekt-vs-vanilla-after-r11-r13-2026-10-25/preservation.json) confere os 2165 inputs
recebidos e 97097 arquivos históricos, incluindo todo o encerramento de R13.
Os assets/fontes permaneceram iguais. A licença DejaVu Sans já normalizada em
HEAD212 foi conferida contra o próprio HEAD; não é uma alteração desta rodada.
A primeira geração da tabela R5 parou ao procurar os nomes dos controles de
gradiente/tiling no recibo de pintura geral. O gerador foi ajustado para ler o
recibo de pintura preparada; a tentativa e o script anteriores foram retidos.
Nenhuma medição ou critério foi alterado por essa correção documental.

As alterações existentes de R11/R12/R13 foram preservadas. Apenas diagnóstico
e ferramentas/artefatos locais de medição foram adicionados ou atualizados.
