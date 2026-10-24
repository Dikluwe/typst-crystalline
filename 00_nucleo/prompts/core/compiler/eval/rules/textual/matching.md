# L0 — compiler/eval/rules/textual/matching
Hash do Código: d0144b3b



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:2213a4793474159cfae64b699263b16621a492ad2056fbaf5d28ac7950593c6b
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3


## Measurement before decision

At the working tree captured in `lab/results/construction/diagnostics/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:96-333` contains realized group traversal, rule precedence, matching, transformation and projection of unmatched original ranges. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own the complete realized textual group algorithm, from grouping by causal snapshot and style boundary through match selection and replacement, delegating preservation of unmatched
content to `00_nucleo/prompts/core/compiler/eval/rules/textual/matching/projection.md`. Search literal/regex over the largest eligible realized group of the same lexical snapshot and style chain, including separated Text/space and dynamic text nodes. Each recipe receives only its matched slice; preserve leftmost order, unmatched prefix/suffix, precedence, guard/revocation and depth limit. A replacement is not fed back into the same revoked rule. Empty regex matches remain rejected and literal '.' stays distinct from regex '.'.

The initial value-stage ContentBlock contract preserved raw Space/SpaceFromNewline
and provisional local show before normalization. Its combined local CJK and
sequential-identical-rule parity gaps are retained as historical measurements;
the later ratified causal frontier below supersedes provisional show and old
regex/literal pass policy within its scope. Ordinary expression evaluation stays.

The coordinator passes its existing inputs after optional normalization and early return when show is disabled. This module may access the parent snapshot and call the existing selector matcher/call dispatcher, but must not duplicate space normalization, alter the engine restoration order or introduce dynamic dispatch. The descendant projection owner preserves unmatched nodes and flattened
replacement sequences; its unmatched-slice test lives with that algorithm. No phase, public signature or default changes.

## Projection refinement evidence

At the working tree captured in
`lab/results/construction/diagnostics/atomizacao-textual-projecao-2026-09-30-before.json.gz`,
this consumer contains `project_original_range` and `append_flat_sequence` at
lines 189-223, followed by the matching transform. The slice projection test
passed in the `p1494` filter after the first movement. Their only input is
content and range/sequence structure, so projection can be a pure descendant
without access to engine state.

## Acceptance

The existing literal/regex slicing, lexical snapshot and guard tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.


## Realized text and Source transparency

Groups may project Realized/Located and exclusively textual element-produced
sequences. Structural elements remain boundaries. Group's causal snapshot retains
origins consumed by splice. Textual element product participates in group matching;
spaces alone require the contextual-support policy below rather than an invented
source recipe.

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Existing carrier operations provide
transparency; Source cannot leak as observable element. Human confirmo2026-10-07,
final approval record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history. Source text grouping delegates only
body for regex/text, retaining origin on reconstruction without extra boundary.
Sole owner/static dispatch/no I/O/new signature/trait/flag/eval-layout phase.
Require C2/C/macro tests/recollections/build/lineage.

