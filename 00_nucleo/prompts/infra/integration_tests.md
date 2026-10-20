# L0 — Infrastructure end-to-end test harness
Hash do Código: ddd4bcd5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3 (cfg(test) only)
**Exclusive consumer:** `03_infra/src/integration_tests.rs`

## Current evidence

- `03_infra/src/integration_tests.rs` — the test-only module and compile/eval/frame helpers.
- `03_infra/src/integration_tests.rs:5203` — inverted-bracket witness from `lab/corpus/legacy-sections/sec_22.typ`.
- `03_infra/src/integration_tests.rs` — bundle feature-gate and active-target cases.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only this end-to-end harness. Exercise SystemWorld, temporary filesystem, evaluation, introspection, layout and export across the real infrastructure boundaries. Helpers remain test-only/private and remove their temporary resources. Productive modules keep their individual owners; test setup cannot move I/O or concrete World access into L1.

Assert language values, morphology, diagnostics/warnings and observable exported structures. Byte/internal-structure identity is acceptance only when explicitly selected as the observable, such as the independently owned snapshot corpus.

Geometric helpers accumulate ancestor affine transforms: Group translation, matrix and inherited transform compose in exporter order; nested groups compose transitively. Project Text, TextShaped, Glyph, Image, Shape and Line endpoints once into global coordinates. Semantic is transparent and Link keeps its existing convention. Counting/classification may inspect local variants, but geometric assertions use global coordinates. Do not compensate an observer bug by changing expected geometry, fixtures or productive layout.

For the pinned no-text placement witnesses, check an empty page's requested PDF MediaBox, preserved cover-image clip and placed-text baseline against the independently observed vanilla geometry. Run the real World and production pipeline for font-demand controls. A no-font declaration with an isolated shape or image must not request the font book; a placed nontext body must meet the same condition without suppressing explicit font-family validation or text shaping. These assertions belong to this test harness; the productive owners remain responsible for their mechanisms. Record RED failures without weakening expectations to match current output.

Use public color values (`red`, `blue`, `black`) in shape/rotation fixtures, not strings that trigger cast errors. `lab/corpus/legacy-sections/sec_22.typ` contains normal and inverted bracket pairs; observe base character, glyph identity and advance and require equivalent vertical variants when positional role changes. That witness protects `00_nucleo/prompts/core/compiler/math/layout/delimited.md` without requiring a particular MathDelimited representation.

Bundle tests distinguish feature absence, out-of-target calls and the active target. Absent bundle feature must fail during eval with its exact gated diagnostic/hints, before layout/export. Active tests cover byte/UTF-8 assets, nested/normalized paths, duplicate asset/document paths, empty bundles, mixed PDF/SVG/PNG/HTML document entries, explicit/unknown formats, HTML without its feature and loose top-level content. Asset names/payloads are exact observables; document acceptance uses language/format structure. Do not accept an empty placeholder asset as data or leave outputs outside isolated temporary directories.

## Verification boundary

Retain existing integration cases and their owner boundaries; this prompt does not assert a current full-suite result or authorize productive changes to accommodate observer mechanics.

## Carrier acceptance

`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` source→PDF RED/handler audit preceded layout_types public approval 2026-09-28. Adapt fixture projections/exhaustive traversals for TextSeparator: position transported, neutral point-count/ink/current behavioral measurements. identidade-hifen-discricionario-2026-09-28 initial/before/approved-before eight U+002D/SHY equal-bbox controls and owner confirmation require transparent TextReplacement child traversal, unchanged text/order/coordinates/calculation, recursive/literal-hyphen/text-box controls, no new advance/algorithm/byte/new-selection criterion. bidi-contexto-2026-10-05/inventory.json identifies this owner; confirmed manifesto1ae7513e requires None for uncertified constructors/read-pattern `..`, transformations obey pinned origin without phase/API expansion.

## Hyphenation and paragraph scope: evidence before boundary criteria

