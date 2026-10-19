# Agrupamento visual e cache bidi — 2026-10-03

## Corte descartado: agrupamento por baseline na reconciliação

`03_infra/src/shaper/reconciliation.rs:160-181` pesquisa as linhas já abertas
para cada item da página. Prototipamos um atalho para `y` crescente que anexa
diretamente à última linha quando as âncoras criadas seguem ordem monótona;
para o resto, preserva a primeira linha compatível na busca original. Quatro
testes cobriram ordem crescente, janelas de 0,5pt sobrepostas, âncoras fora de
ordem e `NaN`/infinito. O protótipo foi removido após a medição.

A/B de 30 pares alternados, um CPU fixo, dois aquecimentos, 320 células por
fixture, `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`:

| Fixture | Antes, mediana | Protótipo, mediana | Diferença pareada mediana | Pares favoráveis |
| --- | ---: | ---: | ---: | ---: |
| texto distinto | 230,57 ms | 230,48 ms | −0,29 ms | 18/30 |
| texto repetido | 220,82 ms | 220,11 ms | +0,51 ms | 13/30 |

As medianas individuais e as diferenças pareadas respondem a estatísticas
distintas; o sinal inconsistente é ruído na escala do compilador completo.
Decisão: não manter a complexidade adicional no L3. Proveniência: UTC
`2026-10-03 03:16:06`, HEAD `ebe96a68d675481c5bebd28e5f1ec763d0432c27`,
working tree não commitado com exatamente
`00_nucleo/prompts/infra/shaper/reconciliation.md` (1 linha adicionada) e
`03_infra/src/shaper/reconciliation.rs` (89 linhas de diff, 80 adições e 10
remoções no total). Binários SHA-256 antes
`f258f380c976183441eb83575acbc41b90fc95967084df8134847d21a3c7d247`
e protótipo
`72519877f5f4e1ca0b6b8990116e89fe644ed565a30357430341351ff3315705`.
Fixtures SHA-256 distinto
`902986bd28ace4b39144ce567592229c5a5621453cbb661a408414c803a58d6d`
e repetido
`a3a8454c6ffa0ab9763ad5ac0e804abf992deba7d1818e58390fe0e3672389d5`.

## Corte mantido: métricas compartilhadas com bidi

`03_infra/src/pipeline/paged.rs:477` criava outro `FallbackFontMetrics`
antes do bidi. `03_infra/src/layout_bidi.rs:128-173` chama `text_width` para
palavras RTL; no percurso paginado comum, a mesma instância de World já tinha
métricas do layout. A passagem bidi agora toma o handle desse layout. O caminho
contextual selecionado e as assinaturas públicas continuam usando os seus
contratos anteriores. O owner L0 paged foi atualizado antes da mudança L3.

O [runner pareado](grid-cache-real-2026-10-03/compare_cache_share.py) gera
deterministicamente uma tabela hebraica de 80×4 células e a mantém fora do
Git como fixture gerada. Fonte SHA-256
`f7dbb3e9b17eb87a9e51f342ea90c8d1939054f3a8148eadf9da2d27fb9b2f3d`.
Cada rodada teve 30 pares alternados, dois aquecimentos, CPU fixo,
`SOURCE_DATE_EPOCH=0` e `RAYON_NUM_THREADS=1`.

| Rodada | UTC | Fixture | Antes, mediana | Depois, mediana | Diferença pareada mediana | Favoráveis |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| 1 | 03:19:04 | RTL | 232,92 ms | 230,82 ms | −1,65 ms | 30/30 |
| 1 | 03:19:04 | LTR distinto | 231,17 ms | 230,43 ms | −0,47 ms | 19/30 |
| 2 | 03:19:42 | RTL | 230,35 ms | 230,33 ms | −0,78 ms | 18/30 |

O sinal RTL permanece positivo nas duas rodadas (48/60 pares), mas a magnitude
é pequena e flutua. Não extrapolar para documentos LTR ou para qualquer fonte
e script. A confirmação de benefício maior exige mais fixtures reais RTL.

Proveniência das duas rodadas: HEAD
`ebe96a68d675481c5bebd28e5f1ec763d0432c27`, working tree não
commitado com três arquivos: `compare_cache_share.py` (3 linhas de diff),
`00_nucleo/prompts/infra/pipeline/paged.md` (1 linha) e
`03_infra/src/pipeline/paged.rs` (6 linhas), total de 7 adições e 3 remoções.
Depois das medições, o runner recebeu apenas o gerador determinístico da
mesma fonte RTL e o par L0/código foi ressellado. SHA-256 dos binários antes
`f258f380c976183441eb83575acbc41b90fc95967084df8134847d21a3c7d247`
e depois `476f73d5a9935936dedceed75cc4847dcdd5d570bb3f62b0d2e7aa29762d94c3`.

`pdftotext -bbox` deu hashes iguais antes/depois: RTL
`d95ea9829e1f1a047ecbc11fa74b0accd748f09bd46742baf51342350a7988a4`
e controle LTR
`bf41d63dd7b695589e4bfcd1916afb72fddb31cf837826f67858260fe89edf1e`.
A igualdade cobre texto e coordenadas, não todos os bytes do PDF. Passaram
13 testes de `layout_bidi::tests` e 121 de `pipeline::` com relógio fixado;
`cargo build` e `crystalline-lint .` retornaram código zero. Nenhuma nova
afirmação de paridade com o vanilla decorre deste A/B interno.