## Nominal barriers and transparent whitespace boundaries

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/tekt-v16-whitespace-2026-10-17/before.json.gz`
retains sources/L0/config/hashes/UTC/status/exact diffstat. contexts.json AST/spans:
11 emitted V16/one latent identity forwarding in classified_content.
measurement-before.log measures five differences in seven Source pairs.
measurement-family-before.log/source-family-before-analysis.json measure28 pairs,
20 differences: five left/right break/Empty-crossing cases fail equally under
Source/Located/Realized.visual/Styled. Parbreak/Text boundaries/support remain.
Real in-crate algorithm in disposable workspace, not adapted private-helper copy
or universal parity proof. First external probe failed E0624, preserved without
functional RED credit.

### Exclusive textual classification

W01 at matching.rs:243, textual_piece: only text/space/empty and existing projected
carriers enter group. Other variants remain boundaries; cross no Styled/ShowRecipe
or alter snapshot/recipe. Exclude only unguarded Realized, Source, Located,
Sequence, Empty, Text, Space, SpaceFromNewline from nominal complement; no guarded
exclusions.

### Acceptance and limits

L0 before code; existing-invariant internal continuous ADR-0127, no new field/
signature/trait/API/default/phase/dependency/flag/compatibility. Complement removes
only unconditional Content arms. Uniform per-case V19 stays visible, not proof
every alternative executed. No allow/expect/@/if-let hiding decision. HTML keeps
absorbing Boundary arms and nominally closes three complement cells; no change
to preformatted/raw/title/ruby/tag tables/SmartQuote.

Add owner regressions in three modules: both normalizers, single/nested Source,
four carriers at breaks/Empty, Parbreak/Text/MathIdent controls, span/style,
Realized.visual versus original, positive/negative matching projections and3×3
inline/top-level Boundary/EmptyInline/Supportive matrices. Relax no old test.
Real RED before code/GREEN after. Future Content/SpaceSupport probes compile
cfg(test): real enum in core; disposable infra copy unlocks only out-of-target
E0004 dependencies using recorded synthetic never-executed panic arms. Never
repair selected site inside probe. Distinguish blocked target compilation from
its actual exhaustiveness. Record12 final matches/13 arms including new helper;
two space_with_origin arms are one compiler match, not two errors.

Require full workspace build/suite/fmt/all-targets Clippy -D warnings/-D
unfulfilled_lint_expectations/Rustdoc -D warnings/four indexes/strict
V1/V5/V15/V26/reverse/replay/full differential SARIF. Expected V16 152→141,
V19 only prescribed groups, no V20 increase. MemoryMax 12 GiB/zero swap/one job,
heavy tasks sequential. No commit/push/merge/release/new CI pin. Global objective/
math-stretch-composite remain open.

## Consumed slots and markup None presentation

### Evidence before decision

`lab/results/construction/diagnostics/sequence-r3-2026-10-18/{before,markup-after,textual-before,transport-before,transport-before-sources}.json.gz`
retains HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/
UTC/diffstat/sources/executables and vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Under nonmatching literal Z,
nested.children exposes packet Sequence+two Empty in Tekt, original three children
in pin; empty-child packet+Empty versus []/A. markup_stream.rs:201–249 rebuilds
slots textual.rs:103–129 zeroed; matching.rs:90–95 also zeroes consumed siblings:
private transport leaks into public morphology. Interpolated CodeBlock/None
vanishes: [#none#[]A] has two children versus three; X#none;B under Empty show gives
XB versus XEB. Historical value_to_display_content omits None. Pinned value.rs:
191–210 displays None as Content::empty; markup.rs:39–56 keeps SetRule/ShowRule
separate boundaries, :78 appends other expression values. No Rust-buffer copy or
implementation-inferred intent required.

### Exclusive mask and acceptance

Internal continuous ADR-0127, no external Rust contract/new field/mode/default/
phase. Private Vec<bool>, no fake Location/public carrier. Refuters: lost genuine
Empty/empty Text, shifted style/boundary, repeated callback/inconsistent indices.
None presentation changes only markup insertion, not generic Math/CodeSetTail
helper. SetRule/ShowRule keep own boundaries.

apply_textual_groups returns private all-false Vec<bool>. Only actual replacement
marks zeroed start+1..end slots consumed; no-match consumes none. Mask revision
retained then-current precedence/transform/splice; later measured ratification
below replaces that policy, not attributed to mask.

Functional precode RED: children/fields under nonmatching literal, []/empty Text
children, literal/let/assert/conditional None, Empty show on None, empty set/style
without artificial Empty. GREEN/bilateral corpus/R2-S/EOF/slots/styles/CJK/loops,
full build/suites/strict Clippy-Rustdoc/fmt/direct-reverse lineage/differential SARIF.
12GiB/High 8 GiB/zero swap/jobs1/heavy tasks sequential, no publication.

Markup value boundaries do not close public Styled/Styles morphology, label repr,
module/include value formation. Historical R3-Reflection/A3-include completion
packages require own evidence; precedence/guards/literal show-set belong to
textual ratification below, not this mask.

transport-after.json.gz confirms consumed Empty removal/genuine None. style-tail
still exposes synthetic Sequence(A,Space) child before Styled; rule-tail keeps
different tail form. Additional to consumption mask, not repr alone. Separate
R3-TextualGroups boundary must complete public group projection/equality/selection
across scopes while preserving authored Sequence; Reflection owns public Styles/
Styled. Do not claim those controls closed or flatten Sequence generically to hide
them. This incompleteness grants no change to other projection owners without L0.

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


## Current exclusive selection and product realization

Replace regex/literal passes with single lowest-start selection, lexical tie-break
and per-product revocation. Input prefix/suffix stay original environment. Capture
function-produced Content before realizing children; revisit only product through
private recipes coordinator. Separate products do not become group solely from
shared guards. Constant/alias/composite body processes once through remaining
rules. Eligible Text whitespace and real Styles show-set follow nucleus; space
owner keeps normalization. Change no Symbol cast/public contract.

Require acceptance.json families/R2-S/R3/single-evaluation RED→GREEN, PDF/HTML
separately, build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/
reverse/differential SARIF without new error-warning.12GiB/High 8 GiB/swap0/jobs1,
heavy sequential. No RAM gain without paired trial/suppression clearing gate/
commit/push/release.

### Weak boundaries and product unity

Candidate1 frozen in r3-t1-materializacao-2026-10-18/candidate1-sources.json.gz
passes51/56 textual probes. T30 gives AB instead of X: [A]+[B] packets must
coalesce in effective product environment before coordinator visit, never completed
products/root-effect packets. T35/T51/T52 turn source-leading newline after show
to S; T53 leaves callback-emitted space without remaining rule. Common-flow
selection excludes weak Space/SpaceFromNewline edges from searched text, keeping
original carriers in projection; Text whitespace stays text. Protected-product
space-only eligibility follows the later bilateral-support refinement below.
Compute UTF-8 bounds by nodes/intervals, no per-byte state. Excluded prefix/suffix
still project with unchanged morphology/normalization.

### Contextual support of emitted spaces

Candidate2 passes63/64; T36 changes A's isolated emitted space to S where oracle
emits no text. T53 requires S when same product lies between X/X. Guards do not
determine textual support. EvalContext privately carries left/right support bools
during product realization. Matcher derives pair from match prefix/suffix intervals
and current external support, restoring on success/error. Include only supported
weak edges; space-only product eligible with support on both sides. Explicit Text
whitespace stays eligible regardless. No public data/global state/layout step/
permanent support capture in values: neighbors depend on current insertion.

significant_bounds has nominal Content complement including guarded Text/Realized,
excluding only unconditional Sequence/Located. All others are boundaries without
text interval per textual_piece. Intentional V19, no every-alternative coverage
proof.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

Expose the complete Symbol text to textual regex matching and significant bounds. Native text-element filters remain separate from regex/literal selectors; no grouping across mathematical elements is introduced.

Candidate1 realization regression fails on math regex: Symbol has textual payload but significant_bounds excludes it. Treat a nonempty Symbol string as significant over its entire byte range, matching Text bounds without trimming or changing text-edge policy.