correcao-politica-hifenizacao-2026-09-28/first-infra-tests.log records 1117 pass/five fail: three canaries expected discarded hyphenate warnings, two en/pt pattern tests set only lang and presumed default hyphenation.52 bilateral probes/pinned text/mod.rs:545–565 refute it:auto depends on justify. before/initial.json hashes preserve unchanged source/L0. Update simple/repeated canary to stroke:1pt, triple to overhang:true, baseline:3pt, stroke:1pt, preserving counts/distinct spans/property-target. Positive en/pt tests explicitly hyphenate:true and still require a hyphen; add false/true/auto/absent versus justify control. These measured input changes preserve purpose, no snapshot/tolerance relaxation.

correcao-composicao-paragrafos-2026-09-28/before-pdf has 34 historical9f2b6e8715237cb086899a42873660fe744622e8 controls with initial/before state.180pt/margin8: first justified line ends145.52versus172.000002pt, last110.048both; BEFORE/par[MIDDLE]/AFTER two physical lines versus three; empty-start AFTERy3.74versus18.14; empty-auto24.568versus38.968. native-leading-two-lines requires leading scope close before style restoration. line_commit.rs:195–236 already justifies, word_layout flushes ordinary breaks, layout/mod.rs:1469 treats Par transparent, style_scope.rs:18–77 restores after body, eval/rules.rs:663–705 groups show flow. XML groups are not physical lines: measure coincident coordinates separately.

Canonical paragraph owners retain internal continuous parity/no public entity/signature/trait/flag/phase change. This harness adds native par before/inside/after, empty-start/auto, continued-line justification/natural final width, native override/pagination controls. Measure positions/text, not page count; derive expectations from oracle or independent FixedMetrics, never candidate/tolerances/snapshots. Global preparation/optimized were explicitly incomplete until separate measured selectors/costs work; this slice alone proves neither costs nor optimized.

precedence-state.json/regression-acceptance.json records 191 checks/two failures(show-par-sets-false): local extraordi-/interna- versus unhyphenated upstream. show-par-full-tree.log puts SetPage/global scope inside synthesized Par with recipe outside text.hyphenate=true; eval/mod.rs:1532–1560 has actual #set provenance, rules.rs:660–727 erased it by treating all Styled inline. Boundary requirement: eval_markup fold transports private eval.flow.scope=true in existing Styles; realize_flow recognizes it before first material paragraph and keeps #set scope outside paragraph realization, preserving SetPage/Empty/Space/SpaceFromNewline in order without Par wrapping. After inline material it stays inline; no style-key provenance inference. Eval hub produces marker only; rules owns preparation/recognition, shared paragraph-composition nucleus owns invariant. No new entity/API/default/phase.

precedence-pdf/resultados.json/proveniencia.json eight prior-build controls refute blanket constructor-inline exclusion: native-text-inline-override/mid-par-set-override hyphenate both(with separate -/SHY identity gap), native-text-only-override only local hyphenates. realize_flow also factors an all-material Styled wrapper whose prefix/suffix contain only Empty/Space/SpaceFromNewline/SetPage, retaining carrier order and realizing body under exterior scope. Fragments with text outside wrapper stay inline/no new boundaries. This narrow whole-flow rule supersedes only absolute constructor exclusion, not public contracts.

correcao-hifenizacao-global-2026-09-28/infra-verified.log shows en/pt/lexical-precedence failures: integration_tests.rs:3657 only counts multi-character Text ending'-', while paragraph_emit.rs:68–73 preserves fragments and paragraph_hyphen splits prefix/hyphen. Source→PDF matches words/positions66of67; this is observer failure, not permission to reduce language expectation. build_doc/FixedMetrics helper accepts same-Text word-hyphen or adjacent alphabetic prefix+separate hyphen on same baseline/page. Derive adjacency from FixedMetrics with existing global Group projection, not real-font guesses/emission order. Keep inputs/options/lang expectations; negative isolated/spaced/other-line hyphens and positive contiguous test. No productive/snapshot change.

## Costs, regional protection and historical canary supersession

