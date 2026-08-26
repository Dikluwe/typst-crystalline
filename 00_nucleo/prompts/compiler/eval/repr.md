# Prompt L0 — `compiler/eval/repr` — representação morfológica
Hash do Código: 40c8d20b


**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/eval/repr.rs`
**Vanilla ratificado:** `a51e02804`

## Contrato

Produzir `repr` exaustivo e determinístico para `Value`, `Content` e
`Selector`, preservando a morfologia pública da linguagem, escaping e formas
constructoras. A aceitação é textual/linguística; a estrutura Rust é mecânica.

Labels usam `<nome>` somente quando o nome satisfaz a gramática literal; caso
contrário usam `label(<repr string>)`. Raiz matemática sem índice usa
`root(radicand: <repr>)`; com índice conserva `root(<index>, <radicand>)` até
medição específica. Symbols multi-codepoint preservam todos os scalar values,
incluindo variation selectors e ZWJ, sem conversão para `char`, normalização ou
substituição pelo nome canônico.

## Restrições e aceitação

P1224: `Stroke` simples conserva a forma histórica `thickness + paint`.
Quando qualquer dimensão complexa diverge do default, usa dict morfológico com
campos explícitos `paint`, `thickness` quando não-default, `cap`, `join`,
`dash: (array:, phase:)` e `miter-limit`. A ordem do dash é preservada;
`DashLength::LineWidth` usa `"dot"`. Não usar `Debug` dos enums.

P1225 permite que o módulo pai exponha esse formatter por uma função pública
estreita para o serializer L2. O formatter genérico de `Value` continua
interno; somente a representação de `Stroke` integra o contrato cross-layer.

L1 puro, sem I/O. Toda nova variante pública exige medição anterior contra o
vanilla e branch explícito; não usar `Debug` como fallback. Testes focais
cobrem escaping, labels, conteúdo matemático, selectors e symbols.

O constructor nativo `native_repr` pertence a
`compiler/stdlib/foundations/repr.md`; este owner especifica a serialização
efetiva chamada por ele.
