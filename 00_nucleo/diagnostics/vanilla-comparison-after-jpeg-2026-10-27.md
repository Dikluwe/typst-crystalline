# Comparação geral após as otimizações JPEG — 2026-10-27

Comparação nova com o vanilla fixado **`64c5e8c02fe0394d913ead36e42ef8c1334d2107`**,
árvore `483bd6f1d8ac8810db47b65336d402647955fd17`. O código produtivo e L0
recebidos foram preservados integralmente. **Os 12 gates técnicos passaram**;
a comparação de linguagem/render continua parcial e identifica divergências.
Os resultados históricos contra `bd5ebec2` não foram renomeados como atuais.

## Proveniência e método

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, working tree não commitada. UTC, status, lista exata
`git diff HEAD --stat`, 2165 hashes e arquivos recebidos em
[baseline.json](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/baseline.json).
[Fontes congelados](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/received-source.json.gz),
recibo do novo build, hashes por execução e manifesto final permitem reproduzir
os números. Oráculo SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`; receita e verificação
de fonte/árvore ligadas em `oracle-provenance.json`, sem atualização do pin.

Release reconstruído: `2941a5bae502951c2028d3e6a218349e45a1a89512f43124fad854cb1dc47bce`;
debug: `64479782bbb969e014b82be6c54f93f35d6578dd8ee84e3fafc812a103f2d03d`. O release coincide byte a byte com o
encerramento JPEG recebido. Cargo/validação usam um job, cgroup 12 GiB,
MemoryHigh 8 GiB e zero swap. Benchmark sem build concorrente. A primeira
revisão automática para iniciar os comparadores expirou antes do processo;
a única repetição permitida iniciou normalmente. Nenhum gate foi dispensado.

Os critérios continuam separados: aceitação/diagnóstico; texto completo e
páginas físicas; geometria PDF/SVG a 0,001 pt; pintura independente; recursos.
Bytes de PDF não são critério de paridade da linguagem (ADR-0107/0108).
É uma tarefa diagnóstica: não houve nova implementação ou RED artificial;
os testes das correções anteriores foram executados novamente.

## Notas, linhas relativas e sobrescritos

Resultados por build, iguais entre debug e release. Contagens de corpora,
com possíveis repetições de fontes; não representam cobertura global da linguagem.

| Escopo | PDF qualificados/total | SVG qualificados/total |
|---|---:|---:|
| Nota inicial | 21/21 | 21/21 |
| Quebra terminal | 48/48 | 48/48 |
| R11 | 68/68 | 68/68 |
| R12 | 58/58 | 58/58 |
| R13 | 25/25 | 25/25 |
| R8 | 28/28 | 22/22 |
| R9 | 75/75 | 75/75 |
| R10 | 57/69 | 57/69 |

Os 440 diagnósticos stderr do produto nos corpora R11–R13 e das duas notas
posteriores foram conferidos contra
os recebidos, com normalização apenas do caminho da campanha, sem diferenças.
As duas omissões R11, entradas R12, comprimentos relativos R13 e correções de
quebra terminal/linha inicial vazia permanecem qualificadas. As três variantes
adicionais de linhas iniciais realmente vazias continuam geometricamente não
qualificadas, com caracteres completos; ficam separadas dos 32 casos antigos.

### Tracking: diferença exposta pelo pin novo

Os 15 controles originais de R10 passam. Na matriz de 54, **12 fontes com
tracking positivo/negativo não qualificam**: três famílias × dois tamanhos ×
dois sinais. Deltas PDF aproximadamente 0,180–0,240 pt. O comparador original
falhou e interrompeu o encadeamento R8–R10; o registro foi preservado.
`finish-observations.json` completa os estágios restantes sem alterar predicados
ou tolerâncias; falhas SVG também permanecem dados, sem falso sucesso.

A triangulação dos mesmos 12 fontes mostra Tekt qualificado contra o oráculo
anterior `bd5ebec2` e não qualificado contra `64c5e8c0`. O patch upstream
ratificado contém mudanças de tracking em shaping/final de linha. **Medido:**
a divergência é exposta pela troca de referência e não exige JPEG na entrada.
**Inferência limitada:** o novo tratamento de tracking é o candidato causal;
um diagnóstico de causa exata exige isolamento próprio antes de corrigir.
Uma diferença igual contra o oráculo antigo refutaria esta atribuição; ela não
ocorreu. Fonte e números em `tracking-pin-delta/receipt.json`;
[exemplo de tracking](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/r8-r10/r10-matrix-final-release/DejaVu%20Sans-11pt-tracking-plus.typ).

### Os 15 dos 32 refutadores antigos que permanecem abertos

Seis fontes combinam argumentos públicos `typographic`/`baseline` de `super`
rejeitados pelo Tekt; nove envolvem scripts aninhados, Box ou quebra em link.
As nove sondas isoladas preservam seis rejeições de argumentos e três controles
`size` qualificados; não são nove defeitos adicionais independentes.

| Fonte | Resultado PDF remanescente |
|---|---|
| [DejaVu Sans-forced-synthesis](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/DejaVu%20Sans-forced-synthesis.typ) | compile-outcome |
| [DejaVu Sans-explicit-baseline](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/DejaVu%20Sans-explicit-baseline.typ) | compile-outcome |
| [DejaVu Sans-nested-super](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/DejaVu%20Sans-nested-super.typ) | 0.173812 pt |
| [DejaVu Sans-nested-sub](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/DejaVu%20Sans-nested-sub.typ) | 3.097485 pt |
| [DejaVu Sans-boxed-break](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/DejaVu%20Sans-boxed-break.typ) | 2.147092 pt |
| [Nimbus Sans-forced-synthesis](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Nimbus%20Sans-forced-synthesis.typ) | compile-outcome |
| [Nimbus Sans-explicit-baseline](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Nimbus%20Sans-explicit-baseline.typ) | compile-outcome |
| [Nimbus Sans-nested-super](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Nimbus%20Sans-nested-super.typ) | 0.152900 pt |
| [Nimbus Sans-nested-sub](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Nimbus%20Sans-nested-sub.typ) | 2.655940 pt |
| [Nimbus Sans-linked-break](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Nimbus%20Sans-linked-break.typ) | páginas/segmentação |
| [Nimbus Sans-boxed-break](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Nimbus%20Sans-boxed-break.typ) | páginas/segmentação |
| [Liberation Serif-forced-synthesis](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Liberation%20Serif-forced-synthesis.typ) | compile-outcome |
| [Liberation Serif-explicit-baseline](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Liberation%20Serif-explicit-baseline.typ) | compile-outcome |
| [Liberation Serif-nested-sub](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Liberation%20Serif-nested-sub.typ) | páginas/segmentação |
| [Liberation Serif-boxed-break](../../lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/refuters-release/Liberation%20Serif-boxed-break.typ) | 2.503193 pt |

São fontes de teste, não 15 causas independentes nem o total global de defeitos.
As notas com linha inicial vazia/quebra terminal já não estão nessa lista.

## Matriz, símbolos e limites anteriores

| Build / perfil | MATCH | Desabilitados | DIFFERENCE | VIOLATED |
|---|---:|---:|---:|---:|
| debug / default | 27 | 14 | 2 | 0 |
| debug / html+bundle | 40 | 1 | 2 | 0 |
| release / a11y-extras | 28 | 13 | 2 | 0 |
| release / bundle | 31 | 10 | 2 | 0 |
| release / default | 27 | 14 | 2 | 0 |
| release / html+a11y | 35 | 6 | 2 | 0 |
| release / html+bundle | 40 | 1 | 2 | 0 |
| release / html | 34 | 7 | 2 | 0 |

As diferenças observáveis de raster/nome interno de fonte continuam separadas
de violações de linguagem. Casos desabilitados não recebem crédito de paridade.
As sondas de linguagem e os 198 símbolos foram repetidos nos dois builds;
162 MATCH e 36 EXTENSION no inventário de símbolos.

Os 21 replays A–H/R1–R7 e cinco suplementos passaram. Permanecem limites
explícitos nos recibos frescos: R2 (justificação/overhang, contextos recusados e
tracking não zero), R3 (markup/base/VS e quebra matemática explícita), R4
(geometria de accent/class/equation; campos/conversões preservados) e R5 (pintura).
Não se declara paridade universal a partir desses controles finitos.

## Pintura

Todos os casos que compilam nos 32 refutadores foram rasterizados por página;
controles preselecionados também a 600 dpi. Há resíduos de antialiasing mesmo
em fontes geometricamente qualificadas. A diferença pequena não foi tolerância
adicionada nem prova de identidade de pixels. Notas e separadores têm recibos
próprios, com consistência debug/release.

Diferenças R5 atuais, pixels alterados, medidas separadamente de geometria:

| Fonte | PDF 72 dpi | SVG 72 dpi | PDF 600 dpi | SVG 600 dpi |
|---|---:|---:|---:|---:|
| different | 26 | 172 | 1704 | 2911 |
| fill | 0 | 0 | 0 | 12 |
| gutter | 562 | 1006 | 33664 | 37241 |
| gradient-line | 556 | 473 | 32490 | 28178 |
| gradient-table | 1482 | 1968 | 102675 | 107172 |
| tiling-line | 0 | 0 | 0 | 105 |
| tiling-local | 0 | 0 | 0 | 38 |

Os contornos de `gutter` foram também inspecionados visualmente nesta rodada.
Pintura preparada e compacta conservaram seus gates. PNG nativo, gradientes e
outros resíduos estão em `revalidation/after-release/receipt.json`; não foram
apagados do diagnóstico por os testes técnicos passarem.

## JPEG: resultados novos contra vanilla

Mesma receita, fixtures congelados, processos novos e ordens alternadas;
61 pares no gradiente e 31 nos outros dois casos, dois warmups por papel/caso.
Tempo é CLI completo, não somente decoder. RSS em KiB; medianas:

| Caso | Tekt / vanilla, ms | Tempo relativo | Tekt / vanilla, KiB | RSS relativo |
|---|---:|---:|---:|---:|
| Cinza com gradiente | 97.22 / 104.35 | -6.8% | 28852 / 61656 | -53.2% |
| Cinza uniforme | 31.09 / 30.66 | +1.4% | 17436 / 46100 | -62.2% |
| RGB grande | 45.71 / 68.45 | -33.2% | 17616 / 93284 | -81.1% |

O antigo +16,4% e seu residual de +13,2% eram contra versões anteriores do
Tekt, não contra vanilla. A campanha de quatro releases foi repetida com
936 compilações em 20 fontes; os gates congelados de tempo,
IC pareado e RSS estão em `jpeg/resource-gates.json` (aprovados).
A decodificação completa, limites, ausência de destino integral e deduplicação
por Arc continuam cobertos pelos testes. O replay de compatibilidade executou
17922 comparações com o legado, sem diferenças;
isso não é uma comparação de qualquer entrada possível com vanilla.

No replay direto dos 20 fontes: 16 positivos bilaterais;
mesmos códigos de saída em todos: **não**. Divergências de aceitação:
**bad-entropy, no-eoi**; saídas não qualificadas por qpdf:
**bad-entropy, no-eoi**. O primeiro observador interrompeu
ao presumir que toda compilação bem-sucedida passaria qpdf; a tentativa original
está arquivada em `jpeg/observer-attempt`. A observação completa conserva esses
resultados como não qualificados, sem relaxar o gate de validade. Diferenças de páginas físicas:
**shared-rgb-16**; sequência global de texto:
**nenhuma diferença**. A lista por página difere em
**shared-rgb-16**, por haver uma página versus duas com
texto vazio; isso não é omissão de caracteres. Diferenças de raster:
**cmyk, distinct-rgb-4, gray-gradient, gray-horizontal, gray-vertical, shared-rgb-16, shared-rgb-pages**. Diferenças de stderr:
**bad-entropy, bad-header, no-eoi, oversized**. Os erros ficam em seu eixo próprio,
sem confundir rejeição bilateral com identidade de diagnóstico.
`jpeg/observables/receipt.json` conserva qpdf, páginas, texto e todos os rasters.

Diferenças de raster com página comum observada (900 px), sem tolerância adicionada:

| Fonte | Pixels diferentes somados | Maior delta de canal |
|---|---:|---:|
| cmyk | 13872 | 62 |
| distinct-rgb-4 | 10608 | 164 |
| gray-gradient | 60322 | 157 |
| gray-horizontal | 602 | 255 |
| gray-vertical | 60802 | 255 |
| shared-rgb-16 | 23120 | 164 |
| shared-rgb-pages | 42432 | 164 |

`shared-rgb-16` gera **uma página no Tekt e duas no vanilla**. O controle da
alteração 2 do Tekt também gera uma página; a comparação dos quatro releases
preserva todos os 249 rasters/textos/JPEGs extraídos. Isso mostra um limite de
layout já presente, sem regressão causada pela última otimização de validação.
Quando as páginas diferem, o raster da primeira página não qualifica o documento.

Os payloads JPEG originais estão presentes nos PDFs dos dois compiladores.
No gradiente, as matrizes medidas são quase iguais (escala 100 × 66,66667 pt,
posição 5 / 128,33333 pt), com diferenças abaixo de 0,001 pt; o inventário
mostra DeviceGray no Tekt e ICC de uma componente no vanilla. Esses fatos
não isolam a causa do raster. Não se infere perda de decodificação nem se copia
a serialização do oracle como requisito. `jpeg/image-differences.json` conserva
streams, inventários e hashes. A pintura e a quebra das imagens ficam abertas.

Em `bad-entropy` e `no-eoi`, o Tekt rejeita a entrada; o vanilla termina 0,
mas qpdf termina 3 e relata dados JPEG inválidos na saída. É divergência de
aceitação/robustez observada, sem inferir intenção do upstream. Não autoriza
retirar a decodificação completa exigida pelo L0. Os outros dois negativos
são rejeitados bilateralmente; os quatro têm diferenças de diagnóstico.

A amostra sintética e a máquina não estabelecem superioridade universal.

Os reprodutores das issues #8507, #8965, #8966, #8984 e #9015 também foram
executados novamente. Os recibos mantêm aceitação, XML/referências, atributos
HTML e qpdf; não certificam visual integral ou texto fora da página por um
mero sucesso de compilação. Resultados em `issues/bilateral-results.json`.

## Recursos PDF gerais

Sete cargas; três rodadas × três processos por papel/carga, dois warmups.
Tempo GNU tem resolução de 0,01 s: os valores abaixo são medianas de contexto,
sem IC de superioridade de tempo geral.

| Carga | Tekt / vanilla, ms | Tekt / vanilla, KiB | RSS relativo |
|---|---:|---:|---:|
| empty | 90 / 120 | 26884 / 27284 | -1.5% |
| hello | 90 / 120 | 30592 / 31184 | -1.9% |
| prose | 120 / 130 | 37268 / 34848 | +6.9% |
| math | 160 / 130 | 35580 / 34024 | +4.6% |
| table | 100 / 130 | 31924 / 33548 | -4.8% |
| multipage | 100 / 120 | 31168 / 36092 | -13.6% |
| context | 100 / 120 | 31796 / 34780 | -8.6% |

Média geométrica RSS Tekt/vanilla: **0.970712**
(-2.9%). ELF original:
51005664 / 52288088 bytes (-2.45%).
Cópias normalizadas por strip: 43039904 /
52288080 bytes (**-17.69%**).
Originais e símbolos foram preservados. Esses dois tamanhos não são intercambiáveis.

Os critérios R6 originais passaram: razão normalizada contra R5
0.860247 ≤0,95 e abaixo do vanilla; razão RSS geométrica contra
R5 0.953129 ≤0,97; limites por carga/repetibilidade e
matemática contra z+shared. Os três stresses históricos foram repetidos.
Não se promete que cada carga use menos memória ou tempo que vanilla.

## Validação, preservação e próximos limites

- 12 gates, **9972 testes Rust aprovados, 7 ignorados, 42 Python aprovados**;
  build, doctests, all-targets, Clippy, fmt e rustdoc, ownership/Núcleos e
  linhagem estrita/reversa aprovados; linter completo sem warning/error.
- Comparadores R10 retêm as falhas de tracking; validação técnica verde não
  converte essas fontes em qualificadas. Fontes completos, tolerâncias e
  diagnósticos foram conservados.
- **2165 inputs produtivos/L0 preservados**. 159427
  arquivos dos manifestos históricos selados e 1853 assets
  foram conferidos. Arquivos recebidos permanecem, com apenas uma entrada
  aditiva no índice e este novo diagnóstico. Não houve commit/push.
- Permanecem os 15/32 refutadores antigos, as três linhas vazias repetidas,
  os 12 tracking da matriz R10, layout/pintura de imagens, aceitação/diagnóstico
  de JPEG inválido e os limites R2/R3/R4/R5 citados. Não somar
  esses corpora como número global de bugs. Escopos não medidos continuam abertos.

Evidências e execução reproduzível:
`lab/results/tekt-vs-vanilla-after-jpeg-2026-10-27/`.
Fechamento do texto: `2026-10-27T12:49:01.135987+00:00` (datas locais America/Sao_Paulo, recibos UTC).
