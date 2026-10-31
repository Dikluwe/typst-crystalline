# L0 — raw highlight
Hash do Código: c9fbb837

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:a3772778f9e542de87823509c8eac57904fc73fea9526cebe6052833a69ef806
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:24f0b69fa1db115ff13a786121c80cb7be3abfa2028a9a473075374a6815cfd4
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:a0d3cae81e77da5875204a94bc2fbd707009f0d751cf50ed870c416a01187a22
- 00_nucleo/prompts/_nuclei/layout/typc-length-display.toml sha256:37f7c44553aaf173c60a6d9a525e196042143ccb52ec617a29d8bc62ea7dfeea
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/raw.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/raw.rs:103`, `01_core/src/compiler/layout/raw.rs:117`, `01_core/src/compiler/layout/raw.rs:20`, `01_core/src/compiler/layout/raw.rs:41`, `01_core/src/compiler/layout/raw.rs:47`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Historical pins do not certify current ratified `64c5e8c02fe0394d913ead36e42ef8c1334d2107` without new measurement.

Full historical observations/provenance/failed candidates remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/raw.md`. `raw-defaults-2026-10-07/{before,before-state}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/diffstat/UTC/sources/commands, Tekt `58456b9b54041983612e5946f89fef2cf4591b9f31a09d8dcce9ae5c8a9c0eea`, historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8` binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Sixteen controls refute old 100%-size/Serif defaults: Mono 0.8, absolute/show-font/relative overrides, substituted ABC retaining Raw defaults and inherited red fill. Pinned text/raw.rs:658–670 installs defaults before overrides; Raw:103–123 retained wrong font/size/discarded fill, apply.rs:207–253 lost defaults in replacement, style_scope.rs:41–44 projected abs-only size. green.log/tested-state refutes first fill fix: chain.fill:422 reads typed only while set/text.rs:57 stores custom text.fill, already consumed by Text. Styles/geometry are language, not bytes/buffer requirements.

`raw-block-spacing-2026-10-07/{before,before-additional}.json.gz` and before-state preserve same HEAD/dirty state/UTC/diffstat/status/source/commands, Tekt `43207349fba51afbdbcac3847c1a7d884e9ff845b6401022be22272679c00575`, same historical oracle/hash. Prefix/suffix/cell/unknown-language/replacement/show-size/show-em/margins/par.spacing/empty-Raw probes expose missing block boundary, wrong local origin and collapsed explicit lines. Pinned typst-layout/src/rules.rs:655–679 realizes Raw as Block, container.rs:324–360 defines margin/collapse; cursor.rs:88–132 already consumes pending below, sub_frame.rs:315–325 uses hypothetical terminal leading. after/after-additional, refuters-red and refuters-second preserve failed candidates: extra2.64pt consecutive gap, cell offset/Strong-cell height, explicit margins ignored at set.rs:976–1006, prior paragraph sizes-up/down negatives, then remaining 0.14302pt Strong-versus-plain origin. Strong interior top is not region exterior origin.

edges-red.json.gz refutes below0/negative/auto-spacing priority and next-auto cases; explicit edges must win weak edges in both directions before magnitude. suites-third.log refutes unconditional FontMetrics initialization by new image/shape Place font-book access: only legacy subframe with terminal Raw block needs installed regional ascent. These measurements retain exact binaries/snapshots/UTC in their diagnostics; no later tolerance may hide residuals.

