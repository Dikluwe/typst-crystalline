# L0 — core/entities/font_list
Hash do Código: 18c72ed8

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/font_list.rs`
**References:** ADR-0053, ADR-0033, ADR-0037, ADR-0107, DEBT-52.

## Contract

FontList is the prioritized family list for `#set text(font: ...)`, including
dictionary regex keys. Preserve this interface:

```rust
pub enum Covers {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontNamePattern { Literal(EcoString), Regex(Regex) }
impl FontNamePattern {
    pub fn is_match(&self, name: &str) -> bool;
    pub fn as_str(&self) -> Option<&str>;
}
pub struct FontFamily {
    pub name: FontNamePattern,
    pub variants: Vec<EcoString>,
    pub covers: Option<Covers>,
}
impl FontFamily {
    pub fn new(name: EcoString) -> Self;
    pub fn new_literal(name: EcoString, variants: Vec<EcoString>) -> Self;
    pub fn new_regex(regex: Regex, variants: Vec<EcoString>) -> Self;
}
pub struct FontList(Vec<FontFamily>);
impl FontList {
    pub fn new(families: Vec<FontFamily>) -> Option<Self>;
    pub fn single(name: EcoString) -> Self;
    pub fn as_slice(&self) -> &[FontFamily];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

Literal family constructors lowercase names. new supplies empty variants and
covers None; new_literal/new_regex preserve supplied variants. Regex uses the
domain Regex::is_match and Arc<regex::Regex> backing with O(1) clone; its pattern
is preserved. Covers is uninhabited, so covers is always None. FontList is
nonempty by construction: new(empty) returns None; single creates one literal
with empty variants; is_empty always returns false. Preserve order/first-match
fallback priority. Literal matching is exact, not a prefix match.

FontBook::select_pattern resolves case-insensitive literals or regex against
the catalog. The layout text owner decodes custom-chain `"text.font"` into
FontList. Style::Font remains Clone+PartialEq; FontNamePattern keeps
Clone+PartialEq+Eq+Hash. Parsing/validation belongs exclusively to
`00_nucleo/prompts/core/compiler/eval/font_dict.md`.

## Acceptance

FontFamily::new("Arial").name.as_str() is Some("arial"). Regex("Name.*") matches
"Name Bold"; Literal("Name") does not. FontList::single("X").len() is 1;
new(vec![]) is None. Attempting to match a Covers variant cannot compile.

## Deferred scope

Concrete covers (LatinInCjk or regex coverage), variant-aware selection
(DEBT-53 / ADR-0054bis), O(1) literal lookup in regex dictionaries and
IntoIterator remain deferred. The historical partial-parity claim does not
certify current upstream parity.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/font_list.md`. Historical observations are not a new parity certification.
