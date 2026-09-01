# Passo 1290 — relatório de execução

**Estado do fragmento:** `Preserved`
**Estado da árvore global:** não certificada por falhas concorrentes fora de `repr`
**HEAD medido:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`

## Resultado

O owner `compiler/eval/repr` passou a reproduzir a morfologia ratificada de
arrays/tuplas e `Content::MathOp`. Arrays usam a fronteira ASCII medida de 50
caracteres, preservam a vírgula do singleton e quebram/reindentam conteúdo
multilinha. `MathOp` emite sempre `op(text: ..., limits: ...)`, na ordem medida,
com o booleano real. A proveniência de smartquotes já resolvidas continua
explicitamente `Unknown`; não foi reconstruída por glifo e nenhum outro owner
ou fase foi alterado por P1290.

O contrato teve uma primeira versão invalidada por um probe que exigia
proveniência já perdida. O contrato v2 corrigiu o probe. O resselo de linhagem
teve uma tentativa v3 invalidada porque os dois campos do diagnóstico do linter
foram inicialmente invertidos; o par final confirmado é `Hash do Código:
19c40081` no L0 e `@prompt-hash 52efe145` no consumer. O selo consumível é o v4.

## Evidência segregada

- Contrato/manifesto v4 foram repinados sem leitura da candidata; o
  reverse-repin reconstrói os artefatos v2 fora da linhagem.
- Oráculos v4 preservam 21 casos positivos e 6 casos opacos `Unknown`.
- A campanha adversarial matou 14/14 mutações válidas; score `1.0`, mesma
  resposta em ordem direta e reversa.
- Testes A observaram RED contratual no baseline; após implementação B, 7/7
  focais e 59/59 testes de `repr` passaram antes da quebra concorrente da crate.
- O binário release final passou o gate bilateral duas vezes: 21/21 casos
  idênticos ao vanilla, sem `Unknown` ou `Violated`.

## Regressão e atribuição

A amostra integral de 111 probes passou de 72 para 95 matches. Todos os 72
matches anteriores foram preservados; não houve perda. Dos 23 ganhos, 17 são
projetados pelo P1290 (15 representações de arrays/símbolos/emoji, `math.csc`
e `math.dim`). Seis ganhos de membros de `float`/`math` pertencem a trabalho
concorrente e não são atribuídos a este passo. A matriz estabilizada fechou com
15 `MATCH`, 2 `DIFFERENCE`, 3 `DISABLED_BY_PROFILE`, 0 `UNKNOWN`; todas as
expectativas foram satisfeitas.

## Gates globais

O build release passou e `git diff --check` passou. O workspace compartilhado
mudou durante a execução e terminou com 149 ficheiros modificados. Por isso:

- `cargo test --workspace` não chegou aos testes: há 29 erros concorrentes fora
  de `repr`, incluindo aridade de `make_stdlib` e campos ausentes
  `TableElem.summary`/`TableCellElem.kind` em testes;
- `crystalline-lint` encontra 2 V0 e 20 V5 fora de `repr`; V5 não lista
  `01_core/src/compiler/eval/repr.rs`;
- `cargo fmt --all -- --check` encontra outros ficheiros não formatados;
  `repr.rs` não aparece no diff de formatação.

Consequentemente, o certificado final declara `Preserved` somente para o
fragmento selado P1290 e não certifica a árvore global concorrente.

## Artefatos canônicos

- `p1290-seal-v4.json`
- `p1290-candidate-receipt-v4.json`
- `p1290-integration-receipt.json`
- `p1290-final-certificate.json`

A segregação de papéis foi declarada e observada, mas o filesystem partilhado
não constitui prova técnica de isolamento de leitura.
