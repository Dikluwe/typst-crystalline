# L0 — compiler/eval/closures
Hash do Código: 6edc6204

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/closures.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/closures.rs:40` — `fn apply_closure`.
- `01_core/src/compiler/eval/closures.rs:186` — `fn eval_closure_expr`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/crates/typst-eval/src/call.rs:693-715,728-735`.
- `typst-eval/src/call.rs:638`.

## Contract

Own closure construction and application. Capture an eager immutable lexical
snapshot. Preserve parameter patterns, positional/named/default/spread handling,
definition-time evaluation of defaults, recursive self binding and the existing
maximum call depth of 80. Use isolated local scopes and a reborrowed Engine with
the causal route, styles and sink. Consume Return and reject residual Break or
Continue with their owned forbidden diagnostic.

Missing required parameters fail as missing arguments, not None; extra arguments
are errors, with leftover positional errors preceding leftover named errors.
Named-default parameters are keyword-only and do not consume positional values.
Recursive self is bound before parameters, so a same-named parameter shadows it.
Pattern parameters consume one positional value. Consume named and
positional arguments through remove_named/remove_positional(0), keeping Args
occurrences and views coherent. Preserve all residual sink occurrences, their
names, ordering, duplicates, individual spans and aggregate span. The current
sink order is residual named occurrences first, then residual positional ones,
preserving order within each group. Nonterminal-sink limitations are not silently
changed.

Keep the causal parameter/body origin in the private Func origin metadata;
known origins survive aliases, With and factories and are never replaced merely
because a later expression yields the same value. Do not execute closures to
compare identities or recover a source by searching equal values.

Under cfg(p1339_observation), record the real function/body SyntaxNode and actual
result/flow along the existing callback ledger path. Do not manufacture a Func
for a body, re-evaluate a body for instrumentation, or expose ledger behavior in
normal builds. Preserve recursion, style and context semantics in both builds.