correcao-hardcode-paragrafo-2026-09-28/before-pdf20 controls at historical9f2b6e8715237cb086899a42873660fe744622e8/initial/before show eval/rules.rs:3483–3525 and text/constructor.rs:153–190 omit overhang/accept auto-int-string; pinned text/mod.rs:391–393 defines bool default true, line.rs:518–525 uses final style. paragraph_hyphen.rs:221–224 always multiplies0.55. Initial suspicion about paragraph_select.rs:78–82 was refuted: upstream linebreak.rs:551–559 uses attempt.justify, disabled on mandatory at line.rs:138–139. Preserve formula/natural-last-line test. Normative0.55/base135/distance5/increment0.15 are named by meaning/source, not ignored configuration repaired by renaming constants. No inferred intent; literal-hyphen identity originally required separate carrier, not glyph/position heuristics.

The intermediate debt49 replacement overhang→costs:(hyphenation:100%) was superseded once costs became supported. Current canary uses kerning:false(known scope-out) instead of captured costs, retaining three distinct warnings/spans/baseline/stroke/all other expectations, no unsupported overhang warning. This preserves both partial supersessions without reactivating obsolete fixtures.

correcao-custos-paragrafo-2026-09-28/before-pdf66 controls/initial/before at historical pin: text/mod.rs:1507–1564 validates Dictionary/Ratio/component partial composition, inline/mod.rs:240 reads shared scope, linebreak.rs:29–30,961–962 uses135/100×ratio. Tekt discards costs in eval/rules/constructor and fixes defaults in paragraph_hyphen/select; upstream stderr preserves errors/spans/accepted negatives. Canonical `00_nucleo/prompts/core/compiler/text_costs.md` owns casting/component transport/shared resolution; hyphenation/runt consult global config, widow/orphan validate/transport with influence only in certified regional subset below. Preserve defaults, variable-width limits, no glyph-inferred cost; partial-fragment costs never replace shared scope.

