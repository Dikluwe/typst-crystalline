# Prompt L0 — `entities/enum_numbering` — `EnumNumbering`
Hash do Código: 20d78a76

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/enum_numbering.rs`

## Contract

EnumNumbering derives Debug/Clone/PartialEq/Hash/Default and retains closed
variants Decimal (#[default]), LowerAlpha, UpperAlpha, LowerRoman, and
Custom(EcoString). Preserve `format(&self, n: u32) -> String` for one-based n
and `from_pattern(p: &str) -> Self`.

Decimal formats n plus `.`; LowerAlpha/UpperAlpha use lowercase/uppercase
alphabetic helper plus `)`; LowerRoman uses its helper plus `)`; Custom
preserves the unknown pattern but formats with Decimal fallback. Parse `1.`
or `1` as Decimal, `a)` as LowerAlpha, `A)` as UpperAlpha, `i)` as LowerRoman,
and every other pattern as Custom unchanged.

Private nth_alpha(n, upper) uses `(n-1) % 26` for the letter. The original
higher-cycle description duplicates letters (aa, bb...), an intentional
bounded approximation rather than upstream's complete sequence; n=1 is a/A,
26 is z/Z and 27 is aa/AA. The same original contract also excludes full
cycles beyond 26 from certified scope. Preserve both this declared fallback
and the coverage limit; do not promote it to full alphabetic parity.
Private to_roman_lower uses the static 1..=12 table
`["i","ii","iii","iv","v","vi","vii","viii","ix","x","xi","xii"]`;
above 12 use decimal n.to_string().

## Acceptance

Decimal(1/3) → `1.`/`3.`; LowerAlpha(1/2) → `a)`/`b)`;
UpperAlpha(1) → `A)`; LowerRoman(1/4) → `i)`/`iv)`.
from_pattern("a)") is LowerAlpha, from_pattern("1.") Decimal,
from_pattern("?!") Custom("?!"), and default is Decimal.

## Deferred scope

Full patterns such as `(1a)`, offsets, arbitrary prefixes/suffixes, UpperRoman,
and complete higher alphabetic cycles remain excluded. The old reserved
UpperRoman proposal does not authorize adding a variant. Zero-input behavior
is not specified by the one-based contract; do not invent it during cleanup.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/enum_numbering.md`. Historical observations are not a new parity certification.
