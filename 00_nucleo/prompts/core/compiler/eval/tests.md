# L0 — compiler/eval/tests
Hash do Código: 3c5a7671



**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/markup-sequence-children.toml sha256:cd963a81970f86c33abcf36ffb19ec69acfd789ab528092c8abb7a95129ebd99
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843

- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:66cf98676c3f6ddd3641e4e67eaed2ed2b71a06d5fd9a0be93dbc3059e506074
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:727a433beef7710d70729176c3b5a5f851b7e98be320c7594b86e5b269d8d49d
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:2213a4793474159cfae64b699263b16621a492ad2056fbaf5d28ac7950593c6b
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:a0d3cae81e77da5875204a94bc2fbd707009f0d751cf50ed870c416a01187a22
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/paragraph-hanging-validation.toml sha256:00e900edd4f7f3d6cd9aef545f6631615d612b377b17e9dd37f0254c4fd7cf51
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:041abef9e300ff0412d9631dced12480f8e0e6e65fb9cc3733f290e6664f649b

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/tests.rs:26` — `fn eval_for_test`.
- `01_core/src/compiler/eval/tests.rs:36` — `fn eval_for_test_keep_sink`.
- `01_core/src/compiler/eval/tests.rs:60` — `fn eval_for_test_with_registry`.
- `01_core/src/compiler/eval/tests.rs:85` — `fn eval_for_test_full_error`.
- `01_core/src/compiler/eval/tests.rs:112` — `fn eval_for_test_with_limits`.

## Contract

Own only the eval unit-test consumer, pure test Worlds, fixtures and test helpers,
not productive evaluation code. Test public semantics, syntax, morphology and
complete observable diagnostics/spans rather than Debug, pointer addresses,
render-byte identity or an expectation copied from the candidate.
For collection method calls, test that a receiver block with a local mutation is
evaluated once. Baseline measured on HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`
with the uncommitted tree captured in the item 6 experiment: vanilla pinned
`9f2b6e8715237cb086899a42873660fe744622e8` produces `(3, 1)` for
`({ n += 1; (1, 2, 3) }).len()`, while the prior Tekt binary produces `(3, 2)`.
This is a language-level observable; the source-level inference is duplicate
receiver evaluation in `instance_methods.rs:18` and `collection_methods.rs:18`.

Cover literals/units, expression/block/array/dictionary/spread order, scopes,
bindings/destructuring/assignment, closures/With, operators/short-circuit, loops,
imports/includes/modules, static/bound calls, field/content/snapshot methods and
Content construction. Positive tests retain values/effects; negatives retain
severity, exact message, hint/trace order and publicly available UTF-8 half-open
ranges. Preserve legitimately absent sources without inventing anchors.

Repeat applicable feature cases under default/html/a11y/html+a11y and the independent
bundle combinations. Disabled profiles do not count as exercised parity; local
homonyms remain resolvable. Test asset/std.asset/format.bundle against the real
active constructor identity, path normalization, UTF-8/arbitrary Bytes and schema
errors (including prohibited kind), not an old active-feature placeholder.
Actual target export obligations remain with L3/L4 tests.

Imports distinguish bare no-effect identifier warning from alias/field/items/
wildcard/file/dynamic/failure cases; warnings survive later errors. Module field
diagnostics use stored names and field-only spans; std aliases retain actual
identity and imported sources retain Source. Test global color membership and
PDF accessibility gates without giving this test prompt registry ownership.
PDF type/repr are function/pdf; emoji remains module.

Array repr tests cover boundary cardinalities (0/1/39/40/41/42/81/256), nested
escaping/Unicode and full payload conservation. Elision cannot leak into Args or
Content sequences. Named-module, alias, reexport and collision tests retain stored
names. JSON/TOML/YAML default/pretty/compact, CBOR, decoders/read and nonfinite/-0.0
cases preserve their own contracts, not generic warning/trace normalization.

Args tests preserve complete occurrence order, repeats, individual and aggregate
origins through direct/alias/With/spread/sink/linked calls and distinct equal-output
factories. Callback filter/map/join order and short-circuit remain discriminatory;
arguments.filter bool failures anchor at function origin while callback failures
retain their own causal origin. Queried-content tests retain authoritative ordered
snapshot fields through arrays/closures; language equality ignores label/location
as specified but detects relevant field/numbering changes. Do not replace it with
the internal observation relation.

Textual tests separate raw ContentBlock repr from realized flow: preserve Space
after Linebreak and between newline-separated CJK before interpolation, normalize
only later insertion. Test origin/collapse/discard independently from literal/regex
cross-node matching. Keep explicit-space, Latin/Korean, ideographic punctuation,
ambiguous quotes, comments, strong/dynamic, destructive Linebreak, style-chain and
nontext boundaries as controls. Local pre-normalized show replaces only its match;
combined local CJK show and sequential identical-rule override were separate unresolved cases at that baseline; only the later ratified textual-realization scope below supersedes their earlier limits after its own controls pass.

Math tests preserve real syntax/qualified construction, arity/casts, presence and
identities. Keep primes slot/nesting distinctions, cases MathAlignPoint and lr body
order. norm/round/abs require their public MathLr bodies with ‖/‖, ⌊/⌉ and |/|;
do not require historical MathDelimited lowering. lr cases likewise retain public
MathLr while unrelated MathDelimited cases keep their own contract. Annotation
families require each actual public variant/body/annotation/explicit-none state,
not glyph equivalence or MathUnderover. Mathematical overline is not text overline.
Test func/has/at/fields alongside repr where contracted. before/after tests cover
construction/default inclusive and explicit false; contextual query tests belong
to introspection's exclusive test owner.

Keep regression families for bytes, gradients, datetime, int, counter and scope,
with pure Worlds and independently frozen expectations. Neutral rounding values
are fixture mechanics, not public contracts.
No parser/build/fixture/import failure may masquerade as the expected result.
Unknown/timeout/opaque observation fails closed. Repeat/reverse orders must be
stable and comparisons may not strip diagnostics to manufacture equality.

## Paragraph validation, composition and lexical-cost controls

correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json records 52 bilateral inputs/source/commands/boxes under initial.json. absent/auto/false/true were locally equivalent; historical9f2b6e8715237cb086899a42873660fe744622e8 rejects hyphenate integer/string, justify integer/auto, invalid linebreaks; literal errors/spans in stderr. rules.rs:2972–3010,3407–3420 drops options, structural/par.rs:119–126 accepts/ignores, break_selection.rs:39 ignores policy; upstream text/mod.rs:545–565/model/par.rs:227–242, 627–636 defines domains. This measures behavior/types, not old intent.

Internal continuous parity narrowly supersedes atomization's prohibition on honoring discarded properties for this package, without public signature/entity/phase change; shared paragraph-break-policy is not productive owner. Test set/native valid types/errors/spans/lexical custom Auto/Bool. Replace discarded-hyphenate warning canary with still-unsupported stroke, never remove assertion or weaken geometry/tolerance/language tests. first-core-integration.log6316pass/one P134 failure expects discarded justify:preserve known-property-versus-unknown-target purpose with unconsumed first-line-indent, include valid leading/no warning.

correcao-composicao-paragrafos-2026-09-28 initial/before/before-pdf34 historical controls measure first justified end145.52versus172.000002pt at180pt/margin8, final110.048both; native Par2versus3 physical lines, empty-starty3.74versus18.14, empty-auto24.568versus38.968. line_commit already justifies, word_layout ordinary flush/layout treats Par transparent/style restore late/eval groups show flow. XML groups are not lines. Test repr/native Par/body and show-par inline styles/actual boundaries, no extra Par per wrapper, keeping earlier diagnostics/other assertions. Global preparation/optimized were separately incomplete until measured selector/cost owners; do not claim them from this slice.

precedence-state.json/regression-acceptance191checks/two show-par-sets-false failures and show-par-full-tree show swallowed #set scope. eval/mod.rs:1532–1560 has actual provenance, rules.rs:660–727 treats all Styled inline. Hub emits private eval.flow.scope in existing Styles; rules keeps pre-material #set scopes outside paragraph realization, preserves SetPage/Empty/Space/SpaceFromNewline without wrapping, postmaterial remains inline. No style-key provenance heuristic/public field/API/default/phase. Later measured whole-material constructor Styled scope factors too only with prefix/suffix Empty/Space/SpaceFromNewline/SetPage; partial fragments remain inline. The narrow whole-flow rule supersedes absolute constructor exclusion, not all boundaries. Preserve real style/precedence/restore controls.

rules.rs:3070/par.rs:131 validates but fails transport; correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json refutes distinct strategies. Extend lexical-channel test for par.linebreaks set/native simple/optimized/auto, keeping invalid casts/spans, no public/language mechanism change.

