# P1343 — inventário candidate-free e fronteiras iniciais de cápsula R1

Regime: **executado sem atestacao de isolamento**. Papel:
`independent_candidate_free_baseline_authority`, executor
`/root/p1343_baseline`. Este artefato mede e congela a topologia anterior ao
candidato; não é contrato, oráculo, selo, teste nem implementação.

## Entradas protegidas

- Passo autorizado `materialization/typst-passo-1343.md`, SHA-256
  `6db7b3bb119f4038f88b8916209a4fcd7d1485b820143308ae788192a921a87b`.
- Freeze L0 P1342, SHA-256
  `2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e`.
- Fixture real de 178 bytes, SHA-256
  `98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714`.
- Binding P1342 R3, SHA-256
  `8630a376350d374f854345c37282ac8fbb657f2f9a8506b7de47bda7b8276bf5`.
- Parada focal R5, SHA-256
  `43eb0716901da8d49835bfef72508900dffb2461e3f5d7cb0796e6eaed8de713`.

Medição anterior à escrita deste inventário, em
`2026-09-10T20:42:03-03:00`: HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada,
SHA-256 de `git status --porcelain=v1 -z`
`efeeabd2ccabc874cd7faf204f7655af56ecc0e14102bb403cee47ef86a8a710`,
SHA-256 de `git diff --binary HEAD`
`f73da54bdbeb1bd086caeff259b9517bdbf0f1648efcb8119d36699f5bcf0be5`
e stat `72 files changed, 10889 insertions(+), 830 deletions(-)`.

## Resultado da auditoria L0 e candidate-free

Os dez pares owner/consumer continuam byte-idênticos ao freeze P1342:

| owner L0 SHA-256 | consumer candidate-free SHA-256 |
|---|---|
| `infra/pipeline/context_stabilization.md` `ee9b2fda…` | `03_infra/src/pipeline/context_stabilization.rs` `2f03c9e5…` |
| `compiler/eval.md` `580849e8…` | `01_core/src/compiler/eval/mod.rs` `8706947c…` |
| `compiler/eval/bindings/value_methods.md` `2a0febd2…` | `01_core/src/compiler/eval/bindings/value_methods.rs` `26865ce0…` |
| `compiler/stdlib/counter.md` `a7739f41…` | `01_core/src/compiler/stdlib/counter.rs` `239561a4…` |
| `entities/func.md` `92b69388…` | `01_core/src/entities/func.rs` `4c1c0ca9…` |
| `entities/elements/counter_update.md` `3a8cb1f6…` | `01_core/src/entities/elements/counter_update.rs` `d417085e…` |
| `compiler/introspect.md` `13b5b367…` | `01_core/src/compiler/introspect.rs` `6d2de23b…` |
| `compiler/introspect/from_tags.md` `04596c56…` | `01_core/src/compiler/introspect/from_tags.rs` `92d87a56…` |
| `compiler/eval/call_dispatch.md` `9d00297e…` | `01_core/src/compiler/eval/call_dispatch.rs` `16fa614b…` |
| `compiler/eval/closures.md` `4909fe13…` | `01_core/src/compiler/eval/closures.rs` `58b0a334…` |

Busca fechada por `p1342|P1342|P1343-CAPSULE` nesses dez consumers não
encontrou ocorrência. Isto confirma apenas ausência textual de candidato nos
owners produtivos junto com os hashes integrais; não transforma o teste P1342
histórico nem o observador P1340 existente em implementação válida.

Os L0 já legitimam carrier, ledger, hooks reais e fachada estritamente sob
`cfg(p1339_observation)` ou `cfg(all(test, p1339_observation))`. O P1343 muda a
representação da prova de diff, não contrato público, default, compatibilidade
nem fase; não foi encontrada necessidade de editar L0. Se o candidato exigir
um ponto fora destas dez unidades ou uma superfície ADR-0127, esta conclusão é
refutada e a cadeia deve parar.

## Topologia medida antes da decisão

- `Session::{discover,realize,execute,stabilize}` contém as âncoras reais de
  H00D/H00S/H01/H02/H03/H16; a fachada test-only pode permanecer no mesmo owner.
- `EvalContext` e `eval_expr::Expr::Dict` contêm respectivamente o transporte
  H04 e a produção ordenada real H05.
- `eval_counter_method_value` conhece `args.span()` antes da chamada que o
  perde; o owner counter tem a classificação efetiva e o constructor real.
- `Func` é tuple struct com dois campos e possui oito rotas construtoras no
  próprio owner; todas precisam de forma observada explícita se o terceiro
  campo cfg-only H08 existir. `Func::with` é separada como H09.
- `CounterUpdateElem::to_payload`, `walk`,
  `resolve_filtered_counter`, `apply_func` e `apply_closure` contêm as arestas
  reais H10–H15. Nenhuma exige novo consumer fora do freeze.

## Decisão de baseline de cápsulas

`p1343-capsule-baseline-r1.json` contém 36 cápsulas iniciais em dez arquivos:

| arquivo | total | insert | replace | bytes exatos de reposição |
|---|---:|---:|---:|---:|
| `context_stabilization.rs` | 10 | 10 | 0 | 0 |
| `eval/mod.rs` | 4 | 4 | 0 | 0 |
| `value_methods.rs` | 1 | 0 | 1 | 149 |
| `stdlib/counter.rs` | 2 | 1 | 1 | 572 |
| `entities/func.rs` | 11 | 2 | 9 | 1256 |
| `counter_update.rs` | 1 | 0 | 1 | 191 |
| `introspect.rs` | 1 | 1 | 0 | 0 |
| `introspect/from_tags.rs` | 3 | 2 | 1 | 100 |
| `call_dispatch.rs` | 2 | 2 | 0 | 0 |
| `closures.rs` | 1 | 0 | 1 | 72 |

Cada `replace` guarda os bytes candidate-free exatos em base64, seu SHA-256 e
comprimento. Cada `insert` guarda a reposição vazia, SHA-256
`e3b0c442…` e comprimento zero. Toda cápsula possui âncoras anterior e
posterior em UTF-8/LF com SHA-256 e cardinalidade integral 1 no arquivo
baseline. A concatenação `before + replacement + after` também ocorre
exatamente uma vez para as 36 entradas.

As âncoras foram escolhidas fora dos cores de todas as demais cápsulas: nenhum
range `replace` nem ponto `insert` cai dentro da âncora de outra entrada. Isso
permite resolver as cápsulas na fonte live sem depender de linha estável e
normalizá-las em offsets decrescentes. Os aliases H do JSON fecham exatamente
os 19 rows do binding, e `test-facade` é o único papel que não é row.

O inventário não desenha corpo candidato. As regras por cápsula limitam apenas
owner, símbolo, espécie insert/replace, cfg, posição causal e proibições. O
autor do contrato/oráculo ainda deve provar scanner lexical, normalização,
branch normal equivalente, escritor único, projeção read-only e coverage real.

## Veredito e limitações

Veredito: `CANDIDATE_FREE_CAPSULE_BASELINE_READY_NOT_SEALED`.

A representação é finita e suficiente para uma primeira materialização de
H00D/H00S/H01–H16 sobre a topologia hoje medida. Ela não prova que um corpo
candidato compila, alcança a fixture ou satisfaz cardinalidades; também não
executa corpus e não autoriza implementação antes do futuro selo. O workspace
é compartilhado, portanto isolamento técnico não é atestado. O resultado é
restrito ao binding focal P1342 e não fecha P1340, NT01–NT06, retenção geral ou
política terminal.
