# Remaining compiler L0 sanitation: scope and history

## Inventory recorded before L0 edits

Audit HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
Inventory capture (UTC): `2026-09-27T01:21:30.330891+00:00`.

Scope follows the actual consumer's `@prompt`, not an ambiguous leaf name.
The explicitly requested math family includes `compiler/math/layout` and its
separate test owners, not the excluded `compiler/layout` family. The complete
assigned inventory is below. This diagnostic is not a materializable prompt.

| Assigned L0 | Declared consumer | Intake state |
| --- | --- | --- |
| `00_nucleo/prompts/compiler/lang/equation_supplement.md` | `01_core/src/compiler/lang/equation_supplement.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lang/figure_supplement.md` | `01_core/src/compiler/lang/figure_supplement.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lang/heading_supplement.md` | `01_core/src/compiler/lang/heading_supplement.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lang/outline_title.md` | `01_core/src/compiler/lang/outline_title.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lang/quotes.md` | `01_core/src/compiler/lang/quotes.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lang.md` | `01_core/src/compiler/lang/mod.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lexer/code.md` | `01_core/src/compiler/lexer/code.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lexer/markup.md` | `01_core/src/compiler/lexer/markup.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lexer/math.md` | `01_core/src/compiler/lexer/math.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/lexer/mod.md` | `01_core/src/compiler/lexer/mod.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/_comum.md` | `01_core/src/compiler/math/layout/mod.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/accent.md` | `01_core/src/compiler/math/layout/accent.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/assembly.md` | `01_core/src/compiler/math/layout/assembly.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/attach.md` | `01_core/src/compiler/math/layout/attach.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/callbacks.md` | `01_core/src/compiler/math/layout/callbacks.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/cancel.md` | `01_core/src/compiler/math/layout/cancel.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/cases.md` | `01_core/src/compiler/math/layout/cases.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/delimited.md` | `01_core/src/compiler/math/layout/delimited.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/frac.md` | `01_core/src/compiler/math/layout/frac.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/matrix.md` | `01_core/src/compiler/math/layout/matrix.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/op.md` | `01_core/src/compiler/math/layout/op.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/p1425-tests.md` | `01_core/src/compiler/math/layout/p1425_tests.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/p1426-tests.md` | `01_core/src/compiler/math/layout/p1426_tests.rs` | Existing local change; preserve |
| `00_nucleo/prompts/compiler/math/layout/p1427-tests.md` | `01_core/src/compiler/math/layout/p1427_tests.rs` | Existing local change; preserve |
| `00_nucleo/prompts/compiler/math/layout/p1429-tests.md` | `01_core/src/compiler/math/layout/p1429_tests.rs` | Existing local change; preserve |
| `00_nucleo/prompts/compiler/math/layout/p1430-tests.md` | `01_core/src/compiler/math/layout/p1430_tests.rs` | Existing local change; preserve |
| `00_nucleo/prompts/compiler/math/layout/p1431-tests.md` | `01_core/src/compiler/math/layout/p1431_tests.rs` | Existing local change; preserve |
| `00_nucleo/prompts/compiler/math/layout/root.md` | `01_core/src/compiler/math/layout/root.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/spacing.md` | `01_core/src/compiler/math/layout/spacing.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/stretch.md` | `01_core/src/compiler/math/layout/stretch.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/stretchy.md` | `01_core/src/compiler/math/layout/stretchy.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/tests.md` | `01_core/src/compiler/math/layout/tests.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/underline.md` | `01_core/src/compiler/math/layout/underline.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/underover.md` | `01_core/src/compiler/math/layout/underover.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/layout/vec.md` | `01_core/src/compiler/math/layout/vec.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/mod.md` | `01_core/src/compiler/math/mod.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/math/symbols.md` | `01_core/src/compiler/math/symbols.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/mod.md` | `01_core/src/compiler/mod.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/scanner.md` | `01_core/src/compiler/lexer/scanner.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/scopes.md` | `01_core/src/compiler/scopes.rs` | Clean relative to HEAD |
| `00_nucleo/prompts/compiler/utils.md` | `01_core/src/utils.rs` | Clean relative to HEAD |

