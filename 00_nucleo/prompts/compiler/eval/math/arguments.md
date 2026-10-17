# L0 — compiler/eval/math/arguments
Hash do Código: 8b9fae7b

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/arguments.rs`. No HEAD `e754ab364a221dbe344852178e84bd42b02c463e` e árvore limpa, `01_core/src/compiler/eval/math.rs:10-16,251-575` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contrato

Interpretar named, argumentos e linhas de matriz preservando ordem de AST e spans. Preservar assinaturas, semântica da linguagem e diagnósticos de `00_nucleo/prompts/compiler/eval/math.md`; não alterar fase do pipeline, defaults ou compatibilidade. A extração é interna a L1, sem I/O nem estado global mutável.

## Aceitação

A suíte L1 e os testes de math passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

## Resíduos D07 — medição antes da decisão

`00_nucleo/diagnosticos/math-residuos-table-cell-2026-10-06/before.json.gz` mostra bare-none, bare-align-left e bare-center
resolvendo por Code/global onde o vanilla pinado produz unknown variable com
hints matemáticos. code-none e code-align-left são controles positivos.
`lab/typst-original/crates/typst-library/src/foundations/scope.rs:81–100`
consulta escopos lexicais e base.math; o global só informa o diagnóstico.
`compiler/eval/math/arguments.rs:40–75` reparseava none como literal e
`:115–130` desviava MathIdent para global. code-auto mostra ainda um literal
Code Auto convertido a Content.

Decisão de paridade de sintaxe/semântica: um wrapper unitário com MathIdent
continua matemático e não é reparsed como literal Code. Só Ident de Code pode
usar a rota de globals; MathIdent mantém resolução lexical/math e seus hints.
Auto explícito, como None/Bool, usa eval_expr nos dois braços autorizados.
Conservar code expressions, locais, conteúdo e callbacks anteriores. Inferência:
essas duas rotas causam os controles negativos; seria refutada se continuassem
divergentes após sua correção. Sem mudança pública ou de fase. Aceitação:
RED→GREEN de bare-none/left e bilateral dos controles bare/#/local.

## D07 — refutação de conservação dos tipos locais

`00_nucleo/diagnosticos/math-residuos-table-cell-2026-10-06/local-refuters-before.json.gz`
mede local-left/local-none válidos no vanilla e rejeitados como content no Tekt;
local-auto/local-boolean têm cast error de auto/boolean no vanilla e content no
Tekt. Essa coleta usa o binário do primeiro reparo, a62789ae46d2dbc3ab9ecd0137a94f3ed18c6d44a2b54869b9b77f600d36cdd5,
com fontes/UTC/estado completos no artefato. Refuta a hipótese de que conservar
o caminho display dos locais bastava para o argumento manter seu tipo.
A fonte upstream foundations/scope.rs:81–100 retorna o Value lexical real.

Decisão: MathIdent que resolve em binding local retorna seu Value clonado,
sem display coercion antes do cast. Um wrapper unitário contendo MathIdent
delega à mesma rota de argumento, preservando o limite math/Code e os hints
para nomes ausentes. Local Content permanece Content; None/Auto/Align/Bool
retêm o tipo. Não alterar resolução matemática geral de texto nem globals.
Aceitação: RED→GREEN de locais typed e bilateral dos quatro refutadores,
conservando os controles #, bare e Content local já medidos.
