# L0 — compiler/eval/operators/equality
Hash do Código: 6126614c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:7e7373acce2b00247d18b9c5ea1b9d53833275997cd2c304e9d919d2cce10648

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/equality.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/equality.rs:21` — `fn apply_binary`.
- `01_core/src/compiler/eval/operators/equality.rs:91` — `fn value_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:103` — `fn values_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:138` — `fn selector_contains_element`.
- `01_core/src/compiler/eval/operators/equality.rs:163` — `fn selectors_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:203` — `fn counter_keys_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:210` — `type ContentFields =`.
- `01_core/src/compiler/eval/operators/equality.rs:213` — `fn raw_content_fields`.
- `01_core/src/compiler/eval/operators/equality.rs:226` — `fn content_values_eq`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/content/raw.rs:354–361`.
- `foundations/selector.rs:75-81`.
- `foundations/value.rs:295-299`.
- `introspection/counter.rs:243,256`.

## Contract

Own Eq/Neq/In/NotIn, values_eq and value_eq. This is language equality, not
derived Rust PartialEq or ObservationEq. Coerce Int/Float recursively, including
array/dictionary values. Containers preserve their existing key/cardinality
comparison; raw Content uses morph_canon and ignores baked render style while
retaining semantic morphology. Version compares all components with zero padding.
Length equals Relative only with zero relative component; Ratio equals Relative
only with zero absolute component and |rel-ratio| < 1e-9. Other existing families
retain their derived fallback. Neq is the negation of the same relation.

For any comparison involving LocatedContent Some, use the pinned snapshot
contract: compare canonical element function and linguistic field sets excluding
label; recursively compare values. For raw/None counterparts use only their
explicitly projected fields, without filling defaults. Field insertion order,
Location and carrier discriminant do not determine this equality. Different
field sets differ; raw is neither automatically equal nor automatically unequal
to its realized counterpart. Keep raw/raw and None fallback unchanged.

Selector::Element compares canonical Func using current function equality and
the ordered filter sequence by name position and recursive values_eq. Empty
group differs from bare Kind; aliases retain identity; NaN filters remain
nonreflexive. Propagate through nested containers and selector compositions,
preserving antecedent comparison for pairs without Element. Counter keys with
Element use the same recursive selector relation, including field order. Do not
exclude Int/Float-equal pairs by structural hash bucket or shortcut with Arc
identity. Before compares base/end/inclusive recursively; After compares
base/start/inclusive; distinct variants differ, with no document query.

Membership uses this same relation for array elements; string-in-dictionary
tests key existence and string-in-string tests substring presence. NotIn negates
these supported combinations. Unsupported combinations use the canonical
quoted-operator diagnostic. Do not modify public PartialEq/Hash, morph_canon or
the content IR to achieve language equality.

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

Extend selector_contains_element to NativeElement recursively. Two NativeElement
leaves compare actual static native function identity fn_addr_eq, preserving
aliases sharing native and distinguishing homonymous functions. NativeElement
differs from empty-group Element. Change no Func/Value PartialEq-Hash or legacy
branches without new leaf. Counter keys/Value::Selector use same language
relation; exclude no pairs by mechanics hash/name. Test language alias,
differently named Rust alias and different function named cell.

D08A covers only carrier/construction/repr/equality/composition/locatability.
D08B's direct show/body-fields/explicit-implicit preparation/positions/padding/
spans/header-footer/lexical scope requires measured own L0/gate for public-phase
change; D08A approval authorizes none of it. Hide no negative or claim universal
parity. Atomic approved set, language tests/valid RED before code/static safe
Rust,1:1/V26 before manifest-owner-only reseal; build/core/lint without errors.
Revalidate bare/where/alias/equality true-false/direct-composed query-counter/
homonym/partial-closure, preserving D05/D07/query.