## Exclusions

No eval, layout, introspect, columns, footnote_overflow_columns, parse or stdlib
owner is included. This excludes their subtrees and directly-in-compiler
companions: eval.md, layout.md, introspect.md, parse.md, stdlib.md,
layout-image.md, layout_counters.md, layout_figure.md, layout_outline.md,
layout_references.md, show-regex.md and style/font-dict.md. The latter companions
have eval ownership or non-materializable eval integration; a missing unique
consumer is not permission to edit them. The inventory covers every remaining
compiler prompt found at intake; each has one declared consumer.

## Contract audit and historical reconciliation

This is documentation sanitation, not a functional change or a parity result.
Repository and core instructions and the applicable accepted ADRs were read.
ADR-0107/0108 govern language-level acceptance and evidence; ADR-0109 keeps
geometry in the render owner; ADR-0127 governs changes of contract rather
than routine translation; ADR-0128 keeps semantic targets separate;
ADR-0129/0130 govern unique ownership and canonical references.
Shared nuclei were read but not edited.

The original inventory predates all L0 edits. Work resumed on the same assigned
set; the additional SHA-256 intake manifest below identifies the bytes actually
used for rewriting after resumption. Existing local changes in the five marked
test prompts were incorporated, including accent's exact shortfall witness and
matrix's independent-width, duplicate-offset and rectangular-dimension witnesses.
A clean-at-intake prompt's historical text is recoverable with
`git show 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5:<path>`; that command is
not claimed to recover the marked locally modified prompts.

### Language and compiler facade

The old compiler facade called itself rules/mod, described an outdated module
repertoire and cited V5 as an import-isolation rule. The actual module declares
eval, introspect, lang, layout, lexer, math, parse, scopes and stdlib;
V5 is prompt drift, not import topology. The facade no longer restates child
signatures or owns their pipeline behavior.

Language defaults were historically changed from Portuguese/test-driven choices
to deterministic English. The original supplement/outline probes cited
`a51e02804`; heading reconciliation cited
`c7577a031847ff38e4780044bc4132c1d92a14b9` and the older
`586e1bd43` oracle. Those are snapshot-specific evidence, not new-pin parity.
Heading's private-only helper and narrow compiler reexport were subsequently
promoted to a public helper/reexport (approval recorded in the original P1460
clause, 2026-09-18). The consumer confirms that surface, so the private-only
restriction and future L2 migration narrative were removed. Generated strings,
including French Section and French quotation NBSP, remain normative.
Quotes already validate string/array/dictionary overrides and support German
alternatives and German single quotes; claims that singles were English-only,
fallback doubles were ASCII or overrides were future work were outdated.
No regional/nesting parity is inferred.

### Lexer, scanner, utilities and scope history

Lexer individualization P1199 created distinct mode owners. Raw entry is
universal outside Math while its mechanics remain with Markup. The old hub's
duplicated grammar and downstream smart-quote evaluation rules were removed,
not transferred to a new lexer implementation. Markup's internal ASCII-space
rule retains its morphology; historical P1137 comparison is not a fresh test.

The scanner's unscanny inlining is architectural provenance, not a mechanical
byte-parity acceptance criterion. ADR-0116 preserves its specific unchecked-slice
exception and UTF-8 safety invariant; historical benchmark totals are not
repeated as current performance evidence. Utils still has a local no-op timing
macro; scheduled Step 10 reactivation/export promises were not materialized
contracts and are archived here, not carried forward as a deadline.

Scope history included eager snapshots/lazy shared capture, read-only base/
captured separation and later diagnostic classification (P715/P772/P780).
The real snapshot includes base.global, whereas iter_all does not; the older
prompt incorrectly described their membership as identical. No captured/base
mutation, error priority or math-local lookup behavior was changed.

### Mathematical identity, style and reconstruction

Early math prompts were monolithic and later split (P314/P909); module origins,
execution reports and historical coverage totals no longer define L0 ownership.
The mathematical layout core owns shared machinery only; features and test
modules retain separate consumers.

