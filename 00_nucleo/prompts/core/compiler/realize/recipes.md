# L0 — compiler/realize/recipes
Hash do Código: a0cd5c4a

Layer L1. Sole productive consumer:
`01_core/src/compiler/realize/recipes.rs`. Owner-ratified contract; seal the
consumer during materialization.


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42


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


## Approved Figure/Grid kind repair (2026-10-19)

### Measurement and classification before decision

The frozen proposal at baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, `agents/contracts/semantic-partition.json`, and `agents/counter/design.md` bind measurements and full source/oracle provenance. Current-pin oracle is `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Constructor inference loses typed identity and lexical occurrence styles; semantic inspection and final supplement need separate phases. These are language semantics and causal effects, not a requirement for upstream Rust mechanics. The human explicitly approved the exact public carriers, phase changes and compatibility migration in the current session under ADR-0127.

### Exclusive responsibility

Reached Figure roots obtain a pure kind preview after installing lexical styles and before has_root_effect, root filtering and show-set eligibility. Match against one preview throughout; then fold all matching show-set deltas and prepare a final distinct occurrence before replacement. The no-rule path performs the same final preparation before emitted children, locating the prepared root. Parent-hidden Figures never prepare. Original provenance stores final prepared Located Figure; a returned prepared alias retains its snapshot. Errors preserve nearest Source span.

### Acceptance and boundaries

Require functional RED→GREEN, frozen kind/set/show-set/supplement/hidden-parent cases, alias/homonym and field snapshots, full build/tests, paths, strict forward/reverse lineage, V15/V26 and differential lint. No suppressions, opaque catchall, fake neutral child or weakened assertion. Caption omission/none, complete caption synthesis, supplement-dependent preview selection, complete function-number formatting, HTML implicit captions/references, Dynamic field enumeration, and mixed Table header/ordinary-cell supplemental target order remain explicit bounded debts.

## Actual Figure caption children

The feature owner's nested-supplement fixed-oracle receipts and actual
integration-tests-5 RED distinguish semantic retention from emitted Content.
At a Figure root, delegate child realization to figure's owning helper and return
Some to prevent generic entity mapping from visiting a nonemitted retained
supplement. Keep Located, Source and style identity/restoration. The owner visits
body, the selected supplement only for the current actual numbered caption
prefix, then caption. This does not change semantic maps or introduce a Ref phase.

## Ratified reference supplement admission

The frozen Ref phase proposal SHA1578cbea and human approval receipt authorize
post-root emitted Ref admission. Parent/own show replacement decides before
capturing a generated contextual producer; private already-prepared and
passthrough transports prevent re-admission. Delegate producer construction and
prepared selected-child realization to reference_supplements, preserve guards,
lexical capture and original Ref fields. A no-rule prepared Figure now retains
Realized.original as its prepared snapshot before actual child realization and
visual as same Located Figure afterward, so later reference uses can clone the
raw selected resolved template. Do not descend original children, rerun outer
supplement callbacks or introduce a cache/new public carrier.

Source entry scopes a private eval.reference.origin empty Args span carrier while
realizing its body, restoring Engine styles on every result; reference producer
binding reads the nearest such actual source span. This is internal metadata,
never argument execution or user field. Reached raw ContextBlock visits consult
context_occurrences' scoped template flag so ID instantiation survives lexical
ShowRecipe style restoration, while Located aliases and classified generated
producers preserve identities. Prepared/passthrough Ref token-only Styled roots
are recognized before inherited-style descent; skip their single-root private
token and do not re-admit returned Refs. Actual selected child realization occurs
once under the feature owner, not by generic terminal Ref entity mapping.

Nested generic traversal consumes traversal owner's new internal nearest-origin
preorder callback, retaining the old callback opacity/once contract. Scope the
nearest proven Span around the whole visited/root-child operation, restore styles
on every result, and never call the callback on Source itself. This is necessary
for captured Ref Args spans inside generic containers, not a new public map or
runtime phase. Source's first-origin and ordinary content mapping remain intact.


## Captionless default product Location boundary

Measurement before decision: realize_children:74–81 currently rewraps every Located root. For the narrow Paged captionless default Block product this would bind an actual Block or its replacement at the original Figure Location; walk.rs:415–467 intentionally skips an already indexed non-Figure/Table alias. Layout::realized::layout at realized.rs:558–576 already registers the immutable prepared original before the actual visual. Caller audit: realize_root:494–502 retains original Located Figure in Realized; its guards-empty path cannot bypass that special empty-rules Figure branch; Table entrypoints descend Figure children through ordinary visit/realize_root; sequences::capture_scope:94–109 admits only Empty/Sequence, not root Located Figure; completed products preserve their originals.

For precisely an existing Located captionless Figure under Paged realization, return its actual default Block realization without rewrapping that product at Figure Location. Keep the surrounding canonical Realized.original unchanged. Full-caption Figure numeric Location transport and all other Located cases retain existing behavior. A replacement product's own Located objects remain exactly as returned; do not force Figure identity onto Strong/Block/other output or broaden canonical alias traversal. No new public API/phase/registry rule. Verify same original label/query Location and page capture plus a normally reached raw Context/nested Figure body, independently of the None-pruning control.

## Deferred physical page-template capture

### Measured boundary and ratification

At the implementation baseline, `recipes.rs:25–38` delegates ordinary child
visits to the generic preorder mapper; `:73–110` specially coordinates reached
roots, while SetPage/PageRun remain generic complements. The entity mapper is
SetPage-opaque and traverses PageRun static fields. Consequently this consumer
must own the operational interception rather than changing the global catalogue.

The frozen bilateral `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json`
compares source-bound executables against vanilla commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`: raw SetPage headers/footers
render but their Figure/Context contributions are absent from productive
semantic stores; overwritten headers remain inert. Native PageRun already
prepares raw static fields before knowing physical repetitions, so repeating
that prepared raw object would incorrectly turn independent insertions into
aliases. These are language effects, morphology, order and Location/page
observations, not a requirement to copy upstream mechanics.

