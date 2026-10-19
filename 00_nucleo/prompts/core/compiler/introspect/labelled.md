# L0 — introspect/labelled
Hash do Código: 98bf0d4c

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/labelled.rs`.
The pure compute_labelled body at `:24` reads typed, location-aware counters.
The auto Label branch at `01_core/src/compiler/introspect.rs:2300` passes
the causal wrapper chain's language, with the explicit language argument only
as fallback. Real occurrence Start/End and generic snapshot handling remain parent-owned.
The Label descendant owns its complete walk branch and derived label handling.
Heading language text belongs to the canonical helper at
`00_nucleo/prompts/core/compiler/lang/heading_supplement.md`.

## Walk-branch refinement measurement

At the uncommitted state captured in
`lab/results/construction/diagnostics/atomizacao-introspect-heading-label-2026-09-30-before.json.gz`,
`01_core/src/compiler/introspect.rs:2280-2350` owns the Label walk branch,
while this consumer already owns location-aware resolution. The `introspect`
test filter passed 263 tests before movement. This is evidence of separable
behavior, not a new timing or bilateral vanilla measurement.

## Contract

Preserve the pure descendant free function over introspector, target Location,
Content and optional language, returning (optional resolved text, optional
figure number). Do not mutate stores or allocate identities.

For Heading, read its element-function hierarchical counter at Location and
join canonical localized supplement, a space and the number when available.
For a block Equation, read its element-function flat counter; a positive number
produces the existing compatibility text `Equação (N)`, otherwise no text.
For Figure, read its per-kind counter (default image); a positive number
produces localized figure supplement plus number and returns that figure number.
Otherwise return present empty text and no figure number. Source delegates by borrowing its body to this same helper with unchanged
Introspector, Location and language. Other targets return neither. Do not silently unwrap arbitrary Styled targets or infer a missing
numbering gate from the body's render.

This helper preserves compatibility resolved text; it is not the authoritative
numeric-reference path and must not override its precedence. It does not replace
the equation runtime supplement owner with its compatibility literal.
The Label walk branch reuses target Location, captures causal language and
publishes the derived association only when applicable.

## Derived-label walk

The Label descendant walks its target first with the causal label. For an auto
label it finds the first newly emitted real Start, resolves Heading/Equation/
Figure text with the wrapper chain language before the explicit fallback, and
publishes a derived Labelled Start/zero-hash End at that same Location only when
resolution is available. If the target has no real Start, classify it as
unreferencable. For an explicit user label, walk the body once with the label,
compare known associations before/after and classify unsupported targets when
no association appears. Preserve numeric mapping precedence; do not remove a
counter key to force legacy text. The parent match remains exhaustive and
calls this descendant statically, without a second traversal or new Location.

## Acceptance and phase boundary

Check hierarchical/flat counters at the supplied target Location, missing
values, captioned/numbered Figure paths and unsupported targets without invented
associations. Check mixed-language and forward/backward heading references
against the causal store, including an explicit synthetic language fallback.
A language present in the wrapper chain always wins over that fallback;
reference/layout/environment language is not a substitute.

A show-set that changes language only in the rendered tree is not a pre-show
introspection correction. Its parity is unresolved at this boundary and any
phase change requires its own L0 and ADR-0127 approval. Preserve current
fallbacks without promising a completed language-parity result for them.


## Ratified compact carrier emission

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz`
retains UTC/sources/L0/hashes/status/git diff HEAD --stat. Existing owners were
read before proposing. probe-details.json retains rustc 1.92/argv/rlib/binary/
probe SHA-256/output; probe-result.json records 12 GiB/zero swap/one job/events.
This synthetic carrier/register-copy probe is not compile/layout/RSS benchmarking
or a real workload distribution. Candidates mirror real domain types; inline
uses productive Tag/ShowRule.

Tag is 416 bytes, boxed/Arc Start 48. For 4,096 Start/End pairs with heap-free
Heading, requested bytes/allocations are inline 3,407,872/1, Box 2,031,616/4,097,
Arc 2,097,152/4,097. Cloning boxed Tag allocates per Start; inspected productive
fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads,
without cloning the vector. Walk:325–356 populates the index by reference before
moving info to Start. Synthetic clone frequency is not assumed productive.

Transformation is 328 and ShowRule 376 bytes; Box/Arc<Styles> candidates are 120
and 168. rules/show.rs:464–470 copies to_vec before every registration;
apply.rs:108–112,349–352 filters/clones rules. Across 128 registrations of simple
100% show-set without retained old snapshots, requested bytes/allocations are
inline 12,276,960/384, Box 8,194,560/8,640, Arc 5,530,624/512. With retained
snapshots, requested peaks are 3,203,856, 4,141,744 and 1,477,808. A custom-channel
control copying 4,096 rules requests 4,107 inline, 8,203 Box and 11 Arc
allocations, including Vec growth. apply/intercept/outline/table_cells borrow
styles; application still clones a delta when needed.

Mandatory counterexample: building one simple 100% show-set collection of 4,096
rules requests 1,540,096 bytes inline, 2,031,616 Box and 2,097,152 Arc. Arc may
retain excess custom-channel capacity; the rich 128-registration control without
old snapshots ends with 78,064 live bytes inline and 156,432 Arc. Claim no
universal gain or ignore atomic/indirection cost. This is ADR-0030/0107 storage
mechanics, not language parity or measured AI benefit. Real relevant memory/time
regression refutes the favorable copy-cycle inference; retain those controls.

