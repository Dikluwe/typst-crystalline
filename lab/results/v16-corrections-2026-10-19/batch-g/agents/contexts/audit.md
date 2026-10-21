# Batch G — contextual infrastructure audit

This is an audit and proposed bounded implementation/test plan, not L0 and not
authorization to write code. `audit-state.json` binds UTC, clean baseline
`4382891f0d5f0839669597cb5c90a98f24cfb8bd`, every read source/owner/ADR hash and the
current enum catalogues. No productive source, L0 or test was changed; no Cargo
or linter execution was performed here. The current declarations are 120 Content
variants and 39 Value variants, measured from the bound source rather than an
old step inventory.

## Current owners

| Productive consumer | Canonical L0 owner | Declared source prompt hash | Declared L0 code hash |
|---|---|---|---|
| `03_infra/src/pipeline/context_stabilization.rs` | `00_nucleo/prompts/infra/pipeline/context_stabilization.md` | `d6ab9409` | `29a19e1e` |
| `03_infra/src/pipeline/paged.rs` | `00_nucleo/prompts/infra/pipeline/paged.md` | `3d3dd73a` | `16ce31b0` |
| `03_infra/src/pipeline.rs` | `00_nucleo/prompts/infra/pipeline.md` | `4dfbf129` | `dde3a132` |

Raw SHA-256 values are retained in JSON. These are recorded declarations, not
a claim that an unexecuted new strict-lineage gate passed. The three full current
owners and ADR-0107/0108/0109/0127/0129/0130/0131 were read before this plan.

## Measurement and causal classification

| Original ID | Current decision | Existing complement and bounded correction |
|---|---|---|
| V16-093 | `context_stabilization.rs:263–269`, generic ordering callback | Bare ContextBlock and immediate Located(ContextBlock) are producer callbacks. Name the other 118 outer classes and the other 119 Located-body classes as local nonproducer results. Keep surrounding map traversal, actual-member filtering, encounter order and duplicate rejection exactly. |
| V16-094 | `context_stabilization.rs:390–396`, legacy fallback callback | The same recognition boundary needs both closed complements. Preserve its distinct first-ID selection policy and existing `impossible` state; do not replace it with the ordered path or add another arena. |
| V16-096 | `paged.rs:15–20`, `text.font` Value proof | Str and Array-of-Str are the only accepted font-family proof forms. Explicitly name the other 37 Value variants as proof rejection. Empty Array retains its existing vacuous all-Str result; mixed Array stays false. Unknown string keys remain the existing open-key fallback, outside this enum repair. |
| V16-097 | `paged.rs:55–123`, initial-layout proof | Preserve the current 24 accepted/recursive Content classes and explicitly name the other 96 as unproved. False preserves the complete page/context cycle; it neither rejects the language value nor declares its render empty. |
| V16-098 | `paged.rs:214–219`, immutable outline-root continuity | Retain exactly four same-kind Arc pointer comparisons: Sequence, Styled, Located, Realized. Exhaustively partition the first Content axis; for each qualified first class explicitly classify the second axis into its same-kind comparison and 119 unlike classes. The 116 unqualified first classes reject this proof regardless of the second input. Keep the general prefix-observation/rerun path. |
| V16-099 | `pipeline.rs:509–536`, generic collector node callback | Located, Realized, ShowRecipe, ContextBlock and Styled retain their current reentry/insertion behavior. Explicitly name the other 115 node classes as no action at this callback. Keep the enclosing exhaustive L1 map fallback; these are not terminal-subtree declarations. |

The direct closed matches are the smallest ownership change. No new helper,
module, enum, public facade, dynamic dispatch or independent classification
service is needed. In particular, do not merely move a wildcard into a helper,
replace a match with an `if let` escape, manufacture an empty leaf, or broaden
an existing optimization proof to make the count decrease.

### Traversal and producer identity

`01_core/src/entities/content/traversal.rs:411–438,667–673` establishes the
relevant mechanism: Source is transparent; Located traverses its body with the
body-root callback suppressed; Realized maps only visual; descendant callbacks
are retained. Therefore the immediate Located(ContextBlock) case is required,
and removing it as an alleged duplicate would lose a producer. A bare producer
and its Located transport each yield one event, while two actual occurrences
of the same numeric ID still cause ordered uniqueness failure.

The collector map and the session have different duplicate responsibilities.
`pipeline.rs:429–430` overwrites the map entry for a repeated ID; the existing
`collect_context_blocks_reused_id_in_table_keeps_later_visual_occurrence` test
also requires the later chain. `Session::new:381–407` retains the first encounter
order of each ID, but fetches its Arc/chain from that completed map. Thus the
legacy result is **first-ID order plus last-collected payload/chain**, not
unconditionally the first occurrence/chain. The old case-ledger prose is
imprecise on that point. Preserve the measured code behavior in this batch;
do not silently turn this audit correction into a semantic change.

No Content equality, closure hash, Debug fingerprint or pointer equality is
introduced for producer deduplication. Existing numeric-ID uniqueness checks,
exact callback recognition, causal occurrence frames, descendant invalidation,
five-attempt budget and retained diagnostic transactions stay unchanged.

### Style and actual-view boundaries

Styled pushes its delta on the current chain. ShowRecipe switches to its captured
snapshot. The bottom-up generic collector's later carrier reentry intentionally
overwrites provisional descendant entries with the correct chain. Source adds
no style, semantic occurrence or Location. Do not substitute a common-wrapper
table for the current L1 generic traversal, and do not confuse local no-action
with an assertion that a nested container has no children.

