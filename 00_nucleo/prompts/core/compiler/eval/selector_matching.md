# L0 — compiler/eval/selector_matching
Hash do Código: 71395534



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/selector_matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:7e7373acce2b00247d18b9c5ea1b9d53833275997cd2c304e9d919d2cce10648
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/selector_matching.rs:29` — `fn query_selector_to_show_selector`.
- `01_core/src/compiler/eval/selector_matching.rs:112` — `fn selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:203` — `fn native_element_base`.
- `01_core/src/compiler/eval/selector_matching.rs:212` — `fn is_par_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:226` — `fn native_element_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:306` — `fn native_element_field`.
- `01_core/src/compiler/eval/selector_matching.rs:324` — `fn element_selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:342` — `fn values_eq_semantic`.
- `01_core/src/compiler/eval/selector_matching.rs:354` — `fn is_node_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:370` — `fn is_text_rule`.
- `01_core/src/compiler/eval/selector_matching.rs:376` — `fn textual_selector_matches`.
- `01_core/src/compiler/eval/selector_matching.rs:389` — `fn textual_match_ranges`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/selector.rs:134-140`.
- `foundations/selector.rs:149-153`.
- `typst-realize/src/lib.rs:1261-1325`.
- `typst-realize/src/lib.rs:1320-1371`.

## Contract

Own conversion from query selectors to show selectors, typed node matching,
node-rule classification and literal/regex splicing. Operate on received
values/content without scope mutation, World reads or layout. Splicing invokes
only its supplied replacement closure for each matched slice; this helper does
not independently resolve or apply a user Func.

Literal/regex splicing passes only each nonempty matched text slice to the recipe,
keeps unmatched prefix/suffix, leftmost order and causal content morphology.
This helper does not concatenate descendants or form realized text groups;
00_nucleo/prompts/core/compiler/eval/rules.md owns group construction/normalization.
Text/Regex are not node rules, Label has its separate path, and empty And/Or do
not match. NodeKind/DynKind/Where/native Element and existing compositions retain
the closed dispatch and their current semantics.

NativeElement matching recognizes actual typed native element identity without
executing it or consulting a public spelling. Native paragraph detection is a
typed helper for rules, not a generic expansion to all compositions. Legacy
strong/emph matching uses semantic origin, not a baked render style; list sequence
recognition preserves the current uniformity requirement.

Legacy Where preserves values_eq_semantic; native Element filter groups use
recursive language equality, names and ordered values under
00_nucleo/prompts/core/compiler/eval/operators/equality.md. Use the existing explicit
field projection (including text/raw/lang/block/body), without reverse imports
from entities. Queried-content fields delegate to
00_nucleo/prompts/core/compiler/eval/bindings/field_access.md and its pinned snapshot
contract: Some is authoritative, None uses its legacy path. Label absence and
explicit None remain distinct; Location is not a linguistic filter field.

OutlineEntry matches its own entry identity, not outline or heading. raw.line
uses its real typed built-in element and current dynamic identity; arbitrary
user-defined elements with a similar name do not inherit it. before/after remain
query-only and do not become show predicates. Preserve match/miss behavior for
all pre-existing selector variants; this helper does not fabricate query data.


## Carrier and equation identity evidence

Show selectors project internal carriers' body/visual. Carriers are no new element
or where-matching change.