### Ratification and exclusive responsibility

The human approved manifest
`282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`
records approval. The former ADR-0127 public-payload/compatibility proposal gate
is satisfied; this contract, including T1–T6 and no release, is in force.

Migrate only existing Tag::Start(loc, info) emission to
Tag::Start(loc, Box::new(info)). Preserve info construction/values, index
population before moving info, loc/End/hash, walk order and effects/counters.
There is one allocation per Start; do not clone info or move population, change
deduplication/reservation/snapshot/recursion. Public form belongs to
`00_nucleo/prompts/core/entities/tag.md`; this owner preserves its own behavior.

### Acceptance and owner boundaries

Ratified L0 precedes tests and real RED; reseal strictly changed owners. No
large_enum_variant suppression, threshold/dependency/flag/default/phase change.

- T1: construction Box<ElementInfo>/Arc<Styles> and relative budgets (Tag at most
  three Location; Transformation at most Content plus alignment) fail before
  and pass after without freezing a universal ABI. Those payloads stay owned by
  their entity prompts.
- T2: preserve Start/End/Location/payload/label, value equality/structural hashes,
  independent Box clone and mutation isolation; hashes are not persistence format.
- T3: existing introspection/convergence/nesting/heading/label/state/counter/query/
  metadata tests retain expectations/errors.
- T4: Transformation::Style clone shares Arc; Arc::make_mut on a copy preserves
  original styles including custom; Func/Content/Str variants remain unchanged.
  This is a dependent entity boundary, not implementation in convergence.
- T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex
  controls preserve semantics/morphology/errors with no extra execution.
- T6: full workspace/build/all-targets Clippy with prior expectations/lints as
  errors, rustdoc -D warnings with four indexes, fmt/diff/strict/reverse lineage
  and no Tekt increase, under 12 GiB/zero swap/one job. Record HEAD/working tree/
  hashes/diffstat/UTC and distinguish requested sizes/bytes from RSS.

No release. Any signature/field/phase change beyond these ratified payloads
requires a separate proposal. This owner preserves its existing emission behavior apart from the ratified boxing.


## Measured carrier transparency and nominal classification

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree.
`lab/results/construction/diagnostics/tekt-v16-introspect-carriers-2026-10-17/before.json.gz`
retains UTC/status/diff HEAD --stat, sources/L0 and 2130 verified hashes from the
preceding closure. contexts-before.json identifies three existing V16.
before-citation-probe.json preserves argv, rlib/program SHA-256, source freeze
and output: in 12 cases (four carriers × three positions), unwrapped tree order
k2,k1 becomes empty, losing known citations. bibliography.rs:21–76 collection
and :79–220 conversion do not traverse Source/ShowRecipe/Located/Realized.
These internal carriers have no vanilla syntax; this is a metamorphic proof of
required transparency, not a comparison of Rust representation.

labelled.rs:45–80 returns absence for Source although walk finds the inner target.
Hypothesis: compatibility projection loses text for the same Heading/Equation/
Figure at the same Location; confirm through functional RED before changing
this helper. Existing prohibition on arbitrary Styled unwrapping remains;
Source is not Styled. populate.rs:24–44 saturates future ElementPayload variants
into None. The current domain has 19 variants and active boolean guards remain
required. before-probe-result.json records 290 symbolic cases and zero E0004
when fictitious variants are added to both enums: missing structural protection,
not a demonstrated current numeric defect or measured AI benefit. Opaque
payloads and boolean guards do not prove data semantics.

### Exclusive responsibility

In compute_labelled add only Source's borrowed recursive delegation with the
same Introspector/Location/lang. Preserve Heading, block Equation, Figure and
all walk_label behavior. Replace the final fallback with the nominal complement
of 117 variants, including Equation when its block guard rejects. Styled stays
wrapped even under Source. Add a local test using real counters already prepared
at the supplied Location, comparing all three direct targets with Source plus
negative controls. Compatibility projection neither replaces the authoritative
numeric path nor repairs its supplement policy.

### Acceptance and limits

ADR-0127 internal/parity flow adds no public contract/default/phase/I/O/dynamic
registry/signature. L0 precedes tests/code; functional RED precedes implementation,
not a probe-construction failure. GREEN requires all 12 citation orders equal to
baseline, Realized visual without original contributions, unknowns preserved and
no-key fast path without reconstruction. Label Source preserves direct projection
and Location/lang; Styled and other targets keep their existing boundaries.

The disposable mutant must change zero to two selected E0004. Preserve 290
symbolic classifications except two label-helper cases now recognizing Source.
The 17 negative payload alternatives include rejected flags (Equation separates
block false from block true/numbering false). Source is not neutralized as an
unknown target. Keep owner-justified V19 notes visible, no new V20 or suppression.

SARIF must remove only labelled/populate's two V16, add two nominal V19, and
preserve bibliography::collect_bib_keys V16 until its separately specified
follow-up, with all other diagnostics unchanged by source/rule/severity/message.
Do not call untraversed static variants legitimate terminals. Replay permits
only prescribed repairs/tests. Validate V15/V26 before restricted three-pair
resealing, build/focal/workspace tests, all-targets Clippy -D warnings/-D
unfulfilled_lint_expectations, strict Rustdoc, fmt, strict/reverse lineage,
source freeze and diff --check. Heavy tasks sequential under
MemoryMax 12 GiB/High 10 GiB/swap0/jobs1. No commit/push/merge/release. Global and
preceding work remain active; this scope certifies neither universal bibliography
nor references.
