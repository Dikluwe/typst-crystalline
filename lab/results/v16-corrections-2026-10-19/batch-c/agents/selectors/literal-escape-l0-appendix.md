### Literal escaping within the L1 dependency boundary

`lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/literal-escape-provenance.json`
records the inspected working tree, UTC, lockfile and dependency source hashes.
Cargo.lock pins regex 1.12.3 and regex-syntax 0.8.10. Their cached crate archives
were verified against lockfile checksums and consulted source bytes against those
archives. The locked regex source `src/lib.rs:1351–1353` delegates escape to
regex-syntax. The latter's `src/lib.rs:213–221,262–268` prefixes a backslash before
each ASCII metacharacter in `\ . + * ? ( ) | [ ] { } ^ $ # & - ~`, preserving all
other Unicode scalar values. This establishes the current literal interpretation
and stored pattern identity; a direct crate dependency in this evaluator is not
required.

The candidate's direct external `regex::escape` call in this consumer violates
the L1 wrapper boundary (coordinator's full-lint V14 result). Replace it with a
private pure local String/char escape operation using precisely that metacharacter
set. Construct the regex only through the existing `entities::regex::Regex::new`
wrapper. Do not modify the wrapper API, dependency allowlist, lint configuration,
String-to-Kind constructor extension, admission rules or empty-string refusal.

This is an internal architecture correction under continuous ADR-0127 flow, with
L0 first and restricted resealing. The architectural RED is the actual V14; the
new functional controls are preservation controls, not claimed semantic RED.
Require them before and after: every metacharacter individually and in combined
syntax-looking literals; backslash escape-looking text; Unicode including a
combining sequence and emoji; whitespace/control literals; empty refusal; and
exact literal pattern identity used by the existing Regex language relation.
Substring selection must return the whole literal, without accidental captures
or empty matching; nonliteral alternatives must fail. The final full scan must
remove the direct-dependency V14 without suppressions or new violations.
