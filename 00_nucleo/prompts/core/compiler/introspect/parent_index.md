# L0 — introspect/parent_index
Hash do Código: 1f01298e

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/parent_index.rs`.
At clean-tree state `df72a6c4a701917ab56a7c6169764a92ea775693`, `01_core/src/compiler/introspect.rs:1693-1746` contains the behavior assigned to this unit. Source and state are recorded in `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. Separation is by behavior, preserving the API and without a performance claim.

## Contract

Build parent relationships by pushing real Starts and popping Ends; derived records must neither invent ancestors nor overwrite occurrence relationships.

Language invariants, causal order and parity limits remain defined in `00_nucleo/prompts/core/compiler/introspect.md`; this owner must not create a second interpretation. Code uses static L1 calls without vtables, I/O or mutable global state.

## Acceptance

Preserve introspection tests and the L1 suite, build, and pass `crystalline-lint .` with consistent hashes. Future semantic changes require language measurement and an L0 update before code.
