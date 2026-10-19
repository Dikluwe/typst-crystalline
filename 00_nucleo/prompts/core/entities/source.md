# L0 — In-memory source and syntax tree
Hash do Código: 4a61e5da

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/source.rs`
**Governance:** ADR-0016, ADR-0031, ADR-0045.

## Evidence and responsibility

The historical reference is
`lab/typst-original/crates/typst-syntax/src/source.rs`. Source owns loaded text
and its CST. L3 acquires file bytes/text; this pure type accepts text already
in RAM and parses it. The earlier world_types stub and staged implementation
chronology are archived rather than a second current owner.

## Contract

Preserve private Source backed by Arc<SourceInner>, with fields id: FileId,
text: String, root: SyntaxNode. The original Arc<LazyHash<SourceInner>> was
replaced with Arc and Clone under ADR-0016. Preserve public signatures:

```rust
impl Source {
    pub fn new(id: FileId, text: String) -> Self;
    pub fn detached(text: impl Into<String>) -> Self;
    pub fn id(&self) -> FileId;
    pub fn text(&self) -> &str;
    pub fn root(&self) -> &SyntaxNode;
    pub fn len_bytes(&self) -> usize;
}
```

new/detached parse to construct root. detached uses
FileId::from_raw(NonZeroU16::new(1).unwrap()) as fixed sentinel (or the
corresponding detached helper convention), with no filesystem access.
Retain the later public new_with_parser/detached_with_parser, content_hash,
span_to_line_col, and span_byte_range methods. span_byte_range resolves direct
raw ranges or numbered spans through LinkedNode::find().range(); detached,
other-file or missing spans yield None. The call-trace and shell rendering
consumers rely on this range behavior, analogous to upstream world.range.

## Acceptance and remaining coverage

new(id,"Hello *world*") has non-erroneous Markup root, original text and id.
Empty text has zero bytes. detached("= Heading") has a Heading child;
detached("#{{{broken") is erroneous. Preserve parser injection, content hash,
line/column and range behavior with their current integration tests.
Incremental reparse was excluded in the original bounded scope. The old
blanket exclusion of LinkedNode in find is superseded specifically for the
later span-byte-range method. The old future Hash/Eq/comemo-isolation plan
is historical, not permission to remove current hash APIs or alter L1 purity;
any unresolved trait/representation mismatch needs its own owner review.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/source.md`. Historical observations are not a new parity certification.
