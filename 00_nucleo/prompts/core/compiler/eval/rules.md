# L0 — compiler/eval/rules
Hash do Código: 5571fe65


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:41e38095ab33ab1710ba11f43d6a5ca50def5a703f77a0001a066b72964c4165
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6


## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/rules.rs:43` — `fn type_mismatch`.
- `01_core/src/compiler/eval/rules.rs:59` — `fn expected_length_error`.
- `01_core/src/compiler/eval/rules.rs:87` — `fn edge_cast_error`.
- `01_core/src/compiler/eval/rules.rs:119` — `fn pdf_pages_found_name`.
- `01_core/src/compiler/eval/rules.rs:135` — `fn validate_pdf_page_range_element`.
- `01_core/src/compiler/eval/rules.rs:213` — `fn pdf_standard_expected_list`.
- `01_core/src/compiler/eval/rules.rs:231` — `fn pdf_standard_and_list`.
- `01_core/src/compiler/eval/rules.rs:242` — `fn pdf_version_display`.
- `01_core/src/compiler/eval/rules.rs:248` — `fn pdf_standard_version_hint`.
- `01_core/src/compiler/eval/rules.rs:262` — `fn pdf_validator_label`.
- `01_core/src/compiler/eval/rules.rs:270` — `fn pdf_validator_range`.
- `01_core/src/compiler/eval/rules.rs:290` — `fn pdf_standards_conflict_diagnostic`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `src/space_discarding.rs:53-73,194-256`.
- `text/smartquote.rs:38-89,335-419`.
- `typst-library/src/layout/page.rs:764-772`.
- `typst-library/src/layout/page.rs:813-821`.
- `typst-realize/src/spaces.rs:114-127`.

## Contract

This facade owns generic diagnostics, static bindings and historical tests.
The following set/show, validation, style-delta, rule-application, realization
and interception clauses are canonical child boundary criteria, not a second
implementation. Preserve lexical precedence, innermost-first
application, RuleId revocation/active guards, current casts, warnings, spans and
first-error behavior. Unknown named arguments are never silently ignored.
Show-set transports Styles; transformations preserve semantic morphology.
Delegate selector semantics to
00_nucleo/prompts/core/compiler/eval/selector_matching.md and one-node termination to
00_nucleo/prompts/core/compiler/eval/show_rule_termination.md; do not duplicate either
algorithm. Native Element/Where follow the node path, not literal/regex slicing.
The special paragraph routes use the typed native-paragraph predicate without
expanding to all elements/compositions or applying text/element rules twice.

## Textual realization delegation

The descendant owner `00_nucleo/prompts/core/compiler/eval/rules/textual.md` specifies
causal snapshots, text-node interception, literal/regex grouping and slicing,
space normalization and Unicode nonspacing classification. This parent keeps
set/show routing and one-node application; it neither duplicates the child's
algorithm nor absorbs its consumer. The eval coordinator calls the child under
the same phase and signatures as before the extraction.

## Lexical and synthesized content

Capture immutable lexical rules/styles for an OutlineElem during the renderable
interception pass before applying outline transformations, not from a final global
stack or by inventing snapshots in the pre-show pass. realize_outlines receives
the generated entry, causal snapshot and private parent styles. Apply the normal
matcher/precedence/guards once; absent transformation, use the entry default composed
from its helpers and causal link. Each fixpoint candidate is rebuilt from its own
source, not cumulatively transformed. Do not execute callbacks in pure layout or
insert realized entries as new query occurrences.

Set outline(indent:) accepts length/function/auto and the retained crystalline bool
compatibility. Set outline.entry(fill:) accepts content/none. Store validated
values lexically; explicit constructor values win. Materialize inherited indent
on outlines and fill on direct entries only when omitted. Synthesized entries
receive effective fill. A visually neutral carrier with outline.parent.indent
and outline.parent.key preserves the exact parent for contextual content returned
by a transformation, without changing matching/query order or phase.

For RawElem, synthesize raw.line only when a currently applicable typed rule targets
those instances. Split newline preserving empty segments and ordered one-based
number/count/text/body, run the normal show engine once per line and recompose
with line boundaries. Without such a rule, preserve RawElem/highlighting/default
intact. Manual raw.line uses the same dynamic carrier and show machinery; filters
read its real fields. No alternate guard or precedence algorithm is introduced.

