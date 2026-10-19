# L0 — compiler/eval/rules/textual
Hash do Código: ee2daace

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:66cf98676c3f6ddd3641e4e67eaed2ed2b71a06d5fd9a0be93dbc3059e506074
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:2213a4793474159cfae64b699263b16621a492ad2056fbaf5d28ac7950593c6b
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the measured working tree, `01_core/src/compiler/eval/rules.rs:3932-4435`
contains `TextualSnapshot`, text-node interception, grouping, rule slicing,
space normalization and their focal tests. `compiler/eval/mod.rs:1371,1408,
1416,1470,1550` calls the three entrypoints. The existing `p1494` test filter
passes 17/17 before movement; this is structural baseline, not a new bilateral
parity measurement. Source state and command are recorded in
`lab/results/construction/diagnostics/atomizacao-textual-2026-09-30.md`.

## Decision and ownership

Move that behavior to this descendant module without changing algorithms,
order, diagnostics, spans or language morphology. The parent `rules.rs` retains
set/show dispatch and one-node show application, and exposes this child to the
eval coordinator. No public signature, default, pipeline phase or compatibility
changes. The native dispatcher remains static and exhaustive; the child does
not import render or infra and does not create dynamic dispatch.

`TextualSnapshot` captures the causal rule/style/guard state. The two callable
entrypoints are `intercept_text_element` and `realize_textual_parts`; the
behavioral helpers and focused tests live in the two descendant owners. Calls
from `eval/mod.rs` refer to this coordinator; no copied implementation remains
in `rules.rs`. The shared one-node show application and selector semantics remain
owned respectively by `00_nucleo/prompts/core/compiler/eval/rules.md` and
`00_nucleo/prompts/core/compiler/eval/selector_matching.md`.

## Refinement measurement

At the working tree captured in
`lab/results/construction/diagnostics/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`,
this consumer contains the coordinator, the space algorithm at lines 335-463
and the matching algorithm at lines 96-333. The `p1494` filter passed 17 tests
before this second movement. The distinct inputs and helpers support two
behavioral owners; no CPU reduction follows from that measurement.

## Behavioral delegation

The coordinator retains `TextualSnapshot`, `intercept_text_element` and
`realize_textual_parts` with their existing signatures and phase. It invokes
space normalization only when `normalize_spaces` is true, then returns early
when show is disabled, then invokes the matching owner. Snapshot grouping
retains pointer identity for rules and equality of collapsed style and guard
state. Element-selector interception still uses the one-node show application
owned by `00_nucleo/prompts/core/compiler/eval/rules.md`; textual selectors remain deferred.

`00_nucleo/prompts/core/compiler/eval/rules/textual/space.md` exclusively owns collapse of adjacent
spaces, newline origin, explicit linebreak consumption and the pinned Unicode
spacing predicate. `00_nucleo/prompts/core/compiler/eval/rules/textual/matching.md` exclusively owns
realized group traversal, selector precedence, matched-slice transformation,
while its descendant `00_nucleo/prompts/core/compiler/eval/rules/textual/matching/projection.md`
exclusively owns preservation of unmatched original content. These algorithms stay in the
eval layer as descendant modules. This coordinator must not duplicate either
algorithm or introduce dynamic dispatch.

## Acceptance

The coordinator tests snapshot group identity across shared rules, distinct
rule allocations, collapsed style changes and guard changes. The existing focal
tests remain green, including literal/regex slicing, Unicode space handling,
lexical snapshots and guard behavior. Build and
`crystalline-lint .` have no new violations. Language content, diagnostics,
ordering and public contract are unchanged. No performance gain is claimed
without separate measurement. Header ownership remains 1:1.


## Deferred group transport

With defer_show_rules capture groups by lexical snapshot/style boundary before
textual recipes. Capture groups whose text emerges only from element realization;
do not decide match from pre-show form. Realization reuses matching/splice without
rerunning ordinary program. Internal realize_captured_text realizes resolved
elements in group, preserving causal origin and delegating matching/splice;
text replacement loses no origin. Reuse existing space normalization after
recipe realization, no duplicate Unicode insertion policy. Group snapshot keeps
contextual watermark, including lexical content absent from initial visual tree.

## Pending text single-evaluation evidence and boundaries

r2s-scope.json.gz/text-before.json.gz under
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17` retain
uncommitted state at `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, UTC/
diffstat/binaries and pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
textual-no-repeat/baseline/interpolation/captured produce AAAA(B) in Tekt versus
AA(B) in vanilla. textual.rs:104–122 wraps groups already containing same-snapshot
pending recipes; realize/mod.rs:182–210 realizes both bottom-up, repeating literal.
text-before cross-piece measures AB→X across separate children in both;
outer-hides-text yields Y without child's panic in both.

