# Figure V16-055: public contract and phase proposal

Status: awaiting the repository owner's ADR-0127 decision. This is a diagnostic
review package, not active L0 and not authorization to materialize code. Figure
V16-055 and batch F remain open. No productive source or active prompt has changed.

## Measured cause

The baseline is `d4e72102df01f9c248dffac761662d0f3cbf699e` on
`codex/tekt-mature-layout`. The paired probes in [the independent audit](agents/audit/README.md)
and [counter measurements](agents/counter/counter-probes.json) bind UTC, source,
arguments and executable provenance. The oracle remains commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`.

The existing constructor stores kind as `Option<String>`, loses valid element
function identity and infers from a limited subset before occurrence styles are
known. The wildcard is a symptom of that incomplete contract. Replacing it with
an arm list would leave the measured semantics wrong. Manual counter updates
also expose missing typed automatic steps and confusion between event index and
Figure location. Grid's split fields lose the original child order needed by
semantic inference.

## Decision requested

Approve the exact declarations and amendments in the linked designs as one
bounded correction:

1. [Typed Figure and semantic inspection](agents/contracts/design.md):
   `FigureKind` distinguishes element identity from custom name;
   `FigureSupplement` carries its lexical value and callback origin;
   `FigureElem` separates omitted/auto/explicit input from resolved occurrence
   kind and supplement. The Figure payload carries the typed resolved values.
   Grid retains its original child order through the declared indexed carrier.
   Clone, mapping, reconstruction, reflection and observation retain these facts.
2. [Occurrence preparation](agents/contracts/design.md): a pure kind preview
   selects applicable rules; final preparation uses matched show-set styles and
   resolves the supplement before Figure's own replacement. Hidden ancestors do
   not cause child preparation. Supplement callbacks and counter callbacks keep
   their separately measured contexts and demand rules.
3. [Location-based counters](agents/counter/design.md): automatic counting depends
   on numbering, independently of caption. Actual paged caption/reference demand
   folds the complete relevant history in a local transaction and validates it
   before publication. Readers use the occurrence Location, never an update-event
   ordinal. No shared result cache or cross-transaction publication flag is added.
4. [Bundle facade and compatibility](agents/counter/design.md): the additive
   public runtime facade receives per-body targets and Features. Existing facade
   remains an all-Paged wrapper. Productive typed Figure numbering stops writing
   the legacy `figure:<kind>` user-string namespace; the old string index method
   retains its signature and synthetic registry behavior, but its productive
   automatic-number projection changes. The old string constructor adapter also
   cannot stand in for typed element identity. These semantic compatibility
   changes and migration of affected callers are explicit parts of approval.

The exact source/owner scope is [the integrated manifest](owner-manifest.json).
Existing owners retain their full prior obligations except the explicitly
superseded clauses. New owners are canonical 1:1 counterparts of new feature
units. Approval does not authorize global resealing or edits to inspected-only
regression boundaries.

## Scope and acceptance

The goal is to correct the causal typed-kind, semantic traversal, occurrence and
counter defects behind V16-055. It is not certification of all Figure or Grid
features. The designs and [case classification](agents/audit/closure-case-classification.json)
separate required corrections, retained existing debts and inconclusive controls;
the original oracle observations remain intact. Standalone HTML Figure rendering,
supplement-dependent selector preview, complete caption synthesis and external
Dynamic field enumeration remain explicit limits. Kind/reflection/explicit reads
must still be checked under HTML; paged implicit formatting includes paged bundle
bodies. No broad claim that all targets have Figure rendering parity is allowed.
Grid's existing footer-validation phase and complex Table header order for an
explicit non-Table supplement target are separately identified limitations;
neither is reported as fixed by retaining Grid's source order. Auto inference
recognizes a Table at its root, before that supplemental target-search boundary.

After approval, update the individual owning L0s first and reseal only the valid
ownership graph. Add independent semantic regressions and observe RED before
implementation. The executable acceptance includes typed function/string
homonyms, lexical presence, effective style reuse, ordered semantic fields,
nonexecuted Context/callback fields, Grid source order, callback target/context,
hidden-parent versus own-replacement effects, automatic count versus demand,
interleaved updates, rendered caption/reference numbers and isolated diagnostic
contributions. Actual enum-extension builds must reject missing decision cases;
source-pattern tests alone cannot close the finding.

Closure requires the established workspace/build/tests, all-target, Clippy,
Rustdoc and formatting checks; canonical paths; strict forward/reverse lineage;
V15/V26 before resealing; and full differential lint with exactly the accepted
changes. No suppression, severity downgrade, weakened fixture or convenient
unreviewed leaf classification is authorized. Batch F closes only when these
implementation gates pass; this proposal does not close it.

## Validation performed before the decision

On the unchanged baseline, all 128 existing Figure unit tests passed, along with
the CI tooling tests, path checks, strict V1/V5/V15/V26 and reverse lineage. The
full pinned linter still reports 20 V16 errors, including Figure, with 1262 V19
notes and 877 V20 notes. No product correction has been tested as GREEN yet.
[Baseline validation](validation/baseline-state.json.gz) and
[Figure tests](validation/figure-tests-state.json.gz) preserve exact commands,
times and unchanged input hashes. Reused product/oracle probe binaries are
identified separately; input-equivalent reuse is not described as a new build.

## Why this decision needs confirmation

[ADR-0127](../../../../00_nucleo/adr/adr-0127-l0-gate-stop-vs-continuous-flow.md)
§2 requires confirmation for public entity fields/signatures, phase changes and
compatibility changes. All three apply here. The earlier Circle approval names
different fields and does not cover this Figure/Grid proposal. The requested
decision is approval of this concrete package; routine internal repairs and the
subsequent required validation remain within the already authorized work.