after-first-resultados.json/proveniencia.json66 entries shows only restore/inline diverging among valid inputs; shared scope read and integral Styled factoring already exist, but paragraph_prepare.rs:45–78 rejects channels neutral to glyphs and falls back inconsistently. Admit only text.costs.hyphenation/runt/widow/orphan into preparation Styled whitelist. Preparation's temporary push restores and never changes shared planner read; partial inline cannot select paragraph costs, whole wrapper factors before Par/chain entry. No costs-created space/word boundary or promoted partial scope/phase change. Keep regional exclusions.

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/{before-pdf/,before-extra/}` has 78+24 controls/full initial/before state at historical pin: page-65-100%-0%-False3+1 lines local versus2+2 oracle, zero costs match, tiny/large-positive same protection. flow/collect.rs:196–229 uses strict-positive first/last two and entire three-line groups; distribute.rs:343–364 advances only if group fits next region, while local emit goes linewise. Language line/word distribution, not upstream frames/Child mechanics or inferred old intent.

Regional boundary is only prepared paragraphs with uniform vertical metrics, no baseline offset, constant page geometry, no columns/cells/subframes/notes/floats/active regional transaction; other paths retain fallback/explicit incompleteness, no full algorithm/performance claim. Canonical paragraph_pagination owns certification/grouping/admission only: receives selected ranges, no reselection/glyph emission/new_page duplication. Certify whole input before output: all fragments share exterior size/edges/leading(bold/italic only if metrics equal), finite nonnegative top/descent/leading. Auto bounds need no protection. Account initial baseline/block-chain collapse before page_bottom_limit; next space uses resolved next-page margins. Advance only with current material and group fits next region; too-high groups retain ordinary progress/all content, no new_page loop. text_costs helpers reuse FIELDS/defaultRatio1/strict>0 independently of hyphenation/runt. Ordinary no-linebreak planning default is Simple to enable default protection, preserving justify-based auto/refused contexts. Keep justification/carriers/open-last-line/boundaries/lexical restore; preserve notes/columns by refusing incomplete protection.

Acceptance RED→GREEN preserves same words2+2versus3+1, entire three-line group, independent orphan-off, oversize, restore/default-no-flag; bilateral boxes/words/pages/prior types-spans/fallback. XML grouping is not line count. Restricted owner lineage/build/lint zero; no new public field/trait/API/flag/phase. These are cross-owner boundary criteria, not productive authorization from the harness.

## Ratified bare-math integration

math-bare-2026-10-06/before/additional/refuters and math-bare-medicao-2026-10-06.md preserve HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty tree/UTC/diff/source/binaries/historical9f2b6e8715237cb086899a42873660fe744622e8. frac/mat/sqrt/lr/op bare Equation1upstream/0local; bold ordinary 0both, nested/explicit1both, reuse2upstream; local block/number styles and show/context production locate scope. Explicit Equation preserves fraction; it.body depth error versus valid it/later show only second insertion, raw frac/label refuters. Upstream realize/lib.rs:245–265, 302–334 inserts Mathy before show outside math; local equation.rs:680–689 text-projects, realize:155–199 lacks normalization, traversal14–108 bottom-up, apply564–607 only existing locatables. These are language morphology/semantics/render, not Rust/vtable/byte requirements.

Human ratified `lab/history/construction/records/00_nucleo/materialization/math-bare-2026-10-06.md:3` plus math-bare-aprovacao-2026-10-06.json, manifest 149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6. Earlier pending gate satisfied only at named insertion boundaries; no universal parity/math-label/other-debt closure. Real World paged/query tests cover bare fraction versus Equation/same visible geometry+Formula, frac/mat/root, local block, numbering/alt, textual-it-generated show, context/captured-show, alias reuse/nonmath controls. Expectations come from additional/refuters, not candidate/PDF bytes. Never alter fonts/tolerances/snapshots; PDF text alone cannot prove drawn fraction, measure number positions/content bbox. Keep independent negatives for mat morphology, original Mathy labels, broad HTML/unmeasured math catalog, never matches/hiding certified failures. Reexecute D05(29)/compiled-query(36), L1/L3 suites/build/lint/full provenance; scoped bilateral RED→GREEN mandatory.

## Test-only cleanup and native tiling fixture

infra-imports-docs-ci-2026-10-13/before.json.gz preserves line32/compiler spans/source/L0/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty status/diffstat/UTC/use audit. Remove only proven unused private imports, keep required public/contract/call imports, unchanged names/assertions/cases; all-target/infra suite/build/workspace Clippy/Rustdoc/lineage, no suppression/import-comment mirror tests/public/default/phase/formula change.

integration_tests.rs:226,237,3570 examples and compiler spans identify63 unread introspect(content) calls immediately before layout(content); introspect.rs:135/layout/mod.rs:1703 call the same internal introspect_with_introspector. Remove only verified redundant calls, not underscore substitutions, retaining all used calls/assertions/fixtures/names/internal layout pass/no performance claim. Intermediate direct search finds _state calls831, 1708(initial863,1749), also unused before layout; fixtures-ativos-ci-2026-10-13/refinement-before.json.gz preserves state/UTC. Remove these two too and now-unused introspect import, keep used introspect_with_introspector. Total65; preserve initial-validation, repeat final acceptance on refined state.

ci-simplificacoes-2026-10-14/before.json.gz/red.log preserves source/owner/spans/HEAD/tree/UTC/diffstat/pinned Rust1.92.0 type audit/RED at1503, 1532, 1554, 1558. Four JPEG fixture writes pass byte arrays directly to std::fs::write AsRef<[u8]>, same bytes/paths/I/O order/expectations/minimal fixture, no new JPEG-decoder certification. Continuous internal flow/L0 first/scoped lineage/selected RED→GREEN/existing tolerances/full build/suites/all-target Clippy/fmt/Rustdoc/lineage, no syntax-mirror tests.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

workspace.log fails old Color-without-size fallback fixture at1686. Valid Color tiling requires explicit size, no errors, PatternType1/scn plus rectangle/red within cell. Preserve original missing-size case as preparer's causal diagnostic test. No invalid-output acceptance through absent color/no integration coverage removal.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Equivalent expressions and a complete PDF observer

clippy-expressions-2026-10-16/before.json.gz/initial-diagnostics.json preserve source/L0/HEAD/tree/status/diffstat/UTC/suggestions/spans/type-use audit: needless_lifetimes:43, collapsible_else_if:1270, redundant_guards:4803, 4808, useless_vec:5832, 5977. Elide project_item's sole lifetime preserving borrow/projection; Glyph patterns absorb x/𝑥 and y/𝑦; RGBA arrays.repeat(4) preserve PNG bytes/size/expected. No new parity/performance. The observer repair is functional rather than merely flattening else:

parse_pdf_objects:1273 consumes every'<' as hex before:1275 can recognize'<<', making depth increment unreachable; :1285–1286 inserts through i−2 even after EOF with positive depth. This may omit fields/accept truncated observations, not evidence of produced-PDF defect. Private helper remains limited to `id 0 obj <<`; test'<<' before hex'<' with symmetric'>>' depth and existing literal/escape protection; require full closure before insertion. Truncation explicitly fails, no partial acceptance/general PDF parser/stream-catalog-outline/production expectation change. Independent RED tests: known nested dictionary+literal'>>'/escape+postchild fields remains whole; unclosed object fails; flat/subsequent objects remain visible. Then GREEN/real outline regressions/full workspace/build/all-target Clippy/expectations/fmt/Rustdoc/lineage, unchanged fixtures/tolerances, no syntax tests.

## Exhaustive existing FrameItem decisions


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

- walk (historical F058): replace only `_` with `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, using existing `FrameItem` prefix.
- texto_do_doc (historical F059): replace only `_` with `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`, using existing `FrameItem` prefix.
- p1132b_stack_heading_e_stack_seguinte_paridade_sec43 (historical F060): replace only `_` with `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`, using existing `typst_core::entities::layout_types::FrameItem` prefix.
- p1132o_sec18_fracao_aninhada_suprime_spacing_em_script (historical F061): replace only `_` with `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`, using existing `typst_core::entities::layout_types::FrameItem` prefix.
- items_de (historical F062): replace only `_` with `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`, using existing `FI` prefix.
- items_de (historical F063): replace only `_` with `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`, using existing `FI` prefix.
- p975_pipeline_preserva_ic_no_posicionamento (historical F064): replace only `_` with `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`, using existing `FI` prefix.
At p1132b... guards and Glyph base_char'x'|'𝑥', 'y'|'𝑦' restrict earlier arms; p1132o... Glyph'+' and guards likewise; p975... has guards. Keep all failing-condition cases in nominal fallback, never treat restricted fields/guards as whole-variant coverage.
Preserve complete fallback bodies/prior arms/guards/transforms/calls/returns/order.
Callbacks/effects/metadata/clipping/fonts/positions/ink/width follow current owners;
no expanded traversal/wrapper-leaf change/text-or-ActualText geometry replacement.
Tests retain fixtures/expectations/tolerances/counts/Unknown failures; V19 group
notes stay visible without pretending one arm covers all alternatives.

