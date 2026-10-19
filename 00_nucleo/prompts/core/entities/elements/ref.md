# Prompt L0 — `entities/elements/ref` — `RefElem`
Hash do Código: 0595bcb6

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/ref.rs`
**Architecture:** ADR-0105 model D; nonlocatable terminal leaf.

## Contract

RefElem derives Debug/Clone/PartialEq/Hash and carries name: EcoString,
supplement: Option<Content>, and RefForm::{Normal,Page} with Normal default.
Content::Ref holds Arc<RefElem>. Preserve Content::reference(name:impl
Into<EcoString>) with no supplement and reference_with_supplement(name,
Option<Content>). The reference naming avoids Rust's ref keyword.

plain_text is format!("@{}", self.name); is_empty defaults false. Content/text
maps are terminal; get_field/element_kind/to_payload retain defaults. Store
the label name, not its resolved number. Layout resolves against document
state through Introspector. An explicit supplement takes precedence and is
prefixed to the formatted number; absent supplement uses the referenced type's
default. Page form follows `00_nucleo/prompts/core/entities/page_supplement.md`.

## Historical evidence and unresolved default reconciliation

At upstream `e0e8ca4d`, reference.rs:148-159 describes supplements before
referenced heading/figure numbers. This supports the document-state observable;
it does not prescribe the crystalline Introspector/layout mechanism.
Heading's Refable implementation at heading.rs:321-327 resolves Smart::Auto
through LocalName; translations/en.txt:1-3,5 supplies Figure/Table/Equation/Section.

The 2026-08-13 probe used vanilla e0e8ca4d and crystalline HEAD 4f64e4e69 with
only L0 edits. Its English document set heading numbering "1.", figure "1",
equation "(1)", then referenced one of each. Both produced Section 1;
vanilla Equation 1 versus local (1), and vanilla Figure 1 versus local Fig. 1.
Without explicit figure numbering the local reference was empty, attributed
to the figure default rather than a separate reference defect. The earlier
claim that headings had no default supplement was wrong on both sides.

The archived text also labels this finding corrected while retaining the
contradictory pending figure/equation correction. That heading alone does not
prove closure. Preserve explicit-supplement precedence and the language target;
reconcile the default status with its owning layout contract and approval
receipt before claiming closure or changing behavior under ADR-0127. These
historical observations do not certify the current upstream pin.

## Acceptance

Preserve terminal traversal, label storage, formatted plain text, optional
supplement presence and Normal/Page discrimination. Verify reference defaults
at language level with provenance; do not infer them from Rust representation.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/ref.md`. Historical observations are not a new parity certification.
