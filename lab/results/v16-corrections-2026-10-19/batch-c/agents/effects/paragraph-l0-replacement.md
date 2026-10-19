## Closed prepared-paragraph classification

### Evidence before decision

`lab/results/v16-corrections-2026-10-19/batch-c/agents/effects/` retains
`flow-probes.json`, `flow-repr.json`, `flow-heading-probes.json`,
`flow-heading-alias-repr.json`, `flow-nested-probes.json`,
`flow-local-par-probes.json`, `flow-textual-probes.json`, and
`flow-context-probes.json`. Each records UTC, checkpoint, exact uncommitted
diffstat, source, command, output and immutable binary identity. The oracle is
pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`; `product-provenance.json`
compares the batch-B fresh-build frozen inputs to its committed checkpoint.

The source-level direct Heading-to-Par, styled Heading-to-Par and Heading-to-inline
controls produce one paragraph effect on both sides. Consecutive explicit Par
initially exposed four product effects versus two oracle effects; the refined
repr probe shows that the extra effects concern space-only paragraphs, refuting
the initial carrier-attribution hypothesis. Repeated Heading and textual products
expose early paragraph preparation around the untransformed input, not proof of
an incorrect Located/Realized projection at this selected match. Some query-Par
probes fail due to separate missing-label/depth behavior; invalid grouping-depth
probes are not acceptance evidence.

`is_paragraph_node` recognizes Par through Source/ShowRecipe/Styled at the early
flow stage. `apply.rs:120` prepares before node-rule application, and
`realize/recipes.rs:573` prepares all-Par captured recipes. Direct completed
products use `recipes.rs:195–202` and `:589–630`. The independent audit in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/paragraph-stage-audit.md` records a possible reentry
through markup insertion and paragraph interception; it does not establish a
global unreachable invariant. Ordinary allocation excludes Par, but generic
reconstruction can change located bodies. Constructed carrier diagnostics show
mechanical distinctions only and cannot certify language projection policy.

### Exclusive finite decision and acceptance

Make this existing prepared-paragraph predicate exhaustive over Content. Par is
recognized; Source.body, ShowRecipe.body and Styled.body recurse. Every other
current nominal family is explicitly outside this predicate's recognized set,
including Sequence roots and occurrence carriers. A stopped carrier result means
only that this early predicate did not recognize a prepared paragraph; it is not
an assertion that its body/visual is nonparagraph, that it is unreachable, or that
its existing language transparency obligation is waived. The flow coordinator
and causal realization owners retain responsibility for emitted boundaries.
Keep the narrower flow_boundary_body style view unchanged. Do not peel arbitrary
carriers, inspect both Realized sides, run snapshots/callbacks, change phases, or
promote every container to a paragraph merely to remove V16.

Preservation controls must exercise the matched actual source-level producer
cases, Source/ShowRecipe/Styled recognition, unchanged snapshots/origins, Sequence
root disposition, and whole-flow versus inline style precedence. Constructed
carrier controls may document the current stage stop but must not be represented
as language acceptance. A new Content variant must produce E0004 at this exact
match, requiring a reviewed decision instead of silently inheriting false.

The occurrence reentry question remains open: a valid source that places a
completed visual Par at this selected match and loses a language boundary/effect,
with a narrow projection correction fixing the measured oracle discrepancy,
refutes the current stage policy and requires its L0-first correction. The
space-only paragraph, raw-Heading/textual eager preparation, query-Par errors and
broader paragraph parity remain separately open; exhaustiveness does not close
those debts. This package preserves the local decision and earlier carrier
contracts while making future nominal additions reviewable. It authorizes no
public contract, product mode, phase change or suppression.