Standalone Cell 53-probe evidence and complete source/state/UTC/binaries/pin are in `lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz`; before/refuters/complete are not extra independent rounds. It exposes intrinsic defaults/raw-versus-prepared identity/effects/fresh-generation termination/alias previous-rule reuse without fabricated location; cell-set remains separate. Pinned model/table.rs:734–778 and typst-realize/src/lib.rs:367–386,418–429,491–505,571–586 distinguish prepared arguments/guards/materialization. Owner separately ratified/materialized package on 2026-10-07 (`confirmo`), record `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. This supersedes stale candidate-only gate for B's one new public field/type, never via earlier A approval.

That diagnostic's after-second/html-fields preserves HEAD/state/UTC/binaries: Tekt HTML displays Auto as plain text while vanilla code(data-lang:typc)/constant.language span#d73948. Pinned foundations/value.rs:191–208 presents Auto/Bool as inline Raw typc; markup_stream.rs:210–215 handles Bool only; raw.rs:201–203 colors only true/false; HTML inline lacks Raw/general path. Equal text is not morphology/style parity.

box-baseline-inicial-2026-10-14/before/results.json.gz/before.json.gz preserve twelve-source geometric/adjacent-text/shift probes with pinned binaries/state/HEAD/UTC: zero and absent installed ascents were conflated. `posicoes-compostas-2026-10-14/raw-before.json.gz` preserves raw.rs:178 neutral-carrier first-boundary failure. `lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` preserves full HEAD/dirty state/diffstat/UTC/source/binary/argv/historical pin and certified-flow-anchor defects after line/Shape/Block/Grid/Raw/VSpace, with weak-margin/page/rollback/subframe refuters.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Own only `01_core/src/compiler/layout/raw.rs`, not another renderer/parser. Use embedded two_face syntax definitions/existing Typst Light scope theme, with only immutable lazy syntax/theme sharing. Apply supported foreground/bold/italic token styles and restore exterior. Existing split_whitespace token emission remains an explicit unresolved text/whitespace discrepancy, not license to normalize/fix/weaken it. Unknown syntax/highlight failure uses existing plain-token fallback, refined below only for block explicit line breaks.

Current Raw defaults supersede older clear-fill/100%/font-inheritance clauses: Mono family, size 0.8 of exterior, bold/italic disabled and exterior fill inherited. For fill read custom text.fill before TextStyle.fill fallback, including replacement scope; do not alter global StyleChain contract. Direct Raw applies/restores defaults. Show-product Raw defaults travel in private raw.defaults Styles outside overrides; mark raw.defaults.active only within that scope to prevent double application when returning it. Relative text.size Length resolves against prior size within Raw defaults scope, preserving absolute/relative composition and chain restore. No copied show/matching/guard algorithm or new public type/trait/signature/phase.

For unsupported-grammar typc true/false/auto atoms, use existing palette constant.language foreground#d73948 while retaining Raw morphology. Cross-owner boundaries: `00_nucleo/prompts/core/compiler/eval/markup_stream.md` maps Auto only to Raw(lang:typc,block:false); this owner colors atom auto alongside true/false; `00_nucleo/prompts/infra/export/html/inline.md` serializes those atoms with code/constant span and delegates existing escape/mode. No other Value presentation, new grammar or general arbitrary/language Raw HTML support is authorized.

Own private physical Raw begin/end, local origin, margin resolution and terminal-product classification under raw-block-boundary. `00_nucleo/prompts/core/compiler/layout/style_scope.md` orchestrates them for transformed products and `00_nucleo/prompts/core/compiler/eval/rules/apply.md` transports block flag/size/margin Length unmeasured in existing Styles. On begin close material prefix; use previous baseline/descent plus collapsed above and geometric initial baseline. Previously closed exterior paragraph in subframe still participates. On end publish actual physical bottom and leave below in Cursor protocol; terminal Raw subframe height is actual bottom minus top, never pending leading/final margin.

Default Raw margin is weak and resolves with effective Raw size (e.g.10.56pt at default 8.8pt), while prior exterior11pt paragraph contributes 13.2pt separately. Existing prev_margin_is_parbreak denotes weak paragraph-margin protocol including Raw-inherited margin; explicit below remains strong. Cursor collapses weak margin with current par.spacing without changing strong/region/fraction/checkpoint branch. Compare explicit priority before weak in both directions; equal priority collapses max. Auto edge interrupts shorthand spacing and restores weak par.spacing/default. Explicit below<=0 leaves cursor at actual end+below with pending geometric baseline; weak below including 0 takes weak Cursor path adding next text top. Do not apply special offsets.

First Raw in legacy subframe converts origin using optional ascent actually installed for region, not Strong/Emph interior face; absent means zero shift. Initialize regional ascent only for legacy subframe whose terminal product is block Raw after saving parent ascent, then restore parent. Geometric/nontext paths gain no font query. A current_items buffer of direct neutral position carriers only is empty for first Raw boundary; other items retain meaning. Preserve calibrated Raw exception/defaults/line breaks/dimensions.

Block Raw unknown syntax keeps explicit breaks between lines without new whitespace/tab normalization or changed inline Raw. Terminal classification transparently includes Strong/Emph through explicit nominal cases; it is a boundary predicate, not a data-dropping enum projection. For prepared standalone/positioned Cell inspect only complete prepared visual, preserving Raw formulas; no Engine/preparation in layout.

Set boundary (`00_nucleo/prompts/core/compiler/eval/rules/set.md`) recognizes Native block identity/aliases only for spacing/above/below Length-or-Auto, evaluates once and publishes delta only after whole-set validation. Unsupported fields/types retain prior fallback/warning and remain uncertified. Apply preserves size/margins as Length; Styles never measures glyphs.

After installing below/cursor state, register Raw's physical end excluding below margin. Hanging begin_block closes prior flow and returns private restoration State while installing inline fragment for tokens; layout and style_scope caller restore returned State. end_block retains its margin protocol. Inline Raw inherits exterior paragraph. Do not change tokenizer/whitespace for this context adjustment.

## Acceptance and explicit incompleteness

Internal parity corrections follow continuous-flow ADR-0127 with L0 first/real RED→GREEN/restricted reseal and no new public contract/default/phase except separately approved B scope. Typography tests cover font/size/fill/restoration/overrides/replacement; recollect 16 controls/actual cell glyphs/regressions, preserving messages/order/indexes/targets, not font subset/PDF bytes. Unknown/custom syntax, whitespace/tabs/internal empty lines, fresh Raw reconstruction, rich block styles/HTML remain uncertified. Prior paragraph sizes-down negative is not fixed without separate measurement.

Block repair recollects 14 controls+five refuters, prior typography/Cell morphology/math/query/full suites/build/lint; no posthoc tolerance to erase residual. Strong-cell retains plain origin/height after rebuild/regression tests; no image/shape/no-text font demand. Explicit zero/negative/Auto priority tests retain exact expectations and normal strong/weak Cursor protocols.

B ratification acceptance preserves only approved candidates, preimplementation RED, changed-owner seals, bilateral effects/equality/getters/selection/HTML/real text-style-geometry/exact message-span controls and earlier snapshots via restricted diff without commit/reset/global reseal. Revalidate D08A/B2/B3-A, Raw blocks/defaults, Equation/math/matrix/query, debug/release builds/core-infra-shell suites/lint without errors or owner V5/V6/V15/V26. Cell-set/style defaults remain separate; general Table equality/defaults/rich casts-paints-strokes-insets/lines-gutters-multipage/PDF-UA/global nonlocatable policy and preexisting empty-file warning remain open.

Atom presentation tests first refute Text Auto/general Raw rejection then require correct product/style for typc auto/true/false with Table/Raw/query controls; no broader Value/HTML claim. Installed-ascent/neutral-marker controls retain current Raw/HTML/default/paged geometry. Certified flow producers do not close Block/inset/fragmentation, late Box/Transform or Heading/List/Stack/Equation/Columns; invalid witness stays legacy fallback/pending. LIFO required private causal state; revalidate suite/HTML-context/PDF-SVG/build-doc-fmt-check/lineage/warnings.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

## Measured remaining HTML and Length parity correction

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/agents/html-length/measurement-before-decision.json`
pins receipt SHA-256 `96025a9ff9d2c5a6057e9873285e43e4e81501bb40b55fe43b1244743e8ed09c`,
UTC `2026-10-22T11:53:10.657191+00:00`–`2026-10-22T11:53:35.064880+00:00`,
uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, complete dirty stat,
source archive, commands/fonts/fixtures/build/tool and verified oracle pins.
All provenance checks pass. Oracle commit is `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Length rendering, numeric Raw paint, page warnings, native page aliases and two
container refusals differ. False-condition/uninserted/scalar controls pass. The
coordinator froze exact source rejection in `lab/results/remaining-parity-2026-10-22/html-negative-policy.json`
before implementation; no failed render is waived. Pinned `foundations/value.rs:192–205`,
`text/raw.rs:563–606` and `typst-realize/src/lib.rs:635–655` distinguish actual
content morphology, token styles, target-specific warnings and container errors.

### Exclusive correction and acceptance

Recognize a complete bounded inline typc numeric expression with the existing pure
Code lexer before emitting it. Require numeric terms, optional unary sign, plus/minus
operators and exact plain-space separators; an unrecognized stream takes the old Raw
route without partial output. Apply the pinned shared token colors and existing Raw
font/size/overrides; emit adjacent tokens without invented separators and literal
spaces exactly once. Preserve Bool/Auto and every other syntax/fallback/block route.
Add genuine RED style tests for actual Length-like numeric Raw and space/operator
transport before implementation; verify effective glyph paint and 1e-8 geometry.

Internal continuous ADR-0127 parity correction: no public field/trait/signature,
new mode/default, phase or compatibility break. L0 precedes code and meaningful
RED→GREEN; validate paths, valid owner/Nucleus graph, restricted hashes, tests,
build, strict Clippy/rustdoc, reverse lineage and complete unsuppressed Tekt.

The local numeric-space regression must inspect actual literal TextSeparator
occurrences, advances and effective Raw size in addition to legacy plain_text.
`entities/layout_types.rs:899–934` explicitly ignores those separators and joins
text runs with invented spaces, so its passing pre-correction text assertion is
only a preservation control. It cannot establish lexical spacing. Keep that
original assertion and additionally require the two actual literal separators
of `12pt + 0.5em`, positive finite advances and existing 0.8 Raw size. Full exact
advances and styles remain certified by bilateral glyph geometry/paint at 1e-8.


## Raw is normal text inside an equation

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/candidate-2/qualification/math-length-2/receipt.json.gz`
retains the two immutable Length/explicit-typc-Raw sources, successful byte/font/
source/build guards, UTC 2026-10-22T13:44:23.723338+00:00–13:44:26.943012+00:00,
HEAD/complete uncommitted changed-file list and source-bound candidate
`feb025296531c92350e29bff032f655cc2fa47542cf8ab4aca0e239108bc4e8f`.
The ratified oracle remains bd5ebec285505889e8ff1d4f68804ab147a699f3 with executable
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
Raw morphology/paint now agrees, but actual glyph origins and page height do not.
`agents/inline/math-raw-face-cause.json` under that result root (SHA-256
`6de15bd9fafd430fda8f795e339c295a1710c4f0ee68d5669cc373fcbe4df264`)
binds the actual two font bytes/tables and candidate receipt. At the actual 8.8pt,
NewCM's `12pt` advance/ascent/descent are 17.116/5.8608/1.7072pt; Mono's are
21.1921875/6.53125/1.83046875pt. Those differences exactly explain the measured
centering, baseline and bottom error. This refutes a fitted axis/baseline offset.

