# Prompt L0 — `entities/element_info`
Hash do Código: 0e94a3c4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/element_info.rs`
**Governance:** ADR-0033, ADR-0066.

## Contract

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementInfo {
    pub payload: ElementPayload,
    pub label: Option<Label>,
}
impl ElementInfo {
    pub fn new(payload: ElementPayload) -> Self;
    pub fn with_label(payload: ElementPayload, label: Label) -> Self;
}
```

Keep payload always present and label optional. new stores None;
with_label stores Some(label). Public fields allow direct matching/literal
construction. Equality/hash compare both fields; Clone permits transport by
value inside Tag::Start(Location, ElementInfo). No extra allocation beyond
payload/label. Pure L1, with no I/O.

## Boundary rationale and acceptance

Label is orthogonal to element kind: any indexed element may carry one.
The separate payload/label struct avoids duplicating Option<Label> across
payload variants and supports label→Location indexing without matching the
payload. The historical upstream comparison used per-element labels and
Content::label(); representation differences are deliberate, not a vtable
requirement. The old claim that the whole system has no per-feature entities
is historical rationale, not a ban on the current element architecture.
Tag transport belongs to its owner; introspection constructs unlabeled or
labeled values for wrapped content, and registries may index by Location.
Verify new, with_label, optional-label distinction, both-field equality/hash
and clone transport. Creation-time "no consumers" and prospective registry
status remain historical, not current implementation claims.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/element_info.md`. Historical observations are not a new parity certification.
