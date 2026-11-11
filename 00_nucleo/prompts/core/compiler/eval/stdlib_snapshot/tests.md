# L0 — core/compiler/eval/stdlib_snapshot/tests
Hash do Código: f6e29445

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

## Known-cardinality instantiation regressions

Source evidence at received HEAD `0a15ccebf861e3cc0ea16d0397d55dc057664556`, before this reservation change:
`01_core/src/compiler/eval/stdlib_snapshot.rs:80-85` constructs
one fresh destination from an immutable Scope and fills it in declaration order;
recursive module and native-function namespaces use this same private boundary.
`01_core/src/entities/scope.rs:96-102` exposes cardinality and emptiness without
exposing map capacity. Reservation is an internal RAM policy (ADR-0029/0030),
not a new language-parity criterion. Observable binding order, values, aliases
and fresh identities remain the semantic regression contract.

Append pure descendant tests prefixed `known_cardinality_`, keeping all existing
tests and helpers. Exercise the actual private Instance scope boundary with
0/1/17/257/4096 bindings and a large→small→empty sequence on one Instance;
owned unsorted keys must retain declaration order, values and immutable source
bindings even when a returned scope is mutated. A source with excess reserved
capacity and few actual bindings is a semantic control, not proof of the
allocation choice. Test nested modules of unequal sizes including empty scopes,
local aliases and separate identities across instantiations. Exercise the
qualified real raw native namespace and raw.line With wrapper with aliases in
small and larger roots. Keep source identity intact and aliases local; reuse
existing registry controls for the wider representation/metadata contract.

Do not expose Scope capacity or a new public test portal here. Inline Scope
tests owned by `00_nucleo/prompts/core/entities/scope.md` independently check
initial reservation, no growth through the known cardinality and lawful growth
past actual capacity. These descendant tests establish semantic preservation;
they do not alone prove which initial reservation was selected.