Pinned raw.rs:657–669 installs ordinary Mono/relative-size text defaults;
rules.rs:657–680 realizes Raw lines to text; math/ir/resolve.rs:127–150 and
math/text.rs:17–39 resolve and lay out that actual textual content. Current
Raw inherits `math=true` from its outer Equation. The metric provider appends
MATH fallback and resolution.rs:176–184 then prefers a MATH face over explicit
Mono, while the renderer draws Mono glyphs. The classification, not the bridge
geometry formula, is the missing boundary. This is language typography/geometry;
no copied upstream buffer or flag representation is required.

### Exclusive correction and acceptance

A real Raw token run uses normal text font selection even when its enclosing
content is mathematical. Clear only the effective Raw run's `math` glyph
classification after selecting existing defaults or an already-active Raw scope.
Preserve Mono/explicit scoped font, relative size, color, numeric token paint,
weight/italic behavior, source identity, literal separators and exterior restore.
Do not reset unrelated `math_text_item`, script/size/cramped axes, alter global
MATH covering policy, or change bridge dimensions, anchor formulas or phases.
Both ordinary and already-defaulted Raw products retain one application of size.

Before the body correction, require a reached math-sensitive injected-metric RED
for actual Raw geometry at more than one size, and an independent ordinary
mathematical-identifier control. FixedMetrics alone cannot distinguish the two
face classes. Keep all previous paint/style/baseline assertions and tolerances.
The two complete source-bound real-font comparisons must pass unchanged for
closure. This is ADR-0127 continuous-flow parity work, with no new public field,
method, dependency, default mode, compatibility break or phase.


