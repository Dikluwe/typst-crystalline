# L0 — infra/pipeline/context_stabilization
Hash do Código: 482e0e14

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/context_stabilization.rs`

## Current evidence

- `03_infra/src/pipeline/context_stabilization.rs:42` — `Session`.
- `03_infra/src/pipeline/context_stabilization.rs:651` — `prepare`.
- `03_infra/src/pipeline/context_stabilization.rs:690` — `prepare_html`.
- `03_infra/src/pipeline/context_stabilization.rs:748` — `prepare_bundle`.
- `03_infra/src/pipeline/context_stabilization.rs:897` — `bundle_page_views`.
- `03_infra/src/pipeline/context_stabilization.rs:1043` — `strip_executed_html_markers`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/context_stabilization.rs`.

## Owned contract

- Own only the private per-compilation session: causal discovery, producer contributions, attempts, sinks, retained observations, replay and terminal decisions. Use the existing evaluation/introspection/layout facades; never duplicate selector matching, counter/state folds or Value equality in L3.
- Selection requires an actually reached observation, not AST inspection, variable naming, error substrings or the mere presence of a selector. A failure before demand retains ordinary legacy treatment. Nonselected producers retain their ordinary contribution/error during additional attempts.
- Preserve producer identity, capture, Location, input style chain, target, features and Engine resources per attempt. Replace a producer/closure/chain by invalidating its descendants; numeric context id or Debug/hash of a closure cannot certify reuse. Replay uses captured input styles, not the chain left by execution.
- For selected filtered-counter generations, discard discovery execution/sink and start with empty observational views I0 while retaining origin Locations. Discovery is not an extra stabilization attempt; selection stays latched even if a later branch makes no read.
- Attempt A_k reads I_(k-1), builds current content/document and a complete I_k (runtime introspection, positions and PageStore), then validates retained actual reads against I_k. Reexecute only invalidated selected contributions and descendants; discover real newly produced blocks recursively. Reconstruct from original content/current contributions without accumulating updates or markers. Failed bodies keep only their marker, not stale successful output.
- Reuse only complete valid records with unchanged causal inputs. Equal final values, equal page counts or a lone selection flag do not prove stability. All retained contributions must describe the same candidate generation.
- Share the existing five-attempt budget with paginated invalidations. On validated convergence return the candidate or its pending original error. For proven linguistic nonconvergence without final error, return D5 calculated from I4 and diagnostic history I0 through I5; do not execute A6, substitute replay from I5 or return D4.
- Publish only warnings of retained final executions, once and in causal order; discard validation/replaced/discovery sinks. Preserve original error spans/traces and precedence. Do not reorder by HashMap or infer error classes from text.
- Inability to prove required stability is not linguistic nonconvergence. At the terminal budget, return Error `contextual stability could not be verified`, anchored at `source.root().span()`, without fabricated trace/hints or provisional export. Proven ambiguous identity/topology may terminate at discovery. Original pending errors take precedence; prior global warnings remain.
- HTML preparation preserves causal Html target/features and strips executed context markers only in the terminal serialization view. Residual contexts, unobservably paged dependencies and closure errors fail closed, never become Empty or successful output.
- Standalone HTML position stabilization is selected only by observed LocationPosition, not filtered-counter selection. Recursively discover provisional descendants; discard selected discovery contributions, retain unselected ancestors. Seed with the original HTML introspector and empty HtmlPosition store, not an empty global introspector that would erase query/state.
- Latch the HTML phase after the first observed position read. Every later generation must build and compare its full store, even when producers disappear; nonempty-to-empty requires a further empty-to-empty confirmation. Reintrospect marker-plus-result content, measure only intrinsic islands and validate both full store and observations under the same five-attempt budget.
- Bundle preparation reconstructs/reintrospects all ordered bodies against one global candidate with causal seeds, target/style/features/resources per body. Keep Page boundaries per body in pure and runtime snapshots; local Page reduction only initializes physical pagination and is not a query/validation snapshot.
- Paged bodies contribute positions and PageStore; HTML bodies contribute separate intrinsic HtmlPosition stores without page offsets. Validate the complete set before terminal HTML projection or any export; never promote a body settled under an earlier generation.
- Without selected bundle producers, realize the final paged view once before terminal HTML projection to supply PageStore. With selection, reuse the validated store without a second realization. HTML consumes no physical pages.
- Keep CounterUpdate nodes in candidate snapshots; remove executed HTML updates only in terminal projection. Preserve SpaceFromNewline origin until causal L1 normalization; only an all-whitespace terminal Sequence of Space/SpaceFromNewline collapses to Empty.
- Layout/callback/numbering and intrinsic-position helpers keep their own owners. Do not add an observation ledger, boundary hook or instrumented default.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
