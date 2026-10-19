# Core L0 sanitization

## Scope and outcome

Only prompts matching compiler/eval/observation*, compiler/parse/parser*,
compiler/stdlib/registry*, compiler/stdlib/tests*, entities/content* and
entities/syntax-node* were changed. Normative prose is English. Canonical
tool metadata keys `Hash do Código` and `Núcleos Tekt` remain unchanged for
schema compatibility; these are not normative prose.

Process metrics, steps, timestamps, working-tree inventories and accumulated
historical narratives are absent from these L0 prompts. The complete original
documents are preserved in core-l0-sanitization-history.md, a diagnostic archive
that grants no implementation authority. Existing atomization diagnostics
remain unchanged. Content's root owner now describes only its closed enum,
payload declarations and module composition; behavior and unrelated eval,
stdlib, layout and export contracts are not duplicated there.

Each prompt identifies its sole consumer. Nucleus pins are unchanged. Code
hash metadata was recalculated from unchanged source bytes. The manifest
records original prompt/source SHA-256 and each expected effective L0 hash.

## Verification

- `crystalline-lint --checks v15,v26 --fail-on warning .`: exit 0,
  no violations.
- `crystalline-lint --checks v5,v6,v15,v26 --format sarif .`: exit 0,
  with exactly 75 warning-level V5 reports, one per changed owner; no V6,
  V15 or V26 reports. Default exit 0 is not a claim that lineage is sealed.
- Independent verification matches all 75 expected effective prompt hashes
  against the linter and verifies all consumer files remain byte-identical
  to their pre-edit SHA-256 in the manifest.
- No productive Rust edits, builds, tests or global formatting were performed.

## Reseal completed after explicit permission

The user explicitly authorized updates limited to `@prompt-hash` lines.
All 75 manifest consumers were resealed with individually calculated effective
L0 hashes, after V15/V26 preflight passed with `--fail-on warning`.
No global --fix-hashes or formatting was run. Source code hashes were checked
against L0 metadata before writing. Each file was compared before/after with
only its `@prompt-hash` line excluded: all other bytes remained identical.

The final V5/V6/V15/V26 SARIF check has zero findings for the 75 scoped
consumers. A global `--fail-on warning` invocation returned exit 1 because
of three unrelated V5 warnings in compiler/layout/mod.rs, parbreak.rs and
tests.rs. The subsequent SARIF snapshot additionally reported seven V0
findings in other layout consumers during concurrent edits. These out-of-scope
files were not changed. This is a scoped clean result, not a claim of globally
warning-free lineage.
