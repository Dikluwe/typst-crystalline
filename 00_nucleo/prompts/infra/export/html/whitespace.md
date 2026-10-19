# L0 — html/whitespace
Hash do Código: c3d7a33d

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/whitespace.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

## Consumer evidence

`03_infra/src/export/html/whitespace.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Structural space handling

Own inline_html_body, ruby recognition, space_between/top_level_space_between and
SpaceSupport classification. Traverse Styled and inline wrappers without modifying
text/raw payloads. Trim only structural Sequence edges and spaces adjacent to block
or br boundaries; preserve supported inline spaces. Distinguish Boundary,
EmptyInline and Supportive, with replaced tags supporting visible space.

An isolated ASCII space lacking supportive content on both sides uses exactly
`<span style="white-space: pre-wrap">&#x20;</span>`. Top-level lookup reaching a
non-groupable node drops the boundary space rather than making a paragraph/span.
Within ruby, drop a space only when both immediate non-space neighbors are rp/rt;
base text or ordinary inline siblings still preserve spaces. Use tags' static
classification rather than duplicating display tables or implementing CSS.

SpaceFromNewline has the same local projection as Space in these helpers. CJK
normalization and textual show matching belong upstream; do not inspect neighboring
CJK characters to hide an upstream language error. Do not indiscriminately trim
raw/title or change preformatted content.

## Verification

Check edges, br/block adjacency, supportive/replaced versus empty inline nodes,
top-level boundaries, ruby counterexamples and direct Space/SpaceFromNewline pairs.


## Semantic quotes with late target resolution

### Evidence and approval

`lab/results/construction/diagnostics/smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz`
retains historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256,
HEAD/tree/diffstat/UTC/sources/argv/cwd/environment/stdout/stderr. PDF uses text,
HTML DOM/text/diagnostics, not PDF bytes. Eval converted markup to Text, losing
double/enabled; variables retained English glyphs under German/French/disabled
contexts, primes 6'/2" became quotes. Native show/public name already had separate
passing repairs while markup remained non-SmartQuote. Vanilla empty constructor
has no fields, omitted double is Unset versus known false, enabled:false retains
SmartQuote. Positive set double:false/filtered-show controls supplement literal
HTML Text success/programmatic rejection. Literal title quotes render; script
requires string/rejects Content before export in both.
Local markup_stream.rs:79–128 selects eval glyph, layout smartquote.rs toggles two
flags without preceding text, stdlib/text/smartquote.rs:112–117 removes disabled,
observation/elements.rs:242–250 compares current fields. Vanilla markup.rs:141–146
sets explicit double, inline/collect.rs:196–211 resolves styles/context,
text/smartquote.rs:119–158 distinguishes prime/apostrophe/nesting.
Human approval on 2026-10-06 ratified public double bool→Option<bool>,
enabled:Option<bool>, pure helper/context and eval→render glyph choice. Supersede
only conflicting SmartQuote clauses, never other ADRs/nuclei/proposals.

### Owner boundary and acceptance

Domain preserves quote identity/fields in eval, each target resolves glyph at
render composition. HTML stays semantic without paginated quote layout (ADR-0128).
One pure L1 contextual helper uses per-flow/compilation state, no I/O/globals/vtable/
property registry/upstream memoization copy. Stack/state mechanics are Tekt choices;
language/known-versus-omitted fields/glyph/style are acceptance. Escaping remains
with its owners, no new modes/flags/unsafe/watch API/math phase.


Forward shared quote context to stateful inline helper for siblings in one flow.
Space skip/trim observes only effective separators without inline-span reset.
Keep private isolated-flow wrappers, existing ruby/br/block catalogs and preformatted
raw/title behavior, no new CSS/display table.

## Transparent whitespace carriers