## Set-rule schemas and transport

Keep the existing heading/page/figure/text/par/table schemas and their defaults,
casts and warning policy. Font dictionaries delegate to
00_nucleo/prompts/core/compiler/eval/font_dict.md. Table numbering retains lexical
table.numbering string/none capture, not the cell-cast owner. Smartquote accepts
enabled:boolean, alternative:boolean and canonical quotes string/array/dictionary/
auto forms. Existing crystalline none compatibility normalizes to auto and is not
claimed as vanilla surface. Validate the whole delta before applying it.

Recognize par.line structurally with exactly numbering (string/function/none),
number-align (horizontal alignment/auto), number-margin (left/right/start/end),
number-clearance (length/auto), numbering-scope (document/page). Store validated
par.line.* values; omission is neutral and unknown names fail. Keep numbering
Func identity/span intact: do not call it, convert its return or fabricate a
physical-line marker during set evaluation. Late execution/request handling stays
with its pipeline/layout owners.

Math set schemas delegate domain casts to the same canonical constructor domain:

- frac.style: vertical/skewed/horizontal stored as canonical Str, with explicit
  element value above cascade above vertical default; binom does not consume it.
- accent.size/dotless: relative length/bool, defaults 100%/true.
- cases.delim/reverse/gap: delimiter pair/bool/relative, defaults {}/false/0.2em.
- mat.delim/align/augment/gap/row-gap/column-gap: canonical casts. gap feeds both
  axes; specific gaps win at the same level. Defaults are ()/center/none and
  row 0.2em/column 0.5em. Explicit augment retains its offsets while folding
  cascaded stroke; explicit none clears inherited augment. Inside augment only
  hline/vline/stroke are consumed; other inner keys are tolerated, but invalid
  consumed values and unknown outer named arguments still fail.
- stretch.size: relative length with default 100%.

For all of these, explicit element arguments win over cascade, and cascade never
sets morphological presence bits or appears as fabricated explicit fields/repr.
Unknown/duplicate names and invalid values fail before effective style mutation.

Recognize dotted format targets structurally, not through an empty text_str.
Empty rules are neutral; omitted fields never insert defaults or erase an earlier
lexical decision. Preserve independent fields, schema diagnostics and ordering:

- format.svg: optional pretty Bool -> format.svg.pretty (default false only when
  the document makes no decision).
- format.png: optional ppi Int/Float -> Float at format.png.ppi. Omission does
  not insert 144.0. Finite zero/negative values retain their measured acceptance;
  nonfinite cases with an aborting oracle do not acquire a parity claim.
- format.html: gate Html at field html before evaluating any argument. The exact
  disabled diagnostic is ``cannot access field `html` because the `html` feature
  is not enabled``, with ``try enabling the `html` feature`` and
  `see https://typst.app/help/compiler-features for more details`.
  Active calls evaluate argument expressions before ordinary schema validation;
  optional pretty Bool is stored at format.html.pretty.
- format.pdf is ungated and accepts tagged Auto/Bool, pretty Bool, pages and
  standard independently under format.pdf.*. Invalid tagged uses
  `expected boolean or auto, found <type>`, not the bool-only message. Unknown,
  duplicate or positional arguments fail normally, not as unsupported-target
  warnings; preserve absence of partial effective deltas.

PDF pages validation is complete during set evaluation. Accept positive Int,
from/to dictionary, array of Int/dictionary, or explicit None. Store the original
validated Value; None is a present reset distinct from omission and empty array
is a present empty selection. Preserve `page numbers start at one`,
`page range must not be empty`,
`unexpected key "{key}", valid keys are "from" and "to"`, and
`start of page range cannot be larger than end`. Top-level mismatch expects
integer/dictionary/array/none; array-element mismatch expects only integer/dictionary.
Nested array/None elements are not accepted. The typed reconstruction belongs to
00_nucleo/prompts/core/compiler/layout/format_options.md and fails closed on impossible
already-validated state.