The separate `lab/results/set-page-header-2026-10-21/ambient-baseline/receipt.json`
SHA-256 6291efca219a3558ad2192f99f1208d64ad3e70b38ec96db2d417527549fcb05
refutes freezing creation-time 10pt as insertion styles: the measured header
uses page-effective 20pt, then inherited page-effective 30pt on the next page.
Explicit inner header18pt/footer12pt remain binding preservation controls.

The approved review is `lab/results/set-page-header-2026-10-21/review.md`,
SHA-256 cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039.
The direct human decision is preserved in
`lab/results/set-page-header-implementation-2026-10-21/approval.json`.
It authorizes only the frozen five public facades, opaque carriers and physical
page phase; it does not authorize additional public contracts or generic
observer/traversal changes. The implementation baseline is HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`. Exact owning source/L0 bytes,
SHA-256, UTC and working-tree state before these amendments are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/before.json`.

### Capture and insertion environment

After the parent's/root's show decision permits a reached SetPage, capture raw
Content in background/header/footer/foreground using the existing immutable
ShowRecipe (`textual=false`). Capture full lexical rules/guards/file/context-ID
floor and nearest proven Source; run no descendant, kind preview, Figure
supplement, closure, counter/state update or Context producer. Omitted, Auto,
None and explicit Empty retain their distinct fields. Preserve an existing
captured recipe or prepared Located alias rather than recapturing its environment.

Apply the same boundary to PageRun: capture its four operational templates and
realize only its ordinary lexical body; retain lexical restoration/configuration
semantics. Generic maps and ordinary nonoperational recipe behavior are unchanged.
No inspection clone with cleared semantic fields may be published as a snapshot.
Any feature-owned interception must stop automatic descent after rebuilding this
boundary; replacements still prune hidden fields before capture.

Only an actual typed physical request permits later template insertion. Retain
lexical capture and actual page-effective environment as separate private
provenance. At that boundary, rebase only the operational ambient capture root
onto the actual page environment; preserve explicit inner Styled deltas, genuine
captured local recipe deltas and prepared aliases. Do not install creation10pt
as insertion style, substitute a last-body style heuristic, collapse raw chains
or rewrite unrelated ShowRecipe semantics. Existing pending recipe entry/restoration
remains causal; all borrowed Engine/context state restores on every Result path.

Acceptance requires capture without effects, overridden/hidden inert templates,
page-effective20/30 and explicit inner18/12, captured local/global rules, sources,
prepared alias reuse and fresh raw repetitions. Existing Figure/Ref, contextual,
Table/Cell, textual, termination and HTML contracts remain independently binding.

## Retained actual Context producer witness

Measured recipes.rs:240–248 creates a fresh ContextBlockElem Arc at each raw
running-template replay although the context-occurrence owner has already
certified its operator cohort and creation slot. The integrated descendant-query
RED and exact pre-correction sources are preserved under the physical-page
implementation's runtime-observation-1 and agents/realization/stable-context-before.
For the existing instantiating-template branch, delegate construction to the
context-occurrence owner's private helper: it preserves an immutable producer Arc
only within a certified running cohort and only for the same retained ID and
actual owned Func representation. Outside that boundary construction stays fresh.
Do not execute the closure, compare by function address/name/body/ID, weaken
observations or equate fresh style carriers. Generated reference producers retain
the existing exclusion. This changes no public API, recipe order or phase.

