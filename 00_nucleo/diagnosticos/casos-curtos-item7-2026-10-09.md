# Item 7 — texto denso, tabela local e formas em documentos maiores

## Estado e método

Medição nova em `2026-10-09T11:16:55.549717+00:00` a `11:17:50.300336+00:00`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. O [recibo de evidências](casos-curtos-item7-2026-10-09/evidence.json.gz) conserva o status integral, diff stat, fontes e perfis capturados em cada etapa. O [JSON das medições](casos-curtos-item7-2026-10-09/measurements.json.gz) registra hashes das nove entradas, fontes, scripts e binários, comandos, ambiente, CPU, horário e 324 amostras. Tekt release SHA-256 `076633d63083aaeed0f51443960350412bb6672457d3841629681d8439a8273f`; vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A identidade da árvore e receita vanilla está no P1489; a string de versão não serviu de prova.

O [gerador](casos-curtos-item7-2026-10-09/generate.py) amplia as fixtures anteriores: texto denso com 32/128/512 parágrafos iguais, tabela local com 100/400/1.600 células, e formas com 128 objetos por página em 1/4/16 páginas. Repetir a página de formas mantém as coordenadas dentro da página. As nove entradas geradas e a sonda inicial estão no recibo. A versão 1× só acrescenta newline final nos arquivos de texto/formas; o conteúdo exibido é o mesmo.

O [medidor](casos-curtos-item7-2026-10-09/measure.py) usou processos CLI novos, filesystem aquecido, `RAYON_NUM_THREADS=1`, CPU fixa, `SOURCE_DATE_EPOCH=0`, `--jobs 1` no vanilla e o mesmo conjunto de fontes. Para cada caso: três blocos, um par de aquecimento por bloco e seis pares medidos por bloco, com ordem alternada dos motores. Wall inclui o wrapper comum `/usr/bin/time`; `%M` é pico RSS do processo, não bytes vivos do heap. As medianas de tempo/RSS são por motor; percentuais são medianas das **18 razões pareadas**, não a razão das medianas.

## Saída e qualificação

A [qualificação](casos-curtos-item7-2026-10-09/qualification.json.gz) foi feita em `2026-10-09`; seu estado do Git está no recibo. Nos nove casos, texto extraído, sequência de palavras, número/dimensões de páginas e estilos de fontes coincidem. Desvio máximo das caixas de palavras: `0,00003 pt` em texto denso, `0,000024 pt` na tabela. As formas têm raster idêntico em todas as páginas a 36 dpi.

O raster de texto e tabela já diverge em 1× e continua a divergir nas escalas maiores. A medição de pixels, no recibo com seu estado do Git, encontrou aproximadamente `1,09–1,13%` de pixels diferentes no texto e `0,25–0,31%` na tabela, a 36 dpi; as frações mudam pouco com a escala. Portanto as comparações de texto/tabela têm conteúdo e geometria equivalentes nos controles medidos, **com ressalva de raster**, sem alegar igualdade visual integral ou identidade de bytes PDF.

## Tempos e memória

| Caso | Páginas | Wall Tekt / vanilla (ms) | Variação pareada | RSS Tekt / vanilla (MiB) | Variação pareada |
|---|---:|---:|---:|---:|---:|
| texto denso 1× | 2 | 121,46 / 127,51 | −4,5% | 29,62 / 30,47 | −2,6% |
| texto denso 4× | 8 | 188,68 / 138,71 | +35,8% | 41,07 / 33,24 | +23,5% |
| texto denso 16× | 31 | 451,34 / 174,44 | +159,6% | 85,94 / 45,66 | +88,2% |
| tabela local 1× | 1 | 121,90 / 128,22 | −5,6% | 29,81 / 30,94 | −3,6% |
| tabela local 4× | 4 | 182,87 / 136,95 | +34,4% | 38,30 / 33,83 | +12,7% |
| tabela local 16× | 13 | 520,83 / 170,61 | +205,8% | 74,53 / 45,31 | +64,6% |
| formas 1× | 1 | 10,58 / 10,03 | +4,4% | 19,70 / 21,17 | −7,7% |
| formas 4× | 4 | 16,48 / 12,79 | +29,2% | 20,34 / 22,96 | −11,8% |
| formas 16× | 16 | 41,39 / 24,97 | +63,7% | 23,36 / 28,56 | −18,2% |

O sinal e a ordem de grandeza da desvantagem de tempo nos casos 4×/16× repetiram-se nos três blocos. Variação pareada de wall por bloco em 16×: texto `+159,8/+160,6/+158,4%`; tabela `+216,0/+205,4/+205,4%`; formas `+62,1/+64,3/+64,9%`. As faixas de diferença de RSS nos 18 pares 16× foram texto `+40.832…+41.624 KiB`, tabela `+29.464…+30.780 KiB` e formas `−5.804…−4.780 KiB`. A diferença de 1× das formas é submilissegundo e não é evidência robusta isoladamente; o crescimento de 4× e 16× é persistente nesta bateria.

## Localização inicial e decisão

A sonda `--timings-json`, incluída no recibo, fez três compilações por caso com o mesmo binário, fixtures e ambiente. São tempos **internos do Tekt**, não fases diretamente comparáveis com o vanilla nem prova causal. Entre 1× e 16×, no texto denso `layout` passou de `9,03` a `155,75 ms`, `shape` de `8,66` a `133,17 ms` e `render` de `3,91` a `57,76 ms`; `eval` permaneceu perto de `94 ms`. Na tabela, `layout` passou de `14,06` a `295,54 ms` e `expand_context` de `1,27` a `56,18 ms`. Nas formas, `eval` passou de `3,93` a `22,52 ms`, com raster igual e RSS menor que o vanilla.

**Decisão:** os custos de escala do Tekt atual em texto denso, tabela local e tempo de formas são persistentes nesta população, não apenas variação de um caso curto. A memória das formas segue favorável ao Tekt neste corpus. O experimento não atribui esses custos à mudança do `Value` compacto nem reavalia seu pequeno delta antes/depois: o binário Tekt anterior àquela mudança não está disponível nesta sessão. O próximo perfil deve investigar layout/shaping de texto e layout/expansão de tabela, preservando os controles de saída. Nenhum L0 ou código produtivo foi alterado neste item.