PDF standard accepts a canonical PdfStandard string or array of strings, including
empty array, but not None. Use the exact closed name list from
00_nucleo/prompts/core/entities/document_format_options.md. Unrecognized strings omit
the found-type suffix; wrong-typed values include it. Array-item diagnostics omit
the array alternative and retain the final 'or' list form. Validate PdfStandards::new
and map its structured conflicts to the existing precise messages/hints; normalize
a single string to a one-item Value::Array, retain an original array, then store.
This provides language parsing/conflict validation, not proof of PDF structural
conformance. Preserve distinct conflicts for version/PDF-A/PDF-UA multiplicity,
empty intersection and explicit version outside the allowed range.

This owner does not choose top-level/container boundaries, CLI precedence,
pages/tagged/standard interactions, rasterize, format SVG or export HTML/PDF.
Those retain their format-options and pipeline owners. Feature enablement remains
independent from target selection under the pinned nucleus.

## Paragraph evidence before decisions

All following corpora are historical measurements against pinned vanilla
`9f2b6e8715237cb086899a42873660fe744622e8`, not certification of a later target.
They measure language behavior/types/geometry, not historical prompt intent or
required upstream frames/algorithms. Preserve their initial.json/before/source/
binary/command provenance.

- Hyphenation-policy corpus correcao-politica-hifenizacao-2026-09-28/before-pdf/
  resultados.json:52 bilateral entries/source/commands/boxes at initial.json.
  Tekt absent/auto/false/true equivalent. Pin rejects hyphenate int/string,
  justify int/auto, linebreaks outside simple/optimized/auto; literal errors/spans
  in stderr. rules.rs:2972–3010/3407–3420 drops options, par.rs:119–126
  accepts/ignores, break_selection.rs:39 ignores policy; pinned text/mod.rs:
  545–565 Smart<bool>, par.rs:227–242/627–636 justify/Linebreaks.
- Composition corpus correcao-composicao-paragrafos-2026-09-28/before-pdf:
  34 entries at initial.json/before.180pt/8pt margin: justified first-line end
  145.52pt Tekt versus172.000002pt pin, last110.048pt both.
  BEFORE/par[MIDDLE]/AFTER two physical lines versus three; empty-start AFTER
  yMin3.74 versus18.14, empty-auto height24.568 versus38.968.
  native-leading-two-lines proves leading scope closes before style restoration.
  line_commit.rs:195–236 already justifies; word_layout ordinary flush,
  layout/mod.rs:1469 transparent Par, style_scope.rs:18–77 restores after body,
  rules.rs:663–705 groups show flow. XML groups are not physical lines; coincident
  coordinates measured separately.
- Selection corpus
  `lab/results/construction/diagnostics/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json`:
  32 entries, exact initial.json/before.140pt/8pt margin: simple chooses
  delta epsilon zeta eta; optimized delta epsilon zeta, eta next line; Tekt ignores
  strategy.240pt justified optimized fits eta by shrinking spaces.
  rules.rs:3070–3100/par.rs:131–150 validate/discard; text.rs:41/word_layout.rs:29
  incremental; pinned inline/mod.rs:194–196 resolves auto by justify,
  raw_ratio/raw_cost permits shrink1/3/cubic-quadratic costs, Limits::SPACING_DEFAULT
  2/3..1.5. Adjacent styles/literal edges/opaque content separately measured.
  Inference that explicit L0 scope-outs caused deferral is refutable, not intent
  proved by observation.
- Global hyphenation correcao-hifenizacao-global-2026-09-28/before-pdf:
  67 entries, initial.json/before/proveniencia.json complete. paragraph_prepare.rs:
  80–94 rejects true, paragraph_select.rs:66 word ends only, paragraph_plan.rs:18
  uses transported strategy, break_selection.rs:41–66 local overflow only.
  Pinned linebreak.rs:170–209 shares greedy candidates; :626–684 penalty
  135*(1+0.15*(max(5-left,0)+max(5-right,0))) plus135 for consecutive hyphens;
  shaping.rs:583–635 draws '-' while retaining SHY in copied text.
- Hardcode corpus correcao-hardcode-paragrafo-2026-09-28/before-pdf:
  20 entries, initial.json/before. rules.rs:3483–3525/text/constructor.rs:153–190
  omit overhang, wrongly accept auto/int/string. Pinned text/mod.rs:391–393 bool
  default true, inline/line.rs:518–525 compensation from final style;
  paragraph_hyphen.rs:221–224 always scales0.55. Initial suspicion about
  paragraph_select.rs:78–82 was refuted before code: pinned linebreak.rs:551–559
  uses attempt.justify disabled for mandatory by line.rs:138–139. Keep formula/
  test natural final line.