Older MathPrimes text said evaluation always lowered primes to MathText and
there was no dedicated handler. The current dispatcher and attach recipe
preserve MathPrimes identity, dedicated grouped-prime geometry and the count-zero
case. Historical outer-wins math style composition was explicitly refuted by
the original P1291 bilateral SVG observation against `a51e02804`;
the current prompt and code implement innermost same-axis setters, with
orthogonal axes. ADR-0103 still contains the historical outer-wins account;
this sanitation does not rewrite that ADR or manufacture a new approval.
That documentary conflict remains an explicit governance follow-up.

Old P945 paragraphs deferred size-factor corrections; fraction/grid/root/
annotation consumers now contain later discrete-level sizing. Baseline/axis
history distinguished midpoint-centered grids, axis-anchored fraction bars,
own-baseline radicals and unchanged fenced-body baselines; the blanket
axis-offset description was wrong. MathBox item coordinates remain
baseline-relative, with positive y downward.

Recent root/fraction/accent/cases/matrix/lr/mid/stretch and under/over public
carriers are dispatched in the current consumer. Labels such as PROPOSED,
future consumer and pending implementation no longer describe that code.
Implementation presence does not prove that every historical approval
record exists, that every acceptance case passes or that parity is closed.
The diagnostic therefore separates active contract from proof/coverage gaps.

### Geometry decisions and refuted intermediate accounts

- Fraction offsets were corrected from top-relative to baseline-relative;
  gaps became separate font-derived floors/shifts, Display gained dedicated
  constants, padding became outer-style 0.1em and line=false retained the
  barless Stack path. Three public styles and deparenthesization now have
  independent test obligations; their presence does not prove slash geometry.
- Radical index size/position evolved to absolute ScriptScript and MATH degree
  constants. Display gap and zero surd shortfall are active. The former
  deferred TeXbook excess-height redistribution is implemented; stale pending
  text was removed. Historical descent/extra-ascender observations require
  new evidence before any claim of closure.
- Cases/matrix unconditional script reduction was an inline-only generalization.
  Discrete denominator descent replaced it. The 1.1 grid delimiter target is
  independent of shortfall; original claims that it was an inflation error
  were refuted in P1026. Historical pixel/glyph/output totals are not acceptance.
  Padding removal, one grid axis-centering operation and baseline-preserving
  horizontal composition remain contracted.
- Assembly positions moved from top-of-slot to bottom-of-slot baselines.
  This superseded intermediate centering formulas. Horizontal ink extents
  replaced global font ascent; minConnectorOverlap replaced zero-overlap
  assumptions and hor_advance replaced stretch-axis advance as reported width.
- Accent's old no-dotless/no-size/upper-only scope was superseded by complete
  orientation and presence-aware carriers. Signed-descent additive extent
  calculations were replaced by the max-of-ink-top formula; shortfall is 0.5em.
- Under/over spreaders use accent support geometry, not overbar/underbar constants.
  Annotation placement uses limit shifts, not tight ink stacking. Exact width
  and single-glyph versus assembly attachment semantics remain distinct.
- Cancel's minimal diagonal-only/no-cosmetics account was revoked. Complete
  length/angle/cross/inverted/background/stroke and the validated callback
  transcript are active. Cancel itself stays non-locatable; equation Location
  plus structural occurrence/line supplies request identity. P1415's
  document-free finish_transcript extraction exists and is no longer a future
  coordination step.
- External math content initially flattened to plain_text, then used a broad
  external-layout route, then a restricted route. Later typed-text deltas
  closed the bare text(size/fill/weight) routing gap. Early unconditional
  axis-centered Group wrappers were refuted; declared baseline anchoring and
  translated items are retained. That evolution is history, not alternate
  normative implementations.
- TextItem width, script suppression, weak space at run edges, punctuation
  terminal-glue normalization and attachment semantic italics correction
  preserve structural morphology; no fixture string/coordinate is a trigger.

### Current audit limitations and ambiguities

These are source-inspection findings, not measured new-pin failures or newly
approved exceptions:

1. Accent still expands width when accent.width > 2*base.width
   (`01_core/src/compiler/math/layout/accent.rs:167`). This synthetic
   heuristic is not adopted as a language requirement. Attachment lookup still
   depends on leaf/character recognition; selected-glyph attribution and lower
   attachment coverage need independent evidence.
