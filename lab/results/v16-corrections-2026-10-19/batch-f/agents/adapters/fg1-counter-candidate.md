# FG1 counter materialization candidate and blocking boundary

Status: diagnostic candidate, **not active L0 and not approval-ready**. This
makes the missing work concrete; it does not authorize productive changes.
The companion manifest pins current sources and existing owners. The broader
Figure contract remains in `../../fg1-proposal/design-draft.md`; its old one-pass
preparation wording is superseded by `../../fg1-proposal/readiness.md`.

## Concrete occurrence-number amendment

Retain the proposed public signature
`figure_number_at_index(&self, kind: &FigureKind, idx: usize) -> Option<usize>`.
Define idx as the zero-based ordinal among counted, emitted Figure occurrences
of that kind in causal order, never an ordinal among counter updates. Population
records that ordered occurrence list once from actual Figure Start tags. Manual
Set/Step/Func do not add entries. Caption-less or numbering-disabled Figures do
not consume a numbered-caption ordinal. Layout advances its typed progress only
when it actually requests a numbered caption; checkpoints preserve the same map.
The reader finds the nth occurrence's Location, then reads the complete demanded
history's last state at or before its recorded causal ordinal. Missing synthetic
history returns None; it must not silently select a different event. Existing
synthetic-layout fallback remains separately specified, without a parity claim.

This resolves the concrete index confusion in queries.rs:341–348 and
layout/figure.rs:36–49. Current layout advances even for absent captions, although
population gates automatic Figure counting. That mismatch must be included in
RED acceptance: uncaptioned Figure, counted Figure, manual update, counted Figure.
All four steps must retain their semantic occurrence identities.

## Concrete demand boundary

A per-kind history is demanded by an emitted numbered caption, an emitted numeric
reference requiring that Figure counter, or an explicit language counter read.
No-caption, numbering:none and show-hidden Figure alone do not demand it. The
pinned oracle's panic controls in `../../fg1-proposal/oracle-boundaries.json`
support this distinction. They do not establish arbitrary callback effect reuse.
Use the existing complete ordered action fold: automatic action locations selected
by typed figure.where(kind:...), manual events selected by semantic key equality,
then causal order. Fold the entire demanded log, not only the requested prefix;
retain Context::none for update callbacks, source diagnostics and target/features.
Do not insert an auxiliary automatic event for a typed kind key.

Stage a complete successful history before publishing it. Read-only Introspector
methods, layout and reference rendering cannot execute callbacks. Caption,
reference and get/at/final must consume the same successful history for the same
semantic generation. Any design that eagerly folds every realized kind, uses a
user-string figure namespace, or counts history entries as Figures is rejected.

## Why the current concrete cache suggestion is not yet sufficient

`filtered_history` currently takes `&EvalContext`, calls resolve_filtered_counter
on every read, then records CounterFold. The post-walk finalizer has a separate
mutable TagIntrospector. Adding a cache only there does not share it with earlier
get/at/final reads and can execute the same callback again. TagIntrospector's
public mutable stores and the public EvalContext.introspector field also mean a
cache keyed only by object identity, a hash, or kind is unsound. Current snapshot
replacements occur in realize/mod.rs, introspect/fixpoint.rs, eval/mod.rs,
eval/context_reads.rs and rules/outline.rs. Same kind in a new snapshot is not the
same event sequence or query result.

A compiler-owned cache in EvalContext is structurally possible: the existing
context_reads RefCell demonstrates local interior mutability, without globals or
I/O. That is not by itself a correctness argument. Before drafting its active
contract, the package must choose and test all of the following as one design:

1. An explicit immutable input identity covering selected automatic locations,
   ordered manual actions and callback identity/captures, target/features and
   causal ordering. Raw hashes and Rust Func equality are insufficient proofs.
2. A lifecycle shared by ordinary filtered reads, contextual read replay and
   final caption/reference demand. Snapshot replacement and speculative rollback
   must neither reuse stale results nor discard a still-observed contribution.
3. Callback diagnostic and sink contribution behavior: successful reuse cannot
   suppress a required contribution, and discarded-generation effects cannot
   leak into accepted output. Existing errors keep their causal source/trace.
4. Reentrancy handling without holding a RefCell borrow across apply_func, and
   preservation of existing recursion/budget and Context::none failures.

The available oracle controls prove demand versus no demand, but do not measure
this generation/reuse/effect lifecycle. No product or oracle execution for these
new lifecycle cases was authorized/performed by this reviewer. Claiming
callback-once now would therefore be a design assertion, not measured acceptance.
This is the precise blocker to a truthful approval package, rather than a request
for generic permission to investigate architecture.

## Candidate owning L0 amendments after the lifecycle decision

- entities/introspector: typed public index signature and exact ordinal meaning;
  storage remains immutable to readers; full declaration of any new public field.
- entities/counter_registry: if chosen for frozen results, own private data and
  invalidation/publication operations, never compiler imports or callback execution.
- compiler/introspect/populate: record actual counted Figure occurrences and typed
  label keys; retain one automatic action per occurrence.
- compiler/introspect/queries: occurrence-to-Location prefix lookup and frozen reads.
- compiler/introspect/from_tags/counter_updates: reuse complete semantic fold;
  callback demand and atomic publication are explicit phase/effect amendments.
- compiler/introspect/{entrypoints,fixpoint,from_tags}: all runtime construction
  paths invoke the same finalizer after adequate occurrence/query information.
- compiler/stdlib/counter/filtered_reads and compiler/eval/context_reads: explicit
  reads/replay use the same generation and still record observations/errors.
- compiler/eval/mod and compiler/realize/mod: if cache is EvalContext-owned,
  declare initialization, replacement and candidate/replay lifetimes explicitly.
- compiler/layout/{figure,mod,checkpoint,references} and introspect/labelled:
  consume typed prepared identity and frozen history without callbacks; preserve
  explicit supplement and numeric-reference precedence.
- infra/measurements and infra/integration_tests: forward the changed typed method,
  retain instrumentation slot and migrate test intent rather than mechanically
  converting every string table/image into an element kind.

These are candidate amendments to existing 1:1 owners, not a new cross-owner L0
that authorizes multiple productive consumers. Realization preview/final phase,
semantic-field traversal and custom-name supplement transport remain the other
precisely measured blockers in the encoding review.