- Custom-cost corpus correcao-custos-paragrafo-2026-09-28/before-pdf:
  66 entries at initial.json/before. Pinned text/mod.rs:1507–1564 validates Dict/
  component Ratio/partial composition, inline/mod.rs:240 reads shared costs,
  linebreak.rs:29–30,961–962 defaults135/100 times ratio. Tekt set drops costs,
  constructor accepts without transport, hyphen/select use fixed defaults.
  Literal errors/spans including accepted negatives remain stderr.
- Regional widow/orphan corpus
  `lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/before-pdf/`:
  78 entries plus before-extra24, full initial.json/before. page-65-100%-0%-False
  splits3+1 versus2+2; zero costs match. Tiny/large-positive protect equally.
  Pinned flow/collect.rs:196–229 tests strict positivity/first-two/last-two/
  whole-three; distribute.rs:343–364 advances grouped need only if next region
  fits. paragraph_emit.rs emits singly without grouped need. Observable line/word
  distribution, not upstream Child/frame structure or old-scope intent.

## Current paragraph schema and scoped realization boundaries

These internal parity corrections use continuous ADR-0127, no new public
signature/entity/trait/flag/phase/compatibility. L0 first/RED→GREEN/restricted
owner reseals, preserving pinned shared nuclei as invariants rather than code
owners. The former atomization ban on honoring discarded properties is superseded
only by the individually measured corrections below.

Set text validates hyphenate Bool|Auto with `expected boolean or auto, found <type>`
at expression span, custom text.hyphenate. Set par validates justify Bool,
custom par.justify; true warns factually about still-limited selection, not absence
of implemented space justification/global selection. False gives no warning.
linebreaks accepts Auto|Str(simple|optimized); invalid string:
`expected "simple", "optimized", or auto`, wrong type adds `, found <type>`.
After valid cast transport par.linebreaks including Auto. Empty/omission preserves
prior decision. Keep show-set/style restoration/other fields.

first-policy-green.log passes five policy/transport tests but four message failures:
type_mismatch says int versus pin integer; existing expected_length_error:65 uses
correct vocabulary. For these new casts use private boolean/integer/string helper,
never global type_mismatch/public type_name change. Constructor text's exception
supersedes only mandated type_mismatch reuse for hyphenate.

realize_flow treats Par/Styled(Par) as boundaries, closing current segment before
inserting without extra encompassing Par. Par body/inline Styled stays inline
Sequence, no synthetic Par merely for bold/italic. Private contextual recursion
keeps other bodies/containers/math on existing paths; no string-selected strategy
without consumer. The later certified subset supersedes initial warnings that
optimized/global preparation is entirely absent.

### Causal set scope and whole-flow style

precedence-state.json/regression-acceptance.json passes191 checks, fails two
show-par-sets-false: Tekt extraordi-/interna- versus unhyphenated pin.
show-par-full-tree.log puts SetPage/global scope inside synthetic Par, with recipe
outside hyphenate true. eval/mod.rs:1532–1560 knows #set origin; rules.rs:660–727
lost distinction by treating every Styled inline. Never infer origin from Styles.

eval_markup fold marks #set Styles with private eval.flow.scope=true. Before first
material paragraph realize_flow recognizes marker, keeps scope exterior to
paragraph realization and prior SetPage/Empty/Space/SpaceFromNewline in order,
unwrapped by Par. After material inline content, scope remains inline in current
paragraph. Unmarked constructor styles stay inline except whole-flow refinement
below. Hub only produces marker; realization owner prepares/recognizes it.
No public contract/default/phase change or style-key heuristic.

precedence-pdf/resultados.json/proveniencia.json measure eight old-binary controls.
native-text-inline-override/mid-par-set-override hyphenate in both (open '-' versus
U+AD difference); native-text-only-override only Tekt hyphenates, refuting every-
constructor-inline hypothesis. Whole-material Styled in not-yet-material flow
supplies initial paragraph scope so show-set applies inside. If material lies
outside wrapper before/after, wrapper stays inline. recognize Styled with prefix/
suffix containing only Empty/Space/SpaceFromNewline/SetPage; structural factorization,
not style-field inference. Keep carriers/order/realize body in existing outer
scope. This supersedes absolute constructor exclusion only for whole-flow case.