2. Stretch accepts only the leaf forms recognized by leaf_char and returns
   natural layout if both/neither axes are available
   (`01_core/src/compiler/math/layout/stretch.rs:15`, `:38`).
   Complete attached-body stretch and the earlier phrase “ratified applicable
   diagnostic” cannot be established from that implementation. Preserve the
   approved scope and flag the missing evidence rather than inventing an error.
3. Matrix geometry normalizes offsets to Option and conditional emission skips
   invalid boundaries (`01_core/src/compiler/math/layout/matrix.rs:182`,
   `:194`, `:465`). The contracted diagnostic must be traced through public
   validation/call paths, including cascaded values; this audit does not claim
   that every route rejects invalid offsets. Vertical augment positioning also
   merits independent baseline-relative verification. Do not bless silent
   omission or coordinate approximations.
4. Spacing retains first-scalar classification and incomplete composite-class
   inheritance. Generic spaced frames/normal-class wrapped spaced content and
   mixed-size per-item spacing are not established by the sequence-level flag.
   Historical candidate fixes were not rewritten as requirements.
5. External layout retains constrained contextual/region/introspection/image
   support and approximations in scanning glyph extents. Clip/group behavior
   cited by old exporter measurements is not reclassified as fixed or broken
   globally by this documentation task. It requires a separate in-scope audit.
6. The context-free MathLayouter constructor remains public, contrary to old
   text that implied every constructor required region/callback context.
   Productive equation layout uses new_with_context
   (`01_core/src/compiler/layout/equation.rs:117`); preserving that distinction
   does not authorize removing the compatibility constructor.
7. Historical sources frequently refer to moved engine/rules paths, temporary
   fixtures and removed execution reports. The recovery manifest and current
   consumer audit locators replace them as current traceability. Missing
   artifacts remain historical leads, never proof of a present contract.
8. Current upstream parity target is the repository's pinned
   `9f2b6e8715237cb086899a42873660fe744622e8`. Neither code reading nor old
   `a51e02804`/`586e1bd43` results establish parity against that target.
   No fresh bilateral parity or performance measurement was run here.

## Validation policy

Only assigned Markdown L0s and this diagnostic are writable for this task.
Source/test headers, code hashes, nucleus bytes and pins are preserved.
No productive code/test/ADR/nucleus edit, global formatting, line counting
or global hash reseal is authorized. V5 drift from translated prompt bytes
remains pending source-header resealing by a separately authorized scoped
integration. V15/V26 and the protected-file snapshot are checked read-only;
the actual results are recorded below after validation.

## Resume intake byte manifest