Within ratified S13, textual owner coalesces only still-pending textual recipes
from the same causal environment: rule Arc identity, collapsed styles, guards,
file. Preserve Source/order/morphology and distinct-environment/nontextual
barriers with element effects, except the redundant-scalar rule below. Coalescing
executes no callback. Capture coalesced children; Sequence owner coalesces text
body after realizing emitted sequences, before returning recipe to textual owner.
Do not replay same group per transport layer, skip Realized indiscriminately,
or change matching/precedence/scope/public contracts.

RED→GREEN covers literal/regex interpolation/capture, plain/block Sequence
identity, lexical prefix, cross-piece matching and hidden textual children.
Distinct snapshots stay opaque. Independent local-text-scope C vs B divergence
came from distinct-literal precedence; it remains measured history and cannot
authorize changing matcher order here. Later causal textual ratification below
addresses its own owner scope, without inventing closure from this earlier corpus.

## Completion boundaries and unresolved evidence

r2s-scope.json.gz/text-before.json.gz preserve the incomplete morphology/textual
scope after Sequence approval. Symbol cast identity/equality measured text/true
in Tekt versus symbol/false vanilla; a public variant needs its own gate, not
Sequence authorization. Markup must preserve interpolated Sequence as immediate
ContentBlock child without Sequence rule, previously flattened. Textual matching
must measure/correct cross-scope literals C vs B in its own owner. Generic show
traces for emitted children need their diagnostic owner. Historical package
names R3-Symbol/R3-Markup/R3-Textual/R3-Diagnostics identify these completion
scopes only; they are no L0 or authorization for additional code. Do not label
the differences accepted mechanics or proven parity. Later scoped ratification
does not silently certify every recorded debt.

## Consumed slots versus explicit Empty

empty-slots-before.json.gz in the Sequence diagnostic records sources/binaries/
UTC/HEAD/working tree/diffstat/current pin. Simple body yields EBODY versus BODY;
none/[] children EEA versus EA; A→AA adds Es. markup_stream.rs:205–225 builds
tail with consumed/normalized Empty slots; textual.rs:104–120 builds packets
with them; sequences.rs:114–125 visits them as real occurrences.

Within approved capture remove only naked Content::Empty slots from markup's
Sequence-tail vector and textual's pending-packet body. Explicit Empty under
Sequence rules already has capture/Source before entering vector. Keep those
carriers, empty Text and constructor child arrays. Never filter Empty arbitrarily
in domain visitor. body/[]/none/empty-constructor/none-child/[]-child/prefix/textual
corpus must match pinned PDF, including bodyless document. L0 first, module RED
before code; independently revalidate PDF/HTML.


## Packet-root and causal EOF refinement

emission-before.json.gz confirms AA in both when Sequence emits [A]/text("A")
under A→AA. Native #text("A") still yields AAAA versus AA: separate generic
nontextual replay, incorporated into S13's scalar correction below. Generic
recipes with applicable callbacks retain barriers; do not remove them to hide
realization differences.

validation-attempt-2.json.gz/trace-recipes/trace-markup2/trace-textual2 record 11
GREEN/two RED. eof-before.json.gz measures exact sources without adding newline:
Styled with EOF gives Y in both, without EOF AB in both. Unit Y fixture lacked
EOF used in prior probe; include measured EOF, preserving both external controls
without relaxing show semantics.

Textual alias without EOF yields AAAAAAAAB versus AAB. sequences.rs:112–116 skips
every textual packet root, but Content::sequence normalizes a single item to
itself. If that item is marked ShowRecipe, skipping leaves recipe pending while
capturing children again; generic owner reruns it, repeating text rules.
trace-textual2 shows marked recipe still opaque to later coalescing.

skip_root skips only artificial raw Empty/Sequence root, never normalized
ShowRecipe/Styled/other root. Effective marked root decides before children.
Preserve strict causal styles/rules/guards/file comparison and all slot controls.
Archive temporary cfg(test) traces and remove them from code before final gates.
No new field/type/API/phase.

## Redundant scalar text transport

validation-attempt-3.json.gz/emission-after.json.gz retain working tree/UTC/
sources/argv/binaries/diffstat at current pin. Sequence [A]/text("A") under
A→AA gives AAAA versus AA. Native text("A") already does so in stack-release SHA
`2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c`
(native-before.json.gz). textual.rs:142–161 treats Sequence marker/guard as
textual-scope distinctions, retaining generic scalar recipes applying same
literals before parent. trace-textual2/instrumentation-sources record private fields.