whole-style-first-failure.log has two singleton hyphenations versus zero:
realize_node retains Text directly, factorized whole Styled then had no Par for
recipe. Private realize_scoped_flow crosses Styled, wraps singleton Text in Par
through inline realization, delegates other flow/containers to realize_node.
Do not wrap Heading/blocks in Par. No public signature/phase change.

## Certified global selection and hyphenation boundaries

Initial global preparation certifies whole Latin paragraph of Text/Space/
SpaceFromNewline/Empty/Sequence/typographic Styled/Strong-Emph, LTR, no tracking/
math/smallcaps, finite constant width, outside columns/Cells/floats. Whole input
certified before emission; rejected unit preserves old fallback. Later uniform-
word hyphenation extends earlier hyphenation-disabled subset only to Latin words
with uniform style/policy and simple terminal punctuation. Mixed within-word
styles/internal apostrophes/other UAX14/objects/variable widths retain fallback.
Full UAX14/links/explicit linebreaks/custom spacing bounds/remaining incremental
selection are incomplete, no state-of-art/all-language optimization claim.

Copied '-' versus SHY remains independent export limitation. Geometry draws '-'
separately as measured; acceptance measures words/positions/styles/lexical policy,
normalizing only break hyphen for geometric comparison and reporting textual
difference separately. Preserve literal textual hyphen in hardcode correction:
requires own carrier/contract, never glyph-position heuristic.

Partial-support warnings describe uniform Latin/constant-width subset and remaining
incremental cases, never discard valid casts or claim no global/hyphenation support.
Normative factor0.55/base135/distance5/increment0.15 come from named pinned source;
identify meaning/reference, no claim renaming constants implements config. Costs
subsequently follow measured custom-cost contract below.

Capture overhang Bool as text.overhang; reject other types with existing
`expected boolean`/causal value span helper. No dropped-property warning for valid
overhang. Geometry stays certified subset, no common-punctuation/full-fallback promise.

## Shared custom text costs and regional protection

Private `00_nucleo/prompts/core/compiler/text_costs.md` owns cast/component
transport/shared resolution. Hyphenation/runt global plan reads configuration;
widow/orphan validated/transported with influence only certified regional subset.
Fragment costs never replace shared outer paragraph costs. Four validated
channels are glyph-neutral in Styled preparation whitelist. Whole-flow wrapper
factorization enters chain before Par. Variable-width limitations remain;
never infer costs from glyph; preserve defaults.

Set costs uses shared parser before chain mutation, pushes only supplied canonical
components, error at dictionary span. Widow/orphan warnings must describe current
limited regional support rather than wholly absent influence; hyphenation/runt
are not discarded. Hint says columns/notes/mixed content remain outside. No new
constructor Sink API.

after-first-resultados.json/proveniencia.json has66 first-cost-binary entries,
only restore/inline valid cases differ. Pinned shared.get(TextElem::costs) at
inline/mod.rs:240; realization factors whole Styled before Par. prepare.rs:45–78
rejects new glyph-neutral channels, causing incremental divergent choices.
Absolute exclusion refuted: allow only text.costs.hyphenation/runt/widow/orphan
in Styled whitelist. Temporary preparation push restores before shared plan read;
partial inline never chooses paragraph cost, whole Styled enters before Par.
No space/word boundary from cost differences. Preserve regional/out-of-subset limits.

Regional correction supersedes widow/orphan scope-out only for prepared paragraphs
with uniform vertical metrics/no baseline offset, constant paginated geometry,
no columns/Cells/subframes/notes/floats/active regional transaction. Else fallback;
no full algorithm/performance promise. Private
`00_nucleo/prompts/core/compiler/layout/paragraph_pagination.md` exclusively owns
vertical certification/grouping/regional admission. Receives selected ranges,
no reselection/glyph emission/new_page duplication. Whole certification before
emission: all fragments size/edges/leading match outer style (bold/italic allowed
if metrics equal), finite nonnegative top/descent/leading. Auto bounds need no
protection. Honor initial baseline/existing block-chain collapse before real need
versus page_bottom_limit. Next space uses resolved next-page margins. Advance only
with existing current-region material and group fitting next. Oversize groups make
ordinary progress/content intact, no repeated-new_page loop.

