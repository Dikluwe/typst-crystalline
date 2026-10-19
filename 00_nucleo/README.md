# Architecture and specifications

This directory contains the causal specification of the product and its current
architectural record. It contains no executable tooling or build output.

- [Decisions](adr/README.md): stable ADR identifiers, English filenames, and an
  ordered decision index. Superseded decisions retain their identity and status.
- [L0 specifications](prompts): exactly one productive consumer per materializable
  prompt, and one owner per consumer. Shared invariants belong to pinned TOML
  nuclei in `prompts/_nuclei/`.
- [Diagnostics](diagnostics/README.md): current findings and pointers to reproducible
  evidence. A diagnostic does not authorize product code.

The four executable layers remain `01_core`, `02_shell`, `03_infra`, and
`04_wiring`. L0 precedes changes to those layers. The approval boundary in
ADR-0127 and the ownership/nucleus rules in ADR-0129 remain in force.

Under `prompts/`, `core/`, `shell/`, `infra/`, and `wiring/` mirror their
consumer's module path, omitting only the leading `src/` and replacing `.rs`
with `.md`. Preserve `tests/` and actual `mod`, `lib`, `main`, and `build`
basenames. Use semantic source filenames rather than construction-step IDs.
Shared invariants stay in `_nuclei/`. See
[ADR-0131](adr/adr-0131-owner-aligned-prompt-paths.md) for the rule and gates.

New and substantively revised active prose is written in English. Machine-readable
metadata such as `Hash do Código:` and `Núcleos Tekt:` remains unchanged until the
linter schema is explicitly migrated. Historical records preserve their language.
The existing corpus is being translated by reviewed units, not by replacing
technical literals or observable diagnostic messages.

Operational procedures and the prompt template live in
[lab/tools/workflows](../lab/tools/workflows). There is no permanent execution-step
hierarchy. Durable decisions belong in ADRs, product requirements in their L0
owners, findings here in diagnostics, and reproducible evidence in `lab/results`.
The construction period is preserved in [lab/history](../lab/history/README.md).