correcao-hardcode-paragrafo-2026-09-28 initial/before/before-pdf20 controls show missing bool overhang cast and fixed0.55 hyphen compensation; upstream text/mod.rs:391–393 bool/defaulttrue, line.rs:518–525 final style. Initial paragraph_select suspicion refuted by mandatory-line attempt.justify handling upstream linebreak.rs:551–559,line.rs:138–139; preserve formula/natural-last-line tests. Constants0.55/base135/distance5/increment0.15 are normative source-derived quantities, not configuration repaired by renaming. Test set overhang true/false without discarded-warning, auto/int/string expected errors, no snapshot changes. Literal-hyphen identity requires own carrier, not glyph/position inference.

correcao-custos-paragrafo-2026-09-28 initial/before/before-pdf66 controls, pinned text/mod.rs:1507–1564, inline/mod.rs:240, linebreak.rs:29–30,961–962 establish Dictionary/Ratio field order/partial fold/shared scope/135-100×ratio; local eval drops costs and layout fixes defaults. Test invalid dictionary/component types/normative first-error precedence and valid costs without discarded-warning, expectations from source/probes including negative ratios. text_costs owner resolves components/default1; partial fragments never replace shared costs, only four text.costs.hyphenation/runt/widow/orphan channels join preparation whitelist after restore/inline refuters. Whole-flow Styled factors before Par, temporary preparation push restores; no costs-created typographic boundary/glyph-style inference. Debt49 intermediate overhang→costs canary is superseded by supported costs→kerning:false, retaining three warnings/spans/baseline/stroke. Uniform regional widow/orphan subset, not validation alone, owns pagination; outside subset fallback remains.

correcao-viuvas-orfas-2026-09-28 before-pdf78/before-extra24/fullstate historical controls show3+1versus2+2, zero-cost equality, tiny/large-positive same protection. Pinned flow/collect.rs:196–229/distribute.rs:343–364 defines strict-positive first/last pairs/whole-three and next-region fitting; local emit lacked grouping. Boundary criteria remain prepared uniform vertical metrics/no baseline offset/constant region/no columns-cells-subframes-notes-floats-active transaction; certify entire chosen ranges, finite nonnegative top/descent/leading, exterior-matching size/edges/leading, bold-italic only same metrics. Auto bounds no protection; account initial baseline/block-chain collapse/resolved next margins; advance only nonempty current+whole next fit, oversize ordinary progress/no loop. Separate text_costs widow/orphan default1/strict>0; ordinary no-linebreak Simple default, justify auto/refused contexts remain. No reselection/glyph emission/new_page duplication/justification-carrier-last-line-lexical change. Keep same-word2+2/3+1/whole-three/orphan-off/oversize/restore/default/fallback/preserved-spans controls and no XML-line-count proof. These obligations belong to canonical layout owners, not test authorization.

## Single-evaluation, provenance and occurrence recursion tests

Count World reads/query original+show-generated elements using local MockWorld state, no mutable globals. One common eval also with show; it return retains one origin, equal-body headings retain distinct occurrences. Legacy adapter projects visual for render-structure assertions only; Module.introspection_content remains whole for semantic query/identity. Recipe reads original Location/fields transparently; prepared it twice has one query occurrence/two visual emissions per historical9f2b6e871 probe, distinct from ordinary not-yet-realized insertions. Counters use causal origin/product order, not numeric ids.

Under ratified `00_nucleo/prompts/core/entities/module.md`, show eval provides memoized original/generated index; no-show snapshot absent/fallback remains; columns+show reconstructed fresh preserves body/label/counter sequence. Observe semantic Module/index contracts, not Content visual equality.

show-recursao-auditoria-2026-10-06/measurement.json.gz freezes four wrongly accepted infinite generations/four finite controls against historical pin. After apply-owner confirmation, add productive rejects with message/hints, double alias, finite where, multiple rules, effects/lexical context; legacy fixpoint helper tests stay untouched. core-first.log five P348/P350c failures use revoked policy: only those productive expectations become unlimited-generation rejection with two standard hints/additional full_error classification. Keep a→b→c→it/isolated helper tests. controls-first.json refutes block duplication/local RuleId collision: add container alias/earlier-rule composition/local it rule. markup-span-red.json heading markup lacks origin locally versus upstream range=Z; test actual causal range for markup and heading[Z], no internal Span numbers/hint removal.

## Source identity, quote morphology and mathematical insertion

Source-based identity evidence/approved gate is `lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json` and `lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`, owner confirmo2026-10-06. It records source/UTC/HEAD/tree/binaries/commands at historical9f2b6e8715237cb086899a42873660fe744622e8; world.main reuse mislabeled document/expression, upstream separates ids/origins, integer/interner mechanics not required. Pure World tests distinct-expression Source id, own runtime/parser errors, main Source preserved, read/import base world.main, callback origins, fresh scopes/features/legacy string facades; Mock derives no expectation from candidate.

empty-markup-warning-2026-10-06/before.json measures two-character **/__ caret including Unicode/adjacency. Test published UTF-8 ranges/Source id/message/hints/status, newline-trivia exclusion, two adjacent constructs, empty content block; nonempty no warning/open delimiter error, unchanged generic parser-warning expectations.

smartquote-2026-10-06/before/fields-before first supported native-function alias/programmatic show→Q while literal markup remained phase-gated. before/fields-before/partial-and-html/proposal-controls freeze historical9f2b6e8715237cb086899a42873660fe744622e8/source/binary/HEAD/tree/diffstat/UTC/argv/cwd/environment/output:eval Text loses double/enabled, stored quote bakes English under German/French/disabled use, 6'/2" become quotes; omitted fields differ from explicit false, disabled stays SmartQuote upstream; filtered/set positives and programmatic HTML rejection differ. Markup glyph choice:79–128, layout two flags, native disabled erase:112–117, observation:242–250; upstream markup:141–146 creates explicit-double element, inline:196–211 resolves context, smartquote:119–158 prime/apostrophe/nesting. Title renders literal quotes; script accepts string/rejects Content before export in both.