Continuous ADR-0127 adds no enum/field/trait method/public signature/default/
dependency/compatibility/phase and approves no historical pending proposal.
L0 first, scoped bidirectional resealing; no suppression/higher threshold or
match→if-let evasion. Before code run new FrameVisitor control/workspace suite
with unchanged expectations and confirm AST RED for all 70 family warnings.
Afterward zero scoped-family V16/no new warnings; outside V16 remains pending.
For productive core matches reached by cargo check -p typst-core, use a disposable
FrameItem sentinel variant to observe absent E0004 before/present after. That
probe covers only compiled sites, never unbuilt cfg(test) or blocked L3 sites.
For the full batch replay patterns/guards/bodies, verify enum/signatures unchanged,
real build/tests, no stub bodies.

Revalidate full workspace suite/cargo build/strict workspace-all-target Clippy
(-D warnings, -D unfulfilled_lint_expectations)/fmt/Rustdoc -D warnings/four HTML
indexes/strict V1/V5/V15/V26/reverse lineage/SARIF/diff --check. Freeze sources/L0/
configuration during runs; heavy commands sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1. Structural cleanup is not new global parity,
memory gain or measured AI benefit.

## Narrow floating-point ordering observation

mixed-style-metrics-2026-10-18/initial-workspace.log records sole infra-lib RED stress_81_5_row_height_e_maximo_da_linha:41.449999999999996>=41.45 at2212. Difference7.105427357601002e-15pt; magnitude-relative f64 epsilon9.203748874142547e-15pt. rounding-before preserves source/L0/UTC/exact values, other Grid controls pass. This is geometric ordering, not binary-arithmetic identity or proof of row/height/break bug.

