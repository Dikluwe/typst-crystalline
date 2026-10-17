# Conjunto atual — ganho acumulado e distância para vanilla

A nova comparação simultânea mostra redução de 26,68% no tempo agregado do Tekt
frente ao binário anterior à prioridade 1, nos seis documentos com trabalho
preservado entre esses dois Tekt. O atual usa 10,64% menos tempo que vanilla nesse
recorte; antes usava 21,89% mais. Não é prova de superioridade universal.

## Proveniência e método

Nenhum fonte/L0 modificado. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
working tree não commitado. Recibos em `conjunto-prioridades-vanilla-2026-10-08/`:
`state.json.gz` guarda fontes/L0 completos, lista de alterações, diff HEAD --stat,
UTC e fixtures; `completion.json` verifica estado/inputs/binários inalterados.

- Baseline antes da prioridade 1, C2: `ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f`.
- Atual: `398b464d2a1062120928dfaf58351c43acd4a6769af74ac8ee1fc14f31533a14`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A baseline congelada continua byte-idêntica à coleta C2. Seus fontes/receitas
estão em `performance-c2-2026-10-07/{identity.json,consumer-inputs.json.gz,current-tested-terminal.json.gz}`;
`table-local-c2r1-2026-10-07/performance/protocol.md` a identifica como início P1.
Não é reconstrução aproximada por HEAD. Prioridade 6 representa somente o lote A
sintático já implementado; B/C continuam fora. O experimento Arc rejeitado não
está no atual. Não somamos percentuais isolados dos lotes anteriores.

Mesmos sete fixtures/bitmap da coleta C2, byte a byte, e font-path explícito da
baseline. Três rodadas de seis trios por fixture, todas as seis permutações dos
motores, uma warmup por motor/bloco: 378 amostras medidas, processos novos com
cache FS aquecido. Afinidade única, Rayon 1, vanilla jobs 1, epoch 0, LC_ALL C,
TZ UTC, PDF sem tags/pretty. Wall inclui launcher GNU time e lifecycle CLI;
wrapper idêntico nos três motores. %M vem do processo filho, sem pico herdado do
Python. Comandos, horários, fontes/fontconfig/config e fingerprints em
`bench.json.gz`. Monitor de bloqueadores antes de cada invocação; sem build/test/
GDB/lint concorrente. Verificação de saída após concluir os tempos.

A população é nova e compara todos os motores na mesma rodada, não razões entre
coletas históricas em momentos/launchers diferentes. Deltas são medianas das
razões pareadas. Medianas/médias não substituem uma estimativa causal por função.
Sem teste estatístico formal. A regra exploratória de aviso por drift >3% foi
inspirada no critério do scanner ADR-0115; não é extensão normativa dessa ADR
para toda CLI.

## Tempos completos — ms

| Documento | Antes P1 | Atual | Vanilla | Atual/antes, pareado | Atual/vanilla, pareado |
| --- | ---: | ---: | ---: | ---: | ---: |
| Texto simples | 156,63 | 111,63 | 131,50 | −28,77% | −15,18% |
| Contexto | 157,91 | 112,56 | 132,41 | −28,74% | −14,81% |
| Texto denso | 177,61 | 133,02 | 135,73 | −25,08% | −1,97% |
| Imagem JPEG | 9,43 | 9,68 | 11,19 | +3,29% | −15,38% |
| Formas | 13,00 | 13,02 | 12,27 | −0,74% | +5,51% |
| Table, aliases | 165,57 | 118,27 | 135,10 | −28,26% | −12,34% |
| Table, local Styled | 177,13 | 130,64 | 135,71 | não equivalente | −3,84% |

A table local da baseline tem a regressão de tamanho/posição corrigida em P1;
seu tempo anterior não autoriza ganho de desempenho para trabalho equivalente.
A comparação atual/vanilla passou o controle disponível de tamanho/posição.
Texto simples e aliases ficaram mais rápidos nas três rodadas contra baseline
e vanilla. Contra vanilla, ganhos de texto simples por bloco: 15,72/14,61/15,61%;
aliases: 12,39/12,59/11,87%. Denso: 1,74/2,06/2,77%, ganho pequeno consistente
nesta população. A variação entre medianas de rodada ficou abaixo de 3% em todos
os motores dos cinco documentos com texto. Imagem e formas tiveram spreads de
até 4,69% e duram poucos ms: seus resultados são indicativos; não certificar
pequenos deltas nesse recorte. Formas foi mais lento que vanilla nos três blocos.

## Resultado agregado

Agregado = soma construída de uma compilação fresca por caso em cada ciclo
round/pair, 18 ciclos. Não é um lote separado cronometrado nem distribuição de
uso real. Cada documento aparece uma vez; não ponderamos pelo uso do produto.
`aggregate.json` guarda ciclos e recortes.

Seis casos equivalentes anterior/atual, excluindo table local:

- Antes: mediana 682,00 ms; atual: 498,51 ms; vanilla: 558,19 ms.
- Atual/antes: −26,68%; blocos −26,78/−26,66/−26,55%.
- Antes/vanilla: +21,89%; atual/vanilla: −10,64%.

Contexto/denso ainda têm a ressalva de bbox vertical abaixo. No recorte de cinco
casos com mesma sequência de palavras/páginas/fontes e bbox até 0,0001pt contra
vanilla, o atual teve 383,08 ms contra 425,92 ms, −9,91% pareado. Inclui table
local corrigida; não usar a baseline errada para calcular ganho anterior nesse
recorte. Imagem/forma possuem palavras vazias; esse controle não cobre pixels.

Os sete juntos dão 629,28 ms atual contra 694,75 ms vanilla, −9,26% pareado, mas
misturam as ressalvas anteriores; registro de referência, não certificado geral.
Não somar este ganho ao agregado de 1,46% do lote de snapshots: aquela comparação
usou outros fixtures e baseline imediatamente anterior, não o início P1.

## Qualificação de saída e memória

`qualification.json.gz` compara textos sem normalizar whitespace, sequência de
palavras, páginas/dimensões, bboxes, família/tamanho/cor via fontspec, fontes e
raster 36 dpi. Os seis casos elegíveis preservaram anterior/atual em todos esses
controles; table local é a diferença funcional esperada do reparo.

Atual/vanilla: texto simples tem bbox máximo 0,000002pt, aliases/local até
0,000020pt; estilos de fontes/páginas e palavras coincidem. Aliases tem diferença
em whitespace extraído por pdftotext -layout: não normalizamos isso para declarar
paridade. Contexto/denso coincidem em textos/páginas/fontes, mas têm delta vertical
até 0,658006pt já presente na baseline. Imagem tem raster diferente; formas e
texto simples coincidem no raster medido. Resolução 36 dpi não prova renderização
universal. Comparação de lifecycle não substitui semântica/sintaxe/morfologia.

RSS dos documentos com texto: atual 43.116–46.732 KiB (42,11–45,64 MiB) versus
vanilla 29.848–31.682 KiB (29,15–30,94 MiB). Tekt ainda usa mais memória nesses
casos. Imagem/formas usam aproximadamente 19–20 MiB contra 21 MiB vanilla.
São picos de processo; não contagem de malloc, retenção ou atribuição causal.

Esta comparação mede compile completo fresco com FS aquecido. Não mede cold-disk,
watch/recompilação incremental ou ganho exclusivo de cada prioridade. O vanilla
continua com mecanismos incrementais distintos; não inferir resultado de watch
por estes números. Não houve commit, código novo, rebuild ou alteração de L0.
