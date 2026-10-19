# Prompt L0 — `entities/span`
Hash do Código: 2306254e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/span.rs`

## Domain and representation

Span traces AST/Content back to source for diagnostics and introspection.
Span(NonZeroU64) is 8 bytes, Copy, with Option<Span> also 8 bytes.
The high 16 bits encode FileId; low 48 encode a node number or raw range.
Numbered spans use unique node numbers in FULL=2..2^47, stable through
incremental editing; Source maps numbers to byte ranges. Raw spans directly
encode endpoints up to the 2^23 boundary (8MiB), for non-Typst files/assets.
Reserved raw1 is detached synthetic content. No source string is stored.
L1 only: no I/O; depend on NonZeroU64 and FileId.

## Public and crate-private interface

Span derives Debug/Copy/Clone/Eq/PartialEq/Hash. Preserve const methods:
detached()->Self; from_range(FileId,Range<usize>)->Self with endpoint saturation
at the declared 2^23 encoding limit; from_raw(NonZeroU64)->Self;
is_detached(self)->bool (raw==1); id(self)->Option<FileId>;
range(self)->Option<Range<usize>> for raw ranges only;
into_raw(self)->NonZeroU64.
Crate-private const from_number(FileId,u64)->Option<Self> rejects outside
FULL; crate-private number(self)->u64 reads the low 48 bits for syntax/parser.
or(self,other)->Self selects other only if self is detached;
find(impl IntoIterator<Item=Self>)->Self returns the first nondetached span.

Spanned<T> derives Copy/Clone/Eq/PartialEq/Hash and has public v:T, span:Span.
Preserve const new(v,span)->Self, detached(v)->Self and
as_ref(&self)->Spanned<&T>; map<F,U>(self,f)->Spanned<U> transforms value while
retaining span. Debug for T:Debug delegates to v.

## Acceptance

Detached has is_detached true, id/range None. For raw FileId5 and number 10,
id is Some(file), number 10 and range None; numbers0/1 reject. Raw177..233
roundtrips with the same file, as do 0..0 and 0..8388607. Preserve endpoint
saturation without inventing a new bound from this cleanup.
Detached.or(b)=b; nondetached b.or(detached)=b. find([detached,b,c]) selects b.
Spanned::new(42u32,detached) preserves 42 and span;
Spanned::detached(10u32).map(|v|v*2) gives 20 and detached. Keep colocated tests,
size/niche invariants and canonical lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/span.md`. Historical observations are not a new parity certification.
