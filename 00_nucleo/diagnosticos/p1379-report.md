# P1379 — relatório da distribuição fracionária por segmento

## Resultado

Cada região irmã separada por `colbreak()` agora distribui seus próprios
Blocks `fr` desde o cursor de entrada até o fundo finito da coluna. A matriz
bilateral fechou os seis casos declarados em `MATCH`, incluindo `#columns()`,
page columns, offset exterior, pesos diferentes e prefixo fixo.

## Evidência de linguagem

- básico: `One=Two=7,404` e `After=Tail=220,166`;
- offset: `Intro=7,404`, `One=Two=27,842` e caudas em `220,166`;
- pesos locais `1fr:2fr`: `Two=82,724670`, sem contaminar a coluna irmã;
- prefixo fixo: `Lead=7,404`, `One=27,842`, `Two=7,404` e ambas as caudas
  em `220,166`;
- page columns: corpos em `7,404` e caudas em `220,166`;
- o controle simples P1378 continua com `Outside=27,842`.

As posições extraídas dos PDFs coincidem com o vanila ratificado dentro de
0,001pt. O crédito é morfológico, não depende da estrutura interna do Rust.

## RED → GREEN e reaberturas

Os três testes iniciais falharam antes da implementação. A primeira candidata
calculou a parcela, mas um excesso de arredondamento paginava a cauda; a matriz
seguinte expôs ainda o `Styled` exterior de page columns e o prefixo fixo. A
quarta calibração fez a cauda caber, porém a matriz recusou crédito porque o
corpo permanecia 6,05pt alto demais. O teste foi endurecido e a quinta revisão
materializou a fronteira `par.spacing - par.leading`, descontando-a também da
parcela. Os onze mutantes declarados foram discriminados; score 1,0.

## Implementação

Sequence ganhou uma base interna cursor-relativa usada apenas pela entrada de
segmentos explícitos. Cada segmento recalcula pesos e fixos; uma guarda por
escala impede overshoot de ponto flutuante. `columns` transporta um `Styled`
exterior para cada região irmã. Não foi criado campo público, carrier, recursão
nem coleção proporcional adicional.

## Limites conhecidos

Frações aninhadas, overflow multi-região, frações em notas, `grid`, `box`,
`place` e paridade geral continuam `Unknown`. Esses casos não receberam
crédito por aproximação.

## Limite de atestação

Protocolo completo executado sem atestação de isolamento: uma autoridade
compartilha contrato, código, ataques e veredito. Os hashes provam identidade
e ordem, não independência.

## Memória

Testes e links foram seriais sob 4 GiB; check e linter sob 3 GiB; a matriz foi
sequencial sob 2 GiB, ficheiro máximo 16 MiB e timeout de 45 s por processo.
O maior RSS observado no candidato final foi 37.656 KiB e o maior PDF teve
7.028 bytes. Não houve OOM, timeout nem `SIGKILL`.