Only this vertical-order inequality permits signed difference bounded by f64::EPSILON*max(abs(y_linha1),abs(y_max_linha0),1), with both coordinates finite. No document rounding/fixture/item observer/expected position/layout/snapshot/other tolerance change. This narrowly supersedes blanket tolerance-preservation only here; larger geometry error must fail. Parity0.001pt tolerance is unrelated/unchanged. Private ADR-0127/0107 observer correction, recorded RED/L0 first/scoped lineage/wide final-freeze revalidation.

## Fixture relocation

First postmigration compile failed at12 literal include_str! sites because root.typ directory retired; exact failure/freeze in lab/results/mature-layout-2026-10-18/attempt-1/. Use lab/corpus/legacy-sections/sec_*.typ for ten versioned inputs, bytes identical to retained construction fixtures. Change include paths only, keeping all inputs/asserts/counts/tolerances/diagnostics. Test relocation imports no lab Rust/authorizes no product behavior.

## Bounded equation observation

### Evidence before classification

At checkpoint `059d1a0ffa9523dbe21af6860a215acaeef55d73`,
`03_infra/src/integration_tests.rs:4861-4907` obtains the evaluated module for
`lab/corpus/legacy-sections/sec_27.typ`, collects equation bodies through only
Sequence, MathSequence and Styled, then checks four bodies and the third body's
ascent against 11.4686pt with the existing 0.001pt tolerance. Equation is terminal
for this observer: its body is collected once rather than searched for more
equations. The fixture's lines 13-18 contain a heading followed by four top-level
display equations. This is source evidence, not a new runtime shape measurement.

`01_core/src/entities/content/traversal.rs:14-39` separately projects Source,
ShowRecipe and Located through body and Realized through visual for its general
static traversal. That broader traversal does not establish the scope of this
fixture-specific observer. `01_core/src/entities/content/identity.rs:175-180`
hides those carriers in Debug formatting; plain Debug output cannot prove their
absence. Neither source establishes a productive math defect or a current-pin
vanilla geometry measurement.

### Measured fixture boundary

`lab/results/v16-corrections-2026-10-19/batch-b/preservation-before/state.json.gz`
records the uncommitted working tree at checkpoint
`059d1a0ffa9523dbe21af6860a215acaeef55d73`, UTC
`2026-10-19T11:45:55.229047+00:00`, exact changed-file stat and input hashes;
`freeze.json` confirms no input changed during measurement. `new-infra.log`
completed at `2026-10-19T11:48:01.666719+00:00`: the explicit diagnostic projection
found the four required equation roots through Sequence and two Styled bodies,
at final sequence positions 17, 19, 21 and 23. No causal carrier occurs on these
traversed equation paths. Semantic leaves were opaque; this is not evidence that
the whole tree lacks carriers or that a diagnostic `opaque:sequence` name is a
literal Sequence variant. The unchanged real-font geometry case also passed,
recorded in `sec27-geometry.log` at `2026-10-19T11:48:02.426558+00:00`.

The instrumented projection and its logs are retained with
`lab/results/v16-corrections-2026-10-19/batch-b/preimplementation.patch.gz`.
It is a one-time measurement, not a permanent golden topology test. The evidence
refutes a missing required equation in this fixture and does not justify adding
carrier descent. Keep the bounded observer's behavior and close its nominal
domain. A future fixture whose required equation is hidden behind an unsupported
path would reopen this boundary with source and runtime evidence.

### Contract and acceptance

Keep this private, test-only observer bounded to its existing fixture scope.
Extracting its unchanged decision into a private helper is permitted so controls
exercise the same observer as the real-font integration test. Measure the actual
fixture's carrier/sequence/style paths before deciding whether a carrier repair
is necessary; distinguish this evidence from synthetic inputs. Do not adopt the
general static child traversal or search arbitrary semantic containers to make
a count pass. Absent evidence that the fixture loses a required equation, retain
the current traversal boundary and make its entire Content complement nominal.
In particular, heading, strong, paragraph and other semantic containers remain
opaque even when they contain an equation.

