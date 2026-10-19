# L0 — compiler/realize/recipes
Hash do Código: 9ffa46a0

Layer L1. Sole productive consumer:
`01_core/src/compiler/realize/recipes.rs`. Owner-ratified contract; seal the
consumer during materialization.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3

## Causal realization evidence and ratification

`lab/results/construction/diagnostics/textual-realization-2026-10-18/{before,scope,refuters}.json.gz`
retains HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working
tree, UTC/status/diffstat/sources/argv/binary hashes and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. before-sources.json.gz contains
complete current sources/L0. No candidate code was implemented at measurement.
matching.rs:118–150 applies reverse regex then literals in declaration order;
:194–207 exposes products to later passes. textual.rs:102–123 captures full
snapshot. realize/mod.rs:177–217 visits recipes bottom-up. apply.rs:620–648/
matching.rs:218–225 call functions without capturing children.
expression_dispatch.rs:125–129 already distinguishes insertion_rules from local
state.

Measured pairs: identical literals C/B; literal/regex tie R/L; overlap X/YB;
first effect LATER/FIRST; inverse chained function BBBB/BB; separate product
AB/X; product+neighbor X/AB; Strong hiding child gives panic HIDDEN/Q;
captured global followed by new global L/R, captured local L/L; local CJK
水\n果 gives 水果/X. Pinned foundations/selector.rs:110–125 unifies escaped
literal/regex; typst-realize/src/lib.rs:1328–1405 chooses start and carries
revocation into product. These are language/effects, not ratification of Rust
mechanics/buffer equality/PDF bytes. Scope separation and earlier child visits
are inferred causes, refutable by acceptance.json, equality/fields/style/query
and error controls.

Human confirmation of manifest
`47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`
ratified the pinned textual-recipe-frontier nucleus and this order. Within this
scope it supersedes provisional ContentBlock show execution and pending-recipe
bottom-up visits. Ordinary expression evaluation order stays. Do not reevaluate
the program, advance State/Counter, bake render into morphology or create a
public Rust contract.

## Exclusive recipe coordination

Coordinate only causal execution of pending recipes/products with existing
Engine/EvalContext, static nominal dispatch. Install causal lexical frame,
compose current rules with local deltas by identity, realize root through its
Sequence/apply/textual owners, then descend only into emitted children. Never
visit Realized.original as emitted content. Preserve original provenance for
query and product's causal namespace.

General Content maps retain their contracts. Use existing preorder reconstruction,
returning Some after root processing to prevent a second automatic descent. If
an internal map API is strictly needed, specify its owner L0 first; this approval
changes no public map signature. Duplicate no selection/space normalization/
Table-Cell preparation/math/layout construction/termination.

Keep one boundary per product, shared snapshots and existing depth budget. A
single emitted unit may group its own textual nodes; products of distinct matches
stay separate. Restore environment on every return and preserve first effect,
spans and causal traces. Final normalize_insertions must retain information
needed by later contextual realization. Publish no API/field/trait/language
element/flag.

## Integration evidence and current dispatch refinements

A protected Located Table delegates to existing Table preparation, skipping only
its applied recipe; neither skip Cell preparation nor treat its tree generically.
Keep real identities. All Content matches have complete nominal complements:
realize_children excludes only unguarded Located/Sequence/Styled/Equation;
visit excludes ShowRecipe/Realized/Styled/Table/TableCell/Sequence and, under the
later Context capture below, ContextBlock. Guarded Located remains in complement.
Keep visible V19 groups with one responsibility (continue descendant catalogue);
executing a group does not prove every alternative covered.

Candidate4 finished without abort: 7373 passed, 15 failed, five ignored.
Alias/local failures show Location protection cannot skip every rule. For a
protected Location exclude only active applied RuleIds; earlier/local rules with
distinct identity stay eligible. Fresh occurrences inherit no exclusion. When
revisiting completed product also protect original Location without executing
original. Coalesce compatible generic-product packets before visiting, as for
textual product, preserving completed boundaries/root effects. During full_error
causal visit retain morph_canon in path, restoring on error. Recognize Source on
entry; anchor only detached errors to innermost preserved span without changing
mapper contract. For exclusively Par-rule packets call owner realize_paragraphs
before selecting prepared roots; duplicate no logic or layout-time Par synthesis.
Explicit Empty/Sequence under Sequence rules receive a nominal decision before
text grouping; markup owner removes consumed slots.

Located with Table root must pass through realize_table_insertions before child
visits: skipping Located root cannot skip Table preparation. Candidate6 RED has
zero metadata and unprepared Table. Existing facade decides recipe and prepares
placement/Cells; coordinator visits finished product without rerunning root.
Old Table packets also enter this coordination, retaining legacy rule-composition
fallback.

Send apply_root only eligible rules actually matching root via existing
selector_matches. Nonmatching rules cannot allocate identity early for raw
contributions. Without effective recipe preserve root data and coordinate emitted
children. Whole-document canonical finalization keeps its owner.

## Acceptance

Require every T family in acceptance.json and owner revalidation. Specifically
test error before siblings, parent hiding child, alias-returning product,
composite product, once-only effects and restoration after error. Test both
targets, contexts/fixpoint, original/product queries, Table/Cell and Equation
without repeated callbacks. Revalidate alias/local/fresh/query/Table/context
controls without relaxing tests.

## Context textual-inheritance evidence

HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, productive source clean.
`lab/results/construction/diagnostics/context-paridade-2026-10-18/before.json.gz`
contains UTC/argv/diffstat/executable SHA; fresh build is in
baseline-build-resources.json, full freeze in before-sources.json.gz. Vanilla
pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Global/local probes return
A/A in Tekt versus R/L; creation before rule, nesting, regex and product also lose
rule. Local-inside-Context control gives L in both. realize/mod.rs:240–269 captures
only Equation/Table; recipes.rs:343 treats Context as generic leaf;
textual.rs:46–103 preserves baseline and composes local delta by RuleId.
Pinned foundations/context.rs:78–86 executes with contextual-point styles;
typst-realize/src/lib.rs:353–418 prepares root/applies rule/visits product.
This measures language scope/effects, not required upstream structure. Inferred
rule-loss cause is refuted by failing scope/nesting/product/parent-hiding-child
controls.

### Current Context capture boundary

Internal parity correction within ratified contextual realization preserves
deferred execution, API and phases. This scope does not repair Location collisions
with offsets/hashes, remove Located or change contracts. Ordinary Styled stays
opaque in textual matching; change no mapper.

When visiting emitted ContextBlock, delegate effective insertion capture to the
realization hub's private helper and end visit with Some. Execute no closure.
Remove ContextBlock from this match's nominal complement. Capture occurs only
after the parent's decision permits this child; preserve style/source wrappers
and Located's prepared identity.