The approved prepared-Ref child accessor is consulted before the marker-only
Styled wrapper. Generic Ref entity maps remain terminal; L3 must not inspect
the private token, infer kind, or match a selector. Generated reference Context
producers retain the same private recognition, execution Location, ancestry
Route and query exclusion. Actual Figure body/selected emitted supplement/
caption order and dormant-template pruning remain unchanged. Realized original
is deliberately opaque to producer collection, while the layout-independence
proof deliberately checks **both** original and visual; do not unify those two
different contracts.

The current source-origin and known-style/no-Location math/counter corrections
remain protected. This batch must not set `in_context`, populate Location,
install an introspector, execute callbacks in a classifier, relax modern
counter.final, or alter its exact two hints. Source wrapper orientations must
be characterized from actual reached carriers; no generic new unwrapping policy
is authorized by nominal complement closure.

### Conservative proofs, not language identity

`paged.rs:531–535,578–593` confirms that initial reuse requires both contextual
and realized positive proofs plus absent numbering on all pages. A false proof
keeps the ordinary snapshot/expansion/layout cycle; it is not a compilation
failure. Keep PageStore, warning publication, Source transport and the accepted
candidate's metrics lifetime intact. The font predicate performs no catalog,
font resolution, shaping, evaluation or I/O.

`paged.rs:214–237` uses pointer equality only as a sufficient immutable-root
continuity proof for the four already qualified Arc classes. A rebuilt equal
root, an unlike pair, Source, a primitive or any other class still proceeds via
the existing prefix comparison. Do not introduce structural/hash equality or
extend this to more Arc variants. This mechanical proof is not a Typst language
equality rule, under ADR-0107.

## Proposed tests and structural RED

The six current complements are semantically justified by the existing source;
this audit identifies no reason to fabricate a failing old-language expectation.
Characterization controls should preserve behavior, while a real enum-growth
compile witness supplies the RED for the missing future-class decision.

- For ordering: bare and immediate Located producers; distinct IDs in content
  order; repeated actual IDs rejected; Source transport; original-only hidden
  producer; a generic Grid/math wrapper; captured ShowRecipe and nested Styled
  chains; actual Figure caption order and dormant supplement exclusion.
- For legacy fallback: force a duplicate-ID origin, then inspect first-ID order
  and the final collector-map Arc/chain. Confirm only one ordinary execution per
  ID until the existing selection protocol applies. Do not weaken impossible
  topology or convert a first-ID order test into equality/hash deduplication.
- For collection: direct and generic nested containers, immediate Located root,
  Source, visual-only Realized, nested Styled and captured ShowRecipe; raw versus
  resolved Table and standalone/resolved/raw TableCell; repeated-ID last-map
  overwrite. Preserve the approved prepared-Ref child and marker semantics.
- For font proof: Str, all-Str Array, empty Array, mixed Array and representative
  nonfamily Value categories, including Func and Content decoys that must not
  execute. Keep captured recipe and unrecognized-key boundaries unchanged.
- For layout proof: the existing positive physical/table/local-style controls
  and negative Heading/Context/Ref/ShowRecipe/PageRun/Dynamic/marginal controls;
  Source transparency, both Realized branches, Table/Grid slots and SetPage
  restrictions. Unproved but valid content must still compile through the full
  cycle with unchanged text, pages, geometry and diagnostics.
- For outline continuity: all four shared roots, rebuilt roots and unlike pairs;
  nonqualified roots use the general path. Retain actual prefix-change rerun,
  warnings and error behavior. Pointer checks here characterize the existing
  optimization contract only, alongside language-level pipeline acceptance.

Keep new controls inline in the already owned three files, or reuse existing
root-owned test execution without editing another test owner. The root owns
Cargo, shared-enum disposable changes, source freezing and seals. Add no test
or source now: the current assignment is audit-only.

The compiler-negative evidence must be attributable to each repaired decision,
including both nested producer recognitions, the Value font decision and all
tuple axes of the continuity proof. Measure baseline wildcard acceptance and
candidate E0004 separately; new/old, old/new and new/new qualified tuple inputs
must not be inferred from an unrelated error. If dependency compilation blocks
the selected L3 decision, report that obstruction and use only the root's
reviewed real-workspace witness protocol; a toy enum or unrelated E0004 does not
prove this site. Runtime execution of one grouped arm cannot certify every
alternative; preserve the full V19/V20 differential inventory and its coverage
caveat.

Existing forty required contextual Ref cases remain binding regression inputs,
as do the accepted Figure/counter, known-style/no-Location, warning ownership,
HTML/bundle/Source and modern-bound/static final controls. Prior batch-F receipts
are historical evidence, not a newly executed clean batch-G baseline run.

## ADR-0127 class and next boundary

All six bounded changes are **continuous internal, behavior-preserving nominal
exhaustiveness repairs**. They add no public field/signature, mode/default,
pipeline phase, compatibility break, Nucleus or new ownership. Update the three
current L0 owners first with these exact measured classes and protections, then
obtain the root's structural RED release, implement only the closed complements,
and run the established tests/lineage/full diagnostic gates. No separate human
architecture decision is needed for that unchanged scope.

A measured new language omission, a proposed producer-wrapper admission, a
proof-whitelist expansion or new API/phase is not part of this classification:
stop and report it rather than hiding it inside a nominal repair. No behavior,
performance, all-alternative coverage or global-clean-lint improvement is claimed
by this audit.
