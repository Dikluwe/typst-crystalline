# L0 — core/compiler/eval/stdlib_snapshot/tests
Hash do Código: 33d47d17

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/stdlib_snapshot/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130, ADR-0131.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8

## Evidence before decision

The separate laboratory tests at
`lab/experiments/stdlib-qualification/tests/{large_inputs,guard_contract}.rs`
observed input/feature/identity boundaries, with executable mutations recorded
in `00_nucleo/diagnostics/stdlib-qualification-2026-11-06.md`. The productive
helper has no previous consumer or private test portal. These controls are
owned here without granting tests ownership of its implementation.

## Independent executable contract

Pure unit tests of the real helper plus public eval with in-memory Worlds;
no files, env, clock, dynamic policy, production counters or public test portal.
Use the helper's private boundary as its descendant, without broadening API.
Declare `#![cfg(test)]` in this test file after its lineage header. Its parent
uses a plain module declaration, matching the existing file-local test-gate
convention and allowing the architectural parser to recognize the test consumer.

The independent actual-EvalContext::new smoke assertion is co-located in the
helper owner; this consumer owns the broader resolver/public-eval suite below.
Cold slots start empty; first resolve returns/populates a seed; repeated equal
inputs/features reuse its identity while producing separate per-file Module,
raw.line and With identities where present, retaining local aliases including
pdf↔format.pdf and asset↔format.bundle.asset. Registry scopes precede eval's
installation of lexical std: do not invent std inside the builder Scope.

Cover ordered full-pair inputs 0/1/512/4096 (>1 MiB Unicode), changes at
first/middle/last pair, reorder/add/remove, mismatch subtree and return to parent
seed. Seed feature combinations, exercise all complete feature transitions and
actual namespace membership/gates (including pdf.table-summary with A11yExtras).
Do not claim a Features change is reachable inside one nominal EvalContext.

Inspect the real registry's complete graph across all profiles, rejecting
unsupported identity-bearing leaves/functions, closures/plugins/cycles or new
dynamic Elements beyond the qualified raw.line. With is checked by private
fixture if absent in the registry; preserve aliases, args occurrences and spans.

Public eval controls cover root without imports, siblings/nested/repeated file
imports, World.inputs once at each original query point (including scripted
responses), fresh roots A/B/A, shadowing/show isolation, feature/target
propagation, file edits and actual full diagnostics/causal traces. Unsupported
World resource requests fail the test; do not silently return success.
Test exact language-observable values, not Debug or output-byte identity.

Keep tests normal/adversarial in this consumer. In disposable RAM, deliberately
share identities, ignore inputs or ignore features and confirm real assertion
failures; no compile-failure credit. Cold legacy builder behavior must provide
a RED control for the newly specified seed invariant before implementation.
