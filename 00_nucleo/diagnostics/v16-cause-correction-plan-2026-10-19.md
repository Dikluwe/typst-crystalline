# V16 cause correction plan

Status: analysis and proposed implementation sequence only. No productive source,
L0 contract, linter, configuration, or test expectation was changed by this
analysis. The 105 errors remain open. This diagnostic does not authorize code;
each implementation batch must first update its owning L0 specifications.

## Evidence before classification

At clean HEAD `23bcda80d8348b08e5199312fddeec8ece2eb0a5`, branch
`codex/tekt-mature-layout`, the full pinned linter ran from
`2026-10-19T10:03:25.131622+00:00` to `2026-10-19T10:03:30.887833+00:00`.
It reported **105 V16 errors across 81 owners**: 82 occurrences in core, three
in shell, and 20 in infrastructure. There are no wiring occurrences. The full
result also contains 1,140 V19 notes and 879 V20 notes, and matches the previous
consolidation result exactly by rule, level, message, and location. Strict
V1/V5/V15/V26 validation passed. These are lint measurements, not language tests.

The executable is `lab/target/linter/debug/crystalline-lint`, SHA-256
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`, from
linter commit `12797073972e375d9ab078cf6750c8b30b71e2b9`. The exact command,
UTC interval, clean state, source/L0 digests, and complete diagnostics are in the
[evidence directory](../../lab/results/v16-cause-analysis-2026-10-19/README.md).
Three agents read the containing functions, guards/callers, enum definitions,
and current owning L0 clauses for disjoint partitions of 33, 37, and 35 findings.
The primary agent reviewed the proposed repairs and cross-checked sensitive
carrier, arity, conversion, and traversal cases.

V16 is a syntactic signal, not a proof of a runtime defect. In the pinned rule,
`wildcard_saturation.rs:46–85` identifies candidate enums by qualified arm
prefixes and exempts some expression/body shapes; it does not resolve Rust
types. Lines 178–189 emit the principal finding even with an exception
annotation. The rule's assessment test verifies that behavior. Consequently,
neither a justification comment nor a different syntax proves a correction.
This inventory is complete for the 105 emitted findings, not for every latent
catchall in the repository.

## Causes and correction decisions

1. **Valid partial projections lack an explicit decision for enum growth.**
   Operator-token maps, lenient font variations, unsupported casts, JSON leaves,
   and native-format delegation intentionally return `None`, ignore an entry,
   or delegate. Preserve those outcomes and classify current alternatives by
   name, with positive, negative, and boundary controls. A future variant must
   require a decision at the owning match. Never turn legitimate rejection into
   panic or introduce a new conversion merely to remove V16.
2. **Conservative optimization proofs are not general semantic dispatch.**
   Grid measurement cache eligibility and initial-layout independence deliberately
   reject unproved content. Preserve the general path and test its observable
   equivalence, including effects and diagnostics. `None` can mean unavailable
   cached measurement; it must not become zero geometry or missing content.
3. **Traversal callbacks have distinct replacement and root semantics.**
   A no-replacement callback can still traverse descendants. Located suppresses
   the body-root callback (`entities/content/traversal.rs:436,667–673`), while
   replacements are not automatically revisited. Audit root recognition and
   identity before listing terminal classes. The suspected duplicate collection
   of `Located(ContextBlock)` was refuted: retain its explicit producer case.
4. **Some complements combine justified defaults with unresolved obligations.**
   Content reflection, equation carriers, Rust content equality, HTML marker/frame
   projection, numeric conversions, and approximate list summaries require
   targeted characterization before choosing the nominal complement. Do not
   freeze an unexplained omission as an accepted contract.
5. **The diagnostic can describe the wrong domain.** V16-056 matches an argument
   slice, not a single enum. Its complement is zero or at least two arguments.
   Preserve the separate one-argument type error and use an exhaustive arity
   partition such as `[] | [_, _, ..]`; do not enumerate unrelated `Value`
   variants or weaken the rule.

The complete per-occurrence ledger records the actual function, scrutinee,
source/L0 evidence, complement meaning, proposed repair, acceptance/refutation
controls, dependencies, uncertainty, and ADR-0127 classification. The
[execution batches](../../lab/results/v16-cause-analysis-2026-10-19/batches.md)
assign every ID exactly once. Batch assignment is scheduling, not permission to
skip the individual contract or unresolved prerequisite.

## Decisions that must remain open until measured

- **V16-001–004:** native reflection catalogue and nested equation carriers.
  `equation_descendant` only crosses Styled, whereas the entity field projection
  crosses Source/ShowRecipe/Located/Realized.visual. Test nested carrier positions
  and the promised operation before declaring a language bug. Preserve ordinary
  Styled opacity and the scope of the Sequence reflection contract.
- **V16-007/011/012/019/021/026/027:** selector admission/identity, discarded-effect
  warnings, rich-value display, and paragraph recognition have scoped or mixed
  policies. The per-case report distinguishes each measured route from a possible
  gap. Catalogue membership, helper similarity, or an unhandled wrapper alone
  does not authorize broader show/query/display/flow behavior.
- **V16-055:** Figure inference already has the named FG1 limitation in its L0;
  recursive widening is not an automatic lint repair.
- **V16-037/040/053/059/067:** bounded measurement approximations, the higher-order
  constructor subset, invalid-track filtering, and optional locale casts must
  retain explicit limitations. Characterize their actual callers before claiming
  that unsupported means semantically invalid or unreachable. Any expansion or
  stricter validation needs its own measured decision in the existing owner.
- **V16-060/065/066:** numeric-to-zero fallbacks need call-path/type evidence and
  current-pin probes where observable. The ellipse owner explicitly preserves
  fallback behavior. No new invalid-input policy is authorized by this plan.
- **V16-074:** separate Rust heterogeneous inequality and historical always-false
  cases from uncovered same-kind HtmlElem/HtmlFrame/Title/ContextBlock behavior.
  Rust `PartialEq` is not the language equality contract (ADR-0107).
- **V16-088/091/092/095:** test transported marker/equation/frame roots and residual
  bundle containers. Correct a local projection only if its promised behavior
  and reachability are demonstrated; do not globally change carrier traversal.
- **V16-102/103:** raw empty Sequence statically satisfies `all(empty)` and yields
  one group, but the owner documents approximation and normalized construction
  differs. Record empty/singleton/wrapper controls and resolve intended scope;
  no executed CLI or vanilla defect has been established here.

Any language probe must verify the current oracle commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, build recipe and executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Record candidate HEAD, exact uncommitted file list, UTC, inputs, arguments,
outputs, and binary identities. Old receipts and version strings cannot close
these questions. Compare semantics, syntax, and morphology; preserve intentional
mechanical divergence and do not copy vanilla defects.

## Implementation protocol for every batch

1. Reproduce the baseline and remap IDs by function/span if lines moved. Read the
   current enum and full owner; do not apply stale line-based substitutions.
   Resolve each guarded arm's residual payloads and nested enum axes, not only
   top-level variant names. Audit causally adjacent hidden catchalls as specified
   by the row; do not relocate the same hole into a helper.
2. Update the exclusive L0 with evidence before decision, the accepted complement,
   preserved limitations, and meaningful acceptance/refutation tests. Keep 1:1
   owners and pinned Nuclei valid. Public contracts, default behavior, phase or
   compatibility changes require the ADR-0127 decision gate; internal and measured
   parity corrections follow its continuous flow. Classify uncertainty before code.
3. Run focused behavior controls before and after. For preservation work those
   controls should remain green; the missing *structural* obligation supplies
   RED→GREEN. For a demonstrated semantic defect, require a language-level failing
   witness before the implementation and passing witness afterward as well.
4. Use the existing precedent in
   `00_nucleo/prompts/core/entities/value.md:154–160`: in a disposable complete
   workspace under `lab/`, add a sentinel variant only to the copied relevant
   enum and inspect compiler JSON. Baseline lacks E0004 at the selected match;
   the repaired version must report E0004 there. Failure elsewhere is not success.
   Include test targets, nested domains and tuple axes where relevant. For an
   external closed enum, use a copied pinned dependency. No sentinel enters
   productive code. For V16-056 prove exhaustive slice cardinality and preserve
   the one-item type distinction instead of inventing an enum mutation test.
5. Implement explicit, reviewed classifications with shared bodies for identical
   outcomes. Preserve static exhaustive dispatch, L1 purity, layer ownership,
   order, errors, effects, identity and approved target boundaries. No new public
   facade or Nucleus is needed merely for a negative list.
6. Check V15/V26 before restricted resealing; validate forward and reverse seals,
   V1/V5/V15/V26 and exact per-ID V16 removal. Inspect the full SARIF delta: no
   unrelated disappearance, new error, or warning is acceptable. Review V19/V20
   notes by substance; they are not execution counts and must not drive body
   duplication or suppression.
7. Run focused tests per owner, then the workspace build/tests, all-target checks
   including lab benches, strict Clippy, formatting, Rustdoc and path/lineage
   validation for the integrated result. Use the established one-job, 12 GiB,
   zero-swap lab validation setup. Preserve complete commands, resource settings,
   outcomes and state receipts. Run target-specific export/watch/geometry controls
   required by the affected owners; a generic cargo pass is not parity proof.

Implementation can delegate independent owners concurrently, but one coordinator
must own shared enums, cross-owner semantics, integration and resealing. Test
helpers remain in scope and cannot be excluded. Keep build artifacts, mutation
copies, raw diagnostics and measurements in `lab/`; keep active L0 at its canonical
owner path and the concise current status in diagnostics.

## Closure criteria

All 105 rows must have a justified current-domain classification, passing relevant
behavior controls, and the appropriate evolution/arity proof. The pinned full
linter must report zero V16 errors with unchanged severity and scan scope, and
strict lineage and integrated checks must pass. Preserve unresolved broader
parity debts explicitly; clearing these 105 findings is not proof of complete
Typst parity. Do not use `allow`/`expect`, exception/severity changes, exclusions,
tautological rebinding, `if let`/`matches!` substitutions, moved catchalls, or noisy
errors merely to evade the detector.

No Rust test suite, compiler mutation, oracle probe, or implementation acceptance
test was run for this plan. Only fresh lint/lineage measurements, source/L0 review,
and documentation/inventory validation are evidence of work completed here.
