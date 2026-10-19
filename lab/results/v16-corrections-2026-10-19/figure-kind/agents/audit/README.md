# Figure V16-055 independent boundary audit

Status: measured, not implemented and not approval of a public design. These
observations refute a local wildcard replacement or a name-only kind repair as
sufficient to close Figure V16-055.

## Measurement and provenance

`boundaries.json` records 47 paired probes (94 invocations), exact source and
SHA-256, argv, UTC start/end, exit status, stdout and stderr. Baseline HEAD is
`d4e72102df01f9c248dffac761662d0f3cbf699e`, branch
`codex/tekt-mature-layout`; the recorded tracked diffstat is empty. No Cargo build
was performed. The reused product binary is SHA-256
`e114a4889a17292590a6a3c2ed0feeafd13dff970215740624fdf9aae0e8ef78`.
Its accepted receipt is the prior Circle build at HEAD `954d1318` with an
uncommitted tree. All 2132 recorded inputs match the current baseline before and
after the audit. This is input-equivalent reuse, not a build at `d4e72102`.

Oracle commit is `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Both executable hashes remained unchanged. `source-verification.json` verifies
the six cited upstream source files byte-for-byte against that commit's blobs.
These identity checks establish provenance; source bytes are not language parity
acceptance criteria.

`probe.py` reproduces the final observations. `probe-initial.py` and
`boundaries-initial.json` preserve the initial 40-case measurement, including two
inconclusive no-caption supplement-target fixtures. The final run retains these
as negative controls and adds caption-materialized target observations.
`candidate-policies.json` freezes parsed language values or first diagnostic
messages; it is a candidate acceptance inventory, not a claim of full Figure
parity or an authorization to change public contracts.

## Observed behavior

- Typed element functions remain typed: explicit `table`, `raw`, `image`, and
  `heading` with custom supplement are function-valued kinds. The string
  `"table"` is a string-valued custom kind. Current product Figure reflection
  rejects `it.kind`, so that fact alone prevents claiming any typed-kind query
  parity on the baseline.
- With `set figure(kind: raw)`, an omitted kind resolves to raw, while explicit
  auto resolves from the table body to table. Under a filtered show-set
  `figure.where(kind: table): set figure(kind: raw)`, omission resolves to raw,
  but explicit auto and explicit table both remain table. Reusing one Figure
  value under scoped raw/image defaults yields raw and image in occurrence order.
  A single constructor-inferred value cannot represent all of these outcomes.
- Semantic body inspection sees Table in Metadata Content or recursively nested
  Arrays, before a following Raw. Dictionaries are opaque even when containing
  Arrays, and Arrays do not make nested Dictionaries transparent. Raw in the
  first Metadata wins before a later Table. Context-produced Table and later
  show-produced Table remain image in these probes.
- Default numbering on custom string kind requires an explicit supplement.
  Explicit content, explicit none, a callback, or numbering:none pass the tested
  boundary. Table element and string homonym select different supplement callback
  targets: the first Table descendant versus the body Sequence, respectively.
- Supplement callbacks are invoked and their errors propagated even without a
  caption and even with `show figure: none`. This is different from counter
  history demand and must not be folded into a caption-only callback policy.
- Counter callbacks panic with a caption (including custom numbering callback),
  but not without a caption, with numbering:none, with show-hidden Figure, or
  with show-hidden parent Block. A direct `counter.at` demand without a caption
  does propagate the panic. A callback after the final captioned Figure still
  panics: demanding a prefix validates the complete relevant history. Callback
  access to `text.size` reports `can only be used when context is known`.
- Corrected counter fixture puts the metadata label inside the Context-produced
  content. Manual Set(7) followed by one Table Figure gives `[8]` in oracle and
  `[7]` in product. Set(7), Figure, Func(+10), Figure gives prefix states `[8]`
  and `[19]` in oracle versus `[7]` and `[17]` in product. Typed Table and custom
  `"table"` tracks initialized separately give `[8]`/`[21]` versus product
  `[7]`/`[20]`. These paired cases identify missing automatic typed-track steps;
  they do not prove name tracks collide with element tracks in every product path.
- Closure-as-kind and integer-as-kind already reject with the measured expected
  diagnostics on both binaries. Preserve these positive controls.

## Source intent, inference, and refutation

The upstream language documentation at `model/figure.rs:207–245` explicitly
supports auto inference, typed element or string kinds, and kind-specific
counters. This supports treating those observations as language semantics.
The supplement parameter documentation at `:276–284` describes localized default
supplement and callback target. Synthesis at `:345–425` implements typed lookup,
custom-name validation, callback target selection, counter construction, and
caption projection. Current product `entities/elements/figure.rs:25` stores only
`Option<String>`, and `compiler/stdlib/figure_image.rs:129–145` discards element
function identity into inference. These are measured carrier losses, not merely
missing exhaustive match arms.

The selection preview versus final resolution distinction is explicit in the
upstream comments at `typst-realize/src/lib.rs:446–456` and `:557–575`. The local
implementation need not copy clone/memoization mechanics, but must preserve the
observed selector and final-kind outcomes. Inference: independent occurrence
resolution plus lexical presence is necessary; an alternative representation
that reproduces every boundary would refute a claim that any particular struct
layout or enum is uniquely required.

The body traversal at `foundations/content/mod.rs:408–464` uses public fields,
Content, and recursive Arrays, with other values opaque. The observed Metadata
and Dictionary probes agree. This is source-backed observed behavior rather than
an explicit language-documentation promise that every future field stays opaque.
Do not copy the wildcard or call unreviewed wrappers leaves; review the actual
public-field partition and keep Context/show-produced content unexecuted during
inference. A documented alternative traversal rule or counterexample would
require revising this candidate policy before implementation.

Counter `at` calls sequence construction before reading a prefix; the whole
history fold at `introspection/counter.rs:920–966` propagates every demanded
update error. Callback update invokes `Context::none` at `:607–615`. This explains
the measured demand and context boundaries. The literal full-vector algorithm,
cache layout, or pass count is not a parity requirement. Inference: the product
needs one coherent location-keyed demanded history shared by caption/reference
and explicit counter readers; a different design preserving all observed effect
and location behavior would be acceptable. Eager history processing for every
Figure is refuted by the no-caption/hidden controls; stopping validation at the
requested prefix is refuted by the after-last-Figure panic.

## Outstanding scope and gate

This audit does not authorize code or a new phase ordering. The intended fix
requires concrete L0 owner coverage and a reviewable public contract proposal
under ADR-0127: typed kind identity, lexical omission/auto/explicit distinction,
selection preview versus final resolution, payload/reflection reconstruction,
caption/reference counter readers, supplement representation and effects. The
old Circle approval covers none of those Figure contracts. Current user request
authorizes the investigation and repairs within existing approved contracts;
classify any proposed public or phase change explicitly before implementation.

No source, prompt, linter, configuration, severity, or alert suppression changed
in this audit. The evidence is scoped and cannot close Batch F by itself.

## Supplement controls added after the main audit

`supplement-hidden.json` records three additional paired probes with UTC, HEAD,
diffstat, binary hash checks and exact argv/source. Together with the 47-case
main audit there are 50 scoped cases, without claiming full parity. A hidden
parent Block prevents the supplement callback error, whereas a filtered
Figure show-none rule still propagates it. A supplement callback reading
`text.size` succeeds without a caption. This is consistent with
`model/reference.rs:394–410`, which invokes supplement functions with styles but
no location, and converts their result through display. It contrasts with
counter update `Context::none`. The returned context-dependent value is not
rendered in this no-caption control; success establishes accessibility, not a
particular size.

Upstream preview calls full synthesis and discards errors via `.ok()`. No probe
here establishes a user-observable callback invocation count. Do not make clone,
pre-synthesis call count, or exactly-once supplement evaluation a parity gate.
A different pure preview implementation must preserve selector outcomes, final
resolved supplement, error propagation and hidden-parent effects.

## Selector supplement boundary

`selector-supplement.json` adds four paired cases (54 total scoped cases across
this audit). Oracle selectors match both a default synthesized supplement
`[Table]` and callback-resolved `[Custom]`; product returns no matches. A pure
kind-only preview therefore cannot claim general synthesized-field selector
parity. Either preserve supplement-dependent selection semantics or explicitly
name it as unresolved scope with unchanged alerts and no broad selector claim.
An explicit supplement callback `[Explicit]` also takes precedence over a matched
show-set supplement `[Changed]`. Preview callback invocation counts remain
unmeasured, but callback-result-dependent matching is observable here.

Counter agent `../counter/counter-probes.json` separately establishes automatic
count events from uncaptioned Figures and hidden original Figures. The earlier
no-caption/hidden panic controls establish lack of history demand, not lack of
count events. Any design must keep event existence independent from whether a
caption/reference/explicit read demands the counter history.

## Constructor ordering and partition refutations

`constructor-validation.json` adds 12 paired acceptance/diagnostic cases. Invalid
kind wins before invalid supplement; invalid supplement wins before invalid
numbering in both argument orders. Supplement and numbering casts precede unknown
argument handling. Supplement Str/None/Auto/Symbol and numbering Func/None are
accepted; these success-only probes establish acceptance, not rendered content.

`grid-order.json` adds three paired source-order cases. Grid header(Table) before
Raw yields table; Raw before header(Table) yields raw. Footer(Table) before Raw
also yields table when the outer Figure replacement avoids later grid layout.
The separated Grid cells/header/footer representation cannot recover this order.
The coordinator selected original-child transport as part of the public proposal;
these are required closure cases, not a silent representation carve-out.

`state-update-fields.json` adds three paired cases and refutes the draft
StateUpdate traversal. Upstream `introspection/state.rs:393–401` marks update
internal; its public fields expose key only. Figure over StateUpdate(Set(Table))
infers image, and Raw following that update wins. The update Value must remain
opaque. Product's missing key in StateUpdate.fields is a separate existing
reflection debt, not a reason to inspect internal state values.

The authoritative inventory `closure-case-classification.json` now records all
72 cases: 67 required closure checks, three explicitly preserved known debts,
and two non-observation negative controls. It supersedes interpreting the earlier
47-case candidate inventory as a universal all-green oracle gate. The independent
counter agent's six cases are additional, not counted in these 72.

## Final scope refinements from review

`grid-footer-validation.json` distinguishes existing validation timing from
semantic order. Direct invalid footer-before-cell errors on both binaries; the
same Grid under `show figure:none` succeeds on both. Calling Grid.fields succeeds
on oracle but errors in product. Therefore the footer-before-body kind fixture
is retained validation-phase debt, not an instruction to relocate Grid
validation. Its measured product missing-kind error masks the precise outcome
after kind reflection is repaired; the implementation receipt must characterize
that outcome and preserve the original observation. Accepted header-before/after
order remains required closure. This supersedes treating all three original Grid
cases as mandatory oracle matches.

`table-target.json` proves that a nonmatching Table root must still descend its
semantic children for supplement target lookup: heading is selected and the
first heading cell wins. Auto inference may terminate at Table because it matches;
the shared exact-kind target visitor cannot terminate unconditionally.

Final authoritative audit inventory: **78 cases**, comprising **71 required
closure**, **5 preserved known debts**, and **2 negative controls**. The counter
agent's six cases remain additional. No productive implementation changed while
these baseline observations were gathered.
