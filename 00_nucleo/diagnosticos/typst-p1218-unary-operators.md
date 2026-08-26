# P1218 — fechamento dos operadores unários

## Veredito

**UNARY OPERATORS GREEN — BINARY AUDIT EXHAUSTIVE.** `+`, `-` e `not`
fecharam a matriz pública declarada. A relação DSM `eval-apply-binary` não foi
promovida: a auditoria de 19/19 variantes encontrou o gap independente
`decimal-int-arithmetic`.

## Medição antes da decisão

Baseline: HEAD `3f0a2638fffd8cddc0fde6e83058f69dedc7d838`, working tree não
commitado, medição em 2026-08-26 (America/Sao_Paulo). Vanilla ratificado:
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O estado exato das mudanças está recuperável por `git diff HEAD --stat`.

Os oráculos foram congelados antes da implementação em
`p1218-unary-oraculos.tsv`. Os testes focados confirmaram RED e depois 2/2
GREEN. Duas rodadas independentes produziram 16/16 expressões unárias e
58/58 controles herdados de P1216/P1217 com `exit + stdout/stderr` literais
idênticos. Seis compilações deram 3/3 sucessos e 3/3 erros com mensagem e
região iguais; somente a apresentação do caminho é relativa no vanilla e
absoluta no cristalino, fora do contrato do operador.

## Mudança

`Pos` agora preserva identicamente Decimal, Angle, Ratio, Relative e Fraction.
A fronteira inválida usa spelling público do operador e nomes Typst: String,
Array e Datetime preservam os casos vanilla com `unary`; Bool e os demais
casos usam a forma sem `unary`. `Neg Datetime` ganhou seu caso especial e
`not` deixa de expor o variant Rust.

## Auditoria binária e fila

A tabela `p1218-binop-auditoria.tsv` contém uma linha para cada uma das 19
variantes públicas. Assign e assigns compostos pertencem ao avaliador de
atribuição, não ao dispatcher puro. Add/Sub/Mul/Div revelaram oito operações
direcionais Decimal↔Int aceitas pelo vanilla e rejeitadas pelo cristalino.
Logo `unary-operators` foi marcado RESOLVED, `decimal-int-arithmetic` entrou
como MISSING antes de `svg-morphology`, e `eval-apply-binary` permanece
`parcial` com evidência nominal.

## Ataques

Os 14 mutantes obrigatórios foram rejeitados; `mutation_score = 14/14 = 1.0`.
Esta execução seguiu o protocolo completo numa única sessão e, portanto, foi
**EXECUTADA SEM ATESTAÇÃO DE ISOLAMENTO**.

## Gates finais

`cargo test --workspace`, build workspace/release, `crystalline-lint .`, fmt e
`git diff --check` passaram. A lente DSM foi executada duas vezes com saída
estável SHA-256
`d1e25a3d39c6426d9f958f00400340c3e1b9fbeac9c15f7325863c99f155a5af`.
