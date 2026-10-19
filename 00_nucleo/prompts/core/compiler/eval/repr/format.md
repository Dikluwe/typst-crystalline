# L0 — compiler/eval/repr/format
Hash do Código: e319199a

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/repr/format.rs`. At HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` with a clean tree, `01_core/src/compiler/eval/repr.rs:1574-1629` contains the separated behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. The purpose is to make algorithms discoverable by behavior, with no presumed performance gains.

## Contract

Format lists and groups with separators, singleton commas, a 50-byte horizontal limit and two-space vertical indentation. Preserve each piece's content and order.

The detailed contract and historical parity evidence remain in `00_nucleo/prompts/core/compiler/eval/repr.md`; extraction preserves output, public signatures and phase. No I/O or mutable global state is permitted.

## Acceptance

Preserve language morphology tests, build, and pass `crystalline-lint .` with a valid owner and hash.
