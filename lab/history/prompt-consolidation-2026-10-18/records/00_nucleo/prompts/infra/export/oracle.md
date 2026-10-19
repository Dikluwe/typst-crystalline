# L0 — infra/export/oracle
Hash do Código: a99160ac

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/oracle.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/oracle.rs:22` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Diagnostic operator transformation

Own collapse_trivial_tj and collapse_trivial_tj_bytes, conservative ASCII
content-stream transformations, not production painting or font selection.
The caller explicitly enables oracle mode; the ordinary builder path does not
apply this transformation. Its activation/stream envelope belongs to
`00_nucleo/prompts/infra/export/builder/streams.md`.

For a well-formed "[ ... ] TJ" in the emitted ASCII format, concatenate nonempty
hex strings and emit "<...> Tj" only when all parsed integer adjustments are zero;
-0 is zero. Preserve hex order and any bytes outside a recognized array.
A nonzero adjustment, unexpected token, missing delimiter, empty/no-hex array or
unrecognized syntax leaves the candidate untouched. Never remove real spacing.
Invalid UTF-8 in the byte wrapper returns the original bytes unchanged.
The string scanner's supported input is generated ASCII, not arbitrary Unicode.

Mathematical fragment noncoalescing is production geometry owned exclusively by
`00_nucleo/prompts/infra/export/stream/text.md`. This diagnostic compares operator structure without upgrading structural
similarity to language parity or authorizing additional positional splitting.

## Verification

Check zero/-0 collapse, ordered concatenation, nonzero/malformed preservation
and invalid-UTF-8 identity. Oracle-off identity is an explicit local regression
contract, not a universal PDF-byte equivalence requirement.
