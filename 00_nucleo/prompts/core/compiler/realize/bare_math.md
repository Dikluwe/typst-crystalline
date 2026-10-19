# L0 — Mathematical insertion preparation
Hash do Código: 33f29e69

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b

**Layer:** L1.
**Exclusive consumer:** `01_core/src/compiler/realize/bare_math.rs`.
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence and ratification

Historical measurement at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
uncommitted working tree, is preserved with UTC/diff/state/sources/binaries in
`lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz`
and `lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md`.
Its vanilla pin is `9f2b6e8715237cb086899a42873660fe744622e8`, not the current
upstream target. Bare frac, mat, sqrt, lr and op produce one Equation in that
vanilla snapshot and zero in Tekt; math.bold([x]) and ordinary text produce zero
in both. nested/nested-equation produce one in both; reuse produces two in
vanilla. block-style/local-block/number-style locate insertion scope;
show-generated/context-generated expose the same gaps. Explicit Equation
preserves fraction rendering; vanilla rejects it.body with
`maximum show rule depth exceeded` while accepting it; a later show affects only
the second insertion. Refuters cover a raw frac value and original label versus
explicit Equation.

`lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334` transforms
Mathy before show outside math. Historical local `compiler/layout/equation.rs:680–689`
projects bare math to text; `compiler/realize/mod.rs:155–199` prepares show/identity
without math normalization; `entities/content/traversal.rs:14–108` descends
bottom-up into math bodies; `rules/apply.rs:564–607` handles only existing
locatables. These are language semantics, morphology and render observations;
they do not require vtables, buffers, Rust equality or equal PDF bytes.

The human-confirmation gate for the math-bare package is satisfied by
`lab/history/construction/records/00_nucleo/materialization/math-bare-aprovacao-2026-10-06.json`
(manifest `149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6`),
as retained by `00_nucleo/prompts/core/compiler/realize/mod.md`. The historical
proposal-only heading is superseded. Ratification changes phase restrictions
only at the named insertion boundaries; it does not certify general parity,
D07, math labels or other debt.

## Exclusive contract

Own pure mathematical insertion normalization as a descendant of compiler/realize,
with no reverse entities→compiler imports. Expose only this compiler-internal
helper, preserving its existing signature:

```rust
pub(in crate::compiler) fn prepare(
    content: &crate::entities::content::Content,
    styles: &crate::entities::style_chain::StyleChain,
) -> crate::entities::source_result::SourceResult<crate::entities::content::Content>;
```

Traverse boundaries before children using the traversal owner's internal
preorder helper. An existing Equation is a math boundary: retain its body without
adding another Equation. For a supported bare Mathy root, wrap that root exactly
once with Content::equation(original_body, effective_block), without normalizing
its descendants. block comes from insertion-scope Boolean equation.block,
default false; numbering, number-align, supplement and alt retain existing style
transport without new public fields. Do not allocate Location or call Func;
identity and rule application belong to the existing coordinators.

Use closed enum-variant classification and the measured pinned Mathy catalog,
not a Math-prefixed name heuristic. Structural families include Frac, Matrix,
Root, Lr, Mid, Attach, Primes, Stretch, Op, Cases, Vec, Accent, Cancel and their
implemented concrete carriers. Confirm each entry by file:line and the relevant
test before materialization; a similar name cannot establish acceptance.
MathIdent/MathText alone are not upstream Mathy. MathSequence is traversable
flow, not one synthetic Equation around everything. MathStyled is transparent
style: isolated text remains text and an external Mathy descendant becomes
Equation. ClassOverride/LimitsOverride represent the source's Mathy wrappers
where those variants actually exist.

The verified historical mathy-catalog.json records each pinned upstream
file:line. MathUnderover is a legacy extension from native_underover
(`stdlib/structural/math/underover.rs:19`) without a corresponding upstream Mathy
element. This supersedes its initial generic catalog inclusion: preserve its
existing bare fallback and do not certify extension parity by assuming Mathy.
MathDelimited is the legacy Lr carrier; class/limits wrappers correspond to
ClassElem/LimitsElem/ScriptsElem.

Styled retains its wrapper and descends with the overlaid chain. ShowRecipe uses
its own lexical snapshot and retains rules/guards/file/textual/context_id_floor.
Realized retains its original untouched and processes only unprepared visual
content. Prepared Located retains its Location and is not a new insertion.
Labels remain on the original element and are not promoted to the new Equation.
Sequence, ordinary-content containers and contextual/show contributions visit
actual insertions in order while preserving mathematical regions.

Do not change constructors, repr or raw Value. Do not merge adjacent/equal Mathy
roots into one Equation or wrap ordinary content merely for having font/math
style. Preparation is idempotent: an already prepared tree gains no equations.

## Root projection for causal coordination

`bare_math.rs:14–44` owns the nominal Mathy catalog; `:78–95` prepares a Mathy
root and subsequently descends through containers. Hidden-child failures T38/T50
in `lab/results/construction/diagnostics/r3-t1-materializacao-2026-10-18/acceptance-red.log`
establish the causal order defect, not Mathy classification. The approved causal
realization package requires root-before-children and Mathy preparation before
its own insertion selection. before-sources.json.gz preserves the pre-extraction
owner/source. This is not a new parity measurement.

Expose only within compiler `prepare_root(&Content, &StyleChain) -> Option<Content>`:
consult the existing catalog and produce the same effective-block Equation only
when the root itself is Mathy. Do not inspect children, invoke callbacks or
allocate Location. prepare reuses this root operation in its Mathy arm while
preserving all other boundaries, shortcuts and signatures. The causal coordinator
`00_nucleo/prompts/core/compiler/realize/recipes.md` must not duplicate or own the
classifier. This internal extraction adds no external Rust API, dependency, flag
or Mathy criterion. Revalidate existing controls plus Equation/Mathy, context and
emitted-product cases in the broad causal-realization corpus.