| Assigned L0 | SHA-256 before rewriting |
| --- | --- |
| `00_nucleo/prompts/compiler/lang/equation_supplement.md` | `eb139fbbda2c742fda408506de2c018db1b60f62c72824bae31b09f996011148` |
| `00_nucleo/prompts/compiler/lang/figure_supplement.md` | `fd8f727ec47ed59ecdd1394f72a4d13cad18609e5f7ac35edb121650c8cb7f90` |
| `00_nucleo/prompts/compiler/lang/heading_supplement.md` | `987ec566c4871c93db4eab7dbe06a7f1827b7d9ba91584b5ecc9802640a27784` |
| `00_nucleo/prompts/compiler/lang/outline_title.md` | `5e0cac9d80c2e68440be258bd589f6682f9c218ff2b2bffacd773e11bdaee814` |
| `00_nucleo/prompts/compiler/lang/quotes.md` | `74355f2751d3993497ae9bbffaa55d38ded96af82fd55ec9e595e9a43502c311` |
| `00_nucleo/prompts/compiler/lang.md` | `787b173e01f866329611ee8ed708d0c7ba352b23f352f1eaab78a741e0a216c1` |
| `00_nucleo/prompts/compiler/lexer/code.md` | `0c0a074116477344f57951df178f6e1f3f846723944a169cef4ffba42c8edb1e` |
| `00_nucleo/prompts/compiler/lexer/markup.md` | `29472b6537fa28cb8f33da4bc1ea5a6f3c73627d4903a46adcda0babe2216bb8` |
| `00_nucleo/prompts/compiler/lexer/math.md` | `5c6b1f940e3003d109cd492735351a6c0bf448abd0572de1a9e6acee628045d1` |
| `00_nucleo/prompts/compiler/lexer/mod.md` | `21fff52ea6b30357252bcc9d3c63ebe21f2391e597c40f6dd670323c065e4781` |
| `00_nucleo/prompts/compiler/math/layout/_comum.md` | `51308375f1f4322b8a95bf52c9f18a398d83fa59cbec5329bb5cbd9d38612923` |
| `00_nucleo/prompts/compiler/math/layout/accent.md` | `13d509aa5b157b4a379c067b2794bdb67efa9a6462ad0c49afa26208afe37c8f` |
| `00_nucleo/prompts/compiler/math/layout/assembly.md` | `3772841c1ae99f82d2b765b4b38448f3bada7b3461f1473aa2b635590eb3141f` |
| `00_nucleo/prompts/compiler/math/layout/attach.md` | `a36de9faf5c12effe4719b28d313e2d2c710bcea0056635e4f317eb13aab8814` |
| `00_nucleo/prompts/compiler/math/layout/callbacks.md` | `8deb23e6df74836743fdb61d30541c1e889884dc94f086f2a835198eb7812ac0` |
| `00_nucleo/prompts/compiler/math/layout/cancel.md` | `22de6d1a04ae05d56d2094a888a563534a929c2467cd4ebc9fb1d18ded0f9519` |
| `00_nucleo/prompts/compiler/math/layout/cases.md` | `5fae95ecafc61eb7b40ac6990ae5f4c4bfebfecf2ec48ca31a2e31e0a9d460c5` |
| `00_nucleo/prompts/compiler/math/layout/delimited.md` | `338f44f0cf8ce96986f83d834181862adb798aa12973d64516f73249e031afe4` |
| `00_nucleo/prompts/compiler/math/layout/frac.md` | `92480e06e84b4acbb322603d6e8489d4072518c3fd04ecd4ea0bb87f0df8d950` |
| `00_nucleo/prompts/compiler/math/layout/matrix.md` | `7d9fce6f384885f74367c03d965df3fce4e6ce92ddb17e68bccecad28a5f74ba` |
| `00_nucleo/prompts/compiler/math/layout/op.md` | `d8d10da5c02c013eec3cdc22946c9fd828aeb9a6fc10b72b724e337e4afa04f4` |
| `00_nucleo/prompts/compiler/math/layout/p1425-tests.md` | `729b8fd2d959593d0d61a8b43e9ea522308e23e0a18f41486221e3281110147b` |
| `00_nucleo/prompts/compiler/math/layout/p1426-tests.md` | `6fd4e9559b24f4e5709458ffc48ddc2de3d6b45a8d9086745f68dd5cd4e2e692` |
| `00_nucleo/prompts/compiler/math/layout/p1427-tests.md` | `bd727f0f4bb52794e7a088bbf0f3f6f6560e772a96f9dfb4015ff8decc4bbace` |
| `00_nucleo/prompts/compiler/math/layout/p1429-tests.md` | `c704763ad4ac06ca80ac6c28f5fc18787b7d605430fec9ac991c6a9578565bc1` |
| `00_nucleo/prompts/compiler/math/layout/p1430-tests.md` | `59d8037324f8ad6325e0222a14f8c3cc8a90d0a141eec3178f193b4804f9f691` |
| `00_nucleo/prompts/compiler/math/layout/p1431-tests.md` | `685cc3cca1196dd4c7891723e89ceb7a9135a46b61f31881c9784dbbb6cb64ad` |
| `00_nucleo/prompts/compiler/math/layout/root.md` | `462ee09b4c618a7006e8b89916287369bb988a15469047a2b128efa1b728532c` |
| `00_nucleo/prompts/compiler/math/layout/spacing.md` | `a2464f04b9c0a0496c36c9c6fa7f96d41ef15d24b7d22f83127794f61b3439f6` |
| `00_nucleo/prompts/compiler/math/layout/stretch.md` | `c3b3f417542e246238f6efee7cff9e983d7a2b99a3cdc9afe63de21b1e4b4dd5` |
| `00_nucleo/prompts/compiler/math/layout/stretchy.md` | `d8e729f804146f884ea20c545ac17f253fbc0feab1678f1d6504f0fca35c50d4` |
| `00_nucleo/prompts/compiler/math/layout/tests.md` | `ed6169250251ae692c5afcf0ae52e4d4bfb048edbb499306972fedac65a434f3` |
| `00_nucleo/prompts/compiler/math/layout/underline.md` | `3ecf4b76854352a85f050627dc5f2790180b46b389c95c1920a9d56e1c1006b6` |
| `00_nucleo/prompts/compiler/math/layout/underover.md` | `c1072ee862544cb71f6a0dbff5af010850716e2b4640ce0b44da12e76bde75ea` |
| `00_nucleo/prompts/compiler/math/layout/vec.md` | `69bb848d37ee9f1c5eef6837a99bd94ed13b3ff1860d4091870ee081a3fc6997` |
| `00_nucleo/prompts/compiler/math/mod.md` | `f4d5a135e16a380fc86629298a4b20c0b5f23dd36d0f24ffb5e417dc15badff6` |
| `00_nucleo/prompts/compiler/math/symbols.md` | `877e488bc1844dfb0b4cfcde449c332bd5298d2b37b532cbcf7c47c3f481a6a3` |
| `00_nucleo/prompts/compiler/mod.md` | `d0719bb4e675e641279ecb7d066d3ee011522a3c649ccd9e17a0a50afe36f445` |
| `00_nucleo/prompts/compiler/scanner.md` | `89d36dce990544c3bec4eafa1b2c2b9c6043d60ca21483888e4456d62e0810e8` |
| `00_nucleo/prompts/compiler/scopes.md` | `56ede3e2398f8c9a0e4c8d4929324fa621e4d2d8b1f82ac826eca303ba87a080` |
| `00_nucleo/prompts/compiler/utils.md` | `faf1cc836a3db6801e0c04794eddc6b2d152421c4f211004659a3beb74017360` |