Sequence owner supplies crate-internal textual-transport comparison excluding
only eval.sequence.occurrence from collapsed delta and guards whose RuleId belongs
to Sequence rule in snapshot. Unknown/other-family guards remain. Filter no real
style; change no external signature.

Textual owner keeps strict TextualSnapshot::same_group. Pending textual packets
with same rules Arc/file/equivalent transport realize once. Eliminate generic
nontextual redundant transport only for raw Text (Source allowed), ordered rules
with exactly parent's RuleIds, same file/equivalent transport, and no element
rule matching Text. Then no element callback remains and parent already supplies
literal/regex; return body with Source. Never remove Styled/structural/contextual
recipe or applicable-element-rule recipe. Remove no Engine rule and run no
coalescing callback. Existing same-context IDs are identity, not names/Debug/
product text.

RED→GREEN covers [A]/text("A") emission/native scalar with pin expectations;
other-family guard/real-style controls remain opaque. Revalidate prior 13
controls without relaxation. Internal ratified single-application correction,
no new public type/field/phase or other-element policy change.


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

realize_textual_parts returns crate-internal SourceResult<Vec<bool>> matching
parts length, initially false. Deferred capture marks consumed start+1..end.
Without show return all-false consumption mask; immediate route delegates matcher
mask. realize_captured_text builds visual only from preserved slots, retaining
complete original. This mask revision changes no coalescing/guard/rule order/
space-normalization policy; later ratification below governs its own changes.

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


## Current exclusive frames and product completion

Own frames/snapshots/textual-completion transport without public types. Share
baseline/rules by Arc; carry inherited/local classification in private ShowRecipe/
styles, acyclic graph/no fake Location. Baseline metadata stores rule snapshot,
not executable content; never store carrier in itself. Coalesce only PENDING
same-environment packets; completed product keeps boundary. Helpers only crate/
compiler-visible. Restore state/guards even on matcher/callback failure.

Require acceptance.json families/R2-S-R3/single-evaluation RED→GREEN, PDF/HTML
separately, build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/
reverse/differential SARIF without new error-warning.12GiB/High 8 GiB/swap0/jobs1,
heavy sequential. No RAM gain without paired experiment/suppression to clear gate/
commit/push/release.

### Measured coordinator interoperability and supersession

Broad candidate3 validation required compiler-only has_causal_scope(recipe) to
recognize BASELINE metadata without executing it/exposing language key; Table
used it to route new causal packets versus legacy. No public Rust contract.
The later all-Table-packet delegation below removes this predicate; BASELINE
metadata remains unchanged.

Candidate4 refutes coalescing only generic Text packets: extra insertion boundary
also contains purely textual sequences. Generic packet may be removed only when
whole body groupable and no node has applicable element rule in effective
environment. Check complete catalogue with static map, not root type alone.
Never remove marked Sequence packet: it must decide root even when Empty.
Completed products remain opaque. Without eligible text rule realize_captured_text
returns shallow clone, no fabricated Realized without transformation/unmeasured
performance claim.

Candidate5b still fails T52/sequence_constructor_textual_groups_are_not_repeated:
generic packet nests textual packets, making can_group false before redundant
transport removal. For compatible environment prepare recursively pure coalesced
body view before deciding wholly textual/no element effects. Otherwise retain
original packet; execute nothing/change no observable child. Marked Sequence
remains opaque, completed products never join. Preserve both RED expectations
exactly ASB/X.

Candidate6 fixes AB grouping but T52 produces ASSB: two Spaces in child Sequence
were not normalized when space owner received external vector only. Before
matching already-eligible group, build flat emitted-node view retaining Source
per slice and Realized provenance through empty-visual marker. Store whole original
as Realized.original; change no value/field/Sequence before nominal root decision.
Completed products stay boundaries. Apply existing normalize_markup_spaces to
emitted view, then match/project there; no duplicate Unicode/weak-space/per-byte
state. All Table packets now delegate coordinator, removing has_causal_scope
without changing BASELINE.

### Deferred Context contribution and canonical identity

ContextBlock is deferred marker, already transported by capture_context_recipe
owner where pertinent. capture_local_output creates no extra generic packet merely
because new rules surround ContextBlock, including through Source/Styled. That
forced realization/document finalization despite no applicable root recipe,
changing identity range before contribution existed. Pure contextual_root recognizes
only marker; executes/removes no wrapper/closure. Preserve local deltas of emitted
values and Raw/Sequence transport. Nominal Content complement retains visible V19.
