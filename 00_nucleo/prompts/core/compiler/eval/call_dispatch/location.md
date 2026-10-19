# L0 — compiler/eval/call_dispatch/location
Hash do Código: 3be228cd

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/call_dispatch/location.rs`. At HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` with a clean tree, `01_core/src/compiler/eval/call_dispatch.rs:2126-2207` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contract

Query Location methods with the existing contextual reads and target semantics. Preserve public signatures, diagnostics, interception priorities, traces and argument semantics specified in `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Extraction remains in L1 with static dispatch, without I/O or mutable global state.

## Acceptance

Preserve the L1 suite, call_dispatch tests and `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.
