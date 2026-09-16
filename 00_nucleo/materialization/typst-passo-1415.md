# Passo 1415 — realizar callbacks e coordenar colunas de `par.line`

## Objetivo

Completar os dois subconjuntos que P1414 declarou incompletos: aceitar
`numbering: function` sem executar `Func` no layout puro e forçar a numeração
da última coluna física para a margem final. O resultado deve preservar a
semântica, a sintaxe e a morfologia da linguagem Typst, sem copiar a mecânica
interna do vanila.

## Estado e proveniência inicial

- baseline: commit `0a7de3dd9ac7de214066aa6b77f36bb4f9d178a4`, árvore limpa
  antes da criação dos cinco oráculos deste passo;
- cristalino: `target/release/typst`, SHA-256
  `271ae0a93f77fad737debe856c9913688bacd7b6b4367c91308163e82ebc5df6`;
- vanila ratificado: commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- medição congelada às `2026-09-15T22:12:43-03:00`;
- regime: protocolo Tekt completo em rehearsal, executado sem atestação de
  isolamento.

## Medição antes da decisão

A fonte vanila declara que `ParLine.numbering` aceita pattern ou função unária
em `lab/typst-original/crates/typst-library/src/model/par.rs:791-813`. A mesma
fonte declara em `:835-856` que a última coluna sempre usa a margem `end`.
Depois de o layout físico encontrar e ordenar as linhas, o compositor realiza
o número em `lab/typst-original/crates/typst-layout/src/flow/compose.rs:1003-1057`
e força `End` quando `column + 1 == columns.count` em `:1059-1066`.

No baseline cristalino, `01_core/src/compiler/eval/rules.rs:1330-1335` aceita
somente `string | none`. O Cursor só entra no branch `Value::Str` em
`01_core/src/compiler/layout/cursor.rs:790-860`; em colunas, usa a configuração
pedida e não identifica a última. A pipeline já possui o precedente puro
P1291: `03_infra/src/pipeline.rs:997-1032` recebe requests math de L1, executa
callbacks com `Engine` em L3 e repete uma passagem limitada.

Sob 512 MiB, uma thread e timeout de 15 s, o vanila produziu:

```text
callback content:       L1 alpha / L2 beta / L3 gamma
callback esparso:       N1 alpha / beta / N3 gamma / delta
callback scope page:    P1 alpha / P2 beta / form-feed / P1 gamma / P2 delta
callback aridade 2:     exit 1, error: missing argument: b
duas colunas margin:left:
                        1 left-a   right-a 3
                        2 left-b   right-b 4
```

O cristalino rejeitou os quatro callbacks como tipo incompatível e dispôs os
números `3` e `4` à esquerda da coluna final. O maior RSS observado entre os
dois binários foi 40.828 KiB; não houve timeout, OOM nem swap.

## Oráculos congelados

| Fixture | SHA-256 |
|---|---|
| `par-line-callback-arity-error.typ` | `3f7f165ad5d640cd67930618764eb4005d03c08d3ab37fc3152ca47c4437eb16` |
| `par-line-callback-content.typ` | `149aa6232ec62e8950ff154bc47dc599bc34313ccd93d267575eb4f48782aa19` |
| `par-line-callback-page-scope.typ` | `886b0948ca14d50f406d1019cfb6c51ae8ee9eaf98337284ec348cc08b611465` |
| `par-line-callback-sparse.typ` | `19ce2f467fb7335f24b0ce1881ba1374ec8747fa3706a50f162f593adf39afd4` |
| `par-line-last-column.typ` | `4003dc2d22d2cc67e157d7595c9c648e889f541c0c79fa0e6473b8154dc6d1df` |

Os arquivos em `lab/parity/corpus/p1415/` são externos ao candidato e não podem
ser adaptados à implementação. O caso esparso exige também inspeção da
morfologia: `strong([N#n])` continua conteúdo forte, não string de debug.

Política de `Unknown`: timeout, OOM, swap, falta de output, callback contextual
que dependa de capacidade ainda não contratada ou retorno deliberadamente
gigante não contam como preservação. Um observável focal conhecido e divergente
é falha. Casos opacos ficam fora do veredito, nunca são convertidos em PASS.

