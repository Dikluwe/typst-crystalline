# L0 — compiler/stdlib/registry/mod
Hash do Código: 9d73338d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/mod.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Compose a fresh Scope per invocation using explicit inputs and features.
Call registry families in the established binding order. Pass the single PDF
returned by formats to modules. Do not use a dynamic registry, wildcard imports,
reflective lookup, global state or native wrappers. Transport feature arguments
without implementing their behavior. Keep microbenchmarks test-only through its own inner
#![cfg(test)]; do not contain clocks, Criterion or production benchmarking APIs.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Integração condicional dos módulos de teste — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado. A duplicação cfg(test) ocorre em `01_core/src/compiler/stdlib/registry/tests.rs:7` / pai :48; `01_core/src/compiler/stdlib/registry/microbenchmarks.rs:7` / pai :51.
Fontes, L0, UTC, status e diffstat estão em
`lab/results/construction/diagnostics/cfg-test-ci-2026-10-13/before.json.gz` e
`refinement-before.json.gz`. O Clippy pinado reproduz os atributos duplicados.
A tentativa de retirar a condição interna mantém os testes Rust, mas o linter
arquitetural pinado não propaga a condição do pai e acusa V2 nos helpers.

Neste owner, retirar apenas o atributo externo cfg(test) das declarações
listadas acima. Manter os caminhos, nomes e visibilidades dos módulos e seus
atributos path. A condição interna `#![cfg(test)]` de cada arquivo listado
continua a excluir integralmente o módulo do build de produção, inclusive
seus imports e helpers. Os consumers de teste e seus L0 permanecem intactos.
Não acrescentar dependências, lógica, API, modo padrão ou fase. Validar a
mesma lista de testes, suíte core, build sem cfg(test), Clippy e arquitetura.
