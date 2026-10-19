# Prompt L0 — `entities/file_id`
Hash do Código: e17da6aa

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/file_id.rs`
**Reference:** ADR-0001, as amended by the ratified virtual-path contract.

## Evidence and architecture

Original upstream path.rs used a global LazyLock<RwLock<...>> interner mapping
RootedPath to NonZeroU16, incompatible with L1 mutable-global prohibition V13.
FileId therefore remains an opaque handle in L1; the interner lives in L3.
The later virtual-path contract supersedes the broader claim that all path
semantics belong in L3: ADR-0024 permits EcoString in L1, and VirtualPath,
RootedPath and VirtualRoot are pure L1 identity. See
`00_nucleo/prompts/core/entities/path.md`. Only FileId↔RootedPath association,
physical roots and I/O remain in World/L3. No Deref or global state is added.

## Contract

`pub struct FileId(NonZeroU16)` derives Copy/Clone/Eq/PartialEq/Hash.
Preserve public const from_raw(v:NonZeroU16)->Self and
into_raw(self)->NonZeroU16 as the only construction/extraction API.
Use from_raw only with numbers obtained through into_raw. Debug prints
FileId(n). This opaque source-origin key is two bytes, and Option<FileId>
has the same size through the NonZero niche. No I/O/global state/external
crates; only std::num::NonZeroU16 and std::fmt.

L1 uses FileId as a cache/map key without depending on L3. Span encodes it
in the top 16 bits of NonZeroU64. Source uses the key when building syntax;
SystemWorld associates virtual identity, FileId and physical path and answers
world.source(id). RootedPath is contextually obtained through World.

## Acceptance

Raw42 roundtrips; equal raw1 values compare equal and raw1/raw2 differ.
Copying raw7 leaves both values valid/equal. Verify Copy/Clone/Eq/PartialEq/
Hash/Debug, two-byte/niche properties, no external dependencies and zero V13
violations (no L1 LazyLock/RwLock). Keep colocated tests.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/file_id.md`. Historical observations are not a new parity certification.
