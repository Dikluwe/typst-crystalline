# Prompt L0 — `entities/counter_format`
Hash do Código: 4c221be3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/counter_format.rs`
**References:** ADR-0033; hierarchical counter behavior is separately owned.

## Evidence before pattern contract

Historical upstream e0e8ca4d model/numbering.rs:19-21,70-75 specifies counting
symbols with per-symbol prefixes and one final suffix;89-90 repeats the last
symbol with its prefix for extra values;84-87 explains one-based counting
and zero before a first element. The original four-token subset is bounded.

The repeat/truncate correction was measured 2026-08-13,17:30–17:45 -03:00,
HEAD 0c8b64a41 with only L0 edits; reconstructed crystalline release and
historical a51e02804 vanilla (MD5 36da18895eeb5e0136c068a7634e3f82, byte-identical
to the then-lab binary). Before correction, one-token heading patterns consumed
only one level; excess tokens returned None and triggered different caller
fallbacks. The later explicit repeat/truncate rules supersede that old gap.
This is historical evidence, not the current oracle identity.

## Contract

Public pure format_counter(values:&[usize],pattern:&str)->Option<String>
formats hierarchical values. Recognized tokens:1 decimal,I uppercase Roman
(up to 3999; decimal fallback outside range),a lowercase alphabetic
(a1..z26,aa27...),A uppercase equivalent. Other characters are literal prefixes
or suffix. Empty values or no recognized token returns None.

Consume values in order. All characters after the last token form one suffix,
emitted once at the end; other literals prefix their following token.
Extra values repeat the last token with its prefix. Extra tokens/prefixes
are discarded after the last value, then emit the suffix; do not return None.

## Acceptance examples

| Values | Pattern | Result |
|---|---|---|
|[1]|1.|1.|
|[1,2]|1.1|1.2|
|[4]|I.|IV.|
|[2]|(a)|(b)|
|[3]|A.|C.|
|[2,3]|I.1|II.3|
|[1,1,1]|1.|1.1.1.|
|[1]|1.1|1|
|[1,1,1,1]|A.1.a|A.1.a.a|

Test one/two levels of 1., hierarchical 1.1, Roman I., alphabetic(a) including
aa wrap, uppercase A., extra-value repetition and extra-token truncation with
suffix preserved. Keep pure implementation/tests and declared entity facade
re-export boundary.

## Remaining scope and duplicate implementation

Upstream historically enumerated 24 symbols:
1,a,A,i,I,α,Α,一,壹,あ,い,ア,イ,א,가,ㄱ,*,١,۱,१,১,ক,①,⓵.
The star sequence is *,†,‡,§,¶,‖ with repetition above six. The other 20 symbols,
including circled/kanji forms, and nonlinear token placement remain outside
this four-token contract; ordinary last-token repetition is already covered.

The separate compiler stdlib numbering format_pattern already produced
1.2.3. for numbering("1.",1,2,3) in both historical binaries and supports
additional i/א/①. Semantic agreement does not remove duplicate bodies.
Unification requires resolving its Engine dependency used for א/① warnings
under a separate contract; it is not authorized or declared complete here.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/counter_format.md`. Historical observations are not a new parity certification.
