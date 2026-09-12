# P1355 — Relatório de resync do vanilla ratificado

## Veredito

O alvo de paridade foi movido de
`a51e028041cac426f97d34335bb01d8f1d8e5e8f` para
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. A árvore upstream correspondente
é `2847920eeaa98d26a41dc6fb0f9c02334b9d593b`. O resync preservou apenas o
overlay local do enumerador P1140 e seu ajuste de lockfile.

O resultado **não** declara paridade. Os 175 paths do P1353 permanecem uma
medição histórica do alvo anterior. O P1356 deve reconstruir a matriz e
atribuir cada delta entre produto, mudança do alvo e limitação do harness.

## Proveniência da execução

- UTC da verificação final de identidade: `2026-09-12T17:26:55Z`;
- HEAD de execução, antes do commit deste relatório:
  `92f2f78ec1b09e8ab8416bb134136acc051daa96`;
- estado: working tree não commitado, com `197 files changed, 3024 insertions(+),
  2631 deletions(-)` após resselo de linhagem;
- intervalo upstream: 35 commits entre o pin anterior e o novo;
- patch determinístico do intervalo, SHA-256:
  `3ebad68debae9b15347c7d9b2d4b8b2857655cee13919269660868007cab8250`;
- Rust: `rustc 1.92.0 (ded5c06cf 2025-12-08)`;
- Cargo: `cargo 1.92.0 (344c4567c 2025-10-21)`;
- Python: `3.12.3`.

O contrato e o manifesto selado estão em
`p1355-resync-contract.md` e `p1355-resync-manifest.json`.

## Build vanilla ratificado

Receita:

```sh
TYPST_COMMIT_SHA=586e1bd43fae6c9a973218163d3165c53ab8d16d \
  cargo build --manifest-path lab/typst-original/Cargo.toml \
  --package typst-cli --release --locked
```

Identidade observada nas duas cópias canônicas:

```text
lab/typst-original/target/release/typst  eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f
/usr/local/bin/typst                       eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f
--version                                  typst 0.15.1 (586e1bd4)
```

A string de versão é apenas sentinela; pin, árvore, receita e hash do binário
constituem a prova de identidade.

## Mudanças upstream relevantes à linguagem

A inspeção do intervalo encontrou mudanças públicas que exigem rebaseline:

- novo módulo `format`, com `format.pdf/html/svg/png/bundle` e aliases públicos;
- opções de formato settable e precedência entre documento e CLI;
- `columns.separator`;
- correções em `luma`, `array.to-dict`, expansão de bloco e pares CJK;
- um elemento `meta author` por autor no HTML;
- novas opções CLI de input, watch, viewer, pretty, tagged PDF e PPI;
- fronteiras exatas do formatter de duração.

O commit de ponta melhora a tipagem interna do flow layout. Essa alteração é
mecânica até que um witness de linguagem demonstre mudança observável.

## Sinal preliminar do inventário

Esta varredura serve para orientar o P1356; não substitui sua matriz funcional.
Os binários usados foram o vanilla acima e o cristalino de SHA-256
`55fd2fe785d75bcb2505e7595c9afa56c1acd25afea9ffa28e4eb9fe4ae0152b`.

No perfil default, o merge de 2.052 entradas classificou:

```text
MATCH                 1551
MISSING_BINDING          1
MISSING_MEMBER          43
WRONG_KIND               3
UNVERIFIED_METADATA    412
EXTRA_BINDING           42
UNKNOWN                  0
```

No perfil HTML, o merge de 2.123 entradas classificou:

```text
MATCH                 1551
MISSING_BINDING          1
MISSING_MEMBER          43
WRONG_KIND              74
UNVERIFIED_METADATA    412
EXTRA_BINDING           42
UNKNOWN                  0
```

Os hashes dos merges foram, respectivamente,
`67f541829e02f50dc1a9aaea83710c0ba919fdbdd5339f2101f83d551ce31510`
e `b2ea84df1ef937e75f85e628d6ce1e49accf606bc16e8f94d0f3b13032325da2`.
O binding ausente `format` e as diferenças de kind de `pdf`/`html` confirmam
que o alvo novo não pode reutilizar o saldo antigo sem atribuição de delta.

## Processo e verificação

O trabalho usou autoria de contrato, inspeção adversarial e adaptação do
mecanismo separadas por capacidade e ordem, mas o ambiente não forneceu
atestação técnica de isolamento. Portanto: **executado sem atestação de
isolamento**.

A restrição operacional que proibia leitura espontânea de `materialization/`
e `context/` foi removida de `CLAUDE.md` por solicitação explícita do dono. Isso
altera o processo de trabalho, não a arquitetura L0 nem o produto.

O passo seguinte é `typst-passo-1356.md`, que exige catálogo regenerado,
comparação tripla entre alvo antigo, alvo novo e cristalino, quatro perfis e
atribuição path a path antes de abrir novos lotes de implementação.
