# L0 — core/compiler/eval/stdlib_snapshot/tests
Hash do Código: 8447fe5f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/builtin-root-retention.toml sha256:08e9f23ba773c1fd5ed43bf67a985c2593f0c002b0fb6084ef9f883a0ef020c7


**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/stdlib_snapshot/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130, ADR-0131.



## Measurement before decision — laboratory builtin graph, 2026-11-13

Base is frozen builtin-root, HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2,
uncommitted laboratory composition. freeze-builtin-root.json records exact
UTC/source/binary hashes. Its census still observes1411 Scope::define calls
per plain/layout warm compilation despite zero registry builder calls.
stdlib_snapshot.rs:42,80-248 recreates Modules/functions; eval/mod.rs:877-917
and modules.rs:60-87 repeat global composition. This is construction mechanics.

builtin-identity-audit/receipt.json records pinned vanilla64c5e8c0 queries at
2026-11-13T07:16:50Z and negative controls at07:19:05Z, source/fixture hashes,
argv, HEAD and checkout diffstat. Language comparisons root/import sym,std,
calc,raw.line return true upstream and false in line-products. Local aliases
stay equal and a user module named calc remains unequal in both. Upstream
typst-library/src/lib.rs:231-247 constructs one Library graph, and typst-eval/
src/lib.rs:42,67,105,151 gives each file that library. This corroborates observed
language behavior, not an assertion of upstream intent from behavior alone.
Sharing the immutable builtin graph is an inference supported by these probes;
stale inputs/features, authority leakage, changed diagnostics or failed negative
homonym/registry tests refute it. Separately created raw.line.with() equality
is a known independent debt and is outside this experiment.

User authorizes experiments without another permission, laboratory only.
The current contract below supersedes the old per-file builtin instantiation
requirement, including the prior deliberate cross-file std equality debt.
No changes to Module/Func/Value equality or user-module evaluation are authorized.

## Current exclusive contract

Own tests of real public eval and private graph helpers. Change only prior builtin freshness expectations superseded by pinned language evidence. Matching root/import Modules and raw.line become equal; separately evaluated user modules and constructed With products remain outside this change. Different ordered inputs/features and separate sessions still use distinct builtin graphs. Preserve all query counts, mismatch ancestry, feature/target gates, authority/homonyms, syntax/cycle/diagnostic trace tests, edits, current registry constructors and all registry domain checks.

Replace obsolete Instance cardinality tests with real StdlibSnapshot raw-view tests for0/1/17/257/4096, sparse/excess capacity, nested unequal namespaces, aliases/With occurrences/spans and independent mutation. Those synthetic private fixtures do not broaden production registry admission. Require actual shared binding pointers as a mechanics control in addition to language comparisons. Keep tests that mutate a clone and verify the original and sibling view remain intact. A read-only global view must expose one stable std Module, current colors/text and ordered shadowing semantics. Full key transitions and bounds/drop tests remain. Do not remove negative different-key identity tests or change global equality to pass.

Final controls additionally exercise actual expression imports, oversized-input
session roots (no cross-root graph retention but shared same-root imports), and
global Scope storage with stable std identity and detached registry-like overlay.
The nested/package test retains numeric values, query counts and resolution
ancestry; only same-key calc comparisons follow the new builtin graph contract.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.

## Builtin identity evidence for productive adoption

Before choosing the productive identity contract, identity-probe/main.typ was
queried on the frozen current product, finalized laboratory candidate and pinned
64c5e8c0 oracle. Six language comparisons returned false×6 before,
(true,true,true,true,true,false) in the candidate and true×6 in vanilla. The first
five concern builtin calc/std/raw.line graph identity, not Rust-only equality.
Adopt that measured builtin correction and preserve alias/input/feature/mutation
controls; four former per-file/seed-only tests are superseded by their matching
shared-graph tests, not silently dropped. Context creation remains lazy until
composition; its renamed test is otherwise unchanged. The sixth comparison of
repeated user imports remains a preexisting non-parity debt, not closed here.

## Strict test qualification for adoption

Preserve every existing test assertion, reference comparison and negative case.
Remove redundant cfg/borrows, use a named private snapshot type and array, and
pass configured raw-shaping inputs as the existing named RawShapeInput value
instead of a nine-argument helper. Test FontList HashMap/HashSet contracts through
generic Eq+Hash helpers so immutable regex backend caches do not trigger a
concrete mutable-key lint; keep actual map lookup, deduplication and field coverage.
No lint suppression or productive algorithm change is permitted by this cleanup.
