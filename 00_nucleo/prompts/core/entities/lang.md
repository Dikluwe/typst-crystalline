# Prompt L0 — entities/lang
Hash do Código: 0f56ef44

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/lang.rs`
**Governance:** ADR-0052, ADR-0033, ADR-0037.

## Evidence before specification

The original comparison is in
`lab/results/construction/diagnostics/diagnostico-lang-passo-131a.md`.
It motivated the compact upstream-like representation and language-code
surface; it is not a fresh current-pin certification.

## Public contract

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lang([u8; 3], u8);

impl Lang {
    pub const ENGLISH: Self;
    pub fn as_str(&self) -> &str;
}

impl std::str::FromStr for Lang {
    type Err = &'static str;


}
```

Use three ASCII bytes padded with b' ' for a two-byte code plus a u8 length
(2 or 3), with Copy and no allocation. Normalize ASCII case to lowercase;
as_str returns only actual bytes, without trailing padding. ENGLISH.as_str()
is `en`. The specification's detailed validation accepts two or three ASCII
printable characters, including digits, despite the shorter prose/error saying
"letters"; do not silently tighten this unresolved wording to letters only.
Reject non-ASCII such as `日本`, empty input, and length-five `en-GB` with:
`expected two or three letter language code (ISO 639-1/2/3)`.

## Acceptance and deferred scope

Parse `pt` as `pt`, `PT` as `pt`, and `fil` as `fil`; reject empty and en-GB
with the specified diagnostic. ENGLISH is the initial required constant;
further constants are consumer-driven, not an automatic full upstream catalog.
The original scope deferred dir(), additional constants, Region/WritingScript/
Locale, and the hint to put en-GB's region in a region parameter. Those are
separate-owner or future requirements; their old dependency-state descriptions
are historical and do not certify completion or authorize a new API here.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/lang.md`. Historical observations are not a new parity certification.