## Contrato proposto

1. `#set par.line(numbering: ...)` aceita `string | function | none`. A função
   é guardada como valor lexical com seu span; eval não a chama nem a converte.
2. Cada linha física confirmada avança o contador exatamente uma vez. Pattern
   continua pelo caminho P1414. Função gera request com a própria `Func`, o
   inteiro lógico, o `StyleChain` causal e o span; não gera texto provisório.
3. L1 mantém layout puro. Uma nova passagem explícita recebe stores seladas e
   devolve `Complete(PagedDocument)` ou `Pending(requests)`. Documento
   provisório de `Pending` é consumido e nunca chega a numbering de página,
   shaping ou export.
4. L3 realiza cada request em ordem com exatamente um `Value::Int(n)`, usando
   `Engine` e o estilo capturado, e converte o retorno pela conversão canônica
   de valor para conteúdo. `none` produz conteúdo vazio; `Content` preserva sua
   morfologia. Erro da closure, aridade ou conversão é propagado com diagnóstico
   e nenhum documento é exportado.
5. A passagem seguinte recebe somente resoluções seladas. Transcript novo,
   resolução não consumida ou consumo divergente invalida o candidato e repete
   sob teto finito. Oito tentativas, como no precedente math, são o limite;
   excedê-lo é diagnóstico de não convergência.
6. A coordenação L3 estabiliza callbacks math e de `par.line` no mesmo ciclo de
   candidato. Nenhum dos dois executa dentro do outro; stores são locais à
   compilação e reconstruídos antes de cada relayout externo P1159.
7. O conteúdo realizado é disposto na margem como conteúdo, alinhado à baseline
   causal. A maior largura é calculada por região/coluna física; não se usa uma
   largura global da página nem se achata conteúdo para `FrameItem::Text`.
8. Em uma página com duas ou mais colunas, a última coluna física usa sempre a
   margem final, independentemente de `number-margin`. As demais respeitam a
   configuração P1414. O contador permanece único no scope escolhido.
9. Entry points compatíveis continuam válidos para árvores sem requests. Se um
   caller incapaz de realizar callbacks encontrar uma request, falha fechado;
   não devolve documento parcial, vazio ou sem numeração.

## Limites e refutadores

Não criar `Engine` em L1, não guardar `World`/`EvalContext` em `Layouter`, não
executar a função em eval antes de wrap, não inferir linhas por `\\`, não
duplicar o algoritmo de `stdlib/numbering`, não serializar `Content` como texto
e não reutilizar a largura máxima de uma coluna em outra. Qualquer uma dessas
formas refuta o contrato mesmo que os textos simples coincidam.

Callbacks contextuais arbitrários e política de múltiplas direções de escrita
permanecem fora deste recorte até medição própria. O callback unário puro, seu
diagnóstico de aridade, retorno `none`/conteúdo e reinício por página são
Known e obrigatórios.

## Gate ADR-0127

P1415 amplia o contrato público de `par.line(numbering:)` e acrescenta uma
realização entre passagens de layout. São simultaneamente os pontos 1 e 3 do
ADR-0127. Este passo e os L0 proprietários devem ser commitados e ressellados;
a implementação fica bloqueada até confirmação humana explícita posterior ao
commit do contrato.

## Protocolo de memória para a implementação

- probes Typst: 512 MiB, uma thread, timeout 15 s, saída isolada em `/tmp`;
- build release: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512` e linker `gold`;
- lint: 1 GiB, uma thread, timeout 60 s;
- proibido `cargo test -p typst-core --lib`, `cargo check --tests` e qualquer
  matriz ampla antes do GREEN dos cinco oráculos;
- requests/resoluções são lineares no número de linhas e não guardam páginas,
  frames ou cópias acumuladas do documento provisório.

## Aceitação futura

Os três casos válidos de callback coincidem bilateralmente em texto e
morfologia; o caso binário falha com `missing argument: b`; a última coluna
move os números para a margem final. P1414 permanece verde. Build focal,
formatação, `git diff --check` e `crystalline-lint .` passam nos limites, sem
OOM, swap ou documento parcial. O veredito será rehearsal completo, sem
atestação de isolamento.
