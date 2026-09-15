# Passo 1413 — materializar `raw.line`

## Objetivo

Fechar `P1400-ELEM-013 raw.line`, uma das duas últimas rotas `ABSENT` da
matriz P1409/P1412. `par.line` permanece fora do recorte porque depende do
fechamento físico das linhas no layout; `raw.line` é uma unidade morfológica
sintetizada antes do layout e pode reutilizar `Content::Dynamic`, show rules e
layout de `body` já existentes.

## Estado e proveniência inicial

- baseline: commit `e7b2ddd4424d3e86f1e3361d3c08e81a8fc427a7`, árvore limpa;
- cristalino: `target/release/typst`, SHA-256
  `9e4c8876d5e588410a6fdab843cbda93522f82bdd031645c575f7c3e8d1d781c`;
- vanila ratificado: commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- medição preparada às `2026-09-15T19:18:48-03:00`;
- regime: protocolo Tekt completo em rehearsal, sem atestação de isolamento.

## Medição antes da decisão

No cristalino, `raw` é uma função nativa sem namespace `line`; a forma
qualificada falha antes de avaliar o show. `RawElem` guarda apenas
`text/lang/block`, e `eval_raw` devolve o elemento sem atravessar
`intercept_content`. A fronteira dinâmica existente já oferece identidade de
kind, campos, matching de `#show` e layout do campo `body`, portanto uma nova
variante no enum fechado `Content` não é necessária.

Medições limitadas a 512 MiB no vanila:

- fixture P1400: `QUAL=line INSTANCE=x` e `QUAL=line INSTANCE=y` em linhas
  separadas;
- bloco `x`, vazio, `y`: campos `(number, count, text)` iguais a
  `(1,3,"x")`, `(2,3,"")`, `(3,3,"y")`; `body` conserva o conteúdo destacado;
- raw inline `z`: uma instância com `number=1`, `count=1`, `text="z"`;
- `raw.line(1, 1, "x", [X])` é construção pública válida e renderiza `X`.

O maior RSS vanila nesses quatro controles foi 33.976 KiB. O cristalino antigo
falhou nas três formas qualificadas com `function raw does not contain field
line`; o constructor falhou pelo mesmo motivo.

## Contrato proposto

1. `RawLineElem` possui `number: i64`, `count: i64`, `text: EcoString` e
   `body: Content`; expõe os quatro campos e usa `raw.line` como identidade
   dinâmica estável.
2. `raw` recebe namespace contendo `line`. A função é um constructor de
   elemento, selecionável por `#show`, cujo `repr` público é `line`.
3. Raw inline ou bloco é dividido preservando linhas vazias. Cada linha recebe
   índice baseado em 1 e o mesmo total; o body é o conteúdo raw daquela linha.
4. Quando há regra aplicável a `raw.line`, cada instância atravessa o motor de
   show vigente antes do layout. Sem regra aplicável, o `RawElem` original é
   preservado, inclusive highlighting e comportamento cristalino atual.
5. Resultados das linhas mantêm ordem e separação de linhas. Filtros `where`
   usam os campos públicos, e argumento inválido do constructor é erro.

## Gate ADR-0127

Este passo acrescenta o contrato público `raw.line`, seus quatro campos e
constructor. Os prompts L0 proprietários devem ser guardados, ressellados e
commitados agora. A implementação L1 fica bloqueada até confirmação humana
explícita posterior a este contrato.

## Protocolo de memória para a implementação

- probes Typst: 512 MiB, uma thread, timeout 15 s e saída isolada em `/tmp`;
- build release: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512` e linker `gold`;
- lint: 1 GiB, uma thread, timeout 60 s;
- proibido `cargo test -p typst-core --lib` e `cargo check --tests`;
- executar primeiro apenas as quatro fixtures P1413; matriz P1409 completa só
  depois do GREEN focal.

## Aceitação futura

As quatro fixtures devem coincidir bilateralmente com o vanila em texto
extraído, exit e stderr. A matriz P1409 deve terminar `PASS` com
`{ABSENT: 1, MATCH: 5}`, promovendo somente `raw.line`; `par.line` permanece
`ABSENT`. Build, formatação, diff-check e linter devem passar nos limites.
