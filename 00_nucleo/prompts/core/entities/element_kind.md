# Prompt L0 — `entities/element_kind`
Hash do Código: ed42fddf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/element_kind.rs`

## Design evidence and boundary

The original introspection inventory, `lab/results/construction/diagnostics/inventario-tipos-introspection-vanilla.md` (2026-04-30), distinguishes upstream dynamic Element/vtable, selector elements and marker traits from the local closed Content model. Local ElementKind is a narrow, pure, allocation-free discriminator; element-specific level/kind/key data belongs to ElementPayload. Upstream storage/type-erasure is not required language behavior.

## Contract

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementKind {
    Heading, Figure, Citation, Metadata, State, StateUpdate,
    Outline, Bibliography, Equation, CounterUpdate, Table,
    List, Enum, Par, Link, Raw, Quote, Footnote,
    StateDisplay, CounterDisplay, OutlineEntry,
}
impl ElementKind {
    pub fn as_str(self) -> &'static str;
}
```

The complete declared set supersedes the original three-variant introduction. OutlineEntry is required by the later specification's selectable `outline.entry` addition and has textual name `entry`; it was omitted from the older inline enum. Preserve all additions rather than reverting to that stale excerpt.

as_str returns stable names such as `heading`, `figure`, `citation`; reconstruction uses from_name, not a newly invented from_str. Bibliography and Equation names round-trip. Derived Eq/Hash/Copy permit map keys. There are no payload fields and no Other/Unknown catchall. New kinds require a dedicated consumer inventory and justified architectural change when the feature is activated.

## Introspection integration and qualifications

Heading/Figure/Citation, metadata, state/update, outline and bibliography supply corresponding payload identities. Bibliography is locatable and its from-tags consumer populates BibStore. Equation is indexed for kind queries; its documented automatic counter gate is block mode with active equation numbering. The old dormant-until-numbering status is historical, not a global assertion about today's pipeline. CounterUpdate is locatable and its from-tags action applies to CounterRegistry. Table indexes locations and advances its flat counter when numbering and caption are active. Footnote's later locatable contract supersedes its earlier selector-only status and advances once per footnote. StateDisplay and CounterDisplay index locations for their separately owned post-fixpoint display consumers. OutlineEntry indexes synthesized selectable entries.

The original selector-support slice represented List/Enum as item Sequences and Par as text within Sequence. It counted List/Enum/Par/Link/Raw/Quote in the L3 query helper, with Par explicitly approximate (presence of document text), rather than guaranteeing kind_index-backed concrete containers. Preserve this limitation as historical integration debt; the existence of a kind alone does not prove full query support. Do not reapply the old selector-only Footnote limitation after its promotion.

Consumers include Selector::Kind, Introspector kind/first/unique queries, selector parsing in stdlib and infra query helpers, payload extraction and the introspection walk. This data owner neither implements L3 counting nor authorizes reverse imports.

## Acceptance and deferred scope

Verify equality, stable as_str and hash semantics, name round-trips and OutlineEntry identity. Preserve all existing discriminator uses and closed dispatch. Upstream Locatable/Tagged/Unqueriable marker-trait architecture remains outside this design; adding variants or replacing the enum is not part of editorial consolidation.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/element_kind.md` preserves the original evidence and chronology.