Controls must preserve direct hits, sequence and nested style ordering, empty
input, semantic-container decoys, and terminal Equation behavior without nested
double counting. Record actual fixture root paths with carrier names preserved;
source/original versus visual distinction cannot be inferred from Debug output.
The existing four-equation assertion, equation index, fonts, source bytes,
geometry, and tolerance remain unchanged. Synthetic carrier cases characterize
the existing boundary unless a separately measured obligation authorizes a
repair; they do not define new language admission.

Require the full existing integration case and independent observer controls,
plus an attributable enum-growth E0004 at this exact cfg(test) decision in a
disposable actual workspace. Baseline wildcard acceptance and candidate
exhaustiveness must be recorded separately; unrelated compiler errors are not
proof. If adding a Content sentinel blocks dependency metadata before this L3
test compiles, omit one current observer arm in the disposable workspace as a
paired domain-closure witness: baseline wildcard absorbs it, exhaustive
candidate fails at this decision. Keep real declarations, dependency types and
function bodies. No productive enum mutation, suppression, threshold change,
public API, product default or pipeline-phase change is authorized. This is
ADR-0127 continuous internal flow with L0 first, restricted sealing, and final
test/path/lineage validation.


## Approved reference contribution integration

Measured existing boundary before decision: `03_infra/src/integration_tests.rs:1521 and :1538` retains the old generic transport or legacy assertion behavior. Source inspection establishes ownership; the independently frozen actual L3 INNER_PREPARATION RED establishes the language omission.

The human approved frozen proposal SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2, recorded in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-phase-approval.json`, including the separate no-child clarification. Actual L3 explicit/callback INNER_PREPARATION regressions failed on the unchanged pre-implementation source (`reference-phase-red-frozen`), authorizing RED-to-GREEN for this exact phase. Existing unrelated obligations remain.

Migrate only pipeline_figura_numerada_prefixo_no_pdf from the intentionally removed productive figure:<kind> string-shadow projection to the existing public runtime typed image counter/actual Location proof. Preserve its original source, image fixture and PDF nonempty guarantee; inject the runtime introspector into layout and require actual Figure 1 caption text. The unchanged full-suite failure None versus Some(1) is the functional migration evidence. Do not restore productive legacy strings, change synthetic API behavior or weaken numeric/render expectations.

No additional public contract beyond the exact three approved transport functions, new target inference, second arena, shared result/effect cache, fake observation, weakened diagnostic or unrelated generic visitor change is authorized. Preserve five-attempt stabilization, canonical paths and scoped lineage; validate reached and hidden effects, forward/repeated references, aliases, missing/unnumbered/non-Figure/bibliography/page controls and mixed target behavior.

### Ordinary Heading reference fixtures use the retained contextual phase

The actual full12 run in figure-kind-implementation/validation-final fails
p788_ref_happy_path_section_en and p788_ref_suplemento_explicito. The exact
sources at integration_tests.rs:4822–4850 evaluate correctly but the tests send
the newly admitted private Ref Context producer directly to pure layout. That
layer correctly does not execute callbacks. Independent actual CLI non-Figure
forward-reference controls already pass through the retained Paged phase.

Preserve both source strings and their exact existing Section 1/not Secção and
Cap 1 plain_text predicates. As in the neighboring p844_a8 integration control,
index the evaluated content, invoke existing public
expand_context_blocks_and_reintrospect with the real world/source, then use
layout_with_introspector on its actual expanded content and returned index.
This executes the required production contextual Session before observing the
same paginated language text; it does not require PDF operator inspection or a
new compiler facade. Change neither do_eval nor generic layout helpers, and do
not manually invoke/unpack a reference producer or restore eager evaluation.
Require preserved full-suite RED and focused GREEN with exact state/UTC and
source hashes. No productive behavior, public API or phase contract is changed.