## Readiness: literal U+0020 in Raw (2026-10-28)

### Evidence before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/before.json`,
source-before.json.gz and table-number-build-source.json.gz preserve exact HEAD
212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diffstat, UTC and all inputs.
Oracle: 64c5e8c02fe0394d913ead36e42ef8c1334d2107, binary SHA-256
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
Table correction aligns code-data vertically, but table-number-candidate/
comparison.json retains 2083 differing PDF pixels and collapsed code words.
`raw.rs:439-459` discards whitespace in highlighted/error/unknown syntax paths.
Actual SVG glyph origins confirm lost indent/gaps, not merely extraction.
extras-before/receipt.json preserves explicit leading/doubled spaces in plain,
Rust, unknown-syntax and inline Raw. Current-pin typst-layout/src/rules.rs:
658-670 retains whole line text with Linebreak separators. Space positions are
language observables; the syntax token container is implementation mechanics.

### Exclusive correction and acceptance

Emit every actual U+0020 through the existing Literal TextSeparator path,
including leading, repeated, trailing and inter-token spaces. Retain existing
nonspace shaping/wrapping/highlighting in highlighted, fallback and unknown
syntax routes. Keep the special bounded numeric typc tokenizer, defaults,
colors, source routes, newline/begin/end/margin protocol and exterior restore.
This supersedes earlier no-whitespace-change clauses only for U+0020 in these
three paths. Tabs, other Unicode whitespace, blank-line struts, rich themes
and long-line wrapping remain uncertified; no tab-size entity field is added.
ADR-0127 continuous internal parity repair: no public signature/field/default
mode/phase/compatibility change. Tests must fail on actual alpha/beta origins
and literal separator multiplicity, not plain_text's fabricated spaces.
Revalidate extras/code-data text/pages/0.001pt/independent paint, previous typc,
math/image/JPEG controls and full gates, with V15/V26 before resealing.