## Closed absence proof and qualified performance decision

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/phases-after-reuse.json.gz`
preserves release SHA-256 `5b4b34340a5f637d9e99c0ad3ee9e209c14b22c18c8890b23d344ea0a2778ecf`,
HEAD/uncommitted working tree/status/diffstat/UTC/fixtures/CPU and three runs per
case. For 1,600 cells, local-set evaluation measured 150.42 ms versus no-fill
118.99 ms, layout 90.54 versus 68.47 ms. stacks-after-reuse.json.gz sampled one
of every 64 glibc requests and located styled/with_source_span reconstruction
through map_content, including bare_math::prepare. bare-math-red-gate.json.gz
records 635,978 local requests against a precoded 600,000 limit. bare_math.rs:45–84
always calls map_content_pre and rebuilds Styled/TableCell even for plain text.
A gate failure, paired time or changed output can refute the benefit inference.

Before transforming, recursively prove Mathy's absence only for
Empty/Text/Space/SpaceFromNewline leaves and Source, Styled, Sequence,
ShowRecipe.body, Realized.visual, TableCell.visual, resolved Table cells+caption
or raw Table children+header/footer/caption. Equation and Located retain the
existing preserved-boundary policy. With positive proof return an original-node
clone without visiting/reconstructing children, comparing visuals, executing
rules/callbacks or changing source/scope. Every other carrier runs the full
existing map_content_pre path; unknown variants are never assumed empty.
Mathy bodies, Label with Mathy, MathStyled and all unlisted math keep the existing
path. This is a same-owner/phase internal cost change, with no new public
contract, default, unsafe code or I/O (ADR-0127).

The subsequent bare-math-{red,green}-gate.json.gz,
measurements-{local-reuse,bare-math}.json.gz and validation-bare-math.json.gz in
that diagnostic directory preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
uncommitted state/diffstat, UTC, commands, sources, fixtures and binary SHA-256.
The glibc gate **failed**: 635,978→607,135, a reduction of 28,843 but still 7,135
above 600,000. Paired CLI median improved 262.72→249.00 ms; excess over no-fill
50.16→40.10 ms; paired local Tekt/vanilla overhead +53.6%→+45.8%; no-fill
212.56→208.90 ms. Different windows have host variation. Allocation reduction
and the joint direction support but do not precisely isolate the benefit.
The 23 fixtures had no differences in text/pages/boxes/raster/warnings.

Retain the optimization subject to correctness/build qualification because the
primary search criterion is reduced excess time with equivalent language/render.
Keep 600,000 explicitly as an unmet quantitative hypothesis; never relabel the
failed gate GREEN or treat the failure as a regression when requests decreased.
Revisit residual allocations in a separate experiment; remove this implementation
if correctness or build gates fail. Acceptance retains RED→GREEN Arc-structure
reuse controls and actual Mathy transformation in cells/caption/recipes, paired
local/no-fill/vanilla time, and the 23 observable fixture checks. Remove if excess
time does not decrease; run suites/build/lint and restricted owner resealing only
after qualification. Arc identity is engineering evidence, not language parity.

## Context capture boundary and superseded selector maintenance

The current evidence is before.json.gz under
`lab/results/construction/diagnostics/context-paridade-2026-10-18`, HEAD
`b7e7c83223e5daa67bbdf124c36b5b7f93200731`, vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, with sources, UTC, argv and identities.
Global/local textual probes return A/A versus R/L. captures_equation_rule had
only capture_context_recipe in the hub as a productive caller. Its Equation/Table
whitelist prevents valid rule transport; general lexical transport makes it
unused (green-text1.log).

The ADR-0127 internal correction supersedes only the historical restricted
context-capture boundary and maintenance clauses for that filter. Remove
captures_equation_rule and the unit test freezing its obsolete whitelist;
functional Context tests owned by `00_nucleo/prompts/infra/pipeline/tests.md`
replace that criterion. Capture preserves rules without claiming they match
any element. Preserve selector recognition/matching, the Mathy catalog,
Table/Cell preparation, aliases, wrappers and public interfaces. Silence no lint
and weaken no language expectation.

The prior selector-filter maintenance measurement, its exhaustive nominal
alternatives, native-equation identity rule, S3 controls, V16/V19 policy and
structural sentinel acceptance are preserved in
`lab/results/construction/diagnostics/tekt-v16-show-selectors-2026-10-16/{before.json.gz,classification.json}`
and the archived original. They no longer authorize retaining/recreating the
removed whitelist. Their general limits still apply to surviving selector work:
no name-based identity, no implicit evaluation of filters/conjunctions during
conservative capture, no duplicated bodies to eliminate informational V19, and
no allow/expect, severity change or generic helper to hide V16. A shared-body
execution does not prove all alternatives; guards/recursion need controls.

## Acceptance and scope

Require scoped RED→GREEN and bilateral revalidation of ratified insertion
behavior. Preserve the compiler-internal helpers and all owner boundaries;
introduce no public signature/field/trait, product default, compatibility or
phase changes beyond the ratified insertion package. Do not reopen historical
proposals through this owner. L0 precedes tests/code and restricted bilateral
resealing. Run real tests, build, format, strict Clippy/Rustdoc and direct/reverse
lineage under 12 GiB, zero swap and one Cargo job. Do not claim the global gate
is green while external V16 findings remain. Historical sequence, superseded
filter-specific structural tests and approvals are recoverable from
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/realize/bare_math.md`.
