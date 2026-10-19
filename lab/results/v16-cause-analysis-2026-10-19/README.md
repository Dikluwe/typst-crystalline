# V16 cause analysis evidence

This is a plan-only record for clean product HEAD
`23bcda80d8348b08e5199312fddeec8ece2eb0a5` on
`codex/tekt-mature-layout`. Proposed behavior tests, compiler mutations and oracle
probes have not been executed. No product source, L0, linter or configuration was
changed. Original diagnostic messages retain the tool's language verbatim.

- [Current concise plan](../../../00_nucleo/diagnostics/v16-cause-correction-plan-2026-10-19.md)
- [105-case index](case-index.md) and [complete merged ledger](case-ledger.json)
- [Execution batches](batches.md) and [machine-readable assignment](batches.json)
- [Evaluation/parse/realization analysis](eval-realize-analysis.md), [JSON](eval-realize-analysis.json)
- [Layout/standard-library analysis](layout-stdlib-analysis.md), [JSON](layout-stdlib-analysis.json)
- [Entities/shell/infrastructure/test analysis](entities-boundaries-analysis.md), [JSON](entities-boundaries-analysis.json)

The individual reports retain their richer domain-specific schemas. The merged
ledger normalizes the common action fields and preserves each original row,
including source/L0 SHA-256, exact diagnostic span, current prompt seal,
uncertainties, refuters and proposed tests. Batch membership is not a replacement
for the per-row gate or the owning L0.

## Fresh measurements

- [Baseline state and pinned executable identity](baseline-state.json)
- [Compressed complete SARIF](baseline.sarif.gz)
- [Counts and comparison with the prior consolidation result](baseline-comparison.json)
- [105-row source/owner inventory](inventory.json)
- [Strict V1/V5/V15/V26 output](strict-lineage.log)
- [Rule and structural-test precedent source digests](rule-evidence.json)

Full scan command: `lab/target/linter/debug/crystalline-lint . --format sarif`.
It returned exit 1 due to 105 existing V16 errors, with 1,140 V19 and 879 V20 notes.
The strict command `lab/target/linter/debug/crystalline-lint . --checks v1,v5,v15,v26 --fail-on warning`
returned exit 0. The recorded baseline was clean; later changes in this turn are
only these evidence files and the linked diagnostic plan.

The rule source is quarantined at
`lab/scratch/tekt-linter-pinned/01_core/rules/wildcard_saturation.rs`, commit
`12797073972e375d9ab078cf6750c8b30b71e2b9`. Its current implementation and tests,
rather than the status of historical linter ADR proposals, establish observed
V16 behavior. Neither comments nor exception annotations suppress the principal
diagnostic. The snapshot uses no new exception, reduced severity, or exclusion.

## Validation of this analysis

[validate.py](validate.py) checks coverage, unique assignment, preserved inventory
fields, source/owner hashes, populated action fields and report membership.
[validation.json](validation.json) records the outcome and state. The repository's
existing mature-layout checker validates owner paths and active Markdown links;
its [output](mature-layout.log) is retained. These checks validate the analysis
artifact, not the proposed future implementation or Typst parity.
