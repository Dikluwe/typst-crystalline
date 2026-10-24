# L0 — paragraph_prepare
Hash do Código: e7be8dca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_prepare.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

### Evidence before specification

`lab/results/construction/diagnostics/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` records 32 inputs, state in initial.json/before/, at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. At 140pt/margin 8, Simple/Optimized choose different eta placement; justified 240pt Optimized shrinks spaces. Tekt discards strategy (`eval/rules.rs:3070-3100`, `stdlib/structural/par.rs:131-150`, `text.rs:41`, `word_layout.rs:29`); pinned inline/mod.rs:194-196 resolves auto from justification and linebreak raw_ratio/raw_cost plus model/par Limits::SPACING_DEFAULT define shrink/costs. Adjacent styles/Literal edges/opaque content are separate probes. This establishes observed syntax/geometry, not intent. Original preparation scope-outs explain deferred implementation but are superseded only within certified preparation below.

`lab/results/construction/diagnostics/correcao-hifenizacao-global-2026-09-28/before-pdf/` measures 67 inputs against historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`; initial.json/before/proveniencia.json preserve state/binaries/sources/commands. paragraph_prepare.rs:80-94 refuses true policy; paragraph_select.rs:66 only considers word ends; paragraph_plan.rs:18 selects transported strategy; break_selection.rs:41-66 hyphenates only local overflow. Pinned inline/linebreak.rs:170-209 uses same greedy candidates,`:626-684` penalizes 135*(1+0.15*(max(5-left,0)+max(5-right,0))) per hyphen plus 135 on consecutive lines; shaping.rs:583-635 draws '-' while preserving SHY for copy. Behavior/contracts do not establish historical intent.

Initial after-pdf has matching breaks but justified hyphen lines 2.1252pt short: en-140-optimized-True ex- x 115.26..132 versus 117.385197..134.1252. Pinned inline/line.rs:518-526 adds overhang only to emission remainder,`:708-709` uses 0.55 for '-' or SHY and text/mod.rs:391-393 defaults true. This supports emitted right-overhang, never changed cost/selection.

correcao-hardcode-paragrafo-2026-09-28/before-pdf records 20 inputs at historical pin/state initial.json/before/. eval/rules.rs:3483-3525 and stdlib/text/constructor.rs:153-190 omit overhang and wrongly accept auto/int/string. Pinned text/mod.rs:391-393 declares bool default true; inline/line.rs:518-525 conditions compensation on last style; paragraph_hyphen.rs:221-224 always used 0.55. Initial suspicion about paragraph_select.rs:78-82 was refuted: pinned linebreak.rs:551-559 uses attempt.justify, disabled for mandatory at line.rs:138-139. Preserve current last-natural-line formula. Algorithm constants 0.55/135/5/0.15 have source meaning; renaming numbers does not implement ignored configuration. Hyphen copy identity initially required a separate approved carrier, not glyph/position heuristics.

correcao-custos-paragrafo-2026-09-28/before-pdf records 66 inputs/state in initial.json/before/ against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`; exact errors/spans including accepted negatives are in vanilla stderr. Pinned text/mod.rs:1507-1564 validates Dictionary/Ratio-per-field/partial composition; inline/mod.rs:240 reads shared costs, not each fragment; linebreak.rs:29-30,961-962 applies 135/100 ratio defaults. Tekt discarded costs in eval/constructor and used fixed selectors. after-first-resultados.json/proveniencia.json isolates restore/inline failures among valid inputs: preparation refused newly transported glyph-neutral costs while whole Styled factoring already occurs before Par. This refutes blanket channel refusal without promoting partial fragments to shared scope or changing phase.

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/{before-pdf,before-extra}/` preserves 78 inputs/24 controls/full state in initial.json/before/ at historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`. page-65-100%-0%-False gives 3+1 Tekt versus 2+2 vanilla; zero costs match and tiny/large-positive retain protection. Pinned `flow/collect.rs:196-229` enables strictly-positive first-two/last-two and whole-three-line grouping; `flow/distribute.rs:343-364` advances only if required group fits next region. Emit previously progressed line by line. Distribution of words/lines in regions is language, not copied Frame/Child structure or inferred old-scope intent.