## Validation results and scope boundary

Read-only `crystalline-lint --checks v15,v26 --fail-on warning .` completed
with exit 0 and “No violations found” after the rewrites.
Read-only V5 inspection (SARIF) reports drift for the assigned consumers;
all assigned prompts were translated without resealing productive headers.
V5 is therefore pending, not passed. An unsupported JSON output attempt was
rejected without changes; the supported SARIF inspection was used instead.
Scoped `git diff --check` is clean after removing introduced trailing spaces.
All assigned code-hash metadata and nucleus-pin declarations match intake,
and canonical prompt targets were checked; the facade's stale stdlib.md
reference was replaced by its declared owner,
`00_nucleo/prompts/compiler/stdlib/_comum.md`.

The protected-file aggregate differs from the resume snapshot:
intake SHA-256 `4f0af2e3b6cc21415f0d5abbf2388570701a23ae9c3a5f9975aa4f249beedb79`;
first validation SHA-256
`a9e4e60fb81494407f664f53751e0d727e6aabfbfed546a76519a04e8d0200f4`.
There are concurrent out-of-scope writes: modification timestamps during this
work include excluded parse/stdlib prompts and contract/entity prompts.
No task patch targeted them; they were not restored or overwritten.
Consequently, global byte-for-byte immutability is not claimed. The aggregate
alone cannot attribute every difference to a specific concurrent writer.
The task's patch targets are exactly the assigned inventory and this diagnostic.

The original underline evidence cited the historical audited
`typst-layout/src/math/line.rs` SHA-256
`d71623f47d8d3d9d9b5435820e5ca05399563642a4a48627f76ce0c8ed997b34`.
It supported logical frame width versus italics-adjusted rule width at that
snapshot; it does not identify the current upstream source without rechecking.

No productive code, tests, ADRs or shared nuclei were edited by this task.
No build, bilateral parity run, global formatter or hash repair was performed.
Completion applies to this manifested documentation batch, not global sanitation.
