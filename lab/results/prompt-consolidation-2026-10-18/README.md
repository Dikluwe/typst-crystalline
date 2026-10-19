# English prompt consolidation evidence

The source checkpoint is `ef4a08ec3f48b2eecc470d9b2582349665cb29c1`, tagged
`tekt-before-prompt-consolidation-2026-10-18`. This directory records editorial
consolidation on `codex/tekt-mature-layout`; it does not authorize new behavior.

- `coverage-final.json` and `reviewed-final-before-repin.json`: full-read coverage
  of every owner and nucleus, with per-file source/current digests and review ledgers.
- `final-integrity.json`: final document digests, original archive verification,
  stable reverse seals, stable nucleus structure/edges, effective pins, and proof
  that each changed Rust file differs only in its forward lineage seal.
- `final-text-audit.json`: historical paths, canonical references, semantic
  headings, and individually reviewed literal language exceptions.
- `repin-receipt.json`, `preflight-commands.json`, `reseal.log`: effective nucleus
  hash and forward lineage update. The rejected raw-hash attempt is retained;
  V15/V26 passed after its correction and before any Rust resealing.
- `lint-comparison.json` and `final-lint.sarif`: exact comparison with the previous
  complete linter baseline, including the existing V16 debt.
- `validation-summary.json`: all 12 jobs passed, with 34 Python tests, 9,128
  Rust unit/integration tests and five doctests; seven existing ignores remain.
- `validation/`: reproducible command records/logs, source freeze, full source
  provenance, observer/oracle/linter identities, and resource limits.
- `fresh-checkout.json` and `fresh-input-equivalence.json`: isolated Git-export
  tree, its path/lineage/Cargo checks and equality of all 2,130 frozen inputs.
- `compiler/peer-review/`, `boundaries/`, `entities/`, `layout/`, `nuclei/`:
  full-file review ledgers, bounded approval reconciliation, semantic cross-checks
  and readability corrections. Intermediate progress files retain their timestamps;
  final aggregate and final input digests supersede earlier coverage snapshots.
- `audit-scripts/`: exact supplementary task scripts, with their assumptions.

Exact original files and the byte/SHA-256 manifest are under
`lab/history/prompt-consolidation-2026-10-18/`. Those historical originals retain
their language and chronology and never replace the active 1:1 owner contract.
The human-readable outcome is
`00_nucleo/diagnostics/prompt-consolidation-2026-10-18.md`.