## Python explicit line continuation — issue 9015 (2026-10-31)

### Evidence before decision

`lab/results/construction/diagnostics/issue-9015-2026-10-31/before-state.json`
preserves UTC, HEAD and the exact uncommitted diffstat/status. raw-before.rs.txt
and raw-before.md.txt preserve the prechange owner and consumer. The official
reproduction and audit are in
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/agent-text-svg/issue-9015.typ`
and report.md. Current oracle remains commit
64c5e8c02fe0394d913ead36e42ef8c1334d2107, not a moving upstream target.
`raw.rs:90-94` loads two-face's no-newlines syntax set, and `raw.rs:426-434`
feeds str::lines() without terminators. The isolated probe.log compares both
syntax modes on the same Python input: without terminators world loses string
paint and code after its closing quote gains it; preserving terminators with
extra_newlines corrects both. Syntect 5.3.0 easy.rs:129-157 documents this mode
as more robust. This localizes the defect to the raw highlighting adapter's
lexical input, not Typst's lexer, shaping, or the string color table.

The first candidate's green-ram.log refutes raw CRLF transport: the newline
syntax still loses continuation with CRLF. refuters-probe.log isolates this
for single/double quotes and ordinary/raw/f strings; normalized-probe.log
shows that lexical CRLF→LF transport repairs it. The same probe measures
triple-quoted docstring paint as #74747c under both old and new grammars,
refuting the initial test's unsupported green expectation. Preserve that
existing theme classification. refuters-state.json pins this measurement.

### Exclusive correction and acceptance

Use the newline-aware embedded syntax set only when the resolved syntax is
Python (including py/python and case-normalized aliases). Preserve LF in
input delivered to HighlightLines and normalize a physical CRLF to lexical
LF only within this Python adapter, retaining one state across all physical
lines. Remove only the terminator from painted ranges/fallback; do not
normalize visible text, spaces or layout breaks.
Never invent a visible separator or extra terminal line. Keep every other
language on its existing no-newlines grammar/input route. No grammar patch,
manual quote/backslash state machine, new dependency, public API, flag or
pipeline change. This is an explicitly requested internal defect correction
under ADR-0127 continuous flow and ADR-0107, not a requirement to copy the
ratified vanilla defect.

RED→GREEN must inspect actual laid-out text paint for continued single and
double quotes, aliases/case, CRLF, repeated continuations, raw/f strings and
ordinary code/comment/string paint after closure. Include unescaped newline,
escaped backslash, triple-quoted string and plain Python controls; preserve
block line positions, literal spaces, exterior style restoration and inline
Raw. Controls for Rust, JavaScript, JSON and unknown syntax retain existing
styles and geometry. Preserve all prior limits on tabs, blank-line struts,
wrapping and HTML. Reseal only this owner/consumer after V15/V26 preflight;
run raw tests, proportional layout tests, cargo build and full crystalline-lint.


## Other-language regression coverage refinement (2026-10-31)

### Evidence before decision

`lab/results/construction/diagnostics/issue-9015-2026-10-31/coverage/before-state.json`
and before.rs.txt/before.md.txt preserve UTC, HEAD, exact dirty diffstat and
this owner pair before the refinement. `raw.rs:684-700` observes only Text;
`raw.rs:776-794` drops whitespace and projects away geometry and font flags.
Thus the old observer cannot certify all obligations above. FixedMetrics at
`metrics.rs:424-432` gives an independent monospace advance; unknown-syntax
Raw retains an independent plain layout route. style-probe.log refutes an
indented Markdown positive control; style-probe-additional.log establishes
actual bold and italic spans with unindented Markdown/reStructuredText.
The obsolete current-pin wording at this prompt's line 22 is corrected to
the ratified 64c5e8c02fe0394d913ead36e42ef8c1334d2107; dated historical
measurements keep their historical pins and qualifications.

### Test-only correction and acceptance

Extend observations to literal TextSeparator occurrences and per-character
text positions/advances/full TextStyle. Compare Rust, JavaScript, JSON,
unknown syntax and genuine Markdown/reStructuredText bold/italic controls
in inline and block Raw, using LF and CRLF. Preserve leading, repeated,
inter-token and trailing U+0020; require exact source text minus physical
terminators, literal provenance/count, monospace horizontal advances, line
origins and per-line baseline geometry. The independent unknown-syntax
route with the same source provides geometry and default font/size; the
legacy no-newlines highlighter provides token color and bold/italic flags.
Require positive and negative font-flag observations and restoration of
exterior nondefault paint/size/bold/italic.

First demonstrate RED for the old test observer dropping source spaces,
then extend that test-only observer to actual separator fields for GREEN.
Additionally require the comparator to reject missing/extra literal spaces,
x/y/advance changes and flipped bold/italic, including real positive spans;
these mutations operate on observations, never on productive code. This
strengthens certification without inventing a functional product defect.
No productive behavior/API/phase/dependency changes; ADR-0127 continuous
flow. L0 and its forward seal precede tests; reseal only this owner pair,
then run Raw tests, build, full lint and reverse lineage with Cargo artifacts
and temporary files confined to the existing exclusive #9015 tmpfs target.