whitespace.rs:31 previously recognizes only direct Sequence; :173 direct HtmlElem
neighbors. Pin-verified html-probes.json.gz under auditoria-producao-tekt-2026-10-13
and correcoes-producao-tekt-2026-10-13/html-cli-red.log expose retained br/block/ruby
spaces after Source introduction. Transparently traverse Source.body/ShowRecipe.body/
Located.body/Realized.visual at Sequence entry and space/neighbor classification,
Styled with accumulated chain. Preserve original content for inline emission,
never Realized.original or executing recipes. Wrapped/direct trimming/quote context
must agree, including wrapped break/ruby annotation/empty inline neighbors. No
public API/phase/display table/raw-title trimming. Internal ADR-0127 unit RED
covers wrappers/interior text/br/block/ruby/protected space.

## Nominal decisions and transparent support

### Evidence

`lab/results/construction/diagnostics/tekt-v16-whitespace-2026-10-17/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/sources/
L0/configuration/hashes/UTC/status/diffstat; contexts.json AST/spans. Eleven V16
and one latent identity-forwarding omission in classified_content were measured.
measurement-before.log has five differences/seven Source pairs; measurement-family-
before.log/source-family-before-analysis.json has 28 pairs/20 different: five
left/right break/through-Empty cases fail under Source/Located/Realized.visual/
Styled. Parbreak/Text boundary/support controls agree. This executes the actual
crate algorithm in disposable workspace, not copied/adapted private helpers or
global parity. Preserved initial external E0624 probe earns no functional RED.

### Exact owner decisions

- visual_content at :31 projects only Source/ShowRecipe/Located/Realized; every
  other variant is the borrowed stopping point. Complement excludes only those
  four, with no guards (historical W07).
- space_between :117 and top_level_space_between :128 explicitly cover the three
  remaining cells of each SpaceSupport 3×3 matrix. Boundary absorbs edges,
  Supportive/Supportive uses normal space; all remaining EmptyInline combinations
  require protected space. No guards (W08/W09).
- paragraph_space_support :153 excludes only unconditional Parbreak/Heading/Par
  boundary arms; guarded HtmlElem remains in complement (W10).
- supports_adjacent_space :215 explicitly names prior-true inline-support variants.
  Exclude only Text/Sequence/Strong/Emph/Styled/HtmlElem/Empty/Space/
  SpaceFromNewline/Parbreak, no guards (W11).
- classified_content :39 first binds let content = visual_content(content), then
  Styled delegates and complement returns that exact projected borrow. No clone/@
  capture; exclude only Styled, no guards (W12).

### Acceptance and gates

L0 first/continuous ADR-0127 existing-invariant correction, no public field/signature/
trait/API/default/phase/dependency/flag/compatibility. Complements exclude only
unconditional Content arms, preserve guarded fallback/had_newline. V19 groups
share decisions but remain visible and do not prove every alternative executes.
No allow/expect/@/if-let evasion. Keep existing absorbing Boundary arms and exact
three complement cells, unchanged preformatted/raw/title/ruby/tag tables/SmartQuote.

Regressions in all three owning modules cover both normalization consumers,
single/nested Source/all four carriers at breaks/Empty, Parbreak/Text/MathIdent,
span/style, Realized.visual versus original, positive/negative matching and both
3×3 Boundary/EmptyInline/Supportive matrices. No relaxed existing tests; actual
functional RED→GREEN. Future Content/SpaceSupport sentinel probes compile cfg(test):
core uses real enum; infra only unblocks external-target dependency E0004 using
recorded never-executed synthetic panic arms in disposable copy, never repairs
the selected site. Distinguish blocked compilation from site exhaustivity.
Record 12 final matches/13 arms including new helper; two space_with_origin arms
share one match, not two compiler errors.

Full build/workspace suite/fmt/strict Clippy -D warnings/-D unfulfilled_lint_expectations/
Rustdoc -D warnings/four indexes/strict V1/V5/V15/V26/reverse/replay/full differential
SARIF. Historical expected V16 152→141, only prescribed V19 and no V20 growth;
12 GiB/zero swap/one Cargo job/sequential heavy commands. No commit/push/merge/
release/CI-pin change. General objective/math-stretch-composite remain open.
