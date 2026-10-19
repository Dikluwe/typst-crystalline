# L0 — entities/syntax-node/errors
Hash do Código: 03453213

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_node/errors.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own SyntaxErrorKind, SyntaxError, NumberingResult and Unnumberable. Preserve
Other as default and InvalidHexNumber, InvalidUnicodeCodepoint,
NoTextWithinStars and NoTextWithinUnderscores categories. Do not infer categories
from text or decide eval warning severity. SyntaxError has public span, kind,
message and hints fields, new/with_kind constructors, detached default spans
and empty hints. spanless_eq compares kind/message/hints, ignoring only span,
and remains parent-visible for the representation sibling. NumberingResult is
crate-private Result<(), Unnumberable>; preserve its derives, Error trait and
Display text "cannot number within this interval". ErrorNode storage is outside
this owner. Local tests cover kinds, defaults, hints and numbering failure.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
