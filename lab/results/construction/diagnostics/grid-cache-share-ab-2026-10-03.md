# Grid: A/B de caches de métricas entre duas passagens de layout

## Hipótese medida antes do código

O [diagnóstico de referência](grid-cache-real-2026-10-03.md) mediu, numa
tabela de 320 células distintas, 2.880 consultas e 962 falhas no cache de
shaping. O texto `C000` falhou em três instâncias de cache: duas passagens de
layout e ajuste final das linhas. Texto repetido teve apenas 5 falhas. A
hipótese foi compartilhar `FallbackFontMetrics` apenas entre os candidatos de
layout do ciclo normal de paginação, sob o mesmo `World`. Isso é uma mudança
interna de cache, não uma nova regra de igualdade de `Content` ou uma mudança
de linguagem. O owner [L0 de paged](../prompts/infra/pipeline/paged.md) foi
atualizado antes do código, e a contagem original foi o gate RED.

## Variante e controles

O [consumer](../../03_infra/src/pipeline/paged.rs) agora cria uma instância
de `FallbackFontMetrics` antes da primeira tentativa de layout no ciclo
comum e clona seu handle que compartilha caches nas tentativas seguintes.
Cada documento candidato, introspector e store de callbacks continua sendo
construído por passagem. Os wrappers usados por estabilização de contexto e
bundle continuam criando métricas novas; o ajuste final de linhas também
mantém sua instância independente.

O [runner pareado](grid-cache-real-2026-10-03/compare_cache_share.py) comparou
o binário Tekt original SHA-256
`a12d75ded40944932b20dbd751d7969d3f846ac04de10c817cfffee707175cc4`
com o binário final SHA-256
`acedd6624e8f370cc2b622976b6fb44ae5f04ca47b84b1414718236400965ff9`.
O primeiro foi preservado antes da alteração; o segundo foi construído com
`cargo build --offline --release --locked -p typst-wiring`.

As duas rodadas finais ocorreram em 2026-10-03 02:51:29 UTC e depois,
HEAD `616a8ff50281efd5bec8329a3fc8f9fdbfbe2612`, árvore não commitada:
`M 00_nucleo/prompts/infra/pipeline/paged.md`,
`M 03_infra/src/pipeline/paged.rs` e
`?? 00_nucleo/diagnosticos/grid-cache-real-2026-10-03/compare_cache_share.py`.
O `git diff HEAD --stat` foi:

```text
 00_nucleo/prompts/infra/pipeline/paged.md |  4 +-
 03_infra/src/pipeline/paged.rs            | 86 +++++++++++++++++++++++--------
 2 files changed, 68 insertions(+), 22 deletions(-)
```

Cada rodada usou 2 warmups e 20 pares de subprocessos novos por fixture,
ordem alternada, CPU 0, `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`, mesmas
opções CLI e PDF escrito em `/tmp`. O tempo inclui spawn, compilação, exportação
e escrita. Os [dados da primeira rodada final](grid-cache-real-2026-10-03/cache-share-ab-final.json)
e os [da confirmação](grid-cache-real-2026-10-03/cache-share-ab-final-confirm.json)
guardam amostras, hashes, horários e estado exato. A primeira teve picos de
470–535 ms em alguns processos; a confirmação teve faixas estreitas para
texto distinto. A decisão usa a mediana pareada e ambas as rodadas.

| Fixture | Rodada | Antes | Depois | Delta pareado | Pares favoráveis |
| --- | --- | ---: | ---: | ---: | ---: |
| 320 textos distintos | final | 244,63 ms | 234,17 ms | −10,39 ms | 18/20 |
| 320 textos distintos | confirmação | 243,42 ms | 234,23 ms | **−9,46 ms** | **19/20** |
| 320 textos iguais | final | 211,47 ms | 210,44 ms | −0,74 ms | 15/20 |
| 320 textos iguais | confirmação | 211,80 ms | 210,90 ms | −0,93 ms | 17/20 |

Um controle de quatro células foi incluído na
[primeira rodada exploratória](grid-cache-real-2026-10-03/cache-share-ab-initial.json),
mas recebeu picos de carga grandes em ambos os binários; não foi usado para
decidir. Outra [rodada exploratória](grid-cache-real-2026-10-03/cache-share-ab-repeat.json)
do protótipo inicial também apontou −9,13 ms em 20/20 pares distintos. Após
restringir o escopo dos wrappers, a variante final foi recompilada e medida
novamente nos números da tabela.

## Trabalho evitado e saída

No binário debug final (SHA-256
`0524ec535f0e27924d54fd655c0e632384e147fe07f6710d9d2ef8a262bfd7d4`),
o mesmo [script GDB](grid-cache-real-2026-10-03/grid-font-cache-count.gdb)
encontrou 2.880 consultas ao cache de shaping, mas as falhas caíram de
**962 para 641** com textos distintos e de **5 para 3** com textos iguais:
[log distinto](grid-cache-real-2026-10-03/distinct-after-final-cache.txt) e
[log igual](grid-cache-real-2026-10-03/same-after-final-cache.txt). Isso
confirma o mecanismo; o tempo vem do runner sem GDB.

Antes/depois produziram hashes idênticos de `pdftotext -bbox` em cada fixture:
`bf41d63dd7b695589e4bfcd1916afb72fddb31cf837826f67858260fe89edf1e`
para textos distintos e
`d720c2bca3ef7e3b64607c4d8d7e4321094733d1e9635d56565af24f4f96d80b`
para textos iguais. A igualdade cobre texto extraído e coordenadas das 320
palavras; não é uma prova de todas as estruturas possíveis do PDF.

## Verificação e decisão

`cargo test --offline --locked -p typst-infra --lib pipeline::` passou 121
testes; `cargo build --offline --locked`, `rustfmt --check` do consumer e
`crystalline-lint .` passaram, com zero violações do linter. O ganho
reproduzível no caso distinto é aproximadamente **9–10 ms (cerca de 4% do
tempo total desse documento)**. Com texto já repetido, o cache antigo evita
quase todo o shaping e o ganho adicional ficou abaixo de 1 ms.

A variante merece permanecer como otimização interna limitada ao mesmo
`World` e ciclo comum de paginação. A medição usa uma tabela simples de uma
página; não atribui o restante da diferença frente ao vanilla ao cache e
não prevê ganho para outros documentos. A próxima investigação deve separar
o custo de `measure_content_constrained`, medição de altura e emissão com
conteúdo longo, estilos e múltiplas páginas, preservando a equivalência de
geometria antes de escolher outro algoritmo.
