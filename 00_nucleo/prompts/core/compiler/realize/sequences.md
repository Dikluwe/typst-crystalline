# L0 — compiler/realize/sequences
Hash do Código: 3dd9da1e

Layer: L1
Exclusive consumer: `01_core/src/compiler/realize/sequences.rs`


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:66cf98676c3f6ddd3641e4e67eaed2ed2b71a06d5fd9a0be93dbc3059e506074
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3

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

## Exclusive capture and nominal root decision

Implement the pure Sequence capture/realization descendant. Transport boundaries
through existing ShowRecipe with a private style-snapshot marker. Keep rules/
styles/guards/file/origin snapshots; protected argument carries consumed RuleIds.
Filter global Sequence guards from fresh capture, never other-family guards.

For a marked root install/restore lexical environment on every return. Select
applicable unconsumed rule with existing precedence, build protected it and
delegate one step to apply owner. Revisit new products; guarded aliases may
apply earlier unconsumed rules. With none remaining realize effectively emitted
children and remove only private transport. Execute no hidden recipe, reevaluate
no file, fabricate no Sequence tag/Location. Retain marker until current product
no longer needs protection; body equality is no guard.

Sequence show-set preserves styles without repeated effects. Expansion uses
existing private occurrence_show_depth and limit/hints. full_error adds only
diagnostic classification, never termination by morph_canon. Source/inheritance/
other callbacks stay with their owners. Helpers are crate/compiler-visible only,
no new external type/field/trait/pub fn. Local RAM, no globals/I/O/unsafe.

Under the later causal-coordinator ratification below, recipes calls this owner's
realize_root only on the emitted occurrence before children. No global generic-
recipe prepass or textual-recipe execution belongs here. Never flatten Sequence
before selection. Preserve guards/alias-fresh/Source/styles/S1–S20 and pinned
sequence-realization nucleus. Remove obsolete crate-internal sequences::realize
facade: recipes is the sole coordinator entry and calls realize_root. marked is
visible only within crate::compiler so textual owner preserves Sequence packets
before coalescing; it reads only existing private marker, no public API.

## Transport flags, emitted children and generic snapshots

R2s-first.log leaves Empty without Y while other sequences pass; eval uses
has_deferred_show_rules to decide realization. Every effective marked capture,
even empty, sets this private flag. Installing recipe styles shadows transport
marker with false: callback-created generic recipes must not inherit Sequence
classification. Protected argument receives its own true marker. Other families'
capture/public surface stays.

After consuming root guards visit effectively emitted children, including raw
nested Sequence created before rule. Do not recapture guarded root. When crossing
generic ShowRecipe install/restore its environment to visit children, without
changing type or executing generic recipe in this phase. Internal text-grouping
root is not a fresh language Sequence; skip only artificial raw Empty/Sequence
root, preserving children. Return changed body to generic owner; other families'
semantics remain there.

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

## HTML scaffolding evidence and intentional boundary

Sequence target-controls.json.gz/footnotes-controls.json.gz retain uncommitted
state/UTC/diffstat/sources/argv/binaries at
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Vanilla bare-show HTML contains
<p>Y</p>Y, PDF Y. children:() filter adds FOOTER after BODY only in automatic
HTML; explicit html.body removes it. Tekt in this collection independently has
EBODY slot defect above, not accepted here.

Pinned typst-html/src/document.rs:293–302 always realizes FootnoteContainer and
appends fragment to automatic body. rules.rs:382–387 intends no container without
notes but returns Content::empty; typst-realize/src/lib.rs:259–270 applies show
before expanding Sequence. Supported inference from explicit html.body control:
extra text leaks internal empty-note scaffolding into author show, neither note
content nor author occurrence. Explicit body retaining extra text or author
origin would refute it. Do not emulate this render artifact. Preserve show of []
and explicitly constructed Empty children. Intent is supported by comment/source/
differential control, not behavior alone. Keep individual target difference;
claim no general HTML equality.

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

## Conditional origin capture

validation-attempt-4.json.gz/complete-workspace.log/conditional-origin-before-sources.json.gz
retain HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted state/
UTC/diffstat/sources/binaries/current pin. Core suite: 7228 passed, 24 failed,
five ignored. Two local-numbering controls lose inner elements; others fail
prefix/tail geometry after 1fr. Prior validated R1 reference had zero failures.
markup_stream.rs passes content.with_source_span(child.span()) even without
Sequence rule or capturable body. Helper returns added carrier without capture;
ContentBlock append_markup_sequence preserves Source, altering legacy style/
column boundaries needing no capture. Origin addition belongs to effective capture.

Crate-internal capture_at takes raw body and real Span, adding Source only when
apply_show_rules is active, pertinent snapshot exists and body is Empty/Sequence.
capture keeps current internal signature, delegates with detached Span;
capture_scope receives explicit Span, uses it only when creating marked recipe.
Markup/CodeBlock pass raw body and real origin separately. No pertinent rule or
uncapturable body returns exact input, preserving default scopes and avoiding
useless carrier allocations. Change no layout algorithm or 24 expectations.

Full-suite RED already observed; require GREEN for f2s, set/prefix/1fr integration
and Sequence controls before full validation. Retain []/call origins, nominal
guards, nonlocation, causal coalescing. No external Rust signature/public field/
default mode/additional phase.

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


## Acceptance and limits

Require ratified S1–S20, real RED before code, bilateral GREEN/regressions;
never reduce expectations for mechanical limits. All 1:1 owners/pinned nuclei
valid before reseal. Require separate build/workspace/doctests, strict Clippy/
Rustdoc, fmt/lineage/reverse/SARIF with no new error/warning. Use
12GiB/High 8 GiB/swap0/jobs1, heavy commands sequential. No publication/release.

For the later causal revision, require acceptance.json families, R2-S/R3 corpus
and single-evaluation controls. Validate PDF/HTML separately; build/workspace/
doctests, strict Clippy/Rustdoc, fmt, V1/V5/V15/V26/reverse and differential
SARIF without new error/warning. Same resource ceiling. Claim no RAM benefit
without paired experiment; no suppression to clear gates. No commit/push/release.