## Prepared operational Context transport re-entry

Source-boundary-9 certifies actual request chains20/30, while round8 integrated
Context execution observes11 because the custom-only capture delta loses its
ambient chain at L3 collection. The page-running owner defines an authenticated
passive ShowRecipe over an already prepared Context carrier. Before entering
realize_pending, recognize only that owner's exact body-local prepared transport
and return the node unchanged. Its marker must not be recognized by inherited
StyleChain lookup: fresh captured Table recipes can copy the ambient chain without
a new head delta. The original operational template marker is not this prepared
transport and must still execute normally. Preserve all ordinary pending-rule,
Table/Cell, Source and Context instantiation behavior. No generic observer/map,
public API or phase changes. Require unchanged integrated style RED controls,
prepared Context identity on revisit and actual fresh pending products executing
under inherited metadata.


## Guarded paragraph packets retain inline authority (2026-10-22)

### Measurement before decision

`lab/results/remaining-parity-2026-10-22/agents/repeat/inline-paragraph-correction-before.json`
retains the exact uncommitted source/L0, HEAD, diffstat and UTC before this
correction. Candidate-8's successful build receipt and archived source identify
product SHA-256 `339e5a90739470d82799ac70a4841854d24d9950d93f1b1648d4aef9f84d3d42`.
Its unchanged `composition_show_par_preserves_inline_style_without_extra_paragraph`
test fails in `candidate-8/validation/tests-domain-infra.log:7901-7914`: two
emitted Par nodes instead of one. The bilateral receipt
`agents/repeat/inline-paragraph-before/bilateral/measurement.json.gz` under the
same result root binds that executable and the ratified oracle to the exact
source `#show par: set text(fill: red)` followed by
`alpha *beta gamma* delta`. Product emits three actual PDF text baselines;
oracle emits one, with the same four words and successful diagnostics-free
compilation. This is an extra language block boundary, not output-byte parity.

`eval/markup.rs:26-68` evaluates Strong's markup body recursively;
`eval/markup_stream.rs:333-350` intercepts paragraphs at that nested markup
frontier as well as the assembled outer flow. The captured packet therefore
need not be an independent paragraph root. `recipes.rs:501-560` keeps applied
outer RuleIds active during emitted-child realization. `realize_pending` installs
both inherited and captured guards before its all-Par preparation branch.
The newly scoped singleton entry currently wraps even a packet whose entire Par
recipe set is already guarded, creating the inner block without an eligible
recipe. The shared RuleId revocation contract supplies the causal distinction;
style fields, names, numeric values or text equality do not.

### Bounded correction and acceptance

For a nonempty exclusively Par-rule packet, establish a scoped singleton flow
only when at least one of its captured Par RuleIds remains unguarded in the
installed causal environment. If every captured Par recipe is guarded, retain
the preexisting `realize_node` preparation, which leaves singleton Text inline
while preserving existing authored Sequence, Par, container and Source handling.
Then coordinate the result through the same realization path and restore all
lexical state as before. Explicit Par remains an authored boundary; no rule is
removed, guard weakened, callback replayed or product swallowed. Genuine root
packets with eligible recipes retain the corrected scoped singleton entry.

The existing realization owner remains sole owner of both transformations; the
eval rules hub may reexport its existing crate-private `realize_node` beside
`realize_paragraphs` solely for this delegation. Add no public signature, field,
flag, phase, global state or second paragraph algorithm. Preserve the unchanged
inline regression, the four singleton/frontier controls, original paragraph
identity/function/product-effect tests and local recipe/guard controls. Recheck
actual bilateral text/baselines, complete suites, strict Clippy/rustdoc, raw
parity, hashes and lineage without suppressions or changed tolerances.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

Measured symbol-render-v2-before/receipt.json and pinned typst-realize/src/lib.rs:242-260,317-345 show the kind-dependent boundary precedes native show rules: outside math convert direct/source-wrapped Symbol to Text before root rules and textual grouping; inside math retain Symbol and apply textual regex per element before native element rules. Preserve source spans, wrappers, guards and existing realization order without phase migration. Adjacent textual runs may span the converted symbol.

Actual math depth excludes textual grouping of adjacent elements (including Symbol), preserving the measured per-element regex rule. Outside math, normalize selected Symbol to Text before existing grouping so compatible adjacent text/symbol/text captures can match one regex.
