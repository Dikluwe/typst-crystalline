# Prompt L0 — `entities/path` — rooted virtual paths
Hash do Código: 0447f38e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/path.rs`
**Governance:** ADR-0024, ADR-0107, ADR-0108, ADR-0127.

## Evidence before specification and approval

Historical upstream `a51e02804`, foundations/path.rs:134–224 defines the public
path type, contextual constructor, repr, PathOrStr; typst-syntax/src/path.rs:
16–60,72–81,189–357 defines root plus normalized virtual path. Recorded probes
establish type(path("x")) == path, idempotent path(path("x")), equality after
./.. normalization, repr(path("a")) == `path("/a")`, backslash/root-escape
rejection, and base retention across files. Commands, outputs and provenance
are in `lab/history/construction/records/00_nucleo/materialization/typst-passo-1141.md`.
Its §6 records the owner's `Prossiga` approval on 2026-08-24, superseding the
original pending public-contract gate. No new public interface is authorized
by this consolidation.

## Pure virtual-path contract

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VirtualRoot { Project, Package(PackageSpec) }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtualPath(EcoString);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootedPath { root: VirtualRoot, vpath: VirtualPath }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathOrStr { Path(RootedPath), Str(EcoString) }
```

Keep fields private. Preserve minimal constructors/accessors VirtualPath::new,
join, parent, get_with_slash; RootedPath::new, root, vpath; and PathOrStr path/
string conversions. EcoString is permitted by ADR-0024. No physical Path or
PathBuf type crosses this boundary; FileId and geometry::PathItem do not
substitute for language path identity.

Normalize lexically using slash: remove ., pop the previous segment for ..,
reject root escape with PathError::Escapes and any backslash with
PathError::Backslash. Virtual paths are normalized absolute strings beginning
with /. Never consult the filesystem. Absolute input starts at the caller's
root; relative input joins the caller file's parent through World resolution.
Root participates in equality/hash: different project/package roots are
distinct even with identical textual repr. Repr is path(<repr absolute vpath>)
and deliberately omits package specification.
PathOrStr::Path is never re-resolved. Resolve Str once at the span/caller FileId
and convert it to RootedPath before I/O.

## Boundary and acceptance

L1 owns virtual identity, normalization and domain errors. World bridges
(FileId,str) to RootedPath and RootedPath to resources; L3 owns physical roots,
caches, defensive canonicalization and I/O. Do not copy upstream's global
interner; ID/cache mechanics may differ under ADR-0107.
Verify a/./b/../c → /a/c; ./x equals x within a root; escaped root and backslash
produce measured error classes/messages; distinct roots remain distinct; and
a path constructed in one file keeps root/vpath when consumed in another.
No I/O, clock, environment, global state or PathBuf in L1; never import or
convert geometry::PathItem.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/path.md`. Historical observations are not a new parity certification.