text_costs resolves widow/orphan independently of hyphenation/runt weights via
FIELDS, default Ratio1/strict>0. Ordinary paragraph with omitted linebreaks plans
Simple, enabling default protection; justify-based auto/other refused contexts
stay. Emit without changing justification/carriers/open final line/boundaries/
lexical restoration. Preserve notes/columns by not applying incomplete protection.

Acceptance: unchanged words2+2 versus3+1, whole-three protection, independently
disabled orphan, oversize group/restoration/default without flag. Bilateral boxes/
words/pages, prior types/spans/fallback controls. XML grouping is not physical line
count. Restricted reseal/build/crystalline-lint without errors.

## Static behavioral boundaries and lineage

The historical uncommitted extraction snapshot in
`lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz`
contains4754 consumer lines; L1 passed6401/two ignored, no benchmark/new bilateral
comparison. This owner retains generic diagnostics/bindings/tests. Canonical
exclusive owners under `00_nucleo/prompts/core/compiler/eval/rules/` are pdf,
math_set,realize,apply,outline,intercept,set,show, and independent textual.
Historical clauses map their surface as boundary criteria, no duplicate production.
No public/default/phase change from extraction. Realization consumer reuses
apply_show_rules, not duplicate matching/termination. Captured-text-group service
is crate-internal/reusable by realization.

Measurements/shape-RTL refinement in
`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18`
precede shared selection-nucleus pin. This owner receives only pin; executable
code/API/phase/other responsibilities unchanged by pin refinement.

## Closed custom-style fixture observation

### Evidence before decision

Source inspection at HEAD `059d1a0ff` on 2026-10-19:
`01_core/src/compiler/eval/rules.rs:317–338` visits Styled and Sequence, updates
existing keys in place, and silently ignores every other Content variant.
`01_core/src/compiler/eval/rules.rs:340–346` uses this observer to inspect custom
style values from evaluated fixtures; callers supply set-format rules followed by
plain text. A Styled target hidden inside another wrapper is outside this
observer's domain. This is a bounded test observation, not language inheritance
or a general content traversal. The source reading establishes observed behavior,
not upstream intent or proof that every wrapper contains no styles.

### Decision and acceptance

Keep this observer local to tests. Styled updates existing keys without changing
their encounter order and then descends; Sequence visits children in order.
Every other current nominal Content alternative remains opaque and contributes
no keys. List these alternatives at the actual match; no wildcard, conditional
disguise, generic walker, suppression, error substitution or deleted assertion.
The result describes the supported fixture paths and must never certify global
absence of styles. A new causal wrapper in an evaluated fixture requires a
separate observer-scope review before its output can certify that fixture.

Before implementation, require independent direct and nested Styled controls for
inner-over-outer precedence, Sequence encounter order, independent custom keys,
and style-free terminal preservation. A decoy Strong wrapper with a hidden Styled
target contributes no keys, while a following supported Styled sibling remains
observed; assert both so opaque traversal cannot silently erase the positive
control. Retain every existing format-rule test. Changed precedence, entering the
opaque decoy, or loss of a supported target refutes this bounded repair.
In a disposable real workspace, adding a Content sentinel must produce E0004 at
this exact observer after the change, whereas the old wildcard absorbs it.
Record source state, UTC, commands and results. Internal test-boundary correction
uses ADR-0127 continuous flow after sealing this owner; no public API, default,
phase or compatibility change is authorized.


## Existing private paragraph-preparation delegation (2026-10-22)

The C8 actual-source extra-paragraph evidence and bounded causal decision are
specified by `00_nucleo/prompts/core/compiler/realize/recipes.md` under
"Guarded paragraph packets retain inline authority". The realization leaf
already owns crate-private `realize_node` and `realize_paragraphs`. Reexport both
through this existing private facade so the coordinator can select the existing
preparation authorized by actual RuleId guards. Keep all algorithms in their
owning leaf; add no public API, signature, data, phase or generic wrapper logic.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
