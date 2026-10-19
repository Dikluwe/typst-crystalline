# Prompt L0 — `entities/elements/smartquote` — `SmartQuoteElem`
Hash do Código: 977a3ba2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/smartquote.rs`
**Architecture:** ADR-0105 model D, nonlocatable terminal leaf.

## Evidence before ratified semantic quotes

Historical upstream text/smartquote.rs:51-89,201-317,335-419 distinguishes
omission,explicit auto and single/double overrides. Styled was transparent
to repr but not content.func, so it cannot replace the SmartQuote carrier.
The later smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
records pin9f2b6e8715237cb086899a42873660fe744622e8,binary hashes,HEAD/tree/
diffstat/UTC/source/argv/cwd/environment/stdout/stderr. PDF uses extracted text;
HTML uses DOM/text/diagnostics, not PDF-byte equality.

Markup eval converted quotes to Text, losing double/enabled; stored quote
variables retained English glyphs under German/French or enabled:false.
6'/2" became quotes rather than primes. Native show/name already passed,
but markup was still not SmartQuote. Upstream empty constructor has no fields;
explicit double:false is known, omission Unset; enabled:false remains an
element. set double:false and filtered show provide positive controls.
Literal HTML worked through Text while programmatic SmartQuote rejected.
Literal title rendered quotes; script expected String and rejected Content
before export in both binaries.

Sources: markup_stream.rs:79-128 selected glyph in eval; layout smartquote.rs
used two flags without preceding text; stdlib/text/smartquote.rs:112-117 erased
disabled elements; observation/elements.rs:242-250 compared current fields.
Upstream eval/markup.rs:141-146 creates explicit-double elements;
inline/collect.rs:196-211 resolves styles/context; smartquote.rs:119-158
distinguishes primes/apostrophes/nesting.

## Current public contract: ratified 2026-10-06

SmartQuoteElem derives Debug/Clone/PartialEq/Eq/Hash with public
double:Option<bool>,enabled:Option<bool>,alternative:Option<bool>,
quotes:Option<SmartQuoteQuotes>. This supersedes the older mandatory bool
and three-field schema. Content::SmartQuote holds Arc.

SmartQuotePair has public open/close:EcoString; SmartQuoteOverrides has
public single/double:Option<SmartQuotePair>; SmartQuoteQuotes is Auto or
Custom(SmartQuoteOverrides). These retain Debug/Clone/PartialEq/Eq/Hash.
Outer None means omitted/inherited. Some(false) is explicit. Some(Auto)
clears inherited custom quotes. A missing Custom member uses its localized
pair; simple string/array overrides fill double only, dictionaries may fill
single and double.

Markup stores double Some(kind); a native constructor without double stores
None. Existing Content::smartquote(double:bool) preserves its signature and
explicit double while leaving added configuration omitted. Clone/equality/
hash include all four fields/presence; both maps keep the terminal leaf and
configuration. plain_text is only ASCII fallback double.unwrap_or(true),
never resolved style/glyph. is_empty remains false; nonlocatable kind/payload
remain defaults. The earlier unmeasured fields shape must not be invented;
known/omitted projection follows the ratified nucleus and owning field APIs.
Keep shared terminal match arms rather than splitting merely for extraction.

## Rendering boundary and acceptance

Preserve SmartQuote identity/fields in eval; resolve glyph at each target's
render composition. Paged uses its unit; HTML stays semantic and never calls
paged layout for quotes (ADR-0128). One pure L1 contextual helper uses state
per flow/compilation; no I/O/globals/vtable/property registry or copied upstream
memoization architecture. Stack representation is mechanics; language,
known/omitted fields,glyph/style are acceptance. HTML escaping stays with its
owners. No new modes/flags,unsafe,watch API or math phase.

Human confirmation 2026-10-06 covers bool→Option double, enabled, pure helper/
context and glyph selection eval→render, explicitly including Rust literal
compatibility changes. Require RED→GREEN and exclusive ownership/pins;
other ADRs/nuclei/proposals are unaffected. Preserve all measured prime,
apostrophe,nesting,language/region,disabled,set/show and target controls.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/smartquote.md`. Historical observations are not a new parity certification.
