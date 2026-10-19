# Independent paragraph-stage reachability audit

This is a read-only source audit, not a new oracle or runtime measurement.
The paired `paragraph-stage-audit-state.json` records UTC, HEAD and changed-file
stat. Paths below are relative to `01_core/src/compiler/`, except where stated.

## Evidence before conclusion

- All productive entry points to `eval/rules/realize_paragraphs` are
  `eval/rules/apply.rs:116–120` and `realize/recipes.rs:565–574`.
  `realize_flow` and `is_paragraph_node` are called only inside their owner.
  The classifier is consulted for each realized child at
  `eval/rules/realize.rs:184–201`; a false result contributes that child to an
  enclosing fresh Par. The owner retains Located and Realized unchanged at its
  generic `realize_node` fallback. Thus a fabricated Realized(visual=Par) child
  demonstrates an internal shape distinction but does not establish language
  reachability or justify choosing either original or visual transparency.
- Initial occurrence allocation excludes direct Par: `realize/mod.rs:47–55`
  gates `locate` on extract_payload, as do `eval/rules/apply.rs:158–161`,
  `:178–182` and `:791–795`. `introspect/extract_payload.rs:18–23,112`
  returns no payload for Par and completed/pending carriers. This is an
  allocation invariant, not a type guarantee: `realize/recipes.rs:45–53`
  rebuilds Located children, and `eval/rules/outline.rs:77–89` retains an
  Outline location around its realization. The latter still returns an
  Outline with realized body (`:253`), not a direct Par.
- Main language evaluation sets defer_show_rules before evaluating the file
  (`eval/mod.rs:815–824`). `apply_show_rules_inner` captures ShowRecipe and
  returns before paragraph preparation when this flag is set
  (`eval/rules/apply.rs:79–97`). Root-only/sequence/prepared-cell callback paths
  enable deferral during the callback (`:224–233`, `:661–670`). Textual
  callbacks also enable deferral during transformation
  (`eval/rules/textual/matching.rs:168–177`). These facts refute the naive
  argument that every nested show callback eagerly produces completed children
  before its enclosing markup is prepared.
- Fresh bindings do not themselves guarantee a pending carrier:
  `eval/bindings/binding.rs:37–55` simply evaluates and stores the value.
  Pending capture is upstream: content-block textual groups are captured when
  `!normalize_spaces` (`eval/rules/textual.rs:379–418`), code-block local
  content and returns use capture_local_output
  (`eval/expression_dispatch.rs:202–207,360–371`), and markup captures local
  output while deferred (`eval/markup_stream.rs:226–229`).
- Realized roots dispatch directly to realize_completed
  (`realize/recipes.rs:199`), whose paths realize the visual and restore the
  carrier (`:588–628`), without calling realize_paragraphs on that carrier.
  Root par rules are admitted only when the actual root is Par after
  Source/Located peeling (`:372–399`). This supports the narrower statement
  that ordinary completed-root visitation bypasses the paragraph classifier.
- It does not prove a universal unreachable state. Paragraph preparation is
  also available from apply_show_rules whenever deferral is false, before
  completed-node skipping (`eval/rules/apply.rs:116–120,149–153`). Public
  expression evaluation (`eval/mod.rs:625–655`) and L3 context production
  (`03_infra/src/pipeline/context_stabilization.rs:262–299`, repository-relative)
  use EvalContext defaults (defer=false, realizing=false at
  `eval/mod.rs:430–432`). Markup can realize inserted occurrences when realizing
  and not deferred (`eval/markup_stream.rs:203–218`), then call paragraph
  interception at the end (`:317`). A concrete language path from these flags
  to a completed Par sibling has not been demonstrated by this audit.
- Realization of a pending all-Par recipe prepares its body before recursively
  realizing it (`realize/recipes.rs:565–574`); it does not prepare the output
  after completion. This ordering is central to any proposed reentry witness.

## Bounded decision and remaining debt

The evidence supports preserving the existing Source/ShowRecipe/Styled
recognition partition while replacing its unknown-variant wildcard with an
explicit Content complement. It does not support claiming universal
unreachability of Located/Realized or adding visual/original transparency as a
new semantic rule. A structural closure must explicitly retain that limitation.
The source-based refutations reported by the effects agent (whitespace
paragraphs; raw Heading grouping before completion) must replace the initial
causal hypothesis in L0, not be left as supporting evidence for it.

Continued semantic debts are concrete and separate:

- Eager flow can group raw block outputs before their realization; the reported
  repeated Heading-to-Par source case must remain open with its exact receipt.
  Changing completed-carrier recognition does not repair that demonstrated path.
- Whitespace-only flow paragraphs can invoke Par recipes; effects measurements
  must distinguish their bodies from duplicated authored paragraphs.
- Query-based Par reinsertion is blocked by existing missing-label/depth errors
  in the reported probes; those errors neither certify this classifier nor
  authorize inventing a new carrier policy.
- A future reentry repair needs a real source witness in main compile, context
  production or expression evaluation, plus observables showing which authored
  paragraph/effect duplicated. Instrumented evidence must demonstrate that the
  selected classifier sees a completed carrier at that point. An internal
  fabricated shape alone is not that witness.

No production code, L0, seals, or tests were changed by this audit. No Cargo
command or new runtime/oracle probe was run by this agent.
