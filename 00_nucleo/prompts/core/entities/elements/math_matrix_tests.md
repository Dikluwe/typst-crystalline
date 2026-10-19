# L0 — core/entities/elements/math_matrix_tests
Hash do Código: 92800067

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/elements/math_matrix_tests.rs`

## Contract and acceptance

Construct the carrier through its public boundary. Discriminate rows/cells,
optional delimiter sides, alignment, horizontal/vertical offsets and their
order, stroke, relative gaps, and all five presence bits. Clone, equality,
hash, repr, plain_text, both maps, and reconstruction preserve every field.
MathAlignPoint stays inside its cell until layout.

Compare pairs differing only in cell order/content, an optional delimiter
side, alignment, offset axis/order, stroke component including cap, gap axis/
component, or one presence bit. Both equality and hash must discriminate each
pair. Also hold presence identical while changing only align:left/right, or
only the relative/absolute component of each row-gap/column-gap. Eq and Hash
must change independently of presence-bit discrimination.
Zero/multiple carriers, inseparable cascade, or unreadable extraction are
Unknown, never success or mutation kills.

## Ratified compact Value adaptation

Before the decision,
`lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/before.json.gz`
and inventory.json froze this consumer and every Value::Stroke file:line.
sizes.json measured 272→144 bytes in an external Value model, not RSS.
The public carrier belongs exclusively to
`00_nucleo/prompts/core/entities/value.md`.

The human confirmed manifesto SHA-256
`6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`
with `confirmo` on 2026-10-08; the earlier pending ADR-0127 gate is satisfied.
This owner adapts only stroke Value construction to From<Stroke>, preserving
all independent morphology/presence/hash pairs and expectations. Preserve
existing oracles/tests; add pertinent RED→GREEN for new isolation behavior,
reseal only the owner and revalidate. No enlarged semantics, defaults, or phase.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_matrix_tests.md`. Historical observations are not a new parity certification.
