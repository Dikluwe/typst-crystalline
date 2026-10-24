# L0 — In-memory source and syntax tree
Hash do Código: dda06691

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


## Approved fallibility contract — 2026-10-22

Supersede only the four infallible Source construction signatures with:

```rust
pub fn new(id: FileId, text: String) -> SourceResult<Self>;
pub fn new_with_parser(id: FileId, text: String, parser: fn(&str) -> SyntaxNode) -> SourceResult<Self>;
pub fn detached(text: impl Into<String>) -> SourceResult<Self>;
pub fn detached_with_parser(text: impl Into<String>, parser: fn(&str) -> SyntaxNode) -> SourceResult<Self>;
```

SourceResult is the existing `Result<T, Vec<SourceDiagnostic>>`; add no error entity,
trait, mode or flag. Numbering failure returns one error diagnostic
`cannot number source spans`, anchored with the real FileId to the input's raw
range via existing Span::from_range. Do not publish an unnumbered/partial Source,
construct a detached success, substitute empty text, catch a panic or exit the
process. Keep content hashes/id/text/AST and valid-source spans unchanged.

Use NonZeroU16::MIN for the existing detached sentinel. Propagate invalid internal
numbering intervals, overflow and invalid child-range failures via existing
NumberingResult before the constructor maps that failure. This does not authorize
synthetic success for invalid SyntaxNode kinds or ignore their separate constructor
contract. Parser callbacks keep their existing function-pointer signature; this
phase does not certify arbitrary external callback implementations.

Migrate productive callers to propagate the existing Result channel, including
expression eval. Test helpers may assert a known fixture succeeded. No productive
caller may restore unwrap/expect, unsafe, a default Source or catch_unwind.
Source/byte cache identity and diagnostics must retain actual causal origin.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.

The private constructor may accept a numbering interval so unit controls can
exercise the real capacity/error mapping without allocating an enormous tree.
Public constructors always supply the immutable Span::FULL domain interval;
this introduces no runtime mode, flag, limit or production fallback.
