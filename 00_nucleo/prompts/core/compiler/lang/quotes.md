# L0 — lang/quotes
Hash do Código: c551f096

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:9dda215eac16294572a1775094e5ecf6dc4b1118d9b2e5300e3ce668dcf65494
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/quotes.rs`

## Consumer evidence

`01_core/src/compiler/lang/quotes.rs:57` locates `fn pair(open: &str, close: &str) -> SmartQuotePair`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Localization and validation

Own pure quote-pair lookup, validation and resolution, using the public types
owned by `00_nucleo/prompts/core/entities/elements/smartquote.md`.
`DEFAULT_QUOTES` is `“`/`”`; `DEFAULT_SINGLE_QUOTES` is `‘`/`’`.

Primary double pairs: pt/es/it `«`/`»`, en `“`/`”`, de `„`/`“`,
fr `«\u{00A0}`/`\u{00A0}»`. Single pairs: en `‘`/`’`, de `‚`/`‘`.
Uncovered or missing language uses the corresponding English curly pair.
German alternative pairs are single `›`/`‹` and double `»`/`«`.
Exact language lookup does not infer BCP47 regions.

`parse_smartquote_quotes(value, span)` accepts auto/none, string, array or
dictionary. Auto/none selects automatic localization. A string must have
exactly two grapheme clusters; an array must have exactly two strings.
A bare string/array overrides the double pair only. A dictionary accepts only
`single` and `double`; each member is auto/none or a valid pair.
Explicit overrides affect only their member; auto preserves its localized
alternative. Pair resolution remains independent of context; resolve_smart_quote owns
nesting, opening/closing context, apostrophes and primes as ratified below.

Preserve span-anchored diagnostics, including singular/plural forms:
`expected 2 characters, found {n} character{s}`,
`expected 2 quotes, found {n} quote{s}`,
`expected string, found {type}`,
`unexpected key "{key}", valid keys are "double" and "single"`,
`expected auto, string, or array, found {type}` and
`expected auto, string, array, or dictionary, found {type}`.
Regional and nesting parity is not established by this owner.


## Ratified semantic quotes and deferred glyph resolution

The following SmartQuote contract supersedes only conflicting historical
SmartQuote clauses. Human confirmation on 2026-10-06 authorizes materialization
with RED→GREEN, exclusive ownership and the pinned nuclei above. Other ADRs,
nuclei and unrelated proposals retain their authority.

### Evidence before decision

`smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz`
records historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256,
HEAD/working tree, diff HEAD --stat, UTC, sources, argv/cwd/environment and
stdout/stderr. PDF acceptance uses extracted text; HTML uses DOM/text and
diagnostics, not PDF byte equality. Evaluation converts markup quotes to Text,
losing double/enabled fields. A value containing quotes retains English glyphs
when inserted in German/French or smartquote(enabled:false). Tekt turns primes
6'/2" into quotes. Native show and the public name were independently corrected
and pass before this gate; markup still is not SmartQuote. An empty vanilla
constructor has empty fields; double:false is known while omitted double is
Unset. enabled:false remains SmartQuote. set double:false and filtered show
supply new positives. Literal HTML works because it receives Text; programmatic
SmartQuote is rejected in the measured implementation. A literal title renders
quotes; script receives a string and rejects Content before export in both
binaries.

Historical source anchors: markup_stream.rs:79–128 selects glyphs in eval;
layout smartquote.rs alternates two flags without preceding text;
stdlib/text/smartquote.rs:112–117 erases disabled elements;
observation/elements.rs:242–250 compares existing fields. Vanilla
`typst-eval/markup.rs:141–146` constructs an element with explicit double;
`typst-layout/inline/collect.rs:196–211` resolves style/context during collection;
`typst-library/text/smartquote.rs:119–158` distinguishes prime/apostrophe/nesting.
These observations retain their historical snapshot qualifications and are not
new current-pin parity evidence.

### Public contract and target boundary

Preserve SmartQuote identity/fields during eval and resolve glyphs only during
each target's render composition. Paged uses its own unit; HTML remains semantic
and must not call paginated layout for quotes (ADR-0128). Contextual selection is
one shared pure L1 helper with per-stream/compilation state, no I/O, globals,
vtable, property registry or upstream memoization copy. Tekt's stack/state is
chosen mechanics; language, known/omitted fields, glyph and style are acceptance.
HTML escaping stays with its owners. Add no mode/flag, unsafe code, watch API or
math phase.

The approved boundary changes public double from bool to Option<bool>, adds
enabled:Option<bool>, exposes the pure helper/context and moves glyph selection
from eval to render. Preserve the ratified interface shared by Paged and HTML:

```rust
#[derive(Debug, Clone, Default)]
pub struct SmartQuoteContext { /* private stack and preceding character */ }
impl SmartQuoteContext {
    pub fn observe_text(&mut self, text: &str);
    pub fn observe_object(&mut self);
    pub fn reset_paragraph(&mut self);
}
pub fn resolve_smart_quote(
    context: &mut SmartQuoteContext,
    element: &SmartQuoteElem,
    chain: &StyleChain,
    effective_lang: Option<&Lang>,
) -> SourceResult<EcoString>;
```

Resolve explicit Option > chain > defaults: double/enabled true, alternative
false, quotes auto. Reuse parse_smartquote_quotes/resolve_smartquote_pair, explicit
or chain effective language and the existing catalog; never use process locale.
Context observes final logical text after show, not Rust bytes of the original
source or the last painted frame. Preserve type-specific nesting, opening/closing,
apostrophe after letter/object and primes after numbers according to the controls.
Disabled emits ASCII without changing the enabled-quote stack; observe emitted
text as subsequent context. Soft wrapping does not reset; parbreak resets;
linebreak represents newline while retaining nesting. Implement no new bidi,
copied arbitrary limit or language/region tables. Already validated configuration
errors retain SourceDiagnostic rather than panic or silent failure.

The historical no-context clause is superseded only for this helper; existing
pair resolution/validation retains its signature and behavior. Regional/nesting
parity outside the ratified controls remains unestablished. Original decision
sequence and approval wording are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/lang/quotes.md`.