Human ratified 2026-10-06 only SmartQuote public double bool→Option<bool>, enabled:Option<bool>, pure helper/context and eval→target-render glyph choice. Preserve semantic identity/known fields in eval, late target style/context, pure per-flow/compilation L1, no I/O/globals/vtable/property registry/unsafe/watch API/math-phase/mode/flag. HTML uses own escaping/composition, never paged layout. This narrowly supersedes conflicting historical quote clauses. RED→GREEN [it's Ω]/["hi"], double presence/false/disabled, callback/alias show; eval tests type/morphology/fields/Source, render owners keep same language/glyph/style expectations. Never delete locale/quote tests; move observation to correct target. Code string stays Value::Str.

D05 after.json.gz/expression_dispatch.rs:270–281 show missing literal-equation capture: test alias math.equation.where(block:true) on $ b $, inline intact, body once/existing guard, no new phase/baked render/representation demand.

math-bare-2026-10-06 before/additional/refuters/math-bare-medicao preserve HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/diff/source/binaries/historical9f2b6e8715237cb086899a42873660fe744622e8. Bare frac/mat/sqrt/lr/op Equation1/0, bold/ordinary0both, nested-explicit1both, reuse2upstream; style/show/context probes distinguish boundaries, explicit fraction/it.body depth error/valid it/later second insertion/raw labels refute shortcuts. Upstream realize:245–265, 302–334 wraps before show outside math; local equation text projection/realize missing normalization/bottom-up traversal/existing-locatable apply explains observed gap, not mandated algorithm/bytes. Owner ratified `lab/history/construction/records/00_nucleo/materialization/math-bare-2026-10-06.md:3`/math-bare-aprovacao-2026-10-06.json, manifest 149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6; earlier pending gate satisfied only named boundary, no universal parity/math-label/other-debt closure.

Test raw frac before insertion; frac/mat/root/lr/op queryable; ordinary/math.bold([x]) no Equation; distinct equal insertions; explicit/math-inside no extra wrapping; where(block), local scope, direct-alias-where show, guarded it, new generation/it.body depth error, nonretroactive later rules, effective styles/alt. Index counts not Rust shapes; Mathy label never promoted. Pure tests here; normalizer/traversal tests with own owners. eval --in exposes no file variable to auxiliary expression; use directly evaluated block, never unknown f as raw-value proof. Bilateral scoped RED→GREEN/full validation required.

## Table insertion, standalone occurrence and declared Cell defaults

 table-cell-realizacao-d08b3-2026-10-07/before.json.gz, red.log, nested-reuse-red.log, local-bool-red.log preserve state/source/separate REDs. Insertion ratified manifest 5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b, owner comfirmo at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`. This cfg(test) owner supplements43 productive owners with field/order/hidden effects/local alias/nested cell/context/Bool-as-Raw/causal-span controls. Preservation record must name this additional test consumer/L0, not claim no outside file changed. No new production authorization/general Raw/Table-default certification.

Standalone complete.json.gz has 53 probes2026-10-07T14:47:56.340144+00:00–14:48:12.375245+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty status/diffstat/hashes/source/argv/cwd/environment/output. Historical pin9f2b6e8715237cb086899a42873660fe744622e8/oracle7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea/Tekt6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. before/refuters/complete are not additional independent rounds. Upstream standalone exposes auto x/y/inset/align/fill/breakable, spans1, empty stroke dict; local omits defaults/duplicates metadata, raw remains body-only/raw-x rejected both. Standalone Table style must not supply Cell defaults; raw/prepared equality false versus true. Nested/effect/late probes refute repeated effects/lost lexical origin; fresh identical/growing/wrapped/multiple Cells must depth-error with new-constructor span while alias/body/conditional-hidden remain valid. Returning it permits earlier unconsumed recipes; query remains cell is not locatable/location none, no false Location.

Upstream table.rs:734–778/realize:367–386, 418–429, 491–505, 571–586 prepares defaults/guards/product depth; local table_cell.rs:111–143/table_cells.rs:110–129,280–304/show_rule_termination:65–70/apply:139–166 expose missing standalone preparation/morphological stop/global revocation. Nominal policy is Tekt choice, refutable by alias/two-rule/new-generation/hidden-content controls, not upstream mechanics. Human confirmo ratified standalone public field/type package at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest 3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df; insertion A alone did not authorize it. Test raw/prepared getters/presence/defaults, once effects/hidden-nested-reused-late-local body, it/alias/two earlier rules, fresh identical/growing/wrappers/conditional/hidden within/outside Table, limits/spans with existing causal Engine helper, no rule-deciding mock.

Require scoped bilateral full effects/equality/getters/selection/HTML/text-style-real geometry/messages/carets without erasing differences; debug/release/core-infra-shell suites/build/lint/no V5/V6/V15/V26; revalidate D08A/B2/B3-A, Raw blocks/defaults, Equation/math/matrix/query. Preserve old tree via snapshot/restricted diff, no commit/reset/global reseal. Standalone slice completed intrinsic defaults/alias-versus-fresh only; declared defaults completed below for its measured subset. General Table defaults/equality, rich casts/paint/strokes/insets/lines/gutters/multipage spans/PDFUA/general nonlocatable policy/empty-file warning remain open, not normalized for parity.

Standalone after-second.json.gz/html-fields measures local Auto as Text versus upstream code(data-lang:typc)/constant.language#d73948. Upstream value.rs:191–208 Auto/Bool Raw inline typc; markup_stream:210–215 covers Bool but Auto fallback Text, layout/raw:201–203 only true/false, HTML Raw generally rejected. Before code test Auto not Text/atomic Raw HTML accepted. Limit correction to Auto→Raw(lang:typc,block:false), auto constant-language color and HTML auto/true/false code+color span, existing escape/mode. No type/API/phase/flag/default/grammar or arbitrary Value/general HTML Raw certification; Table/Raw/query/style/product RED→GREEN.

 table-cell-styles-d08b3c-2026-10-07/before.json.gz58 probes2026-10-07T15:24:58.038947+00:00–15:25:13.727872+00:00 preserve same HEAD/tree/full status/diffstat/source/argv/cwd/environment, pin/oracle above/Tekt721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set/individual-field/late-local-reuse-spread/errors refute empty-target warning/missing defaults; padding-span-one/empty-fixed-rows spans1, Auto delegates Table not Cell set; show-cell-set-self changes standalone but not positioned fill; Stroke/alignment fold. refuters-before adds header/partial inset/args-condition/snapshot Table; alias-inside-table valid upstream, recurses locally. Upstream table.rs:734–778,785–800/resolve:203–278 uses styles before position-fill, spans1 padding; local set.rs:23–27 only text target/table_cells:262–310 lexical props/grid_placement:80 raw slots/standalone no private effective props. This is language, private representation not Packed/Rust equality requirement; refute with any negative, lost lexical source/raw mutation/repeated effect/lost alias guard/geometry-cache regression.

Test RED before repair: setCell raw/prepared defaults/getters, span-padding-placement style, alias/local/late, constructor/Auto/None/fold, conditional/spread/causal errors, standalone-versus-positioned show-set, it alias in Table. Observe language/real effect order, not private structs; revalidate standalone/Raw/math/query/build/lint. Internal continuous mapping parity, no new public contract/external compatibility/flag/default/phase; props/helpers private in already externally nonconstructible carrier, lexical contracts intact. Certified subset: integer/auto, Color/None/Auto, uniform Stroke/None, alignment, Length/partial Length-Auto inset, Bool/Auto/existing typed schemas. Gradient/tiling/functions, per-side strokes, percent inset/rich constructor casts, rich groups/multipage spans/PDFUA, positional set/text/general Table defaults remain incomplete pending measured owner L0. No warning fallback certification/upstream defect copying.

Refuters show whole constructor stroke/alignment overrides Cell set for standalone getters; self show-set colspan2 occurs after placement1, not proof of reoccupancy. Upstream resolve:218–244 freezes occupancy but not content span fields; table.rs:785–800 fixes padding spans1. Private positioned TableCellResolved remains physical plan; standalone projection may coexist only for observed recipe properties, overriding lexically omitted spans through show-set while preserving resolved fields. Getters/canonicalization use projection, placement uses original complete plan; unsourced padding has lexical spans1 so show-set cannot change it. Stage/guards stay causal, no public/source mutation; getter RED→GREEN plus geometry/regressions. This narrowly supersedes earlier mutually-exclusive prepared-stage wording only for this observed-properties overlay.

extra-first.json.gz freezes full state/source/binaries:successive red then 2pt stroke sets yield2pt+red standalone/positioned; render constructor2pt resolves inherited Cell red before Table3pt although standalone getter exposes constructor only. Upstream #[fold]/stroke.resolve differs from raw getter. Successive declarations fold specified Value::Stroke components, no carrier/callback in Styles. Positioned constructor folds captured Cell base then Table; private TableCellProperties.stroke_inherited:Option<Stroke> carries causal base, not lexical getter. ObservationEq includes base, canonicalization ignores render-derived base; already resolved Table base None, clone/alias retain prior base, explicit None wins. Constructor whole-precedence applies standalone getter, not this measured render fold. Private continuous RED→GREEN, no public/phase/default.

show-fold-before.json.gz/geometry-final.json.gz measures red lost on accumulated show-set/black1pt despite known red2pt. Pure private Cell chain_styles folds cell.stroke deltas; canonical borders owner reads surviving cell_body snapshot with no resolved-Cell fallback. Other styles/guards/order unchanged; helper folds declarative Value::Stroke only, no carrier/layout/callback/public field/API/phase/default. Real stroke recollection/RED→GREEN mandatory.

 table-origens-d08b3c2-2026-10-07/baseline.json.gz freezes HEAD/tree/UTC/source/binaries; owner confirmo2026-10-07. Test local set/show func styled, content-origin bare/alias/spread caret+trace, raw body shape/distinct sources/reuse/metadata once/hidden body. additional-debug/prior-local-body refutes old FUNCsequence with FUNCstyled:only this morphology expectation/test name changes. Structural inspection traverses Source without hiding child style/element/assertions. Exclusive/static boundaries/no I/O/new API/trait/flag/eval-layout phase; C2/C/macro/build/lineage required.

## Labels, conditional set and code-tail tests

labels-expressoes-2026-10-14/before/results.json.gz freezes HEAD/tree/UTC/source/binaries/historical9f2b6e871:literal works, alias/call/closure/block label renders text/query misses/orphan warning absent. Pure World RED tests whole names, Heading association, preserved spaces/orphan warning-range, block confinement, show-label/text rule once, expression once. No whitespace/diagnostic stripping/vanilla-tree identity; unlabelable-target/repeated-label controls remain separate labels-associacao-alvos. Initial raw newline declaration spaces in fixtures were resolved by oracle-confirmed semicolon-equivalent fixture-controls.json, not output trim/space-semantic change.

labels-escopos-2026-10-14/before/results.json.gz freezes same provenance: Parbreak blocks target, set/show/equal-false set crosses scope incorrectly, set page hides orphan. Test literal/expression, block and alias-interpolated parbreak, separators, set/show/equal/false, page without target, exact warning span/new tail target; let/code-block creates no outer boundary. Reuse helpers/Worlds/full untrimmed expectations, no repeated-label/explicit-Empty/show realization closure from local selection.

set-condicoes-2026-10-14/before/extra-before/results.json.gz freezes pin/HEAD/tree/UTC/source/binaries:nonCell conditions ignored, false args execute, Cell type uses int, Heading swallows arg error. Test shared condition gate before target/args/features, false no mutation/carrier/warning, condition once before args, long type names/ranges/causal errors/Cell aliases/show-set/numbering. Active Heading and registered Tekt callout argument failures retain causal diagnostics, callout has no false vanilla oracle. True/absent executes args. No trim/changed old expectations; disabled HTML tests short circuit/access denial, not active parity.

codeblock-set-2026-10-14/before/results.json.gz/`00_nucleo/prompts/core/compiler/eval/expression_dispatch.md` measure raw-tail types/lost lexical styles/prefix join/absent return warning with exact HEAD/tree/UTC/source/binaries. RED tests tail conversion(false/Empty), prefix/span, local/nested/equal styles, once eval, return/loop/Table/Math; no-set controls retain types. No old-expectation changes/private-stack/byte acceptance; scoped continuous flow/suite/product probes. Other Value presentation, selectorless show and SetPage morphology remain separate. refuters-after/raw-rule-false/active loses Raw show on Bool:add independent false/true/nested-set Bool/Auto tests; style observer rejects unexpected variants, never silently skips.

first-validation/workspace.log fails old #heading[Alvo]#{set text(fill:red)}<alvo> expecting Heading; labels-first/results.json.gz shows upstream styled empty child/cannot reference styled, COUNT1 never proved Heading target. let returns None/upstream labels empty sequence. Correct old expectation, not code-tail implementation; keep new Heading after set/comment as prior-target controls, test empty CodeBlock target separately. Empty let/query association/publication remains open under actual owners, not closed by count equality; retain logs/report correction.

## Raw presence, Default and text-size fixture approvals

content-serialization-2026-10-14/before/raw-controls/results.json.gz freezes HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/source/binaries/historicalpin. RawElem:21–25/native_raw:78–136 loses omission/collapses false-none; fields/equality/encoders differ upstream. Ratified manifest b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, `lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`, supersedes only earlier no-presence-mask clause. Test omitted/false/true block, omitted/none/string lang, alias/equality/at-has-fields/JSON-YAML/markup presentation, no trim, new render/default/phase or unrelated contract change. RED→GREEN/suite/build/lineage.

EvalContext::new:397–424/source-owner snapshot shows missing Default/explicit states. `00_nucleo/prompts/core/compiler/eval/mod.md` owns public contract; owner approved manifest 11f3e138f5ba006d04ed4dd1e829e27c004778e691f4a1df63127b1644146d03 on2026-10-16 with clippy-default-2026-10-16 approval-receipt. Earlier pending gate satisfied only three impls/D1–D5. Test Default-bound use and isolation: Paged, empty features, full_error=false, recipes enabled/current budget, independent counters/locations/flow, mutated context cannot contaminate next Default/new. Missing-impl RED→delegation GREEN, unchanged language/diagnostics/fixtures, no new upstream parity.

text-argumentos-2026-10-17 before/before-measurement/morph-before-measurement/audit preserve HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/status/diffstat/source/L0/binaries/argv/historical9f2b6e8715237cb086899a42873660fe744622e8. named-em0versus24pt, affine3versus27, nested18→0versus36, six Size/Color/body orders rejected locally/accepted upstream, positional18 ignored12versus18. Pinned text/mod.rs:290, 303, 924–930, 1136–1142, args.rs:178–247 typed search/named priority/restbody/affine composition versus style_chain:513–517/constructor:123 discarded em/collapse top-wins:394–395, 419–422. Restoration/Raw/heading/math/capture-show/duplicates-spans/zero-negative/metrics refute; upstream text2em==text24pt compares Styled child(content:779–782), separate equality defect not closed by full type.

Human approved manifest 771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba/two public contracts, S1–S16 under12GiB/swap0, no release/merge/push, seal only after implementation/valid graph, RG1 independent. Only this owner's fixture/adapters change Style::Size(Pt)→Size(absolute Length), StyleDelta.size expectations→Option<Length>, retaining language/function/geometry criteria, not new-representation snapshots.

## Independent test observations and CI cleanup

`lab/results/construction/diagnostics/testes-ci-2026-10-13/before.json.gz` preserves HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty source/hashes/UTC/status/diffstat. matematica-ci-2026-10-13/clippy-inventory/evidence.json.gz records initial warnings. tests.rs:3989–4022 uses prohibited native math for four inverse-trig expectations; ratified oracle angles.json gives 30, 60, 45, 63.43494882292201 degrees with recorded float representation. Replace only platform-dependent expectations with independent degree constants/tolerance1e-12, retaining Value::Angle/public x, y order. At 6492 require actual Equation instead of discarded Module; 6501–6519 require observed mathematical𝑥 and no brackets, not conditional Text-only acceptance. Observe missing/empty negatives too. Singleton Range is spans collection, not offset iteration: use from_ref/once with full bounds. Simplify only proven redundant imports/bindings/Copy/borrows, rename private internal tests module regressions/private matrix tuple alias, keep all cases/spans/messages/fixtures/assertions. Private prefix rename removes no test, new lint exceptions forbidden. Internal tests only, Clippy-denied warning structural RED not fabricated functional RED; run cases/lineage.

## Exhaustive existing FrameItem decision


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace has_line's `_` at tests.rs:7015(historical F001) with FrameItem::TextReplacement/TextSeparator/Text/TextShaped/Glyph/Image/Shape.
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

## Nominal projection and style complements

tekt-v16-eval-helpers-2026-10-17/before.json.gz, contexts.json, audit-recipe.json preserve HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/exact diffstat/full sources/config/hashes/pinned parser: eleven open unguarded AST arms at tests.rs:129,494,538,566,571,632,645,771,941,963,1098; Content120/Value39, prior passing executable hash-verified. This is test-helper structure, no new language/upstream intent/AI-benefit inference.

For each named helper below replace only `_` with current nominal enum complement excluding listed already handled whole variants. Use in-scope prefix/appropriate `_` or `{..}` payload/flat group, no new binding. scope.json retains full complement/spans. Shared prior negative body justifies informational V19, not per-variant execution/coverage. Future enum additions require compiler decision.

- `eval_for_test_with_registry` at tests.rs:129 (Content; historical E01): exclude `Located, Realized, ShowRecipe` from nominal complement. Legacy visual adapter removes only Located/Realized/ShowRecipe; no other node replacement.

- `go` at tests.rs:494 (Content; historical E02): exclude `Heading, Sequence, Styled` from nominal complement. Find first Heading with inherited numbering; other variants produce no result or new recursion.

- `go` at tests.rs:538 (Content; historical E03): exclude `Heading, Sequence, Styled` from nominal complement. Collect headings in current order; excluded leaves add no items.

- `go` at tests.rs:566 (Value; historical E04): exclude `Str, None` from nominal complement. Non-Str/None values retain inherited numbering pattern, without validating a new pattern.

- `go` at tests.rs:571 (Content; historical E05): exclude `Equation, Sequence, Styled` from nominal complement. Find Equation with inherited numbering; other nodes produce no result.

- `find_matrix_or_vec_delim` at tests.rs:632 (Content; historical E06): exclude `MathMatrix, MathVec, Sequence, MathSequence, Styled, Equation` from nominal complement. Find Matrix/Vec delimiters through current traversal; other variants provide none.

- `find_op_limits` at tests.rs:645 (Content; historical E07): exclude `MathOp, Sequence, MathSequence, Styled, Equation` from nominal complement. Find MathOp limits through current traversal; other variants provide none.

- `go` at tests.rs:771 (Content; historical E08): exclude `Figure, Sequence, Styled, Label` from nominal complement. Collect Figure numbering with current style/Label; other nodes add no items.

- `has_dynamic` at tests.rs:941 (Content; historical E09): exclude `Dynamic, Sequence, Styled` from nominal complement. Detect Dynamic through existing traversal; others do not satisfy detector.

- `find_styled_custom` at tests.rs:963 (Content; historical E10): exclude `Styled, Sequence` from nominal complement. Find custom within Styled/Sequence with existing preference/short circuit; other nodes provide none.

- `find_custom_in_styled` at tests.rs:1098 (Content; historical E11): exclude `Styled, Sequence` from nominal complement. Find custom within Styled/Sequence preserving first key/order; other nodes provide none.

Keep other Rust tokens byte-identical after rustfmt: prior bodies/guards/payloads/calls/returns/order/short circuit/inherited styles/fixtures/assertions. No Source normalization/traversal expansion in this slice; semantic provenance remains distinct from visual adapter. No match→if-let/matches!, allow/expect/threshold/preexisting-group change to hide V19; notes stay visible, no absent observation accepted. Structural RED11V16/no sentinel E0004→GREEN11E0004 in disposable cfg(test)workspace Content/Value probes, not functional RED/new parity. Replay unchanged bodies/enum/signatures; SARIF11V16 removed/11V19 added/V20 unchanged; workspace/build/fmt/strict all-target Clippy+expectations/Rustdoc four HTML indexes/V1-V5-V15-V26/reverse/scoped pair/frozen provenance/12GiB/swap0/jobs1/sequential. Other52V16 and Source(Table) were outside this slice, not silently closed; later named math work below supersedes only its own scope. No release.

## Nominal mathematical and structural complements

tekt-v16-eval-math-2026-10-17/before.json.gz, contexts/all-contexts/wildcard-inventory preserve same HEAD/tree/UTC/full source-L0-hashes-config/status/exact diffstat, pinned AST/prior passing executable verified.52emitted V16 plus18latent fallbacks(seven Content/eleven MathAttachSlot). leaf/MathFrac/Quote guards and Present(MathPrimes) payload do not cover whole variants. Replace only each `_` with nominal complement of unconditionally handled variants below, Content120/MathAttachSlot Omitted-ExplicitNone-Present(Content). Retain guarded variants in complement, all bodies/slot-search order/any-find_map/callbacks/returns/errors/fixtures/expected. No if-let/matches!, suppression/threshold; V19 stays informative, not all-alternative execution proof.

- `go` at `01_core/src/compiler/eval/tests.rs:1680` (Content; historical M01, latent): exclude `Table, Sequence, Styled, Label`; retain fallback `(no guard)`. Collect only properties in contracted traversal; other variants add no items, inheritance/order unchanged.

- `has_math_root` at `01_core/src/compiler/eval/tests.rs:8128` (Content; historical M02, emitted): exclude `MathRoot, Equation, MathSequence, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `has_math_root_with_index` at `01_core/src/compiler/eval/tests.rs:8146` (Content; historical M03, emitted): exclude `MathRoot, Equation, MathSequence, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `eval_label_anexa_ao_bloco_anterior` at `01_core/src/compiler/eval/tests.rs:8405` (Content; historical M04, latent): exclude `Label, Sequence`; retain fallback `(no guard)`. Unexpected assertion variants retain original panic, including Quote without attribution on guard failure; absence never succeeds.

- `tem_styled_sobre_label` at `01_core/src/compiler/eval/tests.rs:9348` (Content; historical M05, emitted): exclude `Styled, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `tem_context_block` at `01_core/src/compiler/eval/tests.rs:10294` (Content; historical M06, emitted): exclude `ContextBlock, Sequence, Styled`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `tem_heading` at `01_core/src/compiler/eval/tests.rs:10312` (Content; historical M07, emitted): exclude `Heading, Sequence, Styled`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `encontra_equation` at `01_core/src/compiler/eval/tests.rs:10542` (Content; historical M08, emitted): exclude `Equation, Sequence, Styled`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `go` at `01_core/src/compiler/eval/tests.rs:10568` (Content; historical M09, emitted): exclude `Strong, Sequence, Styled`; retain fallback `(no guard)`. Collect only properties in contracted traversal; other variants add no items, inheritance/order unchanged.

- `go` at `01_core/src/compiler/eval/tests.rs:10587` (Content; historical M10, emitted): exclude `Emph, Sequence, Styled`; retain fallback `(no guard)`. Collect only properties in contracted traversal; other variants add no items, inheritance/order unchanged.

- `styled_bold_envolve_heading` at `01_core/src/compiler/eval/tests.rs:10621` (Content; historical M11, emitted): exclude `Styled, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `contem_heading` at `01_core/src/compiler/eval/tests.rs:10630` (Content; historical M12, emitted): exclude `Heading, Styled, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `go` at `01_core/src/compiler/eval/tests.rs:10653` (Content; historical M13, emitted): exclude `Text, Styled, Sequence, Heading`; retain fallback `(no guard)`. Collect only properties in contracted traversal; other variants add no items, inheritance/order unchanged.

- `styled_bold_anywhere` at `01_core/src/compiler/eval/tests.rs:10736` (Content; historical M14, emitted): exclude `Styled, Sequence, Heading`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `styled_bold_e_weight` at `01_core/src/compiler/eval/tests.rs:10783` (Content; historical M15, emitted): exclude `Styled, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `eval_terms_construtor_typst_lang` at `01_core/src/compiler/eval/tests.rs:10898` (Content; historical M16, latent): exclude `Terms`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `eval_terms_construtor_typst_lang` at `01_core/src/compiler/eval/tests.rs:10900` (Content; historical M17, emitted): exclude `Terms, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_quote` at `01_core/src/compiler/eval/tests.rs:10932` (Content; historical M18, emitted): exclude `Quote, Sequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `eval_quote_construtor_typst_lang` at `01_core/src/compiler/eval/tests.rs:10949` (Content; historical M19, latent): exclude `Quote`; retain fallback `(no guard)`. Unexpected assertion variants retain original panic, including Quote without attribution on guard failure; absence never succeeds.

- `eval_quote_com_attribution_typst_lang` at `01_core/src/compiler/eval/tests.rs:10965` (Content; historical M20, latent): exclude `(no whole variant)`; retain fallback `Quote`. Unexpected assertion variants retain original panic, including Quote without attribution on guard failure; absence never succeeds. Guard-refused cases stay in nominal complement.

- `eval_quote_block_true_typst_lang` at `01_core/src/compiler/eval/tests.rs:10978` (Content; historical M21, latent): exclude `Quote`; retain fallback `(no guard)`. Unexpected assertion variants retain original panic, including Quote without attribution on guard failure; absence never succeeds.

- `eval_quote_quotes_false_typst_lang` at `01_core/src/compiler/eval/tests.rs:10991` (Content; historical M22, latent): exclude `Quote`; retain fallback `(no guard)`. Unexpected assertion variants retain original panic, including Quote without attribution on guard failure; absence never succeeds.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11282` (MathAttachSlot; historical M23, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11286` (MathAttachSlot; historical M24, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11290` (MathAttachSlot; historical M25, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11294` (MathAttachSlot; historical M26, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11298` (MathAttachSlot; historical M27, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11302` (MathAttachSlot; historical M28, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathop_in` at `01_core/src/compiler/eval/tests.rs:11304` (Content; historical M29, emitted): exclude `MathOp, Sequence, MathSequence, Equation, MathAttach`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathident_in` at `01_core/src/compiler/eval/tests.rs:11320` (MathAttachSlot; historical M30, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathident_in` at `01_core/src/compiler/eval/tests.rs:11324` (MathAttachSlot; historical M31, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathident_in` at `01_core/src/compiler/eval/tests.rs:11328` (MathAttachSlot; historical M32, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathident_in` at `01_core/src/compiler/eval/tests.rs:11332` (MathAttachSlot; historical M33, latent): exclude `Present`; retain fallback `(no guard)`. Omitted/ExplicitNone contain no visitable content; retain None and existing br/tr/b/t/bl/tl order.

- `find_mathident_in` at `01_core/src/compiler/eval/tests.rs:11334` (Content; historical M34, emitted): exclude `MathIdent, MathText, Sequence, MathSequence, Equation, MathAttach`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathstyled_body_in` at `01_core/src/compiler/eval/tests.rs:11345` (Content; historical M35, emitted): exclude `MathStyled, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `leaf` at `01_core/src/compiler/eval/tests.rs:11435` (Content; historical M36, emitted): exclude `Sequence, MathSequence, Equation`; retain fallback `MathIdent, MathText`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit. Guard-refused cases stay in nominal complement.

- `find_mathdelimited_in` at `01_core/src/compiler/eval/tests.rs:11527` (Content; historical M37, emitted): exclude `MathDelimited, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathlr` at `01_core/src/compiler/eval/tests.rs:11578` (Content; historical M38, emitted): exclude `MathLr, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathlr` at `01_core/src/compiler/eval/tests.rs:11608` (Content; historical M39, emitted): exclude `MathLr, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathlr` at `01_core/src/compiler/eval/tests.rs:11638` (Content; historical M40, emitted): exclude `MathLr, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overbrace` at `01_core/src/compiler/eval/tests.rs:11667` (Content; historical M41, emitted): exclude `MathOverbrace, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_underbrace` at `01_core/src/compiler/eval/tests.rs:11695` (Content; historical M42, emitted): exclude `MathUnderbrace, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_underbracket` at `01_core/src/compiler/eval/tests.rs:11742` (Content; historical M43, emitted): exclude `MathUnderbracket, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_underparen` at `01_core/src/compiler/eval/tests.rs:11789` (Content; historical M44, emitted): exclude `MathUnderparen, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_undershell` at `01_core/src/compiler/eval/tests.rs:11836` (Content; historical M45, emitted): exclude `MathUndershell, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overbracket` at `01_core/src/compiler/eval/tests.rs:11883` (Content; historical M46, emitted): exclude `MathOverbracket, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overline` at `01_core/src/compiler/eval/tests.rs:11911` (Content; historical M47, emitted): exclude `MathOverline, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overparen` at `01_core/src/compiler/eval/tests.rs:11940` (Content; historical M48, emitted): exclude `MathOverparen, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overshell` at `01_core/src/compiler/eval/tests.rs:11968` (Content; historical M49, emitted): exclude `MathOvershell, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `prime_count` at `01_core/src/compiler/eval/tests.rs:12007` (MathAttachSlot; historical M50, latent): exclude `(no whole variant)`; retain fallback `(no guard)`. Omitted/ExplicitNone and Present(non-MathPrimes) give no count; both nominal enums require future review. Its final shallow-match refinement below supersedes only nested implementation shape.

- `find_mathmatrix_in` at `01_core/src/compiler/eval/tests.rs:12044` (Content; historical M51, emitted): exclude `MathMatrix, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_unlined_frac_in` at `01_core/src/compiler/eval/tests.rs:12056` (Content; historical M52, emitted): exclude `MathDelimited, Sequence, MathSequence, Equation`; retain fallback `MathFrac`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit. Guard-refused cases stay in nominal complement.

- `find_mathaccent_in` at `01_core/src/compiler/eval/tests.rs:12111` (Content; historical M53, emitted): exclude `MathAccent, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `p958_mathtexts` at `01_core/src/compiler/eval/tests.rs:12343` (Content; historical M54, emitted): exclude `MathText, MathIdent, Sequence, MathSequence, Equation, MathDelimited, MathAttach`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `p962_find_styled` at `01_core/src/compiler/eval/tests.rs:12450` (Content; historical M55, emitted): exclude `MathStyled, MathClassOverride, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find` at `01_core/src/compiler/eval/tests.rs:12491` (Content; historical M56, emitted): exclude `Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathunderover_in` at `01_core/src/compiler/eval/tests.rs:18768` (Content; historical M57, emitted): exclude `MathUnderover, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overbrace_in` at `01_core/src/compiler/eval/tests.rs:18782` (Content; historical M58, emitted): exclude `MathOverbrace, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_underbrace_in` at `01_core/src/compiler/eval/tests.rs:18796` (Content; historical M59, emitted): exclude `MathUnderbrace, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_underbracket_in` at `01_core/src/compiler/eval/tests.rs:18810` (Content; historical M60, emitted): exclude `MathUnderbracket, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_overbracket_in` at `01_core/src/compiler/eval/tests.rs:18824` (Content; historical M61, emitted): exclude `MathOverbracket, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathlr_body` at `01_core/src/compiler/eval/tests.rs:19031` (Content; historical M62, emitted): exclude `MathLr, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `first_boundary` at `01_core/src/compiler/eval/tests.rs:19048` (Content; historical M63, emitted): exclude `MathDelimited, Sequence, MathSequence, Text, MathText, MathIdent`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `last_boundary` at `01_core/src/compiler/eval/tests.rs:19065` (Content; historical M64, emitted): exclude `MathDelimited, Sequence, MathSequence, Text, MathText, MathIdent`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathlimitsoverride_in` at `01_core/src/compiler/eval/tests.rs:19155` (Content; historical M65, emitted): exclude `MathLimitsOverride, Sequence, MathSequence, Equation, MathAttach`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `find_mathdelimited_in` at `01_core/src/compiler/eval/tests.rs:19291` (Content; historical M66, emitted): exclude `MathDelimited, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `has_linebreak` at `01_core/src/compiler/eval/tests.rs:19302` (Content; historical M67, emitted): exclude `Linebreak, Sequence, MathSequence, Equation`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `p1043_math_callee_dot_double_isolada` at `01_core/src/compiler/eval/tests.rs:19418` (Content; historical M68, emitted): exclude `MathAccent, MathSequence`; retain fallback `(no guard)`. Existing helper search/predicate only; no-result variants share prior fallback without broader traversal, false success or changed short circuit.

- `break_policy_custom_values` at `01_core/src/compiler/eval/tests.rs:24262` (Content; historical M69, emitted): exclude `Styled, Sequence, Par`; retain fallback `(no guard)`. Collect only properties in contracted traversal; other variants add no items, inheritance/order unchanged.

- `count` at `01_core/src/compiler/eval/tests.rs:24320` (Content; historical M70, emitted): exclude `Par, Styled, Strong, Emph, Sequence`; retain fallback `(no guard)`. Only Par adds one count; named wrappers recurse; other variants contribute zero.

Only Present(Content) slots delegate existing helpers; Quote/Label assertions keep exact panic text, no error→None/false-success conversion. Initial prime_count nested nominal payload replaced catch-all without Present(_), but candidate-state.json.gz/candidate-architecture.log measured170V16/988V19/872V20 and one added V20 depth, refuting that implementation shape rather than functional behavior.

Current prime_count retains signature and uses two shallow matches: outer Present(prime_content) versus Omitted|ExplicitNone, inner MathPrimes→Some(elem.count) versus other 119named Content→None. No guard, @ capture, clone, if-let, new helper/API or changes to other69sites/fixtures/order; two same-None complements are justified V19. No double validation/traversal, read count only for MathPrimes. Probe original70sites+new inner match:71selected E0004 GREEN versus0RED, no other errors, new MathAttachSlot breaks outer/new Content inner. Compare AST ordinals excluding only added prime_content scrutinee; final SARIF52V16 removed/71prescribed V19 added/one old prime_count V20 removed/no new V20, all other diagnostics retained. Receipt literal refinement authorizes only this extra local restructure.

Initial structural acceptance52V16/no70E0004→nominal barriers is narrowed by current71-match refinement, not ignored. Test guard-refused leaf/lined fraction and Quote unchanged. Replay only patterns/rustfmt/metadata/seals plus exact prime_count refinement; do not demand outside V16 removal. Full workspace/build/fmt/strict all-target Clippy+expectations/Rustdoc4indexes/V1-V5-V15-V26/reverse/diff, frozen provenance, 12GiB/swap0/jobs1/sequential. No new public API/phase/default/compatibility/dependency/parity or AI claim.

## Imported error traces and measured callback/reflection families

import-syntax-2026-10-17/legacy-expected-attempt/workspace/workspace.log records isolated p1306_named_module_contract_all_profiles failure at HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/uncommitted freeze validation-result. Legacy-module-probe fixture imports routes/error.typ, inner.absent gives `module `std` does not contain `absent`` at59..65. Four-profile legacy-module-probe.json freezes UTC/source-binary-fixture hashes/stdout/stderr. Historical9f2b6e8715237cb086899a42873660fe744622e8 and corrected local give Tracepoint::Import("routes/error.typ") at importer expression2..36, identical message/hints/origin; only known empty-file warning literal was discounted, remaining stderr identical. Old empty-trace assumption refuted, not upstream/test intent inferred.

Only this case's trace expectation changes: build expression Source once, require exactly one import trace/path2..36; all others remain trace-empty. Keep one Severity::Error, exact message, empty hints, error origin/range, empty side, four profiles, normal/reverse order/controls. No assertion removal/new public test/suppression/production/Rust contract. Shared source-validation and canonical modules owner govern policy. Focal+full regression/strict Clippy-Rustdoc/lineage/SARIF/new freeze12GiB/High10GiB/swap0/jobs1.

lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz, before-pdf-probe.json.gz preserves HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/tree/UTC/source/current bd5ebec285505889e8ff1d4f68804ab147a699f3, fields/admission/callback/init. paridade-novo-vanilla-2026-10-17/inventario.md E02–E04 discriminates carrier limits, not Rust dumps. Add language regressions: text/h/v/box/block getters, width/weak presence, bare text/bibliography/selectors, counter(math.equation)/alias, negative closure/partial/nonlocatable, state optional init; expectations frozen upstream, not implementation. Withdrawn first State proposal is not current; Heading independent causal controls remain. No new public/default/phase/compatibility in these tests.

State reader corrected review approved at `lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`, SHA 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4. Same-key reader init1/10 after+1 yields2/11; unread Func no execution/reused update per occurrence, pinned state.rs:226–245,419–455,477–520 document causal order. Five signatures/read phase only. Test functional direct update/stored method with inserted/reused content, creation executes nothing, init belongs each reader. State.display is Tekt extension, never vanilla proof; Set states/previous controls retain parity. Heading-supplement review approved SHA 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c at corresponding heading-supplement-runtime-2026-10-17/review.md:raw func/supplement presence/explicit numbering:none/ordered fields/ordinary Styled text opacity, negative absence preserved, no helper-mirror tests.

L0 first/functional RED/bilateral GREEN and Tekt-extension controls, no other public expansion; valid1:1+nuclei before seals, build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1-V5-V15-V26/reverse/no V16 increase/SARIF no new diagnostics, 12GiB/High8GiB/swap0/jobs1/sequential. debug0/incremental0 only validation environment; measure consumption, no presumed product benefit from representation.

## Content concatenation and public Sequence reflection

state-stack-2026-10-17/before/morphology-before/morphology-measured freezes same HEAD/tree/UTC/diffstat/source/currentpin:loops/+ nest local Sequence at every join versus upstream concatenated immediate children; debug-root.log stack-fails scanning64updates. Language form/failure, not mandated Rust storage/stack enlargement. Eval E2E for/while/+ /origin aliases uses language assert.eq(repr(...),...), valid oracle fixtures; do not require children before separate reflection repair. `00_nucleo/prompts/core/compiler/eval/operators/join.md` owns algorithm. Keep original depth/quantity/tests, RED→GREEN/full revalidation12GiB/swap0/jobs1, no public/phase/default.

sequence-reflection-2026-10-17/before.json.gz/before-sources preserve same HEAD/tree/UTC/diffstat/source/binaries/currentpin. Upstream content/mod.rs:724–728 requires Sequence.children, :538–585 has/at/fields; empty-fields/func/children, sequence-fields/children/single/for refute absent local fields/name empty. fields.rs:317–322,identity.rs:24,content_methods.rs:242 omit them. Language morphology/semantics, not Rust storage. Test Empty/single/multi/loops/persistent alias fields/children/has/at, missing-field errors named sequence; Text still lacks children/ordinary Styled stays opaque/empty Text stays text. Initial reflection slice alone did not certify Sequence construction/selector-show-counter/invalid-method generic message; later construction measurements supersede only its named debt. No new Rust field/type/trait/signature/phase/product mode; L0 first/real RED/regressions/full workspace-build-strict Clippy-Rustdoc/fmt/V15-V26-valid seal/reverse/no V16 increase/SARIF, sequential12GiB/High8GiB/swap0/jobs1, measure peak not benefit.

## Sequence construction, selection and ratified realization

sequence-constructor-2026-10-17/before/expanded.json.gz, before-sources freezes same HEAD/tree/UTC/status/diffstat/source/binaries/currentpin. Upstream content.rs:724–728/value.rs:644–650/elem.rs:573–604 defines required children/Content-string-symbol-none cast/generated constructor; local content_methods.rs:358–410 noncallable fallback, element_where/selector_matching:272–337 no Sequence, set.rs:55–76 transform-With-only rejection. Newly measured children() error already correct after getter repair: retire only that stale diagnostic debt, no diagnostic code change.

Test zero/single/multi/nested unflattened children, string/symbol/none, alias/With, casts/extras/positional-children precedence; selector/where positive/invalid names, negative With/homonymous closure; empty/conditional/extra/panic/partial set; bare/filtered/missing/Empty/outer-Styled show. Initial recursive show-once remains a measured refuter, never weakened for green. Baseline children()/error-before-panic controls remain. Construction slice completes earlier named construction/table-composition/internal-callback responsibility only, no new external Rust API/fields/trait/default/phase/compatibility/global show-guard/all-cast/all-element/render parity. Residual refuters retain report/owner L0 before fixes.

sequence-constructor before/expanded/controls/candidate.log eight initial RED, five early pass/show-recursion fail reflect flatten-before-selection/children-first realization/morphological nonlocatable stop versus upstream alias preserved/fresh equal-children rejected. Owner approved `lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`, manifest 46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327. Gate covers order before children/Table/math, no new public API. Retain show/recursion REDs/S1–S20. Correct only fixture conflating Text("") with Empty:use text("") positive empty-text control and retain non-equivalence to[]. controls.json.gz Symbol func symbol-versus-text/equalityfalse-versustrue remains separate debt, repr equality no morphology certificate.

r2s-measurement.json.gz show-panic/show-once/stored-value/show-block-bad-mutation refutes copying oracle depth fallback: upstream realize:394–403 engine.delay stores failure/returns Default=empty Sequence(engine:42–49/content:94–96), Sequence rule rematches fallback until secondary spanless depth error hides primary. Tekt deliberately preserves primary Source/errors `panicked with: X`/`panicked with: EARLY` and `variables from outside the function are read-only and cannot be modified`. Do not return Empty to emulate defect. Keep four individually classified deliberate divergences/exact errors, not blanket exit/divergence suppression. Truly reconstructed equal-children Sequence still rejects `maximum show rule depth exceeded` at unchanged limit; test both causes/origins. ADR0107/0108 requires measured-source classification, not preference for candidate/Rust equality.

Keep prior expectations, RED→GREEN/bilateral corpus/test alternatives for V19, build/workspace/doctests separately, strict Clippy-Rustdoc/fmt/V1-V5-V15-V26/reverse/SARIF no new diagnostic, valid graph before scoped seals, 12GiB/High8GiB/swap0/jobs1/sequential, debug0/incremental0 validation-only, exact provenance/no publish-release.

## Pending text packages: single realization and real children

sequence-constructor-2026-10-17/r2s-scope.json.gz/text-before.json.gz preserves same HEAD/tree/UTC/diffstat/binaries/currentpin:textual-no-repeat/baseline/interpolation/captured AAAA(B)versusAA(B); textual.rs:104–122 nests same-snapshot pending groups, realize:182–210 bottom-up repeats literal. Cross-piece AB→X and outer-hides-textY/no child panic match both.

Within ratified single-group scope, textual owner coalesces only pending packages with same causal rule Arc/collapsed styles/guards/file, retaining Source/order/morphology/different-environment and nontext-element-effect boundaries. No callback in coalescing. Capture coalesced group; Sequence owner coalesces textual body after actually emitted sequences realize, before returning textual recipe. No one-rule-per-transport reapplication/indiscriminate Realized skip/matching-precedence-scope/public change. Test literal/regex interpolation-capture, identity Sequence simple/block, lexical prefix, cross-piece matching/hidden text; distinct snapshots opaque. Independent local-text-scope CversusB precedence was not part of this optimization and is superseded only by separately ratified textual matcher below.

empty-slots-before.json.gz freezes source/binaries/UTC/HEAD/tree/diffstat/currentpin:body EBODYversusBODY, none/[]children EEAversusEA, text rule extraEs. markup_stream:205–225/textual:104–120 includes transport Empty slots, sequences:114–125 visits as real. First narrow repair removes bare Content::Empty slots only in markup-tail/pending-package construction; explicit occurrences already captured Source/ShowRecipe, empty Text/constructor child arrays must survive, no arbitrary domain visitor filtering. Later explicit consumption mask below refines proof of which slots to remove; it does not erase genuine bare Empty by content. Full body/[]/none/emptyconstructor/childnone/child[]/prefix/textual/no-body PDF corpus, prior RED/full PDF+HTML validation separately.

validation-attempt-2/trace-recipes/trace-markup2/trace-textual2 preserve11green/twoRED; eof-before.json.gz uses exact source(no appended newline):Styled+EOF Yboth, without EOF ABboth. Unit Y fixture must include measured EOF, not relax show; keep both external probes. Alias without EOF localAAAAAAAAB/upstreamAAB:sequences:112–116 skips every package root though Content::sequence singleton becomes actual ShowRecipe; skipping it leaves original pending and recaptures children, trace-textual2 proves opaque marked recipe. skip_root only skips artificial raw Empty/Sequence root, never normalized ShowRecipe/Styled/other root; decide actual marked root before children. Keep strict causal styles/rules/guards/file and slot controls, no public type/field/API/phase; archive/remove temporary cfg(test)logs before final validation.

validation-attempt-3/emission-after.json.gz freezes tree/UTC/source/argv/binaries/diffstat/currentpin:[A]/text("A") emitted by Sequence under A→AA givesAAAAversusAA; native text without Sequence alreadyAAAA in archived 2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c(native-before). textual.rs:142–161 mistakes Sequence transport mark/guard as textual-scope distinction; trace-textual2/instrumentation-sources preserves fields.

Sequence's crate-internal helper compares textual transport excluding only eval.sequence.occurrence collapsed-delta key and guards whose RuleId is a Sequence rule in snapshot; unknown/other-family guards and all actual styles remain. TextualSnapshot::same_group lexical grouping stays strict. Pending coalescing needs same rule Arc/file/equivalent textual transport. Remove generic nontext scalar recipe wrapper only if raw body Text(Source allowed), ordered RuleIds exactly parent, file/transport identical, and no element rule matches Text. Then no element callback is lost, parent already owns literal/regex; return body with Source. Never eliminate Styled/structural/context recipe/applicable-element rule, remove Engine rules, execute callback during coalescing or compare only name/Debug/product text. RED→GREEN emitted[A]/textA/native-scalar control, other-family guard/real-style opacity, all13prior controls unchanged. No public type/field/phase/other-element policy change.

## Markup values, consumed slots and None insertion

sequence-r3-2026-10-18/before.json.gz/before-sources freezes HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/tree/UTC/status/diffstat/source/executables, current bd5ebec285505889e8ff1d4f68804ab147a699f3/treecac836c07e4dec2a89219370428292393a3216b5/oracle9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9. [#([A]+[B]) C] exposes 4local children A/B/space/C versus3upstream Sequence(A,B)/space/C; alias/two siblings/later+/for same loss, singleton and show-identity-preserved body match. append_markup_sequence:286–311 always recurses Sequence even preserve_source ContentBlock; upstream markup:78 appends entire display value/content:240–250 normalizes0/1only. Morphology, not Vec/Rust sharing mandate.

Value-child collection versus realized-flow normalization must remain distinct. Do not change Content::sequence, concat_content, alias/Source/local rules/preflatten capture/Table-Cell/style-label-snapshot indices/space normalization. Refute collection hypothesis if immediate children unrestored or singleton/+ /visual changes. sequence_r3_markup tests children/func/fields/language equality/immutable alias/two siblings/singleton/later+/for/flow, not Debug/PDF bytes; markup owner's unit tests collection/flow mode/origin with no callbacks/general-helper change. Functional RED first/GREEN/CJK/show/text/label/Source/Strong64-256 recursion/full strict validation12GiB/High8GiB/swap0/jobs1/sequential/no publication.

sequence-r3 before/markup-after/textual-before/transport-before/transport-before-sources preserves same state/currentpin:nonmatching Z show nested.children exposes synthetic Sequence+twoEmpty locally versus original three children; empty-child package+Empty versus[]/A. markup_stream:201–249 reconstructs slots zeroed by textual:103–129/matching:90–95. None/CodeBlock insertion disappears:[#none#[]A]2childrenversus3, X#none; B under showEmpty XBversusXEB. Historical generic display omits None, upstream value:191–210 displays Content::empty/markup:39–56 handles set/show own boundaries, :78 appends other expressions.

Use private Vec<bool> consumed-slot mask, no new public carrier/false Location/Rust API/mode/default/phase. Genuine Empty/Text-empty loss, changed scope/boundaries, repeated callbacks/inconsistent indices refute. None presentation changes only markup insertion, not generic Math/CodeSetTail helper; SetRule/ShowRule retain distinct syntax boundaries. sequence_r3_transport tests real children/presentation/nonmatching recipe, []/text("")func text, literal-let-assert-conditional None, showEmpty(None), emptyset/style without artificialEmpty. Shared helper gains no relaxed expectation/normalization hiding slots; expectations from pairs. RED/GREEN/bilateral/R2-S-EOF-slots-styles-CJK-loops/full separate PDFHTML strict gates12GiB/High8GiB/swap0/jobs1/no publication.

transport-after.json.gz confirms realNone/consumedEmpty fixed but style-tail still synthetic Sequence(A,Space) child before Styled/rule-tail form differs beyond repr. R3-TextualGroups must complete synthetic-group multi-tail language projection/equality/selection while preserving author Sequence; R3-Reflection must complete Styled/Styles/label repr; A3/include owns module/include value form; R3-Symbol owns Symbol; no generic flatten/closure claim or intervention without owner L0. Earlier literal-regex precedence/show-set debt is superseded only by ratified matcher below, not by transport. These remaining reflection/groups/attribution limitations stay active.

## Hanging-indent type and bibliography boundary acceptance

bibliography-indent-2026-10-18/before.json.gz has 60pairs at same HEAD/tree/UTC/status/exact diffstat/source/argv/binaries/text/boxes, before-sources full, currentpin/tree/oracle above, Tekt97f6fa3078bdd6c40aabb80d6f93f3fd019b405ce70fbf2266686d8fb42271cb. Untitled MLA starts citation line(Silvax41.84/y5.168)versusx8/y27.463995; titled local merges/interleaves(y6.7568/y8.108)while upstream separates. Eight positive paragraph indents missing, ten none/auto/int/ratio/string set/constructor wrongly accepted versus expected length. bibliography:106–123 title/cache inline, set:1180–1192 unknown, par:173 ignores. Upstream layout/rules:463–531 separates title/entries, unprefixed inset+compensation, prefix two columns, par:431 Length/inline:146–148 absolute initial advance. Position/type/hint/span controls refute block-boundary/validation hypothesis; one offset never proves all indentation or extraction difference.

Add canonical existing regression-module/MockWorld tests set/constructor direct/alias invalid Length types, including error-before-missing-body, exact message/public type/hint/value span; accept0/negative/abs+em. No duplicate World/new owner/weakened existing tests/geometric-indent claim. L0 merge must be additive with textual-realization clauses, never restore old complete candidate over this scope when approving another package. Internal continuous parity, no public field/trait/API/default/flag/compatibility/phase. This boundary/type slice alone incomplete: A2-Structure retains CSL hanging_indent/prefix/body+margins/columns, A2-HangingTransport handles per-occurrence overrides/priorities, A2-ParIndent ordinary set-constructor strategies/directions, A2-Multi attribution/cache. Do not silence incomplete-support warnings before consumers or call positive indent fixed from negative rejection.

Functional RED before code/GREEN title-citation-body-fallback/types+zero-negative-affine-empty controls, remeasure PDF boxes/extraction, retain indent/multiple-bibliography REDs, no unsupported Tekt advantage. Full build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1-V5-V15-V26/reverse/SARIF no new diagnostics, 12GiB/High8GiB/swap0/jobs1/debug0/incremental0/sequential, no commit/push/release. Other owners' later individual completion requires their own evidence, not blanket closure here.

## Ratified textual matching, styles and final observer refinements

textual-realization-2026-10-18/before/scope/refuters/before-sources freezes HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/tree/UTC/status/diffstat/source/L0/argv/binaries/currentpin before candidate code. matching:118–150 reverse regex then declaration literals, :194–207 reexposes products; textual:102–123 captures whole snapshot, realize:177–217 bottom-up, apply:620–648/matching:218–225 evaluates functions without child capture, expression_dispatch:125–129 separates insertion_rules/local state. Measured local/upstream:identical literalsC/B, literal-regex tieR/L, overlapX/YB, first effectLATER/FIRST, inverse chainBBBB/BB, separate productAB/X, product+siblingX/AB, hidden Strong-child panicHIDDEN/Q, oldglobal-newglobalL/R, capturedlocalL/L, CJK水\n果→水果/X. Pinned selector:110–125/realize:1328–1405 unifies earliest match/product revocation; scope/root-first cause remains refutable by acceptance/equality/fields/style/query/error controls, not buffer/byte mechanics.

Human approved manifest 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302/pinned textual-recipe-frontier. Narrowly supersedes provisional ContentBlock show/pending-recipe bottom-up, not ordinary expression eval order/program once/State-Counter laziness/render-morphology distinction/public Rust API. Test each acceptance.json family before code, global/local returned values, latest global/tie-earliest UTF8/Unicode/cycles/alias-versus-composite/adjacent-hidden products/style confinement/next rule within product, all error-identity-morphology controls. Frozen oracle expectations not candidate; keep all previous R2/R3 controls and individually deliberate primary-error-versus-oracle-depth differences, no new-algorithm failure relaxation.

Candidate 1 visually matches eight color sets but observer Rust equality confuses named/sRGB/f32 rounding. In these eight new tests convert Color::to_srgb channels/alpha to u8 and compare exact measured oracle SVG#0074d9/#ff4136. Confirm SVGs, keep product/colors/global tolerances/confinement unchanged. Candidate 3 passes 64probes, refutes old Par observer stopping at wrapper: count only Realized.visual and Source/Located bodies, exactly onePar/full text, never original emission. Add refuter-fragment.rs prepared-environment controls: text/Strong rules, success/error, reemitted alias, callback once, restore styles/rules/guards/file/defer/realizing/frames/depth/text support, local state only. Existing Raw/Table integration REDs retain expectations.

Ratification plus eight passing bilateral color probes narrowly supersedes two historical literal/regex show-set rejection tests: they now accept with full text, style proof remains eight sRGB/SVG controls. Other13candidate4 integration REDs unchanged. candidate5b-skip-reuse exposes wrappers hiding expected visual: only has_dynamic, styled_bold_envolve_heading, contem_heading, styled_bold_anywhere, styled_bold_e_weight, and go in texto_em_strong/emph add Source.body/Located.body/Realized.visual traversal, never original, retaining every nominal complement/content-style expectation. This measured observer exception supersedes earlier no-traversal-expansion rules only for these helpers.

Before revising show_rule_regex_ultima_declarada_vence, regex-oracle.json at current bd5ebec285505889e8ff1d4f68804ab147a699f3 freezes full binarySHA/source/argv/UTC/state and exact HTML `<p>abc<em><strong>123</strong></em>def</p>`. Latest rule gives outerEmph, earlier still acts inside product; require both Strong/Emph/full text, not vanished Strong/Rust wrapper equality. texto_em_strong also traverses Emph.body and texto_em_emph Strong.body solely to observe that measured nested product. Preserve language nodes, no original-as-emission count.

All textual acceptance families/R2-S/R3/single-eval controls require RED→GREEN, PDF/HTML validated separately, full build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1-V5-V15-V26/reverse/SARIF no new errors-warnings, 12GiB/High8GiB/swap0/jobs1/sequential, frozen provenance, no suppression/commit/push/release or unmeasured RAM gain.