Approved textual-identity evidence `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` measures U+002D/SHY with matching geometry in eight controls, consumer inspection in approved-before/, public layout_types approval recorded.

`lab/results/construction/diagnostics/paragrafo-tracking-global-2026-09-29/focused-before/results.json` measures eight tracking 0/±0.2/1pt Simple/Optimized combinations with state/sources/pinned binaries in initial.json; Tekt chooses 10+2/9+3 words versus vanilla 7+5/6+6 while no-tracking-baseline.json matches. paragraph_prepare.rs:78-95 rejects any Some(tracking), even 0, skipping selectors/regional protection. Same divergence after extension would refute the inference; no intent claimed.

`lab/results/construction/diagnostics/paragrafo-tracking-hifenizacao-2026-09-30/{before,after-third,after-second}/results.json` records explicit 0 hyphenation divergence before and four 100/140pt Simple/Optimized matching text/pages/order/boxes afterwards (<0.001pt). Nonzero still differs by 0.2/1pt/new breaks; adding interval at every cut fixes some and breaks others. Pinned inline/shaping.rs:546-571 retains safe-cut glyphs or reshapes; current FontMetrics lacks that signal. Future safe-cut-aware shaping, not prefix hardcode, can refute the limitation.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized branches contain the corresponding transparent operation; Source must not leak as observable content. Human approval `confirmo` was received 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

