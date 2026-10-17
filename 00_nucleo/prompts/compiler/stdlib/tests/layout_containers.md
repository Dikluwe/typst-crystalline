# L0 — compiler/stdlib/tests/layout_containers
Hash do Código: dc726914

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/layout_containers.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the layout_containers regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Saneamento do CI — 2026-10-13

Estado medido: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree
não commitado. Fontes, hashes, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/testes-ci-2026-10-13/before.json.gz`. O inventário
Clippy de entrada é `matematica-ci-2026-10-13/clippy-inventory.json`, conservado
no evidence.json.gz daquela rodada. Medir antes de decidir (ADR-0108).

Medição: o inventário aponta assertions de igualdade com bool literal e
bindings de pattern não consumidos no consumer. Os valores observados incluem
clip, breakable, sticky e overhang.

Decisão: expressar bools por assert!/assert!(!...), mantendo polaridade e
mensagem; renomear bindings não consumidos sem remover a expressão, execução
ou o pattern. Preservar todos os inputs, defaults e controles negativos.
Nenhuma mudança de código produtivo ou nova exceção de lint.

Reparo interno de testes, ADR-0127 contínuo. O RED de qualidade é o Clippy
com -D warnings. Reexecutar os casos e validar linhagem; testes sem falha
funcional anterior não recebem alegação artificial de RED→GREEN funcional.
