# P1415 — relatório de implementação e verificação

## Proveniência

- checkpoint L0 confirmado: `1146b634e9a809f8707df23dc4013f5a28c5e3d2`;
- medição final: `2026-09-15T23:02:35-03:00`;
- estado: working tree não commitada, com os nove arquivos listados por
  `git diff HEAD --stat` neste relatório e este recibo acrescentado depois;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário cristalino final: SHA-256
  `1fe7fe27c0613fb9ca30fb295210acfe26f0a28059d6dc610da3a21fc2df6d95`;
- regime Tekt: rehearsal completo, executado sem atestação de isolamento.

Arquivos produtivos alterados: `eval/rules.rs`, `layout/mod.rs`,
`layout/cursor.rs`, `layout/columns.rs`, `math/layout/callbacks.rs` e
`infra/pipeline.rs`. Os L0 adicionais de columns, cursor e callbacks math foram
atualizados antes da continuidade da implementação e ressellados.

## RED congelado

Sob 512 MiB, uma thread e timeout de 15 s, o baseline rejeitou
`numbering: n => [L#n]` com `expected string or none, found function`. A fixture
de duas colunas produziu `1 left-a   3 right-a` e
`2 left-b   4 right-b`, mantendo a numeração da última coluna na margem
inicial.

## Materialização

- eval aceita `string | function | none` e preserva a `Func` lexical;
- L1 registra requests somente depois da linha física estabilizada;
- L3 realiza a função pelo formatter canônico de numbering, com um inteiro;
- stores math e `par.line` são independentes e estabilizados no mesmo ciclo de
  oito passagens; somente `Complete` transporta `PagedDocument`;
- retorno é `Content`, disposto em subframe marginal sem achatamento;
- alinhamento e maior largura são drenados por coluna física;
- a última coluna usa a margem final e o contador não reinicia por coluna.

## Gates e ataques

| Gate | Resultado |
|---|---|
| callback `[L#n]` | MATCH: `L1/L2/L3` |
| callback esparso `strong`/`none` | MATCH: `N1`, vazio, `N3`, vazio |
| scope `page` | MATCH: `P1/P2`, reset, `P1/P2` |
| callback binário | MATCH: exit 1, `missing argument: b` |
| última coluna com `number-margin:left` | MATCH: `right-a 3`, `right-b 4` |
| morfologia do callback | MATCH: `N1/N3` Bold; corpo Regular nos dois PDFs |
| callbacks math + linha no mesmo candidato | MATCH: `L1 x / L2 tail` |
| repetição em ordem inversa | PASS: texto extraído idêntico byte a byte |
| regressão P1414 | cinco válidos PASS; constructor exit 1 esperado |
| `cargo check -p typst-infra -j1` | PASS sob 2 GiB |
| build release definitivo | PASS em 3 min 33 s sob 2 GiB/um job |
| `crystalline-lint .` | exit 0 sob 1 GiB/uma thread |
| `cargo fmt --check` / `git diff --check` | PASS |

O maior RSS focal válido registrado foi 43.328 KiB no primeiro corpus P1415;
o controle combinado final consumiu 42.788 KiB. Não houve OOM, swap ou
timeout.

O primeiro ataque de coluna após a implementação revelou os números `3/4`
fora da página: a posição final usava largura global antes da translação da
mini-região. A correção passou a usar `regions.current.width`; o ataque e o
controle P1414 de margem direita ficaram GREEN. Esta revisão acrescentou uma
testemunha discriminatória real e não repetiu o corpus amplo antes do foco.

Um segundo ataque distinguiu colunas de página de um contentor `#columns()`:
no segundo caso, o vanila deduplica a linha física transversal e não numera a
coluna irmã. A coordenação de "última coluna" foi por isso limitada ao
`#set page(columns:)`; o relatório não reivindica essa regra para
`#columns()`.

## Limitações do veredito

O veredito cobre o fragmento P1415 congelado, não equivalência geral de
`par.line`. Callbacks contextuais arbitrários, conteúdo deliberadamente
gigante/multilinha e múltiplas direções de escrita permanecem `Unknown` segundo
o passo. Não houve separação de autoridades ou ambientes; portanto este
relatório não reivindica materialização segregada atestada nem mutation score
formal de 1,0.