snapshot-scopes-prioridade7-2026-10-08/fixtures/context100.typ/context300.typ/plain300.typ emit contiguous digits. Pinned comparison atual-vanilla-memoria5-2026-10-08.md and fluxo-numerico-contextual-2026-10-08 preserve exact probe state: single string is one line both, for with[#i] wraps only Tekt. paragraph_plan inspection finds Sequence(ContextBlock,Text) each contextual iteration; context_block.rs:15-36 is paged no-op unless capture_html_context_markers. This is observed source/behavior, not historical intent.

TS1 full-size probes `lab/results/construction/diagnostics/text-argumentos-2026-10-17/{before,before-measurement,morph-before-measurement}.json.gz` and audit.json preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/UTC/status/diffstat/source/L0/binaries/argv/historical pin. Relative/affine sizes/permutations/positional set expose abs-only loss at style_chain.rs:513-517,constructor.rs:123 and wrong top-wins size collapse; declared-size equality was separately refuted by pinned StyledElem::eq child comparison. Human approved TS1/two public contracts on 2026-10-17, manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`, implementation/tests S1–S16 under 12GiB/zero swap/no release-merge-push, reseal after implementation/valid graph; RG1 remains independent.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/paragraph_prepare.rs`. Prepare words with effective-style fragments and injected word metrics; gaps retain space width/origin/style. Leading Literal spaces preserve advance; initial markup does not. Adjacent text fragments belong to same word with no inserted space; compatible same-style fragments may fuse before measurement. Remove terminal gaps from cost without losing emitted carriers. Prepare borrowed parts with identical collect/finalization, no Content allocation/clone; entirely neutral segments fabricate no material Prepared.

Certify whole initial Latin text subset before emission: Text/Space/SpaceFromNewline/Empty/Sequence/typographic Styled/Strong/Emph, finite constant width/no math-smallcaps/outside columns-cells-floats and initial LTR. Reject partial par.*, private markers/unknown custom and unsupported nested transforms; whole state failure restores chain/style with no emission/cursor/Location. Strong/Emph retain morphological provenance. Later clauses below refine only measured restrictions, not all UAX14/objects/Linebreak/variable widths/custom limits/incremental scope.

Each private Fragment records lexical hyphenation bool, resolved overhang bool default true, replacement:Option<String> (None ordinary) and existing visual/style data. Fusion must respect different policy/overhang/replacement identity; preserve identity on clone. Words retain candidates/ranges supplied by `00_nucleo/prompts/core/compiler/layout/paragraph_hyphen.md`. Remove blanket true-policy refusal only for certified Latin uniform-style/policy words with simple terminal punctuation; finalize finite widths and delegate candidate qualification there. Mixed word styles/internal apostrophes/other breaks/objects/variable widths retain fallback; never partial emission or leaked chain.

Allow text.overhang in typographic whitelist, preserving final-style policy for optical compensation. Private Prepared.right_overhang defaults 0; materialized hyphen end uses dash_width*0.55 only when final fragment overhang permits, without changed selector cost. Other configurable punctuation/export-copy issues require their own contracts. Preserve source-derived meanings of 0.55/135/5/0.15; do not imply number renaming implements options.

Allow only validated text.costs.hyphenation/runt/widow/orphan glyph-neutral channels through private text_costs::is_channel using its FIELDS, not duplicated names. Whole scope factoring remains realization-owned; temporary preparation push restores and never changes planning's shared exterior costs. Partial inline costs do not choose paragraph costs or create space/word boundary. Preserve default costs and independently certified widow/orphan/context limits.

Finite resolved tracking is allowed only in uniform LTR text without internal Styled/Strong/Emph, math/smallcaps/objects/variable width or negative/nonfinite effective word/gap widths. Injected metrics remain authority for within-word tracking; never add it again. Expose private uniform-extent helper to selector/emitter. Absent and explicit 0 have same width here while lexical distinction remains outside this subset. Nonzero tracking with active/candidate hyphenation retains whole fallback; candidate-bearing explicit tracking is admitted only when exactly 0 and remaining uniform certificate holds. This later bound supersedes broader tracking+hyphenation language only to that extent; no value/trait/default/phase mutation.

Source transparently collects body preserving chain/tokens/bidi-hyphenation spans/costs; origin alone must not reject Simple/Optimized preparation. ContextBlock is neutral only while HTML marker capture is off; if on, reject entire preparation for old path. Never execute closure, lose marker or create space/word boundary/selection/phase change.

Keep partial par.* refusal/whitelist; `00_nucleo/prompts/core/compiler/layout/style_scope.md` owns its boundary. Admit Latin RTL only without active BidiCursor/tracking, with current textual certificate and uniform direction. Do not consume/synthesize BidiSpan, relax other refusals or force global preparation for other scripts.

TS1 changes this owner's test fixtures/adapters only: Style::Size(Pt)→Size(absolute Length), StyleDelta.size expectations→Option<Length>. Preserve language/functional expected geometry rather than replacing correct snapshots with representation snapshots.

## Acceptance and limits

Require discriminating strategy/auto/override RED→GREEN, intact words/morphological styles/Literal edges/adjacency/overlong/restoration/fallback and bilateral boxes 0.0005pt with baseline-grouped physical lines, not XML counts. Build/lint/1:1 ownership/pins valid. Internal ADR-0127 continuous flow introduces no new public signature/trait/entity/mode/flag/phase/compatibility; only separately approved TS1 public scope applies in its owners.

`00_nucleo/prompts/core/compiler/layout/paragraph_pagination.md` exclusively certifies vertical geometry/groups/regional admission from already-selected ranges, without reselection/glyph emission/duplicate new_page. Initial scope is prepared uniform vertical metrics/zero baseline offset/constant paginated geometry, outside columns/cells/subframes/notes/floats/active regional transaction; refused contexts retain fallback. Full certification precedes emission. Initial size/edges/leading match exterior, bold/italic allowed when metrics equal, finite nonnegative top/descent/leading. Later same-size/leading mixed-face certificate refines only edge uniformity as specified by that owner. Auto bounds need no protection; honor initial baseline/block collapse before real need versus page_bottom_limit. Next-region capacity uses next physical-page resolved margins; advance only with material now and group fitting there. Too-tall groups use ordinary progress/full content, never a new_page loop.

Private text_costs resolves widow/orphan independently of hyphenation/runt via FIELDS, default Ratio 1 and strict>0. Ordinary paragraph without linebreaks defaults to Simple to enable default protection; automatic-by-justify and all refused contexts remain. Preserve justification/carriers/terminal-open line/boundaries/lexical restore; do not apply incomplete protection to notes/columns.

RED→GREEN covers identical words 2+2 versus 3+1, whole-three protection, independently disabled orphan, too-large group, restoration and default without flag. Bilateral boxes/words/pages retain prior types/spans/fallback; XML groups do not count physical lines. Restricted owners/reseal/build/lint remain required. This supersedes old widow/orphan exclusion only inside measured certificates, not full algorithm/performance/parity.

Adapt only two existing optical-margin cfg(test) paragraph_hyphen::select calls to explicit LineCosts::default(), preserving input/expected outputs required by new private costs contract and 66 probes. Custom-cost transport needs same shared-scope restore/inline tests, no partial promotion. Uniform tracking needs whole-certificate/bilateral controls and complete fallback for mixed/nonzero-hyphen cases; future shaping information, not hardcoded prefixes, is needed to expand. Replacement acceptance uses causal text/boxes/recursive/literal-hyphen controls, not PDF bytes.

Preserve exclusive ownership/static dispatch without I/O or a new signature/trait/flag or eval↔layout phase. Source-origin tests/C2/C/macro collections/builds/lineage are required before certification.

Contiguous numeric fixtures keep exact text and one overlong line like vanilla; HTML capture remains own path. TS1 approval retains S1–S16/resource/no-publication/graph gates and independent RG1/equality debt.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

## Prose continuation with colon — 2026-10-23

### Measurement before classification

`lab/results/prose-continuation-2026-10-23/before/receipt.json` and
`baseline-source.json.gz` retain UTC, uncommitted HEAD, exact dirty stat, all
source hashes, input/binary/observer identities and bilateral commands. Oracle
is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
The unchanged 20-paragraph prose first differs at the fourth page's final line;
the first seven paragraphs preserve that transition. Four long words with a
colon yield 3+1 lines versus vanilla 2+2, while replacing only the textual colon
with a period gives 2+2 in both and corrects the full prose continuation.
`paragraph_prepare.rs:145–150` excludes colon; failure bypasses the existing
prepared-line widow admission (`paragraph_plan.rs:90–127`,
`paragraph_pagination.rs:158–208`). Pinned `flow/collect.rs:196–229` groups the
last two lines with positive widow cost. These are language text/line/page
observations, not a requirement to copy upstream machinery. The hypothesis is
refuted by altered word breaks, loss of punctuation, or changed zero-cost,
spacing, leading, lexical or overflow controls after admitting colon.

### Bounded internal correction

Certify ASCII colon in the existing Latin, space-delimited preparation subset,
including attached, internal and adjacent-fragment colons. Preserve it as part
of its word, with measured width, exact text and separators; introduce no break
at colon and no fixture-specific pagination logic. All prior style, script,
object, offset, region and tracking/hyphenation exclusions remain. Delegate
selection, cost resolution and regional protection to their existing owners.
This is an internal vanilla-parity correction under ADR-0127 continuous flow,
with no public API, default policy, phase or compatibility change.

Require actual RED→GREEN for colon preparation and four-line continuation,
positive/zero widow controls, exact punctuation/adjacency and rejection controls;
bilateral full-prose and reduced-case line/page membership, spacing/leading,
paragraph/overflow geometry at the original tolerance, plus complete gates.
This does not certify arbitrary punctuation or all incremental paragraphs.
