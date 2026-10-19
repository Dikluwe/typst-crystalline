# V16 batch C correction evidence

Status: nineteen selected V16 findings validated and closed; 64 remain open.
Implementation checkpoint: `370d78355bd646d1abead5bcacef215288cc8834`, branch
`codex/tekt-mature-layout`. This is diagnostic evidence, not an L0 specification.
The nineteen selected findings and twenty-two exclusive source/L0 pairs are
recorded in `cases.json` and `scope.json`. Internal helper reexports account for
three additional pairs; contextual selector validation and constructor supplement
provenance account for one pair each.

## Evidence and authority

`baseline-state.json.gz` binds the fresh baseline to the checkpoint, UTC, source
state and pinned linter. The baseline has 83 V16 errors, 1,166 V19 notes and 878
V20 notes. The unchanged linter executable SHA-256 is
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`, from commit
`12797073972e375d9ab078cf6750c8b30b71e2b9`. No rule, severity, scope, exception,
configuration or suppression is changed. Informational grouping reminders stay
visible and require review rather than erasure.

Language probes use the ratified oracle commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Agent receipts preserve the recipe reference, source text, arguments, UTC,
checkpoint, exact uncommitted diffstat and binary hashes. Version text is not
used as identity. PDF probes compare extracted language text, never PDF bytes.

Three agents owned disjoint L0/source sets and cross-reviewed each other. The
coordinator owns sealing, compiler experiments, full validation and integration.
All productive repairs follow their sealed L0. One existing Format-identity
Nucleus is newly pinned by equality; its contents and the public equality API
are unchanged. A new shared Nucleus governs private explicit Heading supplement
provenance between its constructor and the diagnostic consumer. No new public contract, default, phase or compatibility change
is authorized by this batch.

## Baseline controls and repairs

`preimplementation.patch.gz` and its receipt capture the final test/L0 state
before productive repairs. `baseline-final/` is frozen and records 22 passing
controls, 18 genuine semantic failures and one intentional paragraph diagnostic
panic. The diagnostic is measurement instrumentation, not a regression test or
semantic RED; its source and output remain in lab evidence. Earlier fixture and
compile failures are retained but do not count as language evidence.

Structural owners preserve current finite classifications, errors, guard order,
nonexecuting refusal, numeric behavior, static paths and snapshot authority.
Payload guards move inside nominal arms so future enum additions cannot inherit
a decision silently. Selector repairs restore operand admission, contextual
boundary validation, typed native identity, nominal Format identity, represented
kind conversion, causal carrier identity and mathematical element matching.
Receiver eligibility remains bounded; binom's unlined internal fraction does not
acquire frac identity. Aliases do not establish identity by name.

Located content now retains its rich content body at markup, code-tail and math
insertion boundaries. This does not transfer query identity or mutate captured
fields. Generic value presentation remains a separate obligation. Discarded
return warnings recognize update nodes and measured static public descendants,
without executing contextual callbacks; additional independent review and RED
controls refine field traversal in the owning diagnostic helper.

The revised paragraph contract preserves the existing early prepared-Par
recognizer. Three preservation controls passed before its finite complement was
implemented (`candidate-controls-2/`, 43 total controls passed, frozen inputs).
The initial four-versus-two paragraph effect hypothesis was refuted: extra
effects belong to space-only paragraphs. Raw Heading/textual eager preparation,
query-Par errors and completed-carrier reentry remain unresolved language debts.
Constructed carrier stops are internal-stage observations, not a claim that their
bodies are nonparagraph, unreachable, or exempt from transparency contracts.
The independent stage audit and exact refuter are retained under `agents/` and
the owning prompt. Exhaustiveness alone does not close paragraph parity.

## Independent review refinements

The supplemental source tests captured four valid semantic failures for Ref,
native RawLine, constructor Heading and deferred PageRun properties; 44 other
controls passed with frozen inputs (`supplement-red/`). Local recursion fixes
avoid changing the shared visual visitor. A second independent review refuted
using a general constructor marker plus the merged public supplement: a
numbering-only constructor can inherit a set-rule supplement. The private
original-Value snapshot fixes this provenance problem with a shared Nucleus.
`heading-red/` records 48 passing controls, two remaining semantic failures and
the existing A1 public identity/presence/reflection test passing before the
snapshot implementation. Its input snapshot is retained separately.

The new nucleus initially used its raw-byte SHA as a pin; V26 rejected it before
any resealing. `heading-nucleus-pin.json` records the correct domain-framed
effective SHA from the pinned linter algorithm. No graph check was bypassed.

The first complete candidate (`candidate-green/`) passed all 50 then-current
controls, the A1 control, a fresh workspace build, 60 bilateral selector cases,
23 bilateral warning cases and rich morphology from an actual query producer
at three insertion routes. It also passed all seven structural experiments.
Its full lint scan was nevertheless rejected: a direct regex escape call added
V14. `initial-candidate-lint/` preserves that rejected state. The owning prompt
requires a pure local char/String transform using the verified locked syntax,
with no dependency allowlist or wrapper API change. Two further preservation
controls cover every metacharacter, nonliteral alternatives, Unicode, control
characters, empty refusal and language-visible regex pattern identity.

Figure/Equation supplement diagnostics remain observed producer-provenance
failures, explicitly separate from local acceptance in the replay. They retain
the required language result and exact follow-up owners; they are not ratified
omissions. Refined paragraph probes likewise preserve unresolved obligations.
One resource-run approval timed out before launch; its permitted single retry
succeeded. This timeout was neither a test failure nor a security finding.

## Structural proof and validation method

`../workers/batch_c_structural.py` copies the complete workspace and uses the
actual locked dependencies, declarations and test target. It adds a sentinel to
real Content, Value, Type, Selector, ElementKind, Expr, P1293HtmlNative and
FuncRepr domains only in disposable copies. Each selected decision needs its own
primary E0004 at the exact file, line and byte span. Unrelated failures do not
count. Selector and FuncRepr pair domains separately isolate new/old, old/new
and new/new. Nested and newly introduced finite helper decisions receive their
own witnesses; they are not misrepresented as original selected findings.
Baseline receipts show no selected E0004 at all nineteen old decisions; the old
FuncRepr accessor decision has its own baseline experiment.

All heavy commands run serially, offline and locked, with one Cargo job, a
12 GiB maximum, 8 GiB high threshold and zero swap. Receipts read actual cgroup
limits, peak and OOM counters. Source/L0 input freezes must remain unchanged.
Copied-workspace experiments reuse the product target cache; the four product
packages must be cleaned before the final real-checkout build and full tests.
Oracle and dependency caches are preserved. Final acceptance includes paths,
forward/reverse lineage, V1/V5/V15/V26, full workspace builds/tests, wiring,
doctests, all targets including lab benches, strict Clippy, formatting and
Rustdoc. A full linter exit of 1 remains expected for other batches; it is not
an overall zero-violation result.

## Accepted closure

The final scan ended at `2026-10-19T13:39:03.479605+00:00`: **83 → 64 V16**,
with exactly the selected nineteen removed and no new error or warning.
`lint-delta.json` verifies unchanged diagnostics outside the declared source
scope and the one retained V16 shifted by two formatting lines inside it.
V19 is 1,195 and V20 is 878; `note-review.md` explains every content change.
The full scan still exits 1 for the remaining findings.

`baseline-test-classification.json` distinguishes 23 valid semantic RED controls
from 29 preservation controls in the delivered 52-test inventory. All 52 are
observed passing in `validation/tests-domain-infra.log`, together with the
existing Heading constructor/reflection control; `new-test-execution.json`
records their names and the log hash. The core suite reports 7,485 passed and
five preexisting ignored; infrastructure reports 1,377 passed and two preexisting
ignored. No new ignored test is added.

`candidate-final/` records the repaired candidate, all seven compiler experiments
and oracle replays with frozen inputs. `structural-witness-review.json` and its
Markdown companion independently reconstruct the exact mutation and compiler
span for all 25 decision targets and six isolated tuple classes. Other expected
E0004 diagnostics remain in the logs but are not counted as selected witnesses.

The accepted full validation completed all twelve gates at `2026-10-19T13:58:58.141405+00:00` with
2,131 unchanged frozen inputs. `validation-run/` binds the four-package clean,
full validation and final selector/effect replay to the actual checkout.
`validation/` retains commands, stdout/stderr, resources, exact uncommitted
diffstat and source/L0/Nucleus hashes. The accepted binary passes 60 bilateral
selector cases, 23 bilateral warning cases and actual query-produced rich
content in three insertion routes. Two Figure/Equation producer mismatches are
reported separately as open dependencies, never counted as passing acceptance.

`closure-ledger.json` binds each original selected ID to its final owner, source,
hashes and exact structural witness. `open-obligations.json` preserves follow-up
contracts and refuters. Post-document path validation and the artifact manifest
complete the reviewable evidence; none of these diagnostic documents authorize
new productive behavior.