equation-where-2026-10-06/before.json.gz retains historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA/UTC/HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`/exact tree/diffstat. Tekt rejects
equation.where as nonelement; vanilla accepts public catalogue, block/label/body/
numbering/alt, aliases/composition/show. Vanilla rejects size/missing/position/
closure/partial. func.rs:433–462 validates field_id/retains group;
math/equation.rs:58–170 separates public/internal fields. element_where.rs:63–490
omits equation; selector_matching.rs:269–310 omits native Equation matching.
block/body getter/snapshot already exist.

NativeElement identifies Equation by actual native_math_equation bridge pointer.
where uses existing identity/field path: authoritative Some snapshot, legacy None
projection. Keep recursive language equality/causal wrappers/other filters.
Reconstruct no payload/default/field or function name/constructor execution.
No NodeKind/new phase. Continuous ADR-0127 internal table correction, no public/
phase change. Consumer RED→GREEN/bilateral controls, preserve D04; this does not
realize bare math/D06 or matrix morphology/D07.

after.json.gz additional probe: valid where query0 versus Kind's two Equations.
from_tags/equations.rs:161–166 wraps realized Equation in Styled. Make Styled
transparent only for Equation identity; causal ShowRecipe/Located/Realized use
existing projection. Keep external object for field/config projection; do not
generalize Styled or override authoritative Some.

## Ratified bare table.cell selector

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz`
retains UTC/diffstat/inputs/commands/source-executable hashes. Historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`: selector(table.cell) gives cell,
where() cell.where(:), equality false, query/counter reject nonlocatable cell.
Real CodeBlock alias preserves results. Earlier document-only alias was unavailable
to expression eval, no positive-alias proof; table-aliases-before corrects fixture.

Pinned selector.rs:309–321 distinguishes dict None/Some; :421–450 separately
validates locatability. Tekt selector.rs:34–37 represents only present Element
group; foundations/selector.rs:69–77,348–354 omits native cell; other limits/hashes
in d08-owner-audit.json. Form/identity/query rejection are language; closed new
carrier is chosen mechanics. Source makes distinction explicit, no inferred
intent solely from output. Inference: variant suffices for selection/locatability
slice, refuted by public no-group/identity case not representable. No presumed
Cell realization correction.

### Approval and current equality contract

Human `confirmo` receipt on2026-10-06:
`lab/history/construction/records/00_nucleo/materialization/table-cell-selector-aprovacao-2026-10-06.json`,
UTC `2026-10-06T22:36:30.591466+00:00`, manifest
`f80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c`.
Earlier proposal gate is satisfied for D08A only.

Convert QuerySelector::NativeElement(f) directly to existing show
Selector::NativeElement(f), no new NodeKind/invented Where group. Keep static
native matching/authoritative Some/wrappers/prior variants. native_element_matches
already recognizes TableCell; no name-based duplicate or locatability promotion.
No preparation/position/field/direct show-table.cell producer authorized here;
those retain separate realization owners.

D08A covers only carrier/construction/repr/equality/composition/locatability.
D08B's direct show/body-fields/explicit-implicit preparation/positions/padding/
spans/header-footer/lexical scope requires measured own L0/gate for public-phase
change; D08A approval authorizes none of it. Hide no negative or claim universal
parity. Atomic approved set, language tests/valid RED before code/static safe
Rust,1:1/V26 before manifest-owner-only reseal; build/core/lint without errors.
Revalidate bare/where/alias/equality true-false/direct-composed query-counter/
homonym/partial-closure, preserving D05/D07/query.


## Ratified Source transparency

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical pin9f2b6e871. Existing carrier
operations provide transparency; no observable Source element. Human confirmo
2026-10-07 final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history. Existing ShowRecipe.body delegation
also accepts Source.body without separate-origin execution/published carrier/
Location. Preserve this operation's classification/preparation/serialization.
Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage before certification.

## Unused reexport evidence and scope

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
selector_matching.rs:21–23 composition/uses, separate productive/test import
checks in `lab/results/construction/diagnostics/imports-ci-2026-10-13/before.json.gz`
with sources/L0/UTC/status/diffstat. No parity/performance measurement.
Remove only unused crate-internal counter_keys_eq/selector_contains_element/
selectors_eq reexports here. Preserve values_eq and equality owner's definitions/
direct users; remove no external path. Internal composition, no public signature/
default/phase/language change. Keep scenarios/assertions/core suite/build/Clippy/
lineage, no allow/expect masking.

## Approved typed-transform identity

transformacoes-2026-10-14/results.json.gz/supplemental controls measure lost
identity/presence in matrix/body. Premigration source/owner in
`lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz`,
hashes/lines consumers.json. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`
authorizes new representation/layout resolution. Language semantics/morphology
versus Rust/reconstruction mechanics. Native selection distinguishes each operation
by executable function identity/typed variant; Matrix matches no native function.
Filters use retained lexical fields. RED→GREEN/dependent revalidation; build
alone closes no X1–X5.

## Exhaustive native-base classification

### Evidence before decision

selector_matching.rs:213 native_element_base is emitted V16. show.rs:63–96
has Text/NodeKind/NativeElement/DynKind/Regex/Where/And/Or/Label. Current source/
lines/functions checked against pinned SARIF/earlier AST inventory/literal matches.
`lab/results/construction/diagnostics/tekt-v16-show-selectors-2026-10-16/before.json.gz`
retains HEAD/uncommitted tree/UTC/hashes/sources/status/exact diffstat;
classification.json distinguishes emitted warning/latent predicate. Pattern/
exhaustiveness mechanics, existing language selection/text/effects/error preserved;
no fresh parity/readability/performance proof.

### Exclusive base extraction and acceptance