## sRGB language equality after component preservation

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/limites-cor-2026-10-13/equality-before.json.gz`
retains sources/L0/UTC/status/diffstat/binary hashes. Historical pin9f2b6e871
compares rgb(50%,0%,0%) with rgb("#800000") equal, including arrays/dictionaries/
in-not-in/alpha. Tekt differs after correctly preserving f32. Distinct-byte,
LinearRgb/different-space controls stay false. Upstream visualize/color.rs:2064–2083
compares four Rgb u8 components; :1780–1783 uses `(x * 255).round() as u8`, not
hex ties-even.90.5/255 witness keeps to-hex #5a0000 but equals rgb(91,0,0),
not rgb(90,0,0). equality-tie-before.json retained in evidence.json.gz.

### Exclusive projection and acceptance

==/!=/membership are language observables unlike structural Rust equality.
Continuous internal ADR-0127: change no public signature/PartialEq-Hash/components/
serialization/phase/default. Refine only Srgb/Srgb Value::Color pair in values_eq:
compare r/g/b/alpha after measured u8 projection, ties-away-from-zero round, not
to_srgb/to_hex ties-even. Existing recursion propagates to containers/operators.
Keep other spaces/heterogeneous pairs/structural identities. Tests first:
RGB/alpha/tie positives; channel/alpha/different-space/LinearRgb negatives;
==/!=/in/not-in/arrays/dictionaries. Show structural PartialEq may stay false.
RED/GREEN/build/seals/bilateral probes without normalization. Certify neither
Luma/spot nor all equality consumers.

## Styled child equality in this operator

### Evidence before decision

`lab/results/construction/diagnostics/text-styled-equality-2026-10-17/before-corrected-measurement.json.gz`
records24 pairs/48 invocations/17 divergences, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/
diffstat/source-binary hashes. Historical pin9f2b6e871 content/mod.rs:779–782
compares StyledElem child only; :376–385 coalesces adjacent Styled on nonempty
map. Tekt morph_canon then structural PartialEq still observes Styles. Complete
text-size transport exposes two true→false regressions, relative-zero/affine-
absolute-part, while pin stays true. Declared-value equality cannot be inferred.
Initial corpus had two nested bodies as literal markup and missing calc.nan,
invalid RED for those cases. Corrected run uses call expressions/float("nan"),
all24 execute.

### Current private comparison projection

Only raw Content paths in Eq/Neq, recursive values_eq and raw/None fallback in
content_values_eq use private content_language_eq. First keep existing morph_canon;
then create ephemeral comparison-only projection: Styled retains variant/child,
coalesces only directly adjacent Styled and replaces only its map with empty
Styles. Other nodes retained completely with bottom-up visited children.
Passthrough returns whole node, no neutral default/V16 suppression/Arc identity.

Never render/store in Value-Engine/publish as morph_canon/use for Hash-ObservationEq-
termination this empty projection. No second size/fake scalar/all-wrapper erasure.
Do not morph_canon it again: wrapper versus plain stays distinct. Strong/Emph/
Heading/body/field distinctions retain identity. Rust PartialEq/public morph_canon
stay; structural Styles comparison elsewhere allowed. Snapshot Some/fields/labels/
other-value coercions keep prior contract.

Propagate relation to arrays/dicts/membership/!=. Ignoring discarded style NaN
never makes Float NaN reflexive. Preserve first Source origin; evaluate no
argument/body/style function. Internal continuous ADR-0127, no new signature/
field/Rust compatibility/product-default/phase. Historical custom-only morph_canon
transparency is neither changed nor universally certified.

### Acceptance and limits

Six functional tests first: styles versus structural equality, wrapper coalescing,
nominal-node/Sequence children, containers/operators, negatives, Source/NaN.
Require RED→GREEN/all24 matching pairs/no formerly matching regression. Revalidate
text-size corpus/Par-Box/RG1 preserving intentional equality corrections. Full
suite/build/Clippy -D warnings/-D unfulfilled_lint_expectations/Rustdoc -D warnings/
four indexes/fmt/diff/strict-reverse lineage/differential SARIF under 12 GiB/
MemoryHigh 10 GiB/zero swap/jobs1. No commit/push/merge/release; not all Content
certified resolved.
