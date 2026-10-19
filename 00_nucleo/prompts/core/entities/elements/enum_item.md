# Prompt L0 — `entities/elements/enum_item` — `EnumItemElem`
Hash do Código: f6fd97e4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/enum_item.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical upstream `e0e8ca4d`, crates/typst-library/src/model/enum.rs:
89–90,149–154 places tight(default true), indent and body_indent(default 0.5em)
on EnumElem, not its item. The 2026-08-13 two-item probe rendered `1. Um` /
`2. Dois` upstream and `1.Um` / `2.Dois` locally. Replicating container settings
into item data is permitted mechanics; a wrong rendered 0pt versus 0.5em
default is an actual language divergence. Its fix belongs to
`00_nucleo/prompts/core/compiler/layout/enum_item.md` and the corresponding
list-item owner; this carrier does not certify it fixed.

## Carrier and constructors

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumItemElem {
    pub number: Option<u32>,
    pub body: Content,
    pub numbering: Option<EnumNumbering>,
    pub indent: Option<Length>,
    pub body_indent: Option<Length>,
    pub tight: Option<bool>,
}
```

Preserve Content::EnumItem(Arc<EnumItemElem>). None numbering resolves to
Decimal at the applicable boundary. indent offsets numbering from left margin;
body_indent offsets body from numbering. tight false adds paragraph spacing;
true is the default adjoining behavior. enum_item(number,body) sets all other
options None; enum_item_with_numbering(number,body,numbering) sets only
numbering Some; enum_item_full(number:Option<u32>,body:Content,
numbering:Option<EnumNumbering>,indent:Option<Length>,body_indent:Option<Length>,
tight:Option<bool>) propagates every field.

## Behavior and acceptance

Text uses no numeric label when number is None. The original formula formats
numbering (or Decimal) with number.unwrap_or(1) and concatenates body. Its
explicit compatibility rule and examples require a separating space when a
number is present: Some(3),body x,no numbering → `3. x`;
None,body x → `x`; Some(1),body a,LowerAlpha → `a) a`.
The compact formula omitted that separator; preserve these explicit observable
criteria, not a silent whitespace change. Both maps recurse into body and
preserve all five other fields. is_empty defaults false; field/kind/payload
default None, non-locatable. Derived equality compares all six fields.
Verify text cases, recursive maps preserving every option, equality including
indent/body_indent/tight, and full-constructor propagation.

Enum expands to a Sequence of items with container settings replicated. This
is intentional under ADR-0107: preserve language argument/visual semantics,
not identical upstream storage. Layout indentation debt remains separately
owned and is not closed by this editorial consolidation.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/enum_item.md`. Historical observations are not a new parity certification.