native_element_base returns NativeElement Some, recurses Where only. Nominal
NodeKind/DynKind/Text/Regex/And/Or/Label group returns None. And/Or gain no native
base. Change no kind_to_node/query conversion/Content match/field projection/
other fallback. S1 tests nine variants/nested Where same Func/compositions without
base/executable par alias/false homonym. No constructor/filter-field consultation.
Uniform outcomes share body; explicit groups demand decision for new Selector,
not duplicated bodies to zero V19. Informational V19 implies neither coverage
deficit nor multiplied tests; guard/recursive policies need own controls. One
body execution proves no entire group. No allow/expect/severity/config/helper
hiding V16.

Continuous ADR-0127, no new public signature/field/trait/default/compatibility/
phase or reopened historical proposal. L0/tests before code/restricted bilateral
reseal. Precode local controls confirm behavior; structural RED is absent E0004
at six points in real copied core with copied-enum sentinel. After nominalization
all six require E0004, no stub/productive mutation. Remove five emitted V16,
keep external diagnostics/V19. Real tests/build/fmt/strict Clippy/Rustdoc/direct-
reverse lineage, 12 GiB/zero swap/one job; external V16 prevents global GREEN.

## Sequence construction/selection facade

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree.
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17/{before,expanded}.json.gz`
and before-sources.json.gz retain UTC/diffstat/status/binaries/sources/bilateral
cases at `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Vanilla
content/mod.rs:724–728 requires children; value.rs:644–650 casts Content/string/
symbol/none; typst-macros/src/elem.rs:573–604 generates constructor. Tekt
field_access/content_methods.rs:358–410 still exposes noncallable fallback;
element_where catalogue/selector_matching.rs:272–337 lack Sequence identity;
rules/set.rs:55–76 rejects only transform With, misses Sequence set. Language
semantics/morphology/diagnostics, not required upstream storage. Earlier pending
children() message already matches current baseline: fixed getter lets existing
owner produce correct error. New evidence rectifies debt; edit no diagnostic code.

### Exclusive Sequence identity and acceptance

Recognize native_sequence in Empty/Sequence by real pointer. Prescribed group:
two Tekt representations of same children-schema language element. Test both,
one/many children, alias/false homonym. Source/Located/ShowRecipe/Realized stay
transparent; ordinary Styled opaque, possibly holding selected Sequence child.
Filters use existing get_field/language equality, no flatten/plain_text inference;
Some remains authoritative. Change no NodeKind/ElementKind/show guard/phase/public
representation. Completes earlier R1-named R2 responsibility. Continuous
internal table/composition/callback correction, no new external field/trait/Rust
signature/default mode/phase/compatibility break. L0 before tests/code, functional
RED/GREEN/bilateral corpus. Certify no global show guards/all casts-elements/
universal render parity. Residual refuters stay reportable with owner/L0 before
intervention; change no tests to hide them.

Require separate build/workspace/doctests, strict Clippy/Rustdoc/fmt,
V1/V5/V15/V26/reverse/differential SARIF without new error-warning. Prescribed
nominal V19 allowed visibly without suppression, alternatives tested. Heavy tasks
sequential, MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; debug0/incremental0 in validation
environment only. Reseal after valid V15/V26/exact provenance.

## Sequence realization evidence and approval

before/expanded/controls.json.gz and candidate.log in
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17` fix HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree,
UTC/diffstat/sources/binaries and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Eight initial REDs; candidate
passed five, with show/recursion unresolved. markup_stream.rs:240–260 flattens
before selecting Sequence; realize/mod.rs:170–210 visits children before recipe;
apply.rs:140–170 delegates nonlocatable elements to fixed point. Vanilla accepts
alias and rejects a new Sequence with the same children. These are language
effects/scope/values/diagnostics.

Owner approved
`lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`,
manifest `46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327`,
on 2026-10-17. ADR-0127 §2.3 gate covers realization order of this boundary
before children/Table/math, not a new public contract.


### Exclusive Sequence-rule predicate and acceptance

Crate-internal is_sequence_rule recognizes NativeElement with actual native_sequence
callback. Where delegates base, And/Or search pertinent branch. Text/Regex/Label/
NodeKind/DynKind are nominal false sharing only result, no V19 suppression.
Execute no callback/use Func.name/make Styled transparent.

Ratified S1–S20 require precode RED/bilateral GREEN/regressions, no lowered
expectations for mechanical cap. Valid1:1/pinned graph before reseal. Separate
build/workspace/doctests/strict Clippy-Rustdoc/fmt/direct-reverse lineage/SARIF
without new error-warning.12GiB/High 8 GiB/swap0/jobs1/heavy sequential. No
publication/release.
